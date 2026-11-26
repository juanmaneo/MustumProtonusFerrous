//! Oracle-derived test vectors for the ported graphicscapture logic.
//!
//! Vectors were captured by compiling the verbatim Wine bodies of `DllGetActivationFactory`
//! (`dlls/graphicscapture/main.c`), the session object (`dlls/graphicscapture/session.c`) and the
//! default `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/graphicscapture/`, and running the
//! harness there. All 25 printed rows ran clean under ASan and UBSan, so none are excluded.

use super::*;

const SESSION_CLASS: &str = "Windows.Graphics.Capture.GraphicsCaptureSession";

fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// The reference count as the harness reads it: `AddRef` returns the new count, so the count
/// before it is one less. The `Release` that follows leaves the count unchanged.
fn ref_now(session: &mut Session) -> i32 {
    let n = session.add_ref();
    session.release();
    n.wrapping_sub(1) as i32
}

#[test]
fn oracle_graphicscapture_rows() {
    let mut session = Session::default();
    let mut factory = None;
    let mut out;
    let mut result;

    // row 01 DllCanUnloadNow ret=0x00000001 out=- ref=0
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
    assert_eq!(ref_now(&mut session), 0);

    // row 02 Release_fresh ret=0xffffffff out=- ref=-1
    assert_eq!(session.release(), 0xffff_ffff);
    assert_eq!(ref_now(&mut session), -1);

    // row 03 AddRef ret=0x00000000 out=- ref=0
    assert_eq!(session.add_ref(), 0x0000_0000);
    assert_eq!(ref_now(&mut session), 0);

    // row 04 DllGetActivationFactory_session ret=0x00000000 out=factory ref=1
    assert_eq!(
        dll_get_activation_factory(&mut session, &units(SESSION_CLASS), &mut factory),
        0x0000_0000
    );
    assert_eq!(factory, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 1);

    // row 05 DllGetActivationFactory_session_long ret=0x80040111 out=NULL ref=1
    let long = units("Windows.Graphics.Capture.GraphicsCaptureSessionX");
    assert_eq!(
        dll_get_activation_factory(&mut session, &long, &mut factory),
        0x8004_0111
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 1);

    // row 06 DllGetActivationFactory_null ret=0x80040111 out=NULL ref=1
    assert_eq!(
        dll_get_activation_factory(&mut session, &[], &mut factory),
        0x8004_0111
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 1);

    // row 07 DllGetActivationFactory_session_junk ret=0x00000000 out=factory ref=2
    let junk = units("Windows.Graphics.Capture.GraphicsCaptureSession\0junk");
    assert_eq!(
        dll_get_activation_factory(&mut session, &junk, &mut factory),
        0x0000_0000
    );
    assert_eq!(factory, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 2);

    // row 08 DllGetActivationFactory_lower ret=0x80040111 out=NULL ref=2
    let lower = units("windows.graphics.capture.graphicscapturesession");
    assert_eq!(
        dll_get_activation_factory(&mut session, &lower, &mut factory),
        0x8004_0111
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 2);

    // row 09 DllGetActivationFactory_session_short ret=0x80040111 out=NULL ref=2
    let short = units("Windows.Graphics.Capture.GraphicsCaptureSessio");
    assert_eq!(
        dll_get_activation_factory(&mut session, &short, &mut factory),
        0x8004_0111
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 2);

    // row 10 DllGetActivationFactory_other ret=0x80040111 out=NULL ref=2
    let other = units("Windows.Graphics.Capture.Other");
    assert_eq!(
        dll_get_activation_factory(&mut session, &other, &mut factory),
        0x8004_0111
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 2);

    // row 11 QueryInterface_IUnknown ret=0x00000000 out=factory ref=3
    out = Some(Interface::SessionStatics);
    assert_eq!(session.query_interface(&IID_UNKNOWN, &mut out), 0x0000_0000);
    assert_eq!(out, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 3);

    // row 12 QueryInterface_IInspectable ret=0x00000000 out=factory ref=4
    out = Some(Interface::SessionStatics);
    assert_eq!(
        session.query_interface(&IID_INSPECTABLE, &mut out),
        0x0000_0000
    );
    assert_eq!(out, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 4);

    // row 13 QueryInterface_IAgileObject ret=0x00000000 out=factory ref=5
    out = Some(Interface::SessionStatics);
    assert_eq!(
        session.query_interface(&IID_AGILE_OBJECT, &mut out),
        0x0000_0000
    );
    assert_eq!(out, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 5);

    // row 14 QueryInterface_IActivationFactory ret=0x00000000 out=factory ref=6
    out = Some(Interface::SessionStatics);
    assert_eq!(
        session.query_interface(&IID_ACTIVATION_FACTORY, &mut out),
        0x0000_0000
    );
    assert_eq!(out, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 6);

    // row 15 QueryInterface_IGraphicsCaptureSessionStatics ret=0x00000000 out=statics ref=7
    out = None;
    assert_eq!(
        session.query_interface(&IID_GRAPHICS_CAPTURE_SESSION_STATICS, &mut out),
        0x0000_0000
    );
    assert_eq!(out, Some(Interface::SessionStatics));
    assert_eq!(ref_now(&mut session), 7);

    // row 16 QueryInterface_bogus ret=0x80004002 out=NULL ref=7
    let bogus = Guid {
        data1: 0x0000_0000,
        data2: 0x0000,
        data3: 0x0000,
        data4: [0, 0, 0, 0, 0, 0, 0, 1],
    };
    out = Some(Interface::SessionStatics);
    assert_eq!(session.query_interface(&bogus, &mut out), 0x8000_4002);
    assert_eq!(out, None);
    assert_eq!(ref_now(&mut session), 7);

    // row 17 IsSupported ret=0x00000000 result=0 ref=7
    result = true;
    assert_eq!(is_supported(&mut result), 0x0000_0000);
    assert!(!result);
    assert_eq!(ref_now(&mut session), 7);

    // row 18 factory_GetIids ret=0x80004001 out=- ref=7
    assert_eq!(session.get_iids(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 19 factory_GetRuntimeClassName ret=0x80004001 out=- ref=7
    assert_eq!(session.get_runtime_class_name(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 20 factory_GetTrustLevel ret=0x80004001 out=- ref=7
    assert_eq!(session.get_trust_level(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 21 factory_ActivateInstance ret=0x80004001 out=- ref=7
    assert_eq!(session.activate_instance(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 22 statics_GetIids ret=0x80004001 out=- ref=7
    assert_eq!(session.get_iids(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 23 statics_GetRuntimeClassName ret=0x80004001 out=- ref=7
    assert_eq!(session.get_runtime_class_name(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 24 statics_GetTrustLevel ret=0x80004001 out=- ref=7
    assert_eq!(session.get_trust_level(), 0x8000_4001);
    assert_eq!(ref_now(&mut session), 7);

    // row 25 statics_AddRef ret=0x00000008 out=- ref=8
    assert_eq!(session.add_ref(), 0x0000_0008);
    assert_eq!(ref_now(&mut session), 8);
}
