//! Headless subset of Wine's `dlls/credui/credui_main.c`.
//!
//! One export is ported: `CredUIParseUserNameW`, which splits a user name into a user and a domain over caller-sized UTF-16 buffers. The port keeps Wine's split rules and error codes, including one boundary where Wine checks the `@` case one unit stricter than the data needs. The oracle confirms that behaviour, so the port keeps it.
//!
//! Ported (1 of the 21 `credui.spec` entries):
//! - `CredUIParseUserNameW`: `cred_ui_parse_user_name_w`. A name with a `\` splits at the first one into domain and user. Otherwise a name with an `@` splits at the last one into user and domain. Otherwise the whole name is the user and the domain is empty. The buffer capacities are the C `ulMaxUserChars` and `ulMaxDomainChars`, and nothing is written on error.
//!
//! Skipped (20 entries, not ported):
//! - FIXME stubs, 8 entries: bodies that log `FIXME` and return a fixed value or nothing: `CredUIInitControls`, `CredUIStoreSSOCredA(W)`, `CredUIReadSSOCredA(W)`, `CredUIPromptForWindowsCredentialsW`, `CredPackAuthenticationBufferW`, `CredUnPackAuthenticationBufferW`. Porting them would invent behaviour.
//! - Spec names with no C body, 9 entries: `CredUICmdLinePromptForCredentialsA(W)`, `CredUIConfirmCredentialsA`, `CredUIParseUserNameA`, `CredUIPromptForCredentialsA`, `DllGetClassObject`, `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`.
//! - Dialog-driven, 2 entries: `CredUIPromptForCredentialsW` and `SspiPromptForCredentialsW`. Both have deterministic argument checks, but a successful call looks up the credential vault and then shows a dialog, so the arguments alone do not decide the result.
//! - Pending credentials and vault writes, 1 entry: `CredUIConfirmCredentialsW`. Its only deterministic part is the NULL check. The rest works on process-wide pending state and saves to the vault.
//!
//! Not handled, as in Wine: the `FIXME` "handle marshaled credentials" in `CredUIParseUserNameW`.
//!
//! Oracle provenance: `CredUIParseUserNameW` is copied verbatim from Wine's `dlls/credui/credui_main.c` by `awk`, with a sha256 recorded next to it. It is compiled with `gcc -std=gnu11 -Wall -Wextra -Wno-sign-compare -Werror -fsanitize=address,undefined -g` against small Win32 type and string shims. `-Wno-sign-compare` is needed because Wine's body compares signed and unsigned lengths. The harness, its vectors and output live in the git-ignored `target/canon/credui/`.

/// The call succeeded.
pub const ERROR_SUCCESS: u32 = 0;
/// A pointer is null, or a buffer has no room at all.
pub const ERROR_INVALID_PARAMETER: u32 = 87;
/// A part of the name does not fit, with its NUL terminator, in its buffer.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

const BACKSLASH: u16 = b'\\' as u16;
const AT: u16 = b'@' as u16;

/// Splits `user_name` into `user` and `domain`, as Wine's `CredUIParseUserNameW` does.
///
/// `user_name` counts up to its first NUL, or to its end when it has none. The
/// capacities of `user` and `domain` are the slice lengths. On success each
/// buffer holds its part followed by a NUL. On error neither buffer is written.
pub fn cred_ui_parse_user_name_w(
    user_name: Option<&[u16]>,
    user: Option<&mut [u16]>,
    domain: Option<&mut [u16]>,
) -> u32 {
    let (Some(user_name), Some(user), Some(domain)) = (user_name, user, domain) else {
        return ERROR_INVALID_PARAMETER;
    };
    if user.is_empty() || domain.is_empty() {
        return ERROR_INVALID_PARAMETER;
    }
    let name = until_nul(user_name);
    let max_user = user.len() - 1;
    let max_domain = domain.len() - 1;

    if let Some(separator) = name.iter().position(|&unit| unit == BACKSLASH) {
        let (domain_part, user_part) = (&name[..separator], &name[separator + 1..]);
        if domain_part.len() > max_domain || user_part.len() > max_user {
            return ERROR_INSUFFICIENT_BUFFER;
        }
        store(domain, domain_part);
        store(user, user_part);
        return ERROR_SUCCESS;
    }
    if let Some(at) = name.iter().rposition(|&unit| unit == AT) {
        // Wine checks `at + 1` against the user capacity, one unit more than
        // the user part `name[..at]` needs. The port keeps that boundary.
        let domain_part = &name[at + 1..];
        if at + 1 > max_user || domain_part.len() > max_domain {
            return ERROR_INSUFFICIENT_BUFFER;
        }
        store(domain, domain_part);
        store(user, &name[..at]);
        return ERROR_SUCCESS;
    }
    if name.len() > max_user {
        return ERROR_INSUFFICIENT_BUFFER;
    }
    store(user, name);
    store(domain, &[]);
    ERROR_SUCCESS
}

/// The units before the first NUL, or all of them when there is no NUL.
fn until_nul(units: &[u16]) -> &[u16] {
    match units.iter().position(|&unit| unit == 0) {
        Some(end) => &units[..end],
        None => units,
    }
}

/// Copies `src` to the front of `dst` and writes a NUL after it.
fn store(dst: &mut [u16], src: &[u16]) {
    dst[..src.len()].copy_from_slice(src);
    dst[src.len()] = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SENTINEL: u16 = 0x005A;

    /// One oracle row: id, name (`None` is NULL), user capacity, domain capacity,
    /// the null-pointer flag (`U` or `D`, or `-`), rc, user and domain.
    type Row = (
        &'static str,
        Option<&'static str>,
        usize,
        usize,
        &'static str,
        u32,
        Option<&'static str>,
        Option<&'static str>,
    );

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    /// `Some(text)` must hold `text` and then a NUL. `None` must be untouched.
    fn expect_buffer(id: &str, buffer: &[u16], expected: Option<&str>) {
        match expected {
            Some(text) => {
                let units = wide(text);
                assert_eq!(&buffer[..units.len()], units.as_slice(), "{id}");
                assert_eq!(buffer[units.len()], 0, "{id}");
            }
            None => assert!(buffer.iter().all(|&unit| unit == SENTINEL), "{id}"),
        }
    }

    fn parse(
        name: Option<&[u16]>,
        user_len: usize,
        domain_len: usize,
    ) -> (u32, Vec<u16>, Vec<u16>) {
        let mut user = vec![SENTINEL; user_len];
        let mut domain = vec![SENTINEL; domain_len];
        let rc =
            cred_ui_parse_user_name_w(name, Some(user.as_mut_slice()), Some(domain.as_mut_slice()));
        (rc, user, domain)
    }

    #[test]
    fn oracle_rows_match_wine() {
        // Rows P01 to P24 of target/canon/credui/out.txt, in input order.
        let rows: [Row; 24] = [
            (
                "P01",
                Some("DOMAIN\\user"),
                32,
                32,
                "-",
                0,
                Some("user"),
                Some("DOMAIN"),
            ),
            ("P02", Some("ABCDEF\\u"), 32, 6, "-", 122, None, None),
            (
                "P03",
                Some("ABCDEF\\u"),
                32,
                7,
                "-",
                0,
                Some("u"),
                Some("ABCDEF"),
            ),
            ("P04", Some("D\\abc"), 3, 32, "-", 122, None, None),
            ("P05", Some("D\\abc"), 4, 32, "-", 0, Some("abc"), Some("D")),
            ("P06", Some("\\user"), 8, 8, "-", 0, Some("user"), Some("")),
            ("P07", Some("DOM\\"), 8, 8, "-", 0, Some(""), Some("DOM")),
            (
                "P08",
                Some("ABCDEFGH\\ijklmnop"),
                4,
                4,
                "-",
                122,
                None,
                None,
            ),
            (
                "P09",
                Some("user@DOMAIN"),
                32,
                32,
                "-",
                0,
                Some("user"),
                Some("DOMAIN"),
            ),
            ("P10", Some("a@b@c"), 8, 8, "-", 0, Some("a@b"), Some("c")),
            ("P11", Some("user@dom"), 5, 32, "-", 122, None, None),
            (
                "P12",
                Some("user@dom"),
                6,
                32,
                "-",
                0,
                Some("user"),
                Some("dom"),
            ),
            ("P13", Some("user@ABCDEF"), 32, 6, "-", 122, None, None),
            (
                "P14",
                Some("user@ABCDEF"),
                32,
                7,
                "-",
                0,
                Some("user"),
                Some("ABCDEF"),
            ),
            ("P15", Some("@dom"), 4, 4, "-", 0, Some(""), Some("dom")),
            ("P16", Some("plain"), 6, 4, "-", 0, Some("plain"), Some("")),
            ("P17", Some("plain"), 5, 4, "-", 122, None, None),
            ("P18", Some(""), 1, 1, "-", 0, Some(""), Some("")),
            ("P19", None, 8, 8, "-", 87, None, None),
            ("P20", Some("DOM\\user"), 8, 8, "U", 87, None, None),
            ("P21", Some("DOM\\user"), 8, 8, "D", 87, None, None),
            ("P22", Some("DOM\\user"), 0, 8, "-", 87, None, None),
            ("P23", Some("DOM\\user"), 8, 0, "-", 87, None, None),
            ("P24", Some("a@b\\c"), 8, 8, "-", 0, Some("c"), Some("a@b")),
        ];
        for (id, name, max_user, max_domain, flag, rc, user, domain) in rows {
            let name = name.map(wide);
            let mut user_buf = vec![SENTINEL; max_user];
            let mut domain_buf = vec![SENTINEL; max_domain];
            let user_arg = (flag != "U").then_some(user_buf.as_mut_slice());
            let domain_arg = (flag != "D").then_some(domain_buf.as_mut_slice());
            let got = cred_ui_parse_user_name_w(name.as_deref(), user_arg, domain_arg);
            assert_eq!(got, rc, "{id}");
            expect_buffer(id, &user_buf, user);
            expect_buffer(id, &domain_buf, domain);
        }
    }

    #[test]
    fn a_name_without_a_nul_is_read_to_its_end() {
        let (rc, user, domain) = parse(Some(&wide("DOM\\user")), 8, 8);
        assert_eq!(rc, ERROR_SUCCESS);
        expect_buffer("no nul", &user, Some("user"));
        expect_buffer("no nul", &domain, Some("DOM"));
    }

    #[test]
    fn a_nul_ends_the_name_before_any_separator() {
        let name = [b'a' as u16, b'b' as u16, 0, b'@' as u16, b'c' as u16];
        let (rc, user, domain) = parse(Some(&name), 8, 8);
        assert_eq!(rc, ERROR_SUCCESS);
        expect_buffer("embedded nul", &user, Some("ab"));
        expect_buffer("embedded nul", &domain, Some(""));
    }

    #[test]
    fn only_the_exact_unit_is_a_separator() {
        // U+015C has a low byte of 0x5C, but it is not a backslash.
        let (rc, user, domain) = parse(Some(&wide("A\u{15C}B")), 8, 8);
        assert_eq!(rc, ERROR_SUCCESS);
        expect_buffer("not a separator", &user, Some("A\u{15C}B"));
        expect_buffer("not a separator", &domain, Some(""));
    }

    #[test]
    fn writes_stop_at_the_terminator() {
        let (rc, user, domain) = parse(Some(&wide("u@d")), 10, 10);
        assert_eq!(rc, ERROR_SUCCESS);
        assert_eq!(&user[..1], &wide("u")[..]);
        assert_eq!(user[1], 0);
        assert!(user[2..].iter().all(|&unit| unit == SENTINEL));
        assert_eq!(&domain[..1], &wide("d")[..]);
        assert!(domain[2..].iter().all(|&unit| unit == SENTINEL));
    }

    #[test]
    fn a_null_argument_is_invalid_even_with_a_valid_name() {
        let mut user = [SENTINEL; 4];
        let mut domain = [SENTINEL; 4];
        let name = wide("d\\u");
        let rc = cred_ui_parse_user_name_w(None, Some(&mut user), Some(&mut domain));
        assert_eq!(rc, ERROR_INVALID_PARAMETER);
        let rc = cred_ui_parse_user_name_w(Some(&name), None, Some(&mut domain));
        assert_eq!(rc, ERROR_INVALID_PARAMETER);
        let rc = cred_ui_parse_user_name_w(Some(&name), Some(&mut user), None);
        assert_eq!(rc, ERROR_INVALID_PARAMETER);
        assert!(user
            .iter()
            .chain(domain.iter())
            .all(|&unit| unit == SENTINEL));
    }
}
