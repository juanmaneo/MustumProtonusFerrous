use super::*;

#[test]
fn type_equality_uses_descriptor_identity() {
    let descriptor = TypeDescriptor { identity: 7 };
    let same_payload_but_distinct_descriptor = TypeDescriptor { identity: 7 };
    let left = PlatformType {
        descriptor: Some(&descriptor),
    };
    let same_descriptor = PlatformType {
        descriptor: Some(&descriptor),
    };
    let distinct_descriptor = PlatformType {
        descriptor: Some(&same_payload_but_distinct_descriptor),
    };
    let no_descriptor_a = PlatformType { descriptor: None };
    let no_descriptor_b = PlatformType { descriptor: None };

    assert!(platform_type_equals_object(None, None));
    assert!(!platform_type_equals_object(Some(&left), None));
    assert!(!platform_type_equals_object(None, Some(&left)));
    assert!(platform_type_equals_object(Some(&left), Some(&left)));
    assert!(platform_type_equals_object(
        Some(&left),
        Some(&same_descriptor)
    ));
    assert!(!platform_type_equals_object(
        Some(&left),
        Some(&distinct_descriptor)
    ));
    assert!(platform_type_equals_object(
        Some(&no_descriptor_a),
        Some(&no_descriptor_b)
    ));
    assert!(!platform_type_equals_object(
        Some(&left),
        Some(&no_descriptor_a)
    ));
    assert!(!platform_type_equals_object(
        Some(&no_descriptor_a),
        Some(&left)
    ));
}

#[test]
fn guid_copy_and_utf16_preserve_edge_values() {
    let source = [
        Guid {
            data1: u32::MAX,
            data2: u16::MAX,
            data3: 0,
            data4: [u8::MAX, 0, 1, 2, 3, 4, 5, 6],
        },
        Guid {
            data1: 0,
            data2: 1,
            data3: 2,
            data4: [3, 4, 5, 6, 7, 8, 9, 10],
        },
    ];

    assert_eq!(get_iids_fn(&[]), Vec::<Guid>::new());
    assert_eq!(get_iids_fn(&source), source);
    assert_eq!(char16_to_string(0xd800), vec![0xd800]);
    assert_eq!(char16_to_string(0xdc00), vec![0xdc00]);
    assert_eq!(char16_to_string(0xffff), vec![0xffff]);
}

#[test]
fn typecode_table_and_decimal_fallback_match_wine() {
    let named = [
        (3, "Boolean"),
        (4, "char16"),
        (6, "uint8"),
        (7, "int16"),
        (8, "uint16"),
        (9, "int32"),
        (10, "uint32"),
        (11, "int64"),
        (12, "uint64"),
        (13, "float32"),
        (14, "double"),
        (16, "DateTime"),
        (18, "String"),
        (20, "Point"),
        (21, "Size"),
        (22, "Rect"),
        (23, "Guid"),
    ];

    for (code, name) in named {
        assert_eq!(typecode_name(code), name);
    }
    assert_eq!(typecode_name(0), "0");
    assert_eq!(typecode_name(17), "17");
    assert_eq!(typecode_name(19), "19");
    assert_eq!(typecode_name(24), "24");
    assert_eq!(typecode_name(-1), "-1");
    assert_eq!(TypeCode::Boolean as i32, 3);
    assert_eq!(TypeCode::Guid as i32, 23);
}

#[test]
fn float_special_values_and_exponent_boundaries() {
    assert_eq!(float64_to_string(f64::NAN), "nan");
    assert_eq!(float64_to_string(f64::INFINITY), "inf");
    assert_eq!(float64_to_string(f64::NEG_INFINITY), "-inf");
    assert_eq!(float64_to_string(0.0), "0");
    assert_eq!(float64_to_string(0.0001), "0.0001");
    assert_eq!(float64_to_string(0.0000999999), "9.99999e-05");
    assert_eq!(float64_to_string(999_999.0), "999999");
    assert_eq!(float64_to_string(f64::MIN_POSITIVE), "2.22507e-308");
    assert_eq!(float64_to_string(f64::MAX), "1.79769e+308");
    assert_eq!(float32_to_string(f32::INFINITY), "inf");
}
