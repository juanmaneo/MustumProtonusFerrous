//! Edge checks for the branches that the oracle rows do not reach. Each assertion's comment says
//! where its expected value comes from: `C:` a row that the verbatim Wine C printed in
//! `target/canon/propsys`, `C source:` a branch read in the C but not run, and `port:` a choice
//! of this crate where the C is undefined or reads memory it should not.

use super::*;

type GuidFields = (u32, u16, u16, [u8; 8]);

const SAMPLE: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x9abc,
    data3: 0xdef0,
    data4: [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0],
};
const SAMPLE_TEXT: &str = "{12345678-9abc-def0-1234-56789abcdef0}";
const SAMPLE_FIELDS: GuidFields = (
    0x1234_5678,
    0x9abc,
    0xdef0,
    [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0],
);
const ZERO_FIELDS: GuidFields = (0, 0, 0, [0; 8]);
const SENTINEL_FIELDS: GuidFields = (0xDEAD_BEEF, 0xFFFF, 0xFFFF, [0xAA; 8]);

fn wide(text: &str) -> Vec<u16> {
    text.bytes().map(u16::from).collect()
}

fn lpw(text: &str) -> PropValue {
    PropValue::LpWStr(Some(wide(text)))
}

fn bstr(text: &str) -> PropValue {
    PropValue::Bstr(Some(wide(text)))
}

fn ansi(text: &str) -> PropValue {
    PropValue::LpStr(Some(text.as_bytes().to_vec()))
}

/// `SAMPLE_TEXT` with the unit at `at` replaced by `c`.
fn lpw_with(at: usize, c: u8) -> PropValue {
    let mut units = wide(SAMPLE_TEXT);
    units[at] = u16::from(c);
    PropValue::LpWStr(Some(units))
}

fn guid_fields(guid: &Guid) -> GuidFields {
    (guid.data1, guid.data2, guid.data3, guid.data4)
}

fn sentinel() -> Guid {
    Guid {
        data1: 0xDEAD_BEEF,
        data2: 0xFFFF,
        data3: 0xFFFF,
        data4: [0xAA; 8],
    }
}

fn guid_from(value: &PropValue) -> (HResult, GuidFields) {
    let mut guid = sentinel();
    let hr = prop_variant_to_guid(value, &mut guid);
    (hr, guid_fields(&guid))
}

fn key_from(text: &str) -> (HResult, GuidFields, u32) {
    key_from_units(&wide(text))
}

/// Parses into a key that starts as `sentinel()` with pid 7, so the zeroing is visible.
fn key_from_units(text: &[u16]) -> (HResult, GuidFields, u32) {
    let mut key = PropertyKey {
        fmtid: sentinel(),
        pid: 7,
    };
    let hr = ps_property_key_from_string(Some(text), Some(&mut key));
    (hr, guid_fields(&key.fmtid), key.pid)
}

/// Formats into a buffer of `len` units that starts as 0xCCCC.
fn key_to(key: Option<&PropertyKey>, len: usize) -> (HResult, Vec<u16>) {
    let mut buf = vec![0xCCCC_u16; len];
    let hr = ps_string_from_property_key(key, Some(buf.as_mut_slice()));
    (hr, buf)
}

fn sample_key(pid: u32) -> PropertyKey {
    PropertyKey { fmtid: SAMPLE, pid }
}

fn i16_of(value: &PropValue) -> (HResult, i16) {
    let mut out = 7;
    let hr = prop_variant_to_int16(value, &mut out);
    (hr, out)
}

fn i32_of(value: &PropValue) -> (HResult, i32) {
    let mut out = 7;
    let hr = prop_variant_to_int32(value, &mut out);
    (hr, out)
}

fn i64_of(value: &PropValue) -> (HResult, i64) {
    let mut out = 7;
    let hr = prop_variant_to_int64(value, &mut out);
    (hr, out)
}

fn u16_of(value: &PropValue) -> (HResult, u16) {
    let mut out = 7;
    let hr = prop_variant_to_uint16(value, &mut out);
    (hr, out)
}

fn u32_of(value: &PropValue) -> (HResult, u32) {
    let mut out = 7;
    let hr = prop_variant_to_uint32(value, &mut out);
    (hr, out)
}

fn u64_of(value: &PropValue) -> (HResult, u64) {
    let mut out = 7;
    let hr = prop_variant_to_uint64(value, &mut out);
    (hr, out)
}

fn bool_of(value: &PropValue) -> (HResult, bool) {
    let mut out = true;
    let hr = prop_variant_to_boolean(value, &mut out);
    (hr, out)
}

/// The bits of the output, so that an untouched 7.5 can be compared exactly.
fn double_of(value: &PropValue) -> (HResult, u64) {
    let mut out = 7.5_f64;
    let hr = prop_variant_to_double(value, &mut out);
    (hr, out.to_bits())
}

fn buffer_of(value: &PropValue, len: usize) -> (HResult, Vec<u8>) {
    let mut out = vec![0xCC_u8; len];
    let hr = prop_variant_to_buffer(value, &mut out);
    (hr, out)
}

fn str_default_of(value: &PropValue, default: Option<&str>) -> Option<Vec<u16>> {
    let default = default.map(wide);
    prop_variant_to_string_with_default(value, default.as_deref()).map(<[u16]>::to_vec)
}

#[test]
fn null_pointers_report_e_pointer_and_leave_outputs_alone() {
    // C: `if (!pszString || !pkey) return E_POINTER;` and `if (!psz) return E_POINTER;`
    let text = wide(SAMPLE_TEXT);
    let mut key = PropertyKey {
        fmtid: sentinel(),
        pid: 7,
    };
    assert_eq!(ps_property_key_from_string(None, Some(&mut key)), E_POINTER);
    assert_eq!((guid_fields(&key.fmtid), key.pid), (SENTINEL_FIELDS, 7));
    assert_eq!(
        ps_property_key_from_string(Some(text.as_slice()), None),
        E_POINTER
    );
    assert_eq!(ps_property_key_from_string(None, None), E_POINTER);
    assert_eq!(
        ps_string_from_property_key(Some(&sample_key(25)), None),
        E_POINTER
    );
    assert_eq!(ps_string_from_property_key(None, None), E_POINTER);
    // port: a NULL `puuid` is `E_POINTER`, where C crashes in `memcpy`
    assert_eq!(
        guid_from(&PropValue::Clsid(None)),
        (E_POINTER, SENTINEL_FIELDS)
    );
}

#[test]
fn key_pid_follows_the_comma_and_minus_rules() {
    // C: PSPropertyKeyFromString after the GUID. The `" 25"`-style rows are C rows, and the
    // rest are read from the C source. A failure after the GUID keeps pid 0 (memset).
    let rows: [(&str, HResult, u32); 23] = [
        ("", E_INVALIDARG, 0),
        (" ", E_INVALIDARG, 0),
        (",", E_INVALIDARG, 0),
        (" ,", E_INVALIDARG, 0),
        ("-", S_OK, 0),
        (" -", S_OK, 0),
        (",-", S_OK, 0),
        (",,", S_OK, 0),
        (",,,", S_OK, 0),
        (",-,", S_OK, 0),
        (",--25", S_OK, 0),
        (",,25", S_OK, 0),
        (",25", S_OK, 25),
        (" , 25", S_OK, 25),
        (" ,-25", S_OK, 4_294_967_271),
        (" 25", S_OK, 25),
        ("25a", S_OK, 25),
        ("-- 25", S_OK, 4_294_967_271),
        ("- - 25", S_OK, 4_294_967_271),
        ("  - 25", S_OK, 25),
        ("-25", S_OK, 25),
        (" 99999999999", S_OK, 1_215_752_191),
        (" 4294967295", S_OK, 4_294_967_295),
    ];
    for (tail, hr, pid) in rows {
        let text = format!("{SAMPLE_TEXT}{tail}");
        assert_eq!(key_from(&text), (hr, SAMPLE_FIELDS, pid), "tail {tail:?}");
    }
}

#[test]
fn key_rejects_a_bad_character_at_every_guid_index() {
    // C: validate_indices rejects every index that is not its separator or a hex digit
    for at in 0..38 {
        let mut text = wide(SAMPLE_TEXT);
        text[at] = u16::from(b'g');
        assert_eq!(key_from_units(&text).0, E_INVALIDARG, "index {at}");
    }
}

#[test]
fn key_fields_before_a_bad_index_stay_written() {
    // C: string_to_guid writes each field once its indices validate, then returns FALSE
    let cases: [(&str, GuidFields); 5] = [
        (
            "{12345678-zzzz-def0-1234-56789abcdef0} 25",
            (0x1234_5678, 0, 0, [0; 8]),
        ),
        (
            "{12345678-9abc-zef0-1234-56789abcdef0} 25",
            (0x1234_5678, 0x9abc, 0, [0; 8]),
        ),
        (
            "{12345678-9abc-def0-g234-56789abcdef0} 25",
            (0x1234_5678, 0x9abc, 0xdef0, [0; 8]),
        ),
        (
            "{12345678-9abc-def0-1234-g6789abcdef0} 25",
            (0x1234_5678, 0x9abc, 0xdef0, [0x12, 0x34, 0, 0, 0, 0, 0, 0]),
        ),
        (
            "{12345678-9abc-def0-1234-56789abcdefg} 25",
            (
                0x1234_5678,
                0x9abc,
                0xdef0,
                [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0],
            ),
        ),
    ];
    for (text, fields) in cases {
        assert_eq!(key_from(text), (E_INVALIDARG, fields, 0), "{text}");
    }
    assert_eq!(key_from("1234567g"), (E_INVALIDARG, ZERO_FIELDS, 0));
}

#[test]
fn key_reads_a_data1_nibble_of_eight_or_more() {
    // port: C shifts the nibble into the sign bit of an `int`, which is undefined
    // (sanitizer-flagged). The port reads the nibble as a plain hex digit.
    assert_eq!(
        key_from("{F2345678-9abc-def0-1234-56789abcdef0} 25"),
        (S_OK, (0xF234_5678, 0x9abc, 0xdef0, SAMPLE.data4), 25)
    );
}

#[test]
fn key_string_needs_more_than_forty_units() {
    // C: `if (cch <= GUIDSTRING_MAX + 1) return E_NOT_SUFFICIENT_BUFFER;` writes nothing
    for len in [0, 1, 39, 40] {
        assert_eq!(
            key_to(Some(&sample_key(25)), len),
            (E_NOT_SUFFICIENT_BUFFER, vec![0xCCCC; len])
        );
    }
    // C: the key is checked after the length, so a NULL key with 40 units writes nothing
    assert_eq!(
        key_to(None, 40),
        (E_NOT_SUFFICIENT_BUFFER, vec![0xCCCC; 40])
    );
}

#[test]
fn key_none_writes_only_a_nul_when_the_buffer_has_room() {
    // C: `if (!pkey) { psz[0] = '\0'; return E_NOT_SUFFICIENT_BUFFER; }`, oracle `! 25 41`
    let (hr, buf) = key_to(None, 41);
    assert_eq!(hr, E_NOT_SUFFICIENT_BUFFER);
    assert_eq!(buf[0], 0);
    assert!(buf[1..].iter().all(|&u| u == 0xCCCC));
}

#[test]
fn key_string_fits_and_ends_with_a_nul() {
    // C: the pid follows a space, then a NUL; the oracle rows cover pids 25 and 4294967295
    let (hr, buf) = key_to(Some(&sample_key(25)), 50);
    assert_eq!(hr, S_OK);
    assert_eq!(
        buf[..42],
        wide("{12345678-9ABC-DEF0-1234-56789ABCDEF0} 25\0")[..]
    );
    assert!(buf[42..].iter().all(|&u| u == 0xCCCC));

    let (hr, buf) = key_to(Some(&sample_key(u32::MAX)), 60);
    assert_eq!(hr, S_OK);
    assert_eq!(
        buf[..50],
        wide("{12345678-9ABC-DEF0-1234-56789ABCDEF0} 4294967295\0")[..]
    );
    assert!(buf[50..].iter().all(|&u| u == 0xCCCC));
}

#[test]
fn key_string_overflow_writes_the_pid_backwards() {
    // C: the quirk loop writes from the second-to-last digit down, after a NUL at the pid start.
    // Each row is (length, pid, result, the units from index 39 to the end).
    let rows: [(usize, u32, HResult, &[u16]); 8] = [
        (41, 25, E_NOT_SUFFICIENT_BUFFER, &[0, 0x32]),
        (41, u32::MAX, E_NOT_SUFFICIENT_BUFFER, &[0, 0x39]),
        (42, u32::MAX, E_NOT_SUFFICIENT_BUFFER, &[0, 0x39, 0x32]),
        (
            43,
            u32::MAX,
            E_NOT_SUFFICIENT_BUFFER,
            &[0, 0x39, 0x32, 0x37],
        ),
        (
            45,
            u32::MAX,
            E_NOT_SUFFICIENT_BUFFER,
            &[0, 0x39, 0x32, 0x37, 0x36, 0x39],
        ),
        (41, 0, S_OK, &[0x30, 0]),
        (42, 25, S_OK, &[0x32, 0x35, 0]),
        (43, 25, S_OK, &[0x32, 0x35, 0, 0xCCCC]),
    ];
    for (len, pid, hr, tail) in rows {
        let (got, buf) = key_to(Some(&sample_key(pid)), len);
        assert_eq!(got, hr, "length {len} pid {pid}");
        assert_eq!(buf[38], u16::from(b' '), "length {len} pid {pid}");
        assert_eq!(buf[39..], tail[..], "length {len} pid {pid}");
    }
}

#[test]
fn guid_text_parses_in_either_case() {
    // C: PROPVAR_WCHARToGUID. The oracle row is `LPWSTR:{12345678-9abc-def0-1234-56789abcdefg}`.
    assert_eq!(guid_from(&lpw(SAMPLE_TEXT)), (S_OK, SAMPLE_FIELDS));
    assert_eq!(guid_from(&bstr(SAMPLE_TEXT)), (S_OK, SAMPLE_FIELDS));
    assert_eq!(
        guid_from(&lpw("{12345678-9ABC-DEF0-1234-56789ABCDEF0}")),
        (S_OK, SAMPLE_FIELDS)
    );
    // C source: Data1 is a DWORD, so a first nibble of 8 or more is plain
    assert_eq!(
        guid_from(&lpw("{F2345678-9abc-def0-1234-56789abcdef0}")),
        (S_OK, (0xF234_5678, 0x9abc, 0xdef0, SAMPLE.data4))
    );
    // C: the oracle row; the `fg` pair stores (BYTE)-1 in the last byte
    assert_eq!(
        guid_from(&lpw("{12345678-9abc-def0-1234-56789abcdefg}")),
        (
            S_OK,
            (
                0x1234_5678,
                0x9abc,
                0xdef0,
                [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xff]
            )
        )
    );
    // C source: lstrlenW stops at the NUL, so the text is 38 units
    assert_eq!(
        guid_from(&lpw("{12345678-9abc-def0-1234-56789abcdef0}\0x")),
        (S_OK, SAMPLE_FIELDS)
    );
    // C source: SysStringLen counts the NUL and the x, so the text is 40 units
    assert_eq!(
        guid_from(&bstr("{12345678-9abc-def0-1234-56789abcdef0}\0x")),
        (E_INVALIDARG, ZERO_FIELDS)
    );
}

#[test]
fn clsid_copies_the_guid_and_defaults_are_zero() {
    // C: `memcpy(guid, ppropvar->puuid, sizeof(*ppropvar->puuid))`
    assert_eq!(
        guid_from(&PropValue::Clsid(Some(SAMPLE))),
        (S_OK, SAMPLE_FIELDS)
    );

    // port: derived Default is the zero GUID and key, which the failure paths write
    assert_eq!(guid_fields(&Guid::default()), ZERO_FIELDS);
    let key = PropertyKey::default();
    assert_eq!((guid_fields(&key.fmtid), key.pid), (ZERO_FIELDS, 0));
}

#[test]
fn guid_text_with_a_bad_layout_is_invalid_and_zeroed() {
    // C: `len != 38 || str[0] != '{' || ...` then `memset` and E_INVALIDARG
    assert_eq!(guid_from(&lpw("")), (E_INVALIDARG, ZERO_FIELDS));
    assert_eq!(
        guid_from(&lpw(&SAMPLE_TEXT[..37])),
        (E_INVALIDARG, ZERO_FIELDS)
    );
    assert_eq!(
        guid_from(&lpw(&format!("{SAMPLE_TEXT}0"))),
        (E_INVALIDARG, ZERO_FIELDS)
    );
    for at in [0, 9, 14, 19, 24, 37] {
        assert_eq!(
            guid_from(&lpw_with(at, b'_')),
            (E_INVALIDARG, ZERO_FIELDS),
            "separator {at}"
        );
    }
}

#[test]
fn guid_text_with_a_bad_hex_digit_in_data1_to_data3_is_invalid() {
    // C: PROPVAR_HexToNum returns -1, the loop stops, and the GUID is memset to zero
    for at in (1..=8).chain(10..=13).chain(15..=18) {
        assert_eq!(
            guid_from(&lpw_with(at, b'g')),
            (E_INVALIDARG, ZERO_FIELDS),
            "index {at}"
        );
    }
}

#[test]
fn guid_text_with_a_bad_hex_digit_in_data4_stores_ff() {
    // C: `val = guid->Data4[i] = PROPVAR_HexToNum(p)` stores (BYTE)-1 and the loop goes on
    for (k, at) in [20, 22, 25, 27, 29, 31, 33, 35].into_iter().enumerate() {
        for pos in [at, at + 1] {
            let mut data4 = SAMPLE.data4;
            data4[k] = 0xFF;
            assert_eq!(
                guid_from(&lpw_with(pos, b'g')),
                (S_OK, (0x1234_5678, 0x9abc, 0xdef0, data4)),
                "index {pos}"
            );
        }
    }
}

#[test]
fn guid_from_other_shapes_is_not_implemented_and_untouched() {
    // C: `default: FIXME(...); return E_NOTIMPL;` leaves the GUID alone
    for value in [
        PropValue::Empty,
        PropValue::Null,
        PropValue::R8(1.5),
        PropValue::I4(1),
        PropValue::VectorUi1(vec![1]),
        ansi(SAMPLE_TEXT),
    ] {
        assert_eq!(guid_from(&value), (E_NOTIMPL, SENTINEL_FIELDS));
    }
}

#[test]
fn null_text_guids_are_invalid_and_zeroed() {
    // port: a NULL LPWSTR reads as empty, which fails the length check
    assert_eq!(
        guid_from(&PropValue::LpWStr(None)),
        (E_INVALIDARG, ZERO_FIELDS)
    );
    // C source: SysStringLen(NULL) is 0
    assert_eq!(
        guid_from(&PropValue::Bstr(None)),
        (E_INVALIDARG, ZERO_FIELDS)
    );
}

#[test]
fn int64_reads_each_integer_shape() {
    // C: PropVariantToInt64 over PROPVAR_ConvertNumber
    assert_eq!(i64_of(&PropValue::I2(-5)), (S_OK, -5));
    assert_eq!(i64_of(&PropValue::I1(-128)), (S_OK, -128));
    assert_eq!(
        i64_of(&PropValue::Ui8(9_223_372_036_854_775_807)),
        (S_OK, i64::MAX)
    );
    assert_eq!(i64_of(&PropValue::I8(i64::MIN)), (S_OK, i64::MIN));
    // C source: `ERROR_ARITHMETIC_OVERFLOW` for a negative LONGLONG from an unsigned source
    assert_eq!(
        i64_of(&PropValue::Ui8(u64::MAX)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // C: `default: FIXME; return E_NOTIMPL`
    assert_eq!(i64_of(&PropValue::Null), (E_NOTIMPL, 7));
    // C source: VT_BOOL is not in ConvertNumber
    assert_eq!(i64_of(&PropValue::Bool(-1)), (E_NOTIMPL, 7));
    // C: oracle row `to_i64 LPSTR:%20%20010`, which is octal
    assert_eq!(i64_of(&ansi("  010")), (S_OK, 8));
    // C: oracle row `to_i64 LPWSTR:4294967296`, which clamps to a 32-bit long
    assert_eq!(i64_of(&lpw("4294967296")), (S_OK, 2_147_483_647));
    // C source: wcstol on a 32-bit long
    assert_eq!(i64_of(&lpw("-2147483648")), (S_OK, -2_147_483_648));
    // C: oracle row `to_i64 LPWSTR:-2147483649`
    assert_eq!(i64_of(&lpw("-2147483649")), (S_OK, -2_147_483_648));
    // C: oracle row `to_i64 LPSTR:-9223372036854775808` (strtoll gives LLONG_MIN)
    assert_eq!(i64_of(&ansi("-9223372036854775808")), (S_OK, i64::MIN));
    // C: the row `to_i64 LPSTR:9223372036854775808` clamps to LLONG_MAX
    assert_eq!(i64_of(&ansi("-9223372036854775809")), (S_OK, i64::MIN));
    assert_eq!(i64_of(&ansi("9223372036854775807")), (S_OK, i64::MAX));
    assert_eq!(i64_of(&ansi("9223372036854775808")), (S_OK, i64::MAX));
    // C source: a BSTR goes through wcstol like LPWSTR
    assert_eq!(i64_of(&bstr("42")), (S_OK, 42));
    // port: a NULL text reads as empty. C calls strtoll(NULL), which crashes
    assert_eq!(i64_of(&PropValue::LpStr(None)), (DISP_E_TYPEMISMATCH, 7));
    assert_eq!(i64_of(&PropValue::LpWStr(None)), (DISP_E_TYPEMISMATCH, 7));
    assert_eq!(i64_of(&PropValue::Bstr(None)), (DISP_E_TYPEMISMATCH, 7));
}

#[test]
fn int16_and_int32_reject_values_outside_their_width() {
    // C: oracle rows `to_i16 I4:40000` and `to_i32 UI4:4294967295`
    assert_eq!(
        i16_of(&PropValue::I4(40_000)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // C source: the first value past `SHORT_MAX`
    assert_eq!(
        i16_of(&PropValue::I4(32_768)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // port: sanitizer-flagged, since C shifts -1 by 15 bits, so the port's own result stands
    assert_eq!(
        i16_of(&PropValue::I4(-40_000)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    assert_eq!(i16_of(&PropValue::I4(-32_768)), (S_OK, -32_768));
    assert_eq!(i16_of(&PropValue::I2(-5)), (S_OK, -5));
    assert_eq!(i16_of(&PropValue::Ui1(200)), (S_OK, 200));
    assert_eq!(i16_of(&PropValue::I4(32_767)), (S_OK, 32_767));
    assert_eq!(
        i32_of(&PropValue::Ui4(u32::MAX)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // port: sanitizer-flagged for the same shift as above
    assert_eq!(i32_of(&PropValue::I4(5)), (S_OK, 5));
    assert_eq!(i32_of(&PropValue::I4(i32::MIN)), (S_OK, i32::MIN));
    assert_eq!(i32_of(&PropValue::R8(-0.5)), (S_OK, 0));
    // C: `default` for VT_CLSID and VT_VECTOR|VT_UI1
    assert_eq!(i32_of(&PropValue::Clsid(Some(SAMPLE))), (E_NOTIMPL, 7));
    assert_eq!(i32_of(&PropValue::VectorUi1(vec![1])), (E_NOTIMPL, 7));
}

#[test]
fn unsigned_conversions_reject_negatives_and_wide_values() {
    // C: oracle rows `to_u16 I1:-1`, `to_u16 UI2:65535`, `to_u32 EMPTY`, `to_u64 I8:-5`
    assert_eq!(
        u16_of(&PropValue::I1(-1)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    assert_eq!(u16_of(&PropValue::Ui2(65_535)), (S_OK, 65_535));
    // C source: a signed source that fits the unsigned destination
    assert_eq!(u16_of(&PropValue::I4(65_535)), (S_OK, 65_535));
    assert_eq!(
        u16_of(&PropValue::I4(-1)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    assert_eq!(
        u16_of(&PropValue::I4(65_536)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // C source: a double truncates toward zero, and then `(LONGLONG)` makes the result
    assert_eq!(u16_of(&PropValue::R8(65_535.9)), (S_OK, 65_535));
    assert_eq!(u16_of(&PropValue::R8(-0.5)), (S_OK, 0));
    assert_eq!(u32_of(&PropValue::Empty), (S_OK, 0));
    assert_eq!(u32_of(&PropValue::Ui4(u32::MAX)), (S_OK, u32::MAX));
    assert_eq!(
        u32_of(&PropValue::I8(4_294_967_296)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // C: a negative text value is an overflow to an unsigned type
    assert_eq!(u32_of(&ansi("-1")), (ERROR_ARITHMETIC_OVERFLOW_HR, 7));
    assert_eq!(
        u32_of(&ansi("4294967296")),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // C: oracle row `to_u32 LPSTR:%20%2042%20`, which is "  42 "
    assert_eq!(u32_of(&ansi("  42 ")), (S_OK, 42));
    assert_eq!(
        u32_of(&PropValue::Ui8(1_u64 << 63)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    // C: oracle row `to_u64 I8:-5`, and the row `to_u64 UI8:18446744073709551615`
    assert_eq!(
        u64_of(&PropValue::I8(-5)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    assert_eq!(u64_of(&PropValue::Ui8(u64::MAX)), (S_OK, u64::MAX));
    assert_eq!(
        u64_of(&PropValue::I4(-1)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
    assert_eq!(u64_of(&PropValue::Ui8(0)), (S_OK, 0));
    // C source: the 64-bit destination has no range check
    assert_eq!(u64_of(&PropValue::Ui8(1_u64 << 63)), (S_OK, 1_u64 << 63));
    // port: out of range, so the port's `i64::MIN` is negative and overflows
    assert_eq!(
        u32_of(&PropValue::R8(1e300)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, 7)
    );
}

#[test]
fn ansi_text_follows_strtoll_base_zero() {
    // C: strtoll with base 0. The rows are C rows: the `to_i64 LPSTR:` rows of edge batch 5.
    let rows: [(&str, i64); 18] = [
        ("0y", 0),
        ("5", 5),
        ("0", 0),
        ("-0", 0),
        ("0x1f", 31),
        ("0X1F", 31),
        ("0x1G", 1),
        ("12a", 12),
        ("-0x10", -16),
        ("  +7", 7),
        ("\u{b}7", 7),
        ("\u{c}7", 7),
        ("\t\r7", 7),
        ("12\u{0}4", 12),
        ("0x", 0),
        ("0xg", 0),
        ("09", 0),
        ("017", 15),
    ];
    for (text, want) in rows {
        assert_eq!(i64_of(&ansi(text)), (S_OK, want), "{text:?}");
    }
    // C: `if (pv->pszVal == end) return DISP_E_TYPEMISMATCH;`, with no digits
    for text in ["++5", "-", " ", "+", "--5", "", "\u{0}"] {
        assert_eq!(i64_of(&ansi(text)), (DISP_E_TYPEMISMATCH, 7), "{text:?}");
    }
}

#[test]
fn wide_text_follows_wcstol_base_zero() {
    // C: wcstol with base 0. The rows are C rows: the `to_i64 LPWSTR:` rows of edge batch 5
    // and the oracle; the rest are read from the C source.
    assert_eq!(i64_of(&lpw("  -0x10")), (S_OK, -16));
    assert_eq!(i64_of(&lpw("+2147483647")), (S_OK, 2_147_483_647));
    assert_eq!(i64_of(&lpw("0x")), (S_OK, 0));
    assert_eq!(i64_of(&lpw("\t\r7")), (S_OK, 7));
    assert_eq!(i64_of(&lpw(" ")), (DISP_E_TYPEMISMATCH, 7));
    assert_eq!(i64_of(&lpw("-")), (DISP_E_TYPEMISMATCH, 7));
}

#[test]
fn text_numbers_saturate_at_the_width_of_the_type() {
    // C: strtoll gives LLONG_MAX or LLONG_MIN, and wcstol gives LONG_MAX or LONG_MIN
    assert_eq!(i64_of(&ansi("0x8000000000000000")), (S_OK, i64::MAX));
    assert_eq!(i64_of(&ansi("-0x8000000000000000")), (S_OK, i64::MIN));
    assert_eq!(i64_of(&ansi("0xFFFFFFFFFFFFFFFFFF")), (S_OK, i64::MAX));
    assert_eq!(i64_of(&ansi("-0x8000000000000001")), (S_OK, i64::MIN));
    assert_eq!(i64_of(&ansi("99999999999999999999")), (S_OK, i64::MAX));
    // C: oracle rows `to_i64 LPWSTR:2147483648` and `to_i64 LPWSTR:-0x80000001`
    assert_eq!(i64_of(&lpw("2147483648")), (S_OK, 2_147_483_647));
    assert_eq!(i64_of(&lpw("0x80000000")), (S_OK, 2_147_483_647));
    assert_eq!(i64_of(&lpw("-0x80000001")), (S_OK, -2_147_483_648));
    // C source: the 32-bit long clamps a decimal that is too long
    assert_eq!(i64_of(&lpw("99999999999999999999")), (S_OK, 2_147_483_647));
}

#[test]
fn double_truncates_toward_zero_and_saturates_out_of_range() {
    // C: `*res = pv->dblVal` into a LONGLONG. Oracle rows `to_i64 R8:9.2e18`, `R8:-1.5`, `R8:-0.5`
    assert_eq!(
        i64_of(&PropValue::R8(9.2e18)),
        (S_OK, 9_200_000_000_000_000_000)
    );
    // C source: -2^63 is in range
    assert_eq!(
        i64_of(&PropValue::R8(-9_223_372_036_854_775_808.0)),
        (S_OK, i64::MIN)
    );
    assert_eq!(i64_of(&PropValue::R8(-1.5)), (S_OK, -1));
    assert_eq!(i64_of(&PropValue::R8(-0.5)), (S_OK, 0));
    // port: NaN and values outside -2^63..2^63 are undefined in C, and convert to i64::MIN
    assert_eq!(i64_of(&PropValue::R8(f64::NAN)), (S_OK, i64::MIN));
    assert_eq!(i64_of(&PropValue::R8(1e300)), (S_OK, i64::MIN));
    assert_eq!(
        i64_of(&PropValue::R8(9_223_372_036_854_775_808.0)),
        (S_OK, i64::MIN)
    );
}

#[test]
fn boolean_reads_literals_numbers_and_nulls() {
    // C: VT_BOOL compares with VARIANT_TRUE
    assert_eq!(bool_of(&PropValue::Bool(-1)), (S_OK, true));
    assert_eq!(bool_of(&PropValue::Bool(1)), (S_OK, false));
    // C source: VT_BOOL of 0 is FALSE
    assert_eq!(bool_of(&PropValue::Bool(0)), (S_OK, false));
    // C: oracle row `bool LPWSTR:TRUE` and the lstrcmpiW / lstrcmpW checks
    assert_eq!(bool_of(&lpw("TrUe")), (S_OK, true));
    assert_eq!(bool_of(&lpw("#TRUE#")), (S_OK, true));
    assert_eq!(bool_of(&lpw("FALSE")), (S_OK, false));
    assert_eq!(bool_of(&lpw("#FALSE#")), (S_OK, false));
    // C: oracle row `bool LPWSTR:%23false%23`: the `#...#` forms are case-sensitive
    assert_eq!(bool_of(&lpw("#false#")), (DISP_E_TYPEMISMATCH, false));
    // C: oracle row `bool LPSTR:FaLsE`
    assert_eq!(bool_of(&ansi("FaLsE")), (S_OK, false));
    assert_eq!(bool_of(&ansi("TRUE")), (S_OK, true));
    assert_eq!(bool_of(&ansi("#TRUE#")), (S_OK, true));
    // C source: lstrcmpA stops at the NUL
    assert_eq!(bool_of(&ansi("false\0x")), (S_OK, false));
    // C: oracle row `bool LPSTR:True%20`
    assert_eq!(bool_of(&ansi("truex")), (DISP_E_TYPEMISMATCH, false));
    assert_eq!(bool_of(&ansi("True ")), (DISP_E_TYPEMISMATCH, false));
    assert_eq!(bool_of(&bstr("tRuE")), (S_OK, true));
    assert_eq!(bool_of(&bstr("false")), (S_OK, false));
    // C source: a BSTR with an embedded NUL compares as the text before it
    assert_eq!(bool_of(&bstr("true\0x")), (S_OK, true));
    // C: oracle row `bool BSTR:!`, and the NULL checks in the literal branch
    assert_eq!(
        bool_of(&PropValue::Bstr(None)),
        (DISP_E_TYPEMISMATCH, false)
    );
    assert_eq!(
        bool_of(&PropValue::LpWStr(None)),
        (DISP_E_TYPEMISMATCH, false)
    );
    // C: oracle row `bool LPSTR:!`
    assert_eq!(
        bool_of(&PropValue::LpStr(None)),
        (DISP_E_TYPEMISMATCH, false)
    );
    // C source: the literals fail, and then strtoll finds no digits in "\0"
    assert_eq!(bool_of(&ansi("\0")), (DISP_E_TYPEMISMATCH, false));
    // C: a text that is not a literal is read as a number
    assert_eq!(bool_of(&lpw("5")), (S_OK, true));
    // C: oracle rows `bool R4:0.5` and `bool UI8:18446744073709551615`
    assert_eq!(bool_of(&PropValue::R8(3.5)), (S_OK, true));
    assert_eq!(bool_of(&PropValue::R8(0.5)), (S_OK, false));
    assert_eq!(bool_of(&PropValue::R4(0.5)), (E_NOTIMPL, false));
    assert_eq!(bool_of(&PropValue::I4(1)), (S_OK, true));
    assert_eq!(bool_of(&PropValue::I4(0)), (S_OK, false));
    // C: oracle row `bool UI1:0`
    assert_eq!(bool_of(&PropValue::Ui1(0)), (S_OK, false));
    // C: oracle row `bool EMPTY`
    assert_eq!(bool_of(&PropValue::Empty), (S_OK, false));
    // port: C returns with *ret uninitialised; the port writes false
    assert_eq!(bool_of(&PropValue::Null), (E_NOTIMPL, false));
    // C: oracle row `bool CLSID:...`
    assert_eq!(bool_of(&PropValue::Clsid(Some(SAMPLE))), (E_NOTIMPL, false));
    // C: oracle row `bool UI8:18446744073709551615`, where `*ret = !!res` runs on error too
    assert_eq!(
        bool_of(&PropValue::Ui8(u64::MAX)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, true)
    );
}

#[test]
fn double_reads_reals_and_integers_and_keeps_the_output_on_error() {
    // C: PropVariantToDouble. Oracle rows `double R8:1.5`, `double R4:2.5`, `double R4:-0.5`
    assert_eq!(double_of(&PropValue::R8(1.5)), (S_OK, 1.5_f64.to_bits()));
    assert_eq!(double_of(&PropValue::R4(2.5)), (S_OK, 2.5_f64.to_bits()));
    assert_eq!(
        double_of(&PropValue::R4(-0.5)),
        (S_OK, (-0.5_f64).to_bits())
    );
    assert_eq!(
        double_of(&PropValue::R8(-0.25)),
        (S_OK, (-0.25_f64).to_bits())
    );
    assert_eq!(double_of(&PropValue::I8(-7)), (S_OK, (-7.0_f64).to_bits()));
    assert_eq!(
        double_of(&PropValue::Ui4(u32::MAX)),
        (S_OK, 4_294_967_295.0_f64.to_bits())
    );
    assert_eq!(double_of(&PropValue::I2(-3)), (S_OK, (-3.0_f64).to_bits()));
    // C: oracle row `double EMPTY`
    assert_eq!(double_of(&PropValue::Empty), (S_OK, 0.0_f64.to_bits()));
    // C: the output is written only on success, as `if (SUCCEEDED(hr)) *ret = ...`
    let untouched = 7.5_f64.to_bits();
    assert_eq!(double_of(&ansi("zz")), (DISP_E_TYPEMISMATCH, untouched));
    // C: oracle row `double BOOL:1`
    assert_eq!(double_of(&PropValue::Bool(1)), (E_NOTIMPL, untouched));
    // C: oracle row `double UI8:18446744073709551615`
    assert_eq!(
        double_of(&PropValue::Ui8(u64::MAX)),
        (ERROR_ARITHMETIC_OVERFLOW_HR, untouched)
    );
    assert_eq!(double_of(&PropValue::Null), (E_NOTIMPL, untouched));
}

#[test]
fn buffer_copies_vector_bytes_up_to_the_vector_length() {
    // C: `VT_VECTOR|VT_UI1`. The oracle rows are `buffer VEC_UI1:0102 2`, `01ff10 4` and `01ff10 0`
    assert_eq!(
        buffer_of(&PropValue::VectorUi1(vec![1, 2]), 2),
        (S_OK, vec![1, 2])
    );
    assert_eq!(
        buffer_of(&PropValue::VectorUi1(vec![1, 2]), 1),
        (S_OK, vec![1])
    );
    // C: `if (cb > propvarIn->caub.cElems) return E_FAIL;` with the output untouched
    assert_eq!(
        buffer_of(&PropValue::VectorUi1(vec![1, 2]), 3),
        (E_FAIL, vec![0xCC; 3])
    );
    assert_eq!(
        buffer_of(&PropValue::VectorUi1(vec![0xFF]), 1),
        (S_OK, vec![0xFF])
    );
    assert_eq!(buffer_of(&PropValue::VectorUi1(vec![]), 0), (S_OK, vec![]));
    assert_eq!(
        buffer_of(&PropValue::VectorUi1(vec![1, 0xFF, 0x10]), 4),
        (E_FAIL, vec![0xCC; 4])
    );
    // C: `default: WARN; hr = E_INVALIDARG`
    assert_eq!(buffer_of(&PropValue::Empty, 0), (E_INVALIDARG, vec![]));
    assert_eq!(buffer_of(&PropValue::Null, 1), (E_INVALIDARG, vec![0xCC]));
    assert_eq!(buffer_of(&PropValue::I4(1), 1), (E_INVALIDARG, vec![0xCC]));
    // C source: LPSTR is not a vector
    assert_eq!(buffer_of(&ansi("ab"), 2), (E_INVALIDARG, vec![0xCC; 2]));
}

#[test]
fn uint32_with_default_uses_the_default_on_error() {
    // C: PropVariantToUInt32WithDefault returns ulDefault unless the conversion succeeds
    assert_eq!(prop_variant_to_uint32_with_default(&PropValue::Null, 9), 9);
    assert_eq!(
        prop_variant_to_uint32_with_default(&PropValue::Ui4(u32::MAX), 9),
        u32::MAX
    );
    // C source: a hex text value is read by strtoll
    assert_eq!(prop_variant_to_uint32_with_default(&ansi("0x2a"), 9), 42);
    // C: oracle row `u32def I4:-1 99`
    assert_eq!(
        prop_variant_to_uint32_with_default(&PropValue::I4(-1), 99),
        99
    );
    assert_eq!(
        prop_variant_to_uint32_with_default(&PropValue::I8(-1), 9),
        9
    );
    // C: oracle row `u32def UI1:200 9`, and the value of a small unsigned
    assert_eq!(
        prop_variant_to_uint32_with_default(&PropValue::Ui1(7), 9),
        7
    );
    // C: oracle row `u32def UI8:9223372036854775808 9`
    assert_eq!(
        prop_variant_to_uint32_with_default(&PropValue::Ui8(1_u64 << 63), 9),
        9
    );
}

#[test]
fn string_with_default_returns_the_text_or_the_default() {
    // C: PropVariantToStringWithDefault. Oracle rows `str_default LPWSTR:%41%00%42 !` (gives
    // "A"), `BSTR:! 61` (gives ""), `LPWSTR:%20 61`, `NULL !`
    assert_eq!(str_default_of(&lpw("A\0B"), None), Some(wide("A")));
    assert_eq!(str_default_of(&lpw(" "), Some("61")), Some(wide(" ")));
    assert_eq!(
        str_default_of(&PropValue::LpWStr(None), Some("61")),
        Some(wide("61"))
    );
    assert_eq!(
        str_default_of(&PropValue::Bstr(None), Some("61")),
        Some(Vec::new())
    );
    assert_eq!(str_default_of(&bstr("ab x"), None), Some(wide("ab x")));
    // C source: a BSTR is returned as a pointer, and the caller reads it to the NUL
    assert_eq!(str_default_of(&bstr("ab\0x"), None), Some(wide("ab")));
    assert_eq!(str_default_of(&PropValue::Null, None), None);
    assert_eq!(
        str_default_of(&PropValue::Null, Some("61")),
        Some(wide("61"))
    );
    assert_eq!(
        str_default_of(&PropValue::I4(1), Some("61")),
        Some(wide("61"))
    );
    assert_eq!(str_default_of(&PropValue::I4(1), None), None);
    // port: the default is read to its NUL, as the other results are
    assert_eq!(
        str_default_of(&PropValue::Null, Some("a\0b")),
        Some(wide("a"))
    );
}
