//! Oracle-derived test vectors for the ported davclnt logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `dlls/davclnt/main.c`, `libs/winecrt0/dll_main.c`,
//! `libs/winecrt0/dll_canunload.c` and `libs/winecrt0/stub.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/davclnt/`. The
//! harness calls each export and prints one `ROW` per call. The 25 printed rows
//! were all kept: neither sanitizer reported anything, so no vector is excluded.
//! The stub rows print `nargs=2` for every stub; the port always carries two
//! strings, so that field is not modelled.

use super::*;

#[test]
fn oracle_register_auth_callback_rows() {
    // register_null_cb_version1 -> 0xdeadbeef
    assert_eq!(dav_register_auth_callback(0, 1), 0xdead_beef);
    // register_nonnull_cb_version0 -> 0xdeadbeef
    assert_eq!(dav_register_auth_callback(0x1000, 0), 0xdead_beef);
}

#[test]
fn oracle_unregister_auth_callback_row() {
    // unregister_handle_deadbeef -> void
    dav_unregister_auth_callback(0xdead_beef);
}

#[test]
fn oracle_dll_main_rows() {
    // dll_main_process_attach -> ret=1 disable_calls=1 inst_ok=1
    assert_eq!(
        dll_main(DLL_PROCESS_ATTACH),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: true
        }
    );
    // dll_main_process_detach -> ret=1 disable_calls=0
    assert_eq!(
        dll_main(0),
        DllMainOutcome {
            ret: true,
            disable_thread_library_calls: false
        }
    );
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // dll_can_unload_now -> 1
    assert_eq!(dll_can_unload_now(), 1);
}

#[test]
fn oracle_stub_export_rows() {
    // Each row is `stub_<name> -> code=0x80000100 flags=0x01 nargs=2 module=davclnt.dll function=<name>`.
    let rows = [
        "DavCancelConnectionsToServer",
        "DavFreeUsedDiskSpace",
        "DavGetDiskSpaceUsage",
        "DavGetTheLockOwnerOfTheFile",
        "DavInvalidateCache",
        "DllGetClassObject",
        "NPAddConnection3",
        "NPAddConnection",
        "NPCancelConnection",
        "NPCloseEnum",
        "NPEnumResource",
        "NPFormatNetworkName",
        "NPGetCaps",
        "NPGetConnection",
        "NPGetResourceInformation",
        "NPGetResourceParent",
        "NPGetUniversalName",
        "NPGetUser",
        "NPOpenEnum",
    ];
    assert_eq!(STUB_EXPORTS, rows);
    for name in rows {
        assert_eq!(
            export_stub(name),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                module: "davclnt.dll",
                function: name,
            }
        );
    }
}
