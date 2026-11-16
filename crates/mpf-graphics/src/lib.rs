//! Graphics layer (Phase 3), mirroring how Proton wires dxvk and
//! vkd3d-proton as native companions: MPF detects the companion DLLs in the
//! prefix and translates `dxvk.conf` / env into the DXVK_* / VKD3D_* settings
//! the native libraries read. Porting the D3D cores themselves to Rust is a
//! later phase; for now they ship native, exactly like Proton.

pub mod dxvk {
    //! DXVK (doitsujin/dxvk) companion: config + detection.

    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    /// DLLs a full dxvk install drops into `system32`.
    pub const DXVK_DLLS: [&str; 5] = ["d3d8.dll", "d3d9.dll", "d3d10.dll", "d3d11.dll", "dxgi.dll"];

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct DxvkConfig {
        pub entries: BTreeMap<String, String>,
    }

    /// Parses dxvk.conf text: `key = value` lines, `#` comments, blanks and
    /// malformed lines (no `=`) skipped — like dxvk's ini parser.
    pub fn parse_dxvk_conf(text: &str) -> DxvkConfig {
        let mut cfg = DxvkConfig::default();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            cfg.entries
                .insert(key.trim().to_string(), value.trim().to_string());
        }
        cfg
    }

    /// Formats back to dxvk.conf text.
    pub fn format_dxvk_conf(cfg: &DxvkConfig) -> String {
        let mut out = String::new();
        for (k, v) in &cfg.entries {
            out.push_str(&format!("{k} = {v}\n"));
        }
        out
    }

    /// A detected dxvk install inside a prefix.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DxvkInstall {
        pub system32: PathBuf,
        /// Which of DXVK_DLLS are present.
        pub found: Vec<String>,
    }

    impl DxvkInstall {
        pub fn is_complete(&self) -> bool {
            self.found.len() == DXVK_DLLS.len()
        }
    }

    /// Detects dxvk in `prefix/drive_c/windows/system32`; None if the dir
    /// does not exist.
    pub fn detect(prefix: &Path) -> Option<DxvkInstall> {
        let system32 = prefix.join("drive_c/windows/system32");
        if !system32.is_dir() {
            return None;
        }
        let found = DXVK_DLLS
            .iter()
            .filter(|dll| system32.join(dll).is_file())
            .map(|dll| dll.to_string())
            .collect();
        Some(DxvkInstall { system32, found })
    }

    /// Maps dxvk.conf keys to the DXVK_* env vars the native library reads:
    /// `dxvk.hud = 1` -> `DXVK_HUD=1` (`.` -> `_`, uppercased).
    pub fn env_from_config(cfg: &DxvkConfig) -> Vec<(String, String)> {
        cfg.entries
            .iter()
            .filter_map(|(k, v)| {
                k.strip_prefix("dxvk.").map(|rest| {
                    (
                        format!("DXVK_{}", rest.to_uppercase().replace('.', "_")),
                        v.clone(),
                    )
                })
            })
            .collect()
    }
}

pub mod vkd3d {
    //! vkd3d-proton (HansKristian-Work/vkd3d-proton) companion: detection.

    use std::path::{Path, PathBuf};

    /// DLLs vkd3d-proton installs into `system32`.
    pub const VKD3D_DLLS: [&str; 2] = ["d3d12.dll", "d3d12core.dll"];

    /// Detects vkd3d-proton in the prefix's system32; Some(dir) only when all
    /// DLLs are present, None otherwise.
    pub fn detect(prefix: &Path) -> Option<PathBuf> {
        let system32 = prefix.join("drive_c/windows/system32");
        if VKD3D_DLLS.iter().all(|dll| system32.join(dll).is_file()) {
            Some(system32)
        } else {
            None
        }
    }

    /// Extracts the version from the first `## vX.Y...` heading of a
    /// vkd3d-proton CHANGELOG.md.
    pub fn version_from_changelog(text: &str) -> Option<String> {
        text.lines()
            .find_map(|line| line.strip_prefix("## v").map(|v| v.trim().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::dxvk;
    use super::vkd3d;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_prefix() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("mpf-gfx-{nonce}"))
    }

    #[test]
    fn dxvk_conf_parse_format_roundtrip() {
        let text =
            "# comment\n\ndxvk.hud = 1\ndxvk.enableNVAPI = true\nmalformed line\n dxvk.async=on\n";
        let cfg = dxvk::parse_dxvk_conf(text);
        assert_eq!(cfg.entries.len(), 3);
        assert_eq!(cfg.entries["dxvk.hud"], "1");
        assert_eq!(cfg.entries["dxvk.async"], "on");
        let formatted = dxvk::format_dxvk_conf(&cfg);
        assert_eq!(dxvk::parse_dxvk_conf(&formatted), cfg);
        assert_eq!(dxvk::parse_dxvk_conf(""), dxvk::DxvkConfig::default());
    }

    #[test]
    fn dxvk_env_mapping() {
        let cfg = dxvk::parse_dxvk_conf("dxvk.hud = fps\ndxvk.enableNVAPI = 1\nother.key = x\n");
        let env = dxvk::env_from_config(&cfg);
        assert!(env.contains(&("DXVK_HUD".to_string(), "fps".to_string())));
        assert!(env.contains(&("DXVK_ENABLENVAPI".to_string(), "1".to_string())));
        assert_eq!(env.len(), 2); // non-dxvk keys are not mapped
    }

    #[test]
    fn dxvk_detect_in_prefix() {
        let prefix = temp_prefix();
        let sys32 = prefix.join("drive_c/windows/system32");
        assert!(dxvk::detect(&prefix).is_none()); // no system32 yet
        std::fs::create_dir_all(&sys32).unwrap();
        let install = dxvk::detect(&prefix).unwrap();
        assert!(install.found.is_empty());
        assert!(!install.is_complete());
        for dll in dxvk::DXVK_DLLS {
            std::fs::write(sys32.join(dll), b"fake").unwrap();
        }
        let install = dxvk::detect(&prefix).unwrap();
        assert!(install.is_complete());
        assert_eq!(install.found.len(), 5);
        std::fs::remove_dir_all(&prefix).unwrap();
    }

    #[test]
    fn vkd3d_detect_and_version() {
        let prefix = temp_prefix();
        assert!(vkd3d::detect(&prefix).is_none());
        let sys32 = prefix.join("drive_c/windows/system32");
        std::fs::create_dir_all(&sys32).unwrap();
        assert!(vkd3d::detect(&prefix).is_none()); // partial
        std::fs::write(sys32.join("d3d12.dll"), b"x").unwrap();
        assert!(vkd3d::detect(&prefix).is_none()); // still partial
        std::fs::write(sys32.join("d3d12core.dll"), b"x").unwrap();
        assert_eq!(vkd3d::detect(&prefix), Some(sys32.clone()));
        std::fs::remove_dir_all(&prefix).unwrap();

        assert_eq!(
            vkd3d::version_from_changelog("# Changelog\n\n## v3.0a\n\n- stuff\n\n## v2.14\n"),
            Some("3.0a".to_string())
        );
        assert_eq!(vkd3d::version_from_changelog("no headings"), None);
    }
}
