//! Port of the portable logic in Wine's `dlls/scrrun`.
//!
//! `scrrun.spec` exports the class factories of `Scripting.FileSystemObject` and
//! `Scripting.Dictionary`. The functions below compute from values alone and are ported
//! verbatim in behavior:
//!
//! - `filesystem.c`: `build_path` (`IFileSystem3::BuildPath`), `parent_folder_name`
//!   (`GetParentFolderName`), `file_name` (`GetFileName`), `base_name` (`GetBaseName`),
//!   `extension_name` (`GetExtensionName`), `drive_name` (`GetDriveName`), `create_error`
//!   (the `ERROR_*` to `CTL_E_*` mapping), `version_string` (the formatting in
//!   `get_versionstring`), `is_dir_data` and `is_file_data`.
//! - `dictionary.c`: `str_hash` (`get_str_hash`), `num_hash` (`get_num_hash`) and `flt_hash`
//!   (`get_flt_hash`).
//!
//! Skipped: `get_ptr_hash` (depends on object addresses), `get_date_from_filetime` (local time
//! zone), `variant_from_largeint` (VARIANT conversion), `create_textstream` and the stream I/O
//! (files), drives, enumerators, copy, move and delete (files, handles), `GetAbsolutePathName`
//! (file system lookups), `GetTempName` (randomness and files), and the COM and IDispatch glue.
//!
//! Strings are `&[u16]` holding the code units of a `BSTR` or `WCHAR` string, without the
//! terminating NUL. A NULL string is modelled by the caller as `None` or an empty slice, as
//! documented per function. Functions that use `lstrlenW`, `wcscmp` or `while (*str)` in Wine see
//! only the part before the first NUL unit, which `until_nul` models.

/// `E_FAIL`, returned by `create_error` for an unmapped error code.
pub const E_FAIL: u32 = 0x8000_4005;
/// `CTL_E_ILLEGALFUNCTIONCALL`, returned by `flt_hash` for NaN.
pub const CTL_E_ILLEGALFUNCTIONCALL: u32 = 0x800A_0005;
/// `CTL_E_FILENOTFOUND`.
pub const CTL_E_FILENOTFOUND: u32 = 0x800A_0035;
/// `CTL_E_PATHNOTFOUND`.
pub const CTL_E_PATHNOTFOUND: u32 = 0x800A_004C;
/// `CTL_E_PERMISSIONDENIED`.
pub const CTL_E_PERMISSIONDENIED: u32 = 0x800A_0046;
/// `CTL_E_FILEALREADYEXISTS`.
pub const CTL_E_FILEALREADYEXISTS: u32 = 0x800A_003A;

const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_PATH_NOT_FOUND: u32 = 3;
const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_FILE_EXISTS: u32 = 80;
const ERROR_ALREADY_EXISTS: u32 = 183;

/// `FILE_ATTRIBUTE_DIRECTORY`.
pub const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;

/// The modulus of the dictionary hash, `DICT_HASH_MOD`.
const DICT_HASH_MOD: u32 = 1201;

const BACKSLASH: u16 = b'\\' as u16;
const SLASH: u16 = b'/' as u16;
const COLON: u16 = b':' as u16;
const DOT: u16 = b'.' as u16;

fn is_separator(c: u16) -> bool {
    c == SLASH || c == BACKSLASH
}

/// The last code unit, or 0 for an empty string. The C code reads `Path[len - 1]`, which for an
/// empty `BSTR` lands on the length prefix, whose high half is 0.
fn last_unit(s: &[u16]) -> u16 {
    s.last().copied().unwrap_or(0)
}

fn first_unit(s: &[u16]) -> u16 {
    s.first().copied().unwrap_or(0)
}

/// The part of a string before its first NUL unit, as `lstrlenW` and `wcscmp` see it.
fn until_nul(s: &[u16]) -> &[u16] {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    &s[..end]
}

/// The code unit at `index`, or 0 at the end of the string, as the NUL terminator reads.
fn unit_or_nul(s: &[u16], index: usize) -> u16 {
    s.get(index).copied().unwrap_or(0)
}

/// `IFileSystem3::BuildPath`. A `None` side contributes nothing, and both `None` gives "".
/// Joins with one backslash, collapsing a doubled separator at the seam, and adds no backslash
/// after a trailing colon. In that case Wine sizes the result for the backslash it skips, so the
/// BSTR keeps a trailing NUL unit.
///
/// Both-sided inputs must not contain NUL units. Wine's `lstrcpyW` and `wcscat` stop at a NUL
/// and leave the rest of the buffer unwritten, so there is no deterministic result to port.
pub fn build_path(path: Option<&[u16]>, name: Option<&[u16]>) -> Vec<u16> {
    match (path, name) {
        (Some(path), Some(name)) => {
            let path_ends_sep = last_unit(path) == BACKSLASH;
            let name_starts_sep = first_unit(name) == BACKSLASH;
            let mut out = Vec::with_capacity(path.len() + name.len() + 1);
            match (path_ends_sep, name_starts_sep) {
                (true, true) => out.extend_from_slice(&path[..path.len() - 1]),
                (false, false) => {
                    out.extend_from_slice(path);
                    if last_unit(path) == COLON {
                        out.extend_from_slice(name);
                        out.push(0);
                        return out;
                    }
                    out.push(BACKSLASH);
                }
                _ => out.extend_from_slice(path),
            }
            out.extend_from_slice(name);
            out
        }
        (Some(s), None) | (None, Some(s)) => until_nul(s).to_vec(),
        (None, None) => Vec::new(),
    }
}

/// The length of the parent folder prefix of `path`, including its trailing separator, or 0 if
/// there is none. Trailing separators are skipped first, so `"C:\\dir\\"` has parent `"C:\\"`.
fn parent_folder_len(path: &[u16]) -> usize {
    let mut i = path.len() as isize - 1;
    while i >= 0 && is_separator(path[i as usize]) {
        i -= 1;
    }
    while i >= 0 && !is_separator(path[i as usize]) {
        i -= 1;
    }
    while i >= 0 && is_separator(path[i as usize]) {
        i -= 1;
    }
    if i < 0 {
        return 0;
    }
    if path[i as usize] == COLON && i == 1 {
        i += 1;
    }
    i as usize + 1
}

/// `IFileSystem3::GetParentFolderName`. `None` when the path has no parent folder.
pub fn parent_folder_name(path: &[u16]) -> Option<Vec<u16>> {
    let len = parent_folder_len(path);
    (len != 0).then(|| path[..len].to_vec())
}

/// `IFileSystem3::GetFileName`: the last component after the final separator, or `None` when
/// there is no such component (an empty path, a root such as `"C:\\"`, or only separators).
pub fn file_name(path: &[u16]) -> Option<Vec<u16>> {
    let path = until_nul(path);
    let mut end = path.len() as isize - 1;
    while end >= 0 && is_separator(path[end as usize]) {
        end -= 1;
    }
    let mut start = end;
    while start >= 0 && !is_separator(path[start as usize]) {
        start -= 1;
    }
    start += 1;
    if start > end || (start == 0 && end == 1 && path[1] == COLON) {
        return None;
    }
    Some(path[start as usize..=end as usize].to_vec())
}

/// `IFileSystem3::GetBaseName`: the file name without its last extension. A leading dot does not
/// start an extension. `None` when there is no name (for example `"C:"`).
pub fn base_name(path: &[u16]) -> Option<Vec<u16>> {
    let path = until_nul(path);
    let mut end = path.len() as isize - 1;
    while end >= 0 && is_separator(path[end as usize]) {
        end -= 1;
    }
    let mut start = end;
    while start >= 0 {
        if path[start as usize] == DOT && unit_or_nul(path, (end + 1) as usize) != DOT {
            end = start - 1;
        }
        if is_separator(path[start as usize]) {
            break;
        }
        start -= 1;
    }
    start += 1;
    if (start > end && unit_or_nul(path, (end + 1) as usize) != DOT)
        || (start == 0 && end == 1 && path[1] == COLON)
    {
        return None;
    }
    Some(path[start as usize..(end + 1) as usize].to_vec())
}

/// `IFileSystem3::GetExtensionName`: the text after the last dot, which may be empty. `None`
/// when the string has no dot.
pub fn extension_name(path: &[u16]) -> Option<Vec<u16>> {
    let dot = path.iter().rposition(|&c| c == DOT)?;
    Some(until_nul(&path[dot + 1..]).to_vec())
}

/// `IFileSystem3::GetDriveName`: the first two code units when the string is `X:...` with at
/// least two units, otherwise `None`.
pub fn drive_name(path: &[u16]) -> Option<Vec<u16>> {
    let path = until_nul(path);
    (path.len() > 1 && path[1] == COLON).then(|| path[..2].to_vec())
}

/// The `CompareMethod` of a dictionary, `scrrun.idl`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompareMethod {
    Binary,
    Text,
    Database,
}

/// `get_str_hash`: a multiplicative string hash modulo `DICT_HASH_MOD`. Text and database
/// comparison fold ASCII `A`-`Z` to lowercase first. An empty or absent string hashes to 0.
pub fn str_hash(s: &[u16], method: CompareMethod) -> u32 {
    let mut hash: u32 = 0;
    for &unit in until_nul(s) {
        let ch = match method {
            CompareMethod::Binary => unit,
            CompareMethod::Text | CompareMethod::Database => ascii_lower(unit),
        };
        hash = hash.wrapping_add((hash << 4).wrapping_add(u32::from(ch)));
    }
    hash % DICT_HASH_MOD
}

fn ascii_lower(c: u16) -> u16 {
    match c {
        0x41..=0x5A => c + 0x20,
        _ => c,
    }
}

/// `get_num_hash`: the bit pattern of a single-precision number modulo `DICT_HASH_MOD`.
pub fn num_hash(num: f32) -> u32 {
    num.to_bits() % DICT_HASH_MOD
}

/// `get_flt_hash`: an infinity hashes to 0 and NaN is rejected with `CTL_E_ILLEGALFUNCTIONCALL`.
pub fn flt_hash(flt: f32) -> Result<u32, u32> {
    if flt.is_infinite() {
        Ok(0)
    } else if !flt.is_nan() {
        Ok(num_hash(flt))
    } else {
        Err(CTL_E_ILLEGALFUNCTIONCALL)
    }
}

/// `create_error`: maps a Win32 error code to the `HRESULT` that `scrrun` reports. Unmapped
/// codes become `E_FAIL`.
pub fn create_error(err: u32) -> u32 {
    match err {
        ERROR_FILE_NOT_FOUND => CTL_E_FILENOTFOUND,
        ERROR_PATH_NOT_FOUND => CTL_E_PATHNOTFOUND,
        ERROR_ACCESS_DENIED => CTL_E_PERMISSIONDENIED,
        ERROR_FILE_EXISTS | ERROR_ALREADY_EXISTS => CTL_E_FILEALREADYEXISTS,
        _ => E_FAIL,
    }
}

/// The `a.b.c.d` text of a `VS_FIXEDFILEINFO` file version, from its `MS` and `LS` halves.
pub fn version_string(file_version_ms: u32, file_version_ls: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        file_version_ms >> 16,
        file_version_ms & 0xFFFF,
        file_version_ls >> 16,
        file_version_ls & 0xFFFF
    )
}

/// `is_dir_data`: a directory entry other than `.` and `..`.
pub fn is_dir_data(attributes: u32, name: &[u16]) -> bool {
    let name = until_nul(name);
    attributes & FILE_ATTRIBUTE_DIRECTORY != 0 && name != [DOT, DOT] && name != [DOT]
}

/// `is_file_data`: an entry without the directory attribute.
pub fn is_file_data(attributes: u32) -> bool {
    attributes & FILE_ATTRIBUTE_DIRECTORY == 0
}

#[cfg(test)]
pub mod edge_checks;
#[cfg(test)]
pub mod oracle_checks;
