//! Edge cases: every branch of the mapping and validation functions, and the documented
//! divergences from the C code. Expectations follow the C in `dlls/cfgmgr32`
//! (`devobject.c`, `cfgmgr32.c`). The oracle rows in `oracle_checks.rs` cover the
//! behaviour that the C harness can run directly.

use crate::configret::*;
use crate::devprop::*;
use crate::filter::*;
use crate::{comp_key_eq, CompKey, Filter, Property};

const NONE: u32 = DEVPROP_OPERATOR_NONE;
const EXISTS: u32 = DEVPROP_OPERATOR_EXISTS;
const NOT_EXISTS: u32 = DEVPROP_OPERATOR_NOT_EXISTS;
const EQUALS: u32 = DEVPROP_OPERATOR_EQUALS;
const NOT_EQUALS: u32 = DEVPROP_OPERATOR_NOT_EQUALS;
const EQUALS_IGNORE_CASE: u32 = DEVPROP_OPERATOR_EQUALS_IGNORE_CASE;
const LESS_THAN: u32 = DEVPROP_OPERATOR_LESS_THAN;
const GREATER_THAN_EQUALS: u32 = DEVPROP_OPERATOR_GREATER_THAN_EQUALS;
const BITWISE_AND: u32 = DEVPROP_OPERATOR_BITWISE_AND;
const BITWISE_OR: u32 = DEVPROP_OPERATOR_BITWISE_OR;
const BEGINS_WITH: u32 = DEVPROP_OPERATOR_BEGINS_WITH;
const ENDS_WITH: u32 = DEVPROP_OPERATOR_ENDS_WITH;
const CONTAINS: u32 = DEVPROP_OPERATOR_CONTAINS;
const NOT: u32 = DEVPROP_OPERATOR_MODIFIER_NOT;
const IGNORE_CASE: u32 = DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE;
const LIST_CONTAINS: u32 = DEVPROP_OPERATOR_LIST_CONTAINS;
const ARRAY_CONTAINS: u32 = DEVPROP_OPERATOR_ARRAY_CONTAINS;
const AND_OPEN: u32 = DEVPROP_OPERATOR_AND_OPEN;
const AND_CLOSE: u32 = DEVPROP_OPERATOR_AND_CLOSE;
const OR_OPEN: u32 = DEVPROP_OPERATOR_OR_OPEN;
const OR_CLOSE: u32 = DEVPROP_OPERATOR_OR_CLOSE;
const NOT_OPEN: u32 = DEVPROP_OPERATOR_NOT_OPEN;
const NOT_CLOSE: u32 = DEVPROP_OPERATOR_NOT_CLOSE;

const STRING: u32 = DEVPROP_TYPE_STRING;
const STRING_LIST: u32 = 0x2012;
const EMPTY: u32 = DEVPROP_TYPE_EMPTY;
const GUID: u32 = DEVPROP_TYPE_GUID;
const INT32: u32 = 0x06;
const UINT32: u32 = 0x07;
const BYTE: u32 = 0x03;
const INT32_LIST: u32 = 0x2006;

const FMT_A: [u8; 16] = [1; 16];
const FMT_B: [u8; 16] = [2; 16];
const G1: [u8; 16] = [1; 16];
const G2: [u8; 16] = [2; 16];

/// NUL-terminated UTF-16LE "AB", "ab" and "AC".
const AB: &[u8] = b"A\0B\0\0\0";
const AB_LOWER: &[u8] = b"a\0b\0\0\0";
const AC: &[u8] = b"A\0C\0\0\0";

fn pa(pid: u32, ty: u32, buffer: Option<&[u8]>) -> Property<'_> {
    prop(FMT_A, pid, 0, None, ty, buffer)
}

fn eq_ab(pid: u32) -> Filter<'static> {
    flt(EQUALS, pa(pid, STRING, Some(AB)))
}

fn grp(operator: u32) -> Filter<'static> {
    flt(operator, pa(0, EMPTY, None))
}

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

fn flt(operator: u32, property: Property<'_>) -> Filter<'_> {
    Filter { operator, property }
}

#[test]
fn cr_to_win32_error_maps_each_listed_code() {
    let cases = [
        (CR_SUCCESS, ERROR_SUCCESS),
        (CR_OUT_OF_MEMORY, ERROR_NOT_ENOUGH_MEMORY),
        (CR_INVALID_POINTER, ERROR_INVALID_USER_BUFFER),
        (CR_INVALID_FLAG, ERROR_INVALID_FLAGS),
        (CR_INVALID_DEVNODE, ERROR_INVALID_DATA),
        (CR_INVALID_DEVICE_ID, ERROR_INVALID_DATA),
        (CR_INVALID_MACHINENAME, ERROR_INVALID_DATA),
        (CR_INVALID_PROPERTY, ERROR_INVALID_DATA),
        (CR_INVALID_REFERENCE_STRING, ERROR_INVALID_DATA),
        (CR_NO_SUCH_DEVNODE, ERROR_NOT_FOUND),
        (CR_NO_SUCH_VALUE, ERROR_NOT_FOUND),
        (CR_NO_SUCH_DEVICE_INTERFACE, ERROR_NOT_FOUND),
        (CR_ALREADY_SUCH_DEVNODE, ERROR_ALREADY_EXISTS),
        (CR_BUFFER_SMALL, ERROR_INSUFFICIENT_BUFFER),
        (CR_NO_REGISTRY_HANDLE, ERROR_INVALID_HANDLE),
        (CR_REGISTRY_ERROR, ERROR_REGISTRY_CORRUPT),
        (CR_NO_SUCH_REGISTRY_KEY, ERROR_FILE_NOT_FOUND),
        (CR_REMOTE_COMM_FAILURE, ERROR_SERVICE_NOT_ACTIVE),
        (CR_MACHINE_UNAVAILABLE, ERROR_SERVICE_NOT_ACTIVE),
        (CR_NO_CM_SERVICES, ERROR_SERVICE_NOT_ACTIVE),
        (CR_ACCESS_DENIED, ERROR_ACCESS_DENIED),
        (CR_CALL_NOT_IMPLEMENTED, ERROR_CALL_NOT_IMPLEMENTED),
    ];
    for (code, error) in cases {
        assert_eq!(cr_to_win32_error(code, 0xdead), error, "code {code:#x}");
    }
}

#[test]
fn cr_to_win32_error_uses_default_for_other_codes() {
    assert_eq!(cr_to_win32_error(CR_FAILURE, 1234), 1234);
    assert_eq!(cr_to_win32_error(CR_INVALID_DATA, 5), 5);
    assert_eq!(cr_to_win32_error(CR_SUCCESS, 1234), ERROR_SUCCESS);
}

#[test]
fn map_error_maps_each_listed_error() {
    let cases = [
        (ERROR_INVALID_PARAMETER, CR_FAILURE),
        (ERROR_INVALID_USER_BUFFER, CR_INVALID_POINTER),
        (ERROR_FILE_NOT_FOUND, CR_NO_SUCH_REGISTRY_KEY),
        (ERROR_MORE_DATA, CR_BUFFER_SMALL),
        (ERROR_NO_MORE_ITEMS, CR_NO_SUCH_VALUE),
        (ERROR_NOT_FOUND, CR_NO_SUCH_VALUE),
        (ERROR_SUCCESS, CR_SUCCESS),
        (ERROR_UNKNOWN_PROPERTY, CR_INVALID_PROPERTY),
    ];
    for (err, code) in cases {
        assert_eq!(map_error(err), code, "error {err}");
    }
}

#[test]
fn map_error_falls_back_to_cr_failure() {
    for err in [ERROR_ACCESS_DENIED, ERROR_INVALID_DATA, 0x7fff_ffff] {
        assert_eq!(map_error(err), CR_FAILURE, "error {err}");
    }
}

#[test]
fn map_error_node_maps_missing_nodes() {
    assert_eq!(map_error_node(ERROR_INVALID_DATA), CR_INVALID_DEVNODE);
    assert_eq!(map_error_node(ERROR_NOT_FOUND), CR_NO_SUCH_DEVNODE);
    assert_eq!(map_error_node(ERROR_FILE_NOT_FOUND), CR_NO_SUCH_DEVNODE);
}

#[test]
fn map_error_node_defers_to_map_error() {
    assert_eq!(map_error_node(ERROR_SUCCESS), CR_SUCCESS);
    assert_eq!(map_error_node(ERROR_INVALID_PARAMETER), CR_FAILURE);
    assert_eq!(map_error_node(ERROR_MORE_DATA), CR_BUFFER_SMALL);
}

#[test]
fn type_validate_fixed_types_need_exact_size() {
    assert!(type_validate(INT32, 4));
    assert!(!type_validate(INT32, 3));
    assert!(type_validate(GUID, 16));
    assert!(!type_validate(GUID, 15));
    assert!(type_validate(DEVPROP_TYPE_NTSTATUS, 4));
    assert!(!type_validate(DEVPROP_TYPE_NTSTATUS, 3));
    // DEVPROPKEY is 20 bytes and DEVPROPTYPE is 4.
    assert!(type_validate(0x15, 20));
    assert!(!type_validate(0x15, 19));
    assert!(type_validate(0x16, 4));
}

#[test]
fn type_validate_array_needs_at_least_the_element_size() {
    assert!(type_validate(0x1000 | INT32, 8));
    assert!(type_validate(0x1000 | INT32, 4));
    assert!(!type_validate(0x1000 | INT32, 3));
    assert!(type_validate(0x1000 | GUID, 16));
    assert!(!type_validate(0x1000 | GUID, 0));
}

#[test]
fn type_validate_empty_and_null_take_no_modifier() {
    assert!(type_validate(EMPTY, 0));
    assert!(type_validate(EMPTY, 8));
    assert!(type_validate(DEVPROP_TYPE_NULL, 0));
    assert!(!type_validate(0x1000 | EMPTY, 0));
    assert!(!type_validate(0x2000 | EMPTY, 0));
    assert!(!type_validate(0x1000 | DEVPROP_TYPE_NULL, 0));
}

#[test]
fn type_validate_security_descriptor_and_string_indirect_need_a_size() {
    assert!(type_validate(DEVPROP_TYPE_SECURITY_DESCRIPTOR, 1));
    assert!(!type_validate(DEVPROP_TYPE_SECURITY_DESCRIPTOR, 0));
    assert!(!type_validate(0x1000 | DEVPROP_TYPE_SECURITY_DESCRIPTOR, 1));
    assert!(!type_validate(0x2000 | DEVPROP_TYPE_SECURITY_DESCRIPTOR, 1));
    assert!(type_validate(DEVPROP_TYPE_STRING_INDIRECT, 2));
    assert!(!type_validate(DEVPROP_TYPE_STRING_INDIRECT, 0));
    assert!(!type_validate(0x1000 | DEVPROP_TYPE_STRING_INDIRECT, 2));
}

#[test]
fn type_validate_strings_allow_lists_but_not_arrays() {
    assert!(!type_validate(STRING, 0));
    assert!(type_validate(STRING, 2));
    assert!(type_validate(STRING_LIST, 2));
    assert!(!type_validate(0x1000 | STRING, 2));
    assert!(type_validate(DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING, 2));
    assert!(type_validate(
        0x2000 | DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING,
        2
    ));
    assert!(!type_validate(
        0x1000 | DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING,
        2
    ));
    assert!(!type_validate(DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING, 0));
}

#[test]
fn type_validate_rejects_unknown_modifiers_and_bases() {
    assert!(!type_validate(0x4000 | INT32, 4));
    assert!(!type_validate(0x3000 | STRING, 2));
    assert!(!type_validate(0x8000 | EMPTY, 0));
    assert!(!type_validate(0x1a, 4));
    assert!(!type_validate(0x101a, 4));
    assert!(!type_validate(0x0fff, 4));
    assert!(!type_validate(INT32_LIST, 4));
    assert!(type_validate(0x11, 1));
    assert!(!type_validate(0x2011, 1));
}

#[test]
fn type_validate_ignores_bits_above_the_type_mask() {
    assert!(type_validate(0x10000 | STRING, 4));
    assert!(type_validate(0x10000 | INT32, 4));
}

#[test]
fn filters_validate_rejects_empty_list() {
    assert!(!filters_validate(&[]));
}

#[test]
fn filters_validate_accepts_none_and_exists_without_a_type_check() {
    assert!(filters_validate(&[flt(NONE, pa(1, INT32_LIST, None))]));
    assert!(filters_validate(&[flt(EXISTS, pa(1, INT32_LIST, None))]));
    assert!(filters_validate(&[flt(NOT, pa(1, INT32_LIST, None))]));
}

#[test]
fn filters_validate_checks_compare_and_modifier_bits() {
    let ok = pa(1, STRING, Some(AB));
    assert!(filters_validate(&[flt(EQUALS, ok)]));
    assert!(filters_validate(&[flt(EQUALS_IGNORE_CASE, ok)]));
    assert!(filters_validate(&[flt(CONTAINS, ok)]));
    assert!(!filters_validate(&[flt(0xc, ok)]));
    assert!(!filters_validate(&[flt(0xfff, ok)]));
    assert!(!filters_validate(&[flt(EQUALS | 0x30000, ok)]));
    assert!(!filters_validate(&[flt(EQUALS | 0x40000, ok)]));
    assert!(!filters_validate(&[flt(EQUALS | 0x5000, ok)]));
    assert!(filters_validate(&[flt(EQUALS | LIST_CONTAINS, ok)]));
}

#[test]
fn filters_validate_rejects_logical_operators_with_other_bits() {
    let ok = pa(1, STRING, Some(AB));
    assert!(!filters_validate(&[flt(AND_OPEN | EQUALS, ok)]));
    assert!(!filters_validate(&[flt(NOT_OPEN | NOT, ok)]));
    assert!(!filters_validate(&[flt(AND_CLOSE | LIST_CONTAINS, ok)]));
}

#[test]
fn filters_validate_checks_array_operators() {
    let ok = pa(1, STRING, Some(AB));
    assert!(filters_validate(&[flt(ARRAY_CONTAINS, ok)]));
    assert!(!filters_validate(&[flt(ARRAY_CONTAINS | EQUALS, ok)]));
}

#[test]
fn filters_validate_checks_property_buffers_and_types() {
    assert!(!filters_validate(&[flt(EQUALS, pa(1, STRING, Some(&[])))]));
    assert!(!filters_validate(&[flt(EQUALS, pa(1, INT32, Some(AB)))]));
    assert!(!filters_validate(&[flt(
        EQUALS,
        pa(1, INT32_LIST, Some(&[1, 0, 0, 0]))
    )]));
    assert!(filters_validate(&[flt(
        EQUALS,
        pa(1, INT32, Some(&[1, 0, 0, 0]))
    )]));
    assert!(filters_validate(&[flt(
        EQUALS,
        pa(1, STRING_LIST, Some(AB))
    )]));
}

#[test]
fn filters_validate_checks_groups() {
    assert!(filters_validate(&[grp(AND_OPEN), eq_ab(1), grp(AND_CLOSE)]));
    assert!(filters_validate(&[grp(NOT_OPEN), eq_ab(1), grp(NOT_CLOSE)]));
    assert!(filters_validate(&[
        grp(AND_OPEN),
        eq_ab(1),
        grp(AND_CLOSE),
        eq_ab(2)
    ]));
    assert!(!filters_validate(&[grp(AND_OPEN), eq_ab(1)]));
    assert!(!filters_validate(&[grp(AND_CLOSE)]));
    assert!(!filters_validate(&[grp(AND_OPEN), eq_ab(1), grp(OR_CLOSE)]));
    assert!(!filters_validate(&[grp(AND_OPEN), grp(AND_CLOSE)]));
    assert!(!filters_validate(&[
        grp(NOT_OPEN),
        flt(NOT_EQUALS | IGNORE_CASE, pa(1, STRING, Some(AB))),
        grp(NOT_CLOSE),
    ]));
}

#[test]
fn filters_validate_nests_groups_by_kind() {
    assert!(filters_validate(&[
        grp(AND_OPEN),
        grp(AND_OPEN),
        eq_ab(1),
        grp(AND_CLOSE),
        eq_ab(2),
        grp(AND_CLOSE),
    ]));
    assert!(!filters_validate(&[
        grp(AND_OPEN),
        grp(AND_OPEN),
        eq_ab(1),
        grp(AND_CLOSE)
    ]));
    assert!(filters_validate(&[
        grp(OR_OPEN),
        grp(AND_OPEN),
        eq_ab(1),
        grp(AND_CLOSE),
        grp(OR_CLOSE),
    ]));
}

#[test]
fn filters_match_none_operator_fails() {
    let props = [pa(1, STRING, Some(AB))];
    assert!(!filters_match(
        &props,
        &[flt(NONE, pa(1, STRING, Some(AB)))]
    ));
}

#[test]
fn filters_match_bare_modifier_is_true() {
    // No branch applies to a bare NOT, so the result keeps its initial TRUE.
    let props = [pa(1, STRING, Some(AB))];
    assert!(filters_match(&props, &[flt(NOT, pa(1, STRING, Some(AB)))]));
}

#[test]
fn filters_match_without_filters_matches() {
    assert!(filters_match(&[], &[]));
    assert!(filters_match(&[pa(1, STRING, Some(AB))], &[]));
}

#[test]
fn filters_match_exists_checks_presence_and_type() {
    let props = [pa(1, STRING, Some(AB)), pa(2, EMPTY, None)];
    assert!(filters_match(&props, &[flt(EXISTS, pa(1, STRING, None))]));
    assert!(!filters_match(&props, &[flt(EXISTS, pa(2, EMPTY, None))]));
    assert!(!filters_match(&props, &[flt(EXISTS, pa(3, STRING, None))]));
    assert!(filters_match(
        &props,
        &[flt(NOT_EXISTS, pa(3, STRING, None))]
    ));
    assert!(!filters_match(
        &props,
        &[flt(NOT_EXISTS, pa(1, STRING, None))]
    ));
}

#[test]
fn filters_match_locale_none_differs_from_empty() {
    let null_locale = prop(FMT_A, 1, 0, None, STRING, Some(AB));
    let empty_locale = prop(FMT_A, 1, 0, Some(""), STRING, Some(AB));
    assert!(filters_match(&[null_locale], &[flt(EQUALS, null_locale)]));
    assert!(!filters_match(&[null_locale], &[flt(EQUALS, empty_locale)]));
    assert!(!filters_match(&[empty_locale], &[flt(EQUALS, null_locale)]));
}

#[test]
fn filters_match_requires_the_full_key() {
    let base = prop(FMT_A, 1, 0, None, STRING, Some(AB));
    let other_store = prop(FMT_A, 1, 1, None, STRING, Some(AB));
    let other_pid = prop(FMT_A, 2, 0, None, STRING, Some(AB));
    let other_fmtid = prop(FMT_B, 1, 0, None, STRING, Some(AB));
    for other in [other_store, other_pid, other_fmtid] {
        assert!(!filters_match(&[other], &[flt(EQUALS, base)]));
    }
    assert!(filters_match(&[base], &[flt(EQUALS, base)]));
}

#[test]
fn filters_match_requires_the_same_type_and_size() {
    let long = prop(FMT_A, 1, 0, None, INT32, Some(&[1, 0, 0, 0, 0, 0, 0, 0]));
    let short = prop(FMT_A, 1, 0, None, INT32, Some(&[1, 0, 0, 0]));
    assert!(!filters_match(&[long], &[flt(EQUALS, short)]));
    assert!(filters_match(&[long], &[flt(NOT_EQUALS, short)]));
    let unsigned = prop(FMT_A, 1, 0, None, UINT32, Some(&[1, 0, 0, 0]));
    assert!(!filters_match(&[short], &[flt(EQUALS, unsigned)]));
}

#[test]
fn filters_match_string_equality_is_case_sensitive() {
    let props = [pa(1, STRING, Some(AB))];
    assert!(!filters_match(
        &props,
        &[flt(EQUALS, pa(1, STRING, Some(AB_LOWER)))]
    ));
    assert!(filters_match(
        &props,
        &[flt(EQUALS_IGNORE_CASE, pa(1, STRING, Some(AB_LOWER)))]
    ));
}

#[test]
fn filters_match_ignore_case_folds_only_ascii_letters() {
    let upper_z: &[u8] = b"Z\0\0\0";
    let lower_z: &[u8] = b"z\0\0\0";
    let at: &[u8] = b"@\0\0\0";
    let grave: &[u8] = b"`\0\0\0";
    assert!(filters_match(
        &[pa(1, STRING, Some(upper_z))],
        &[flt(EQUALS_IGNORE_CASE, pa(1, STRING, Some(lower_z)))]
    ));
    assert!(!filters_match(
        &[pa(1, STRING, Some(at))],
        &[flt(EQUALS_IGNORE_CASE, pa(1, STRING, Some(grave)))]
    ));
}

#[test]
fn filters_match_string_stops_at_the_first_nul() {
    let a: &[u8] = &[0x41, 0, 0x42, 0, 0, 0, 0x43, 0, 0x44, 0, 0, 0];
    let b: &[u8] = &[0x41, 0, 0x42, 0, 0, 0, 0x58, 0, 0x59, 0, 0, 0];
    assert!(filters_match(
        &[pa(1, STRING, Some(a))],
        &[flt(EQUALS, pa(1, STRING, Some(b)))]
    ));
}

#[test]
fn filters_match_string_without_nul_ends_at_the_buffer() {
    // The C wcscmp would read past the buffer here. The port stops at the end instead.
    let ab: &[u8] = &[0x41, 0, 0x42, 0];
    let ac: &[u8] = &[0x41, 0, 0x43, 0];
    assert!(!filters_match(
        &[pa(1, STRING, Some(ab))],
        &[flt(EQUALS, pa(1, STRING, Some(ac)))]
    ));
    assert!(filters_match(
        &[pa(1, STRING, Some(ab))],
        &[flt(LESS_THAN, pa(1, STRING, Some(ac)))]
    ));
}

#[test]
fn filters_match_ignores_a_trailing_odd_byte() {
    let a: &[u8] = &[0x41, 0, 0x42];
    let b: &[u8] = &[0x41, 0, 0x43];
    assert!(filters_match(
        &[pa(1, STRING, Some(a))],
        &[flt(EQUALS, pa(1, STRING, Some(b)))]
    ));
}

#[test]
fn filters_match_orders_bytes_unsigned() {
    let one: &[u8] = &[0x01];
    let two: &[u8] = &[0x02];
    let high: &[u8] = &[0x80];
    assert!(filters_match(
        &[pa(1, BYTE, Some(one))],
        &[flt(LESS_THAN, pa(1, BYTE, Some(two)))]
    ));
    assert!(!filters_match(
        &[pa(1, BYTE, Some(two))],
        &[flt(LESS_THAN, pa(1, BYTE, Some(one)))]
    ));
    assert!(!filters_match(
        &[pa(1, BYTE, Some(high))],
        &[flt(LESS_THAN, pa(1, BYTE, Some(one)))]
    ));
    assert!(filters_match(
        &[pa(1, BYTE, Some(one))],
        &[flt(GREATER_THAN_EQUALS, pa(1, BYTE, Some(two)))]
    ));
    assert!(!filters_match(
        &[pa(1, BYTE, Some(two))],
        &[flt(GREATER_THAN_EQUALS, pa(1, BYTE, Some(one)))]
    ));
}

#[test]
fn filters_match_guid_only_equality_is_ordered() {
    let same = prop(FMT_A, 1, 0, None, GUID, Some(&G1));
    let other = prop(FMT_A, 1, 0, None, GUID, Some(&G2));
    assert!(filters_match(
        &[same],
        &[flt(EQUALS, pa(1, GUID, Some(&G1)))]
    ));
    assert!(!filters_match(&[same], &[flt(EQUALS, other)]));
    assert!(!filters_match(&[same], &[flt(LESS_THAN, other)]));
    assert!(!filters_match(&[same], &[flt(GREATER_THAN_EQUALS, other)]));
    assert!(filters_match(&[same], &[flt(NOT_EQUALS, other)]));
}

#[test]
fn filters_match_unsupported_operators_never_match() {
    let props = [pa(1, STRING, Some(AB))];
    for op in [
        BITWISE_AND,
        BITWISE_OR,
        BEGINS_WITH,
        ENDS_WITH,
        CONTAINS,
        0xc,
        0xfff,
    ] {
        assert!(
            !filters_match(&props, &[flt(op, pa(1, STRING, Some(AB)))]),
            "op {op:#x}"
        );
        assert!(
            !filters_match(&props, &[flt(op | NOT, pa(1, STRING, Some(AB)))]),
            "op {op:#x} with NOT"
        );
    }
}

#[test]
fn filters_match_list_and_array_operators_fail() {
    let props = [pa(1, STRING_LIST, Some(AB))];
    assert!(!filters_match(
        &props,
        &[flt(EQUALS | LIST_CONTAINS, pa(1, STRING_LIST, Some(AB)))]
    ));
    assert!(!filters_match(
        &props,
        &[flt(ARRAY_CONTAINS, pa(1, STRING_LIST, Some(AB)))]
    ));
}

#[test]
fn filters_match_and_group_stops_at_the_first_failure() {
    let props = [pa(1, STRING, Some(AB))];
    assert!(!filters_match(
        &props,
        &[grp(AND_OPEN), eq_ab(2), eq_ab(1), grp(AND_CLOSE)]
    ));
    assert!(!filters_match(&props, &[eq_ab(2), eq_ab(1)]));
    assert!(filters_match(
        &props,
        &[grp(AND_OPEN), eq_ab(1), eq_ab(1), grp(AND_CLOSE)]
    ));
}

#[test]
fn filters_match_or_group_needs_one_match() {
    let props = [pa(1, STRING, Some(AB))];
    assert!(!filters_match(
        &props,
        &[grp(OR_OPEN), eq_ab(2), eq_ab(3), grp(OR_CLOSE)]
    ));
}

#[test]
fn filters_match_not_group_negates_its_result() {
    let props = [pa(1, STRING, Some(AB))];
    assert!(filters_match(
        &props,
        &[grp(NOT_OPEN), eq_ab(2), grp(NOT_CLOSE)]
    ));
}

#[test]
fn filters_match_unclosed_group_reads_the_rest_of_the_list() {
    // The C code would dereference the NULL that find_closing_filter returns. The port
    // treats the rest of the list as the group.
    assert!(filters_match(
        &[pa(1, STRING, Some(AB))],
        &[grp(AND_OPEN), eq_ab(1)]
    ));
}

#[test]
fn filters_match_accepts_unsorted_properties() {
    let props = [pa(2, STRING, Some(AB)), pa(1, STRING, Some(AB))];
    assert!(filters_match(
        &props,
        &[flt(EQUALS, pa(1, STRING, Some(AB)))]
    ));
}

#[test]
fn filters_match_duplicate_keys_use_the_first() {
    let props = [pa(1, STRING, Some(AB)), pa(1, STRING, Some(AC))];
    assert!(filters_match(
        &props,
        &[flt(EQUALS, pa(1, STRING, Some(AB)))]
    ));
    assert!(!filters_match(
        &props,
        &[flt(EQUALS, pa(1, STRING, Some(AC)))]
    ));
}

#[test]
fn comp_key_eq_compares_each_field() {
    let base = CompKey {
        fmtid: FMT_A,
        pid: 1,
        store: 0,
        locale: Some("en"),
    };
    assert!(comp_key_eq(
        &base,
        &CompKey {
            locale: Some("EN"),
            ..base
        }
    ));
    assert!(!comp_key_eq(
        &base,
        &CompKey {
            fmtid: FMT_B,
            ..base
        }
    ));
    assert!(!comp_key_eq(&base, &CompKey { pid: 2, ..base }));
    assert!(!comp_key_eq(&base, &CompKey { store: 1, ..base }));
    assert!(!comp_key_eq(
        &base,
        &CompKey {
            locale: Some("de"),
            ..base
        }
    ));
    assert!(!comp_key_eq(
        &base,
        &CompKey {
            locale: None,
            ..base
        }
    ));
    let none = CompKey {
        locale: None,
        ..base
    };
    assert!(!comp_key_eq(&none, &base));
    assert!(comp_key_eq(
        &none,
        &CompKey {
            locale: None,
            ..base
        }
    ));
}

#[test]
fn property_size_is_the_buffer_length() {
    assert_eq!(pa(1, STRING, None).size(), 0);
    assert_eq!(pa(1, STRING, Some(&[])).size(), 0);
    assert_eq!(pa(1, STRING, Some(AB)).size(), 6);
}
