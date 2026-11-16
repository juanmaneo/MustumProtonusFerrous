//! `mpf-umu` — Rust port of umu-launcher (Open-Wine-Components/umu-launcher).
//!
//!   mpf-umu config [--config FILE] [--prefix DIR]   print resolved config
//!   mpf-umu setup  [--config FILE] [--prefix DIR]   create the WINE prefix
//!   mpf-umu run <exe> [args...] [--config FILE]     run-in-prefix via mpf-proton

use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage: mpf-umu <command> [--config FILE] [--prefix DIR]\n  config                  print resolved UMU config\n  setup                   create the WINE prefix\n  run <exe> [args...]     run-in-prefix via mpf-proton";

fn main() -> ExitCode {
    ExitCode::from(run(std::env::args().skip(1)))
}

fn run(args: impl IntoIterator<Item = String>) -> u8 {
    run_with(&|k: &str| std::env::var(k).ok(), args.into_iter().collect())
}

fn run_with(getenv: &dyn Fn(&str) -> Option<String>, args: Vec<String>) -> u8 {
    let Some((cmd, config, prefix, rest)) = parse_args(&args) else {
        eprintln!("{USAGE}");
        return 2;
    };
    let cfg = match mpf_umu::load_config(getenv, config.as_deref().map(Path::new)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let cfg = match prefix {
        Some(p) => {
            let mut c = cfg;
            c.wineprefix = Some(std::path::PathBuf::from(p));
            c
        }
        None => cfg,
    };
    match cmd.as_str() {
        "config" => {
            let pfx = mpf_umu::resolve_prefix(&cfg, getenv);
            println!("UMU_ID={}", cfg.umu_id);
            println!("GAMEID={}", cfg.game_id);
            if let Some(appid) = mpf_umu::steam_compat_app_id(&cfg.umu_id) {
                println!("STEAM_COMPAT_APP_ID={appid}");
            }
            println!("PROTONPATH={}", cfg.protonpath);
            if let Some(rt) = &cfg.runtime {
                println!("UMU_RUNTIME={rt}");
            }
            println!("WINEPREFIX={}", pfx.display());
            0
        }
        "setup" => {
            let pfx = mpf_umu::resolve_prefix(&cfg, getenv);
            match mpf_umu::setup_prefix(&pfx) {
                Ok(()) => {
                    println!("prefix ready: {}", pfx.display());
                    0
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    1
                }
            }
        }
        "run" if !rest.is_empty() => {
            match mpf_umu::run(getenv, &cfg, Path::new(&rest[0]), &rest[1..]) {
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

fn parse_args(args: &[String]) -> Option<(String, Option<String>, Option<String>, Vec<String>)> {
    let mut cmd = None;
    let mut config = None;
    let mut prefix = None;
    let mut rest = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--config" if i + 1 < args.len() => {
                config = Some(args[i + 1].clone());
                i += 2;
            }
            "--prefix" if i + 1 < args.len() => {
                prefix = Some(args[i + 1].clone());
                i += 2;
            }
            other if cmd.is_none() => {
                cmd = Some(other.to_string());
                i += 1;
            }
            other => {
                rest.push(other.to_string());
                i += 1;
            }
        }
    }
    cmd.map(|c| (c, config, prefix, rest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(tag: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("mpf-umu-cli-{tag}-{nonce}"))
    }

    fn env_get(map: &BTreeMap<String, String>) -> impl Fn(&str) -> Option<String> + '_ {
        move |k| map.get(k).cloned()
    }

    #[test]
    fn parse_args_variants() {
        assert_eq!(parse_args(&[]), None);
        assert_eq!(
            parse_args(&["config".to_string()]),
            Some(("config".to_string(), None, None, vec![]))
        );
        assert_eq!(
            parse_args(&[
                "run".to_string(),
                "e.exe".to_string(),
                "a".to_string(),
                "--config".to_string(),
                "f".to_string(),
                "--prefix".to_string(),
                "p".to_string()
            ]),
            Some((
                "run".to_string(),
                Some("f".to_string()),
                Some("p".to_string()),
                vec!["e.exe".to_string(), "a".to_string()]
            ))
        );
    }

    #[test]
    fn usage_errors() {
        let empty = BTreeMap::new();
        let g = env_get(&empty);
        assert_eq!(run_with(&g, vec![]), 2);
        assert_eq!(run_with(&g, vec!["bogus".into()]), 2);
        assert_eq!(run_with(&g, vec!["run".into()]), 2);
        assert_eq!(run_with(&g, vec!["--config".into()]), 2);
    }

    #[test]
    fn config_command() {
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), "/home/u".to_string());
        env.insert("GAMEID".to_string(), "umu-42".to_string());
        let g = env_get(&env);
        assert_eq!(run_with(&g, vec!["config".into()]), 0);
    }

    #[test]
    fn setup_and_run_commands() {
        let dir = temp_dir("cmd");
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), dir.join("home").to_string_lossy().into_owned());
        env.insert("XDG_DATA_HOME".to_string(), dir.join("data").to_string_lossy().into_owned());
        std::fs::create_dir_all(dir.join("data/umu/sniper_platform_0.1")).unwrap();
        let g = env_get(&env);

        let prefix = dir.join("myprefix").to_string_lossy().into_owned();
        assert_eq!(
            run_with(&g, vec!["setup".into(), "--prefix".into(), prefix.clone()]),
            0
        );
        assert!(std::path::Path::new(&prefix).join("pfx").is_symlink());

        let exe = dir.join("t.exe");
        std::fs::write(&exe, mpf_pe::testing::build_test_pe64()).unwrap();
        assert_eq!(
            run_with(
                &g,
                vec![
                    "run".into(),
                    exe.to_string_lossy().into_owned(),
                    "arg".into(),
                    "--prefix".into(),
                    prefix.clone()
                ]
            ),
            0
        );
        assert_eq!(
            run_with(
                &g,
                vec!["run".into(), dir.join("nope.exe").to_string_lossy().into_owned(), "--prefix".into(), prefix.clone()]
            ),
            1
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
