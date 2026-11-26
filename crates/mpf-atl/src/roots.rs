use crate::text::eq_fold_str;

/// The predefined root keys a script can name.
pub enum RootKey {
    ClassesRoot,
    CurrentUser,
    LocalMachine,
    Users,
    PerformanceData,
    DynData,
    CurrentConfig,
}

const ROOT_KEYS: [(&str, RootKey); 14] = [
    ("HKEY_CLASSES_ROOT", RootKey::ClassesRoot),
    ("HKEY_CURRENT_USER", RootKey::CurrentUser),
    ("HKEY_LOCAL_MACHINE", RootKey::LocalMachine),
    ("HKEY_USERS", RootKey::Users),
    ("HKEY_PERFORMANCE_DATA", RootKey::PerformanceData),
    ("HKEY_DYN_DATA", RootKey::DynData),
    ("HKEY_CURRENT_CONFIG", RootKey::CurrentConfig),
    ("HKCR", RootKey::ClassesRoot),
    ("HKCU", RootKey::CurrentUser),
    ("HKLM", RootKey::LocalMachine),
    ("HKU", RootKey::Users),
    ("HKPD", RootKey::PerformanceData),
    ("HKDD", RootKey::DynData),
    ("HKCC", RootKey::CurrentConfig),
];

/// `do_process_root_key`'s lookup: the whole word, case-insensitively.
pub(crate) fn find_root(word: &[u16]) -> Option<RootKey> {
    ROOT_KEYS
        .into_iter()
        .find(|&(name, _)| eq_fold_str(word, name))
        .map(|(_, key)| key)
}
