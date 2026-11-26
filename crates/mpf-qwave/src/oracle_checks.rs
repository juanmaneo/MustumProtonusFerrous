//! Oracle-derived test vectors for `qos_create_handle`.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `QOSCreateHandle`
//! from `dlls/qwave/main.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/qwave/` and running the harness. The 21 printed rows all ran clean
//! under ASan and UBSan, so none were excluded. Every row returned `ret=0` (FALSE),
//! which matches the BOOL result documented on `qos_create_handle`.

use super::*;

/// (version as `(major, minor)` or NULL, handle_present, last_error)
type Row = (Option<(u16, u16)>, bool, u32);

#[test]
fn oracle_qos_create_handle_rows() {
    let rows: [Row; 21] = [
        (None, false, 87),
        (None, true, 87),
        (Some((1, 0)), false, 87),
        (Some((1, 0)), true, 1056),
        (Some((0, 0)), true, 87),
        (Some((0, 0)), false, 87),
        (Some((1, 1)), true, 87),
        (Some((0, 1)), true, 87),
        (Some((2, 0)), true, 87),
        (Some((1, 5)), true, 87),
        (Some((1, 256)), true, 87),
        (Some((256, 0)), true, 87),
        (Some((257, 0)), true, 87),
        (Some((65535, 0)), true, 87),
        (Some((1, 65535)), true, 87),
        (Some((65535, 65535)), true, 87),
        (Some((0, 65535)), true, 87),
        (Some((257, 256)), true, 87),
        (Some((256, 1)), true, 87),
        (Some((1, 256)), false, 87),
        (Some((65281, 0)), true, 87),
    ];
    for (version, handle_present, expected) in rows {
        let version = version.map(|(major, minor)| QosVersion { major, minor });
        assert_eq!(
            qos_create_handle(version, handle_present),
            expected,
            "version={version:?} handle_present={handle_present}"
        );
    }
}
