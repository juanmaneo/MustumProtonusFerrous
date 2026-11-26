//! Boundary and branch checks for the ported msauddecmft logic.
//!
//! Each test says where its expectations come from. "Not oracle-derived" marks expectations taken from the
//! C source or the Wine headers, or from the port's own design, rather than from a printed oracle row.

use super::*;

const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

#[test]
fn failed_is_a_sign_test() {
    // Not oracle-derived: FAILED(hr) is `(HRESULT)(hr) < 0`, from include/winerror.h.
    assert!(!failed(0));
    assert!(!failed(S_FALSE));
    assert!(!failed(i32::MAX));
    assert!(failed(-1));
    assert!(failed(i32::MIN));
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: IsEqualGUID compares all sixteen bytes, so one differing field is enough.
    let base = IID_CLASS_FACTORY;
    assert!(is_equal_guid(&base, &IID_CLASS_FACTORY));
    let variants = [
        Guid { data1: 2, ..base },
        Guid { data2: 1, ..base },
        Guid { data3: 1, ..base },
        Guid {
            data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x47],
            ..base
        },
    ];
    for other in &variants {
        assert!(!is_equal_guid(&base, other));
    }
}

#[test]
fn class_factory_refuses_near_miss_iids() {
    // Not oracle-derived: the QueryInterface test is an exact IID match. One changed field is refused, even
    // when the other IID is matched in every other field.
    let near_unknown = Guid {
        data3: 1,
        ..IID_UNKNOWN
    };
    assert_eq!(
        class_factory_query_interface(&near_unknown),
        Err(E_NOINTERFACE)
    );
    let near_factory = Guid {
        data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x47],
        ..IID_CLASS_FACTORY
    };
    assert_eq!(
        class_factory_query_interface(&near_factory),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn dll_get_class_object_checks_the_clsid_before_the_iid() {
    // Not oracle-derived: DllGetClassObject tests the CLSID first, so a wrong CLSID gives
    // CLASS_E_CLASSNOTAVAILABLE whatever the riid is.
    assert_eq!(
        dll_get_class_object(&GUID_NULL, &GUID_NULL),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    let near_clsid = Guid {
        data4: [0x88, 0x76, 0xDD, 0x77, 0x27, 0x3A, 0x89, 0x98],
        ..CLSID_MSAAC_DEC_MFT
    };
    assert_eq!(
        dll_get_class_object(&near_clsid, &IID_CLASS_FACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn create_instance_passes_the_co_result_through() {
    // Not oracle-derived: the port returns whatever CoCreateInstance returns, including negative values,
    // and passes the CLSID, the context and the riid through unchanged.
    let mut seen = None;
    let ret = aac_decoder_factory_create_instance(&IID_CLASS_FACTORY, |clsid, clsctx, riid| {
        seen = Some((*clsid, clsctx, *riid));
        i32::MIN
    });
    assert_eq!(ret, i32::MIN);
    assert_eq!(
        seen,
        Some((
            CLSID_WG_AAC_DECODER,
            CLSCTX_INPROC_SERVER,
            IID_CLASS_FACTORY
        ))
    );
}

#[test]
fn register_server_returns_the_first_failure_unchanged() {
    // Not oracle-derived: a failed step's HRESULT is returned as it is, and the steps after it do not run.
    // The closure is shared so that the skipped call is visible in the count.
    let mut mft_calls = 0;
    let mut mft = |_: &MftRegistration| {
        mft_calls += 1;
        S_OK
    };
    assert_eq!(dll_register_server(|| i32::MIN, &mut mft), i32::MIN);
    assert_eq!(dll_register_server(|| S_OK, &mut mft), S_OK);
    assert_eq!(mft_calls, 1);

    // Not oracle-derived: an MFTRegister failure is returned as it is, and MFTRegister gets the AAC decoder table.
    let mut name = "";
    let ret = dll_register_server(
        || S_OK,
        |reg| {
            name = reg.name;
            i32::MIN + 1
        },
    );
    assert_eq!(ret, i32::MIN + 1);
    assert_eq!(name, AAC_DECODER_REGISTRATION.name);
}

#[test]
fn unregister_server_returns_the_first_failure_unchanged() {
    // Not oracle-derived: the same first-failure rule as DllRegisterServer, with the CLSID for MFTUnregister.
    let mut clsids = Vec::new();
    let mut mft = |clsid: &Guid| {
        clsids.push(*clsid);
        i32::MIN + 1
    };
    assert_eq!(dll_unregister_server(|| i32::MIN, &mut mft), i32::MIN);
    assert_eq!(dll_unregister_server(|| S_OK, &mut mft), i32::MIN + 1);
    assert_eq!(clsids, [CLSID_MSAAC_DEC_MFT]);
}

#[test]
fn is_export_matches_the_spec_names_case_sensitively() {
    // Not oracle-derived: the four names are from msauddecmft.spec. The negative cases check spelling and case.
    for name in EXPORTS {
        assert!(is_export(name));
    }
    assert!(!is_export("dllgetclassobject"));
    assert!(!is_export("DllMain"));
    assert!(!is_export(""));
}
