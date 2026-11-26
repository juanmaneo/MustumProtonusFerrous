//! Edge-case tests for the ported l3codecx.ax logic.
//!
//! These cover the HRESULT failure boundary, the early returns of the registration paths, the
//! arguments the registration passes to the host, the factory's `QueryInterface` and
//! `CreateInstance`, and the export table. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;
use crate::oracle_checks::{guid_text, register_with, unregister_with, Recorder};

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/uuids.h, include/axextend.idl and include/winerror.h
    // in the Wine tree. These values are not printed by the harness rows.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_FAIL, 0x8000_4005_u32 as Hresult);
    assert_eq!(E_NOINTERFACE, 0x8000_4002_u32 as Hresult);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111_u32 as Hresult);
    assert_eq!(REG_PINFLAG_B_OUTPUT, 0x8);
    assert_eq!(
        guid_text(&CLSID_FILTER_MAPPER2),
        "cda42200-bd88-11d0-bd4e-00a0c911ce86"
    );
    assert_eq!(
        guid_text(&IID_IFILTER_MAPPER2),
        "b79bb0b0-33c1-11d1-abe1-00a0c905f375"
    );
    assert_eq!(MPEG_LAYER3_FILTER_NAME, "MPEG Layer-3 Decoder");
}

#[test]
fn is_failed_tests_the_sign_bit() {
    // Not oracle-derived: FAILED(hr) is hr < 0, so negative values other than E_FAIL fail too.
    assert!(!is_failed(S_OK));
    assert!(!is_failed(S_FALSE));
    assert!(!is_failed(i32::MAX));
    assert!(is_failed(-1));
    assert!(is_failed(i32::MIN));
    assert!(is_failed(E_FAIL));
    assert!(is_failed(CLASS_E_CLASSNOTAVAILABLE));
}

#[test]
fn failed_resource_registration_returns_at_once() {
    // Not oracle-derived: any failing HRESULT stops the sequence, not only E_FAIL (row 14).
    for hr in [-1, i32::MIN] {
        let (got, host) = register_with(hr, S_OK, S_OK);
        assert_eq!(got, hr);
        assert_eq!(host.trace_text(), "RES");
        assert_eq!(host.last_cc, None);
    }
}

#[test]
fn failed_mapper_creation_is_returned_unchanged() {
    // Not oracle-derived: a failing CoCreateInstance returns its own HRESULT, and RegisterFilter is
    // not reached. Row 15 covers E_FAIL and row 18 covers REGDB_E_CLASSNOTREG.
    for hr in [-1, i32::MIN] {
        let (got, host) = register_with(S_OK, hr, S_OK);
        assert_eq!(got, hr);
        assert_eq!(host.trace_text(), "RES CC");
        assert_eq!(host.last_rf, None);
    }
}

#[test]
fn register_filter_result_is_returned_unchanged() {
    // Not oracle-derived: the RegisterFilter result is returned whatever its sign, and the mapper is
    // still released. Row 17 covers E_FAIL.
    for hr in [S_FALSE, -1, i32::MIN] {
        let (got, host) = register_with(S_OK, S_OK, hr);
        assert_eq!(got, hr);
        assert_eq!(host.trace_text(), "RES CC RF REL");
    }
}

#[test]
fn registration_creates_the_mapper_in_process() {
    // Not oracle-derived: the arguments of the CoCreateInstance call that the rows do not print.
    // The C passes CLSID_FilterMapper2, a NULL outer, CLSCTX_INPROC_SERVER and IID_IFilterMapper2.
    let (_, host) = register_with(S_OK, S_OK, S_OK);
    assert_eq!(
        host.last_cc,
        Some((CLSID_FILTER_MAPPER2, false, IID_IFILTER_MAPPER2))
    );
}

#[test]
fn registration_passes_the_decoder_clsid_name_and_descriptor() {
    // Not oracle-derived: the typed arguments of RegisterFilter. Row 23 checks the same call as text.
    let (_, host) = register_with(S_OK, S_OK, S_OK);
    let (clsid, name, desc) = host.last_rf.expect("RegisterFilter was called");
    assert_eq!(clsid, CLSID_MPEG_LAYER3_DECODER);
    assert_eq!(name, MPEG_LAYER3_FILTER_NAME);
    assert_eq!(desc, MPEG_LAYER3_DECODER_FILTER);
}

#[test]
fn failed_unregister_resources_returns_at_once() {
    // Not oracle-derived: any failing HRESULT stops the sequence, not only E_FAIL (row 19).
    for hr in [-1, i32::MIN] {
        let (got, host) = unregister_with(hr, S_OK, S_OK);
        assert_eq!(got, hr);
        assert_eq!(host.trace_text(), "UNRES");
        assert_eq!(host.last_uf, None);
    }
}

#[test]
fn failed_unregister_mapper_creation_is_returned_unchanged() {
    // Not oracle-derived: a failing CoCreateInstance returns its own HRESULT, and UnregisterFilter
    // is not reached. Row 20 covers E_FAIL.
    for hr in [-1, i32::MIN] {
        let (got, host) = unregister_with(S_OK, hr, S_OK);
        assert_eq!(got, hr);
        assert_eq!(host.trace_text(), "UNRES CC");
        assert_eq!(host.last_uf, None);
    }
}

#[test]
fn unregister_filter_result_is_ignored() {
    // Not oracle-derived: the bug-compatible S_OK once the mapper exists. Row 21 covers E_FAIL, and
    // these cover the sign boundaries.
    for hr in [S_FALSE, -1, i32::MIN] {
        let (got, host) = unregister_with(S_OK, S_OK, hr);
        assert_eq!(got, S_OK);
        assert_eq!(host.trace_text(), "UNRES CC UF REL");
    }
}

#[test]
fn unregistration_removes_the_decoder_by_clsid() {
    // Not oracle-derived: the arguments of UnregisterFilter and of the mapper's CoCreateInstance.
    let (hr, host) = unregister_with(S_OK, S_OK, S_OK);
    assert_eq!(hr, S_OK);
    assert_eq!(host.last_uf, Some(CLSID_MPEG_LAYER3_DECODER));
    assert_eq!(
        host.last_cc,
        Some((CLSID_FILTER_MAPPER2, false, IID_IFILTER_MAPPER2))
    );
}

#[test]
fn factory_query_interface_serves_only_iclassfactory_and_iunknown() {
    // Not oracle-derived: the QueryInterface branches for an IID that is not served.
    for iid in [IID_ICLASS_FACTORY, IID_IUNKNOWN] {
        let (hr, out) = MPEG_LAYER3_DECODER_FACTORY.query_interface(&iid);
        assert_eq!(hr, S_OK);
        assert!(out.is_some());
    }
    let (hr, out) = MPEG_LAYER3_DECODER_FACTORY.query_interface(&CLSID_MPEG_LAYER3_DECODER);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(out.is_none());
}

#[test]
fn create_instance_returns_the_host_result_unchanged() {
    // Not oracle-derived: CreateInstance returns CoCreateInstance's result as it is. Row 13 covers
    // REGDB_E_CLASSNOTREG, and this covers a different failure.
    let mut host = Recorder {
        cc: E_FAIL,
        ..Recorder::default()
    };
    let hr = MPEG_LAYER3_DECODER_FACTORY.create_instance(&mut host, false, &IID_IUNKNOWN);
    assert_eq!(hr, E_FAIL);
    assert_eq!(
        host.last_cc,
        Some((CLSID_WG_MP3_DECODER, false, IID_IUNKNOWN))
    );
}

#[test]
fn dll_get_class_object_refuses_other_classes() {
    // Not oracle-derived: CLSID_FilterMapper2 is a class this DLL does not serve. Rows 04, 05 and 07
    // cover the other refusals.
    let (hr, out) = dll_get_class_object(&CLSID_FILTER_MAPPER2, &IID_ICLASS_FACTORY);
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
    assert!(out.is_none());
}

#[test]
fn export_table_matches_the_spec_file() {
    // Not oracle-derived: lines 1 to 3 of dlls/l3codecx.ax/l3codecx.ax.spec.
    assert_eq!(EXPORTS.len(), 3);
    assert_eq!(export_signature("DllGetClassObject"), Some("ptr ptr ptr"));
    assert_eq!(export_signature("DllRegisterServer"), Some(""));
    assert_eq!(export_signature("DllUnregisterServer"), Some(""));
    assert_eq!(export_signature("DllCanUnloadNow"), None);
    assert_eq!(export_signature(""), None);
}
