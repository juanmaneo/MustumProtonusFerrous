use super::*;

#[test]
fn oracle_rows_from_wine_formatters() {
    let guid_a = Guid {
        data1: 0x1234_5678,
        data2: 0x9abc,
        data3: 0xdef0,
        data4: [0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67],
    };
    let guid_zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    assert_eq!(
        guid_to_string(&guid_a),
        "{12345678-9abc-def0-89ab-cdef01234567}"
    );
    assert_eq!(
        guid_to_string(&guid_zero),
        "{00000000-0000-0000-0000-000000000000}"
    );
    assert_eq!(boolean_to_string(0), "false");
    assert_eq!(boolean_to_string(255), "true");
    assert_eq!(char16_to_string(b'A' as u16), vec![0x0041]);
    assert_eq!(char16_to_string(0), vec![0x0000]);
    assert_eq!(char16_to_string(b'Z' as u16), vec![0x005a]);
    assert_eq!(float32_to_string(1.234_567_9_f32), "1.23457");
    assert_eq!(float32_to_string(1.0e-5_f32), "1e-05");
    assert_eq!(float64_to_string(1.234_567_89), "1.23457");
    assert_eq!(float64_to_string(0.000_012_345), "1.2345e-05");
    assert_eq!(float64_to_string(1_000_000.0), "1e+06");
    assert_eq!(float64_to_string(999_999.9), "1e+06");
    assert_eq!(float64_to_string(-0.0), "-0");
    assert_eq!(int8_to_string(i8::MIN), "-128");
    assert_eq!(int16_to_string(i16::MIN), "-32768");
    assert_eq!(int32_to_string(i32::MIN), "-2147483648");
    assert_eq!(int64_to_string(i64::MIN), "-9223372036854775808");
    assert_eq!(uint8_to_string(u8::MAX), "255");
    assert_eq!(uint16_to_string(u16::MAX), "65535");
    assert_eq!(uint32_to_string(u32::MAX), "4294967295");
    assert_eq!(uint64_to_string(u64::MAX), "18446744073709551615");
    assert_eq!(typecode_name(3), "Boolean");
    assert_eq!(typecode_name(19), "19");
    assert_eq!(typecode_name(99), "99");
}
