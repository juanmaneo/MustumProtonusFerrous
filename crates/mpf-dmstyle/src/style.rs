//! The motif enumeration of `IDirectMusicStyle8` in `dlls/dmstyle/style.c`.

use crate::{Hresult, E_POINTER, S_FALSE, S_OK};

/// `DMUS_MAX_NAME`: the units of a name buffer, including the terminating NUL.
pub const DMUS_MAX_NAME: usize = 0x40;

/// `DMUS_OBJ_NAME`: the `dwValidData` bit that says `wszName` holds a name.
pub const DMUS_OBJ_NAME: u32 = 0x004;

/// A motif pattern of a style: the `dwValidData` flags of its `DMUS_OBJECTDESC` and its `wszName`.
/// The name ends at its first NUL unit, or at the end of the slice.
pub struct Pattern<'a> {
    pub valid_data: u32,
    pub name: &'a [u16],
}

/// `style_EnumMotif`: copies the name of the pattern at `index` into `name`. A missing `name` is
/// `E_POINTER`, and an `index` past the last pattern is `S_FALSE` with `name` left untouched. For
/// an unnamed pattern, `name` becomes the empty string.
pub fn enum_motif(
    patterns: &[Pattern<'_>],
    index: u32,
    name: Option<&mut [u16; DMUS_MAX_NAME]>,
) -> Hresult {
    let Some(name) = name else {
        return E_POINTER;
    };
    let Some(pattern) = patterns.get(index as usize) else {
        return S_FALSE;
    };
    if (pattern.valid_data & DMUS_OBJ_NAME) != 0 {
        copy_name(name, pattern.name);
    } else {
        name[0] = 0;
    }
    S_OK
}

/// `lstrcpynW(name, src, DMUS_MAX_NAME)`: copies `src` up to its NUL, and at most `DMUS_MAX_NAME - 1`
/// units, then terminates the copy with a NUL.
fn copy_name(name: &mut [u16; DMUS_MAX_NAME], src: &[u16]) {
    let len = src
        .iter()
        .take(DMUS_MAX_NAME - 1)
        .take_while(|&&unit| unit != 0)
        .count();
    name[..len].copy_from_slice(&src[..len]);
    name[len] = 0;
}
