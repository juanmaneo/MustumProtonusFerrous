//! msi subset for MPF: the GUID base85 and squash codecs, the stream name codec, and the version
//! string helpers of `dlls/msi`.
//!
//! Wine's `dlls/msi` is the Windows Installer engine. Most of it reads files, the registry, COM
//! objects, handles and the installer database. This crate ports the pure logic that works on
//! values and UTF-16 strings. The verbatim C is in `target/canon/msi`, built with ASan and UBSan.
//! Its `wcstol` and `wcschr` are the stand-ins in `crt_shim.h`.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `unsquash_guid` (`registry.c`): a 32-digit packed GUID becomes a braced GUID with the digits
//!   of each group reversed in pairs.
//! - `decode_base85_guid` and `encode_base85_guid` (`registry.c`): the 20-unit base85 form of a
//!   GUID, as four DWORDs in memory order.
//! - `version_str_to_dword` (`msi_version_str_to_dword`, `registry.c`): `major.minor.build` packed
//!   as `MAKELONG(build, MAKEWORD(minor, major))`.
//! - `encode_streamname` and `decode_streamname` (`table.c`): the MIME stream name codec, with
//!   `utf2mime` and `mime2utf` as its helpers.
//! - `parse_version_string` (`msi_parse_version_string`, `appsearch.c`): the four dotted fields.
//! - `compare_file_versions` and `compare_font_versions` (`action.c`).
//!
//! Deviations from the C, all in cases the oracle excludes:
//! - A C string ends at its NUL. Every function here reads its input up to the first NUL, or to
//!   the end of the slice. Fixed-width reads (the 20 units of a base85 GUID) see NUL past the end,
//!   as the C does at the terminator.
//! - `decode_base85_guid` returns `None` without the partial words the C may already have stored.
//!   A slice cannot be NULL, so the C's `!str` check has no counterpart.
//! - `encode_base85_guid` returns the 20 units without the NUL the C writes after them.
//! - `parse_version_string` shifts an `int` in C. UBSan flags `x << 16` once `x` is outside the
//!   16-bit range, and flags a negative `x`. This crate shifts a `u32`, so the result is the
//!   wrapped DWORD the C computes on its usual two's-complement target.
//!
//! Skipped (the rest of `dlls/msi`): the database engine (`table.c`, `where.c`, `select.c`,
//! `update.c`, `insert.c`, `create.c`, `delete.c`, `distinct.c`, `drop.c`, `alter.c`), the
//! install and action code and its file, registry and assembly work (`action.c`, `files.c`,
//! `registry.c`, `assembly.c`, `media.c`, `script.c`, `install.c`, `source.c`, `patch.c`,
//! `upgrade.c`), the MSI API surface (`msi.c`, `msi_main.c`, `msiquery.c`, `package.c`, `record.c`,
//! `handle.c`, `streams.c`, `storages.c`, `suminfo.c`), the UI (`dialog.c`, `font.c`, `custom.c`,
//! `classes.c`), the SQL front end (`sql.y`, `cond.y`, `tokenize.c`, `query.h`), and the COM
//! server (`msiserver.idl`, `winemsi.idl`, `msiserver_dispids.h`).

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `table_enc85`: the base85 digit set, in value order.
const TABLE_ENC85: &[u8; 85] =
    b"!$%&'()*+,-.0123456789=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz{}~";

/// `table_dec85`: the value of each ASCII unit in the base85 digit set, or `0xff`.
#[rustfmt::skip]
const TABLE_DEC85: [u8; 0x80] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x00, 0xff, 0xff, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0xff,
    0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0xff, 0xff, 0xff, 0x16, 0xff, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0xff, 0x34, 0x35, 0x36,
    0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
    0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0xff, 0x53, 0x54, 0xff,
];

/// `MAX_STREAM_NAME`: the units of a table stream name, the table marker included.
const MAX_STREAM_NAME: usize = 0x1f;
/// The first unit of an encoded table stream name.
const TABLE_MARKER: u16 = 0x4840;

/// The unit at `i`. Reads past the end see the NUL terminator, as the C does.
fn unit(input: &[u16], i: usize) -> u16 {
    input.get(i).copied().unwrap_or(0)
}

/// The part of a string before its NUL terminator, or all of it when there is none.
fn until_nul(input: &[u16]) -> &[u16] {
    let end = input.iter().position(|&u| u == 0).unwrap_or(input.len());
    &input[..end]
}

/// `wcstol(s, NULL, 10)` as the `crt_shim.h` stand-in reads it: C-locale white space, an optional
/// sign, decimal digits, and saturation at the 32-bit range.
fn wcstol(input: &[u16]) -> i32 {
    let blanks = input
        .iter()
        .take_while(|&&u| u == u16::from(b' ') || (0x09..=0x0d).contains(&u))
        .count();
    let mut rest = &input[blanks..];
    let negative = rest.first() == Some(&u16::from(b'-'));
    if negative || rest.first() == Some(&u16::from(b'+')) {
        rest = &rest[1..];
    }
    let mut value: i64 = 0;
    for &u in rest
        .iter()
        .take_while(|&&u| (u16::from(b'0')..=u16::from(b'9')).contains(&u))
    {
        value = (value * 10 + i64::from(u - u16::from(b'0'))).min(1 << 40);
    }
    if negative {
        value = -value;
    }
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// The text after the first `.`, as `wcschr(p, '.') + 1` gives it.
fn after_dot(input: &[u16]) -> Option<&[u16]> {
    input
        .iter()
        .position(|&u| u == u16::from(b'.'))
        .map(|i| &input[i + 1..])
}

/// `unsquash_guid`: a 32-digit packed GUID becomes the braced GUID, each group's digit pairs in
/// reverse order. Only the length is checked, so the digits need not be hex.
pub fn unsquash_guid(input: &[u16]) -> Option<Vec<u16>> {
    let input = until_nul(input);
    if input.len() != 32 {
        return None;
    }
    let dash = u16::from(b'-');
    let mut out = vec![u16::from(b'{')];
    out.extend((0..8).map(|i| input[7 - i]));
    out.push(dash);
    out.extend((0..4).map(|i| input[11 - i]));
    out.push(dash);
    out.extend((0..4).map(|i| input[15 - i]));
    out.push(dash);
    out.extend((0..2).flat_map(|i| [input[17 + 2 * i], input[16 + 2 * i]]));
    out.push(dash);
    out.extend((2..8).flat_map(|i| [input[17 + 2 * i], input[16 + 2 * i]]));
    out.push(u16::from(b'}'));
    Some(out)
}

/// `decode_base85_guid`: twenty base85 units as the four DWORDs of a GUID, in memory order.
pub fn decode_base85_guid(input: &[u16]) -> Option<[u32; 4]> {
    let mut words = [0u32; 4];
    let mut val = 0u32;
    let mut base = 1u32;
    for i in 0..20 {
        if i % 5 == 0 {
            val = 0;
            base = 1;
        }
        let digit = *TABLE_DEC85.get(usize::from(unit(input, i)))?;
        if digit == 0xff {
            return None;
        }
        val = val.wrapping_add(u32::from(digit).wrapping_mul(base));
        if i % 5 == 4 {
            words[i / 5] = val;
        }
        base = base.wrapping_mul(85);
    }
    Some(words)
}

/// `encode_base85_guid`: the four DWORDs of a GUID as twenty base85 units, least significant digit
/// first within each DWORD.
pub fn encode_base85_guid(words: &[u32; 4]) -> [u16; 20] {
    let mut out = [0u16; 20];
    for (i, &word) in words.iter().enumerate() {
        let mut x = word;
        for digit in 0..5 {
            out[i * 5 + digit] = u16::from(TABLE_ENC85[(x % 85) as usize]);
            x /= 85;
        }
    }
    out
}

/// `msi_version_str_to_dword`: `major.minor.build` packed as `MAKELONG(build, MAKEWORD(minor, major))`.
/// Missing fields are zero, and fields past the third are ignored.
pub fn version_str_to_dword(input: &[u16]) -> u32 {
    let input = until_nul(input);
    let major = wcstol(input) as u32;
    let mut minor = 0;
    let mut build = 0;
    if let Some(rest) = after_dot(input) {
        minor = wcstol(rest) as u32;
        if let Some(rest) = after_dot(rest) {
            build = wcstol(rest) as u32;
        }
    }
    (build & 0xffff) | ((minor & 0xff) << 16) | ((major & 0xff) << 24)
}

/// `msi_parse_version_string`: the first four dotted fields as `(ms, ls)`, where each pair packs as
/// `x1 << 16 | x2`. Fields that are missing are zero.
pub fn parse_version_string(input: &[u16]) -> (u32, u32) {
    let input = until_nul(input);
    let mut fields = [wcstol(input), 0, 0, 0];
    let mut rest = input;
    for field in &mut fields[1..] {
        let Some(after) = after_dot(rest) else {
            break;
        };
        *field = wcstol(after);
        rest = after;
    }
    let [x1, x2, x3, x4] = fields;
    (
        ((x1 as u32) << 16) | (x2 as u32),
        ((x3 as u32) << 16) | (x4 as u32),
    )
}

/// `msi_compare_file_versions`: the file's `(MS, LS)` against the version string, as `-1`, `0` or
/// `1`.
pub fn compare_file_versions(file_ms: u32, file_ls: u32, version: &[u16]) -> i32 {
    let (ms, ls) = parse_version_string(version);
    (file_ms, file_ls).cmp(&(ms, ls)) as i32
}

/// `msi_compare_font_versions`: the `ms` of two version strings, as `-1`, `0` or `1`.
pub fn compare_font_versions(ver1: &[u16], ver2: &[u16]) -> i32 {
    let (ms1, _) = parse_version_string(ver1);
    let (ms2, _) = parse_version_string(ver2);
    ms1.cmp(&ms2) as i32
}

/// `utf2mime`: the 6-bit value of an ASCII unit in the stream name alphabet.
fn utf2mime(x: u16) -> Option<u16> {
    match x {
        0x30..=0x39 => Some(x - 0x30),
        0x41..=0x5a => Some(x - 0x41 + 10),
        0x61..=0x7a => Some(x - 0x61 + 36),
        0x2e => Some(62),
        0x5f => Some(63),
        _ => None,
    }
}

/// `mime2utf`: the ASCII unit of a 6-bit value. Values past 62 map to `_`, as the C does.
fn mime2utf(x: u16) -> u16 {
    match x {
        0..=9 => x + 0x30,
        10..=35 => x - 10 + 0x41,
        36..=61 => x - 36 + 0x61,
        62 => 0x2e,
        _ => 0x5f,
    }
}

/// `encode_streamname`: the MIME form of a stream name, where two MIME characters share one unit.
/// A table name starts with `TABLE_MARKER` and holds at most `MAX_STREAM_NAME` units, so it returns
/// `None` when the name does not finish within that budget.
pub fn encode_streamname(table: bool, input: &[u16]) -> Option<Vec<u16>> {
    let input = until_nul(input);
    let mut count = if table {
        MAX_STREAM_NAME
    } else {
        input.len() + 2
    };
    let mut out = Vec::with_capacity(count);
    if table {
        out.push(TABLE_MARKER);
        count -= 1;
    }
    let mut pos = 0;
    while count > 0 {
        count -= 1;
        let ch = unit(input, pos);
        pos += 1;
        if ch == 0 {
            return Some(out);
        }
        let mut packed = u32::from(ch);
        if let Some(first) = utf2mime(ch) {
            packed = 0x4800 + u32::from(first);
            if let Some(second) = utf2mime(unit(input, pos)) {
                pos += 1;
                // The C adds (second + 0x3ffffc0) << 6 modulo 2^32. Only the low 16 bits reach a WCHAR.
                packed = packed.wrapping_add((u32::from(second) + 0x3ffffc0) << 6);
            }
        }
        out.push(packed as u16);
    }
    None
}

/// `decode_streamname`: the ASCII form of an encoded stream name. Units outside the MIME range pass
/// through unchanged.
pub fn decode_streamname(input: &[u16]) -> Vec<u16> {
    let mut out = Vec::new();
    for &ch in until_nul(input) {
        if !(0x3800..0x4840).contains(&ch) {
            out.push(ch);
        } else if ch >= 0x4800 {
            out.push(mime2utf(ch - 0x4800));
        } else {
            let low = ch - 0x3800;
            out.push(mime2utf(low & 0x3f));
            out.push(mime2utf((low >> 6) & 0x3f));
        }
    }
    out
}
