//! Oracle-derived test vectors for the ported connect logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `IsInternetConnected` (`dlls/connect/main.c`), `DllCanUnloadNow`
//! (`libs/winecrt0/dll_canunload.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/connect/` (harness `oracle.c`). The run printed 15 rows, one per
//! export in `connect.spec`. ASan and UBSan reported nothing, so 0 vectors were
//! excluded. The 13 `stub` rows use the stub calls that winebuild generates for
//! `@ stub` entries; the harness reproduces those calls with the module string
//! `connect`.

use super::*;

#[test]
fn oracle_is_internet_connected_row() {
    // export=IsInternetConnected returns=0x00000000
    assert_eq!(is_internet_connected(), 0x0000_0000);
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // export=DllCanUnloadNow returns=0x00000001
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
}

#[test]
fn oracle_stub_rows() {
    // export=<name> code=0x80000100 flags=0x01 params=2 module=connect function=<name>
    // params=2 is implied by StubFault carrying exactly two ExceptionInformation values.
    let names = [
        "AddConnectionOptionListEntries",
        "CreateVPNConnection",
        "DllGetClassObject",
        "GetInternetConnected",
        "GetNetworkConnected",
        "GetVPNConnected",
        "HrIsInternetConnected",
        "HrIsInternetConnectedGUID",
        "IsInternetConnectedGUID",
        "IsUniqueConnectionName",
        "RegisterPageWithPage",
        "UnregisterPage",
        "UnregisterPagesLink",
    ];
    assert_eq!(STUB_EXPORTS, names);
    for name in names {
        let fault = stub_fault(name).expect("stub export");
        assert_eq!(fault.code, 0x8000_0100);
        assert_eq!(fault.flags, 0x01);
        assert_eq!(fault.module, "connect");
        assert_eq!(fault.function, name);
    }
}
