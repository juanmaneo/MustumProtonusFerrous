//! Oracle-derived test vectors for the ported dx8vb logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject` and the
//! class factory vtable from `dlls/dx8vb/main.c`, and of `__wine_spec_unimplemented_stub` from
//! `dlls/ntdll/exception.c`, with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/dx8vb/`. The harness printed 18 rows (R01-R18). None were flagged by ASan or
//! UBSan, so none are excluded.

use super::*;

const IID_OTHER: Guid = Guid {
    data1: 0x1DD9_E8DA,
    data2: 0x1C77,
    data3: 0x4D40,
    data4: [0xB0, 0xCF, 0x98, 0xFE, 0xFD, 0xFF, 0x95, 0x12],
};

const CLSID_OTHER: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x1234,
    data3: 0x5678,
    data4: [1, 2, 3, 4, 5, 6, 7, 8],
};

/// `(row id, rclsid, riid, alloc_ok, expected hr, expected ref count if *ppv is set)`.
type ClassObjectRow = (&'static str, Guid, Guid, bool, HResult, Option<u32>);

/// R01-R07: `DllGetClassObject` rows. `ref_count` is `Some` where the C code sets `*ppv`.
#[test]
fn dll_get_class_object_rows() {
    let cases: [ClassObjectRow; 7] = [
        ("R01", GUID_NULL, IID_CLASS_FACTORY, true, S_OK, Some(1)),
        ("R02", GUID_NULL, IID_UNKNOWN, true, S_OK, Some(1)),
        ("R03", GUID_NULL, IID_OTHER, true, E_NOINTERFACE, None),
        (
            "R04",
            CLSID_OTHER,
            IID_CLASS_FACTORY,
            true,
            CLASS_E_CLASSNOTAVAILABLE,
            None,
        ),
        (
            "R05",
            CLSID_OTHER,
            IID_UNKNOWN,
            true,
            CLASS_E_CLASSNOTAVAILABLE,
            None,
        ),
        ("R06", CLSID_OTHER, IID_OTHER, true, E_NOINTERFACE, None),
        (
            "R07",
            GUID_NULL,
            IID_CLASS_FACTORY,
            false,
            E_OUTOFMEMORY,
            None,
        ),
    ];
    for (id, clsid, riid, alloc_ok, hr, ref_count) in cases {
        match dll_get_class_object(&clsid, &riid, alloc_ok) {
            Ok(factory) => {
                assert_eq!(S_OK, hr, "{id}");
                assert_eq!(Some(factory.ref_count()), ref_count, "{id}");
            }
            Err(err) => {
                assert_eq!(err, hr, "{id}");
                assert_eq!(ref_count, None, "{id}");
            }
        }
    }
}

/// R08-R15: class factory method rows, driven in the same order as the harness.
#[test]
fn class_factory_rows() {
    let mut f = dll_get_class_object(&GUID_NULL, &IID_CLASS_FACTORY, true).unwrap();

    // R08: QueryInterface(IUnknown) on ref 1 -> S_OK, ref 2.
    assert_eq!(f.query_interface(&IID_UNKNOWN), Ok(()));
    assert_eq!(f.ref_count(), 2);
    f.release();

    // R09: QueryInterface(IClassFactory) on a fresh factory -> S_OK, ref 2.
    let mut f = dll_get_class_object(&GUID_NULL, &IID_CLASS_FACTORY, true).unwrap();
    assert_eq!(f.query_interface(&IID_CLASS_FACTORY), Ok(()));
    assert_eq!(f.ref_count(), 2);
    f.release();
    f.release();

    // R10: QueryInterface(other) -> E_NOINTERFACE, ref unchanged.
    let mut f = dll_get_class_object(&GUID_NULL, &IID_CLASS_FACTORY, true).unwrap();
    assert_eq!(f.query_interface(&IID_OTHER), Err(E_NOINTERFACE));
    assert_eq!(f.ref_count(), 1);

    // R11-R13: AddRef -> 2, Release -> 1 (not freed), Release -> 0 (freed).
    assert_eq!(f.add_ref(), 2);
    assert_eq!(f.release(), 1);
    assert_eq!(f.release(), 0);

    // R14-R15: LockServer returns S_OK for both values.
    let f = dll_get_class_object(&GUID_NULL, &IID_CLASS_FACTORY, true).unwrap();
    assert_eq!(lock_server(false), S_OK);
    assert_eq!(lock_server(true), S_OK);
    assert_eq!(f.ref_count(), 1);
}

/// R16-R18: stub rows. Each export raises `EXCEPTION_WINE_STUB` with flags 1 and two parameters.
#[test]
fn stub_rows() {
    for name in ["VB_D3DXVec2Dot", "D3DVertexBuffer8SetData", "DXCopyMemory"] {
        assert_eq!(
            unimplemented_stub(name),
            Some(StubException {
                code: 0x8000_0100,
                flags: 1,
                number_parameters: 2,
                module: "dx8vb.dll",
                function: name,
            })
        );
    }
}
