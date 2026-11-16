//! Rust port of umu-launcher (Open-Wine-Components/umu-launcher):
//! UMU_ID/GAMEID resolution, prefix bootstrap (`$HOME/Games/umu/<id>` with
//! a `pfx` self-symlink), Steam Runtime (sniper/soldier/...) bootstrap and
//! protonfixes, then run-in-prefix via `mpf-proton`.
//!
//! Like umu, runtimes are never downloaded here: `ensure_runtime` fails when
//! the runtime is not already installed under `$XDG_DATA_HOME/umu`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Default id when neither UMU_ID nor GAMEID is set (umu-launcher).
pub const DEFAULT_ID: &str = "umu-default";
/// Default PROTONPATH codename.
pub const DEFAULT_PROTONPATH: &str = "umu-sniper";

/// Minimal ini/TOML-subset parser: `[section]`, `key = value` lines and
/// `#`/`;` comments. Covers umu's `config.toml` `[umu]` section.
pub fn parse_ini(text: &str) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut out: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut section = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].trim().to_string();
            out.entry(section.clone()).or_default();
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim().trim_matches('"');
            out.entry(section.clone())
                .or_default()
                .insert(key.trim().to_string(), value.to_string());
        }
    }
    out
}

/// umu-launcher run configuration (env vars + `[umu]` config file section).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UmUConfig {
    pub umu_id: String,
    pub game_id: String,
    pub protonpath: String,
    pub runtime: Option<String>,
    pub no_protonfixes: bool,
    pub wineprefix: Option<PathBuf>,
}

impl UmUConfig {
    pub fn new(game_id: &str) -> Self {
        UmUConfig {
            umu_id: game_id.to_string(),
            game_id: game_id.to_string(),
            protonpath: DEFAULT_PROTONPATH.to_string(),
            runtime: None,
            no_protonfixes: false,
            wineprefix: None,
        }
    }
}

fn truthy(v: &str) -> bool {
    matches!(v, "1" | "true" | "yes")
}

/// Loads the config: the `[umu]` section of `config_path` (when it exists),
/// overridden by env UMU_ID / GAMEID / PROTONPATH / UMU_RUNTIME /
/// UMU_NO_PROTONFIXES / WINEPREFIX. UMU_ID defaults to GAMEID, like umu.
pub fn load_config(
    getenv: &dyn Fn(&str) -> Option<String>,
    config_path: Option<&Path>,
) -> Result<UmUConfig, String> {
    let mut cfg = UmUConfig::new(DEFAULT_ID);
    let mut umu_id_set = false;
    if let Some(path) = config_path {
        if path.is_file() {
            let text =
                std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            let ini = parse_ini(&text);
            if let Some(umu) = ini.get("umu") {
                if let Some(v) = umu.get("gameid") {
                    cfg.game_id = v.clone();
                }
                if let Some(v) = umu.get("umu_id") {
                    cfg.umu_id = v.clone();
                    umu_id_set = true;
                }
                if let Some(v) = umu.get("protonpath") {
                    cfg.protonpath = v.clone();
                }
                if let Some(v) = umu.get("runtime") {
                    cfg.runtime = Some(v.clone());
                }
                if let Some(v) = umu.get("no_protonfixes") {
                    cfg.no_protonfixes = truthy(v);
                }
                if let Some(v) = umu.get("wineprefix") {
                    cfg.wineprefix = Some(PathBuf::from(v));
                }
            }
        }
    }
    if let Some(v) = getenv("GAMEID").filter(|v| !v.is_empty()) {
        cfg.game_id = v;
    }
    if let Some(v) = getenv("UMU_ID").filter(|v| !v.is_empty()) {
        cfg.umu_id = v;
        umu_id_set = true;
    }
    if !umu_id_set {
        cfg.umu_id = cfg.game_id.clone();
    }
    if let Some(v) = getenv("PROTONPATH").filter(|v| !v.is_empty()) {
        cfg.protonpath = v;
    }
    if let Some(v) = getenv("UMU_RUNTIME").filter(|v| !v.is_empty()) {
        cfg.runtime = Some(v);
    }
    if let Some(v) = getenv("UMU_NO_PROTONFIXES") {
        cfg.no_protonfixes = truthy(&v);
    }
    if let Some(v) = getenv("WINEPREFIX").filter(|v| !v.is_empty()) {
        cfg.wineprefix = Some(PathBuf::from(v));
    }
    Ok(cfg)
}

/// `umu-<appid>` -> STEAM_COMPAT_APP_ID (umu-launcher behaviour).
pub fn steam_compat_app_id(umu_id: &str) -> Option<String> {
    let rest = umu_id.strip_prefix("umu-")?;
    if rest.is_empty() || !rest.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    Some(rest.to_string())
}

/// Default WINEPREFIX: `$HOME/Games/umu/<game_id>` (umu-launcher default).
pub fn default_prefix(getenv: &dyn Fn(&str) -> Option<String>, game_id: &str) -> PathBuf {
    let home = getenv("HOME").unwrap_or_else(|| ".".to_string());
    Path::new(&home).join("Games/umu").join(game_id)
}

/// WINEPREFIX from config/env, else the umu default.
pub fn resolve_prefix(cfg: &UmUConfig, getenv: &dyn Fn(&str) -> Option<String>) -> PathBuf {
    cfg.wineprefix
        .clone()
        .unwrap_or_else(|| default_prefix(getenv, &cfg.game_id))
}

/// Creates the umu prefix layout (umu_run.setup_pfx): the directory itself,
/// a `pfx` self-symlink, `shadercache/`, `gstreamer-1.0/` and `tracked_files`.
pub fn setup_prefix(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;
    std::fs::create_dir_all(path.join("shadercache"))?;
    std::fs::create_dir_all(path.join("gstreamer-1.0"))?;
    let pfx = path.join("pfx");
    if std::fs::symlink_metadata(&pfx).is_err() {
        std::os::unix::fs::symlink(".", &pfx)?;
    }
    std::fs::File::create(path.join("tracked_files"))?;
    Ok(())
}

/// Maps a PROTONPATH codename to its Steam Runtime name
/// (umu_run.resolve_runtime's named_runtimes table).
pub fn runtime_name(protonpath: &str) -> Option<&'static str> {
    match protonpath {
        "umu-sniper" => Some("sniper"),
        "umu-soldier" | "umu-scout" => Some("soldier"),
        "umu-steamrt4" | "umu-steamrt4-arm64" | "umu-latest" | "umu-proton" | "ge-proton"
        | "ge-latest" => Some("steamrt4"),
        _ => None,
    }
}

/// `$XDG_DATA_HOME/umu` (umu_consts.UMU_LOCAL).
pub fn umu_local(getenv: &dyn Fn(&str) -> Option<String>) -> PathBuf {
    let home = getenv("HOME").unwrap_or_else(|| ".".to_string());
    let data = getenv("XDG_DATA_HOME").unwrap_or_else(|| format!("{home}/.local/share"));
    Path::new(&data).join("umu")
}

/// First installed `<runtime>_platform_*` dir under `$XDG_DATA_HOME/umu`.
pub fn find_runtime(getenv: &dyn Fn(&str) -> Option<String>, runtime: &str) -> Option<PathBuf> {
    let needle = format!("{runtime}_platform");
    let entries = std::fs::read_dir(umu_local(getenv)).ok()?;
    entries.flatten().map(|e| e.path()).find(|p| {
        p.is_dir()
            && p.file_name()
                .map(|n| n.to_string_lossy().starts_with(&needle))
                .unwrap_or(false)
    })
}

/// umu's runtime bootstrap, minus the download: fails when the runtime is
/// not already installed (umu would fetch it; MPF never does network I/O).
pub fn ensure_runtime(
    getenv: &dyn Fn(&str) -> Option<String>,
    runtime: &str,
) -> Result<PathBuf, String> {
    find_runtime(getenv, runtime).ok_or_else(|| {
        format!(
            "runtime '{runtime}' is not installed under {}",
            umu_local(getenv).display()
        )
    })
}

/// `$XDG_DATA_HOME/Steam/compatibilitytools.d` (umu_consts.STEAM_COMPAT).
pub fn steam_compat_dir(getenv: &dyn Fn(&str) -> Option<String>) -> PathBuf {
    let home = getenv("HOME").unwrap_or_else(|| ".".to_string());
    let data = getenv("XDG_DATA_HOME").unwrap_or_else(|| format!("{home}/.local/share"));
    Path::new(&data).join("Steam/compatibilitytools.d")
}

/// `$PROTONPATH/protonfixes` — a direct path, or a codename resolved under
/// compatibilitytools.d (umu_run's winetricks handling).
pub fn protonfixes_dir(getenv: &dyn Fn(&str) -> Option<String>, protonpath: &str) -> PathBuf {
    let p = Path::new(protonpath);
    if p.is_dir() {
        p.join("protonfixes")
    } else {
        steam_compat_dir(getenv).join(protonpath).join("protonfixes")
    }
}

/// protonfixes (`$PROTONPATH/protonfixes/winetricks`), run with WINEPREFIX
/// and GAMEID in the environment and cwd = the protonfixes dir.
#[derive(Debug, Clone)]
pub struct Protonfixes {
    pub dir: PathBuf,
}

impl Protonfixes {
    pub fn new(dir: PathBuf) -> Self {
        Protonfixes { dir }
    }

    /// Runs winetricks. Returns Ok(false) when disabled or not installed.
    pub fn apply(&self, prefix: &Path, game_id: &str, no_protonfixes: bool) -> Result<bool, String> {
        if no_protonfixes {
            return Ok(false);
        }
        let winetricks = self.dir.join("winetricks");
        if !winetricks.is_file() {
            return Ok(false);
        }
        let status = std::process::Command::new("sh")
            .arg(&winetricks)
            .current_dir(&self.dir)
            .env("WINEPREFIX", prefix)
            .env("GAMEID", game_id)
            .status()
            .map_err(|e| format!("cannot run {}: {e}", winetricks.display()))?;
        if status.success() {
            Ok(true)
        } else {
            Err(format!("protonfixes exited with {status}"))
        }
    }
}

/// Full umu run: prefix setup, runtime bootstrap, protonfixes, then
/// run-in-prefix through `mpf-proton` (which never executes foreign code).
pub fn run(
    getenv: &dyn Fn(&str) -> Option<String>,
    cfg: &UmUConfig,
    exe: &Path,
    args: &[String],
) -> Result<mpf_proton::RunReport, String> {
    let prefix = resolve_prefix(cfg, getenv);
    setup_prefix(&prefix).map_err(|e| format!("cannot set up prefix {}: {e}", prefix.display()))?;
    let runtime = match &cfg.runtime {
        Some(r) => r.clone(),
        None => runtime_name(&cfg.protonpath)
            .ok_or_else(|| format!("unknown PROTONPATH codename: {}", cfg.protonpath))?
            .to_string(),
    };
    ensure_runtime(getenv, &runtime)?;
    let pf = Protonfixes::new(protonfixes_dir(getenv, &cfg.protonpath));
    pf.apply(&prefix, &cfg.game_id, cfg.no_protonfixes)?;
    mpf_proton::run_executable(&mpf_proton::ProtonConfig::new(&prefix), exe, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(tag: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("mpf-umu-{tag}-{nonce}"))
    }

    fn env_get(map: &BTreeMap<String, String>) -> impl Fn(&str) -> Option<String> + '_ {
        move |k| map.get(k).cloned()
    }

    #[test]
    fn parse_ini_sections_and_comments() {
        let ini = parse_ini(
            "# comment\n; also comment\n[umu]\ngameid = umu-42\nprotonpath = \"umu-sniper\"\n\nbad line\nno_protonfixes = true\n[other]\nk = v\n",
        );
        assert_eq!(ini["umu"]["gameid"], "umu-42");
        assert_eq!(ini["umu"]["protonpath"], "umu-sniper");
        assert_eq!(ini["umu"]["no_protonfixes"], "true");
        assert!(!ini["umu"].contains_key("bad line"));
        assert_eq!(ini["other"]["k"], "v");
        assert!(parse_ini("").is_empty());
    }

    #[test]
    fn load_config_defaults_env_and_file() {
        let empty = BTreeMap::new();
        let cfg = load_config(&env_get(&empty), None).unwrap();
        assert_eq!(cfg.game_id, DEFAULT_ID);
        assert_eq!(cfg.umu_id, DEFAULT_ID);
        assert_eq!(cfg.protonpath, DEFAULT_PROTONPATH);
        assert_eq!(cfg.runtime, None);
        assert!(!cfg.no_protonfixes);
        assert_eq!(cfg.wineprefix, None);

        let dir = temp_dir("cfg");
        let file = dir.join("config.toml");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            &file,
            "[umu]\ngameid = umu-7\numu_id = custom-id\nprotonpath = umu-soldier\nruntime = soldier\nno_protonfixes = 1\nwineprefix = /tmp/pfx\n",
        )
        .unwrap();
        let cfg = load_config(&env_get(&empty), Some(&file)).unwrap();
        assert_eq!(cfg.game_id, "umu-7");
        assert_eq!(cfg.umu_id, "custom-id");
        assert_eq!(cfg.protonpath, "umu-soldier");
        assert_eq!(cfg.runtime.as_deref(), Some("soldier"));
        assert!(cfg.no_protonfixes);
        assert_eq!(cfg.wineprefix, Some(PathBuf::from("/tmp/pfx")));

        let mut env = BTreeMap::new();
        env.insert("GAMEID".to_string(), "umu-99".to_string());
        env.insert("PROTONPATH".to_string(), "umu-sniper".to_string());
        env.insert("UMU_NO_PROTONFIXES".to_string(), "0".to_string());
        env.insert("WINEPREFIX".to_string(), "/env/pfx".to_string());
        let cfg = load_config(&env_get(&env), Some(&file)).unwrap();
        assert_eq!(cfg.game_id, "umu-99");
        assert_eq!(cfg.umu_id, "custom-id");
        assert_eq!(cfg.protonpath, "umu-sniper");
        assert!(!cfg.no_protonfixes);
        assert_eq!(cfg.wineprefix, Some(PathBuf::from("/env/pfx")));

        let cfg = load_config(&env_get(&env), None).unwrap();
        assert_eq!(cfg.game_id, "umu-99");
        assert_eq!(cfg.umu_id, "umu-99");

        let mut env2 = env.clone();
        env2.insert("UMU_ID".to_string(), "umu-5".to_string());
        let cfg = load_config(&env_get(&env2), Some(&file)).unwrap();
        assert_eq!(cfg.umu_id, "umu-5");
        assert_eq!(cfg.game_id, "umu-99");
        assert_eq!(cfg.runtime.as_deref(), Some("soldier"));

        let missing = dir.join("nope.toml");
        assert!(load_config(&env_get(&empty), Some(&missing)).is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn steam_compat_app_id_cases() {
        assert_eq!(steam_compat_app_id("umu-1234"), Some("1234".to_string()));
        assert_eq!(steam_compat_app_id("umu-default"), Some("default".to_string()));
        assert_eq!(steam_compat_app_id("1234"), None);
        assert_eq!(steam_compat_app_id("umu-"), None);
        assert_eq!(steam_compat_app_id("umu-a b"), None);
    }

    #[test]
    fn prefix_resolution() {
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), "/home/u".to_string());
        assert_eq!(
            default_prefix(&env_get(&env), "umu-1"),
            PathBuf::from("/home/u/Games/umu/umu-1")
        );
        let cfg = UmUConfig::new("umu-1");
        assert_eq!(
            resolve_prefix(&cfg, &env_get(&env)),
            PathBuf::from("/home/u/Games/umu/umu-1")
        );
        let mut cfg2 = cfg.clone();
        cfg2.wineprefix = Some(PathBuf::from("/custom"));
        assert_eq!(resolve_prefix(&cfg2, &env_get(&env)), PathBuf::from("/custom"));
    }

    #[test]
    fn setup_prefix_layout_idempotent() {
        let dir = temp_dir("pfx");
        let p = dir.join("pfxroot");
        setup_prefix(&p).unwrap();
        assert!(p.join("pfx").is_symlink());
        assert!(p.join("shadercache").is_dir());
        assert!(p.join("gstreamer-1.0").is_dir());
        assert!(p.join("tracked_files").is_file());
        setup_prefix(&p).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn runtime_name_cases() {
        assert_eq!(runtime_name("umu-sniper"), Some("sniper"));
        assert_eq!(runtime_name("umu-soldier"), Some("soldier"));
        assert_eq!(runtime_name("umu-scout"), Some("soldier"));
        assert_eq!(runtime_name("umu-steamrt4"), Some("steamrt4"));
        assert_eq!(runtime_name("umu-latest"), Some("steamrt4"));
        assert_eq!(runtime_name("/some/path"), None);
    }

    #[test]
    fn ensure_runtime_found_and_missing() {
        let dir = temp_dir("rt");
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), dir.to_string_lossy().into_owned());
        env.insert("XDG_DATA_HOME".to_string(), dir.join("data").to_string_lossy().into_owned());
        let g = env_get(&env);
        assert!(ensure_runtime(&g, "sniper").is_err());
        std::fs::create_dir_all(dir.join("data/umu/sniper_platform_0.20240530.90143")).unwrap();
        std::fs::create_dir_all(dir.join("data/umu/soldier_platform_1")).unwrap();
        std::fs::write(dir.join("data/umu/sniper_platform_notes.txt"), b"x").unwrap();
        let found = ensure_runtime(&g, "sniper").unwrap();
        assert!(found.ends_with("sniper_platform_0.20240530.90143"));
        assert!(ensure_runtime(&g, "steamrt4").is_err());
        assert_eq!(find_runtime(&g, "soldier").unwrap().file_name().unwrap(), "soldier_platform_1");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn protonfixes_dir_variants() {
        let dir = temp_dir("pfdir");
        std::fs::create_dir_all(dir.join("proton")).unwrap();
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), dir.to_string_lossy().into_owned());
        env.insert("XDG_DATA_HOME".to_string(), dir.join("data").to_string_lossy().into_owned());
        let g = env_get(&env);
        assert_eq!(
            protonfixes_dir(&g, dir.join("proton").to_str().unwrap()),
            dir.join("proton/protonfixes")
        );
        assert_eq!(
            protonfixes_dir(&g, "umu-sniper"),
            dir.join("data/Steam/compatibilitytools.d/umu-sniper/protonfixes")
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn protonfixes_apply_branches() {
        let dir = temp_dir("apply");
        let pf_dir = dir.join("protonfixes");
        std::fs::create_dir_all(&pf_dir).unwrap();
        let prefix = dir.join("prefix");
        std::fs::create_dir_all(&prefix).unwrap();
        let pf = Protonfixes::new(pf_dir.clone());

        assert_eq!(pf.apply(&prefix, "umu-1", true).unwrap(), false);
        assert_eq!(pf.apply(&prefix, "umu-1", false).unwrap(), false);

        std::fs::write(
            pf_dir.join("winetricks"),
            "#!/bin/sh\necho \"$GAMEID\" > \"$WINEPREFIX/pf-ran.txt\"\n",
        )
        .unwrap();
        assert_eq!(pf.apply(&prefix, "umu-77", false).unwrap(), true);
        let marker = std::fs::read_to_string(prefix.join("pf-ran.txt")).unwrap();
        assert_eq!(marker.trim(), "umu-77");

        std::fs::write(pf_dir.join("winetricks"), "#!/bin/sh\nexit 3\n").unwrap();
        assert!(pf.apply(&prefix, "umu-77", false).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn run_end_to_end() {
        let dir = temp_dir("run");
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), dir.join("home").to_string_lossy().into_owned());
        env.insert("XDG_DATA_HOME".to_string(), dir.join("data").to_string_lossy().into_owned());
        let g = env_get(&env);
        std::fs::create_dir_all(dir.join("data/umu/sniper_platform_0.1")).unwrap();

        let exe = dir.join("game.exe");
        std::fs::write(&exe, mpf_pe::testing::build_test_pe64()).unwrap();

        let proton = dir.join("proton");
        let pf_dir = proton.join("protonfixes");
        std::fs::create_dir_all(&pf_dir).unwrap();
        std::fs::write(
            pf_dir.join("winetricks"),
            "#!/bin/sh\necho \"$GAMEID\" > \"$WINEPREFIX/pf-ran.txt\"\n",
        )
        .unwrap();

        let mut cfg = UmUConfig::new("umu-1234");
        cfg.protonpath = proton.to_string_lossy().into_owned();
        cfg.runtime = Some("sniper".to_string());
        let report = run(&g, &cfg, &exe, &["a".to_string()]).unwrap();
        let expected_prefix = dir.join("home/Games/umu/umu-1234");
        assert_eq!(report.prefix, expected_prefix);
        assert_eq!(report.env["WINEPREFIX"], expected_prefix.to_string_lossy());
        let marker = std::fs::read_to_string(expected_prefix.join("pf-ran.txt")).unwrap();
        assert_eq!(marker.trim(), "umu-1234");
        assert!(expected_prefix.join("pfx").is_symlink());
        assert!(expected_prefix.join("drive_c/windows/system.reg").is_file());

        let cfg2 = UmUConfig::new("umu-1234");
        let report2 = run(&g, &cfg2, &exe, &[]).unwrap();
        assert_eq!(report2.prefix, expected_prefix);

        let mut cfg3 = UmUConfig::new("umu-1234");
        cfg3.protonpath = "bogus-codename".to_string();
        assert!(run(&g, &cfg3, &exe, &[]).is_err());

        let mut env2 = env.clone();
        env2.remove("XDG_DATA_HOME");
        env2.insert("HOME".to_string(), dir.join("empty-home").to_string_lossy().into_owned());
        let g2 = env_get(&env2);
        assert!(run(&g2, &UmUConfig::new("umu-1234"), &exe, &[]).is_err());

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
