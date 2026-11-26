//! Oracle-derived test vectors for the ported chakra logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/chakra/chakra.c`, `libs/winecrt0/dll_canunload.c`, `libs/winecrt0/stub.c`
//! and `libs/winecrt0/register.c` (lines 52, 56-61 and 119-141) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/chakra/`, then running
//! the harness. It printed 21 rows: G01 to G05, U01 to U02, R01, X01 and S01 to S12.
//! ASan and UBSan flagged none of them, so none are excluded.
//!
//! The stub rows call the `winebuild` wrapper shape (`import.c`), and the harness's
//! `RaiseException` shim longjmps out, because the real call never returns. The
//! `nargs=2` field of each row is fixed by the `StubRaise` construction, so it is not checked here.
//!
//! The C `DllGetClassObject` takes `clsid` and `riid`, and the Rust port does not. So
//! G02 and G03 map to the same call, which only shows that those arguments do not matter.

use super::*;

#[test]
fn oracle_dll_get_class_object_rows() {
    // G01: out set to 0. Returns CLASS_E_CLASSNOTAVAILABLE and leaves the slot at 0.
    let mut slot: usize = 0;
    assert_eq!(dll_get_class_object(Some(&mut slot)), 0x8004_0111u32 as i32);
    assert_eq!(slot, 0);
    // G02: out null, clsid and riid null.
    assert_eq!(dll_get_class_object::<usize>(None), 0x8004_0111u32 as i32);
    // G03: out null, clsid and riid set.
    assert_eq!(dll_get_class_object::<usize>(None), 0x8004_0111u32 as i32);
    // G04: out preset to 0x1234. The slot must stay 0x1234.
    let mut slot: usize = 0x1234;
    assert_eq!(dll_get_class_object(Some(&mut slot)), 0x8004_0111u32 as i32);
    assert_eq!(slot, 0x1234);
    // G05: clsid null, riid set, out set to 0.
    let mut slot: usize = 0;
    assert_eq!(dll_get_class_object(Some(&mut slot)), 0x8004_0111u32 as i32);
    assert_eq!(slot, 0);
}

#[test]
fn oracle_dll_can_unload_now_rows() {
    // U01 and U02: both return S_FALSE (0x00000001).
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
}

#[test]
fn oracle_dll_register_unregister_rows() {
    // R01: DllRegisterServer with no WINE_REGISTRY resources returns S_OK (0x00000000).
    assert_eq!(dll_register_server(), 0x0000_0000);
    // X01: DllUnregisterServer, same result.
    assert_eq!(dll_unregister_server(), 0x0000_0000);
}

#[test]
fn oracle_stub_rows() {
    // Each row: (row id, export name). The oracle gave code 0x80000100, flags 0x01,
    // module "chakra.dll" and function equal to the export name for every row.
    let rows = [
        ("S01", "JsProjectWinRTNamespace"),
        ("S02", "JsAddRef"),
        ("S03", "JsRunScript"),
        ("S04", "CreateChakraEngine"),
        ("S05", "MemProtectHeapCreate"),
        ("S06", "RecyclerNativeHeapAllocLeaf"),
        ("S07", "JsCreateRuntime"),
        ("S08", "JsVariantToValue"),
        ("S09", "JsSetPromiseContinuationCallback"),
        ("S10", "DumpDiagInfo"),
        ("S11", "RecyclerNativeHeapRootRelease"),
        ("S12", "JsParseScriptWithAttributes"),
    ];
    for (id, export) in rows {
        assert!(is_stub_export(export), "{id}");
        assert_eq!(
            unimplemented_stub("chakra.dll", export),
            StubRaise {
                code: 0x8000_0100,
                flags: 0x01,
                module: "chakra.dll",
                function: export,
            },
            "{id}"
        );
    }
}
