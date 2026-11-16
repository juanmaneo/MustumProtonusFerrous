//! `mpf-proton` — Rust equivalent of Proton's `proton` script.
//!
//!   mpf-proton setup <prefix>                 create/refresh the prefix
//!   mpf-proton env <prefix> [--dxvk-conf F]   print the run environment
//!   mpf-proton run <prefix> <exe> [args...]   run-in-prefix (loader report)

use std::process::ExitCode;

const USAGE: &str = "usage: mpf-proton <command>\n  setup <prefix>\n  env <prefix> [--dxvk-conf FILE]\n  run <prefix> <exe> [args...]";

fn main() -> ExitCode {
    ExitCode::from(run(std::env::args().skip(1)))
}

fn run(args: impl IntoIterator<Item = String>) -> u8 {
    let args: Vec<String> = args.into_iter().collect();
    match args.first().map(String::as_str) {
        Some("setup") if args.len() == 2 => match mpf_wineboot::init_prefix(std::path::Path::new(&args[1])) {
            Ok(p) => {
                println!("prefix ready: {}", p.root.display());
                0
            }
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
        Some("env") => {
            let (prefix, conf) = match parse_env_args(&args[1..]) {
                Some(v) => v,
                None => {
                    eprintln!("{USAGE}");
                    return 2;
                }
            };
            let mut cfg = mpf_proton::ProtonConfig::new(prefix);
            if let Some(f) = conf {
                match std::fs::read_to_string(&f) {
                    Ok(text) => cfg.dxvk_conf = Some(text),
                    Err(e) => {
                        eprintln!("error: cannot read {f}: {e}");
                        return 1;
                    }
                }
            }
            let gfx = mpf_proton::select_graphics(&cfg);
            print!("{}", mpf_proton::env_string(&mpf_proton::build_env(&cfg, &gfx)));
            0
        }
        Some("run") if args.len() >= 3 => {
            let cfg = mpf_proton::ProtonConfig::new(&args[1]);
            match mpf_proton::run_executable(&cfg, std::path::Path::new(&args[2]), &args[3..]) {
                Ok(r) => {
                    println!("prefix:      {}", r.prefix.display());
                    println!("module base: {:#x}", r.module_base);
                    println!("entry point: {:#x}", r.entry_point);
                    println!("relocations: {}", r.relocations_applied);
                    println!("imports:     {} resolved", r.imports_resolved);
                    println!("graphics:    {:?}", r.graphics);
                    0
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    1
                }
            }
        }
        _ => {
            eprintln!("{USAGE}");
            2
        }
    }
}

fn parse_env_args(args: &[String]) -> Option<(String, Option<String>)> {
    let mut prefix = None;
    let mut conf = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dxvk-conf" if i + 1 < args.len() => {
                conf = Some(args[i + 1].clone());
                i += 2;
            }
            other if prefix.is_none() => {
                prefix = Some(other.to_string());
                i += 1;
            }
            _ => return None,
        }
    }
    prefix.map(|p| (p, conf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(tag: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("mpf-proton-cli-{tag}-{nonce}"))
    }

    #[test]
    fn usage_errors() {
        assert_eq!(run(vec![]), 2);
        assert_eq!(run(vec!["bogus".into()]), 2);
        assert_eq!(run(vec!["setup".into()]), 2);
        assert_eq!(run(vec!["env".into()]), 2);
        assert_eq!(run(vec!["env".into(), "p".into(), "--dxvk-conf".into()]), 2);
        assert_eq!(run(vec!["run".into(), "p".into()]), 2);
    }

    #[test]
    fn parse_env_args_variants() {
        assert_eq!(parse_env_args(&["p".to_string()]), Some(("p".to_string(), None)));
        assert_eq!(
            parse_env_args(&["p".to_string(), "--dxvk-conf".to_string(), "f".to_string()]),
            Some(("p".to_string(), Some("f".to_string())))
        );
        assert_eq!(parse_env_args(&["--dxvk-conf".to_string(), "f".to_string()]), None);
        assert_eq!(
            parse_env_args(&["p".to_string(), "q".to_string()]),
            None
        );
    }

    #[test]
    fn setup_and_run_commands() {
        let dir = temp_dir("cmd");
        let prefix = dir.join("prefix").to_string_lossy().into_owned();
        assert_eq!(run(vec!["setup".into(), prefix.clone()]), 0);
        assert!(std::path::Path::new(&prefix).join("drive_c/windows/system.reg").is_file());

        let exe = dir.join("t.dll");
        std::fs::write(&exe, mpf_pe::testing::build_test_pe64()).unwrap();
        assert_eq!(
            run(vec!["run".into(), prefix.clone(), exe.to_string_lossy().into_owned(), "a".into()]),
            0
        );
        assert_eq!(
            run(vec!["run".into(), prefix.clone(), dir.join("nope.dll").to_string_lossy().into_owned()]),
            1
        );
        assert_eq!(run(vec!["env".into(), prefix.clone()]), 0);
        assert_eq!(
            run(vec!["env".into(), prefix.clone(), "--dxvk-conf".into(), dir.join("no.conf").to_string_lossy().into_owned()]),
            1
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
