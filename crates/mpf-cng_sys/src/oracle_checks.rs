//! Oracle-derived test vectors for the ported cng.sys logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DriverEntry` from
//! `dlls/cng.sys/main.c` and of `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`
//! with `gcc -fsanitize=address,undefined -g`, under `target/canon/cng.sys/`. The harness printed
//! 16 rows (R01-R16). ASan and UBSan reported nothing, so no row is excluded.

use super::*;

/// `(row id, expected NTSTATUS)`. `DriverEntry` ignores its arguments, so R01 and R02 differ only
/// in the pointers the C harness passed.
const DRIVER_ENTRY_ROWS: [(&str, u32); 2] = [("R01", 0x0000_0000), ("R02", 0x0000_0000)];

/// `(row id, export name, code, flags, number of parameters, module, function)`.
type StubRow = (
    &'static str,
    &'static str,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// R03-R16: one row per stub call. The C harness printed the fields of the exception record.
const STUB_ROWS: [StubRow; 14] = [
    (
        "R03",
        "BCryptCreateMultiHash",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "BCryptCreateMultiHash",
    ),
    (
        "R04",
        "BCryptEnumProviders",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "BCryptEnumProviders",
    ),
    (
        "R05",
        "BCryptProcessMultiOperations",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "BCryptProcessMultiOperations",
    ),
    (
        "R06",
        "BCryptRegisterConfigChangeNotify",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "BCryptRegisterConfigChangeNotify",
    ),
    (
        "R07",
        "BCryptResolveProviders",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "BCryptResolveProviders",
    ),
    (
        "R08",
        "BCryptUnregisterConfigChangeNotify",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "BCryptUnregisterConfigChangeNotify",
    ),
    (
        "R09",
        "CngGetFipsAlgorithmMode",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "CngGetFipsAlgorithmMode",
    ),
    (
        "R10",
        "EntropyPoolTriggerReseedForIum",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "EntropyPoolTriggerReseedForIum",
    ),
    (
        "R11",
        "EntropyProvideData",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "EntropyProvideData",
    ),
    (
        "R12",
        "EntropyRegisterCallback",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "EntropyRegisterCallback",
    ),
    (
        "R13",
        "SslDecryptPacket",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "SslDecryptPacket",
    ),
    (
        "R14",
        "SslOpenProvider",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "SslOpenProvider",
    ),
    (
        "R15",
        "SymCrypt802_11SaeCustomInit",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "SymCrypt802_11SaeCustomInit",
    ),
    (
        "R16",
        "SystemPrng",
        0x8000_0100,
        0x1,
        2,
        "cng.sys",
        "SystemPrng",
    ),
];

#[test]
fn oracle_driver_entry_rows() {
    for (row, expected) in DRIVER_ENTRY_ROWS {
        assert_eq!(driver_entry(), expected, "{row}");
    }
}

#[test]
fn oracle_stub_rows() {
    for (row, name, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            unimplemented_stub(name),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }),
            "{row}"
        );
    }
}
