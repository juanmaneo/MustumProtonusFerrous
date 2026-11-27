//! Oracle-derived test vectors for the msctfp port.
//!
//! Rows 01-18 are the printed output of `target/canon/msctfp/harness.c`, built with
//! `gcc -fsanitize=address,undefined -g -DPROXY_CLSID=CLSID_PSFactoryBuffer -DWINE_REGISTER_DLL`.
//! That harness compiles the verbatim reference text: the proxy macros in `include/rpcproxy.h`,
//! and `__wine_register_resources` / `__wine_unregister_resources` from `libs/winecrt0/register.c`.
//! Rows 19-23 are `winebuild --def -m32` output for `dlls/msctfp/msctfp.spec` (winebuild 11.19;
//! the reference is 11.22). Row 24 is the CLSID that `widl -u` writes to `msctfp_i.c` from
//! `msctfp.idl`. ASan and UBSan flagged no row, so no row is excluded.

use super::*;

const S_FALSE: Hresult = 1;
const E_FAIL: Hresult = 0x8000_4005_u32 as Hresult;
const E_OUTOFMEMORY: Hresult = 0x8007_000e_u32 as Hresult;

/// The GUID text that the harness prints with `{%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x}`.
fn braced(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// Runs `dll_main` from a fresh state, as each harness row does.
fn run_dll_main(hinstance: usize, reason: u32) -> (bool, ProxyDll) {
    let mut state = ProxyDll::default();
    let ret = dll_main(&mut state, hinstance, reason);
    (ret, state)
}

#[test]
fn oracle_get_proxy_dll_info_row() {
    // row 01 GetProxyDllInfo -> clsid={b5f8fb3b-393f-4f7c-84cb-504924c2705a}
    assert_eq!(
        braced(get_proxy_dll_info()),
        "{b5f8fb3b-393f-4f7c-84cb-504924c2705a}"
    );
}

#[test]
fn oracle_dll_main_rows() {
    // row 02 DllMain hinst=0x1000 reason=1 -> ret=1 hProxyDll=0x1000 disable_calls=1
    let (ret, state) = run_dll_main(0x1000, DLL_PROCESS_ATTACH);
    assert!(ret);
    assert_eq!(
        state,
        ProxyDll {
            hinstance: 0x1000,
            thread_library_calls_disabled: true,
        }
    );

    // row 03 DllMain hinst=0x1000 reason=0 -> ret=1 hProxyDll=0 disable_calls=0
    let (ret, state) = run_dll_main(0x1000, 0);
    assert!(ret);
    assert_eq!(state, ProxyDll::default());

    // row 04 DllMain hinst=0x1000 reason=2 -> ret=1 hProxyDll=0 disable_calls=0
    let (ret, state) = run_dll_main(0x1000, 2);
    assert!(ret);
    assert_eq!(state, ProxyDll::default());

    // row 05 DllMain hinst=0x1000 reason=3 -> ret=1 hProxyDll=0 disable_calls=0
    let (ret, state) = run_dll_main(0x1000, 3);
    assert!(ret);
    assert_eq!(state, ProxyDll::default());

    // row 06 DllMain hinst=0 reason=1 -> ret=1 hProxyDll=0 disable_calls=1
    let (ret, state) = run_dll_main(0, DLL_PROCESS_ATTACH);
    assert!(ret);
    assert_eq!(
        state,
        ProxyDll {
            hinstance: 0,
            thread_library_calls_disabled: true,
        }
    );

    // row 07 DllMain hinst=0x1000 reason=5 -> ret=1 hProxyDll=0 disable_calls=0
    let (ret, state) = run_dll_main(0x1000, 5);
    assert!(ret);
    assert_eq!(state, ProxyDll::default());
}

#[test]
fn oracle_dll_register_server_rows() {
    use CallbackOutcome::{Set, Untouched};

    // row 08 DllRegisterServer outcomes=none -> 00000000
    assert_eq!(dll_register_server(), S_OK);
    // row 09 DllRegisterServer outcomes=set:00000000 -> 00000000
    assert_eq!(enumerate_registry_resources(&[Set(S_OK)]), S_OK);
    // row 10 DllRegisterServer outcomes=set:00000001 -> 00000001
    assert_eq!(enumerate_registry_resources(&[Set(S_FALSE)]), S_FALSE);
    // row 11 DllRegisterServer outcomes=set:80004005 -> 80004005
    assert_eq!(enumerate_registry_resources(&[Set(E_FAIL)]), E_FAIL);
    // row 12 DllRegisterServer outcomes=set:80004005,set:00000000 -> 80004005
    assert_eq!(
        enumerate_registry_resources(&[Set(E_FAIL), Set(S_OK)]),
        E_FAIL
    );
    // row 13 DllRegisterServer outcomes=set:00000000,set:8007000e -> 8007000e
    assert_eq!(
        enumerate_registry_resources(&[Set(S_OK), Set(E_OUTOFMEMORY)]),
        E_OUTOFMEMORY
    );
    // row 14 DllRegisterServer outcomes=untouched,set:80004005 -> 00000000
    assert_eq!(
        enumerate_registry_resources(&[Untouched, Set(E_FAIL)]),
        S_OK
    );
    // row 15 DllRegisterServer outcomes=set:00000000,untouched,set:80004005 -> 00000000
    assert_eq!(
        enumerate_registry_resources(&[Set(S_OK), Untouched, Set(E_FAIL)]),
        S_OK
    );
    // row 16 DllRegisterServer outcomes=untouched -> 00000000
    assert_eq!(enumerate_registry_resources(&[Untouched]), S_OK);
}

#[test]
fn oracle_dll_unregister_server_rows() {
    // row 17 DllUnregisterServer outcomes=none -> 00000000
    assert_eq!(dll_unregister_server(), S_OK);
    // row 18 DllUnregisterServer outcomes=set:80004005,set:00000000 -> 80004005
    assert_eq!(
        enumerate_registry_resources(&[CallbackOutcome::Set(E_FAIL), CallbackOutcome::Set(S_OK)]),
        E_FAIL
    );
}

#[test]
fn oracle_export_rows() {
    // Rows 19-23: `winebuild --def -m32` writes `name@bytes @ordinal PRIVATE`.
    // row 19 DllCanUnloadNow@0 @1 PRIVATE
    // row 20 DllGetClassObject@12 @2 PRIVATE
    // row 21 DllRegisterServer@0 @3 PRIVATE
    // row 22 DllUnregisterServer@0 @4 PRIVATE
    // row 23 GetProxyDllInfo@8 @5 PRIVATE
    let rows = [
        ("DllCanUnloadNow", 0, 1),
        ("DllGetClassObject", 12, 2),
        ("DllRegisterServer", 0, 3),
        ("DllUnregisterServer", 0, 4),
        ("GetProxyDllInfo", 8, 5),
    ];
    assert_eq!(EXPORTS.len(), rows.len());
    for (name, stack_bytes, ordinal) in rows {
        let export = find_export(name).unwrap();
        assert_eq!(export.i386_stack_bytes(), stack_bytes);
        assert_eq!(export.ordinal, ordinal);
    }
}

#[test]
fn oracle_clsid_row() {
    // row 24 widl -u msctfp.idl: MIDL_DEFINE_GUID(CLSID, CLSID_PSFactoryBuffer,
    //   0xb5f8fb3b, 0x393f, 0x4f7c, 0x84,0xcb, 0x50,0x49,0x24,0xc2,0x70,0x5a)
    assert_eq!(
        CLSID_PSFACTORYBUFFER,
        Guid {
            data1: 0xb5f8_fb3b,
            data2: 0x393f,
            data3: 0x4f7c,
            data4: [0x84, 0xcb, 0x50, 0x49, 0x24, 0xc2, 0x70, 0x5a],
        }
    );
}
