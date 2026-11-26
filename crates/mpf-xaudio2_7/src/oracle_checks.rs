//! Oracle-derived test vectors for the ported xaudio2_7 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`,
//! `XAudio2CF_CreateInstance` (up to its `pOuter` check), `xapocf_CreateInstance`,
//! `XAudio2CF_LockServer`, `xapocf_LockServer`, `XAPOFX_QueryInterface`,
//! `XAPOFX_CalcInputFrames`, `XAPOFX_CalcOutputFrames`, `XAPO_GetRegistrationProperties`,
//! `IXAudio2Impl_QueryInterface`, `IXAudio2Impl_RegisterForCallbacks`,
//! `IXAudio2Impl_UnregisterForCallbacks` and `IXAudio2Impl_CreateMasteringVoice` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/xaudio2_7/`, and running the harness.
//! The harness printed 38 rows. ASan and UBSan flagged none of them, so none is excluded.
//!
//! Notes on how the rows map to the port:
//! - A NULL callback is printed as `0` in the callback rows, and is `None` here.
//! - Row 38 passes a non-NULL effect chain. The port does not model the chain argument, so the
//!   row is checked with the same call as row 36 and 37.
//! - The `addref` counts and the `ppobj` out-pointer values are not modelled.

use super::*;

#[test]
fn dll_get_class_object_rows() {
    // row 01 fn=DllGetClassObject clsid=XAudio2 -> hr=00000000 factory=XAudio2
    assert!(class_object(&CLSID_XAUDIO2) == Ok(ClassObject::XAudio2));
    // row 02 fn=DllGetClassObject clsid=VolumeMeter -> hr=00000000 factory=XapoVolumeMeter
    assert!(
        class_object(&CLSID_AUDIO_VOLUME_METER) == Ok(ClassObject::Xapo(XapoClass::VolumeMeter))
    );
    // row 03 fn=DllGetClassObject clsid=Reverb -> hr=00000000 factory=XapoReverb
    assert!(class_object(&CLSID_AUDIO_REVERB) == Ok(ClassObject::Xapo(XapoClass::Reverb)));
    // row 04 fn=DllGetClassObject clsid=FXReverb -> hr=80040111 factory=none
    assert!(class_object(&CLSID_FX_REVERB) == Err(0x8004_0111));
}

#[test]
fn xapocf_create_instance_rows() {
    // row 05 fn=xapocf_CreateInstance clsid=VolumeMeter outer=0 -> hr=00000000 create=volume
    assert!(
        xapo_create_instance_class(&CLSID_AUDIO_VOLUME_METER, false) == Ok(XapoClass::VolumeMeter)
    );
    // row 06 fn=xapocf_CreateInstance clsid=Reverb outer=0 -> hr=00000000 create=reverb
    assert!(xapo_create_instance_class(&CLSID_AUDIO_REVERB, false) == Ok(XapoClass::Reverb));
    // row 07 fn=xapocf_CreateInstance clsid=FXReverb outer=0 -> hr=80070057 create=none
    assert!(xapo_create_instance_class(&CLSID_FX_REVERB, false) == Err(0x8007_0057));
    // row 08 fn=xapocf_CreateInstance clsid=Reverb outer=1 -> hr=80040110 create=none
    assert!(xapo_create_instance_class(&CLSID_AUDIO_REVERB, true) == Err(0x8004_0110));
}

#[test]
fn xaudio2cf_create_instance_rows() {
    // row 09 fn=XAudio2CF_CreateInstance outer=0 -> hr=00000000
    assert!(xaudio2_create_instance_gate(false) == Ok(()));
    // row 10 fn=XAudio2CF_CreateInstance outer=1 -> hr=80040110
    assert!(xaudio2_create_instance_gate(true) == Err(0x8004_0110));
}

#[test]
fn lock_server_rows() {
    // row 11 fn=xapocf_LockServer dolock=1 -> hr=00000000
    assert!(xapo_cf_lock_server(true) == 0);
    // row 12 fn=XAudio2CF_LockServer dolock=0 -> hr=00000000
    assert!(xaudio2_cf_lock_server(false) == 0);
}

#[test]
fn xapofx_query_interface_rows() {
    // row 13 fn=XAPOFX_QueryInterface riid=IUnknown -> hr=00000000 iface=IXAPO
    assert!(xapo_query_interface(&IID_IUNKNOWN) == Ok(XapoInterface::Xapo));
    // row 14 fn=XAPOFX_QueryInterface riid=IXAPO27 -> hr=00000000 iface=IXAPO
    assert!(xapo_query_interface(&IID_IXAPO27) == Ok(XapoInterface::Xapo));
    // row 15 fn=XAPOFX_QueryInterface riid=IXAPOParameters -> hr=00000000 iface=IXAPOParameters
    assert!(xapo_query_interface(&IID_IXAPO_PARAMETERS) == Ok(XapoInterface::Parameters));
    // row 16 fn=XAPOFX_QueryInterface riid=IXAudio2 -> hr=80004002 iface=NULL
    assert!(xapo_query_interface(&IID_IXAUDIO2) == Err(0x8000_4002));
    // row 17 fn=XAPOFX_QueryInterface riid=IXAPO -> hr=00000000 iface=IXAPO
    assert!(xapo_query_interface(&IID_IXAPO) == Ok(XapoInterface::Xapo));
    // row 18 fn=XAPOFX_QueryInterface riid=IXAPO27Parameters -> hr=00000000 iface=IXAPOParameters
    assert!(xapo_query_interface(&IID_IXAPO27_PARAMETERS) == Ok(XapoInterface::Parameters));
}

#[test]
fn xapofx_calc_frames_rows() {
    // row 19 fn=XAPOFX_CalcInputFrames output_frames=480 -> 0
    assert!(calc_input_frames(480) == 0);
    // row 20 fn=XAPOFX_CalcOutputFrames input_frames=480 -> 0
    assert!(calc_output_frames(480) == 0);
}

#[test]
fn xapo_get_registration_properties_rows() {
    // row 21 fn=XAPO_GetRegistrationProperties hr=S_OK -> ret=00000000
    assert!(get_registration_properties_status(S_OK) == 0);
    // row 22 fn=XAPO_GetRegistrationProperties hr=S_FALSE -> ret=00000000
    assert!(get_registration_properties_status(S_FALSE) == 0);
    // row 23 fn=XAPO_GetRegistrationProperties hr=E_NOTIMPL -> ret=80004001
    assert!(get_registration_properties_status(E_NOTIMPL) == 0x8000_4001);
}

#[test]
fn xaudio2_query_interface_rows() {
    // row 24 fn=IXAudio2Impl_QueryInterface riid=IXAudio2 -> hr=00000000 self=yes
    assert!(xaudio2_query_interface(&IID_IXAUDIO2) == Ok(()));
    // row 25 fn=IXAudio2Impl_QueryInterface riid=IXAPO27 -> hr=80004002 self=NULL
    assert!(xaudio2_query_interface(&IID_IXAPO27) == Err(0x8000_4002));
}

#[test]
fn register_unregister_callback_rows() {
    let mut e = Engine::default();
    // row 26 fn=IXAudio2Impl_RegisterForCallbacks cb=1 -> hr=00000000 ncbs=1 slots=[1]
    assert!(e.register_for_callbacks(Some(1)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1)]);
    // row 27 fn=IXAudio2Impl_RegisterForCallbacks cb=2 -> hr=00000000 ncbs=2 slots=[1,2]
    assert!(e.register_for_callbacks(Some(2)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), Some(2)]);
    // row 28 fn=IXAudio2Impl_RegisterForCallbacks cb=1 -> hr=00000000 ncbs=2 slots=[1,2]
    assert!(e.register_for_callbacks(Some(1)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), Some(2)]);
    // row 29 fn=IXAudio2Impl_RegisterForCallbacks cb=0 -> hr=00000000 ncbs=3 slots=[1,2,0]
    assert!(e.register_for_callbacks(None) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), Some(2), None]);
    // row 30 fn=IXAudio2Impl_RegisterForCallbacks cb=3 -> hr=00000000 ncbs=3 slots=[1,2,3]
    assert!(e.register_for_callbacks(Some(3)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(1), Some(2), Some(3)]);
    // row 31 fn=IXAudio2Impl_UnregisterForCallbacks cb=1 -> ncbs=3 slots=[2,3,0]
    e.unregister_for_callbacks(Some(1));
    assert_eq!(e.callbacks(), &[Some(2), Some(3), None]);
    // row 32 fn=IXAudio2Impl_UnregisterForCallbacks cb=9 -> ncbs=3 slots=[2,3,0]
    e.unregister_for_callbacks(Some(9));
    assert_eq!(e.callbacks(), &[Some(2), Some(3), None]);
    // row 33 fn=IXAudio2Impl_RegisterForCallbacks cb=4 -> hr=00000000 ncbs=3 slots=[2,3,4]
    assert!(e.register_for_callbacks(Some(4)) == S_OK);
    assert_eq!(e.callbacks(), &[Some(2), Some(3), Some(4)]);
    // row 34 fn=IXAudio2Impl_UnregisterForCallbacks cb=3 -> ncbs=3 slots=[2,4,0]
    e.unregister_for_callbacks(Some(3));
    assert_eq!(e.callbacks(), &[Some(2), Some(4), None]);
    // row 35 fn=IXAudio2Impl_UnregisterForCallbacks cb=4 -> ncbs=3 slots=[2,0,0]
    e.unregister_for_callbacks(Some(4));
    assert_eq!(e.callbacks(), &[Some(2), None, None]);
}

#[test]
fn create_mastering_voice_rows() {
    let mut e = Engine::default();
    // row 36 fn=IXAudio2Impl_CreateMasteringVoice chain=NULL -> hr=00000000 in_use=1
    assert!(e.create_mastering_voice() == S_OK);
    assert!(e.mastering_in_use());
    // row 37 fn=IXAudio2Impl_CreateMasteringVoice second chain=NULL -> hr=88960001 in_use=1
    assert!(e.create_mastering_voice() == 0x8896_0001);
    // row 38 fn=IXAudio2Impl_CreateMasteringVoice second chain=set -> hr=88960001 in_use=1
    assert!(e.create_mastering_voice() == 0x8896_0001);
    assert!(e.mastering_in_use());
}
