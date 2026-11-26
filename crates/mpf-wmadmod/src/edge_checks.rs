//! Edge-case tests for the ported wmadmod logic.
//!
//! These cover the `FAILED` boundary, the GUID comparison, the registration tables, the export
//! list, the `S_FALSE` rule in both step chains, and the clsid check in `DllGetClassObject`.
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment. The
//! short-circuit rule (a failing step skips the later steps) is checked by the oracle trace
//! assertions, which list the calls made.

use super::*;
use std::cell::Cell;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h and include/mfapi.h.
    assert_eq!(S_OK, 0x0000_0000);
    assert_eq!(S_FALSE, 0x0000_0001);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_FAIL, 0x8000_4005);
    assert_eq!(E_UNEXPECTED, 0x8000_FFFF);
    assert_eq!(E_ACCESSDENIED, 0x8007_0005);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(MFT_ENUM_FLAG_SYNCMFT, 1);
}

#[test]
fn failed_boundaries() {
    // Not oracle-derived: the FAILED macro is `(HRESULT)(hr) < 0`, so only the sign bit matters.
    assert!(!failed(0x0000_0000));
    assert!(!failed(S_FALSE));
    assert!(!failed(0x7FFF_FFFF));
    assert!(failed(0x8000_0000));
    assert!(failed(0x8000_0001));
    assert!(failed(E_FAIL));
    assert!(failed(0xFFFF_FFFF));
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: IsEqualGUID compares all 16 bytes, so changing any one field must
    // make the GUIDs differ.
    let base = CLSID_WMA_DEC_MEDIA_OBJECT;
    assert!(is_equal_guid(&base, &base));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data1: base.data1 ^ 1,
            ..base
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data2: base.data2 ^ 1,
            ..base
        }
    ));
    assert!(!is_equal_guid(
        &base,
        &Guid {
            data3: base.data3 ^ 1,
            ..base
        }
    ));
    let mut data4 = base.data4;
    data4[7] ^= 1;
    assert!(!is_equal_guid(&base, &Guid { data4, ..base }));
}

#[test]
fn registration_tables_match_the_headers() {
    // Not oracle-derived: the GUIDs and flags from include/mfapi.h, include/dmoreg.h,
    // include/mfidl.idl and include/uuids.h. The oracle rows print the same GUID names
    // through the register trace.
    assert_eq!(WMA_DECODER_MFT.clsid, CLSID_WMA_DEC_MEDIA_OBJECT);
    assert_eq!(WMA_DECODER_MFT.category, MFT_CATEGORY_AUDIO_DECODER);
    assert_eq!(WMA_DECODER_MFT.name, "WMAudio Decoder MFT");
    assert_eq!(WMA_DECODER_MFT.flags, MFT_ENUM_FLAG_SYNCMFT);
    assert_eq!(WMA_DECODER_DMO.name, "WMAudio Decoder DMO");
    assert_eq!(WMA_DECODER_DMO.clsid, CLSID_WMA_DEC_MEDIA_OBJECT);
    assert_eq!(WMA_DECODER_DMO.category, DMO_CATEGORY_AUDIO_DECODER);
    assert_eq!(WMA_DECODER_DMO.flags, 0);
    assert_eq!(WMA_DECODER_MFT.inputs, WMA_DECODER_DMO.inputs);
    assert_eq!(WMA_DECODER_MFT.outputs, WMA_DECODER_DMO.outputs);
    assert_eq!(WMA_DECODER_INPUTS.len(), 4);
    assert_eq!(WMA_DECODER_OUTPUTS.len(), 2);
    assert_eq!(WMA_DECODER_INPUTS[0].subtype, SUBTYPE_MSAUDIO1);
    assert_eq!(WMA_DECODER_INPUTS[1].subtype, SUBTYPE_WMAUDIO2);
    assert_eq!(WMA_DECODER_INPUTS[2].subtype, SUBTYPE_WMAUDIO3);
    assert_eq!(WMA_DECODER_INPUTS[3].subtype, SUBTYPE_WMAUDIO_LOSSLESS);
    assert_eq!(WMA_DECODER_OUTPUTS[0].subtype, SUBTYPE_PCM);
    assert_eq!(WMA_DECODER_OUTPUTS[1].subtype, SUBTYPE_IEEE_FLOAT);
    assert!(WMA_DECODER_INPUTS
        .iter()
        .chain(WMA_DECODER_OUTPUTS.iter())
        .all(|t| t.major == MEDIA_TYPE_AUDIO));
}

#[test]
fn export_list_matches_the_spec() {
    // Not oracle-derived: the names and order of dlls/wmadmod/wmadmod.spec.
    assert_eq!(
        EXPORTS,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer"
        ]
    );
    for name in EXPORTS {
        assert!(is_export(name));
    }
    // Not oracle-derived: names that the spec does not list, including a case variant.
    assert!(!is_export("DllMain"));
    assert!(!is_export("dllregisterserver"));
    assert!(!is_export(""));
}

#[test]
fn register_s_false_from_resources_still_continues() {
    // Not oracle-derived: the oracle covers S_FALSE for the MFT step only (row 18). This pins
    // the same rule for the resource step: S_FALSE is not a failure.
    let calls = Cell::new(0);
    let hr = dll_register_server(
        || S_FALSE,
        |_| {
            calls.set(calls.get() + 1);
            S_OK
        },
        |_| {
            calls.set(calls.get() + 1);
            S_OK
        },
    );
    assert_eq!(hr, S_OK);
    assert_eq!(calls.get(), 2);
}

#[test]
fn unregister_s_false_from_resources_still_continues() {
    // Not oracle-derived: the oracle covers S_FALSE for the MFT step only (row 24). This pins
    // the same rule for the resource step of DllUnregisterServer.
    let calls = Cell::new(0);
    let hr = dll_unregister_server(
        || S_FALSE,
        |_| {
            calls.set(calls.get() + 1);
            S_OK
        },
        |_, _| {
            calls.set(calls.get() + 1);
            S_OK
        },
    );
    assert_eq!(hr, S_OK);
    assert_eq!(calls.get(), 2);
}

#[test]
fn class_object_clsid_is_checked_before_riid() {
    // Derived from oracle rows 10 to 13: with a non-matching clsid, the riid is never used,
    // so even IClassFactory returns CLASS_E_CLASSNOTAVAILABLE.
    let other = Guid {
        data1: 0,
        ..CLSID_WMA_DEC_MEDIA_OBJECT
    };
    assert_eq!(
        dll_get_class_object(&other, &IID_ICLASS_FACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        dll_get_class_object(&CLSID_WMA_DEC_MEDIA_OBJECT, &IID_IUNKNOWN),
        Ok(())
    );
}
