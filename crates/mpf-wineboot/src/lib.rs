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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    /// A fresh path under the temp dir. The counter keeps tests running in
    /// parallel (or back to back within one clock tick) from sharing a path.
    fn temp_root() -> PathBuf {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("mpf-prefix-{nonce}-{n}"))
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
    fn fails_when_the_prefix_root_is_a_file() {
        let root = temp_root();
        fs::write(&root, b"not a directory").unwrap();
        assert!(init_prefix(&root).is_err());
        fs::remove_file(&root).unwrap();
    }

    /// A symlink to a missing directory: `exists()` is false, but writing
    /// through it fails.
    #[cfg(unix)]
    fn plant_dangling_symlink(at: &Path) {
        std::os::unix::fs::symlink("/nonexistent-mpf-dir/hive.reg", at).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn reports_registry_hive_write_failures() {
        // system.reg cannot be written.
        let root = temp_root();
        let prefix = init_prefix(&root).unwrap();
        fs::remove_file(prefix.system_reg()).unwrap();
        plant_dangling_symlink(&prefix.system_reg());
        assert!(init_prefix(&root).is_err());
        fs::remove_dir_all(&root).unwrap();

        // user.reg cannot be written (system.reg already exists).
        let root = temp_root();
        let prefix = init_prefix(&root).unwrap();
        fs::remove_file(prefix.user_reg()).unwrap();
        plant_dangling_symlink(&prefix.user_reg());
        assert!(init_prefix(&root).is_err());
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
