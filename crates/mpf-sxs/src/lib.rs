//! Portable identity and assembly-name logic from Wine's `dlls/sxs`.
//!
//! Ported deterministic routines:
//! - `escape`, `unescape`, and `parse_version` from `name.c`.
//! - The display-name parsing logic used by `CreateAssemblyNameObject`.
//! - `append_string` and `build_assembly_name`, used by
//!   `SxspGenerateManifestPathOnAssemblyIdentity`.
//!
//! The assembly component generator models that export's required attributes,
//! length checks, and caller-buffer capacity check. It does not access the
//! Windows directory, files, registry, COM, or handles.
//!
//! Skipped: `SxsLookupClrGuid` reads activation-context memory; cache path
//! generation uses the live Windows directory; cache queries/install/uninstall
//! use filesystem and XML state; COM interface wrappers are not modelled; and
//! FIXME/stub exports have no deterministic implementation to port.
//!
//! Divergences from C: inputs are bounded slices and are read only through
//! their first NUL. Invalid memory accesses become safe errors. The C
//! `parse_version` left-shift can invoke signed-overflow UB for some negative
//! or large components; this port uses explicit 32-bit wrapping arithmetic.
//! Case folding in `build_assembly_name` is ASCII-only, matching the oracle
//! shim's deterministic mapping.

/// `S_OK`.
pub const S_OK: i32 = 0;
/// `E_INVALIDARG`.
pub const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
/// `ERROR_INSUFFICIENT_BUFFER`.
pub const ERROR_INSUFFICIENT_BUFFER: i32 = 122;
/// `ERROR_INVALID_PARAMETER`.
pub const ERROR_INVALID_PARAMETER: i32 = 87;
/// `ERROR_NOT_FOUND`.
pub const ERROR_NOT_FOUND: i32 = 1168;
/// `ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME`.
pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME: i32 = 14080;
/// `ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE`.
pub const ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE: i32 = 14079;

const CANOF_PARSE_DISPLAY_NAME: u32 = 1;
const HRESULT_FROM_WIN32_INVALID_ATTRIBUTE: i32 = 0x8007_3700u32 as i32;

/// Parsed fields of a Win32 assembly identity. `None` means the attribute was
/// absent; an empty vector means it was present with an empty quoted value.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AssemblyIdentity {
    /// Assembly name.
    pub name: Vec<u16>,
    /// `processorArchitecture`.
    pub arch: Option<Vec<u16>>,
    /// `publicKeyToken`.
    pub token: Option<Vec<u16>>,
    /// `type`.
    pub kind: Option<Vec<u16>>,
    /// `version`.
    pub version: Option<Vec<u16>>,
    /// `language`.
    pub language: Option<Vec<u16>>,
}

fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&unit| unit == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// Ports Wine's `escape`: XML-escapes special characters and emits every other
/// non-ASCII/non-identity character as a lowercase hexadecimal numeric entity.
pub fn escape(text: Option<&[u16]>) -> Option<Vec<u16>> {
    let text = wstr(text?);
    let mut output = Vec::with_capacity(text.len());
    for &unit in text {
        match unit {
            0x2d | 0x2e | 0x5f | 0x30..=0x39 | 0x41..=0x5a | 0x61..=0x7a => output.push(unit),
            0x3c => output.extend("&lt;".encode_utf16()),
            0x3e => output.extend("&gt;".encode_utf16()),
            0x26 => output.extend("&amp;".encode_utf16()),
            0x22 => output.extend("&quot;".encode_utf16()),
            0x27 => output.extend("&apos;".encode_utf16()),
            _ => {
                output.extend("&#x".encode_utf16());
                output.extend(format!("{unit:x}").encode_utf16());
                output.push(b';' as u16);
            }
        }
    }
    Some(output)
}

fn entity_at(text: &[u16], index: usize, entity: &str) -> bool {
    let entity: Vec<u16> = entity.encode_utf16().collect();
    text.get(index..index.saturating_add(entity.len())) == Some(entity.as_slice())
}

fn digit_value(unit: u16, base: u32) -> Option<u32> {
    let digit = match unit {
        0x30..=0x39 => u32::from(unit - b'0' as u16),
        0x61..=0x66 => u32::from(unit - b'a' as u16) + 10,
        0x41..=0x46 => u32::from(unit - b'A' as u16) + 10,
        _ => return None,
    };
    (digit < base).then_some(digit)
}

fn parse_unsigned(text: &[u16], start: usize, base: u32) -> (u32, usize) {
    let mut index = start;
    while text
        .get(index)
        .is_some_and(|&unit| unit == b' ' as u16 || (b'\t' as u16..=b'\r' as u16).contains(&unit))
    {
        index += 1;
    }
    let negative = text.get(index) == Some(&(b'-' as u16));
    if negative || text.get(index) == Some(&(b'+' as u16)) {
        index += 1;
    }
    let mut magnitude = 0u64;
    let mut any = false;
    let mut overflow = false;
    while let Some(digit) = text.get(index).and_then(|&unit| digit_value(unit, base)) {
        any = true;
        magnitude = magnitude
            .saturating_mul(u64::from(base))
            .saturating_add(u64::from(digit));
        overflow |= magnitude > u64::from(u32::MAX);
        index += 1;
    }
    if !any {
        return (0, start);
    }
    let value = if overflow {
        u32::MAX
    } else if negative {
        0u32.wrapping_sub(magnitude as u32)
    } else {
        magnitude as u32
    };
    (value, index)
}

/// Ports Wine's `unescape`. `len` is the caller's logical bound; as in Wine,
/// a recognized entity may consume characters beyond that bound if they are
/// still present in the supplied NUL-terminated slice.
pub fn unescape(text: &[u16], len: usize) -> Option<Vec<u16>> {
    let text = wstr(text);
    let limit = len.min(text.len());
    let mut output = Vec::with_capacity(limit);
    let mut index = 0;
    while index < limit {
        if text[index] != b'&' as u16 {
            output.push(text[index]);
            index += 1;
            continue;
        }
        let named = [
            ("&lt;", b'<' as u16),
            ("&gt;", b'>' as u16),
            ("&amp;", b'&' as u16),
            ("&quot;", b'"' as u16),
            ("&apos;", b'\'' as u16),
        ];
        if let Some((entity, value)) = named
            .iter()
            .find(|(entity, _)| entity_at(text, index, entity))
        {
            output.push(*value);
            index += entity.encode_utf16().count();
        } else if index + 1 < limit && text[index + 1] == b'#' as u16 {
            let mut digits = index + 2;
            let base = if text
                .get(digits)
                .is_some_and(|&unit| unit == b'x' as u16 || unit == b'X' as u16)
            {
                digits += 1;
                16
            } else {
                10
            };
            let (value, end) = parse_unsigned(text, digits, base);
            if text.get(end) != Some(&(b';' as u16)) {
                return None;
            }
            output.push(value as u16);
            index = end + 1;
        } else {
            let mut end = index + 1;
            while end < limit && text[end] != b';' as u16 {
                end += 1;
            }
            if end - index > 2 {
                return None;
            }
            index = end + 1;
        }
    }
    Some(output)
}

fn parse_signed(text: &[u16]) -> u16 {
    let mut index = 0;
    while text
        .get(index)
        .is_some_and(|&unit| unit == b' ' as u16 || (b'\t' as u16..=b'\r' as u16).contains(&unit))
    {
        index += 1;
    }
    let negative = text.get(index) == Some(&(b'-' as u16));
    if negative || text.get(index) == Some(&(b'+' as u16)) {
        index += 1;
    }
    let mut value = 0i64;
    let mut any = false;
    while let Some(digit) = text.get(index).and_then(|&unit| digit_value(unit, 10)) {
        any = true;
        value = value.saturating_mul(10).saturating_add(i64::from(digit));
        index += 1;
    }
    if !any {
        return 0;
    }
    let signed =
        if negative { -value } else { value }.clamp(i64::from(i32::MIN), i64::from(i32::MAX));
    signed as u16
}

/// Ports Wine's four-component version parser. Missing components are zero;
/// text after the fourth component is ignored, and each component is converted
/// to a 16-bit word as in the C implementation.
pub fn parse_version(version: &[u16]) -> (u32, u32) {
    let version = wstr(version);
    let mut words = [0u16; 4];
    let mut start = 0;
    for (slot, word) in words.iter_mut().enumerate() {
        if start >= version.len() {
            break;
        }
        let end = version[start..]
            .iter()
            .position(|&unit| unit == b'.' as u16)
            .map_or(version.len(), |offset| start + offset);
        *word = parse_signed(&version[start..end]);
        if end == version.len() {
            break;
        }
        if slot == 3 {
            break;
        }
        start = end + 1;
    }
    (
        (u32::from(words[0]) << 16).wrapping_add(u32::from(words[1])),
        (u32::from(words[2]) << 16).wrapping_add(u32::from(words[3])),
    )
}

fn parse_value(text: &[u16], start: usize) -> Option<(Vec<u16>, usize)> {
    if text.get(start) != Some(&(b'"' as u16)) {
        return None;
    }
    let close = text[start + 1..]
        .iter()
        .position(|&unit| unit == b'"' as u16)?
        + start
        + 1;
    Some((unescape(&text[start + 1..], close - start - 1)?, close + 1))
}

fn attribute_name(text: &[u16]) -> Option<u8> {
    if text == "processorArchitecture".encode_utf16().collect::<Vec<_>>() {
        Some(0)
    } else if text == "publicKeyToken".encode_utf16().collect::<Vec<_>>() {
        Some(1)
    } else if text == "type".encode_utf16().collect::<Vec<_>>() {
        Some(2)
    } else if text == "version".encode_utf16().collect::<Vec<_>>() {
        Some(3)
    } else if text == "language".encode_utf16().collect::<Vec<_>>() {
        Some(4)
    } else {
        None
    }
}

/// Parses the display-name grammar used by Wine's `CreateAssemblyNameObject`.
pub fn parse_display_name(
    display_name: Option<&[u16]>,
    flags: u32,
) -> Result<AssemblyIdentity, i32> {
    let input = wstr(display_name.ok_or(E_INVALIDARG)?);
    if input.is_empty() || flags != CANOF_PARSE_DISPLAY_NAME {
        return Err(E_INVALIDARG);
    }
    let mut identity = AssemblyIdentity::default();
    let mut comma = input
        .iter()
        .position(|&unit| unit == b',' as u16)
        .unwrap_or(input.len());
    identity.name = unescape(input, comma).ok_or(E_INVALIDARG)?;
    if comma == input.len() {
        return Ok(identity);
    }
    loop {
        let start = comma + 1;
        let equal = input[start..]
            .iter()
            .position(|&unit| unit == b'=' as u16)
            .map(|offset| start + offset)
            .ok_or(E_INVALIDARG)?;
        let attr =
            attribute_name(&input[start..equal]).ok_or(HRESULT_FROM_WIN32_INVALID_ATTRIBUTE)?;
        let (value, after_quote) = parse_value(input, equal + 1).ok_or(E_INVALIDARG)?;
        match attr {
            0 => identity.arch = Some(value),
            1 => identity.token = Some(value),
            2 => identity.kind = Some(value),
            3 => identity.version = Some(value),
            _ => identity.language = Some(value),
        }
        comma = input[after_quote..]
            .iter()
            .position(|&unit| unit == b',' as u16)
            .map(|offset| after_quote + offset)
            .unwrap_or(input.len());
        if comma == input.len() {
            return Ok(identity);
        }
    }
}

/// Ports `CreateAssemblyNameObject`'s deterministic argument checks and display
/// name parser. The output object/COM lifetime is deliberately not represented.
pub fn create_assembly_name(
    display_name: Option<&[u16]>,
    flags: u32,
) -> Result<AssemblyIdentity, i32> {
    let display_name = display_name.ok_or(E_INVALIDARG)?;
    if wstr(display_name).is_empty() || flags != CANOF_PARSE_DISPLAY_NAME {
        return Err(E_INVALIDARG);
    }
    parse_display_name(Some(display_name), flags)
}

fn append_string(buffer: &mut Vec<u16>, text: &[u16], max_len: usize) {
    if text.first().copied().unwrap_or(0) == 0 {
        return;
    }
    let start = buffer.len();
    let mut filtered: Vec<u16> = text
        .iter()
        .copied()
        .take_while(|&unit| unit != 0)
        .filter(|&unit| {
            unit == b'-' as u16
                || unit == b'.' as u16
                || (0x30..=0x39).contains(&unit)
                || (b'A' as u16..=b'Z' as u16).contains(&unit)
                || (b'a' as u16..=b'z' as u16).contains(&unit)
        })
        .collect();
    if filtered.len() > max_len {
        let pos = max_len / 2;
        filtered[pos - 1] = b'.' as u16;
        filtered[pos] = b'.' as u16;
        let suffix_start = filtered.len() - pos + 1;
        filtered.copy_within(suffix_start..suffix_start + pos - 1, pos + 1);
        filtered.truncate(max_len);
    }
    buffer.extend(filtered);
    buffer.push(b'_' as u16);
    for unit in &mut buffer[start..] {
        if (b'A' as u16..=b'Z' as u16).contains(unit) {
            *unit += 32;
        }
    }
}

/// Builds the deterministic assembly-name component used by Wine's cache and
/// `SxspGenerateManifestPathOnAssemblyIdentity`.
pub fn build_assembly_name(
    arch: Option<&[u16]>,
    name: &[u16],
    token: Option<&[u16]>,
    version: &[u16],
    language: Option<&[u16]>,
) -> Vec<u16> {
    const NONE: &[u16] = &[b'n' as u16, b'o' as u16, b'n' as u16, b'e' as u16];
    let mut result = Vec::new();
    append_string(&mut result, arch.unwrap_or(NONE), 16);
    append_string(&mut result, name, 40);
    result.extend(wstr(token.unwrap_or(NONE)));
    result.push(b'_' as u16);
    result.extend(wstr(version));
    result.push(b'_' as u16);
    append_string(&mut result, language.unwrap_or(NONE), 8);
    result.extend("deadbeef".encode_utf16());
    result
}

/// Errors produced by the portable part of `SxspGenerateManifestPathOnAssemblyIdentity`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManifestComponentError {
    /// The display name or its flags are invalid.
    InvalidIdentity,
    /// Required `name` or `version` attribute is absent.
    MissingRequiredAttribute,
    /// The C export rejects an overlong version or public key token.
    AttributeTooLong,
    /// Caller capacity must exceed the returned component's length to include NUL.
    InsufficientBuffer { required: usize },
}

/// Parses an assembly identity and generates the filesystem-independent
/// component. `capacity` includes space for the C NUL terminator.
pub fn generate_manifest_component(
    identity: Option<&[u16]>,
    capacity: usize,
) -> Result<Vec<u16>, ManifestComponentError> {
    let identity = create_assembly_name(identity, CANOF_PARSE_DISPLAY_NAME)
        .map_err(|_| ManifestComponentError::InvalidIdentity)?;
    let (Some(version), Some(name)) = (identity.version.as_deref(), Some(identity.name.as_slice()))
    else {
        return Err(ManifestComponentError::MissingRequiredAttribute);
    };
    if version.len() > 103
        || identity
            .token
            .as_ref()
            .is_some_and(|token| token.len() > 100)
    {
        return Err(ManifestComponentError::AttributeTooLong);
    }
    let component = build_assembly_name(
        identity.arch.as_deref(),
        name,
        identity.token.as_deref(),
        version,
        identity.language.as_deref(),
    );
    if capacity <= component.len() {
        return Err(ManifestComponentError::InsufficientBuffer {
            required: component.len() + 1,
        });
    }
    Ok(component)
}

#[cfg(test)]
mod oracle_checks {
    use super::*;

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    fn shown(text: &[u16]) -> String {
        String::from_utf16_lossy(wstr(text))
    }

    fn field(value: Option<&[u16]>) -> String {
        value.map(shown).unwrap_or_else(|| "NULL".to_owned())
    }

    fn identity_output(identity: &AssemblyIdentity) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            shown(&identity.name),
            field(identity.arch.as_deref()),
            field(identity.token.as_deref()),
            field(identity.kind.as_deref()),
            field(identity.version.as_deref()),
            field(identity.language.as_deref())
        )
    }

    fn build_row(args: &[&str]) -> String {
        let optional = |text: &str| (text != "-").then(|| wide(text));
        let arch = optional(args[0]);
        let name = wide(args[1]);
        let token = optional(args[2]);
        let version = wide(args[3]);
        let language = optional(args[4]);
        let output = build_assembly_name(
            arch.as_deref(),
            &name,
            token.as_deref(),
            &version,
            language.as_deref(),
        );
        format!("{}|{}", shown(&output), output.len())
    }

    #[test]
    fn matches_sanitized_wine_oracle_table() {
        let cases: &[(&str, &[&str], &str)] = &[
            ("escape", &["A<&>\"'Z"], "A&lt;&amp;&gt;&quot;&apos;Z"),
            ("escape", &["/"], "&#x2f;"),
            ("unescape", &["&lt;&gt;&amp;&quot;&apos;", "25"], "<>&\"'"),
            ("unescape", &["&#65;&#x42;&#X43;", "18"], "ABC"),
            ("unescape", &["&amp;", "1"], "&"),
            ("unescape", &["&#;", "3"], ""),
            ("unescape", &["&bad;", "5"], "NULL"),
            ("unescape", &["&#xzz;", "6"], "NULL"),
            ("version", &["1.2.3.4"], "00000000,00010002,00030004"),
            ("version", &["7.8"], "00000000,00070008,00000000"),
            ("version", &["65536.65537.2.3"], "00000000,00000001,00020003"),
            ("display", &["name"], "00000000|name|NULL|NULL|NULL|NULL|NULL"),
            ("display", &["A&amp;B,version=\"1.2.3.4\""], "00000000|A&B|NULL|NULL|NULL|1.2.3.4|NULL"),
            ("display", &["n,processorArchitecture=\"x86\",publicKeyToken=\"abcd\",type=\"win32\",version=\"1.0\",language=\"en-US\""], "00000000|n|x86|abcd|win32|1.0|en-US"),
            ("display", &["n,language=\"en&amp;us\""], "00000000|n|NULL|NULL|NULL|NULL|en&us"),
            ("display", &["n,version=\"unterminated"], "80070057|n|NULL|NULL|NULL|NULL|NULL"),
            ("display", &["n,unknown=\"x\""], "80073700|n|NULL|NULL|NULL|NULL|NULL"),
            ("display", &["n,version=x"], "80070057|n|NULL|NULL|NULL|NULL|NULL"),
            ("display", &["n,version=\"a,b\""], "00000000|n|NULL|NULL|NULL|a,b|NULL"),
            ("display", &["n,processorArchitecture=\"\""], "00000000|n||NULL|NULL|NULL|NULL"),
            ("assembly", &["-", "name", "-", "1.2", "-"], "none_name_none_1.2_none_deadbeef|32"),
            ("assembly", &["x86", "Name!", "AbCd", "1.2.3.4", "en-US"], "x86_name_AbCd_1.2.3.4_en-us_deadbeef|36"),
            ("assembly", &["A/B", "hello world", "token", "v-1", "*"], "ab_helloworld_token_v-1__deadbeef|33"),
            ("assembly", &["12345678901234567", "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN", "token", "v", "lllllllll"], "1234567..1234567_nnnnnnnnnnnnnnnnnnn..nnnnnnnnnnnnnnnnnnn_token_v_lll..lll_deadbeef|83"),
        ];
        for (operation, args, expected) in cases {
            let actual = match *operation {
                "escape" => shown(&escape(Some(&wide(args[0]))).unwrap()),
                "unescape" => {
                    let input = wide(args[0]);
                    match unescape(&input, args[1].parse().unwrap()) {
                        Some(output) if wstr(&output).is_empty() && expected.is_empty() => {
                            String::new()
                        }
                        Some(output) => shown(&output),
                        None => "NULL".to_owned(),
                    }
                }
                "version" => {
                    let (high, low) = parse_version(&wide(args[0]));
                    format!("00000000,{high:08x},{low:08x}")
                }
                "display" => {
                    match parse_display_name(Some(&wide(args[0])), CANOF_PARSE_DISPLAY_NAME) {
                        Ok(identity) => format!("00000000|{}", identity_output(&identity)),
                        Err(error) => format!(
                            "{:08x}|{}",
                            error as u32,
                            identity_output(&AssemblyIdentity {
                                name: wide(args[0].split(',').next().unwrap()),
                                ..AssemblyIdentity::default()
                            })
                        ),
                    }
                }
                _ => build_row(args),
            };
            assert_eq!(actual, *expected, "{operation} {args:?}");
        }
    }

    #[test]
    fn validation_and_safe_edges() {
        assert_eq!(escape(None), None);
        assert_eq!(
            shown(&escape(Some(&[b'a' as u16, 0, b'b' as u16])).unwrap()),
            "a"
        );
        assert_eq!(shown(&escape(Some(&[0x00e9])).unwrap()), "&#xe9;");
        assert_eq!(unescape(&[], 0), Some(vec![]));
        assert_eq!(unescape(&wide("&x;"), 3), Some(vec![]));
        assert_eq!(unescape(&wide("&x"), 2), Some(vec![]));
        assert_eq!(unescape(&wide("&a-long;"), 8), None);
        assert_eq!(shown(&unescape(&wide("&# 65;"), 6).unwrap()), "A");
        assert_eq!(unescape(&wide("&#4294967296;"), 13), Some(vec![u16::MAX]));
        assert_eq!(unescape(&wide("&#-1;"), 5), Some(vec![u16::MAX]));
        assert_eq!(parse_version(&wide("-1.2.3.4")), (0xffff_0002, 0x0003_0004));
        assert_eq!(parse_version(&wide("1.bad.3")), (0x0001_0000, 0x0003_0000));
        assert_eq!(parse_version(&wide("")), (0, 0));
        assert_eq!(
            parse_display_name(Some(&wide("&bad;")), CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            parse_display_name(Some(&wide("n,version=\"&bad;\"")), CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            parse_version(&wide("1.2.3.4.5")),
            (0x0001_0002, 0x0003_0004)
        );
        assert_eq!(parse_version(&wide(" 1.2")), (0x0001_0002, 0));
        assert_eq!(unescape(&wide("&#xAb;"), 6), Some(vec![0xab]));
        assert_eq!(parse_display_name(Some(&wide("n")), 0), Err(E_INVALIDARG));
        assert_eq!(
            parse_display_name(Some(&[]), CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            parse_display_name(None, CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            build_assembly_name(Some(&[]), &wide("n"), None, &wide("1"), None),
            wide("n_none_1_none_deadbeef")
        );
        assert_eq!(
            create_assembly_name(None, CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            create_assembly_name(Some(&[]), CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            create_assembly_name(Some(&wide("name")), 0),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            parse_display_name(Some(&wide("n,")), CANOF_PARSE_DISPLAY_NAME),
            Err(E_INVALIDARG)
        );
        assert_eq!(
            parse_display_name(
                Some(&wide("n,version=\"1\",version=\"2\"")),
                CANOF_PARSE_DISPLAY_NAME
            )
            .unwrap()
            .version,
            Some(wide("2"))
        );
    }

    #[test]
    fn manifest_component_matches_export_checks() {
        let identity = wide("N,version=\"1.2\",processorArchitecture=\"x86\"");
        assert_eq!(
            generate_manifest_component(Some(&identity), 100).unwrap(),
            wide("x86_n_none_1.2_none_deadbeef")
        );
        assert_eq!(
            generate_manifest_component(Some(&identity), 28),
            Err(ManifestComponentError::InsufficientBuffer { required: 29 })
        );
        assert_eq!(
            generate_manifest_component(Some(&identity), 29).unwrap(),
            wide("x86_n_none_1.2_none_deadbeef")
        );
        assert_eq!(
            generate_manifest_component(Some(&wide("N")), 100),
            Err(ManifestComponentError::MissingRequiredAttribute)
        );
        assert_eq!(
            generate_manifest_component(Some(&wide("N,version=\"1\"")), 0),
            Err(ManifestComponentError::InsufficientBuffer { required: 28 })
        );
        assert_eq!(
            generate_manifest_component(None, 100),
            Err(ManifestComponentError::InvalidIdentity)
        );
        let long_version = format!("N,version=\"{}\"", "v".repeat(104));
        assert_eq!(
            generate_manifest_component(Some(&wide(&long_version)), 1000),
            Err(ManifestComponentError::AttributeTooLong)
        );
        let long_token = format!("N,version=\"1\",publicKeyToken=\"{}\"", "t".repeat(101));
        assert_eq!(
            generate_manifest_component(Some(&wide(&long_token)), 1000),
            Err(ManifestComponentError::AttributeTooLong)
        );
    }
}
