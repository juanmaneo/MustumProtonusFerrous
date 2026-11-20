//! `get_safearray_size` in `dlls/vbscript/utils.c`.

/// The element count of a `SAFEARRAY`. `dims` holds `cElements` for each of the `cDims`
/// dimensions, and `None` is a null array. The product wraps, as the C `ULONG` does.
pub fn element_count(dims: Option<&[u32]>) -> u32 {
    dims.map_or(0, |dims| {
        dims.iter()
            .fold(1, |count, &elements| count.wrapping_mul(elements))
    })
}
