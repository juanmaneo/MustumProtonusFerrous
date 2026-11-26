/// `TTFCFP_SUBSET` from `include/fontsub.h`. The only format `CreateFontPackage` accepts.
pub const TTFCFP_SUBSET: u16 = 0;
/// `TTFCFP_SUBSET1` from `include/fontsub.h`. Rejected with [`ERR_GENERIC`].
pub const TTFCFP_SUBSET1: u16 = 1;
/// `TTFCFP_DELTA` from `include/fontsub.h`. Rejected with [`ERR_GENERIC`].
pub const TTFCFP_DELTA: u16 = 2;

/// `ERR_GENERIC` from `include/fontsub.h`.
pub const ERR_GENERIC: u32 = 1000;
/// `ERR_MEM` from `include/fontsub.h`.
pub const ERR_MEM: u32 = 1005;
/// `NO_ERROR` from `include/winerror.h`.
pub const NO_ERROR: u32 = 0;

/// The out-parameters of `CreateFontPackage`, owned by the caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontPackageOut {
    /// `*dest`. `None` is the NULL that `ERR_MEM` leaves behind.
    pub dest: Option<Vec<u8>>,
    /// `*dest_len`.
    pub dest_len: u32,
    /// `*written`.
    pub written: u32,
}

/// `CreateFontPackage`: the format gate and the copy path.
///
/// Only `TTFCFP_SUBSET` is accepted. The source is copied into a block from `allocproc`.
/// On `ERR_GENERIC` nothing is written and `allocproc` is not called. On `ERR_MEM` only
/// `dest` changes, to `None`. `allocproc` is called once with `src.len()` and must return a
/// block of at least that many bytes. The C code copies without checking, so a shorter block
/// is a contract violation: the port panics where C would overflow. The C `ULONG` length is
/// `src.len()` narrowed to `u32`, which is exact for every length the C ABI can express.
pub fn create_font_package(
    src: &[u8],
    format: u16,
    out: &mut FontPackageOut,
    allocproc: impl FnOnce(usize) -> Option<Vec<u8>>,
) -> u32 {
    if format != TTFCFP_SUBSET {
        return ERR_GENERIC;
    }
    let Some(mut dest) = allocproc(src.len()) else {
        out.dest = None;
        return ERR_MEM;
    };
    dest[..src.len()].copy_from_slice(src);
    out.dest = Some(dest);
    out.dest_len = src.len() as u32;
    out.written = out.dest_len;
    NO_ERROR
}
