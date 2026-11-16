//! Prefix bootstrap — the `programs/wineboot` equivalent for MPF.
//!
//! Creates the `drive_c` tree and the initial registry hives (`system.reg`,
//! `user.reg`) in wine's .reg text format, mirroring what wineboot does on
//! first run (see `repos/wine/programs/wineboot/`).

use mpf_advapi32::{format_hive, Hive, RegValue, RootKey};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub use mpf_advapi32::REG_HEADER;

#[derive(Debug, Clone)]
pub struct Prefix {
    pub root: PathBuf,
}

impl Prefix {
    pub fn drive_c(&self) -> PathBuf {
        self.root.join("drive_c")
    }

    pub fn system_reg(&self) -> PathBuf {
        self.drive_c().join("windows").join("system.reg")
    }

    pub fn user_reg(&self) -> PathBuf {
        self.drive_c().join("users").join("mpf").join("user.reg")
    }
}

/// The initial HKLM hive content.
pub fn system_hive() -> Hive {
    let mut hive = Hive::default();
    let mpf = hive
        .keys
        .entry(RootKey::LocalMachine.key_path("Software\\Wine\\MPF"))
        .or_default();
    mpf.insert("version".into(), RegValue::String("11.22".into()));
    let win = hive
        .keys
        .entry(RootKey::LocalMachine.key_path(
            "System\\CurrentControlSet\\Control\\Windows",
        ))
        .or_default();
    win.insert(
        "SystemDirectory".into(),
        RegValue::String("C:\\windows\\system32".into()),
    );
    hive
}

/// The initial HKCU hive content.
pub fn user_hive() -> Hive {
    let mut hive = Hive::default();
    let mpf = hive
        .keys
        .entry(RootKey::CurrentUser.key_path("Software\\Wine\\MPF"))
        .or_default();
    mpf.insert("prefix".into(), RegValue::String("C:\\users\\mpf".into()));
    hive
}

/// Creates (or reuses) a prefix at `root`: the `drive_c` directory tree plus
/// initial `system.reg` / `user.reg` hives. Idempotent, like wineboot `-i`.
pub fn init_prefix(root: &Path) -> io::Result<Prefix> {
    let drive_c = root.join("drive_c");
    let dirs = [
        drive_c.join("windows").join("system32"),
        drive_c.join("windows").join("syswow64"),
        drive_c.join("windows").join("fonts"),
        drive_c.join("users").join("mpf").join("AppData").join("Local"),
        drive_c.join("users").join("mpf").join("AppData").join("Roaming"),
        drive_c.join("Program Files"),
        drive_c.join("Program Files (x86)"),
    ];
    for d in &dirs {
        fs::create_dir_all(d)?;
    }
    let system_reg = drive_c.join("windows").join("system.reg");
    if !system_reg.exists() {
        fs::write(&system_reg, format_hive(&system_hive()))?;
    }
    let user_reg = drive_c.join("users").join("mpf").join("user.reg");
    if !user_reg.exists() {
        fs::write(&user_reg, format_hive(&user_hive()))?;
    }
    Ok(Prefix {
        root: root.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_root() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("mpf-prefix-{nonce}-{n}"))
    }

    #[test]
    fn creates_drive_c_tree_and_registry_hives() {
        let root = temp_root();
        let prefix = init_prefix(&root).unwrap();
        assert!(prefix.drive_c().join("windows/system32").is_dir());
        assert!(prefix.drive_c().join("users/mpf/AppData/Local").is_dir());
        let system = fs::read_to_string(prefix.system_reg()).unwrap();
        assert!(system.starts_with(REG_HEADER));
        assert!(system.contains("11.22"));
        let hive = mpf_advapi32::parse_hive(&system);
        assert_eq!(
            hive.keys["\\Machine\\Software\\Wine\\MPF"]["version"],
            RegValue::String("11.22".into())
        );
        let user = fs::read_to_string(prefix.user_reg()).unwrap();
        assert!(user.starts_with(REG_HEADER));
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn init_is_idempotent() {
        let root = temp_root();
        init_prefix(&root).unwrap();
        let before = fs::read_to_string(root.join("drive_c/windows/system.reg")).unwrap();
        init_prefix(&root).unwrap();
        let after = fs::read_to_string(root.join("drive_c/windows/system.reg")).unwrap();
        assert_eq!(before, after);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn hives_have_expected_keys() {
        let sys = system_hive();
        assert!(sys.keys.contains_key("\\Machine\\Software\\Wine\\MPF"));
        assert!(sys
            .keys
            .contains_key("\\Machine\\System\\CurrentControlSet\\Control\\Windows"));
        let user = user_hive();
        assert!(user
            .keys
            .contains_key("\\User\\S-1-5-21-0-0-0-1000\\Software\\Wine\\MPF"));
    }
}
