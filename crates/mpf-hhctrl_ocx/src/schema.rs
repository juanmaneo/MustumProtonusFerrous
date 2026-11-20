/// The schema prefixes that skip_schema removes, in the order chm.c checks them.
const SCHEMAS: [&str; 3] = ["its:", "ms-its:", "mk:@MSITStore:"];

/// The prefix that is_chm recognises. Unlike [`SCHEMAS`] it is case-sensitive.
const CHM_PREFIX: &str = "mk:@MSITStore:";

/// Removes an `its:`, `ms-its:` or `mk:@MSITStore:` prefix from `url`, as skip_schema in chm.c
/// does, and returns the rest. The match ignores ASCII case. Returns `url` unchanged when no
/// prefix matches.
pub fn strip_schema(url: &[u16]) -> &[u16] {
    match SCHEMAS
        .iter()
        .copied()
        .find(|schema| starts_with(url, schema, true))
    {
        Some(schema) => &url[schema.len()..],
        None => url,
    }
}

/// Reports whether `url` starts with the `mk:@MSITStore:` prefix, as is_chm in help.c does.
pub fn is_chm_url(url: &[u16]) -> bool {
    starts_with(url, CHM_PREFIX, false)
}

fn starts_with(url: &[u16], prefix: &str, ignore_case: bool) -> bool {
    url.len() >= prefix.len()
        && url.iter().zip(prefix.bytes()).all(|(&unit, want)| {
            if ignore_case {
                unit < 0x80 && (unit as u8).eq_ignore_ascii_case(&want)
            } else {
                unit == u16::from(want)
            }
        })
}
