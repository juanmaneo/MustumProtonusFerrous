//! Seeded conformance cases, one per ported behavior. Names follow wine's
//! `dlls/<dll>/tests` conventions where applicable.

use crate::Case;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NONCE: AtomicU64 = AtomicU64::new(0);

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let n = NONCE.fetch_add(1, Ordering::Relaxed);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("mpf-conf-{tag}-{t}-{n}"))
}

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn shlwapi_path_helpers() {
    let path = w("dir\\file.txt");
    assert_eq!(mpf_shlwapi::path_find_file_name(&path), 4);
    assert_eq!(mpf_shlwapi::path_find_extension(&path), 8);
    assert!(mpf_shlwapi::path_is_relative(&w("file://folder/test.txt")));
    assert!(!mpf_shlwapi::path_is_relative(&w("C:\\test.txt")));
    let mut added = w("one\\two");
    assert_eq!(mpf_shlwapi::path_add_backslash(&mut added), Some(8));
    assert_eq!(added, w("one\\two\\"));
    let mut stripped = w("short//path\\file.txt");
    mpf_shlwapi::path_strip_path(&mut stripped);
    assert_eq!(stripped, w("file.txt"));
}

fn msvcrt_wide_compare() {
    // mirrors dlls/msvcrt/tests/string.c test_wcscmp: sign-only expectations
    assert_eq!(mpf_msvcrt::wcscmp(&w("a"), &w("z")).signum(), -1);
    assert_eq!(mpf_msvcrt::wcscmp(&w("z"), &w("a")).signum(), 1);
    assert_eq!(mpf_msvcrt::wcscmp(&w("f"), &w("f")), 0);
    assert_eq!(mpf_msvcrt::wcslen(&w("hello")), 5);
    assert_eq!(mpf_msvcrt::wcsicmp(&w("HeLLo"), &w("hello")), 0);
}

fn oleaut32_narrowing() {
    // mirrors dlls/oleaut32/tests/vartype.c CONVERTRANGE/OVERFLOWRANGE bounds
    assert_eq!(mpf_oleaut32::var_i1_from_i2(-128), Ok(-128));
    assert_eq!(
        mpf_oleaut32::var_i1_from_i2(128),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
    assert_eq!(mpf_oleaut32::var_i1_from_ui1(127), Ok(127));
    assert_eq!(
        mpf_oleaut32::var_i1_from_ui1(128),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
    assert_eq!(
        mpf_oleaut32::var_ui1_from_i1(-1),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
    assert_eq!(mpf_oleaut32::var_ui1_from_i2(255), Ok(255));
    assert_eq!(
        mpf_oleaut32::var_ui1_from_i2(256),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
}

fn kernel32_lstrlen_semantics() {
    // mirrors dlls/kernel32/tests/string.c lstrlenW expectations
    assert_eq!(
        mpf_kernel32::lstrlen_w(&w("hello")),
        5,
        "lstrlen_w miscounts"
    );
}

fn kernel32_lstrcpyn_truncation() {
    let src = w("abcdef");
    let mut dst = [0u16; 4];
    let n = mpf_kernel32::lstrcpyn_w(&mut dst, &src);
    assert_eq!(n, 6, "lstrcpyn_w returns the source length");
    assert_eq!(
        dst,
        [b'a' as u16, b'b' as u16, b'c' as u16, 0],
        "lstrcpyn_w truncates and terminates"
    );
}

fn ntdll_rtl_compare() {
    use mpf_ntdll::unicode::*;
    let a = WideString::from_utf8("abc");
    let b = WideString::from_utf8("abd");
    assert!(rtl_compare_unicode_strings(&a, &b) < 0, "compare wrong");
    assert!(
        rtl_compare_unicode_strings(&a, &WideString::from_utf8("ABD")) > 0,
        "compare ordering wrong"
    );
    assert!(
        rtl_equal_unicode_string_ignore_case(&a, &WideString::from_utf8("ABC")),
        "ignore-case equal wrong"
    );
    assert!(
        !rtl_equal_unicode_string(&a, &WideString::from_utf8("ABC")),
        "exact equal wrong"
    );
}

fn ntdll_ansi_roundtrip() {
    use mpf_ntdll::unicode::*;
    let wide = WideString::from_utf8("abc");
    let ansi = rtl_unicode_string_to_ansi_string(&wide);
    assert_eq!(ansi, b"abc", "unicode -> ansi wrong");
    assert_eq!(
        rtl_ansi_string_to_unicode_string(&ansi),
        wide,
        "ansi -> unicode wrong"
    );
}

fn ntdll_heap() {
    use mpf_ntdll::heap::*;
    let heap = rtl_process_heap();
    let p = rtl_allocate_heap(heap, 128, true);
    assert!(!p.is_null(), "alloc failed");
    let bytes = unsafe { std::slice::from_raw_parts(p, 128) };
    assert!(bytes.iter().all(|&b| b == 0), "not zeroed");
    assert_eq!(
        rtl_free_heap(heap, p),
        mpf_ntdll::ntstatus::STATUS_SUCCESS,
        "free failed"
    );
}

fn ntdll_critical_section() {
    use mpf_ntdll::critical_section::CriticalSection;
    let cs = CriticalSection::new();
    let _g1 = cs.enter();
    let _g2 = cs.enter();
    // same thread: recursive try must succeed
    assert!(cs.try_enter().is_some(), "recursive try_enter failed");
}

fn ntdll_object_handles() {
    use mpf_ntdll::ntstatus::{is_success, STATUS_INVALID_HANDLE};
    use mpf_ntdll::object::ObjectManager;
    let mut om = ObjectManager::new();
    let h = om.allocate_handle(3);
    assert_eq!(om.query_object(h), Some(3), "handle query wrong");
    assert!(is_success(om.close_handle(h)), "handle close wrong");
    assert_eq!(
        om.close_handle(h),
        STATUS_INVALID_HANDLE,
        "double close wrong"
    );
}

fn ntdll_exception_guard() {
    use mpf_ntdll::exception::{guard, EXCEPTION_RUST_PANIC};
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = guard(|| -> u32 { panic!("boom") });
    std::panic::set_hook(prev);
    assert_eq!(
        r.unwrap_err().code,
        EXCEPTION_RUST_PANIC,
        "panic not converted"
    );
}

fn pe_parse_headers() {
    let data = mpf_pe::testing::build_test_pe64();
    let pe = mpf_pe::PeImage::parse(&data).unwrap();
    assert_eq!(pe.machine(), mpf_pe::MACHINE_AMD64, "machine wrong");
    assert!(pe.is_64bit(), "not PE32+");
    assert_eq!(pe.size_of_image(), 0x2000, "size of image wrong");
}

fn pe_tables() {
    let data = mpf_pe::testing::build_test_pe64();
    let pe = mpf_pe::PeImage::parse(&data).unwrap();
    let exports = pe.exports().unwrap();
    let imports = pe.imports().unwrap();
    let relocs = pe.relocations().unwrap();
    let delay = pe.delay_imports().unwrap();
    assert_eq!(exports.len(), 1, "exports wrong");
    assert_eq!(exports[0].name, "TestFunc", "export name wrong");
    assert_eq!(imports.len(), 1, "imports wrong");
    assert_eq!(imports[0].function, "BaseFunc", "import name wrong");
    assert_eq!(relocs.len(), 1, "relocs wrong");
    assert_eq!(relocs[0].entries.len(), 1, "reloc entries wrong");
    assert_eq!(relocs[0].entries[0].reloc_type, 10, "reloc type wrong");
    assert!(pe.tls_directory().unwrap().is_some(), "tls missing");
    assert_eq!(delay.len(), 1, "delay wrong");
    assert_eq!(delay[0].dll, "DELAY.dll", "delay dll wrong");
}

fn pe_mapped() {
    let file = mpf_pe::testing::build_test_pe64();
    let mut mapped = vec![0u8; 0x2000];
    mapped[..0x200].copy_from_slice(&file[..0x200]);
    mapped[0x1000..0x1300].copy_from_slice(&file[0x200..0x500]);
    let pe = mpf_pe::PeImage::parse_mapped(&mapped).unwrap();
    assert_eq!(pe.exports().unwrap()[0].rva, 0x1080, "mapped exports wrong");
}

fn loader_pipeline() {
    let data = mpf_pe::testing::build_test_pe64();
    let mut m = mpf_loader::map_image(&data).unwrap();
    assert_eq!(m.relocate(0x5000_0000), 1, "reloc count wrong");
    let mut resolver = |dll: &str, func: &str, _ord: Option<u16>| {
        (dll == "KERNEL32.dll" && func == "BaseFunc").then_some(0xDEAD_BEEF)
    };
    assert_eq!(m.resolve_imports(&mut resolver), 1, "import resolve wrong");
    let v = u64::from_le_bytes(m.image()[0x1170..0x1178].try_into().unwrap());
    assert_eq!(v, 0xDEAD_BEEF, "IAT not patched");
}

fn winnt_layouts() {
    use core::mem::offset_of;
    assert_eq!(offset_of!(mpf_winnt::Peb, ldr), 0x18, "PEB offset wrong");
    assert_eq!(
        offset_of!(mpf_winnt::Teb, process_environment_block),
        0x60,
        "TEB offset wrong"
    );
}

fn wineboot_prefix() {
    let root = temp_dir("prefix");
    let prefix = mpf_wineboot::init_prefix(&root).unwrap();
    let system = std::fs::read_to_string(prefix.system_reg()).unwrap();
    assert!(
        system.starts_with(mpf_advapi32::REG_HEADER) && system.contains("11.22"),
        "system.reg wrong"
    );
    mpf_advapi32::init_from_prefix(&root).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
}

fn advapi32_registry() {
    use mpf_advapi32::{RegValue, Registry, RootKey};
    let mut reg = Registry::new();
    let key = reg
        .reg_create_key_ex_w(RootKey::LocalMachine, "Software\\MPF")
        .unwrap();
    assert_eq!(
        reg.reg_set_value_ex_w(key, "version", RegValue::String("11.22".into())),
        0,
        "set failed"
    );
    assert_eq!(
        reg.reg_query_value_ex_w(key, "version").unwrap(),
        RegValue::String("11.22".into()),
        "query wrong"
    );
}

fn kernel32_file() {
    use mpf_kernel32::file::*;
    let dir = temp_dir("file");
    std::fs::create_dir_all(&dir).unwrap();
    let path = w(&dir.join("data.txt").to_string_lossy());
    let h = create_file_w(&path, GENERIC_READ | GENERIC_WRITE, true);
    assert_ne!(h, INVALID_HANDLE_VALUE, "create failed");
    write_file(h, b"mpf").unwrap();
    close_handle(h);
    let h = create_file_w(&path, GENERIC_READ, false);
    let mut buf = [0u8; 8];
    let n = read_file(h, &mut buf).unwrap();
    close_handle(h);
    assert_eq!(&buf[..n], b"mpf", "content wrong");
    std::fs::remove_dir_all(&dir).unwrap();
}

fn kernel32_module() {
    use mpf_kernel32::module::*;
    let data = mpf_pe::testing::build_test_pe64();
    let module = mpf_loader::map_image(&data).unwrap();
    let base = module.base();
    let size = module.size_of_image;
    // The module table points into the mapping, so it must outlive the case.
    std::mem::forget(module);
    register_module("mpf_test.dll", base, size);
    set_current_exe(base);
    assert_eq!(
        get_module_handle_w(None) as usize,
        base,
        "current exe wrong"
    );
    assert_eq!(
        get_proc_address(base as *const core::ffi::c_void, "TestFunc"),
        Some(base + 0x1080),
        "get_proc_address wrong"
    );
}

fn kernel32_thread() {
    use mpf_kernel32::thread::*;
    extern "system" fn noop(_p: *mut core::ffi::c_void) -> u32 {
        0
    }
    let h = create_thread(noop, core::ptr::null_mut());
    assert_eq!(wait_for_thread(h, 0), WAIT_OBJECT_0, "wait wrong");
    assert_ne!(get_current_thread_id(), 0, "thread id 0");
}

fn kernel32_process() {
    use mpf_kernel32::process::*;
    assert_eq!(get_current_process_id(), std::process::id(), "pid wrong");
    assert_eq!(get_current_process() as isize, -1, "pseudo handle wrong");
}

/// All seeded cases.
pub fn all() -> Vec<Case> {
    vec![
        Case {
            name: "shlwapi!PathFindFileName/Extension/IsRelative/AddBackslash/StripPathW",
            run: shlwapi_path_helpers,
        },
        Case {
            name: "msvcrt!wcscmp/wcslen/_wcsicmp",
            run: msvcrt_wide_compare,
        },
        Case {
            name: "oleaut32!VarI1From*/VarUI1From* range checks",
            run: oleaut32_narrowing,
        },
        Case {
            name: "kernel32!lstrlenW semantics",
            run: kernel32_lstrlen_semantics,
        },
        Case {
            name: "kernel32!lstrcpynW truncation",
            run: kernel32_lstrcpyn_truncation,
        },
        Case {
            name: "ntdll!RtlCompareUnicodeString/Equal",
            run: ntdll_rtl_compare,
        },
        Case {
            name: "ntdll!ANSI<->Unicode roundtrip",
            run: ntdll_ansi_roundtrip,
        },
        Case {
            name: "ntdll!RtlAllocateHeap/RtlFreeHeap",
            run: ntdll_heap,
        },
        Case {
            name: "ntdll!critical section recursion",
            run: ntdll_critical_section,
        },
        Case {
            name: "ntdll!object handle table",
            run: ntdll_object_handles,
        },
        Case {
            name: "ntdll!exception guard catches panic",
            run: ntdll_exception_guard,
        },
        Case {
            name: "pe!parse headers",
            run: pe_parse_headers,
        },
        Case {
            name: "pe!exports/imports/relocs/TLS/delay",
            run: pe_tables,
        },
        Case {
            name: "pe!parse_mapped export lookup",
            run: pe_mapped,
        },
        Case {
            name: "loader!map/relocate/resolve",
            run: loader_pipeline,
        },
        Case {
            name: "winnt!PEB/TEB x64 offsets",
            run: winnt_layouts,
        },
        Case {
            name: "wineboot!prefix + hives",
            run: wineboot_prefix,
        },
        Case {
            name: "advapi32!Reg* roundtrip",
            run: advapi32_registry,
        },
        Case {
            name: "kernel32!CreateFileW/ReadFile/WriteFile",
            run: kernel32_file,
        },
        Case {
            name: "kernel32!GetModuleHandle/GetProcAddress",
            run: kernel32_module,
        },
        Case {
            name: "kernel32!CreateThread/WaitForSingleObject",
            run: kernel32_thread,
        },
        Case {
            name: "kernel32!GetCurrentProcess(Id)",
            run: kernel32_process,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_cases_pass() {
        let cases = all();
        let (passed, failed) = crate::run(&cases);
        assert_eq!(failed, 0, "conformance failures");
        assert_eq!(passed, cases.len());
    }
}
