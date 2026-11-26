//! Boundary and branch tests for the ported graphicscapture logic. Each expectation that does not
//! come from a captured oracle row is marked "Not oracle-derived".

use super::*;

const SESSION_CLASS: &str = "Windows.Graphics.Capture.GraphicsCaptureSession";

fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn ref_now(session: &mut Session) -> i32 {
    let n = session.add_ref();
    session.release();
    n.wrapping_sub(1) as i32
}

fn guid_text(g: &Guid) -> String {
    let tail: String = g.data4[2..].iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{tail}",
        g.data1, g.data2, g.data3, g.data4[0], g.data4[1]
    )
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: the HRESULT values are from include/winerror.h, S_FALSE is the
    // winecrt0 default, and the IID text is the uuid of each interface named in its doc comment.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(
        guid_text(&IID_UNKNOWN),
        "00000000-0000-0000-c000-000000000046"
    );
    assert_eq!(
        guid_text(&IID_INSPECTABLE),
        "af86e2e0-b12d-4c6a-9c5a-d7aa65101e90"
    );
    assert_eq!(
        guid_text(&IID_AGILE_OBJECT),
        "94ea2b94-e9cc-49e0-c0ff-ee64ca8f5b90"
    );
    assert_eq!(
        guid_text(&IID_ACTIVATION_FACTORY),
        "00000035-0000-0000-c000-000000000046"
    );
    assert_eq!(
        guid_text(&IID_GRAPHICS_CAPTURE_SESSION_STATICS),
        "2224a540-5974-49aa-b232-0882536f4cb5"
    );
}

#[test]
fn near_miss_iids_are_rejected_without_a_reference() {
    // Not oracle-derived: the oracle passed the five real IIDs and one unknown IID. Each near miss
    // changes one field of a known IID, and the comparison must be exact.
    let near_misses = [
        Guid {
            data1: 0x0000_0036,
            ..IID_ACTIVATION_FACTORY
        },
        Guid {
            data2: 0x5975,
            ..IID_GRAPHICS_CAPTURE_SESSION_STATICS
        },
        Guid {
            data4: [0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x91],
            ..IID_INSPECTABLE
        },
        Guid {
            data1: 0x94ea_2b95,
            ..IID_AGILE_OBJECT
        },
        Guid {
            data4: [0x00; 8],
            ..IID_UNKNOWN
        },
    ];
    for iid in &near_misses {
        let mut session = Session::default();
        let mut out = Some(Interface::SessionStatics);
        assert_eq!(session.query_interface(iid, &mut out), E_NOINTERFACE);
        assert_eq!(out, None);
        assert_eq!(ref_now(&mut session), 0);
    }
}

#[test]
fn class_name_stops_at_its_first_nul() {
    // Not oracle-derived: oracle row 07 covers a NUL followed by "junk". This covers a NUL that
    // ends the name, and a NUL in the middle of it, which rejects the name.
    let mut session = Session::default();
    let mut factory = None;
    let trailing = units(&format!("{SESSION_CLASS}\0"));
    assert_eq!(
        dll_get_activation_factory(&mut session, &trailing, &mut factory),
        S_OK
    );
    assert_eq!(factory, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 1);

    let mut factory = Some(Interface::SessionStatics);
    let middle = units("Windows.Graphics.Capture.\0GraphicsCaptureSession");
    assert_eq!(
        dll_get_activation_factory(&mut session, &middle, &mut factory),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 1);
}

#[test]
fn slice_without_a_nul_is_compared_whole() {
    // Not oracle-derived: the port's rule for a slice with no terminator. Wine's HSTRINGs are
    // NUL-terminated, so this only fixes the port's behaviour at the end of the slice.
    let mut session = Session::default();
    let mut factory = None;
    assert_eq!(
        dll_get_activation_factory(&mut session, &units(SESSION_CLASS), &mut factory),
        S_OK
    );
    assert_eq!(factory, Some(Interface::ActivationFactory));
    assert_eq!(ref_now(&mut session), 1);

    let one_short = units(&SESSION_CLASS[..SESSION_CLASS.len() - 1]);
    assert_eq!(
        dll_get_activation_factory(&mut session, &one_short, &mut factory),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(ref_now(&mut session), 1);
}

#[test]
fn slice_holding_only_a_nul_is_an_empty_name() {
    // Not oracle-derived: a single NUL compares as the empty string, which is not the class name.
    let mut session = Session::default();
    let mut factory = Some(Interface::SessionStatics);
    assert_eq!(
        dll_get_activation_factory(&mut session, &[0], &mut factory),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(factory, None);
    assert_eq!(ref_now(&mut session), 0);
}

#[test]
fn release_below_zero_keeps_counting_down() {
    // Not oracle-derived: oracle row 02 covers one step below zero. Two more steps show the
    // unsigned view keeps counting down, and the next AddRef climbs back.
    let mut session = Session::default();
    assert_eq!(session.release(), 0xffff_ffff);
    assert_eq!(session.release(), 0xffff_fffe);
    assert_eq!(ref_now(&mut session), -2);
    assert_eq!(session.add_ref(), 0xffff_ffff);
    assert_eq!(ref_now(&mut session), -1);
}

#[test]
fn is_supported_keeps_a_false_flag_false() {
    // Not oracle-derived: oracle row 17 starts from TRUE. A FALSE start must also end FALSE.
    let mut result = false;
    assert_eq!(is_supported(&mut result), S_OK);
    assert!(!result);
}
