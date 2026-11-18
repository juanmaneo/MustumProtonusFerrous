//! ntdsapi subset for MPF: the headless service principal name (SPN) builders.
//!
//! Strings are NUL-terminated UTF-16 slices, as the Win32 API uses. Each
//! optional pointer becomes an `Option<&[u16]>`, and the output buffer is a
//! `&mut [u16]` whose length is the capacity. Input strings are truncated at
//! their first NUL, as `lstrlenW` does.
//!
//! Ported from `dlls/ntdsapi/ntdsapi.c` (2 of 96 exports in `ntdsapi.spec`):
//! - `DsMakeSpnW` as `ds_make_spn_w`: builds `class/host[:port][/instance][/referrer]`.
//!   The referrer is kept only when the host is an IP literal, which the private
//!   parsers reproduce from `RtlIpv4StringToAddressExW` (strict) and
//!   `RtlIpv6StringToAddressExW` in `dlls/ntdll/rtlstr.c`.
//! - `DsClientMakeSpnForTargetServerW` as `ds_client_make_spn_for_target_server_w`:
//!   builds `class/name`.
//!
//! Skipped:
//! - `DsMakeSpnA`: converts through the active ANSI code page (`CP_ACP`), which depends on the locale.
//! - `DsBindA`, `DsBindW`, `DsCrackNamesA`, `DsCrackNamesW`, `DsGetSpnA`,
//!   `DsServerRegisterSpnA`, `DsServerRegisterSpnW`: reference Wine stubs that return
//!   `ERROR_CALL_NOT_IMPLEMENTED`; real behavior needs a directory service.
//! - 86 remaining exports, all reference Wine stubs (`ERROR_CALL_NOT_IMPLEMENTED`)
//!   that need a live directory service, RPC, or the registry:
//!   - Directory binding and session calls: `DsBindWithCredA/W`, `DsBindWithSpnA/W`,
//!     `DsUnBindA/W`, `DsMakePasswordCredentialsA/W`, `DsFreePasswordCredentials`,
//!     `DsaopBind`, `DsaopBindWithCred`, `DsaopBindWithSpn`, `DsaopUnBind`,
//!     `DsaopPrepareScript`, `DsaopExecuteScript`.
//!   - Replication: `DsReplicaAddA/W`, `DsReplicaDelA/W`, `DsReplicaModifyA/W`,
//!     `DsReplicaSyncA/W`, `DsReplicaSyncAllA/W`, `DsReplicaUpdateRefsA/W`,
//!     `DsReplicaVerifyObjectsA/W`, `DsReplicaConsistencyCheck`, `DsReplicaFreeInfo`,
//!     `DsReplicaGetInfoW`, `DsReplicaGetInfo2W`.
//!   - Site, server, domain, and role queries: `DsListSitesA/W`, `DsListServersInSiteA/W`,
//!     `DsListDomainsInSiteA/W`, `DsListServersForDomainInSiteA/W`, `DsListInfoForServerA/W`,
//!     `DsListRolesA/W`, `DsGetDomainControllerInfoA/W`, `DsFreeDomainControllerInfoA/W`,
//!     `DsRemoveDsDomainA/W`, `DsRemoveDsServerA/W`, `DsWriteAccountSpnA/W`,
//!     `DsAddSidHistoryA/W`, `DsInheritSecurityIdentityA/W`, `DsLogEntry`.
//!   - Name, SPN, and schema helpers: `DsCrackSpnA/W`, `DsCrackSpn2A/W`, `DsCrackSpn3W`,
//!     `DsGetSpnW`, `DsCrackUnquotedMangledRdnA/W`, `DsMapSchemaGuidsA/W`,
//!     `DsFreeSchemaGuidMapA/W`, `DsFreeNameResultA/W`, `DsFreeSpnArrayA/W`, `DsGetRdnW`,
//!     `DsClientMakeSpnForTargetServerA`, `DsIsMangledDnA/W`, `DsIsMangledRdnValueA/W`,
//!     `DsQuoteRdnValueA/W`, `DsUnquoteRdnValueA/W`.
//!     Several of these (the quoting, unquoting, mangling, and SPN-cracking helpers)
//!     are documented as pure string helpers. They are stubs in this reference tree, so
//!     there is nothing to port from it yet; this is a follow-up.

const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_BUFFER_OVERFLOW: u32 = 111;

const SLASH: u16 = b'/' as u16;
const COLON: u16 = b':' as u16;
const DOT: u16 = b'.' as u16;
const PERCENT: u16 = b'%' as u16;
const LBRACKET: u16 = b'[' as u16;
const RBRACKET: u16 = b']' as u16;
const ZERO: u16 = b'0' as u16;
const NINE: u16 = b'9' as u16;
const LOWER_X: u16 = b'x' as u16;
const UPPER_X: u16 = b'X' as u16;

/// Failure modes of the ported entry points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DsError {
    /// A required string pointer was NULL.
    InvalidParameter,
    /// The output buffer is too small. `required` counts the terminating NUL.
    BufferOverflow { required: usize },
}

impl DsError {
    /// The Win32 error code the C entry point returns for this failure.
    pub fn code(self) -> u32 {
        match self {
            Self::InvalidParameter => ERROR_INVALID_PARAMETER,
            Self::BufferOverflow { .. } => ERROR_BUFFER_OVERFLOW,
        }
    }
}

/// Builds `class/host[:port][/instance][/referrer]` into `spn`, as `DsMakeSpnW`.
///
/// `inst_port` of 0 means no port. The `inst_name` is used in place of `svc_name`
/// as the host, with `svc_name` appended after it. `reference` is dropped unless
/// `svc_name` is an IP literal. On success returns the length in units including
/// the terminating NUL. On `BufferOverflow` returns the required length and leaves
/// `spn` untouched.
pub fn ds_make_spn_w(
    svc_class: Option<&[u16]>,
    svc_name: Option<&[u16]>,
    inst_name: Option<&[u16]>,
    inst_port: u16,
    reference: Option<&[u16]>,
    spn: &mut [u16],
) -> Result<usize, DsError> {
    let (Some(class), Some(name)) = (svc_class.map(wstr), svc_name.map(wstr)) else {
        return Err(DsError::InvalidParameter);
    };
    let inst = inst_name.map(wstr);
    let reference = reference.map(wstr).filter(|_| is_ip_literal(name));

    let mut out = class.to_vec();
    out.push(SLASH);
    out.extend_from_slice(inst.unwrap_or(name));
    if inst_port != 0 {
        out.push(COLON);
        out.extend(inst_port.to_string().encode_utf16());
    }
    if inst.is_some() {
        out.push(SLASH);
        out.extend_from_slice(name);
    }
    if let Some(reference) = reference {
        out.push(SLASH);
        out.extend_from_slice(reference);
    }
    store(&out, spn)
}

/// Builds `class/name` into `buf`, as `DsClientMakeSpnForTargetServerW`.
///
/// Returns the length in units including the terminating NUL, or
/// `BufferOverflow` with the required length, leaving `buf` untouched.
pub fn ds_client_make_spn_for_target_server_w(
    class: Option<&[u16]>,
    name: Option<&[u16]>,
    buf: &mut [u16],
) -> Result<usize, DsError> {
    let (Some(class), Some(name)) = (class.map(wstr), name.map(wstr)) else {
        return Err(DsError::InvalidParameter);
    };
    let mut out = class.to_vec();
    out.push(SLASH);
    out.extend_from_slice(name);
    store(&out, buf)
}

/// Copies `units` and a terminating NUL into `buf`, or reports the size needed.
fn store(units: &[u16], buf: &mut [u16]) -> Result<usize, DsError> {
    let required = units.len() + 1;
    if buf.len() < required {
        return Err(DsError::BufferOverflow { required });
    }
    buf[..units.len()].copy_from_slice(units);
    buf[units.len()] = 0;
    Ok(required)
}

/// The string up to its first NUL, as the C string functions see it.
fn wstr(s: &[u16]) -> &[u16] {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..len]
}

/// The code unit at `i`, or NUL past the end, so the parsers can probe ahead.
fn at(s: &[u16], i: usize) -> u16 {
    s.get(i).copied().unwrap_or(0)
}

/// The value of `c` as a hex digit, or `None` for any other unit.
fn hex_value(c: u16) -> Option<u32> {
    u8::try_from(c)
        .ok()
        .and_then(|b| char::from(b).to_digit(16))
}

/// Whether `svc_name` is an IP literal that `RtlIpv4StringToAddressExW` (strict)
/// or `RtlIpv6StringToAddressExW` accepts.
fn is_ip_literal(s: &[u16]) -> bool {
    ipv4_literal(s).is_some() || ipv6_literal(s).is_some()
}

/// One field of a dotted IPv4 address, from `parse_ipv4_component`. Advances
/// `pos` past the digits. A `0x` prefix means hex and a leading zero means
/// octal, but `strict` rejects both forms. `None` on no digits or overflow.
fn ipv4_component(s: &[u16], pos: &mut usize, strict: bool) -> Option<u32> {
    let mut base = 10;
    if at(s, *pos) == ZERO {
        let next = at(s, *pos + 1);
        if next == LOWER_X || next == UPPER_X {
            if strict {
                return None;
            }
            *pos += 2;
            base = 16;
        } else if (ZERO..=NINE).contains(&next) {
            if strict {
                return None;
            }
            *pos += 1;
            base = 8;
        }
    }

    let first = *pos;
    let mut value: u32 = 0;
    while let Some(digit) = hex_value(at(s, *pos)).filter(|&d| d < base) {
        let next = value.wrapping_mul(base).wrapping_add(digit);
        if next < value {
            return None;
        }
        value = next;
        *pos += 1;
    }
    (*pos > first).then_some(value)
}

/// `RtlIpv4StringToAddressExW` in strict mode, with the port allowed as in
/// `ipv4_string_to_address`: exactly four fields, each at most 255, then an
/// optional `:port` in 1..=65535, then the end of the string.
fn ipv4_literal(s: &[u16]) -> Option<()> {
    let mut pos = 0;
    let mut fields = [0u32; 4];
    let mut n = 0;
    loop {
        fields[n] = ipv4_component(s, &mut pos, true)?;
        n += 1;
        if at(s, pos) != DOT {
            break;
        }
        if n == 4 {
            return None;
        }
        pos += 1;
    }
    if n < 4 || fields.iter().any(|&f| f > 0xFF) {
        return None;
    }
    if at(s, pos) == COLON {
        pos += 1;
        let port = ipv4_component(s, &mut pos, false)?;
        if port == 0 || port > 0xFFFF {
            return None;
        }
    }
    (at(s, pos) == 0).then_some(())
}

/// A run of digits in `base` starting at `start`, as the C `wcstoul` call
/// behind `parse_ipv6_component` sees it. Returns the end index and the value
/// saturated at `u32::MAX`. `None` when there is no digit.
fn ipv6_component(s: &[u16], start: usize, base: u32) -> Option<(usize, u32)> {
    let mut end = start;
    let mut value: u32 = 0;
    while let Some(digit) = hex_value(at(s, end)).filter(|&d| d < base) {
        value = value.saturating_mul(base).saturating_add(digit);
        end += 1;
    }
    (end > start).then_some((end, value))
}

/// `RtlIpv6StringToAddressExW`, the "ex" rules of `ipv6_string_to_address`:
/// optional `[...]`, `::` compression, an optional dotted IPv4 tail, a
/// `%scope`, and a `:port` inside brackets. Only success is reported, so the
/// address itself is not built. Any `0x` component fails in ex mode.
fn ipv6_literal(s: &[u16]) -> Option<()> {
    let mut pos = 0;
    let bracketed = at(s, 0) == LBRACKET;
    if bracketed {
        pos += 1;
    }
    if at(s, pos) == COLON {
        if at(s, pos + 1) != COLON {
            return None;
        }
        pos += 1;
    }

    let mut n_bytes = 0;
    let mut n_ipv4_bytes = 0;
    let mut gap: Option<usize> = None;
    loop {
        if n_ipv4_bytes == 0 && at(s, pos) == COLON {
            pos += 1;
            gap = Some(n_bytes);
            if n_bytes == 14 || ipv6_component(s, pos, 16).is_none() {
                break;
            }
        }

        let start = pos;
        let ipv4_limit = if gap.is_some() { 10 } else { 12 };
        if n_ipv4_bytes == 0
            && n_bytes <= ipv4_limit
            && ipv6_component(s, start, 10).is_some_and(|(end, _)| at(s, end) == DOT)
        {
            n_ipv4_bytes = 1;
        }

        if n_ipv4_bytes != 0 {
            let (end, value) = ipv6_component(s, start, 10)?;
            if end - start > 3 || value > 255 {
                return None;
            }
            pos = end;
            if at(s, pos) != DOT && (n_ipv4_bytes < 4 || (n_bytes < 15 && gap.is_none())) {
                return None;
            }
            n_bytes += 1;
            if n_ipv4_bytes == 4 || at(s, pos) != DOT {
                break;
            }
            n_ipv4_bytes += 1;
        } else {
            let (end, _) = ipv6_component(s, start, 16)?;
            if at(s, start) == ZERO && matches!(at(s, start + 1), LOWER_X | UPPER_X) {
                return None;
            }
            pos = end;
            if at(s, pos) != COLON && n_bytes < 14 && gap.is_none() {
                return None;
            }
            if end - start > 4 {
                return None;
            }
            n_bytes += 2;
            if at(s, pos) != COLON || (gap.is_some() && at(s, pos + 1) == COLON) {
                break;
            }
        }

        let full = if gap.is_some() { 14 } else { 16 };
        if n_bytes == full {
            break;
        }
        pos += 1;
    }

    if gap.is_none() && n_bytes < 16 {
        return None;
    }
    if at(s, pos) == PERCENT {
        pos += 1;
        ipv4_component(s, &mut pos, true)?;
    }
    if bracketed {
        if at(s, pos) != RBRACKET {
            return None;
        }
        pos += 1;
        if at(s, pos) == COLON {
            pos += 1;
            let port = ipv4_component(s, &mut pos, false)?;
            if port == 0 || port > 0xFFFF {
                return None;
            }
        }
    }
    (at(s, pos) == 0).then_some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fill value that must survive a call untouched, so a write past the reported length shows up.
    const SENTINEL: u16 = 0xAAAA;

    /// What a call must return, with the oracle output it was taken from.
    #[derive(Clone, Copy)]
    enum Want {
        /// Success, with this content before the terminating NUL.
        Spn(&'static str),
        /// `BufferOverflow` with this required length.
        Overflow(usize),
        /// `InvalidParameter`.
        Invalid,
    }

    type SpnCase = (
        Option<&'static str>,
        Option<&'static str>,
        Option<&'static str>,
        u16,
        Option<&'static str>,
        usize,
        Want,
    );

    type ClientCase = (Option<&'static str>, Option<&'static str>, usize, Want);

    /// Expected results come from the oracle (`harness` run on these inputs, see
    /// the 'pick.py' script in `target/canon/ntdsapi`). The buffer length is the
    /// capacity the oracle was given.
    const SPN_VECTORS: &[SpnCase] = &[
        (
            Some("class"),
            Some("host"),
            None,
            0,
            None,
            24,
            Want::Spn("class/host"),
        ),
        (
            Some("class"),
            Some("host"),
            Some("instance"),
            555,
            Some("referrer"),
            40,
            Want::Spn("class/instance:555/host"),
        ),
        (
            Some("class"),
            Some("127.0.0.1:22"),
            None,
            555,
            Some("referrer"),
            40,
            Want::Spn("class/127.0.0.1:22:555/referrer"),
        ),
        (
            Some("class"),
            Some("[::1]"),
            None,
            443,
            Some("ref"),
            40,
            Want::Spn("class/[::1]:443/ref"),
        ),
        (
            Some("class"),
            Some("host"),
            Some("instance"),
            0,
            None,
            3,
            Want::Overflow(20),
        ),
        (None, Some("host"), None, 0, None, 5, Want::Invalid),
        (Some("class"), None, None, 0, None, 5, Want::Invalid),
        (Some(""), Some(""), None, 0, None, 2, Want::Spn("/")),
        (
            Some("class"),
            Some("10.0.0.1"),
            None,
            0,
            Some(""),
            40,
            Want::Spn("class/10.0.0.1/"),
        ),
        // The C driver cannot express an embedded NUL, so the oracle ran on "cl".
        (
            Some("cl\0ass"),
            Some("host"),
            None,
            0,
            None,
            24,
            Want::Spn("cl/host"),
        ),
    ];

    const CLIENT_VECTORS: &[ClientCase] = &[
        (Some("class"), Some("host"), 20, Want::Spn("class/host")),
        (Some("class"), Some("host"), 3, Want::Overflow(11)),
        (None, Some("host"), 5, Want::Invalid),
        (Some("class"), None, 5, Want::Invalid),
    ];

    /// (text, `RtlIpv4StringToAddressExW` strict succeeds, `RtlIpv6StringToAddressExW` succeeds).
    const IP_VECTORS: &[(&str, bool, bool)] = &[
        ("", false, false),
        ("1", false, false),
        ("127.0.0.1", true, false),
        ("127.0.0.1:22", true, false),
        ("127.0.0.1garbage", false, false),
        ("1.2.3", false, false),
        ("1.2.3.4.5", false, false),
        ("256.0.0.1", false, false),
        ("4294967296.0.0.1", false, false),
        ("0.0.0.0:0", false, false),
        ("0.0.0.0:65536", false, false),
        ("1.2.3.4:", false, false),
        ("1.2.3.4:x", false, false),
        ("01.2.3.4", false, false),
        ("0x1.2.3.4", false, false),
        ("1.2.3.4:0x1F", true, false),
        ("1.2.3.4:010", true, false),
        ("1..2.3", false, false),
        ("::", false, true),
        ("::1", false, true),
        ("1:2:3:4:5:6:7::", false, true),
        ("1:2:3:4:5:6:7:8", false, true),
        ("1:2:3:4:5:6:7:8:9", false, false),
        ("1:2:3:4:5:6:1.2.3.4", false, true),
        ("1:2:3:4:5:6:1.2.3.256", false, false),
        ("::ffff:1.2.3.4", false, true),
        ("1::1.2.3.4", false, true),
        ("::1%", false, false),
        ("::1%1", false, true),
        ("::1%0x1", false, false),
        ("::1%4294967296", false, false),
        ("[::1]", false, true),
        ("[::1]:0", false, false),
        ("[::1]:65536", false, false),
        ("[::1]:0x50", false, true),
        ("[::1]:080", false, false),
        ("[::1", false, false),
        ("[::1]x", false, false),
        ("[1.2.3.4]", false, false),
        ("::0x1", false, false),
        ("0x1::", false, false),
        ("::AbCd", false, true),
        ("::12345678", false, false),
        ("::g", false, false),
        ("::::", false, false),
        ("1:2", false, false),
        ("1:::2", false, false),
        ("1::2::3", false, false),
        (":1:2:3:4:5:6:7:8", false, false),
        ("1:2:3:4:5:6:7:8%1", false, true),
        ("1:2:3:4:5:6:7::%5", false, true),
    ];

    fn w(s: Option<&str>) -> Option<Vec<u16>> {
        s.map(|s| s.encode_utf16().collect())
    }

    fn utf16(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }

    /// Checks a result and buffer against the expectation. Success must match the
    /// content and length, and nothing past the reported length may change. An error
    /// must leave the whole buffer untouched.
    fn check(result: Result<usize, DsError>, buf: &[u16], want: Want) {
        match want {
            Want::Spn(content) => {
                let len = result.expect("expected success");
                assert_eq!(len, content.len() + 1);
                assert_eq!(&buf[..content.len()], utf16(content).as_slice());
                assert_eq!(buf[content.len()], 0);
                assert!(buf[len..].iter().all(|&c| c == SENTINEL));
            }
            Want::Overflow(required) => {
                let err = result.expect_err("expected overflow");
                assert_eq!(err, DsError::BufferOverflow { required });
                assert_eq!(err.code(), 111);
                assert!(buf.iter().all(|&c| c == SENTINEL));
            }
            Want::Invalid => {
                let err = result.expect_err("expected invalid parameter");
                assert_eq!(err, DsError::InvalidParameter);
                assert_eq!(err.code(), 87);
                assert!(buf.iter().all(|&c| c == SENTINEL));
            }
        }
    }

    #[test]
    fn ds_make_spn_w_matches_oracle() {
        for &(class, name, inst, port, reference, cap, want) in SPN_VECTORS {
            let (class, name, inst, reference) = (w(class), w(name), w(inst), w(reference));
            let mut buf = vec![SENTINEL; cap];
            let result = ds_make_spn_w(
                class.as_deref(),
                name.as_deref(),
                inst.as_deref(),
                port,
                reference.as_deref(),
                &mut buf,
            );
            check(result, &buf, want);
        }
    }

    #[test]
    fn ds_client_make_spn_for_target_server_w_matches_oracle() {
        for &(class, name, cap, want) in CLIENT_VECTORS {
            let (class, name) = (w(class), w(name));
            let mut buf = vec![SENTINEL; cap];
            let result =
                ds_client_make_spn_for_target_server_w(class.as_deref(), name.as_deref(), &mut buf);
            check(result, &buf, want);
        }
    }

    #[test]
    fn ip_literal_parsers_match_oracle() {
        for &(text, v4, v6) in IP_VECTORS {
            let s = utf16(text);
            assert_eq!(ipv4_literal(&s).is_some(), v4, "ipv4 {text:?}");
            assert_eq!(ipv6_literal(&s).is_some(), v6, "ipv6 {text:?}");
        }
    }
}
