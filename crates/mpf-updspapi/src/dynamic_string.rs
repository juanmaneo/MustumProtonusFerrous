/// `UpdSpSetDynamicStringA` from `dlls/updspapi/main.c`. The C body traces its arguments and
/// returns 1. `hinf` is the `HINF` handle as an opaque value, and `str1` and `str2` are the
/// `LPSTR` arguments, with `None` for NULL. The body never reads any of them.
pub fn upd_sp_set_dynamic_string_a(
    _hinf: usize,
    _str1: Option<&[u8]>,
    _str2: Option<&[u8]>,
) -> u32 {
    1
}
