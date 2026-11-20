//! Oracle rows for `dlls/cfgmgr32`. Each expected result was recorded by running the
//! verbatim Wine C (`devobject.c` and `cfgmgr32.c`, built with ASan and UBSan, no
//! sanitizer report) on the input written in the test.

use crate::configret::{cr_to_win32_error, map_error, map_error_node};
use crate::devprop::type_validate;
use crate::filter::{filters_match, filters_validate};
use crate::{CompKey, Filter, Property};

const GUID_A: [u8; 16] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
];
const GUID_B: [u8; 16] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

fn prop<'a>(
    fmtid: [u8; 16],
    pid: u32,
    store: u32,
    locale: Option<&'a str>,
    ty: u32,
    buffer: Option<&'a [u8]>,
) -> Property<'a> {
    Property {
        comp_key: CompKey {
            fmtid,
            pid,
            store,
            locale,
        },
        ty,
        buffer,
    }
}

fn flt<'a>(operator: u32, property: Property<'a>) -> Filter<'a> {
    Filter { operator, property }
}

#[test]
fn oracle_typevalidate_string_nonempty() {
    // Oracle: devprop_type_validate(0x12, 4) = 1
    assert!(type_validate(0x12, 4));
}

#[test]
fn oracle_typevalidate_security_descriptor_empty() {
    // Oracle: devprop_type_validate(0x13, 0) = 0
    assert!(!type_validate(0x13, 0));
}

#[test]
fn oracle_typevalidate_int32_list() {
    // Oracle: devprop_type_validate(0x2006, 4) = 0
    assert!(!type_validate(0x2006, 4));
}

#[test]
fn oracle_filters_equals_string() {
    // Oracle: devprop_filters_validate = 1
    assert!(filters_validate(&[flt(
        0x2,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
}

#[test]
fn oracle_filters_and_open_unclosed() {
    // Oracle: devprop_filters_validate = 0
    assert!(!filters_validate(&[flt(
        0x100000,
        prop(GUID_A, 0, 0, None, 0x0, None)
    )]));
}

#[test]
fn oracle_filters_empty_buffer_rejected() {
    // Oracle: devprop_filters_validate = 0
    assert!(!filters_validate(&[flt(
        0x2,
        prop(GUID_A, 1, 0, None, 0x12, Some(&[]))
    )]));
}

#[test]
fn oracle_match_equals_string() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x2,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x2,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_equals_ignore_case() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x20002,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x61, 0x00, 0x62, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x20002,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_greater_than_acts_as_equals() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 0
    assert!(filters_validate(&[flt(
        0x3,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x41, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(!filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x42, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x3,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x41, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_greater_equals_acts_as_less_than() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x5,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x42, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x41, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x5,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x42, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_less_equals_acts_as_equals() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 0
    assert!(filters_validate(&[flt(
        0x6,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x42, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(!filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x41, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x6,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x42, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_guid_not_less_than_is_true() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x10004,
        prop(
            GUID_A,
            1,
            0,
            None,
            0xd,
            Some(&[
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
                0x1e, 0x1f
            ])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0xd,
            Some(&[
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f
            ])
        )],
        &[flt(
            0x10004,
            prop(
                GUID_A,
                1,
                0,
                None,
                0xd,
                Some(&[
                    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                    0x1d, 0x1e, 0x1f
                ])
            )
        )]
    ));
}

#[test]
fn oracle_match_not_equals_missing_key() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x10002,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            2,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x10002,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_not_equals_other_fmtid() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x10002,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_B,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x10002,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_or_group() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[
        flt(0x300000, prop(GUID_A, 0, 0, None, 0x0, None)),
        flt(
            0x2,
            prop(
                GUID_A,
                2,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        ),
        flt(
            0x2,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        ),
        flt(0x400000, prop(GUID_A, 0, 0, None, 0x0, None))
    ]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[
            flt(0x300000, prop(GUID_A, 0, 0, None, 0x0, None)),
            flt(
                0x2,
                prop(
                    GUID_A,
                    2,
                    0,
                    None,
                    0x12,
                    Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
                )
            ),
            flt(
                0x2,
                prop(
                    GUID_A,
                    1,
                    0,
                    None,
                    0x12,
                    Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
                )
            ),
            flt(0x400000, prop(GUID_A, 0, 0, None, 0x0, None))
        ]
    ));
}

#[test]
fn oracle_match_not_group() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 0
    assert!(filters_validate(&[
        flt(0x500000, prop(GUID_A, 0, 0, None, 0x0, None)),
        flt(
            0x2,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        ),
        flt(0x600000, prop(GUID_A, 0, 0, None, 0x0, None))
    ]));
    assert!(!filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[
            flt(0x500000, prop(GUID_A, 0, 0, None, 0x0, None)),
            flt(
                0x2,
                prop(
                    GUID_A,
                    1,
                    0,
                    None,
                    0x12,
                    Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
                )
            ),
            flt(0x600000, prop(GUID_A, 0, 0, None, 0x0, None))
        ]
    ));
}

#[test]
fn oracle_match_nested_and_or() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[
        flt(0x100000, prop(GUID_A, 0, 0, None, 0x0, None)),
        flt(0x300000, prop(GUID_A, 0, 0, None, 0x0, None)),
        flt(
            0x2,
            prop(
                GUID_A,
                2,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        ),
        flt(
            0x2,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        ),
        flt(0x400000, prop(GUID_A, 0, 0, None, 0x0, None)),
        flt(0x200000, prop(GUID_A, 0, 0, None, 0x0, None))
    ]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[
            flt(0x100000, prop(GUID_A, 0, 0, None, 0x0, None)),
            flt(0x300000, prop(GUID_A, 0, 0, None, 0x0, None)),
            flt(
                0x2,
                prop(
                    GUID_A,
                    2,
                    0,
                    None,
                    0x12,
                    Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
                )
            ),
            flt(
                0x2,
                prop(
                    GUID_A,
                    1,
                    0,
                    None,
                    0x12,
                    Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
                )
            ),
            flt(0x400000, prop(GUID_A, 0, 0, None, 0x0, None)),
            flt(0x200000, prop(GUID_A, 0, 0, None, 0x0, None))
        ]
    ));
}

#[test]
fn oracle_match_locale_ignore_case() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 1
    assert!(filters_validate(&[flt(
        0x2,
        prop(
            GUID_A,
            1,
            0,
            Some("EN-us"),
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            Some("en-US"),
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x2,
            prop(
                GUID_A,
                1,
                0,
                Some("EN-us"),
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_unsupported_not_bitwise_and() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 0
    assert!(filters_validate(&[flt(
        0x10007,
        prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )
    )]));
    assert!(!filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x10007,
            prop(
                GUID_A,
                1,
                0,
                None,
                0x12,
                Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
            )
        )]
    ));
}

#[test]
fn oracle_match_type_mismatch() {
    // Oracle: devprop_filters_validate = 1, devprop_filter_matches_properties = 0
    assert!(filters_validate(&[flt(
        0x2,
        prop(GUID_A, 1, 0, None, 0x7, Some(&[0x01, 0x00, 0x00, 0x00]))
    )]));
    assert!(!filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x6,
            Some(&[0x01, 0x00, 0x00, 0x00])
        )],
        &[flt(
            0x2,
            prop(GUID_A, 1, 0, None, 0x7, Some(&[0x01, 0x00, 0x00, 0x00]))
        )]
    ));
}

#[test]
fn oracle_match_empty_filter_list() {
    // Oracle: devprop_filter_matches_properties with no filters = 1 (the empty list is not validated)
    assert!(filters_match(
        &[prop(
            GUID_A,
            1,
            0,
            None,
            0x12,
            Some(&[0x41, 0x00, 0x42, 0x00, 0x00, 0x00])
        )],
        &[]
    ));
}

#[test]
fn oracle_cm_map_no_such_devnode() {
    // Oracle: cm_harness map 13 0 = 1168
    assert_eq!(cr_to_win32_error(0xd, 0), 1168);
}

#[test]
fn oracle_cm_map_unlisted_uses_default() {
    // Oracle: cm_harness map 31 5 = 5
    assert_eq!(cr_to_win32_error(0x1f, 5), 5);
}

#[test]
fn oracle_cm_lstatus_unlisted_is_cr_failure() {
    // Oracle: cm_harness lstatus 13 = 19
    assert_eq!(map_error(0xd), 19);
}

#[test]
fn oracle_cm_node_file_not_found() {
    // Oracle: cm_harness node 2 = 13
    assert_eq!(map_error_node(0x2), 13);
}

#[test]
fn constants_match_wine_headers() {
    // Oracle: each constant as defined by the Wine headers
    assert_eq!(crate::configret::CR_SUCCESS, 0x0);
    assert_eq!(crate::configret::CR_OUT_OF_MEMORY, 0x2);
    assert_eq!(crate::configret::CR_INVALID_POINTER, 0x3);
    assert_eq!(crate::configret::CR_INVALID_FLAG, 0x4);
    assert_eq!(crate::configret::CR_INVALID_DEVNODE, 0x5);
    assert_eq!(crate::configret::CR_NO_SUCH_DEVNODE, 0xd);
    assert_eq!(crate::configret::CR_ALREADY_SUCH_DEVNODE, 0x10);
    assert_eq!(crate::configret::CR_FAILURE, 0x13);
    assert_eq!(crate::configret::CR_BUFFER_SMALL, 0x1a);
    assert_eq!(crate::configret::CR_INVALID_DATA, 0x1f);
    assert_eq!(crate::configret::CR_NO_REGISTRY_HANDLE, 0x1c);
    assert_eq!(crate::configret::CR_REGISTRY_ERROR, 0x1d);
    assert_eq!(crate::configret::CR_INVALID_DEVICE_ID, 0x1e);
    assert_eq!(crate::configret::CR_NO_SUCH_REGISTRY_KEY, 0x2e);
    assert_eq!(crate::configret::CR_INVALID_MACHINENAME, 0x2f);
    assert_eq!(crate::configret::CR_REMOTE_COMM_FAILURE, 0x30);
    assert_eq!(crate::configret::CR_MACHINE_UNAVAILABLE, 0x31);
    assert_eq!(crate::configret::CR_NO_CM_SERVICES, 0x32);
    assert_eq!(crate::configret::CR_ACCESS_DENIED, 0x33);
    assert_eq!(crate::configret::CR_CALL_NOT_IMPLEMENTED, 0x34);
    assert_eq!(crate::configret::CR_INVALID_PROPERTY, 0x35);
    assert_eq!(crate::configret::CR_NO_SUCH_VALUE, 0x25);
    assert_eq!(crate::configret::CR_NO_SUCH_DEVICE_INTERFACE, 0x37);
    assert_eq!(crate::configret::CR_INVALID_REFERENCE_STRING, 0x38);
    assert_eq!(crate::configret::ERROR_SUCCESS, 0x0);
    assert_eq!(crate::configret::ERROR_FILE_NOT_FOUND, 0x2);
    assert_eq!(crate::configret::ERROR_ACCESS_DENIED, 0x5);
    assert_eq!(crate::configret::ERROR_INVALID_HANDLE, 0x6);
    assert_eq!(crate::configret::ERROR_NOT_ENOUGH_MEMORY, 0x8);
    assert_eq!(crate::configret::ERROR_INVALID_DATA, 0xd);
    assert_eq!(crate::configret::ERROR_INVALID_PARAMETER, 0x57);
    assert_eq!(crate::configret::ERROR_CALL_NOT_IMPLEMENTED, 0x78);
    assert_eq!(crate::configret::ERROR_INSUFFICIENT_BUFFER, 0x7a);
    assert_eq!(crate::configret::ERROR_ALREADY_EXISTS, 0xb7);
    assert_eq!(crate::configret::ERROR_MORE_DATA, 0xea);
    assert_eq!(crate::configret::ERROR_NO_MORE_ITEMS, 0x103);
    assert_eq!(crate::configret::ERROR_INVALID_FLAGS, 0x3ec);
    assert_eq!(crate::configret::ERROR_REGISTRY_CORRUPT, 0x3f7);
    assert_eq!(crate::configret::ERROR_SERVICE_NOT_ACTIVE, 0x426);
    assert_eq!(crate::configret::ERROR_NOT_FOUND, 0x490);
    assert_eq!(crate::configret::ERROR_UNKNOWN_PROPERTY, 0x648);
    assert_eq!(crate::configret::ERROR_INVALID_USER_BUFFER, 0x6f8);
    assert_eq!(crate::devprop::DEVPROP_TYPE_EMPTY, 0x0);
    assert_eq!(crate::devprop::DEVPROP_TYPE_NULL, 0x1);
    assert_eq!(crate::devprop::DEVPROP_TYPE_GUID, 0xd);
    assert_eq!(crate::devprop::DEVPROP_TYPE_STRING, 0x12);
    assert_eq!(crate::devprop::DEVPROP_TYPE_SECURITY_DESCRIPTOR, 0x13);
    assert_eq!(
        crate::devprop::DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING,
        0x14
    );
    assert_eq!(crate::devprop::DEVPROP_TYPE_NTSTATUS, 0x18);
    assert_eq!(crate::devprop::DEVPROP_TYPE_STRING_INDIRECT, 0x19);
    assert_eq!(crate::devprop::DEVPROP_TYPEMOD_ARRAY, 0x1000);
    assert_eq!(crate::devprop::DEVPROP_TYPEMOD_LIST, 0x2000);
    assert_eq!(crate::devprop::DEVPROP_MASK_TYPE, 0xfff);
    assert_eq!(crate::devprop::DEVPROP_MASK_TYPEMOD, 0xf000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_NONE, 0x0);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_EXISTS, 0x1);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_NOT_EXISTS, 0x10001);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_EQUALS, 0x2);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_GREATER_THAN, 0x3);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_LESS_THAN, 0x4);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_GREATER_THAN_EQUALS, 0x5);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_LESS_THAN_EQUALS, 0x6);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_BITWISE_AND, 0x7);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_BITWISE_OR, 0x8);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_BEGINS_WITH, 0x9);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_ENDS_WITH, 0xa);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_CONTAINS, 0xb);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MODIFIER_NOT, 0x10000);
    assert_eq!(
        crate::filter::DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE,
        0x20000
    );
    assert_eq!(crate::filter::DEVPROP_OPERATOR_NOT_EQUALS, 0x10002);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_EQUALS_IGNORE_CASE, 0x20002);
    assert_eq!(
        crate::filter::DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE,
        0x30002
    );
    assert_eq!(crate::filter::DEVPROP_OPERATOR_LIST_CONTAINS, 0x1000);
    assert_eq!(
        crate::filter::DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH,
        0x2000
    );
    assert_eq!(
        crate::filter::DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH,
        0x3000
    );
    assert_eq!(
        crate::filter::DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS,
        0x4000
    );
    assert_eq!(crate::filter::DEVPROP_OPERATOR_AND_OPEN, 0x100000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_AND_CLOSE, 0x200000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_OR_OPEN, 0x300000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_OR_CLOSE, 0x400000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_NOT_OPEN, 0x500000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_NOT_CLOSE, 0x600000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_ARRAY_CONTAINS, 0x10000000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MASK_EVAL, 0xfff);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MASK_LIST, 0xf000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MASK_MODIFIER, 0xf0000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MASK_NOT_LOGICAL, 0xf00fffff);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MASK_LOGICAL, 0xff00000);
    assert_eq!(crate::filter::DEVPROP_OPERATOR_MASK_ARRAY, 0xf0000000);
}
