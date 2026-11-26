//! Edge-case tests for the ported dsrole logic.
//!
//! Expectations are oracle-derived unless the comment on the test says otherwise.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

fn query(domain_name: &[u16]) -> Query<'_> {
    Query {
        has_buffer: true,
        info_level: PRIMARY_DOMAIN_INFO_BASIC,
        policy_open: true,
        alloc_ok: true,
        pointer_size: 8,
        domain_name,
    }
}

#[test]
fn route_level_boundaries() {
    // Oracle-derived: levels 0, 1, 2, 3, 4 and -1 (rows 8-13 and 10).
    assert_eq!(route(true, 0), Err(ERROR_INVALID_PARAMETER));
    assert_eq!(route(true, 1), Ok(InfoRoute::Basic));
    assert_eq!(route(true, 2), Ok(InfoRoute::NotImplemented));
    assert_eq!(route(true, 3), Ok(InfoRoute::NotImplemented));
    assert_eq!(route(true, 4), Err(ERROR_INVALID_PARAMETER));
    assert_eq!(route(true, -1), Err(ERROR_INVALID_PARAMETER));
}

#[test]
fn route_extreme_levels() {
    // Not oracle-derived: i32::MIN and i32::MAX are outside the enum range by inspection of dsrole.h.
    assert_eq!(route(true, i32::MIN), Err(ERROR_INVALID_PARAMETER));
    assert_eq!(route(true, i32::MAX), Err(ERROR_INVALID_PARAMETER));
}

#[test]
fn route_null_buffer_wins_over_level() {
    // Oracle-derived: row 7 and row 11 (no buffer, valid level) and row 15.
    assert_eq!(route(false, 1), Err(ERROR_INVALID_PARAMETER));
    assert_eq!(route(false, 2), Err(ERROR_INVALID_PARAMETER));
}

#[test]
fn not_implemented_levels_skip_lsa() {
    // Oracle-derived: row 14 has policy_open and alloc_ok false, yet the stub code wins.
    let domain = wide("WORKGROUP");
    let reply = get_primary_domain_information(&Query {
        info_level: 3,
        policy_open: false,
        alloc_ok: false,
        ..query(&domain)
    });
    assert_eq!(
        reply,
        Reply {
            ret: ERROR_CALL_NOT_IMPLEMENTED,
            buffer: None
        }
    );
}

#[test]
fn policy_failure_wins_over_alloc() {
    // Oracle-derived: row 6 (both fail, ERROR_OUTOFMEMORY).
    let domain = wide("CORP.EXAMPLE");
    let reply = get_primary_domain_information(&Query {
        policy_open: false,
        alloc_ok: false,
        ..query(&domain)
    });
    assert_eq!(
        reply,
        Reply {
            ret: ERROR_OUTOFMEMORY,
            buffer: None
        }
    );
}

#[test]
fn copy_stops_at_first_nul() {
    // Not oracle-derived: embedded NUL, lstrcpyW copies through the first terminator only.
    let domain = [b'A' as u16, b'B' as u16, 0, b'C' as u16, 0];
    assert_eq!(wide_strlen(&domain), 2);
    assert_eq!(copy_domain_name(&domain), vec![b'A' as u16, b'B' as u16, 0]);
    assert_eq!(basic_alloc_size(8, &domain), 48 + 3 * 2);
}

#[test]
fn copy_without_terminator_uses_slice_length() {
    // Not oracle-derived: the C code would read past the slice; the port stops at the slice end.
    let domain = [b'A' as u16, b'B' as u16];
    assert_eq!(wide_strlen(&domain), 2);
    assert_eq!(copy_domain_name(&domain), vec![b'A' as u16, b'B' as u16, 0]);
}

#[test]
fn empty_slice_has_zero_length() {
    // Not oracle-derived: no terminator at all. Empty domain names with a NUL are oracle row 2.
    assert_eq!(wide_strlen(&[]), 0);
    assert_eq!(copy_domain_name(&[]), vec![0]);
    assert_eq!(basic_alloc_size(8, &[]), 50);
}

#[test]
fn struct_size_for_32_bit_pointers() {
    // Not oracle-derived: 4 + 4 + 3 * 4 + 16 from the dsrole.h field list, no padding on ILP32.
    assert_eq!(basic_struct_size(4), 36);
}

#[test]
fn success_reports_standalone_workstation_and_flat_name() {
    // Oracle-derived: row 1 (WORKGROUP) with pointer_size 8 gives 68 bytes; this test
    // uses pointer_size 4 instead, so the 56-byte size is not oracle-derived: 36 + (9 + 1) * 2.
    let domain = wide("WORKGROUP");
    let reply = get_primary_domain_information(&Query {
        pointer_size: 4,
        ..query(&domain)
    });
    assert_eq!(reply.ret, ERROR_SUCCESS);
    let basic = reply.buffer.expect("basic info");
    assert_eq!(basic.machine_role, ROLE_STANDALONE_WORKSTATION);
    assert_eq!(basic.alloc_size, 56);
    assert_eq!(basic.domain_name_flat, domain);
}
