//! Partial `userenv` profile-path logic based on Wine's
//! `dlls/userenv/userenv_main.c`.
//!
//! Only the pure string and length computation is ported. Values that Wine
//! reads from the registry, a token, or the process environment are inputs
//! here, and results are returned as `Vec<u16>` instead of filling caller
//! buffers.
//!
//! Ported (3 of 23 exports):
//!
//! - `CreateEnvironmentBlock`: profile variables only (`USERPROFILE`,
//!   `APPDATA`, `LOCALAPPDATA`), see [`profile_vars`].
//! - `GetProfilesDirectoryW`: buffer rule after expansion, see
//!   [`profiles_directory`].
//! - `GetUserProfileDirectoryW`: `<profiles>\<account>` composition and
//!   sizing, see [`user_profile_directory`].
//!
//! Skipped (20 of 23 exports):
//!
//! - `CreateEnvironmentBlock` registry, token and environment-merge steps:
//!   need a live registry, token and process environment.
//! - `DestroyEnvironmentBlock`: frees an NT environment block.
//! - `ExpandEnvironmentStringsForUserA` and `ExpandEnvironmentStringsForUserW`:
//!   `%VAR%` expansion needs the live environment.
//! - `GetProfilesDirectoryA`: ANSI wrapper over registry access and codepages.
//! - `GetUserProfileDirectoryA`: ANSI wrapper over a token lookup.
//! - `GetAllUsersProfileDirectoryA` and `GetAllUsersProfileDirectoryW`: FIXME
//!   stubs that return FALSE.
//! - `GetDefaultUserProfileDirectoryA` and `GetDefaultUserProfileDirectoryW`:
//!   FIXME stubs that return FALSE.
//! - `GetProfileType`: FIXME stub; reports flags from a live process.
//! - `LoadUserProfileA`, `LoadUserProfileW` and `UnloadUserProfile`: load and
//!   unload registry hives.
//! - `EnterCriticalPolicySection`, `LeaveCriticalPolicySection`,
//!   `RegisterGPNotification`, `UnregisterGPNotification` and
//!   `GetAppliedGPOListW`: policy service and event objects.
//! - `CreateAppContainerProfile`: stub returning `E_NOTIMPL`.
//! - `USERENV_138`: undocumented shortcut helper; FIXME stub returning FALSE.
//!
//! Deviations from the C:
//!
//! - Registry values are read and expanded by the caller. An empty
//!   `ProfilesDirectory` is treated as no profiles directory, where Wine reads
//!   `profiles_dir[-1]`, which is undefined.
//! - [`user_profile_directory`] returns `Err(0)` for an empty profiles
//!   directory. Wine leaves `*lpcchSize` untouched on that path.
//! - Only the Default-profile length check is modeled. `MAX_PATH` truncation
//!   and other buffer limits are not.

/// Folder name of the default user profile.
const DEFAULT_PROFILE: &str = "Default";
const SYSTEM_PROFILE_SUFFIX: &str = r"\system32\config\systemprofile";
const LOCAL_SERVICE_SUFFIX: &str = r"\ServiceProfiles\LocalService";
const NETWORK_SERVICE_SUFFIX: &str = r"\ServiceProfiles\NetworkService";
const APPDATA_ROAMING_SUFFIX: &str = r"\AppData\Roaming";
const APPDATA_LOCAL_SUFFIX: &str = r"\AppData\Local";
/// Size in characters of the `buf` array in `CreateEnvironmentBlock`.
const UNICODE_STRING_MAX_CHARS: usize = 32767;
const BACKSLASH: u16 = b'\\' as u16;

/// Built-in account whose profile lives under the Windows directory.
pub enum ServiceAccount {
    LocalSystem,
    LocalService,
    NetworkService,
}

impl ServiceAccount {
    fn profile_suffix(self) -> &'static str {
        match self {
            Self::LocalSystem => SYSTEM_PROFILE_SUFFIX,
            Self::LocalService => LOCAL_SERVICE_SUFFIX,
            Self::NetworkService => NETWORK_SERVICE_SUFFIX,
        }
    }
}

/// Who the environment block is built for.
pub enum Principal<'a> {
    /// No token: the default user profile.
    Default,
    /// A built-in service account.
    Service(ServiceAccount),
    /// A user, identified by the account name from its token.
    User(&'a [u16]),
}

/// Profile variables set by `CreateEnvironmentBlock`. `None` means Wine
/// does not set that variable.
pub struct ProfileVars {
    pub userprofile: Option<Vec<u16>>,
    pub appdata: Option<Vec<u16>>,
    pub localappdata: Option<Vec<u16>>,
}

/// Computes the profile variables for `principal`. `profiles_dir` is the
/// registry `ProfilesDirectory` value after expansion, and `windows_dir` is
/// the Windows directory. Service accounts ignore `profiles_dir`.
pub fn profile_vars(
    profiles_dir: &[u16],
    windows_dir: &[u16],
    principal: Principal<'_>,
) -> ProfileVars {
    match principal {
        Principal::Default => ProfileVars {
            userprofile: default_profile(&with_trailing_backslash(profiles_dir)),
            appdata: None,
            localappdata: None,
        },
        Principal::Service(account) => {
            let root = join(windows_dir, &wide(account.profile_suffix()));
            ProfileVars {
                appdata: Some(join(&root, &wide(APPDATA_ROAMING_SUFFIX))),
                localappdata: Some(join(&root, &wide(APPDATA_LOCAL_SUFFIX))),
                userprofile: Some(root),
            }
        }
        Principal::User(account) => {
            let userprofile = if profiles_dir.is_empty() {
                None
            } else {
                Some(join(&with_trailing_backslash(profiles_dir), account))
            };
            ProfileVars {
                userprofile,
                appdata: None,
                localappdata: None,
            }
        }
    }
}

/// `GetProfilesDirectoryW` after `ExpandEnvironmentStringsW`. `capacity` is
/// the caller's buffer size in characters, or `None` for a NULL buffer.
/// Returns `Err` with the length Wine stores in `*lpcchSize` when the call
/// fails.
pub fn profiles_directory(expanded: &[u16], capacity: Option<usize>) -> Result<Vec<u16>, usize> {
    match capacity {
        Some(cap) if !expanded.is_empty() && cap >= expanded.len() => Ok(expanded.to_vec()),
        _ => Err(expanded.len()),
    }
}

/// `GetUserProfileDirectoryW` for an account in `profiles_dir`. `capacity`
/// is the caller's buffer size in characters, including the NUL, or `None`
/// for a NULL buffer. Returns `Err` with the required size, including the
/// NUL, when the call fails.
pub fn user_profile_directory(
    profiles_dir: &[u16],
    account: &[u16],
    capacity: Option<usize>,
) -> Result<Vec<u16>, usize> {
    if profiles_dir.is_empty() {
        return Err(0);
    }
    let mut path = profiles_dir.to_vec();
    path.push(BACKSLASH);
    path.extend_from_slice(account);
    let required = path.len() + 1;
    match capacity {
        Some(cap) if cap >= required => Ok(path),
        _ => Err(required),
    }
}

fn default_profile(profiles: &[u16]) -> Option<Vec<u16>> {
    // Mirrors `len * sizeof(WCHAR) + sizeof(L"Default") < sizeof(buf)`: the
    // `+ 1` is the NUL that `sizeof(L"Default")` includes.
    if profiles.is_empty() || profiles.len() + DEFAULT_PROFILE.len() + 1 >= UNICODE_STRING_MAX_CHARS
    {
        None
    } else {
        Some(join(profiles, &wide(DEFAULT_PROFILE)))
    }
}

fn with_trailing_backslash(dir: &[u16]) -> Vec<u16> {
    match dir.last() {
        None => Vec::new(),
        Some(&BACKSLASH) => dir.to_vec(),
        Some(_) => join(dir, &[BACKSLASH]),
    }
}

fn join(head: &[u16], tail: &[u16]) -> Vec<u16> {
    let mut out = head.to_vec();
    out.extend_from_slice(tail);
    out
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Who {
        Default,
        LocalSystem,
        LocalService,
        NetworkService,
        User(&'static str),
    }

    enum Call {
        Profile {
            profiles: &'static str,
            windir: &'static str,
            who: Who,
        },
        ProfilesDir {
            value: &'static str,
            cap: Option<usize>,
        },
        UserDir {
            profiles: &'static str,
            account: &'static str,
            cap: Option<usize>,
        },
    }

    /// `(ret, size, text)`. `ret` is TRUE for `Ok`. For `Ok`, `size` is what
    /// Wine stores in `*lpcchSize`, and `text` is the buffer contents. The
    /// environment-block row has no size output, so it uses 0 and carries the
    /// three profile variables in `text`.
    type Out = (bool, usize, [Option<String>; 3]);
    type Want = (bool, usize, [Option<&'static str>; 3]);

    struct Vector {
        name: &'static str,
        call: Call,
        want: Want,
    }

    const WINDIR: &str = r"C:\Windows";
    const SYSTEM_ROOT: &str = r"C:\Windows\system32\config\systemprofile";
    const LOCAL_SERVICE_ROOT: &str = r"C:\Windows\ServiceProfiles\LocalService";
    const NETWORK_SERVICE_ROOT: &str = r"C:\Windows\ServiceProfiles\NetworkService";

    // Vectors from the Wine oracle (`target/canon/userenv/oracle.txt`). The
    // inputs come from `scenarios.txt`; the expectations are the oracle output.
    const VECTORS: &[Vector] = &[
        Vector {
            name: "default_plain",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: WINDIR,
                who: Who::Default,
            },
            want: (true, 0, [Some(r"C:\Users\Default"), None, None]),
        },
        Vector {
            name: "default_trailing_sep",
            call: Call::Profile {
                profiles: r"C:\Users\",
                windir: WINDIR,
                who: Who::Default,
            },
            want: (true, 0, [Some(r"C:\Users\Default"), None, None]),
        },
        Vector {
            name: "default_drive_only",
            call: Call::Profile {
                profiles: "C:",
                windir: WINDIR,
                who: Who::Default,
            },
            want: (true, 0, [Some(r"C:\Default"), None, None]),
        },
        Vector {
            name: "default_root",
            call: Call::Profile {
                profiles: r"\",
                windir: WINDIR,
                who: Who::Default,
            },
            want: (true, 0, [Some(r"\Default"), None, None]),
        },
        Vector {
            name: "default_other_drive_spaces",
            call: Call::Profile {
                profiles: r"D:\Profiles Dir",
                windir: WINDIR,
                who: Who::Default,
            },
            want: (true, 0, [Some(r"D:\Profiles Dir\Default"), None, None]),
        },
        Vector {
            name: "default_missing_profiles",
            call: Call::Profile {
                profiles: "",
                windir: WINDIR,
                who: Who::Default,
            },
            want: (true, 0, [None, None, None]),
        },
        Vector {
            name: "system_basic",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: WINDIR,
                who: Who::LocalSystem,
            },
            want: (
                true,
                0,
                [
                    Some(SYSTEM_ROOT),
                    Some(r"C:\Windows\system32\config\systemprofile\AppData\Roaming"),
                    Some(r"C:\Windows\system32\config\systemprofile\AppData\Local"),
                ],
            ),
        },
        Vector {
            name: "system_windir_trailing_sep",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: r"C:\Windows\",
                who: Who::LocalSystem,
            },
            want: (
                true,
                0,
                [
                    Some(r"C:\Windows\\system32\config\systemprofile"),
                    Some(r"C:\Windows\\system32\config\systemprofile\AppData\Roaming"),
                    Some(r"C:\Windows\\system32\config\systemprofile\AppData\Local"),
                ],
            ),
        },
        Vector {
            name: "localservice",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: WINDIR,
                who: Who::LocalService,
            },
            want: (
                true,
                0,
                [
                    Some(LOCAL_SERVICE_ROOT),
                    Some(r"C:\Windows\ServiceProfiles\LocalService\AppData\Roaming"),
                    Some(r"C:\Windows\ServiceProfiles\LocalService\AppData\Local"),
                ],
            ),
        },
        Vector {
            name: "networkservice",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: WINDIR,
                who: Who::NetworkService,
            },
            want: (
                true,
                0,
                [
                    Some(NETWORK_SERVICE_ROOT),
                    Some(r"C:\Windows\ServiceProfiles\NetworkService\AppData\Roaming"),
                    Some(r"C:\Windows\ServiceProfiles\NetworkService\AppData\Local"),
                ],
            ),
        },
        Vector {
            name: "service_ignores_profiles",
            call: Call::Profile {
                profiles: r"D:\Profiles",
                windir: WINDIR,
                who: Who::LocalSystem,
            },
            want: (
                true,
                0,
                [
                    Some(SYSTEM_ROOT),
                    Some(r"C:\Windows\system32\config\systemprofile\AppData\Roaming"),
                    Some(r"C:\Windows\system32\config\systemprofile\AppData\Local"),
                ],
            ),
        },
        Vector {
            name: "user_basic",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: WINDIR,
                who: Who::User("zed"),
            },
            want: (true, 0, [Some(r"C:\Users\zed"), None, None]),
        },
        Vector {
            name: "user_trailing_sep",
            call: Call::Profile {
                profiles: r"C:\Users\",
                windir: WINDIR,
                who: Who::User("zed"),
            },
            want: (true, 0, [Some(r"C:\Users\zed"), None, None]),
        },
        Vector {
            name: "user_missing_profiles",
            call: Call::Profile {
                profiles: "",
                windir: WINDIR,
                who: Who::User("zed"),
            },
            want: (true, 0, [None, None, None]),
        },
        Vector {
            name: "user_account_with_space",
            call: Call::Profile {
                profiles: r"C:\Users",
                windir: WINDIR,
                who: Who::User("John Doe"),
            },
            want: (true, 0, [Some(r"C:\Users\John Doe"), None, None]),
        },
        Vector {
            name: "gpd_exact",
            call: Call::ProfilesDir {
                value: r"C:\Users",
                cap: Some(8),
            },
            want: (true, 8, [Some(r"C:\Users"), None, None]),
        },
        Vector {
            name: "gpd_short",
            call: Call::ProfilesDir {
                value: r"C:\Users",
                cap: Some(7),
            },
            want: (false, 8, [None, None, None]),
        },
        Vector {
            name: "gpd_null_buffer",
            call: Call::ProfilesDir {
                value: r"C:\Users",
                cap: None,
            },
            want: (false, 8, [None, None, None]),
        },
        Vector {
            name: "gpd_empty_value",
            call: Call::ProfilesDir {
                value: "",
                cap: Some(0),
            },
            want: (false, 0, [None, None, None]),
        },
        Vector {
            name: "gup_exact",
            call: Call::UserDir {
                profiles: r"C:\Users",
                account: "zed",
                cap: Some(13),
            },
            want: (true, 13, [Some(r"C:\Users\zed"), None, None]),
        },
        Vector {
            name: "gup_short",
            call: Call::UserDir {
                profiles: r"C:\Users",
                account: "zed",
                cap: Some(12),
            },
            want: (false, 13, [None, None, None]),
        },
        Vector {
            name: "gup_null_buffer",
            call: Call::UserDir {
                profiles: r"C:\Users",
                account: "zed",
                cap: None,
            },
            want: (false, 13, [None, None, None]),
        },
        Vector {
            name: "gup_trailing_sep",
            call: Call::UserDir {
                profiles: r"C:\Users\",
                account: "zed",
                cap: Some(14),
            },
            want: (true, 14, [Some(r"C:\Users\\zed"), None, None]),
        },
        Vector {
            name: "gup_empty_value",
            call: Call::UserDir {
                profiles: "",
                account: "zed",
                cap: Some(0),
            },
            want: (false, 0, [None, None, None]),
        },
    ];

    fn text(v: &[u16]) -> String {
        String::from_utf16_lossy(v)
    }

    fn opt(v: &Option<Vec<u16>>) -> Option<String> {
        v.as_deref().map(text)
    }

    fn observe(call: &Call) -> Out {
        match call {
            Call::Profile {
                profiles,
                windir,
                who,
            } => {
                let user = match who {
                    Who::User(name) => wide(name),
                    _ => Vec::new(),
                };
                let principal = match who {
                    Who::Default => Principal::Default,
                    Who::LocalSystem => Principal::Service(ServiceAccount::LocalSystem),
                    Who::LocalService => Principal::Service(ServiceAccount::LocalService),
                    Who::NetworkService => Principal::Service(ServiceAccount::NetworkService),
                    Who::User(_) => Principal::User(&user),
                };
                let vars = profile_vars(&wide(profiles), &wide(windir), principal);
                (
                    true,
                    0,
                    [
                        opt(&vars.userprofile),
                        opt(&vars.appdata),
                        opt(&vars.localappdata),
                    ],
                )
            }
            Call::ProfilesDir { value, cap } => match profiles_directory(&wide(value), *cap) {
                Ok(v) => (true, v.len(), [Some(text(&v)), None, None]),
                Err(n) => (false, n, [None, None, None]),
            },
            Call::UserDir {
                profiles,
                account,
                cap,
            } => match user_profile_directory(&wide(profiles), &wide(account), *cap) {
                Ok(v) => (true, v.len() + 1, [Some(text(&v)), None, None]),
                Err(n) => (false, n, [None, None, None]),
            },
        }
    }

    #[test]
    fn matches_wine_oracle_vectors() {
        assert_eq!(VECTORS.len(), 24);
        for v in VECTORS {
            let want = (v.want.0, v.want.1, v.want.2.map(|t| t.map(String::from)));
            assert_eq!(observe(&v.call), want, "{}", v.name);
        }
    }

    #[test]
    fn default_profile_respects_buffer_limit() {
        let fits = vec![u16::from(b'C'); 32757];
        let vars = profile_vars(&fits, &[], Principal::Default);
        assert_eq!(vars.userprofile.as_ref().map(Vec::len), Some(32765));

        let too_long = vec![u16::from(b'C'); 32758];
        let vars = profile_vars(&too_long, &[], Principal::Default);
        assert_eq!(vars.userprofile, None);
    }
}
