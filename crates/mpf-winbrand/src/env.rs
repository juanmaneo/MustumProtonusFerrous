/// `NTSTATUS`, as the `STATUS_*` values in `include/ntstatus.h` are stored.
pub type NtStatus = i32;

/// `STATUS_SUCCESS` from `include/ntstatus.h`.
pub const STATUS_SUCCESS: NtStatus = 0;

/// `STATUS_BUFFER_TOO_SMALL` from `include/ntstatus.h`.
pub const STATUS_BUFFER_TOO_SMALL: NtStatus = 0xC000_0023u32 as i32;

const PERCENT: u16 = b'%' as u16;
const EQUALS: u16 = b'=' as u16;

/// `ENV_FindVariable` from `dlls/ntdll/env.c`: the value of `name` in the environment block `env`.
/// The block is a sequence of NUL-terminated `NAME=VALUE` entries, and an empty entry ends it.
/// Names match ASCII case-insensitively, and the name must end at the first `=` after the first
/// character of the entry.
pub fn find_environment_variable<'a>(env: &'a [u16], name: &[u16]) -> Option<&'a [u16]> {
    env.split(|&c| c == 0)
        .take_while(|entry| !entry.is_empty())
        .find(|entry| {
            entry.len() > name.len()
                && entry[name.len()] == EQUALS
                && eq_ignore_ascii_case(&entry[..name.len()], name)
                && entry
                    .iter()
                    .skip(1)
                    .position(|&c| c == EQUALS)
                    .map(|i| i + 1)
                    == Some(name.len())
        })
        .map(|entry| &entry[name.len() + 1..])
}

/// `RtlExpandEnvironmentStrings` from `dlls/ntdll/env.c`, for a non-NULL environment block.
/// `src` is expanded into `dst`, which receives at most `dst.len()` characters and is NUL-terminated
/// when it has room. Returns the status and the length of the expansion in characters, including
/// the terminating NUL. An empty `dst` only asks for the length.
pub fn expand_environment_strings(
    env: &[u16],
    mut src: &[u16],
    dst: &mut [u16],
) -> (NtStatus, usize) {
    let mut count = dst.len();
    let mut pos = 0;
    let mut total_size = 1;
    while !src.is_empty() {
        let (var, len, mut copy) = if src[0] != PERCENT {
            let len = src.iter().position(|&c| c == PERCENT).unwrap_or(src.len());
            let segment = &src[..len];
            src = &src[len..];
            (segment, len, len)
        } else if let Some(end) = src[1..].iter().position(|&c| c == PERCENT).map(|i| i + 1) {
            match find_environment_variable(env, &src[1..end]) {
                Some(value) => {
                    src = &src[end + 1..];
                    let copy = if count <= value.len() { 0 } else { value.len() };
                    if count != 0 {
                        dst[pos + copy] = 0;
                    }
                    (value, value.len(), copy)
                }
                None => {
                    let len = end + 1;
                    let segment = &src[..len];
                    src = &src[len..];
                    (segment, len, len)
                }
            }
        } else {
            let segment = src;
            src = &[];
            (segment, segment.len(), segment.len())
        };
        total_size += len;
        if count != 0 {
            let written = len.min(count);
            if count <= copy {
                copy = count - 1;
            }
            dst[pos..pos + copy].copy_from_slice(&var[..copy]);
            count -= written;
            pos += copy;
        }
    }
    if count != 0 {
        dst[pos] = 0;
    }
    let status = if count != 0 {
        STATUS_SUCCESS
    } else {
        STATUS_BUFFER_TOO_SMALL
    };
    (status, total_size)
}

fn eq_ignore_ascii_case(a: &[u16], b: &[u16]) -> bool {
    a.iter()
        .zip(b)
        .all(|(&x, &y)| upcase_ascii(x) == upcase_ascii(y))
}

fn upcase_ascii(c: u16) -> u16 {
    if (b'a' as u16..=b'z' as u16).contains(&c) {
        c - 0x20
    } else {
        c
    }
}
