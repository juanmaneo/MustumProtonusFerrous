use crate::c_str;

/// Splits an `HlinkCreateFromString` target at its first `#`. A `#` at the start leaves
/// no target. The text after the `#` becomes the location unless one is given.
pub fn split_target<'a>(
    target: Option<&'a [u16]>,
    location: Option<&'a [u16]>,
) -> (Option<&'a [u16]>, Option<&'a [u16]>) {
    let location = location.map(c_str);
    let Some(target) = target else {
        return (None, location);
    };
    let target = c_str(target);
    match target.iter().position(|&unit| unit == u16::from(b'#')) {
        Some(0) => (None, location.or(Some(&target[1..]))),
        Some(hash) => (
            Some(&target[..hash]),
            location.or(Some(&target[hash + 1..])),
        ),
        None => (Some(target), location),
    }
}
