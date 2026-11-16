//! `mpf-cli` — the MPF developer CLI.
//!
//!   mpf-cli info <pe-file>                          inspect a PE
//!   mpf-cli run <pe-file> [--prefix DIR] [--base N] loader pipeline:
//!       map -> relocate (ASLR) -> resolve imports -> PEB/TEB/LDR setup
//!
//! Handy inside the dev container to compare against reference binaries.

use mpf_pe::{machine_name, PeImage};
use std::process::ExitCode;

const USAGE: &str =
    "usage: mpf-cli info <pe-file>\n       mpf-cli run <pe-file> [--prefix DIR] [--base 0xN]";

fn main() -> ExitCode {
    ExitCode::from(run(std::env::args().skip(1)))
}

fn run(args: impl IntoIterator<Item = String>) -> u8 {
    let args: Vec<String> = args.into_iter().collect();
    match args.first().map(String::as_str) {
        Some("info") if args.len() == 2 => cmd_info(&args[1]),
        Some("run") => cmd_run(&args[1..]),
        _ => {
            eprintln!("{USAGE}");
            2
        }
    }
}

fn cmd_info(path: &str) -> u8 {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return 1;
        }
    };
    let pe = match PeImage::parse(&data) {
        Ok(pe) => pe,
        Err(e) => {
            eprintln!("parse error: {e}");
            return 1;
        }
    };
    println!(
        "machine:      {} ({:#06x})",
        machine_name(pe.machine()),
        pe.machine()
    );
    println!("bits:         {}", if pe.is_64bit() { 64 } else { 32 });
    println!("image base:   {:#x}", pe.image_base());
    println!("size of image:{:#x}", pe.size_of_image());
    println!("entry RVA:    {:#x}", pe.entry_point_rva());
    println!("subsystem:    {}", pe.subsystem());
    println!("sections:");
    for s in &pe.sections {
        println!(
            "  {:<8} VA {:#08x} VS {:#08x} raw {:#08x}+{:#x}",
            s.name(),
            s.virtual_address,
            s.virtual_size,
            s.pointer_to_raw_data,
            s.size_of_raw_data
        );
    }
    match pe.exports() {
        Ok(exports) => {
            println!("exports ({}):", exports.len());
            for e in &exports {
                println!("  ordinal {:<5} RVA {:#08x} {}", e.ordinal, e.rva, e.name);
            }
        }
        Err(e) => eprintln!("export parse error: {e}"),
    }
    match pe.imports() {
        Ok(imports) => {
            println!("imports ({}):", imports.len());
            for i in &imports {
                match i.ordinal {
                    Some(ord) => println!("  {}!#{ord}", i.dll),
                    None => println!("  {}!{}", i.dll, i.function),
                }
            }
        }
        Err(e) => eprintln!("import parse error: {e}"),
    }
    match pe.relocations() {
        Ok(blocks) => {
            let n: usize = blocks.iter().map(|b| b.entries.len()).sum();
            println!("relocations:  {n}");
        }
        Err(e) => eprintln!("reloc parse error: {e}"),
    }
    if let Ok(Some(tls)) = pe.tls_directory() {
        println!("tls callbacks:{:#x}", tls.address_of_call_backs);
    }
    match pe.delay_imports() {
        Ok(delay) => println!("delay imports:{}", delay.len()),
        Err(e) => eprintln!("delay import parse error: {e}"),
    }
    0
}

/// Native export table: the built-in DLLs MPF resolves imports against,
/// standing in for ntdll's builtin module table (`dlls/ntdll/loader.c`).
fn native_resolver(dll: &str, function: &str, _ordinal: Option<u16>) -> Option<u64> {
    match dll.to_ascii_uppercase().as_str() {
        "KERNEL32.DLL" => match function {
            "lstrlenW" => Some(mpf_kernel32::lstrlenW as *const () as usize as u64),
            _ => None,
        },
        "NTDLL.DLL" => match function {
            "RtlProcessHeap" => {
                Some(mpf_ntdll::heap::rtl_process_heap as *const () as usize as u64)
            }
            "RtlAllocateHeap" => {
                Some(mpf_ntdll::heap::rtl_allocate_heap as *const () as usize as u64)
            }
            "RtlFreeHeap" => Some(mpf_ntdll::heap::rtl_free_heap as *const () as usize as u64),
            _ => None,
        },
        _ => None,
    }
}

/// Picks a random 64K-aligned base (wine-11.22 ships ASLR; see ANNOUNCE).
fn aslr_base(preferred: u64) -> u64 {
    use std::io::Read;
    let mut b = [0u8; 8];
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        let _ = f.read_exact(&mut b);
    }
    let rand = u64::from_le_bytes(b);
    let base = 0x0000_6000_0000_0000u64 + (rand & 0x0000_3FFF_FFFF_0000);
    if base == preferred {
        base.wrapping_add(0x1_0000)
    } else {
        base
    }
}

fn cmd_run(args: &[String]) -> u8 {
    let Some(path) = args.first() else {
        eprintln!("{USAGE}");
        return 2;
    };
    let mut prefix: Option<&str> = None;
    let mut base: Option<u64> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--prefix" if i + 1 < args.len() => {
                prefix = Some(&args[i + 1]);
                i += 2;
            }
            "--base" if i + 1 < args.len() => {
                match u64::from_str_radix(args[i + 1].trim_start_matches("0x"), 16) {
                    Ok(b) => {
                        base = Some(b);
                        i += 2;
                    }
                    Err(_) => {
                        eprintln!("error: bad --base value {}", args[i + 1]);
                        return 2;
                    }
                }
            }
            _ => {
                eprintln!("{USAGE}");
                return 2;
            }
        }
    }
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return 1;
        }
    };
    let mut module = match mpf_loader::map_image(&data) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("load error: {e}");
            return 1;
        }
    };
    let new_base = base.unwrap_or_else(|| aslr_base(module.preferred_base));
    let relocs = module.relocate(new_base);
    let imports = module.resolve_imports(&mut |d, f, o| native_resolver(d, f, o));
    let delay = module.resolve_delay_imports(&mut |d, f, o| native_resolver(d, f, o));

    if let Some(dir) = prefix {
        match mpf_wineboot::init_prefix(std::path::Path::new(dir)) {
            Ok(p) => println!("prefix:       {}", p.root.display()),
            Err(e) => {
                eprintln!("error: cannot init prefix {dir}: {e}");
                return 1;
            }
        }
    }

    // PEB/TEB/LDR setup, mirroring LdrInitializeThunk in ntdll.
    use mpf_winnt::{
        LdrDataTableEntry, Peb, PebLdrData, RtlUserProcessParameters, Teb, UnicodeString,
    };
    let mut ldr = Box::new(PebLdrData::new());
    let mut entry = Box::new(LdrDataTableEntry {
        in_load_order_links: mpf_winnt::ListEntry::zeroed(),
        in_memory_order_links: mpf_winnt::ListEntry::zeroed(),
        in_initialization_order_links: mpf_winnt::ListEntry::zeroed(),
        dll_base: module.base() as *mut core::ffi::c_void,
        entry_point: (module.base() + module.entry_point_rva as usize) as *mut core::ffi::c_void,
        size_of_image: module.size_of_image,
        full_dll_name: UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: core::ptr::null_mut(),
        },
        base_dll_name: UnicodeString {
            length: 0,
            maximum_length: 0,
            buffer: core::ptr::null_mut(),
        },
    });
    // Link the module into all three LDR lists (insert at head).
    unsafe {
        let e = &mut *entry;
        let lists = [
            (
                &mut ldr.in_load_order_module_list,
                &mut e.in_load_order_links,
            ),
            (
                &mut ldr.in_memory_order_module_list,
                &mut e.in_memory_order_links,
            ),
            (
                &mut ldr.in_initialization_order_module_list,
                &mut e.in_initialization_order_links,
            ),
        ];
        for (head, link) in lists {
            link.flink = head.flink;
            link.blink = head;
            (*head.flink).blink = link;
            head.flink = link;
        }
    }
    let ldr_ptr = Box::into_raw(ldr);
    let params = Box::new(RtlUserProcessParameters {
        maximum_length: core::mem::size_of::<RtlUserProcessParameters>() as u32,
        length: core::mem::size_of::<RtlUserProcessParameters>() as u32,
        flags: 0,
        debug_flags: 0,
        console_handle: core::ptr::null_mut(),
        console_flags: 0,
        _pad0: [0; 4],
    });
    let params_ptr = Box::into_raw(params);
    let heap = mpf_ntdll::heap::rtl_process_heap() as *const mpf_ntdll::heap::Heap
        as *mut core::ffi::c_void;
    let peb = Peb::new(
        module.base() as *mut core::ffi::c_void,
        ldr_ptr,
        params_ptr,
        heap,
    );
    let peb_ptr = Box::into_raw(peb);
    let teb = Teb::new(peb_ptr);

    println!(
        "module base:  {:#x} (preferred {:#x})",
        module.base(),
        module.preferred_base
    );
    println!("relocations:  {relocs} applied");
    println!(
        "imports:      {imports} resolved ({} total)",
        module.imports.len()
    );
    println!("delay imports:{delay} resolved");
    println!("tls callbacks:{:?}", module.tls_callbacks);
    println!("peb:          {:#x}", peb_ptr as usize);
    println!("teb:          {:#x}", &*teb as *const Teb as usize);
    println!("ldr:          {:#x}", ldr_ptr as usize);
    println!(
        "entry point:  {:#x}",
        (module.base() + module.entry_point_rva as usize)
    );
    // NOTE: the entry point is reported, not executed — MPF does not run
    // foreign machine code; native calls go through the resolver above.
    unsafe {
        drop(Box::from_raw(peb_ptr));
        drop(Box::from_raw(ldr_ptr));
        drop(Box::from_raw(params_ptr));
        drop(entry);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{aslr_base, native_resolver, run};
    use mpf_pe::testing::build_test_pe64;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(contents: &[u8]) -> String {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("mpf-cli-{nonce}.dll"));
        std::fs::write(&path, contents).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn rejects_invalid_usage() {
        assert_eq!(run([]), 2);
        assert_eq!(run(["other".into(), "file".into()]), 2);
        assert_eq!(run(["info".into()]), 2);
        assert_eq!(run(["run".into()]), 2);
        assert_eq!(run(["run".into(), "x".into(), "--bogus".into()]), 2);
    }

    #[test]
    fn reports_file_read_failure() {
        assert_eq!(run(["info".into(), "/missing/file".into()]), 1);
        assert_eq!(run(["run".into(), "/missing/file".into()]), 1);
    }

    #[test]
    fn reports_parse_failure() {
        let path = temp_file(b"not a pe");
        assert_eq!(run(["info".into(), path.clone()]), 1);
        assert_eq!(run(["run".into(), path.clone()]), 1);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn reports_pe_details() {
        let path = temp_file(&build_test_pe64());
        assert_eq!(run(["info".into(), path.clone()]), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn reports_pe32_details() {
        let mut data = build_test_pe64();
        // Switch to PE32 so the CLI prints "bits: 32".
        data[0x94..0x96].copy_from_slice(&0xe0u16.to_le_bytes());
        data[0x98..0x9a].copy_from_slice(&0x10Bu16.to_le_bytes());
        let path = temp_file(&data);
        assert_eq!(run(["info".into(), path.clone()]), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn reports_export_import_errors_and_ordinal_imports() {
        // Broken export name table: the export listing fails, imports still print.
        let mut data = build_test_pe64();
        data[0x238..0x23c].copy_from_slice(&0x1fffu32.to_le_bytes());
        // Ordinal import instead of a name import.
        data[0x360..0x368].copy_from_slice(&((1u64 << 63) | 7).to_le_bytes());
        let path = temp_file(&data);
        assert_eq!(run(["info".into(), path.clone()]), 0);
        std::fs::remove_file(path).unwrap();

        // Broken import directory: the import listing fails.
        let mut data = build_test_pe64();
        data[0x110..0x114].copy_from_slice(&0x1fffu32.to_le_bytes());
        let path = temp_file(&data);
        assert_eq!(run(["info".into(), path.clone()]), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn run_pipeline_loads_relocates_resolves_and_builds_peb() {
        let path = temp_file(&build_test_pe64());
        assert_eq!(
            run([
                "run".into(),
                path.clone(),
                "--base".into(),
                "0x50000000".into()
            ]),
            0
        );
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn run_pipeline_with_prefix_and_aslr() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let prefix = std::env::temp_dir().join(format!("mpf-run-prefix-{nonce}"));
        let path = temp_file(&build_test_pe64());
        assert_eq!(
            run([
                "run".into(),
                path.clone(),
                "--prefix".into(),
                prefix.to_string_lossy().into_owned()
            ]),
            0
        );
        assert!(prefix.join("drive_c/windows/system.reg").is_file());
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir_all(prefix).unwrap();
    }

    #[test]
    fn run_rejects_bad_base() {
        let path = temp_file(&build_test_pe64());
        assert_eq!(
            run(["run".into(), path.clone(), "--base".into(), "zzz".into()]),
            2
        );
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn native_resolver_knows_builtin_exports() {
        assert!(native_resolver("KERNEL32.dll", "lstrlenW", None).is_some());
        assert!(native_resolver("ntdll.dll", "RtlProcessHeap", None).is_some());
        assert!(native_resolver("KERNEL32.dll", "NoSuchFunc", None).is_none());
        assert!(native_resolver("UNKNOWN.dll", "lstrlenW", None).is_none());
        assert!(native_resolver("KERNEL32.dll", "", Some(3)).is_none());
    }

    #[test]
    fn aslr_base_is_aligned_and_differs_from_preferred() {
        let b = aslr_base(mpf_pe::testing::TEST_IMAGE_BASE);
        assert_eq!(b % 0x1_0000, 0);
        assert_ne!(b, mpf_pe::testing::TEST_IMAGE_BASE);
    }

    #[test]
    fn main_entry_point_runs() {
        // The harness passes its own flags, so this exercises the usage path.
        let _ = super::main();
    }
}
