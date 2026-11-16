//! Prefix bootstrap — the `programs/wineboot` equivalent for MPF.
//!
//! Creates the `drive_c` tree and the initial registry hive stubs inside a
//! prefix directory, mirroring what wineboot does on first run
//! (see `repos/wine/programs/wineboot/`). Registry content is a minimal
//! text-format stub; the real hive format lands with the registry crate.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const REG_HEADER: &str = "WINE REGISTRY Version 2\n";

const SYSTEM_REG_KEYS: &str = "[\\\\Machine\\\\Software\\\\Wine\\\\MPF] 1760000000\n\"version\"=\"11.22\"\n\n[\\\\Machine\\\\System\\\\CurrentControlSet\\\\Control\\\\Windows] 1760000000\n\"SystemDirectory\"=\"C:\\\\windows\\\\system32\"\n";

const USER_REG_KEYS: &str = "[\\\\User\\\\S-1-5-21-0-0-0-1000\\\\Software\\\\Wine\\\\MPF] 1760000000\n\"prefix\"=\"C:\\\\users\\\\mpf\"\n";

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

/// Creates (or reuses) a prefix at `root`: the `drive_c` directory tree plus
/// initial `system.reg` / `user.reg` hive stubs. Idempotent, like wineboot
/// `-i`.
pub fn init_prefix(root: &Path) -> io::Result<Prefix> {
    let drive_c = root.join("drive_c");
    let dirs = [
        drive_c.join("windows").join("system32"),
        drive_c.join("windows").join("syswow64"),
        drive_c.join("windows").join("fonts"),
        drive_c
            .join("users")
            .join("mpf")
            .join("AppData")
            .join("Local"),
        drive_c
            .join("users")
            .join("mpf")
            .join("AppData")
            .join("Roaming"),
        drive_c.join("Program Files"),
        drive_c.join("Program Files (x86)"),
    ];
    for d in &dirs {
        fs::create_dir_all(d)?;
    }
    let system_reg = drive_c.join("windows").join("system.reg");
    if !system_reg.exists() {
        let mut content = String::from(REG_HEADER);
        content.push_str(SYSTEM_REG_KEYS);
        fs::write(&system_reg, content)?;
    }
    let user_reg = drive_c.join("users").join("mpf").join("user.reg");
    if !user_reg.exists() {
        let mut content = String::from(REG_HEADER);
        content.push_str(USER_REG_KEYS);
        fs::write(&user_reg, content)?;
    }
    Ok(Prefix {
        root: root.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("mpf-prefix-{nonce}"))
    }

    #[test]
    fn creates_drive_c_tree_and_registry_stubs() {
        let root = temp_root();
        let prefix = init_prefix(&root).unwrap();
        assert!(prefix.drive_c().join("windows/system32").is_dir());
        assert!(prefix.drive_c().join("users/mpf/AppData/Local").is_dir());
        let system = fs::read_to_string(prefix.system_reg()).unwrap();
        assert!(system.starts_with(REG_HEADER));
        assert!(system.contains("11.22"));
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
}
