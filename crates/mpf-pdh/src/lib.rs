//! Safe, deterministic wide-string subset of Wine's `dlls/pdh/pdh_main.c`.
//!
//! Ported (4 of the 163 `pdh.spec` entries). Each has a fixed string-only
//! body in Wine that reads only the `counter_sources` table, plus the local
//! computer name for `\\machine` prefixes:
//! - `PdhValidatePathW`: `validate_path_w`.
//! - `PdhLookupPerfIndexByNameW`: `lookup_perf_index_by_name_w`.
//! - `PdhLookupPerfNameByIndexW`: `lookup_perf_name_by_index_w`.
//! - `PdhMakeCounterPathW`: `make_counter_path_w`.
//!
//! Skipped (159 entries):
//! - 17 ANSI `*A` `stdcall` exports (`PdhAddCounterA`, `PdhLookupPerfIndexByNameA`,
//!   `PdhMakeCounterPathA`, `PdhValidatePathA`, ...). Each converts with the host code
//!   page (`CP_ACP`) before calling its W counterpart, so porting would need a
//!   code-page model.
//! - 19 query, counter and handle `stdcall` exports (`PdhOpenQuery*`, `PdhAddCounterW`,
//!   `PdhCollectQueryData*`, `PdhGetFormattedCounter*`, `PdhGetRawCounter*`,
//!   `PdhRemoveCounter`, `PdhSetCounterScaleFactor`, `PdhVbAddCounter`, ...). They
//!   need query or counter handle state and performance-counter collection.
//! - 4 provider enumeration and path expansion `stdcall` exports (`PdhEnumObjectItemsW`,
//!   `PdhEnumObjectsW`, `PdhExpandCounterPathW`, `PdhExpandWildCardPathW`). Wine
//!   implements them as `FIXME` stubs that return `PDH_NOT_IMPLEMENTED`.
//! - 5 log, data-source and remote `stdcall` exports (`PdhBindInputDataSourceW`,
//!   `PdhGetLogFileTypeW`, `PdhValidatePathExA`, `PdhValidatePathExW`,
//!   `PdhSetDefaultRealTimeDataSource`). They depend on log or source handles, or
//!   they are `FIXME` no-ops.
//! - `PdhGetDllVersion`: writes a constant version, with no string logic.
//! - 113 `pdh.spec` stub entries, including the ordinal-form `PdhPlaGetLogFileNameA`.
//!   Wine marks them unimplemented, so they stay unimplemented here.
//!
//! Local-machine matching folds ASCII case only. Wine delegates to the CRT's
//! Unicode-aware `_wcsnicmp`, so this port does not claim Unicode case equivalence.
//! Output lengths are not limited to Wine's fixed 1024-unit stack buffer.
//! `PdhMakeCounterPathW` ignores its `flags` argument, as Wine does.

/// A PDH status code, as Wine's `PDH_STATUS`.
pub type PdhStatus = u32;

/// `ERROR_SUCCESS`.
pub const ERROR_SUCCESS: PdhStatus = 0;
/// `PDH_CSTATUS_NO_MACHINE`: a remote machine was requested.
pub const PDH_CSTATUS_NO_MACHINE: PdhStatus = 0x8000_07d0;
/// `PDH_MORE_DATA`: the output buffer is too small.
pub const PDH_MORE_DATA: PdhStatus = 0x8000_07d2;
/// `PDH_CSTATUS_NO_COUNTER`: the path names no known counter.
pub const PDH_CSTATUS_NO_COUNTER: PdhStatus = 0xc000_0bb9;
/// `PDH_INVALID_ARGUMENT`.
pub const PDH_INVALID_ARGUMENT: PdhStatus = 0xc000_0bbd;
/// `PDH_CSTATUS_BAD_COUNTERNAME`: the path lacks the leading `\` or a second `\`.
pub const PDH_CSTATUS_BAD_COUNTERNAME: PdhStatus = 0xc000_0bc0;
/// `PDH_STRING_NOT_FOUND`.
pub const PDH_STRING_NOT_FOUND: PdhStatus = 0xc000_0bd4;

/// Wine's `counter_sources` table: each counter path and its perf index.
const COUNTER_SOURCES: [(&str, u32); 2] = [
    (r"\Processor(_Total)\% Processor Time", 6),
    (r"\System\System Up Time", 674),
];

const BACKSLASH: u16 = b'\\' as u16;

/// The elements of a counter path, as Wine's `PDH_COUNTER_PATH_ELEMENTS_W`.
///
/// A `None` field is a null pointer in C. An empty field is an empty string.
#[derive(Clone, Copy, Debug)]
pub struct CounterPathElements<'a> {
    pub machine_name: Option<&'a [u16]>,
    pub object_name: Option<&'a [u16]>,
    pub instance_name: Option<&'a [u16]>,
    pub parent_instance: Option<&'a [u16]>,
    pub instance_index: u32,
    pub counter_name: Option<&'a [u16]>,
}

/// The units before the first NUL, as C reads a wide string.
fn wstr(text: &[u16]) -> &[u16] {
    match text.iter().position(|&unit| unit == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// Converts a length to the `u32` that the C API stores, saturating at `u32::MAX`.
fn saturating_u32(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

/// The last `\`-separated part of a counter path, as Wine's `wcsrchr(path, '\\') + 1`.
fn counter_leaf(source: &str) -> &str {
    source.rsplit_once('\\').map_or(source, |(_, leaf)| leaf)
}

fn units_eq(units: &[u16], text: &str) -> bool {
    units.iter().copied().eq(text.encode_utf16())
}

fn ascii_eq_ignore_case(a: u16, b: u16) -> bool {
    if a <= 0x7f && b <= 0x7f {
        (a as u8).eq_ignore_ascii_case(&(b as u8))
    } else {
        a == b
    }
}

/// Wine's `is_local_machine`: `name` must have the computer name's length and match it.
fn is_local_name(name: &[u16], computer: &[u16]) -> bool {
    name.len() == computer.len()
        && name
            .iter()
            .zip(computer)
            .all(|(&a, &b)| ascii_eq_ignore_case(a, b))
}

/// Drops a leading `\\machine` when `machine` is the local computer, as Wine's `pdh_match_path`.
fn strip_local_machine<'a>(path: &'a [u16], computer_name: Option<&[u16]>) -> &'a [u16] {
    let (Some(computer), Some(rest)) = (
        computer_name.map(wstr),
        path.strip_prefix(&[BACKSLASH, BACKSLASH]),
    ) else {
        return path;
    };
    match rest.iter().position(|&unit| unit == BACKSLASH) {
        Some(separator) if is_local_name(&rest[..separator], computer) => &rest[separator..],
        _ => path,
    }
}

/// Wine's `pdh_match_path`: a path with a backslash must equal the whole counter path,
/// and a bare name must equal its last part.
fn matches_source(source: &str, path: &[u16], computer_name: Option<&[u16]>) -> bool {
    let path = strip_local_machine(path, computer_name);
    let expected = if path.contains(&BACKSLASH) {
        source
    } else {
        counter_leaf(source)
    };
    units_eq(path, expected)
}

/// Implements Wine's `PdhValidatePathW` for the two fixed counter sources.
///
/// `path` may lack a NUL terminator. Units after its first NUL are ignored. `computer_name`
/// models `GetComputerNameW`, and `None` models its failure. Paths are not limited to 1024 units.
pub fn validate_path_w(path: Option<&[u16]>, computer_name: Option<&[u16]>) -> PdhStatus {
    let Some(path) = path.map(wstr) else {
        return PDH_INVALID_ARGUMENT;
    };
    let Some((&first, rest)) = path.split_first() else {
        return PDH_INVALID_ARGUMENT;
    };
    if first != BACKSLASH || !rest.contains(&BACKSLASH) {
        return PDH_CSTATUS_BAD_COUNTERNAME;
    }
    if COUNTER_SOURCES
        .iter()
        .any(|(source, _)| matches_source(source, path, computer_name))
    {
        ERROR_SUCCESS
    } else {
        PDH_CSTATUS_NO_COUNTER
    }
}

/// Implements Wine's `PdhLookupPerfIndexByNameW` for the two fixed counter sources.
///
/// `index` is written only on success. A non-null `machine` requests a remote lookup, and
/// Wine returns `PDH_CSTATUS_NO_MACHINE` for it.
pub fn lookup_perf_index_by_name_w(
    machine: Option<&[u16]>,
    name: Option<&[u16]>,
    computer_name: Option<&[u16]>,
    index: Option<&mut u32>,
) -> PdhStatus {
    let (Some(name), Some(index)) = (name.map(wstr), index) else {
        return PDH_INVALID_ARGUMENT;
    };
    if machine.is_some() {
        return PDH_CSTATUS_NO_MACHINE;
    }
    match COUNTER_SOURCES
        .iter()
        .find(|(source, _)| matches_source(source, name, computer_name))
    {
        Some((_, found)) => {
            *index = *found;
            ERROR_SUCCESS
        }
        None => PDH_STRING_NOT_FOUND,
    }
}

/// Implements Wine's `PdhLookupPerfNameByIndexW` for the two fixed counter sources.
///
/// `size` holds the buffer capacity in UTF-16 units on entry, and the required length
/// including the NUL on return. A `None` buffer has no capacity. A buffer shorter than
/// `size` is limited to its own length. Index `0` succeeds without writing, as in Wine.
pub fn lookup_perf_name_by_index_w(
    machine: Option<&[u16]>,
    index: u32,
    buffer: Option<&mut [u16]>,
    size: Option<&mut u32>,
) -> PdhStatus {
    if machine.is_some() {
        return PDH_CSTATUS_NO_MACHINE;
    }
    let (Some(buffer), Some(size)) = (buffer, size) else {
        return PDH_INVALID_ARGUMENT;
    };
    if index == 0 {
        return ERROR_SUCCESS;
    }
    let Some((source, _)) = COUNTER_SOURCES
        .iter()
        .find(|(_, source_index)| *source_index == index)
    else {
        return PDH_INVALID_ARGUMENT;
    };
    let name: Vec<u16> = counter_leaf(source).encode_utf16().collect();
    let required = name.len() + 1;
    let capacity = (*size as usize).min(buffer.len());
    *size = saturating_u32(required);
    if capacity < required {
        PDH_MORE_DATA
    } else {
        buffer[..name.len()].copy_from_slice(&name);
        buffer[name.len()] = 0;
        ERROR_SUCCESS
    }
}

/// Builds `\\machine\object(parent/instance#index)\counter` as Wine's `PdhMakeCounterPathW`.
///
/// Returns `None` when the object or counter name is null, which Wine rejects.
fn counter_path(elements: &CounterPathElements<'_>) -> Option<Vec<u16>> {
    let (Some(object), Some(counter)) = (elements.object_name, elements.counter_name) else {
        return None;
    };
    let mut path = Vec::new();
    if let Some(machine) = elements.machine_name {
        path.extend_from_slice(&[BACKSLASH, BACKSLASH]);
        path.extend_from_slice(wstr(machine));
    }
    path.push(BACKSLASH);
    path.extend_from_slice(wstr(object));
    if let Some(instance) = elements.instance_name {
        path.push(u16::from(b'('));
        if let Some(parent) = elements.parent_instance {
            path.extend_from_slice(wstr(parent));
            path.push(u16::from(b'/'));
        }
        path.extend_from_slice(wstr(instance));
        path.extend(format!("#{}", elements.instance_index).encode_utf16());
        path.push(u16::from(b')'));
    }
    path.push(BACKSLASH);
    path.extend_from_slice(wstr(counter));
    Some(path)
}

/// Implements Wine's `PdhMakeCounterPathW`.
///
/// `buflen` holds the buffer capacity in UTF-16 units on entry, and the required length
/// including the NUL on return. A `None` buffer has no capacity, so it always reports
/// `PDH_MORE_DATA`. `flags` is ignored, as in Wine.
pub fn make_counter_path_w(
    elements: Option<&CounterPathElements<'_>>,
    buffer: Option<&mut [u16]>,
    buflen: Option<&mut u32>,
    _flags: u32,
) -> PdhStatus {
    let (Some(elements), Some(buflen)) = (elements, buflen) else {
        return PDH_INVALID_ARGUMENT;
    };
    let Some(path) = counter_path(elements) else {
        return PDH_INVALID_ARGUMENT;
    };
    let required = path.len() + 1;
    let capacity = buffer
        .as_ref()
        .map_or(0, |output| (*buflen as usize).min(output.len()));
    *buflen = saturating_u32(required);
    match buffer {
        Some(output) if capacity >= required => {
            output[..path.len()].copy_from_slice(&path);
            output[path.len()] = 0;
            ERROR_SUCCESS
        }
        _ => PDH_MORE_DATA,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fill value that the oracle harness gives every output buffer.
    const FILL: u16 = 0xcccc;
    /// The index value that the oracle harness preloads, to show an untouched write.
    const SENTINEL: u32 = 0xdead_beef;

    struct ValidateRow {
        id: &'static str,
        path: Option<&'static str>,
        computer: Option<&'static str>,
        status: PdhStatus,
    }

    struct LookupIndexRow {
        id: &'static str,
        machine: Option<&'static str>,
        name: Option<&'static str>,
        computer: Option<&'static str>,
        has_index: bool,
        status: PdhStatus,
        index: Option<u32>,
    }

    struct LookupNameRow {
        id: &'static str,
        machine: Option<&'static str>,
        index: u32,
        capacity: u32,
        has_buffer: bool,
        has_size: bool,
        status: PdhStatus,
        size: Option<u32>,
        buffer: Option<&'static str>,
    }

    struct MakePathRow {
        id: &'static str,
        has_elements: bool,
        machine: Option<&'static str>,
        object: Option<&'static str>,
        instance: Option<&'static str>,
        parent: Option<&'static str>,
        index: u32,
        counter: Option<&'static str>,
        capacity: u32,
        has_buffer: bool,
        has_len: bool,
        flags: u32,
        status: PdhStatus,
        len: Option<u32>,
        buffer: Option<&'static str>,
    }

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    /// The units up to the last one written, as the oracle's `put_buffer` prints them.
    fn written(buffer: &[u16]) -> Vec<u16> {
        let used = buffer
            .iter()
            .rposition(|&unit| unit != FILL)
            .map_or(0, |last| last + 1);
        buffer[..used].to_vec()
    }
    const VALIDATE_ROWS: [ValidateRow; 24] = [
        ValidateRow {
            id: "V01",
            path: Some("\\Processor(_Total)\\% Processor Time"),
            computer: None,
            status: 0x00000000,
        },
        ValidateRow {
            id: "V02",
            path: Some("\\System\\System Up Time"),
            computer: None,
            status: 0x00000000,
        },
        ValidateRow {
            id: "V03",
            path: Some("\\\\MPFHOST\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0x00000000,
        },
        ValidateRow {
            id: "V04",
            path: Some("\\\\mpfhost\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0x00000000,
        },
        ValidateRow {
            id: "V05",
            path: Some("\\\\MPFHOST\\System\\System Up Time"),
            computer: None,
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V06",
            path: Some("\\\\OTHER\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V07",
            path: Some("\\\\MPFHOSTX\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V08",
            path: Some("\\\\MPFHOS\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V09",
            path: Some("System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0xC0000BC0,
        },
        ValidateRow {
            id: "V10",
            path: Some(""),
            computer: None,
            status: 0xC0000BBD,
        },
        ValidateRow {
            id: "V11",
            path: None,
            computer: None,
            status: 0xC0000BBD,
        },
        ValidateRow {
            id: "V12",
            path: Some("\\System"),
            computer: None,
            status: 0xC0000BC0,
        },
        ValidateRow {
            id: "V13",
            path: Some("\\"),
            computer: None,
            status: 0xC0000BC0,
        },
        ValidateRow {
            id: "V14",
            path: Some("\\\\"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V15",
            path: Some("\\\\\\"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V16",
            path: Some("\\\\MPFHOST"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V17",
            path: Some("\\\\MPFHOST\\"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V18",
            path: Some("\\System\\System Up Time\0junk"),
            computer: None,
            status: 0x00000000,
        },
        ValidateRow {
            id: "V19",
            path: Some("\\System\\System Up Tim"),
            computer: None,
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V20",
            path: Some("\\System\\System Up Time "),
            computer: None,
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V21",
            path: Some("\\system\\system up time"),
            computer: None,
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V22",
            path: Some("\\\\MPF\u{00C9}HOST\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
        ValidateRow {
            id: "V23",
            path: Some("\\\\MPFHOST\\System\\System Up Time"),
            computer: Some("mpfhost"),
            status: 0x00000000,
        },
        ValidateRow {
            id: "V24",
            path: Some("\\\\MPFHOST\\System\\Nope"),
            computer: Some("MPFHOST"),
            status: 0xC0000BB9,
        },
    ];
    const LOOKUP_INDEX_ROWS: [LookupIndexRow; 22] = [
        LookupIndexRow {
            id: "L01",
            machine: None,
            name: Some("\\Processor(_Total)\\% Processor Time"),
            computer: None,
            has_index: true,
            status: 0x00000000,
            index: Some(6),
        },
        LookupIndexRow {
            id: "L02",
            machine: None,
            name: Some("\\System\\System Up Time"),
            computer: None,
            has_index: true,
            status: 0x00000000,
            index: Some(674),
        },
        LookupIndexRow {
            id: "L03",
            machine: None,
            name: Some("% Processor Time"),
            computer: None,
            has_index: true,
            status: 0x00000000,
            index: Some(6),
        },
        LookupIndexRow {
            id: "L04",
            machine: None,
            name: Some("System Up Time"),
            computer: None,
            has_index: true,
            status: 0x00000000,
            index: Some(674),
        },
        LookupIndexRow {
            id: "L05",
            machine: None,
            name: Some("System\\System Up Time"),
            computer: None,
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L06",
            machine: None,
            name: Some("\\\\MPFHOST\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            has_index: true,
            status: 0x00000000,
            index: Some(674),
        },
        LookupIndexRow {
            id: "L07",
            machine: None,
            name: Some("\\\\mpfhost\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            has_index: true,
            status: 0x00000000,
            index: Some(674),
        },
        LookupIndexRow {
            id: "L08",
            machine: None,
            name: Some("\\\\OTHER\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L09",
            machine: None,
            name: Some("\\\\MPFHOST\\System\\System Up Time"),
            computer: None,
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L10",
            machine: None,
            name: Some(""),
            computer: None,
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L11",
            machine: None,
            name: None,
            computer: None,
            has_index: true,
            status: 0xC0000BBD,
            index: None,
        },
        LookupIndexRow {
            id: "L12",
            machine: None,
            name: Some("\\System\\System Up Time"),
            computer: None,
            has_index: false,
            status: 0xC0000BBD,
            index: None,
        },
        LookupIndexRow {
            id: "L13",
            machine: Some("OTHER"),
            name: Some("\\System\\System Up Time"),
            computer: None,
            has_index: true,
            status: 0x800007D0,
            index: None,
        },
        LookupIndexRow {
            id: "L14",
            machine: Some(""),
            name: Some("\\System\\System Up Time"),
            computer: None,
            has_index: true,
            status: 0x800007D0,
            index: None,
        },
        LookupIndexRow {
            id: "L15",
            machine: Some("MPFHOST"),
            name: Some("\\System\\System Up Time"),
            computer: Some("MPFHOST"),
            has_index: true,
            status: 0x800007D0,
            index: None,
        },
        LookupIndexRow {
            id: "L16",
            machine: Some("OTHER"),
            name: None,
            computer: None,
            has_index: true,
            status: 0xC0000BBD,
            index: None,
        },
        LookupIndexRow {
            id: "L17",
            machine: None,
            name: Some("\\System\\System Up Time\0zzz"),
            computer: None,
            has_index: true,
            status: 0x00000000,
            index: Some(674),
        },
        LookupIndexRow {
            id: "L18",
            machine: None,
            name: Some("\\System\\Sy\u{00E9}tem Up Time"),
            computer: None,
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L19",
            machine: None,
            name: Some("\\\\MPFHOST"),
            computer: Some("MPFHOST"),
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L20",
            machine: None,
            name: Some("% processor time"),
            computer: None,
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
        LookupIndexRow {
            id: "L21",
            machine: Some("OTHER"),
            name: Some("\\System\\System Up Time"),
            computer: None,
            has_index: false,
            status: 0xC0000BBD,
            index: None,
        },
        LookupIndexRow {
            id: "L22",
            machine: None,
            name: Some("\\System\\System Up Time\\"),
            computer: None,
            has_index: true,
            status: 0xC0000BD4,
            index: None,
        },
    ];
    const LOOKUP_NAME_ROWS: [LookupNameRow; 22] = [
        LookupNameRow {
            id: "N01",
            machine: None,
            index: 6,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0x00000000,
            size: Some(17),
            buffer: Some("% Processor Time\0"),
        },
        LookupNameRow {
            id: "N02",
            machine: None,
            index: 674,
            capacity: 15,
            has_buffer: true,
            has_size: true,
            status: 0x00000000,
            size: Some(15),
            buffer: Some("System Up Time\0"),
        },
        LookupNameRow {
            id: "N03",
            machine: None,
            index: 6,
            capacity: 100,
            has_buffer: true,
            has_size: true,
            status: 0x00000000,
            size: Some(17),
            buffer: Some("% Processor Time\0"),
        },
        LookupNameRow {
            id: "N04",
            machine: None,
            index: 6,
            capacity: 16,
            has_buffer: true,
            has_size: true,
            status: 0x800007D2,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N05",
            machine: None,
            index: 674,
            capacity: 14,
            has_buffer: true,
            has_size: true,
            status: 0x800007D2,
            size: Some(15),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N06",
            machine: None,
            index: 6,
            capacity: 0,
            has_buffer: true,
            has_size: true,
            status: 0x800007D2,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N07",
            machine: None,
            index: 0,
            capacity: 16,
            has_buffer: true,
            has_size: true,
            status: 0x00000000,
            size: Some(16),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N08",
            machine: None,
            index: 0,
            capacity: 0,
            has_buffer: true,
            has_size: true,
            status: 0x00000000,
            size: Some(0),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N09",
            machine: None,
            index: 1,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0xC0000BBD,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N10",
            machine: None,
            index: 7,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0xC0000BBD,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N11",
            machine: None,
            index: 673,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0xC0000BBD,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N12",
            machine: None,
            index: 4294967295,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0xC0000BBD,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N13",
            machine: Some("OTHER"),
            index: 6,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0x800007D0,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N14",
            machine: Some(""),
            index: 6,
            capacity: 17,
            has_buffer: true,
            has_size: true,
            status: 0x800007D0,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N15",
            machine: None,
            index: 6,
            capacity: 17,
            has_buffer: false,
            has_size: true,
            status: 0xC0000BBD,
            size: Some(17),
            buffer: None,
        },
        LookupNameRow {
            id: "N16",
            machine: None,
            index: 6,
            capacity: 17,
            has_buffer: true,
            has_size: false,
            status: 0xC0000BBD,
            size: None,
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N17",
            machine: None,
            index: 6,
            capacity: 17,
            has_buffer: false,
            has_size: false,
            status: 0xC0000BBD,
            size: None,
            buffer: None,
        },
        LookupNameRow {
            id: "N18",
            machine: None,
            index: 0,
            capacity: 16,
            has_buffer: false,
            has_size: true,
            status: 0xC0000BBD,
            size: Some(16),
            buffer: None,
        },
        LookupNameRow {
            id: "N19",
            machine: None,
            index: 674,
            capacity: 100,
            has_buffer: true,
            has_size: true,
            status: 0x00000000,
            size: Some(15),
            buffer: Some("System Up Time\0"),
        },
        LookupNameRow {
            id: "N20",
            machine: Some("MPFHOST"),
            index: 0,
            capacity: 16,
            has_buffer: true,
            has_size: true,
            status: 0x800007D0,
            size: Some(16),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N21",
            machine: None,
            index: 6,
            capacity: 1,
            has_buffer: true,
            has_size: true,
            status: 0x800007D2,
            size: Some(17),
            buffer: Some(""),
        },
        LookupNameRow {
            id: "N22",
            machine: None,
            index: 674,
            capacity: 1,
            has_buffer: true,
            has_size: true,
            status: 0x800007D2,
            size: Some(15),
            buffer: Some(""),
        },
    ];
    const MAKE_PATH_ROWS: [MakePathRow; 25] = [
        MakePathRow { id: "P01", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(23), buffer: Some("\\System\\System Up Time\0") },
        MakePathRow { id: "P02", has_elements: true, machine: Some("MPFHOST"), object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(32), buffer: Some("\\\\MPFHOST\\System\\System Up Time\0") },
        MakePathRow { id: "P03", has_elements: true, machine: None, object: Some("Processor"), instance: Some("_Total"), parent: None, index: 0, counter: Some("% Processor Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(38), buffer: Some("\\Processor(_Total#0)\\% Processor Time\0") },
        MakePathRow { id: "P04", has_elements: true, machine: None, object: Some("Processor"), instance: Some("_Total"), parent: None, index: 4294967295, counter: Some("% Processor Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(47), buffer: Some("\\Processor(_Total#4294967295)\\% Processor Time\0") },
        MakePathRow { id: "P05", has_elements: true, machine: None, object: Some("Process"), instance: Some("inst"), parent: Some("Parent"), index: 2, counter: Some("ID"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(27), buffer: Some("\\Process(Parent/inst#2)\\ID\0") },
        MakePathRow { id: "P06", has_elements: true, machine: None, object: Some("Obj"), instance: None, parent: Some("Par"), index: 0, counter: Some("C"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(7), buffer: Some("\\Obj\\C\0") },
        MakePathRow { id: "P07", has_elements: true, machine: None, object: Some("Obj"), instance: Some(""), parent: None, index: 5, counter: Some("C"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(11), buffer: Some("\\Obj(#5)\\C\0") },
        MakePathRow { id: "P08", has_elements: true, machine: None, object: Some("Obj"), instance: Some(""), parent: Some("Par"), index: 5, counter: Some("C"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(15), buffer: Some("\\Obj(Par/#5)\\C\0") },
        MakePathRow { id: "P09", has_elements: true, machine: None, object: Some("Obj"), instance: Some("inst"), parent: Some(""), index: 1, counter: Some("C"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(16), buffer: Some("\\Obj(/inst#1)\\C\0") },
        MakePathRow { id: "P10", has_elements: true, machine: Some(""), object: Some("Obj"), instance: None, parent: None, index: 0, counter: Some("C"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(9), buffer: Some("\\\\\\Obj\\C\0") },
        MakePathRow { id: "P11", has_elements: true, machine: None, object: Some(""), instance: None, parent: None, index: 0, counter: Some("C"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(4), buffer: Some("\\\\C\0") },
        MakePathRow { id: "P12", has_elements: true, machine: None, object: Some("Obj"), instance: None, parent: None, index: 0, counter: Some(""), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(6), buffer: Some("\\Obj\\\0") },
        MakePathRow { id: "P13", has_elements: false, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0xC0000BBD, len: Some(100), buffer: Some("") },
        MakePathRow { id: "P14", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: None, capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0xC0000BBD, len: Some(100), buffer: Some("") },
        MakePathRow { id: "P15", has_elements: true, machine: None, object: None, instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0xC0000BBD, len: Some(100), buffer: Some("") },
        MakePathRow { id: "P16", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 100, has_buffer: true, has_len: false, flags: 0, status: 0xC0000BBD, len: None, buffer: Some("") },
        MakePathRow { id: "P17", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 0, has_buffer: false, has_len: true, flags: 0, status: 0x800007D2, len: Some(23), buffer: None },
        MakePathRow { id: "P18", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 22, has_buffer: true, has_len: true, flags: 0, status: 0x800007D2, len: Some(23), buffer: Some("") },
        MakePathRow { id: "P19", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 23, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(23), buffer: Some("\\System\\System Up Time\0") },
        MakePathRow { id: "P20", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 1, has_buffer: true, has_len: true, flags: 0, status: 0x800007D2, len: Some(23), buffer: Some("") },
        MakePathRow { id: "P21", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 23, has_buffer: true, has_len: true, flags: 2147483648, status: 0x00000000, len: Some(23), buffer: Some("\\System\\System Up Time\0") },
        MakePathRow { id: "P22", has_elements: true, machine: Some("MPFHOST"), object: Some("Process"), instance: Some("inst"), parent: Some("Par"), index: 3, counter: Some("ID"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(33), buffer: Some("\\\\MPFHOST\\Process(Par/inst#3)\\ID\0") },
        MakePathRow { id: "P23", has_elements: true, machine: Some("MPF\u{00C9}HOST"), object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(33), buffer: Some("\\\\MPF\u{00C9}HOST\\System\\System Up Time\0") },
        MakePathRow { id: "P24", has_elements: true, machine: None, object: Some("System"), instance: None, parent: None, index: 0, counter: Some("System Up\0Time"), capacity: 100, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(18), buffer: Some("\\System\\System Up\0") },
        MakePathRow { id: "P25", has_elements: true, machine: None, object: Some("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"), instance: None, parent: None, index: 0, counter: Some("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"), capacity: 1024, has_buffer: true, has_len: true, flags: 0, status: 0x00000000, len: Some(1024), buffer: Some("\\AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\\BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\0") },
    ];

    #[test]
    fn validate_matches_oracle_rows() {
        for row in VALIDATE_ROWS {
            let path = row.path.map(wide);
            let computer = row.computer.map(wide);
            assert_eq!(
                validate_path_w(path.as_deref(), computer.as_deref()),
                row.status,
                "{}",
                row.id
            );
        }
    }

    #[test]
    fn lookup_index_matches_oracle_rows() {
        for row in LOOKUP_INDEX_ROWS {
            let machine = row.machine.map(wide);
            let name = row.name.map(wide);
            let computer = row.computer.map(wide);
            let mut index = SENTINEL;
            let status = lookup_perf_index_by_name_w(
                machine.as_deref(),
                name.as_deref(),
                computer.as_deref(),
                row.has_index.then_some(&mut index),
            );
            let index_out = (index != SENTINEL).then_some(index);
            assert_eq!((status, index_out), (row.status, row.index), "{}", row.id);
        }
    }

    #[test]
    fn lookup_name_matches_oracle_rows() {
        for row in LOOKUP_NAME_ROWS {
            let machine = row.machine.map(wide);
            let mut buffer = vec![FILL; row.capacity as usize];
            let mut size = row.capacity;
            let status = lookup_perf_name_by_index_w(
                machine.as_deref(),
                row.index,
                row.has_buffer.then_some(buffer.as_mut_slice()),
                row.has_size.then_some(&mut size),
            );
            let buffer_out = row.has_buffer.then(|| written(&buffer));
            let size_out = row.has_size.then_some(size);
            let expected_buffer = row.buffer.map(wide);
            assert_eq!(
                (status, size_out, buffer_out),
                (row.status, row.size, expected_buffer),
                "{}",
                row.id
            );
        }
    }

    #[test]
    fn make_path_matches_oracle_rows() {
        for row in MAKE_PATH_ROWS {
            let machine = row.machine.map(wide);
            let object = row.object.map(wide);
            let instance = row.instance.map(wide);
            let parent = row.parent.map(wide);
            let counter = row.counter.map(wide);
            let elements = CounterPathElements {
                machine_name: machine.as_deref(),
                object_name: object.as_deref(),
                instance_name: instance.as_deref(),
                parent_instance: parent.as_deref(),
                instance_index: row.index,
                counter_name: counter.as_deref(),
            };
            let mut buffer = vec![FILL; row.capacity as usize];
            let mut len = row.capacity;
            let status = make_counter_path_w(
                row.has_elements.then_some(&elements),
                row.has_buffer.then_some(buffer.as_mut_slice()),
                row.has_len.then_some(&mut len),
                row.flags,
            );
            let buffer_out = row.has_buffer.then(|| written(&buffer));
            let len_out = row.has_len.then_some(len);
            let expected_buffer = row.buffer.map(wide);
            assert_eq!(
                (status, len_out, buffer_out),
                (row.status, row.len, expected_buffer),
                "{}",
                row.id
            );
        }
    }

    #[test]
    fn unterminated_paths_are_read_to_their_end() {
        let path = wide(r"\System\System Up Time");
        assert_eq!(validate_path_w(Some(path.as_slice()), None), ERROR_SUCCESS);
        let mut index = SENTINEL;
        assert_eq!(
            lookup_perf_index_by_name_w(None, Some(path.as_slice()), None, Some(&mut index)),
            ERROR_SUCCESS
        );
        assert_eq!(index, 674);
    }

    #[test]
    fn non_ascii_machine_names_fold_ascii_case_only() {
        let path = wide("\\\\MPF\u{c9}\\System\\System Up Time");
        let same = wide("MPF\u{c9}");
        let other_case = wide("MPF\u{e9}");
        assert_eq!(
            validate_path_w(Some(path.as_slice()), Some(same.as_slice())),
            ERROR_SUCCESS
        );
        assert_eq!(
            validate_path_w(Some(path.as_slice()), Some(other_case.as_slice())),
            PDH_CSTATUS_NO_COUNTER
        );
    }

    #[test]
    fn name_buffer_is_limited_by_its_slice_length() {
        let mut buffer = [FILL; 16];
        let mut size = 100;
        assert_eq!(
            lookup_perf_name_by_index_w(None, 6, Some(buffer.as_mut_slice()), Some(&mut size)),
            PDH_MORE_DATA
        );
        assert_eq!(size, 17);
        assert!(buffer.iter().all(|&unit| unit == FILL));

        let mut buffer = [FILL; 17];
        let mut size = 5;
        assert_eq!(
            lookup_perf_name_by_index_w(None, 6, Some(buffer.as_mut_slice()), Some(&mut size)),
            PDH_MORE_DATA
        );
        assert_eq!(size, 17);
        assert!(buffer.iter().all(|&unit| unit == FILL));
    }

    #[test]
    fn make_path_is_limited_by_its_declared_capacity() {
        let object = wide("System");
        let counter = wide("System Up Time");
        let elements = CounterPathElements {
            machine_name: None,
            object_name: Some(object.as_slice()),
            instance_name: None,
            parent_instance: None,
            instance_index: 0,
            counter_name: Some(counter.as_slice()),
        };
        let mut buffer = [FILL; 100];
        let mut len = 10;
        assert_eq!(
            make_counter_path_w(
                Some(&elements),
                Some(buffer.as_mut_slice()),
                Some(&mut len),
                0
            ),
            PDH_MORE_DATA
        );
        assert_eq!(len, 23);
        assert!(buffer.iter().all(|&unit| unit == FILL));
    }

    #[test]
    fn make_path_is_not_limited_to_wine_stack_buffer() {
        let object = vec![u16::from(b'A'); 1500];
        let counter = wide("C");
        let elements = CounterPathElements {
            machine_name: None,
            object_name: Some(object.as_slice()),
            instance_name: None,
            parent_instance: None,
            instance_index: 0,
            counter_name: Some(counter.as_slice()),
        };
        let mut buffer = vec![FILL; 1504];
        let mut len = 1504;
        assert_eq!(
            make_counter_path_w(
                Some(&elements),
                Some(buffer.as_mut_slice()),
                Some(&mut len),
                0
            ),
            ERROR_SUCCESS
        );
        assert_eq!(len, 1504);
        assert_eq!(buffer[0], BACKSLASH);
        assert_eq!(buffer[1502], u16::from(b'C'));
        assert_eq!(buffer[1503], 0);
    }
}
