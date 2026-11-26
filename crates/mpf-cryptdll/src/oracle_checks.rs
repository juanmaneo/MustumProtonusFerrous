//! Oracle-derived test vectors for the ported cryptdll export table and stub model.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/cryptdll/`, and running the harness on `dlls/cryptdll/cryptdll.spec` and
//! `dlls/cryptdll/Makefile.in`. The harness printed 15 rows with no ASan or UBSan reports, so none
//! are excluded. Rows 01-11 are `@ stub` exports, rows 12-14 are the `@ stdcall` forwards read from
//! the spec text (cryptdll has no C body for them), and row 15 is the `MODULE` value.

use super::*;

#[test]
fn oracle_stub_rows_01_to_11() {
    // row 01 CDBuildIntegrityVect -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDBuildIntegrityVect
    assert_eq!(
        cd_build_integrity_vect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDBuildIntegrityVect",
        }
    );

    // row 02 CDBuildVect -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDBuildVect
    assert_eq!(
        cd_build_vect(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDBuildVect",
        }
    );

    // row 03 CDFindCommonCSystem -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDFindCommonCSystem
    assert_eq!(
        cd_find_common_csystem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDFindCommonCSystem",
        }
    );

    // row 04 CDFindCommonCSystemWithKey -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDFindCommonCSystemWithKey
    assert_eq!(
        cd_find_common_csystem_with_key(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDFindCommonCSystemWithKey",
        }
    );

    // row 05 CDGenerateRandomBits -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDGenerateRandomBits
    assert_eq!(
        cd_generate_random_bits(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDGenerateRandomBits",
        }
    );

    // row 06 CDLocateCSystem -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDLocateCSystem
    assert_eq!(
        cd_locate_csystem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDLocateCSystem",
        }
    );

    // row 07 CDLocateCheckSum -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDLocateCheckSum
    assert_eq!(
        cd_locate_check_sum(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDLocateCheckSum",
        }
    );

    // row 08 CDLocateRng -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDLocateRng
    assert_eq!(
        cd_locate_rng(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDLocateRng",
        }
    );

    // row 09 CDRegisterCSystem -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDRegisterCSystem
    assert_eq!(
        cd_register_csystem(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDRegisterCSystem",
        }
    );

    // row 10 CDRegisterCheckSum -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDRegisterCheckSum
    assert_eq!(
        cd_register_check_sum(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDRegisterCheckSum",
        }
    );

    // row 11 CDRegisterRng -> raise code=0x80000100 flags=0x1 nargs=2 p0=cryptdll.dll p1=CDRegisterRng
    assert_eq!(
        cd_register_rng(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "cryptdll.dll",
            function: "CDRegisterRng",
        }
    );
}

#[test]
fn oracle_forward_rows_12_to_14() {
    // row 12 MD5Final -> forward advapi32.MD5Final
    assert_eq!(
        export_kind("MD5Final"),
        Some(ExportKind::Forward("advapi32.MD5Final"))
    );

    // row 13 MD5Init -> forward advapi32.MD5Init
    assert_eq!(
        export_kind("MD5Init"),
        Some(ExportKind::Forward("advapi32.MD5Init"))
    );

    // row 14 MD5Update -> forward advapi32.MD5Update
    assert_eq!(
        export_kind("MD5Update"),
        Some(ExportKind::Forward("advapi32.MD5Update"))
    );
}

#[test]
fn oracle_module_row_15() {
    // row 15 MODULE -> cryptdll.dll
    assert_eq!(cd_locate_rng().module, "cryptdll.dll");
}
