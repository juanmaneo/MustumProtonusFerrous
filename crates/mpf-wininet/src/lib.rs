//! Port of the deterministic logic in Wine's `dlls/wininet`: HTTP date parsing and proxy-bypass
//! domain matching from `http.c`, the cookie domain check from `cookie.c`, and the keep-alive and
//! token checks from `http.c`.
//!
//! Ported:
//! - `HTTP_ParseDate` with its RFC 1123, RFC 850 and asctime layouts: [`date::parse_http_date`].
//!   The result is FILETIME ticks.
//! - `HTTP_DomainMatches`: [`proxy_bypass::domain_matches`].
//! - `is_domain_legal_for_cookie`: [`cookie::is_domain_legal`].
//! - `has_keep_alive` and its `has_token` helper: [`header::has_keep_alive`].
//!
//! Skipped:
//! - The URL API (`InternetCrackUrl`, `InternetCanonicalizeUrl`, `InternetCombineUrl`).
//! - The cookie string parser in `cookie.c`.
//! - `InternetTimeFromSystemTime` and `InternetTimeToSystemTime`. These were not reviewed in detail.
//! - FTP, gopher, the URL cache, sockets, sessions and the registry: they need the OS, the network
//!   or shared state.
//!
//! Reference quirks that are kept:
//! - February 29 is accepted in every year. In a non-leap year it gives the same ticks as 1 March.
//! - `wcstoul` accepts a sign, leading white space and white space after a colon, so `08: 49: 37`
//!   parses. A value that is too large is clamped, and a negative value wraps to `u32`.
//! - The asctime layout accepts an optional `GMT` or `UTC` after the year. The RFC layouts require one.
//!
//! Divergences from C:
//! - The weekday of an RFC 850 date may be a full name, such as `Sunday` or `Wednesday`. In C the
//!   full names read an uninitialized unit after the copied weekday, and `Wednesday` also writes past
//!   its buffer, so the C result is undefined.
//! - Case folding and white space are ASCII only, as in the oracle's shim. msvcrt also folds and
//!   classifies code points above U+00FF. That behavior is not verified here.

pub mod cookie;
pub mod date;
pub mod header;
pub mod proxy_bypass;
mod wide;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
