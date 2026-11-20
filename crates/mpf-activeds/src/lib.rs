//! Port of the deterministic logic in Wine's `dlls/activeds/pathname.c`: the `IADsPathname` state
//! for an `LDAP://` path, with `Set`, `Retrieve`, `GetNumElements` and `GetElement`.
//!
//! Ported:
//! - `parse_path`, the `LDAP:[//server[/dn]]` parser used by `path_Set` with `ADS_SETTYPE_FULL`.
//! - `path_Set` for the full, provider, server and dn types: [`Pathname::set`].
//! - `path_Retrieve` for each format: [`Pathname::retrieve`].
//! - `path_GetNumElements` and `path_GetElement`: [`Pathname::num_elements`] and
//!   [`Pathname::element`].
//!
//! Skipped:
//! - COM plumbing: the vtables, `QueryInterface`, reference counting, the class factory and
//!   `DllGetClassObject`.
//! - The FIXME stubs (`SetDisplayType`, `AddLeafElement`, `RemoveLeafElement`, `CopyPath`,
//!   `GetEscapedElement` and the escaped-mode accessors) and the IDispatch methods.
//!
//! Reference quirks that are kept:
//! - `Retrieve(ADS_FORMAT_SERVER)` with no server returns `E_OUTOFMEMORY`, because the reference
//!   calls `SysAllocString(NULL)`, which is NULL.
//! - An unknown format is read as `ADS_FORMAT_X500`.
//! - An empty dn has one element. `Set(ADS_SETTYPE_DN, "")` keeps an empty dn.
//! - A failed `Set(ADS_SETTYPE_FULL)` leaves the state unchanged.
//!
//! Divergences from C:
//! - A NULL argument is not representable in Rust, so the `E_INVALIDARG` checks for a NULL string
//!   or out pointer are not reproduced. The unknown-type `E_INVALIDARG` is kept.
//! - Out-of-memory paths are not reproduced.
//! - The provider is always set. The reference creates it as `"LDAP"`, and `Set` never clears it,
//!   so its NULL branches in `Retrieve` are unreachable and are not ported.
//! - Text is `&str`. The `LDAP:` prefix is matched ASCII-only, which agrees with the reference
//!   because no non-ASCII character folds to one of `L`, `D`, `A`, `P` or `:`.

/// `HRESULT`, as its 32-bit pattern.
pub type HResult = u32;

pub const S_OK: HResult = 0;
pub const E_OUTOFMEMORY: HResult = 0x8007_000E;
pub const E_INVALIDARG: HResult = 0x8007_0057;
pub const E_ADS_BAD_PATHNAME: HResult = 0x8000_5000;
/// `HRESULT_FROM_WIN32(ERROR_INVALID_INDEX)`.
pub const HRESULT_ERROR_INVALID_INDEX: HResult = 0x8007_0585;

pub const ADS_SETTYPE_FULL: i32 = 1;
pub const ADS_SETTYPE_PROVIDER: i32 = 2;
pub const ADS_SETTYPE_SERVER: i32 = 3;
pub const ADS_SETTYPE_DN: i32 = 4;

pub const ADS_FORMAT_X500: i32 = 5;
pub const ADS_FORMAT_X500_DN: i32 = 7;
pub const ADS_FORMAT_SERVER: i32 = 9;
pub const ADS_FORMAT_PROVIDER: i32 = 10;
pub const ADS_FORMAT_LEAF: i32 = 11;

/// The state of an `ADsPathname`: a provider, and optionally a server and a dn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pathname {
    provider: String,
    server: Option<String>,
    dn: Option<String>,
}

impl Default for Pathname {
    /// The state that `Pathname_create` gives a new object.
    fn default() -> Self {
        Self {
            provider: "LDAP".to_string(),
            server: None,
            dn: None,
        }
    }
}

impl Pathname {
    /// `path_Set`. A failed parse leaves the state as it was.
    pub fn set(&mut self, adspath: &str, kind: i32) -> HResult {
        match kind {
            ADS_SETTYPE_PROVIDER => {
                self.provider = adspath.to_string();
                S_OK
            }
            ADS_SETTYPE_SERVER => {
                self.server = Some(adspath.to_string());
                S_OK
            }
            ADS_SETTYPE_DN => {
                self.dn = Some(adspath.to_string());
                S_OK
            }
            ADS_SETTYPE_FULL => match parse_path(adspath) {
                Ok(parsed) => {
                    *self = parsed;
                    S_OK
                }
                Err(hr) => hr,
            },
            _ => E_INVALIDARG,
        }
    }

    /// `path_Retrieve`. The `Err` case is the reference's `E_OUTOFMEMORY` for a missing server.
    pub fn retrieve(&self, format: i32) -> Result<String, HResult> {
        match format {
            ADS_FORMAT_PROVIDER => Ok(self.provider.clone()),
            ADS_FORMAT_SERVER => self.server.clone().ok_or(E_OUTOFMEMORY),
            ADS_FORMAT_X500_DN => Ok(self.dn.clone().unwrap_or_default()),
            ADS_FORMAT_LEAF => Ok(self
                .dn
                .as_deref()
                .unwrap_or_default()
                .split(',')
                .next()
                .unwrap_or_default()
                .to_string()),
            // `ADS_FORMAT_X500`, and any format the reference does not implement.
            _ => Ok(self.x500()),
        }
    }

    /// `path_GetNumElements`: the commas in the dn plus one, or zero when there is no dn.
    pub fn num_elements(&self) -> i32 {
        match &self.dn {
            None => 0,
            Some(dn) => dn.split(',').count() as i32,
        }
    }

    /// `path_GetElement`: the `index`th comma-separated element of the dn.
    pub fn element(&self, index: i32) -> Result<String, HResult> {
        let dn = self.dn.as_deref().ok_or(HRESULT_ERROR_INVALID_INDEX)?;
        usize::try_from(index)
            .ok()
            .and_then(|i| dn.split(',').nth(i))
            .map(str::to_string)
            .ok_or(HRESULT_ERROR_INVALID_INDEX)
    }

    fn x500(&self) -> String {
        let mut out = format!("{}://", self.provider);
        if let Some(server) = &self.server {
            out.push_str(server);
            out.push('/');
        }
        if let Some(dn) = &self.dn {
            out.push_str(dn);
        }
        out
    }
}

/// `parse_path`: splits `LDAP:`, `LDAP://server` or `LDAP://server/dn`. The provider keeps the
/// case it was written in.
fn parse_path(path: &str) -> Result<Pathname, HResult> {
    let is_ldap = path
        .get(..5)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("LDAP:"));
    if !is_ldap {
        return Err(E_ADS_BAD_PATHNAME);
    }
    let provider = path[..4].to_string();
    let rest = &path[5..];
    if rest.is_empty() {
        return Ok(Pathname {
            provider,
            server: None,
            dn: None,
        });
    }
    let authority = rest
        .strip_prefix("//")
        .filter(|s| !s.is_empty())
        .ok_or(E_ADS_BAD_PATHNAME)?;
    let (server, dn) = match authority.split_once('/') {
        None => (authority, None),
        Some((server, dn)) if !dn.is_empty() => (server, Some(dn)),
        Some(_) => return Err(E_ADS_BAD_PATHNAME),
    };
    if server.is_empty() {
        return Err(E_ADS_BAD_PATHNAME);
    }
    Ok(Pathname {
        provider,
        server: Some(server.to_string()),
        dn: dn.map(str::to_string),
    })
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
