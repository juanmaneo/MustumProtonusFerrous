//! Edge-case tests for the ported windowscodecsext logic.
//!
//! Expectations marked "Not oracle-derived" come from the C source or from `include/winerror.h`
//! rather than from a harness row.

use super::*;

#[test]
fn severity_bit_boundaries() {
    // Not oracle-derived: SUCCEEDED and FAILED test the sign bit.
    assert!(succeeded(S_OK));
    assert!(succeeded(1));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(-1));
    assert!(!succeeded(i32::MIN));
    assert!(!failed(S_OK));
    assert!(!failed(1));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
}

#[test]
fn hresult_values_match_winerror_h() {
    // Not oracle-derived: values from Wine's include/winerror.h.
    assert_eq!(S_OK as u32, 0);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000E);
    assert_eq!(E_INVALIDARG as u32, 0x8007_0057);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(RPC_E_CHANGED_MODE as u32, 0x8001_0106);
}

/// The fake COM runtime, with the same log and the same out-parameter rule as the harness.
struct Host {
    init: HRESULT,
    create: HRESULT,
    ct: HRESULT,
    log: Vec<&'static str>,
}

impl ColorTransformHost<u32> for Host {
    fn co_initialize_ex(&mut self) -> HRESULT {
        self.log.push("CoInitializeEx");
        self.init
    }

    fn co_create_imaging_factory(&mut self) -> HRESULT {
        self.log.push("CoCreateInstance");
        self.create
    }

    fn create_color_transformer(&mut self, out: &mut u32) -> HRESULT {
        self.log.push("CreateColorTransformer");
        if succeeded(self.ct) {
            *out = 1;
        }
        self.ct
    }

    fn release_imaging_factory(&mut self) {
        self.log.push("Release");
    }

    fn co_uninitialize(&mut self) {
        self.log.push("CoUninitialize");
    }
}

#[test]
fn failed_init_skips_uninitialize_on_both_paths() {
    // Not oracle-derived: i32::MIN is a FAILED init, so CoUninitialize is skipped whether the
    // factory fails (the error path tests `SUCCEEDED(init)`) or succeeds.
    let mut host = Host {
        init: i32::MIN,
        create: E_FAIL,
        ct: S_OK,
        log: Vec::new(),
    };
    let mut out = 0;
    assert_eq!(
        wic_create_color_transform_proxy(&mut host, Some(&mut out)),
        E_FAIL
    );
    assert_eq!(host.log.join(","), "CoInitializeEx,CoCreateInstance");

    let mut host = Host {
        init: i32::MIN,
        create: S_OK,
        ct: S_OK,
        log: Vec::new(),
    };
    let mut out = 0;
    assert_eq!(
        wic_create_color_transform_proxy(&mut host, Some(&mut out)),
        S_OK
    );
    assert_eq!(out, 1);
    assert_eq!(
        host.log.join(","),
        "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release"
    );
}

#[test]
fn positive_success_hr_proceeds_to_transformer() {
    // Not oracle-derived: a positive success HRESULT from CoCreateInstance (i32::MAX) is not a
    // failure, so the transformer is still requested, and its failure becomes the result.
    let mut host = Host {
        init: S_OK,
        create: i32::MAX,
        ct: E_NOINTERFACE,
        log: Vec::new(),
    };
    let mut out = 0;
    assert_eq!(
        wic_create_color_transform_proxy(&mut host, Some(&mut out)),
        E_NOINTERFACE
    );
    assert_eq!(out, 0);
    assert_eq!(
        host.log.join(","),
        "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release,CoUninitialize"
    );
}

#[test]
fn initialize_forwards_boundary_arguments_unchanged() {
    // Not oracle-derived: the forwarder is generic over the pointer type, so the extreme values
    // of usize pass through unchanged, and so does a failing vtable result.
    struct Vtbl(Vec<(usize, usize, usize, usize)>);
    impl ColorTransformVtable<usize> for Vtbl {
        fn initialize(&mut self, a: usize, b: usize, c: usize, d: usize) -> HRESULT {
            self.0.push((a, b, c, d));
            E_NOTIMPL
        }
    }
    let mut vtbl = Vtbl(Vec::new());
    assert_eq!(
        initialize_proxy_w(&mut vtbl, usize::MAX, 0, usize::MAX, 0),
        E_NOTIMPL
    );
    assert_eq!(vtbl.0, [(usize::MAX, 0, usize::MAX, 0)]);
}

#[test]
fn export_table_matches_spec_entries() {
    // Not oracle-derived: the entries and flags of windowscodecsext.spec, in spec order.
    assert_eq!(EXPORTS.len(), 3);
    assert_eq!(EXPORTS[0], ("DllGetClassObject", ExportKind::Private));
    assert_eq!(
        EXPORTS[1],
        ("IWICColorTransform_Initialize_Proxy", ExportKind::Alias)
    );
    assert_eq!(
        EXPORTS[2],
        ("WICCreateColorTransform_Proxy", ExportKind::Stdcall)
    );
    for (name, kind) in EXPORTS {
        assert_eq!(export_kind(name), Some(kind));
    }
}

#[test]
fn export_kind_is_exact_and_case_sensitive() {
    // Not oracle-derived: lookups are exact. The `_W` implementation name is not an export.
    assert_eq!(export_kind("dllgetclassobject"), None);
    assert_eq!(export_kind("DllGetClassObject "), None);
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("IWICColorTransform_Initialize_Proxy_W"), None);
    assert_eq!(
        format!(
            "{:?} {:?} {:?}",
            ExportKind::Stdcall,
            ExportKind::Alias,
            ExportKind::Private
        ),
        "Stdcall Alias Private"
    );
}
