//! Rust equivalent of Proton's `proton` script (see `repos/proton/proton`):
//! prefix setup, environment construction (WINE*/PROTON_*/DXVK_*/VKD3D_*),
//! graphics companion selection, and run-in-prefix via the MPF loader.
//!
//! Like the rest of MPF, foreign machine code is not executed: `run_executable`
//! performs the full loader pipeline and returns a report of what would run.

use mpf_graphics::{dxvk, vkd3d};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Resolved graphics companions for a prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Graphics {
    None,
    Dxvk(dxvk::DxvkInstall),
    Vkd3d(PathBuf),
    Both(dxvk::DxvkInstall, PathBuf),
}

/// Proton-flavour configuration.
#[derive(Debug, Clone)]
pub struct ProtonConfig {
    pub prefix: PathBuf,
    pub winedebug: String,
    /// Extra dxvk.conf content (merged over the prefix's dxvk.conf).
    pub dxvk_conf: Option<String>,
    pub disable_dxvk: bool,
    pub disable_vkd3d: bool,
}

impl ProtonConfig {
    pub fn new(prefix: impl Into<PathBuf>) -> Self {
        ProtonConfig {
            prefix: prefix.into(),
            winedebug: "-all".to_string(),
            dxvk_conf: None,
            disable_dxvk: false,
            disable_vkd3d: false,
        }
    }
}

/// Default prefix location, like STEAM_COMPAT_DATA_PATH/<appid>.
pub fn default_prefix(getenv: &dyn Fn(&str) -> Option<String>, app_id: Option<&str>) -> PathBuf {
    if let Some(compat) = getenv("STEAM_COMPAT_DATA_PATH") {
        return Path::new(&compat).join(app_id.unwrap_or("default"));
    }
    let home = getenv("HOME").unwrap_or_else(|| ".".to_string());
    let data = getenv("XDG_DATA_HOME").unwrap_or_else(|| format!("{home}/.local/share"));
    Path::new(&data)
        .join("Steam/steamapps/compatdata")
        .join(app_id.unwrap_or("default"))
}

/// Selects dxvk/vkd3d-proton companions present in the prefix.
pub fn select_graphics(cfg: &ProtonConfig) -> Graphics {
    let dxvk = if cfg.disable_dxvk {
        None
    } else {
        dxvk::detect(&cfg.prefix).filter(|i| i.is_complete())
    };
    let vkd3d = if cfg.disable_vkd3d {
        None
    } else {
        vkd3d::detect(&cfg.prefix)
    };
    match (dxvk, vkd3d) {
        (Some(d), Some(v)) => Graphics::Both(d, v),
        (Some(d), None) => Graphics::Dxvk(d),
        (None, Some(v)) => Graphics::Vkd3d(v),
        (None, None) => Graphics::None,
    }
}

/// Builds the environment Proton would set for a run.
pub fn build_env(cfg: &ProtonConfig, graphics: &Graphics) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    env.insert(
        "WINEPREFIX".to_string(),
        cfg.prefix.to_string_lossy().into_owned(),
    );
    env.insert("WINEDEBUG".to_string(), cfg.winedebug.clone());
    env.insert(
        "WINEDLLOVERRIDES".to_string(),
        "winemenubuilder.exe=d".to_string(),
    );
    env.insert("PROTON_LOG".to_string(), "1".to_string());
    if let Graphics::Dxvk(_) | Graphics::Both(_, _) = graphics {
        let conf_path = cfg.prefix.join("dxvk.conf");
        let mut text = std::fs::read_to_string(&conf_path).unwrap_or_default();
        if let Some(extra) = &cfg.dxvk_conf {
            text.push('\n');
            text.push_str(extra);
        }
        let parsed = dxvk::parse_dxvk_conf(&text);
        for (k, v) in dxvk::env_from_config(&parsed) {
            env.insert(k, v);
        }
        env.insert("PROTON_DXVK".to_string(), "1".to_string());
    }
    if matches!(graphics, Graphics::Vkd3d(_) | Graphics::Both(_, _)) {
        env.insert("PROTON_VKD3D".to_string(), "1".to_string());
    }
    env
}

/// Renders env as `KEY=VALUE` lines (sorted), shell-ready.
pub fn env_string(env: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    for (k, v) in env {
        out.push_str(&format!("{k}={v}\n"));
    }
    out
}

/// The built-in native export table used for import resolution.
pub fn native_resolver(dll: &str, function: &str, _ordinal: Option<u16>) -> Option<u64> {
    match dll.to_ascii_uppercase().as_str() {
        "KERNEL32.DLL" => match function {
            "lstrlenW" => Some(mpf_kernel32::lstrlenW as *const () as usize as u64),
            _ => None,
        },
        "NTDLL.DLL" => match function {
            "RtlProcessHeap" => {
                Some(mpf_ntdll::heap::rtl_process_heap as *const () as usize as u64)
            }
            _ => None,
        },
        _ => None,
    }
}

/// Picks a random 64K-aligned base (wine-11.22 ASLR), never `preferred`.
pub fn aslr_base(preferred: u64) -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let rand = std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish();
    aslr_base_from(rand, preferred)
}

fn aslr_base_from(rand: u64, preferred: u64) -> u64 {
    let base = 0x0000_6000_0000_0000u64 + (rand & 0x0000_3FFF_FFFF_0000);
    if base == preferred {
        base + 0x1_0000
    } else {
        base
    }
}

/// What `run` did (foreign code is not executed).
#[derive(Debug, Clone)]
pub struct RunReport {
    pub prefix: PathBuf,
    pub module_base: usize,
    pub entry_point: usize,
    pub relocations_applied: usize,
    pub imports_resolved: usize,
    pub graphics: Graphics,
    pub env: BTreeMap<String, String>,
}

/// Full run-in-prefix pipeline: init prefix, load hives, map + relocate +
/// resolve the executable, register it as a module, build the env.
pub fn run_executable(
    cfg: &ProtonConfig,
    exe: &Path,
    _args: &[String],
) -> Result<RunReport, String> {
    let prefix = mpf_wineboot::init_prefix(&cfg.prefix).map_err(|e| e.to_string())?;
    mpf_advapi32::init_from_prefix(&cfg.prefix).map_err(|e| e.to_string())?;
    let data = std::fs::read(exe).map_err(|e| format!("cannot read {}: {e}", exe.display()))?;
    let mut module = mpf_loader::map_image(&data).map_err(|e| e.to_string())?;
    let base = aslr_base(module.preferred_base);
    let relocs = module.relocate(base);
    let imports = module.resolve_imports(&mut |d, f, o| native_resolver(d, f, o));
    let delay = module.resolve_delay_imports(&mut |d, f, o| native_resolver(d, f, o));
    mpf_kernel32::module::register_module(
        &exe.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
        module.base(),
        module.size_of_image,
    );
    mpf_kernel32::module::set_current_exe(module.base());
    let graphics = select_graphics(cfg);
    let env = build_env(cfg, &graphics);
    Ok(RunReport {
        prefix: prefix.root,
        module_base: module.base(),
        entry_point: module.base() + module.entry_point_rva as usize,
        relocations_applied: relocs,
        imports_resolved: imports + delay,
        graphics,
        env,
    })
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
        std::env::temp_dir().join(format!("mpf-proton-{tag}-{nonce}"))
    }

    fn env_get(map: &BTreeMap<String, String>) -> impl Fn(&str) -> Option<String> + '_ {
        move |k| map.get(k).cloned()
    }

    #[test]
    fn default_prefix_paths() {
        let mut env = BTreeMap::new();
        env.insert("HOME".to_string(), "/home/u".to_string());
        assert_eq!(
            default_prefix(&env_get(&BTreeMap::new()), None),
            PathBuf::from("./.local/share/Steam/steamapps/compatdata/default")
        );
        let p = default_prefix(&env_get(&env), Some("1234"));
        assert_eq!(
            p,
            PathBuf::from("/home/u/.local/share/Steam/steamapps/compatdata/1234")
        );
        env.insert("XDG_DATA_HOME".to_string(), "/data".to_string());
        assert_eq!(
            default_prefix(&env_get(&env), None),
            PathBuf::from("/data/Steam/steamapps/compatdata/default")
        );
        env.insert("STEAM_COMPAT_DATA_PATH".to_string(), "/compat".to_string());
        assert_eq!(
            default_prefix(&env_get(&env), Some("42")),
            PathBuf::from("/compat/42")
        );
    }

    #[test]
    fn graphics_selection() {
        let prefix = temp_dir("gfx");
        let cfg = ProtonConfig::new(&prefix);
        assert_eq!(select_graphics(&cfg), Graphics::None);

        let sys32 = prefix.join("drive_c/windows/system32");
        std::fs::create_dir_all(&sys32).unwrap();
        for dll in dxvk::DXVK_DLLS {
            std::fs::write(sys32.join(dll), b"x").unwrap();
        }
        assert!(matches!(select_graphics(&cfg), Graphics::Dxvk(i) if i.is_complete()));
        std::fs::write(sys32.join("d3d12.dll"), b"x").unwrap();
        std::fs::write(sys32.join("d3d12core.dll"), b"x").unwrap();
        assert!(matches!(select_graphics(&cfg), Graphics::Both(_, _)));

        let mut cfg2 = ProtonConfig::new(&prefix);
        cfg2.disable_dxvk = true;
        let vkd3d = select_graphics(&cfg2);
        assert!(matches!(vkd3d, Graphics::Vkd3d(_)));
        assert!(build_env(&cfg2, &vkd3d).contains_key("PROTON_VKD3D"));
        let mut cfg3 = ProtonConfig::new(&prefix);
        cfg3.disable_vkd3d = true;
        assert!(matches!(select_graphics(&cfg3), Graphics::Dxvk(_)));
        let mut cfg4 = ProtonConfig::new(&prefix);
        cfg4.disable_dxvk = true;
        cfg4.disable_vkd3d = true;
        assert_eq!(select_graphics(&cfg4), Graphics::None);
        std::fs::remove_dir_all(&prefix).unwrap();
    }

    #[test]
    fn build_env_contents() {
        let prefix = temp_dir("env");
        let mut cfg = ProtonConfig::new(&prefix);
        std::fs::create_dir_all(prefix.join("drive_c/windows/system32")).unwrap();
        for dll in dxvk::DXVK_DLLS {
            std::fs::write(prefix.join("drive_c/windows/system32").join(dll), b"x").unwrap();
        }
        std::fs::write(prefix.join("dxvk.conf"), "dxvk.hud = 1\n").unwrap();
        cfg.dxvk_conf = Some("dxvk.async = on".to_string());
        let gfx = select_graphics(&cfg);
        let env = build_env(&cfg, &gfx);
        assert_eq!(env["WINEPREFIX"], prefix.to_string_lossy());
        assert_eq!(env["WINEDEBUG"], "-all");
        assert_eq!(env["WINEDLLOVERRIDES"], "winemenubuilder.exe=d");
        assert_eq!(env["PROTON_LOG"], "1");
        assert_eq!(env["PROTON_DXVK"], "1");
        assert_eq!(env["DXVK_HUD"], "1");
        assert_eq!(env["DXVK_ASYNC"], "on");
        assert!(!env.contains_key("PROTON_VKD3D"));
        let s = env_string(&env);
        assert!(s.contains("WINEPREFIX="));
        assert!(s.ends_with('\n'));
        // sorted output
        let keys: Vec<&str> = s.lines().map(|l| l.split('=').next().unwrap()).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        assert_eq!(keys, sorted);

        let mut cfg_without_extra = ProtonConfig::new(&prefix);
        cfg_without_extra.dxvk_conf = None;
        let env_without_extra = build_env(&cfg_without_extra, &select_graphics(&cfg_without_extra));
        assert!(env_without_extra.contains_key("PROTON_DXVK"));

        let env2 = build_env(&ProtonConfig::new(&prefix), &Graphics::None);
        assert!(!env2.contains_key("DXVK_HUD"));
        assert!(!env2.contains_key("PROTON_DXVK"));
        std::fs::remove_dir_all(&prefix).unwrap();
    }

    #[test]
    fn native_resolver_table() {
        assert!(native_resolver("kernel32.dll", "lstrlenW", None).is_some());
        assert!(native_resolver("NTDLL.DLL", "RtlProcessHeap", None).is_some());
        assert!(native_resolver("NTDLL.DLL", "Nope", None).is_none());
        assert!(native_resolver("OTHER.DLL", "lstrlenW", None).is_none());
        assert!(native_resolver("KERNEL32.DLL", "", Some(1)).is_none());
    }

    #[test]
    fn aslr_base_is_aligned_and_not_preferred() {
        for preferred in [0x1_4000_0000u64, 0x6000_0000_0000] {
            let b = aslr_base(preferred);
            assert_eq!(b % 0x1_0000, 0);
            assert_ne!(b, preferred);
        }
        let preferred = 0x6000_0000_0000u64;
        assert_eq!(aslr_base_from(0, preferred), preferred + 0x1_0000);
        assert_eq!(aslr_base_from(0x1_0000, preferred), preferred + 0x1_0000);
        assert_eq!(aslr_base_from(0x2_0000, preferred), preferred + 0x2_0000);
    }

    #[test]
    fn run_executable_full_pipeline() {
        let dir = temp_dir("run");
        std::fs::create_dir_all(&dir).unwrap();
        let prefix = dir.join("prefix");
        let exe = dir.join("mpf_test.dll");
        std::fs::write(&exe, mpf_pe::testing::build_test_pe64()).unwrap();
        let cfg = ProtonConfig::new(&prefix);
        let report = run_executable(&cfg, &exe, &["arg1".to_string()]).unwrap();
        assert_eq!(report.relocations_applied, 1);
        assert_eq!(report.imports_resolved, 0); // BaseFunc/DelayFunc are not native exports
        assert!(report.module_base != 0);
        assert_eq!(report.entry_point, report.module_base + 0x1000);
        assert_eq!(report.env["WINEPREFIX"], prefix.to_string_lossy());
        assert_eq!(report.graphics, Graphics::None);
        assert!(prefix.join("drive_c/windows/system.reg").is_file());

        let err = run_executable(&cfg, &dir.join("missing.exe"), &[]);
        assert!(err.is_err());

        let blocked_prefix = dir.join("blocked-prefix");
        std::fs::write(&blocked_prefix, b"file").unwrap();
        assert!(run_executable(&ProtonConfig::new(&blocked_prefix), &exe, &[]).is_err());

        let registry_error_prefix = dir.join("registry-error");
        mpf_wineboot::init_prefix(&registry_error_prefix).unwrap();
        std::fs::remove_file(registry_error_prefix.join("drive_c/users/mpf/user.reg")).unwrap();
        std::fs::create_dir(registry_error_prefix.join("drive_c/users/mpf/user.reg")).unwrap();
        assert!(run_executable(&ProtonConfig::new(&registry_error_prefix), &exe, &[]).is_err());

        let invalid = dir.join("invalid.exe");
        std::fs::write(&invalid, b"not a PE image").unwrap();
        assert!(run_executable(&cfg, &invalid, &[]).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
