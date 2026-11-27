//! Oracle-derived test vectors for the ported msdelta stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/msdelta/`, and running the harness. The harness calls the stub once per `@ stub`
//! export in `dlls/msdelta/msdelta.spec`. All 15 rows ran clean under ASan and UBSan, so none are
//! excluded.

use super::*;

/// One row per export, in spec order, as the harness printed them:
/// `row N <name> -> raise x1 code=0x80000100 flags=0x1 nargs=2 p0=msdelta.dll p1=<name>`.
/// Fields: export name, code, flags, number of parameters, module, function.
const ORACLE_ROWS: [(&str, u32, u32, u32, &str, &str); 15] = [
    (
        "ApplyDeltaA",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "ApplyDeltaA",
    ),
    (
        "ApplyDeltaB",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "ApplyDeltaB",
    ),
    (
        "ApplyDeltaProvidedB",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "ApplyDeltaProvidedB",
    ),
    (
        "ApplyDeltaW",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "ApplyDeltaW",
    ),
    (
        "CreateDeltaA",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "CreateDeltaA",
    ),
    (
        "CreateDeltaB",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "CreateDeltaB",
    ),
    (
        "CreateDeltaW",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "CreateDeltaW",
    ),
    (
        "DeltaFree",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "DeltaFree",
    ),
    (
        "DeltaNormalizeProvidedB",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "DeltaNormalizeProvidedB",
    ),
    (
        "GetDeltaInfoA",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "GetDeltaInfoA",
    ),
    (
        "GetDeltaInfoB",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "GetDeltaInfoB",
    ),
    (
        "GetDeltaInfoW",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "GetDeltaInfoW",
    ),
    (
        "GetDeltaSignatureA",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "GetDeltaSignatureA",
    ),
    (
        "GetDeltaSignatureB",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "GetDeltaSignatureB",
    ),
    (
        "GetDeltaSignatureW",
        0x8000_0100,
        0x01,
        2,
        "msdelta.dll",
        "GetDeltaSignatureW",
    ),
];

#[test]
fn oracle_rows_match_each_stub_helper_and_lookup() {
    for ((name, stub), (row_name, code, flags, number_parameters, module, function)) in
        EXPORTS.iter().zip(ORACLE_ROWS)
    {
        assert_eq!(*name, row_name);
        let expected = StubException {
            code,
            flags,
            number_parameters,
            module,
            function,
        };
        assert_eq!(stub(), expected);
        assert_eq!(export_stub(row_name), Some(expected));
    }
}
