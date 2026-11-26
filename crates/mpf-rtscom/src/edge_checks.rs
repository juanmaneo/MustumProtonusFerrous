//! Boundary and branch tests for the ported rtscom logic.
//!
//! The expectations come from Wine's `rtscom_main.c`, `rtscom_reg.idl` and `rtscom.idl`, or
//! from the port's own branches. Each expectation that no oracle row prints is marked
//! "non-oracle".

use super::*;
use crate::com::is_equal_guid;
use crate::oracle_checks::{make_stylus, OTHER_CLSID, OTHER_IID};

/// The C name of each stub method. Each arm is a body in `rtscom_main.c`.
fn c_name(method: StubMethod) -> &'static str {
    match method {
        StubMethod::GetEnabled => "get_Enabled",
        StubMethod::PutEnabled => "put_Enabled",
        StubMethod::GetHwnd => "get_HWND",
        StubMethod::PutHwnd => "put_HWND",
        StubMethod::GetWindowInputRectangle => "get_WindowInputRectangle",
        StubMethod::PutWindowInputRectangle => "put_WindowInputRectangle",
        StubMethod::AddStylusSyncPlugin => "AddStylusSyncPlugin",
        StubMethod::RemoveStylusSyncPlugin => "RemoveStylusSyncPlugin",
        StubMethod::RemoveAllStylusSyncPlugins => "RemoveAllStylusSyncPlugins",
        StubMethod::GetStylusSyncPlugin => "GetStylusSyncPlugin",
        StubMethod::GetStylusSyncPluginCount => "GetStylusSyncPluginCount",
        StubMethod::AddStylusAsyncPlugin => "AddStylusAsyncPlugin",
        StubMethod::RemoveStylusAsyncPlugin => "RemoveStylusAsyncPlugin",
        StubMethod::RemoveAllStylusAsyncPlugins => "RemoveAllStylusAsyncPlugins",
        StubMethod::GetStylusAsyncPlugin => "GetStylusAsyncPlugin",
        StubMethod::GetStylusAsyncPluginCount => "GetStylusAsyncPluginCount",
        StubMethod::GetChildRealTimeStylusPlugin => "get_ChildRealTimeStylusPlugin",
        StubMethod::PutrefChildRealTimeStylusPlugin => "putref_ChildRealTimeStylusPlugin",
        StubMethod::AddCustomStylusDataToQueue => "AddCustomStylusDataToQueue",
        StubMethod::ClearStylusQueues => "ClearStylusQueues",
        StubMethod::SetAllTabletsMode => "SetAllTabletsMode",
        StubMethod::SetSingleTabletMode => "SetSingleTabletMode",
        StubMethod::GetTablet => "GetTablet",
        StubMethod::GetTabletContextIdFromTablet => "GetTabletContextIdFromTablet",
        StubMethod::GetTabletFromTabletContextId => "GetTabletFromTabletContextId",
        StubMethod::GetAllTabletContextIds => "GetAllTabletContextIds",
        StubMethod::GetStyluses => "GetStyluses",
        StubMethod::GetStylusForId => "GetStylusForId",
        StubMethod::SetDesiredPacketDescription => "SetDesiredPacketDescription",
        StubMethod::GetDesiredPacketDescription => "GetDesiredPacketDescription",
        StubMethod::GetPacketDescriptionData => "GetPacketDescriptionData",
    }
}

/// The 31 C names, transcribed from the `FIXME` bodies in `rtscom_main.c` (non-oracle; the
/// harness row R24 prints only the count).
const C_NAMES: [&str; 31] = [
    "get_Enabled",
    "put_Enabled",
    "get_HWND",
    "put_HWND",
    "get_WindowInputRectangle",
    "put_WindowInputRectangle",
    "AddStylusSyncPlugin",
    "RemoveStylusSyncPlugin",
    "RemoveAllStylusSyncPlugins",
    "GetStylusSyncPlugin",
    "GetStylusSyncPluginCount",
    "AddStylusAsyncPlugin",
    "RemoveStylusAsyncPlugin",
    "RemoveAllStylusAsyncPlugins",
    "GetStylusAsyncPlugin",
    "GetStylusAsyncPluginCount",
    "get_ChildRealTimeStylusPlugin",
    "putref_ChildRealTimeStylusPlugin",
    "AddCustomStylusDataToQueue",
    "ClearStylusQueues",
    "SetAllTabletsMode",
    "SetSingleTabletMode",
    "GetTablet",
    "GetTabletContextIdFromTablet",
    "GetTabletFromTabletContextId",
    "GetAllTabletContextIds",
    "GetStyluses",
    "GetStylusForId",
    "SetDesiredPacketDescription",
    "GetDesiredPacketDescription",
    "GetPacketDescriptionData",
];

/// The exported constants. The values are printed by oracle rows R03, R04, R06 and R24.
#[test]
fn constants_are_pinned() {
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
}

/// The GUIDs match `include/unknwn.idl`, `include/rtscom.idl` and
/// `dlls/rtscom/rtscom_reg.idl` (non-oracle; the harness copies the same values).
#[test]
fn guids_match_wine_idl() {
    assert!(is_equal_guid(
        &IID_IUNKNOWN,
        &Guid {
            data1: 0x0000_0000,
            data2: 0x0000,
            data3: 0x0000,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    ));
    assert!(is_equal_guid(
        &IID_ICLASSFACTORY,
        &Guid {
            data1: 0x0000_0001,
            data2: 0x0000,
            data3: 0x0000,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    ));
    assert!(is_equal_guid(
        &IID_IREALTIMESTYLUS,
        &Guid {
            data1: 0xa8bb_5d22,
            data2: 0x3144,
            data3: 0x4a7b,
            data4: [0x93, 0xcd, 0xf3, 0x4a, 0x16, 0xbe, 0x51, 0x3a],
        }
    ));
    assert!(is_equal_guid(
        &CLSID_REALTIMESTYLUS,
        &Guid {
            data1: 0xe26b_366d,
            data2: 0xf998,
            data3: 0x43ce,
            data4: [0x83, 0x6f, 0xcb, 0x6d, 0x90, 0x44, 0x32, 0xb0],
        }
    ));
}

/// Non-oracle, from the `FIXME` bodies: the enum lists each C method once, in C order.
#[test]
fn stub_methods_map_to_their_c_bodies() {
    assert_eq!(STUB_METHODS.map(c_name), C_NAMES);
}

/// Non-oracle, per method, from the `return E_NOTIMPL` in each body. Row R24 confirms the
/// count of 31.
#[test]
fn every_stub_method_returns_e_notimpl() {
    for method in STUB_METHODS {
        assert_eq!(stub_method(method), E_NOTIMPL);
    }
}

/// Non-oracle: `rtstylus_AddRef` and `rtstylus_Release` change the count by one, and the
/// object is freed only when the count reaches zero.
#[test]
fn reference_count_moves_by_one() {
    let mut stylus = make_stylus();
    assert_eq!(stylus.add_ref(), 2);
    assert_eq!(stylus.add_ref(), 3);
    assert_eq!(stylus.release(), 2);
    assert_eq!(stylus.release(), 1);
    assert_eq!(stylus.release(), 0);
}

/// Non-oracle: each successful `QueryInterface` takes one reference, and a rejected one takes
/// none. Row R19 covers the rejected case.
#[test]
fn successful_query_interface_adds_one_reference() {
    let mut stylus = make_stylus();
    let mut obj = None;
    assert_eq!(stylus.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert_eq!(stylus.query_interface(&IID_IREALTIMESTYLUS, &mut obj), S_OK);
    assert_eq!(stylus.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert_eq!(stylus.release(), 3);
    assert_eq!(stylus.release(), 2);
    assert_eq!(stylus.release(), 1);
    assert_eq!(stylus.release(), 0);
}

/// Non-oracle: the class factory answers only `IUnknown` and `IClassFactory`.
#[test]
fn class_factory_rejects_other_iids() {
    let mut obj = Some(Interface::ClassFactory);
    assert_eq!(
        class_factory_query_interface(&OTHER_IID, &mut obj),
        E_NOINTERFACE
    );
    assert!(obj.is_none());
}

/// Non-oracle, from `DllGetClassObject`: the CLSID is tested first, so an unknown CLSID returns
/// `CLASS_E_CLASSNOTAVAILABLE` whatever the IID is.
#[test]
fn unknown_clsid_is_checked_before_iid() {
    let mut obj = Some(Interface::RealTimeStylus);
    assert_eq!(
        dll_get_class_object(&OTHER_CLSID, &OTHER_IID, &mut obj),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert!(matches!(obj, Some(Interface::RealTimeStylus)));
}

/// Non-oracle: `IsEqualGUID` compares all 16 bytes, so a change in any field makes the GUID
/// unequal. Each near-miss is rejected.
#[test]
fn near_miss_guids_are_rejected() {
    let clsids = [
        Guid {
            data1: 0xe26b_366c,
            ..CLSID_REALTIMESTYLUS
        },
        Guid {
            data2: 0xf999,
            ..CLSID_REALTIMESTYLUS
        },
        Guid {
            data3: 0x43cf,
            ..CLSID_REALTIMESTYLUS
        },
        Guid {
            data4: [0x83, 0x6f, 0xcb, 0x6d, 0x90, 0x44, 0x32, 0xb1],
            ..CLSID_REALTIMESTYLUS
        },
    ];
    for clsid in &clsids {
        let mut obj = Some(Interface::RealTimeStylus);
        assert_eq!(
            dll_get_class_object(clsid, &IID_ICLASSFACTORY, &mut obj),
            CLASS_E_CLASSNOTAVAILABLE
        );
        assert!(matches!(obj, Some(Interface::RealTimeStylus)));
    }

    let iids = [
        Guid {
            data1: 0xa8bb_5d23,
            ..IID_IREALTIMESTYLUS
        },
        Guid {
            data2: 0x3145,
            ..IID_IREALTIMESTYLUS
        },
        Guid {
            data3: 0x4a7c,
            ..IID_IREALTIMESTYLUS
        },
        Guid {
            data4: [0x93, 0xcd, 0xf3, 0x4a, 0x16, 0xbe, 0x51, 0x3b],
            ..IID_IREALTIMESTYLUS
        },
    ];
    for iid in &iids {
        let (hr, stylus) = realtime_stylus_create(iid);
        assert_eq!(hr, E_NOINTERFACE);
        assert!(stylus.is_none());
    }
}

/// Non-oracle: `class_factory_LockServer` only logs, so a lock does not change what
/// `DllCanUnloadNow` returns.
#[test]
fn lock_server_has_no_effect_on_can_unload() {
    assert_eq!(class_factory_lock_server(true), S_OK);
    assert_eq!(dll_can_unload_now(), S_FALSE);
    assert_eq!(class_factory_lock_server(false), S_OK);
    assert_eq!(dll_can_unload_now(), S_FALSE);
}
