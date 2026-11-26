//! Port of the portable logic in Wine's `dlls/winscard/winscard.c`.
//!
//! Ported:
//! - `copy_multiszA` and `copy_multiszW`: [`copy_multisz`]. The conversion of one UTF-8
//!   string to ANSI or UTF-16 is an OS call in Wine (`MultiByteToWideChar` and
//!   `WideCharToMultiByte`), so the caller supplies it as the `convert` closure.
//!
//! Skipped:
//! - The `SCard*` entry points and `DllMain`: they call the PC/SC service through `unixlib.c`.
//! - `unixlib.c`, including the `wow64_*` thunks: pcsclite and macOS PC/SC calls and
//!   Unix-side state.
//! - `map_states_inA`, `map_states_inW`, `map_states_out`, `free_states`: copy reader state
//!   between Windows and service structures for `SCardGetStatusChange`.
//! - `utf8_to_utf16`, `utf16_to_ansi`, `utf8_to_ansi`, `utf16_to_utf8`, `ansi_to_utf16`,
//!   `ansi_to_utf8`: wrappers over the OS conversion calls.
//!
//! Differs from C: Wine's loops read past the end of a list that lacks its final empty
//! string, which is undefined behaviour. Here the end of `src` ends the list instead.

mod error;

pub use error::Error;

/// Where [`copy_multisz`] writes. It mirrors the `dst` / `dst_len` pair of the C function.
pub enum Dest<'a, T> {
    /// `dst` is NULL and `dst_len` is not: only the required length is reported.
    Length,
    /// `*dst_len` is `SCARD_AUTOALLOCATE`: the list is returned in a new allocation.
    Allocate,
    /// `dst` points at a caller buffer. Its length is the capacity in units.
    Buffer(&'a mut [T]),
}

/// What [`copy_multisz`] produced. Every length counts the final NUL.
#[derive(Debug, PartialEq, Eq)]
pub enum Written<T> {
    /// Neither a destination nor a length was given (`!dst && !dst_len`).
    Nothing,
    /// The required length, for [`Dest::Length`].
    Length(usize),
    /// The list, for [`Dest::Allocate`].
    Allocated(Vec<T>),
    /// The number of leading units written into the caller's buffer.
    Units(usize),
}

/// `copy_multiszA` / `copy_multiszW`. `src` is a list of NUL-terminated UTF-8 strings,
/// ended by an empty string. `convert` converts the string at the start of the slice it
/// receives, up to its NUL, and returns the units including that NUL, or `None` on failure.
/// `convert` must be deterministic, since the size pass and the copy pass must agree.
pub fn copy_multisz<T>(
    src: &[u8],
    convert: &dyn Fn(&[u8]) -> Option<Vec<T>>,
    dest: Option<Dest<'_, T>>,
) -> Result<Written<T>, Error>
where
    T: Copy + Default,
{
    let Some(dest) = dest else {
        return Ok(Written::Nothing);
    };
    let required = measure(src, convert)?;
    match dest {
        Dest::Length => Ok(Written::Length(required)),
        Dest::Allocate => Ok(Written::Allocated(fill(src, convert)?)),
        Dest::Buffer(buf) if buf.len() < required => Err(Error::InsufficientBuffer { required }),
        Dest::Buffer(buf) => {
            let units = fill(src, convert)?;
            buf[..units.len()].copy_from_slice(&units);
            Ok(Written::Units(units.len()))
        }
    }
}

/// The size pass of `copy_multiszA`: the list length in units, including the final NUL.
fn measure<T>(src: &[u8], convert: &dyn Fn(&[u8]) -> Option<Vec<T>>) -> Result<usize, Error> {
    let mut total = 0;
    walk(src, convert, Error::InvalidParameter, &mut |units| {
        total += units.len();
    })?;
    Ok(total + 1)
}

/// The copy pass of `copy_multiszA`: the converted list, ending with the final NUL.
fn fill<T: Default>(
    src: &[u8],
    convert: &dyn Fn(&[u8]) -> Option<Vec<T>>,
) -> Result<Vec<T>, Error> {
    let mut out = Vec::new();
    walk(src, convert, Error::NoMemory, &mut |units: Vec<T>| {
        out.extend(units)
    })?;
    out.push(T::default());
    Ok(out)
}

/// Converts the strings of `src` in turn. As in the C loop, the position advances by the
/// number of units a string converted to, not by its UTF-8 byte count. Wine behaves this
/// way and the port keeps it. An empty conversion would not advance, so it is a failure.
fn walk<T>(
    src: &[u8],
    convert: &dyn Fn(&[u8]) -> Option<Vec<T>>,
    fail: Error,
    on_units: &mut dyn FnMut(Vec<T>),
) -> Result<(), Error> {
    let mut pos = 0;
    while pos < src.len() && src[pos] != 0 {
        let Some(units) = convert(&src[pos..]).filter(|units| !units.is_empty()) else {
            return Err(fail);
        };
        pos += units.len();
        on_units(units);
    }
    Ok(())
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
