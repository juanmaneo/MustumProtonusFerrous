//! Oracle-derived vectors for the ported dsrole logic.
//!
//! Captured by compiling the verbatim Wine `DsRoleFreeMemory` and
//! `DsRoleGetPrimaryDomainInformation` bodies (`dlls/dsrole/dsrole.c` lines 37-110)
//! with Windows type shims under `target/canon/dsrole/`, using
//! `gcc -fsanitize=address,undefined -g`. The harness printed 20 rows and
//! sanitizers flagged none, so 0 rows were excluded. The host is LP64, where
//! `sizeof(DSROLE_PRIMARY_DOMAIN_INFO_BASIC)` is 48.

use super::*;

const POINTER_SIZE: usize = 8;

/// `(has_buffer, info_level, policy_open, alloc_ok, domain, ret, flat_name_and_alloc_size)`.
/// `None` means Wine left `*Buffer` NULL.
type Row = (
    bool,
    i32,
    bool,
    bool,
    &'static str,
    u32,
    Option<(&'static str, usize)>,
);

const ROWS: [Row; 20] = [
    (true, 1, true, true, "WORKGROUP", 0, Some(("WORKGROUP", 68))),
    (true, 1, true, true, "", 0, Some(("", 50))),
    (
        true,
        1,
        true,
        true,
        "CORP.EXAMPLE",
        0,
        Some(("CORP.EXAMPLE", 74)),
    ),
    (true, 1, false, true, "WORKGROUP", 14, None),
    (true, 1, true, false, "WORKGROUP", 14, None),
    (true, 1, false, false, "CORP.EXAMPLE", 14, None),
    (false, 1, true, true, "WORKGROUP", 87, None),
    (true, 0, true, true, "WORKGROUP", 87, None),
    (true, 4, true, true, "WORKGROUP", 87, None),
    (true, -1, true, true, "WORKGROUP", 87, None),
    (false, 0, true, true, "WORKGROUP", 87, None),
    (true, 2, true, true, "WORKGROUP", 120, None),
    (true, 3, true, true, "", 120, None),
    (true, 2, false, false, "WORKGROUP", 120, None),
    (false, 3, true, true, "CORP.EXAMPLE", 87, None),
    (
        true,
        1,
        true,
        true,
        "CORP.EXAMPLE",
        0,
        Some(("CORP.EXAMPLE", 74)),
    ),
    (true, 5, true, true, "WORKGROUP", 87, None),
    (true, 1, false, true, "", 14, None),
    (true, 2147483647, true, true, "WORKGROUP", 87, None),
    (true, -2147483647, true, true, "WORKGROUP", 87, None),
];

/// NUL-terminated UTF-16 form of an ASCII domain name, as the LSA returns it.
fn wide(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

#[test]
fn oracle_sizeof_basic_rows() {
    assert_eq!(basic_struct_size(POINTER_SIZE), 48);
}

#[test]
fn oracle_primary_domain_information_rows() {
    for (i, &(has_buffer, info_level, policy_open, alloc_ok, domain, ret, expected)) in
        ROWS.iter().enumerate()
    {
        let domain_name = wide(domain);
        let reply = get_primary_domain_information(&Query {
            has_buffer,
            info_level,
            policy_open,
            alloc_ok,
            pointer_size: POINTER_SIZE,
            domain_name: &domain_name,
        });
        assert_eq!(reply.ret, ret, "row {}", i + 1);
        let want = expected.map(|(flat, alloc_size)| PrimaryDomainBasic {
            machine_role: 0,
            domain_name_flat: wide(flat),
            alloc_size,
        });
        assert_eq!(reply.buffer, want, "row {}", i + 1);
    }
}
