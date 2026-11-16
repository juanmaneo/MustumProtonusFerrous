//! advapi32 registry subset, mirroring `dlls/advapi32/registry.c`.
//!
//! Implements the wine `.reg` hive text format (as written by wineboot into
//! `system.reg` / `user.reg`) and a small Reg* key API over in-memory hives.
//! Hive files use the short root forms: `\Machine\...` in system.reg and
//! `\User\<sid>\...` in user.reg.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

pub const REG_HEADER: &str = "WINE REGISTRY Version 2\n";

/// Win32 error codes used by the Reg* API.
pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_INVALID_HANDLE: u32 = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegValue {
    String(String),
    Dword(u32),
}

/// A parsed hive: key path (short root form, e.g. `\Machine\Software\Wine`)
/// -> value name -> value.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Hive {
    pub keys: BTreeMap<String, BTreeMap<String, RegValue>>,
}

/// Escapes a .reg token: `\` becomes `\\` and `"` becomes `\"`.
pub fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Unescapes a .reg token, undoing `escape`: `\\` becomes `\` and `\"`
/// becomes `"`. Any other backslash is kept as it is.
pub fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, chars.peek()) {
            ('\\', Some(&next @ ('\\' | '"'))) => {
                out.push(next);
                chars.next();
            }
            _ => out.push(c),
        }
    }
    out
}

/// Splits a `"..."` token off the front of `s`: its unescaped text and what
/// follows the closing quote. The closing quote is the first one that is not
/// escaped. `None` if `s` does not start with a complete quoted string.
fn quoted(s: &str) -> Option<(String, &str)> {
    let body = s.strip_prefix('"')?;
    let mut escaped = false;
    let end = body.char_indices().find_map(|(i, c)| {
        let closes = c == '"' && !escaped;
        escaped = c == '\\' && !escaped;
        closes.then_some(i)
    })?;
    Some((unescape(&body[..end]), &body[end + 1..]))
}

fn parse_value_line(line: &str) -> Option<(String, RegValue)> {
    let (name, rest) = quoted(line)?;
    let rest = rest.strip_prefix('=')?;
    let value = match rest.strip_prefix("dword:") {
        Some(hex) => RegValue::Dword(u32::from_str_radix(hex.trim(), 16).ok()?),
        None => RegValue::String(quoted(rest)?.0),
    };
    Some((name, value))
}

/// Parses wine .reg hive text. Unknown lines are ignored, like wine's parser
/// tolerates extra whitespace/comments.
pub fn parse_hive(text: &str) -> Hive {
    let mut hive = Hive::default();
    // The values of the key being read: the last well-formed `[key]` line.
    let mut current: Option<&mut BTreeMap<String, RegValue>> = None;
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix('[') {
            if let Some(end) = rest.find(']') {
                current = Some(hive.keys.entry(unescape(&rest[..end])).or_default());
            }
            continue;
        }
        if let (Some(values), Some((name, value))) = (current.as_mut(), parse_value_line(line)) {
            values.insert(name, value);
        }
    }
    hive
}

/// Formats a hive back to .reg text; `parse_hive(format_hive(h)) == h`.
pub fn format_hive(hive: &Hive) -> String {
    let mut out = String::from(REG_HEADER);
    for (path, values) in &hive.keys {
        out.push_str(&format!("\n[{}] 1760000000\n", escape(path)));
        for (name, value) in values {
            match value {
                RegValue::String(s) => {
                    out.push_str(&format!("\"{}\"=\"{}\"\n", escape(name), escape(s)))
                }
                RegValue::Dword(d) => {
                    out.push_str(&format!("\"{}\"=dword:{:08x}\n", escape(name), d))
                }
            }
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RootKey {
    ClassesRoot,
    CurrentUser,
    LocalMachine,
    Users,
}

impl RootKey {
    /// Short root form used inside hive files.
    pub fn file_prefix(self) -> &'static str {
        match self {
            RootKey::ClassesRoot => "\\Classes",
            RootKey::CurrentUser => "\\User\\S-1-5-21-0-0-0-1000",
            RootKey::LocalMachine => "\\Machine",
            RootKey::Users => "\\Users",
        }
    }

    /// Full path for a subkey, e.g. `\Machine\Software\Wine`.
    pub fn key_path(self, subkey: &str) -> String {
        let sub = subkey.trim_matches('\\');
        if sub.is_empty() {
            self.file_prefix().to_string()
        } else {
            format!("{}\\{}", self.file_prefix(), sub)
        }
    }
}

pub struct Registry {
    pub hives: BTreeMap<RootKey, Hive>,
    open: BTreeMap<u64, (RootKey, String)>,
    next: u64,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            hives: BTreeMap::new(),
            open: BTreeMap::new(),
            next: 1,
        }
    }

    /// Loads a hive file for `root` (e.g. prefix's system.reg for HKLM),
    /// replacing any hive already loaded for it.
    pub fn load_hive(&mut self, root: RootKey, path: &Path) -> std::io::Result<()> {
        let text = std::fs::read_to_string(path)?;
        self.hives.insert(root, parse_hive(&text));
        Ok(())
    }

    /// Saves a hive back to disk in .reg format.
    pub fn save_hive(&self, root: RootKey, path: &Path) -> std::io::Result<()> {
        let hive = self.hives.get(&root).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "hive not loaded",
        ))?;
        std::fs::write(path, format_hive(hive))
    }

    /// RegOpenKeyExW: opens `subkey` under `root`; Err(ERROR_FILE_NOT_FOUND)
    /// if the key does not exist.
    pub fn reg_open_key_ex_w(&mut self, root: RootKey, subkey: &str) -> Result<u64, u32> {
        let path = root.key_path(subkey);
        let hive = self.hives.get(&root).ok_or(ERROR_FILE_NOT_FOUND)?;
        if hive.keys.contains_key(&path) {
            let id = self.next;
            self.next += 1;
            self.open.insert(id, (root, path));
            Ok(id)
        } else {
            Err(ERROR_FILE_NOT_FOUND)
        }
    }

    /// RegCreateKeyExW: creates the key if missing, then opens it.
    pub fn reg_create_key_ex_w(&mut self, root: RootKey, subkey: &str) -> Result<u64, u32> {
        let path = root.key_path(subkey);
        if let Some(hive) = self.hives.get_mut(&root) {
            hive.keys.entry(path.clone()).or_default();
        } else {
            let mut hive = Hive::default();
            hive.keys.entry(path.clone()).or_default();
            self.hives.insert(root, hive);
        }
        let id = self.next;
        self.next += 1;
        self.open.insert(id, (root, path));
        Ok(id)
    }

    /// RegQueryValueExW.
    pub fn reg_query_value_ex_w(&self, key: u64, name: &str) -> Result<RegValue, u32> {
        let (root, path) = self.open.get(&key).ok_or(ERROR_INVALID_HANDLE)?;
        self.hives
            .get(root)
            .and_then(|h| h.keys.get(path))
            .and_then(|values| values.get(name))
            .cloned()
            .ok_or(ERROR_FILE_NOT_FOUND)
    }

    /// RegSetValueExW (string or dword).
    pub fn reg_set_value_ex_w(&mut self, key: u64, name: &str, value: RegValue) -> u32 {
        let Some((root, path)) = self.open.get(&key).cloned() else {
            return ERROR_INVALID_HANDLE;
        };
        match self
            .hives
            .get_mut(&root)
            .and_then(|h| h.keys.get_mut(&path))
        {
            Some(values) => {
                values.insert(name.to_string(), value);
                ERROR_SUCCESS
            }
            None => ERROR_INVALID_HANDLE,
        }
    }

    /// RegCloseKey.
    pub fn reg_close_key(&mut self, key: u64) -> u32 {
        match self.open.remove(&key) {
            Some(_) => ERROR_SUCCESS,
            None => ERROR_INVALID_HANDLE,
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// The process-wide registry, loaded from a prefix.
pub fn global() -> &'static Mutex<Registry> {
    static REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}

/// Loads `prefix/drive_c/windows/system.reg` as HKLM and
/// `prefix/drive_c/users/mpf/user.reg` as HKCU into the global registry.
pub fn init_from_prefix(prefix: &Path) -> std::io::Result<()> {
    load_prefix(&mut global().lock().unwrap(), prefix)
}

fn load_prefix(reg: &mut Registry, prefix: &Path) -> std::io::Result<()> {
    reg.load_hive(
        RootKey::LocalMachine,
        &prefix.join("drive_c/windows/system.reg"),
    )?;
    reg.load_hive(
        RootKey::CurrentUser,
        &prefix.join("drive_c/users/mpf/user.reg"),
    )
}

/// Default prefix hive locations for tests and tooling.
pub fn default_hive_paths(prefix: &Path) -> [(RootKey, PathBuf); 2] {
    [
        (
            RootKey::LocalMachine,
            prefix.join("drive_c/windows/system.reg"),
        ),
        (
            RootKey::CurrentUser,
            prefix.join("drive_c/users/mpf/user.reg"),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    const SAMPLE: &str = "WINE REGISTRY Version 2\n\n[\\\\Machine\\\\Software\\\\Wine] 1760000000\n\"version\"=\"11.22\"\n\"flags\"=dword:0000002a\n\n[\\\\User\\\\S-1-5-21-0-0-0-1000\\\\Software] 1760000000\n\"name\"=\"mpf\"\n";

    fn temp_prefix() -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("mpf-advapi32-{nonce}-{n}"))
    }

    #[test]
    fn parses_hive_text() {
        let hive = parse_hive(SAMPLE);
        assert_eq!(hive.keys.len(), 2);
        let machine = &hive.keys["\\Machine\\Software\\Wine"];
        assert_eq!(machine["version"], RegValue::String("11.22".into()));
        assert_eq!(machine["flags"], RegValue::Dword(42));
        assert_eq!(
            hive.keys["\\User\\S-1-5-21-0-0-0-1000\\Software"]["name"],
            RegValue::String("mpf".into())
        );
    }

    #[test]
    fn format_parse_roundtrip() {
        let hive = parse_hive(SAMPLE);
        let text = format_hive(&hive);
        assert!(text.starts_with(REG_HEADER));
        assert_eq!(parse_hive(&text), hive);
    }

    #[test]
    fn escape_roundtrip() {
        assert_eq!(unescape(&escape("a\\b\\c")), "a\\b\\c");
        assert_eq!(escape("a\\b"), "a\\\\b");
        assert_eq!(escape("say \"hi\""), "say \\\"hi\\\"");
        for s in ["", "plain", "\\", "\"", "\\\"", "\"\\", "a\\\\\"b\"\\"] {
            assert_eq!(unescape(&escape(s)), s, "round-trip of {s:?}");
        }
    }

    #[test]
    fn unescape_keeps_backslashes_it_does_not_know() {
        assert_eq!(unescape("a\\nb"), "a\\nb");
        assert_eq!(unescape("trailing\\"), "trailing\\");
        assert_eq!(unescape("\\\\\\\""), "\\\"");
    }

    #[test]
    fn quoted_splits_off_one_token() {
        let cases: [(&str, Option<(&str, &str)>); 9] = [
            ("", None),
            ("abc", None),
            ("\"abc", None),
            ("\"\"", Some(("", ""))),
            ("\"a\"rest", Some(("a", "rest"))),
            ("\"a\\\\\"rest", Some(("a\\", "rest"))),
            ("\"a\\\"b\"x", Some(("a\"b", "x"))),
            ("\"a\\\"", None),
            ("\"a\\\\\\\"b\"", Some(("a\\\"b", ""))),
        ];
        for (input, want) in cases {
            let got = quoted(input).map(|(text, rest)| (text, rest.to_string()));
            let want = want.map(|(text, rest)| (text.to_string(), rest.to_string()));
            assert_eq!(got, want, "quoted({input:?})");
        }
    }

    #[test]
    fn parse_value_line_handles_string_and_dword() {
        assert_eq!(
            parse_value_line("\"n\"=\"v\""),
            Some(("n".into(), RegValue::String("v".into())))
        );
        assert_eq!(
            parse_value_line("\"n\"=dword:0000000a"),
            Some(("n".into(), RegValue::Dword(10)))
        );
        assert_eq!(
            parse_value_line("\"a\\\"b\"=\"say \\\"hi\\\"\""),
            Some(("a\"b".into(), RegValue::String("say \"hi\"".into())))
        );
    }

    #[test]
    fn parse_value_line_rejects_each_malformed_shape() {
        for line in [
            "not a value",    // no opening quote on the name
            "\"n",            // unterminated name
            "\"n\" x",        // no `=` after the name
            "\"n\"=dword:zz", // dword that is not hex
            "\"n\"=hex:zz",   // value type we do not support
            "\"n\"=\"abc",    // unterminated string value
        ] {
            assert_eq!(parse_value_line(line), None, "{line:?}");
        }
    }

    #[test]
    fn parse_hive_tolerates_noise() {
        let text = concat!(
            "WINE REGISTRY Version 2\r\n",
            "\"early\"=\"before any key\"\r\n",
            "# a comment\r\n",
            "\r\n",
            "[\\\\A] 1\r\n",
            "\"k\"=\"v\"\r\n",
            "garbage after a value\r\n",
            "[unclosed\r\n",
            "\"k2\"=\"v2\"\r\n",
        );
        let hive = parse_hive(text);
        // A `[` line without `]` is not a key: its values stay with the
        // previous key, and values before any key are dropped.
        assert_eq!(hive.keys.len(), 1);
        let a = &hive.keys["\\A"];
        assert_eq!(a["k"], RegValue::String("v".into()));
        assert_eq!(a["k2"], RegValue::String("v2".into()));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn hive_with_quotes_and_backslashes_roundtrips() {
        let mut hive = Hive::default();
        hive.keys.insert(
            "\\Machine\\Say \"hi\"".into(),
            BTreeMap::from([
                (
                    "na\"me\\".into(),
                    RegValue::String("C:\\dir\\\"quoted\"\\".into()),
                ),
                ("dw".into(), RegValue::Dword(u32::MAX)),
            ]),
        );
        assert_eq!(parse_hive(&format_hive(&hive)), hive);
    }

    #[test]
    fn registry_default_is_empty() {
        let reg = Registry::default();
        assert!(reg.hives.is_empty());
    }

    #[test]
    fn load_hive_reports_a_missing_file() {
        let mut reg = Registry::new();
        let missing = temp_prefix().join("missing.reg");
        assert!(reg.load_hive(RootKey::LocalMachine, &missing).is_err());
        assert!(reg.hives.is_empty());
    }

    #[test]
    fn replacing_a_hive_invalidates_open_keys() {
        let mut reg = Registry::new();
        let key = reg
            .reg_create_key_ex_w(RootKey::LocalMachine, "Software\\Gone")
            .unwrap();
        reg.hives.insert(RootKey::LocalMachine, Hive::default());
        assert_eq!(
            reg.reg_set_value_ex_w(key, "x", RegValue::Dword(1)),
            ERROR_INVALID_HANDLE
        );
        assert_eq!(
            reg.reg_query_value_ex_w(key, "x"),
            Err(ERROR_FILE_NOT_FOUND)
        );
        reg.hives.clear();
        assert_eq!(
            reg.reg_set_value_ex_w(key, "x", RegValue::Dword(1)),
            ERROR_INVALID_HANDLE
        );
    }

    #[test]
    fn load_prefix_needs_both_hives() {
        let prefix = temp_prefix();
        let system = prefix.join("drive_c/windows/system.reg");
        let user = prefix.join("drive_c/users/mpf/user.reg");
        let mut reg = Registry::new();

        assert!(load_prefix(&mut reg, &prefix).is_err(), "no system.reg");

        std::fs::create_dir_all(system.parent().unwrap()).unwrap();
        std::fs::write(&system, SAMPLE).unwrap();
        assert!(load_prefix(&mut reg, &prefix).is_err(), "no user.reg");

        std::fs::create_dir_all(user.parent().unwrap()).unwrap();
        std::fs::write(&user, SAMPLE).unwrap();
        load_prefix(&mut reg, &prefix).unwrap();
        assert!(reg.hives.contains_key(&RootKey::LocalMachine));
        assert!(reg.hives.contains_key(&RootKey::CurrentUser));

        std::fs::remove_dir_all(&prefix).unwrap();
    }

    #[test]
    fn key_path_joins_and_trims() {
        assert_eq!(
            RootKey::LocalMachine.key_path("Software\\Wine"),
            "\\Machine\\Software\\Wine"
        );
        assert_eq!(RootKey::LocalMachine.key_path(""), "\\Machine");
        assert_eq!(
            RootKey::LocalMachine.key_path("\\Software\\"),
            "\\Machine\\Software"
        );
        assert_eq!(
            RootKey::CurrentUser.file_prefix(),
            "\\User\\S-1-5-21-0-0-0-1000"
        );
        assert_eq!(RootKey::ClassesRoot.file_prefix(), "\\Classes");
        assert_eq!(RootKey::Users.file_prefix(), "\\Users");
    }

    #[test]
    fn reg_api_open_query_set_create_close() {
        let mut reg = Registry::new();
        let mut hive = Hive::default();
        hive.keys.insert(
            "\\Machine\\Software\\Wine".into(),
            BTreeMap::from([("version".into(), RegValue::String("11.22".into()))]),
        );
        reg.hives.insert(RootKey::LocalMachine, hive);

        let key = reg
            .reg_open_key_ex_w(RootKey::LocalMachine, "Software\\Wine")
            .unwrap();
        assert_eq!(
            reg.reg_query_value_ex_w(key, "version").unwrap(),
            RegValue::String("11.22".into())
        );
        assert_eq!(
            reg.reg_query_value_ex_w(key, "missing"),
            Err(ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(
            reg.reg_query_value_ex_w(9999, "version"),
            Err(ERROR_INVALID_HANDLE)
        );

        assert_eq!(
            reg.reg_set_value_ex_w(key, "flags", RegValue::Dword(7)),
            ERROR_SUCCESS
        );
        assert_eq!(
            reg.reg_set_value_ex_w(9999, "x", RegValue::Dword(1)),
            ERROR_INVALID_HANDLE
        );
        assert_eq!(
            reg.reg_query_value_ex_w(key, "flags").unwrap(),
            RegValue::Dword(7)
        );

        let created = reg
            .reg_create_key_ex_w(RootKey::LocalMachine, "Software\\New")
            .unwrap();
        assert_eq!(
            reg.reg_query_value_ex_w(created, "anything"),
            Err(ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(reg.reg_close_key(key), ERROR_SUCCESS);
        assert_eq!(reg.reg_close_key(key), ERROR_INVALID_HANDLE);

        assert_eq!(
            reg.reg_open_key_ex_w(RootKey::LocalMachine, "Software\\Nope"),
            Err(ERROR_FILE_NOT_FOUND)
        );
        assert_eq!(
            reg.reg_open_key_ex_w(RootKey::Users, "anything"),
            Err(ERROR_FILE_NOT_FOUND)
        );
        // create on an unloaded root still works (fresh hive)
        let k2 = reg.reg_create_key_ex_w(RootKey::Users, "X").unwrap();
        assert!(reg.hives.contains_key(&RootKey::Users));
        assert_eq!(reg.reg_close_key(k2), ERROR_SUCCESS);
    }

    #[test]
    fn load_save_and_init_from_prefix() {
        let prefix = temp_prefix();
        let system = prefix.join("drive_c/windows/system.reg");
        let user = prefix.join("drive_c/users/mpf/user.reg");
        std::fs::create_dir_all(system.parent().unwrap()).unwrap();
        std::fs::create_dir_all(user.parent().unwrap()).unwrap();
        std::fs::write(&system, SAMPLE).unwrap();
        std::fs::write(&user, SAMPLE).unwrap();

        let mut reg = Registry::new();
        reg.load_hive(RootKey::LocalMachine, &system).unwrap();
        reg.save_hive(RootKey::LocalMachine, &system).unwrap();
        let reloaded = parse_hive(&std::fs::read_to_string(&system).unwrap());
        assert_eq!(reloaded.keys.len(), 2);
        assert!(reg.save_hive(RootKey::CurrentUser, &user).is_err());

        init_from_prefix(&prefix).unwrap();
        let mut global = global().lock().unwrap();
        let key = global
            .reg_open_key_ex_w(RootKey::LocalMachine, "Software\\Wine")
            .unwrap();
        assert_eq!(
            global.reg_query_value_ex_w(key, "version").unwrap(),
            RegValue::String("11.22".into())
        );
        drop(global);
        std::fs::remove_dir_all(&prefix).unwrap();
    }

    #[test]
    fn default_hive_paths_layout() {
        let paths = default_hive_paths(Path::new("/prefix"));
        assert_eq!(
            paths[0].1,
            PathBuf::from("/prefix/drive_c/windows/system.reg")
        );
        assert_eq!(
            paths[1].1,
            PathBuf::from("/prefix/drive_c/users/mpf/user.reg")
        );
        assert_eq!(paths[0].0, RootKey::LocalMachine);
        assert_eq!(paths[1].0, RootKey::CurrentUser);
    }
}
