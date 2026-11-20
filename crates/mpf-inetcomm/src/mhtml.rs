//! `parse_mhtml_url` from `protocol.c`: splits an `mhtml:` URL into the MHTML file part and the
//! location inside it.

/// The two parts of an `mhtml:` URL. Both are slices of the input, without the terminating NUL.
#[derive(Debug, PartialEq, Eq)]
pub struct MhtmlUrl<'a> {
    /// The text between `mhtml:` and the first `!`, or the end of the URL when there is no `!`.
    pub mhtml: &'a [u16],
    /// The text after `!` or `!x-usc:`. `None` when the URL has no `!`.
    pub location: Option<&'a [u16]>,
}

const PREFIX: &[u8] = b"mhtml:";
const SEPARATOR: &[u8] = b"!x-usc:";

/// Returns `None` when the URL does not start with `mhtml:`, ignoring ASCII case, which is `E_FAIL`
/// in the C code. The URL is read up to its first NUL unit, as the C string functions read it.
pub fn parse_mhtml_url(url: &[u16]) -> Option<MhtmlUrl<'_>> {
    let url = until_nul(url);
    if !has_prefix_ignore_case(url, PREFIX) {
        return None;
    }
    let mhtml = &url[PREFIX.len()..];
    let Some(bang) = mhtml.iter().position(|&c| c == u16::from(b'!')) else {
        return Some(MhtmlUrl {
            mhtml,
            location: None,
        });
    };
    let after = &mhtml[bang..];
    let skip = if starts_with(after, SEPARATOR) {
        SEPARATOR.len()
    } else {
        1
    };
    Some(MhtmlUrl {
        mhtml: &mhtml[..bang],
        location: Some(&after[skip..]),
    })
}

fn until_nul(s: &[u16]) -> &[u16] {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end]
}

fn has_prefix_ignore_case(s: &[u16], prefix: &[u8]) -> bool {
    s.len() >= prefix.len() && s.iter().zip(prefix).all(|(&c, &p)| fold(c) == u16::from(p))
}

fn starts_with(s: &[u16], prefix: &[u8]) -> bool {
    s.len() >= prefix.len() && s.iter().zip(prefix).all(|(&c, &p)| c == u16::from(p))
}

fn fold(c: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&c) {
        c + 0x20
    } else {
        c
    }
}
