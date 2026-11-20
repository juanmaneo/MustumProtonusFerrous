//! Replays rows of the Wine oracle in `target/canon/combase`, which runs the
//! verbatim `string.c`, `usrmarshal.c` and `combase.c` under ASan and UBSan.
//! Each test names its oracle row in its doc comment. The `ref_nullhdr` row is
//! not replayed: UBSan flags `string.c:63` for it, and this port has no header
//! parameter.

use super::*;

pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

pub(crate) fn hstr(text: &str) -> HString {
    let units = wide(text);
    create_string(Some(&units), units.len() as u32).unwrap()
}

/// Oracle row `create_abc3`: `WindowsCreateString("abc", 3)`.
#[test]
fn create_abc3() {
    let s = create_string(Some(&wide("abc")), 3).unwrap();
    assert_eq!(raw_buffer(&s), wide("abc"));
    assert_eq!(get_len(&s), 3);
}

/// Oracle row `create_null2`: a null pointer with a non-zero length is `E_POINTER`.
#[test]
fn create_null2() {
    assert_eq!(create_string(None, 2), Err(E_POINTER));
}

/// Oracle row `create_emb3`: an embedded NUL is kept in the logical length.
#[test]
fn create_emb3() {
    let s = create_string(Some(&[0x61, 0, 0x62]), 3).unwrap();
    assert_eq!(raw_buffer(&s), [0x61, 0, 0x62]);
    assert_eq!(get_len(&s), 3);
}

/// Oracle rows `hasnull_emb` and `hasnull_abc`: only the first string has an embedded NUL.
#[test]
fn hasnull_emb() {
    assert!(has_embedded_null(&hstr("a\0b")));
    assert!(!has_embedded_null(&hstr("abc")));
}

/// Oracle row `ref_xyz3`: a reference to `"xyz"` whose terminator is in place.
#[test]
fn ref_xyz3() {
    let s = create_string_reference(Some(&[0x78, 0x79, 0x7A, 0]), 3).unwrap();
    assert_eq!(raw_buffer(&s), wide("xyz"));
}

/// Oracle row `ref_bad_term`: a non-zero unit at `ptr[len]` is `E_INVALIDARG`.
#[test]
fn ref_bad_term() {
    assert_eq!(
        create_string_reference(Some(&[0x78, 0x79, 0x7A, 0x71, 0]), 3),
        Err(E_INVALIDARG)
    );
}

/// Oracle row `sub_2`: `WindowsSubstring("hello", 2)` is `"llo"`.
#[test]
fn sub_2() {
    let s = substring(&hstr("hello"), 2).unwrap();
    assert_eq!(raw_buffer(&s), wide("llo"));
}

/// Oracle row `subl_wrap`: `start + len` wraps in 32 bits and is `E_BOUNDS`.
#[test]
fn subl_wrap() {
    assert_eq!(
        substring_with_length(&hstr("hello"), u32::MAX, 2),
        Err(E_BOUNDS)
    );
}

/// Oracle row `cat_ab_cd`: `WindowsConcatString("ab", "cd")` is `"abcd"`.
#[test]
fn cat_ab_cd() {
    let s = concat(&hstr("ab"), &hstr("cd")).unwrap();
    assert_eq!(raw_buffer(&s), wide("abcd"));
}

/// Oracle row `cmp_ab_abc`: a proper prefix sorts first.
#[test]
fn cmp_ab_abc() {
    assert_eq!(compare_ordinal(&hstr("ab"), &hstr("abc")), -1);
}

/// Oracle row `cmp_ffff_one`: code units compare unsigned, so U+FFFF sorts after U+0001.
#[test]
fn cmp_ffff_one() {
    assert_eq!(
        compare_ordinal(&from_units(&[0xFFFF]), &from_units(&[1])),
        1
    );
}

/// Oracle row `trimS_sp`: leading spaces are dropped from `"  ab "`.
#[test]
fn trim_start_sp() {
    let s = trim_start(&hstr("  ab "), &hstr(" ")).unwrap();
    assert_eq!(raw_buffer(&s), wide("ab "));
}

/// Oracle row `trimE_sp`: trailing spaces are dropped from `"  ab "`.
#[test]
fn trim_end_sp() {
    let s = trim_end(&hstr("  ab "), &hstr(" ")).unwrap();
    assert_eq!(raw_buffer(&s), wide("  ab"));
}

/// Oracle row `promote_abc`: three units followed by a zero terminator.
#[test]
fn promote_abc() {
    let mut buf = StringBuffer::new(3);
    buf.as_mut_slice()[..3].copy_from_slice(&wide("abc"));
    let s = buf.promote().unwrap();
    assert_eq!(raw_buffer(&s), wide("abc"));
}

/// Oracle row `promote_term_set`: a non-zero terminator slot is `E_INVALIDARG`.
#[test]
fn promote_term_set() {
    let mut buf = StringBuffer::new(3);
    buf.as_mut_slice()
        .copy_from_slice(&[0x61, 0x62, 0x63, 0x78]);
    assert_eq!(buf.promote(), Err(E_INVALIDARG));
}

/// Oracle row `usize_0_ab`: a local record of two units at start 0 is 12 bytes.
#[test]
fn usize_0_ab() {
    assert_eq!(user_size(0, 0, &hstr("ab")), Ok(12));
}

/// Oracle row `usize_inproc3_ab`: an in-process record at start 3 aligns to 8, then adds 16 bytes.
#[test]
fn usize_inproc3_ab() {
    assert_eq!(user_size(MSHCTX_INPROC, 3, &hstr("ab")), Ok(24));
}

/// Oracle row `umarshal_0_ab`: the record at offset 0 is the context, the byte count and the units, and it ends at 12.
#[test]
fn umarshal_0_ab() {
    let mut buf = [0xAA; 16];
    assert_eq!(user_marshal(0, &mut buf, 0, &hstr("ab")), Ok(12));
    assert_eq!(
        buf[..12],
        [0x57, 0x64, 0x74, 0x50, 4, 0, 0, 0, 0x61, 0, 0x62, 0]
    );
}

/// Oracle row `umarshal_3_ab`: offset 3 aligns the record to byte 8, and it ends at 20.
#[test]
fn umarshal_3_ab() {
    let mut buf = [0xAA; 24];
    assert_eq!(user_marshal(0, &mut buf, 3, &hstr("ab")), Ok(20));
    assert_eq!(buf[..8], [0xAA; 8]);
    assert_eq!(
        buf[8..20],
        [0x57, 0x64, 0x74, 0x50, 4, 0, 0, 0, 0x61, 0, 0x62, 0]
    );
}

/// Oracle row `uunmarshal_size3`: an odd byte count of 3 drops its last byte, leaving `"a"` and an end offset of 10.
#[test]
fn uunmarshal_size3() {
    let buf = [0x57, 0x64, 0x74, 0x50, 3, 0, 0, 0, 0x61, 0, 0x62, 0];
    assert_eq!(user_unmarshal(0, &buf, 0), Ok((10, hstr("a"))));
}

/// Oracle row `umarshal_differentmachine`: `MSHCTX_DIFFERENTMACHINE` marshalling raises `RPC_S_INVALID_TAG` (1733).
#[test]
fn umarshal_differentmachine() {
    let mut buf = [0; 16];
    assert_eq!(
        user_marshal(MSHCTX_DIFFERENTMACHINE, &mut buf, 0, &hstr("ab")),
        Err(WireError::DifferentMachine)
    );
}

/// Oracle row `0000`: `VT_EMPTY` is a valid VARTYPE.
#[test]
fn propvar_0000() {
    assert_eq!(validate_vartype(0x0000), Ok(()));
}

/// Oracle row `000C`: `VT_VARIANT` alone is not a valid VARTYPE.
#[test]
fn propvar_000c() {
    assert_eq!(validate_vartype(0x000C), Err(STG_E_INVALIDPARAMETER));
}

/// Oracle row `100C`: `VT_VARIANT | VT_VECTOR` is a valid VARTYPE.
#[test]
fn propvar_100c() {
    assert_eq!(validate_vartype(0x100C), Ok(()));
}

/// Oracle row `4000`: `VT_BYREF` alone is not a valid VARTYPE.
#[test]
fn propvar_4000() {
    assert_eq!(validate_vartype(0x4000), Err(STG_E_INVALIDPARAMETER));
}
