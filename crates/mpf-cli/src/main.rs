//! `mpf-cli info <pe-file>` — inspect a PE with the port's own parser.
//! Handy inside the dev container to compare against reference binaries.

use mpf_pe::{machine_name, PeImage};
use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::from(run(std::env::args().skip(1)))
}

fn run(args: impl IntoIterator<Item = String>) -> u8 {
    let args: Vec<String> = args.into_iter().collect();
    if args.len() != 2 || args[0] != "info" {
        eprintln!("usage: mpf-cli info <pe-file>");
        return 2;
    }
    let data = match std::fs::read(&args[1]) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", args[1]);
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
    0
}

#[cfg(test)]
mod tests {
    use super::run;
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
    }

    #[test]
    fn reports_file_read_failure() {
        assert_eq!(run(["info".into(), "/missing/file".into()]), 1);
    }

    #[test]
    fn reports_parse_failure() {
        let path = temp_file(b"not a pe");
        assert_eq!(run(["info".into(), path.clone()]), 1);
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
    fn main_entry_point_runs() {
        // The harness passes its own flags, so this exercises the usage path.
        let _ = super::main();
    }
}
