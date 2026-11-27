//! Oracle-derived test vectors for the ported ieproxy logic.
//!
//! Three sources produced the 23 printed rows. None was flagged by ASan or UBSan, so none is
//! excluded.
//! - Rows 01 to 06 come from `target/canon/ieproxy/wrappers.c`. It includes the `WINE_REGISTER_DLL`
//!   macro bodies of `include/rpcproxy.h`, copied verbatim, and records the calls to its callees.
//!   It was built with `gcc -fsanitize=address,undefined -g`.
//! - Rows 07 to 19 come from `winebuild --def` on `dlls/ieproxy/ieproxy.spec`, with `-m32` and
//!   `-m64`. The winebuild binary was built with the same sanitizer flags from the verbatim
//!   `tools/winebuild` sources. The `-m64` output has the same ordinals and no `@N` suffix.
//! - Rows 20 to 23 come from the same winebuild on four synthetic specs in
//!   `target/canon/ieproxy/winebuild/synth/`. They check the ordinal rule that `assign_ordinals`
//!   ports. They are not part of the DLL.
//!
//! LeakSanitizer reports leaks inside winebuild at exit. No row depends on them, and the
//! winebuild runs set `ASAN_OPTIONS=detect_leaks=0`.

use super::*;

fn entry(name: &'static str, ordinal: Option<u32>) -> Export {
    Export {
        name,
        ordinal,
        stdcall: true,
        private: false,
        params: &[],
        delegate: None,
    }
}

#[test]
fn oracle_wrapper_rows() {
    // row 01 export=GetProxyDllInfo callee=none stores=pfl=ok clsid=ok
    assert_eq!(find_export("GetProxyDllInfo").unwrap().delegate, None);

    // row 02 export=DllGetClassObject callee=NdrDllGetClassObject ret=0x00001001
    //        args=rclsid=ok riid=ok ppv=ok pfl=ok clsid=ok factory=ok
    // row 03 the same callee for a different rclsid, with the arguments forwarded unchanged.
    //        The row prints ret=0x00001001 for both calls. The port records only the callee.
    assert_eq!(
        find_export("DllGetClassObject").unwrap().delegate,
        Some("NdrDllGetClassObject")
    );

    // row 04 export=DllCanUnloadNow callee=NdrDllCanUnloadNow ret=0x00001002 args=factory=ok
    assert_eq!(
        find_export("DllCanUnloadNow").unwrap().delegate,
        Some("NdrDllCanUnloadNow")
    );

    // row 05 export=DllRegisterServer callee=__wine_register_resources ret=0x00001003
    assert_eq!(
        find_export("DllRegisterServer").unwrap().delegate,
        Some("__wine_register_resources")
    );

    // row 06 export=DllUnregisterServer callee=__wine_unregister_resources ret=0x00001004
    assert_eq!(
        find_export("DllUnregisterServer").unwrap().delegate,
        Some("__wine_unregister_resources")
    );
}

#[test]
fn oracle_ieproxy_spec_rows() {
    // rows 07 to 11: `winebuild --def -m32` lines `Name@N @ordinal PRIVATE`
    //   07 GetProxyDllInfo@8 @3 PRIVATE
    //   08 DllCanUnloadNow@0 @4 PRIVATE
    //   09 DllGetClassObject@12 @5 PRIVATE
    //   10 DllRegisterServer@0 @6 PRIVATE
    //   11 DllUnregisterServer@0 @7 PRIVATE
    for (name, ordinal, suffix) in [
        ("GetProxyDllInfo", 3, 8),
        ("DllCanUnloadNow", 4, 0),
        ("DllGetClassObject", 5, 12),
        ("DllRegisterServer", 6, 0),
        ("DllUnregisterServer", 7, 0),
    ] {
        let export = find_export(name).unwrap();
        assert_eq!(export_ordinal(name), Some(ordinal), "{name}");
        assert_eq!(i386_stdcall_bytes(&export), Some(suffix), "{name}");
        assert!(export.private, "{name}");
    }

    // rows 12 to 16: the same lines read back by ordinal
    for (ordinal, name) in [
        (3, "GetProxyDllInfo"),
        (4, "DllCanUnloadNow"),
        (5, "DllGetClassObject"),
        (6, "DllRegisterServer"),
        (7, "DllUnregisterServer"),
    ] {
        assert_eq!(export_name(ordinal), Some(name));
    }

    // row 17 export ordinal 8: no `.def` line has it
    assert_eq!(export_name(8), None);

    // row 18 name getproxydllinfo: `.def` names are case-sensitive, so there is no line for it
    assert_eq!(export_ordinal("getproxydllinfo"), None);

    // row 19 `LIBRARY ieproxy.dll`
    assert_eq!(MODULE, "ieproxy.dll");
}

#[test]
fn oracle_assign_ordinals_rows() {
    // row 20 synthetic s1.spec: `a@0 @6`, `b@0 @7`, `c@0 @5`, `d@0 @8`
    assert_eq!(
        assign_ordinals(&[
            entry("a", None),
            entry("b", None),
            entry("c", Some(5)),
            entry("d", None),
        ]),
        [6, 7, 5, 8]
    );

    // row 21 synthetic s2.spec: `x@0 @1`, `y@4 @2`
    assert_eq!(
        assign_ordinals(&[entry("x", None), entry("y", None)]),
        [1, 2]
    );

    // row 22 synthetic s3.spec: `z@0 @1`, `w@0 @2`
    assert_eq!(
        assign_ordinals(&[entry("z", Some(1)), entry("w", None)]),
        [1, 2]
    );

    // row 23 synthetic s4.spec: `p@0 @4`, `q@0 @3`, `r@0 @2`, `s@0 @5`
    assert_eq!(
        assign_ordinals(&[
            entry("p", Some(4)),
            entry("q", None),
            entry("r", Some(2)),
            entry("s", None),
        ]),
        [4, 3, 2, 5]
    );
}
