//! Oracle-derived test vectors for the ported mssip32 stub records.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/mssip32/`, and running the harness. The harness calls the stub once per `@ stub`
//! export of `dlls/mssip32/mssip32.spec`, with the module `mssip32.dll` from `Makefile.in`. All 7
//! rows ran without an ASan or UBSan report, so none is excluded.

use super::*;

#[test]
fn oracle_stub_rows() {
    // row 01 in=mssip32.dll,CryptSIPGetInfo out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPGetInfo
    assert_eq!(
        crypt_sip_get_info(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPGetInfo",
        }
    );

    // row 02 in=mssip32.dll,CryptSIPGetRegWorkingFlags out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPGetRegWorkingFlags
    assert_eq!(
        crypt_sip_get_reg_working_flags(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPGetRegWorkingFlags",
        }
    );

    // row 03 in=mssip32.dll,CryptSIPCreateIndirectData out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPCreateIndirectData
    assert_eq!(
        crypt_sip_create_indirect_data(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPCreateIndirectData",
        }
    );

    // row 04 in=mssip32.dll,CryptSIPGetSignedDataMsg out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPGetSignedDataMsg
    assert_eq!(
        crypt_sip_get_signed_data_msg(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPGetSignedDataMsg",
        }
    );

    // row 05 in=mssip32.dll,CryptSIPPutSignedDataMsg out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPPutSignedDataMsg
    assert_eq!(
        crypt_sip_put_signed_data_msg(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPPutSignedDataMsg",
        }
    );

    // row 06 in=mssip32.dll,CryptSIPRemoveSignedDataMsg out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPRemoveSignedDataMsg
    assert_eq!(
        crypt_sip_remove_signed_data_msg(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPRemoveSignedDataMsg",
        }
    );

    // row 07 in=mssip32.dll,CryptSIPVerifyIndirectData out=raise code=0x80000100 flags=0x1 nargs=2 p0=mssip32.dll p1=CryptSIPVerifyIndirectData
    assert_eq!(
        crypt_sip_verify_indirect_data(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mssip32.dll",
            function: "CryptSIPVerifyIndirectData",
        }
    );
}
