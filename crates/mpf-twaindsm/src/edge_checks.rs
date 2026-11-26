//! Edge-case checks beyond the oracle rows. None of the expectations here is oracle-derived:
//! they follow the C control flow in `dsm_ctrl.c` and `twain32_main.c` and cover the branches
//! that the 25 rows do not reach.

use crate::test_host::*;
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

const DG_IMAGE: u32 = 2;

fn init_dsm(dsm: &mut Dsm<FakeHost>, origin: &mut Identity, parent: usize) {
    let rc = dsm.entry(
        origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(parent),
    );
    assert_eq!(rc, TWRC_SUCCESS);
}

/// An initialized DSM with one open source, `5` ("SRC"). The application identity is `9`.
fn session(parent: usize) -> (Dsm<FakeHost>, Rc<RefCell<World>>) {
    let (mut dsm, world) = new_dsm(Vec::new());
    init_dsm(&mut dsm, &mut ident(9, 0, "APP"), parent);
    adopt(&mut dsm, &world, 5, "SRC");
    (dsm, world)
}

fn queue_xferready(dsm: &mut Dsm<FakeHost>, source: &mut Identity) {
    let rc = dsm.entry(
        source,
        None,
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!(rc, TWRC_SUCCESS);
}

fn event_for(message: u32, hwnd: usize) -> TwEvent {
    TwEvent {
        msg: Msg { message, hwnd },
        tw_message: 0x55,
    }
}

// Not oracle-derived: the message classes ControlNull queues, and the constants it depends on.
#[test]
fn control_null_message_classes() {
    assert!(is_control_null_msg(MSG_XFERREADY));
    assert!(is_control_null_msg(MSG_CLOSEDSREQ));
    assert!(is_control_null_msg(MSG_DEVICEEVENT));
    assert!(!is_control_null_msg(MSG_GET));
    assert_eq!(TW_ENTRYPOINT_SIZE, 44);
}

// Not oracle-derived: every handled DAT/MSG pair maps to its operation, and others do not.
#[test]
fn source_manager_op_maps_handled_pairs() {
    let table = [
        ((DAT_IDENTITY, MSG_CLOSEDS), SourceManagerOp::CloseDs),
        ((DAT_IDENTITY, MSG_GETDEFAULT), SourceManagerOp::GetDefault),
        ((DAT_IDENTITY, MSG_GETFIRST), SourceManagerOp::GetFirst),
        ((DAT_IDENTITY, MSG_GETNEXT), SourceManagerOp::GetNext),
        ((DAT_IDENTITY, MSG_OPENDS), SourceManagerOp::OpenDs),
        ((DAT_IDENTITY, MSG_USERSELECT), SourceManagerOp::UserSelect),
        ((DAT_PARENT, MSG_CLOSEDSM), SourceManagerOp::CloseDsm),
        ((DAT_PARENT, MSG_OPENDSM), SourceManagerOp::OpenDsm),
        ((DAT_ENTRYPOINT, MSG_GET), SourceManagerOp::GetEntrypoint),
        ((DAT_STATUS, MSG_GET), SourceManagerOp::GetDsmStatus),
    ];
    for ((dat, msg), op) in table {
        assert_eq!(source_manager_op(dat, msg), Some(op));
    }
    assert_eq!(source_manager_op(DAT_STATUS, MSG_SET), None);
    assert_eq!(source_manager_op(DAT_PARENT, 0x999), None);
    assert_eq!(source_manager_op(DAT_ENTRYPOINT, MSG_SET), None);
    assert_eq!(source_manager_op(0xffff, MSG_GET), None);
    assert_eq!(format!("{:?}", SourceManagerOp::OpenDs), "OpenDs");
}

// Not oracle-derived: device selection by exact name, with the first device as the fallback.
#[test]
fn select_device_matches_name_or_defaults_to_first() {
    let devices = vec![device("a.ds", 1, "Alpha"), device("b.ds", 1, "Beta")];
    assert_eq!(select_device(&devices, ""), 0);
    assert_eq!(select_device(&devices, "Beta"), 1);
    assert_eq!(select_device(&devices, "Gamma"), 0);
}

// Not oracle-derived: the user-select offer list keeps only devices sharing a group.
#[test]
fn selectable_identities_share_a_group() {
    let devices = vec![
        device("a.ds", 0x1, "Alpha"),
        device("b.ds", 0x2, "Beta"),
        device("c.ds", 0x3, "Gamma"),
    ];
    let names: Vec<String> = selectable_identities(&devices, 0x2)
        .into_iter()
        .map(|identity| identity.product_name)
        .collect();
    assert_eq!(names, ["Beta", "Gamma"]);
    assert!(selectable_identities(&devices, 0x4).is_empty());
}

// Not oracle-derived: DS2 detection, the stored copy of an open source, and the derived impls.
#[test]
fn identity_ds2_and_active_copy() {
    let app = ident(3, DF_APP2 | DF_DS2 | 0x1, "Name");
    assert!(app.has_ds2());
    assert!(!ident(3, DF_APP2, "Name").has_ds2());
    let copy = app.active_copy();
    assert_eq!(copy, ident(3, 0, "Name"));
    assert_ne!(copy, app);
    assert_eq!(app.clone(), app);
    assert!(format!("{:?}", copy).contains("Name"));
}

// Not oracle-derived: a callback starts unregistered.
#[test]
fn callback_default_is_unregistered() {
    let callback = Callback::default();
    assert_eq!((callback.proc_addr, callback.ref_con), (0, 0));
    assert!(format!("{:?}", callback).contains("proc_addr"));
}

// Not oracle-derived: lookup by `Id` returns the first match in list order.
#[test]
fn find_source_returns_first_match_in_list_order() {
    let sources = vec![
        ActiveDs::new(ident(4, 0, "x"), ()),
        ActiveDs::new(ident(2, 0, "y"), ()),
        ActiveDs::new(ident(2, 0, "z"), ()),
    ];
    assert_eq!(find_source(&sources, 2), Some(1));
    assert_eq!(find_source(&sources, 9), None);
}

// Not oracle-derived: a DSM_Entry call with an unknown origin on DAT_NULL is a protocol error.
#[test]
fn unknown_origin_on_control_null_is_bad_protocol() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let rc = dsm.entry(
        &mut ident(9, 0, "APP"),
        None,
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADPROTOCOL));
}

// Not oracle-derived: a control operation with no source manager handler is a protocol error.
#[test]
fn unknown_control_operation_is_bad_protocol() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let rc = dsm.entry(
        &mut ident(9, 0, "APP"),
        None,
        DG_CONTROL,
        DAT_STATUS,
        MSG_SET,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADPROTOCOL));
}

// Not oracle-derived: source manager identity operations reject a NULL pData with BADVALUE.
#[test]
fn identity_operations_reject_missing_identity_data() {
    let (mut dsm, _world) = new_dsm(vec![device("a.ds", 1, "Alpha")]);
    let mut app = ident(9, 1, "APP");
    for (dat, msg) in [
        (DAT_IDENTITY, MSG_CLOSEDS),
        (DAT_IDENTITY, MSG_GETDEFAULT),
        (DAT_IDENTITY, MSG_GETFIRST),
        (DAT_IDENTITY, MSG_GETNEXT),
        (DAT_IDENTITY, MSG_OPENDS),
        (DAT_IDENTITY, MSG_USERSELECT),
    ] {
        let rc = dsm.entry(&mut app, None, DG_CONTROL, dat, msg, Data::Null);
        assert_eq!(
            (rc, dsm.condition_code()),
            (TWRC_FAILURE, TWCC_BADVALUE),
            "msg {msg:#x}"
        );
    }
}

// Not oracle-derived: the entrypoint and status queries reject other pData kinds with BADVALUE.
#[test]
fn entrypoint_and_status_reject_wrong_data() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut app = ident(9, 1, "APP");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_ENTRYPOINT,
        MSG_GET,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADVALUE));
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_STATUS,
        MSG_GET,
        Data::Opaque,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADVALUE));
}

// Not oracle-derived: the parent operations accept a NULL pData as window 0 and reject other data.
#[test]
fn parent_operations_null_and_wrong_data() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut app = ident(9, 1, "APP");
    for msg in [MSG_OPENDSM, MSG_CLOSEDSM] {
        let rc = dsm.entry(&mut app, None, DG_CONTROL, DAT_PARENT, msg, Data::Opaque);
        assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADVALUE));
    }
    assert!(!dsm.is_initialized());

    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Null,
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert!(dsm.is_initialized());
    assert_eq!(dsm.parent(), 0);
}

// Not oracle-derived: OpenDSM sets the DSM2 bit only for DF_APP2 applications.
#[test]
fn open_dsm_sets_dsm2_only_for_app2_applications() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut app = ident(9, 0x1, "APP");
    init_dsm(&mut dsm, &mut app, 0x10);
    assert_eq!(app.supported_groups, 0x1);

    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut app = ident(9, DF_APP2, "APP");
    init_dsm(&mut dsm, &mut app, 0x10);
    assert_eq!(app.supported_groups, DF_APP2 | DF_DSM2);
}

// Not oracle-derived: CloseDSM with no open sources still resets the DSM state.
#[test]
fn close_dsm_with_no_open_sources_resets_state() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut app = ident(9, 0, "APP");
    init_dsm(&mut dsm, &mut app, 0x20);
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_CLOSEDSM,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_SUCCESS, TWCC_SUCCESS));
    assert!(!dsm.is_initialized());
    assert_eq!(dsm.parent(), 0);
    assert!(world.borrow().ds_calls.is_empty());
}

// Not oracle-derived: OpenDS before OpenDSM is a sequence error, and an empty device list is NODS.
#[test]
fn open_ds_with_empty_device_list_is_nods() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut app = ident(9, 1, "APP");
    let mut target = ident(0, 0, "");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut target),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_SEQERROR));

    init_dsm(&mut dsm, &mut app, 0);
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut target),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_NODS));
}

// Not oracle-derived: an unknown product name loads the first device, and the source keeps a
// copy with SupportedGroups cleared.
#[test]
fn open_ds_unknown_name_loads_first_device() {
    let (mut dsm, world) = new_dsm(vec![
        device("a.ds", 0x1, "Alpha"),
        device("b.ds", 0x1, "Beta"),
    ]);
    let mut app = ident(9, 0x1, "APP");
    init_dsm(&mut dsm, &mut app, 0);
    let mut target = ident(0, 0, "Gamma");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut target),
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert_eq!(world.borrow().loads, ["a.ds"]);
    assert_eq!(dsm.sources()[0].identity, ident(0, 0, "Gamma"));
}

// Not oracle-derived: the entrypoint is set up only for a source that reports DF_DS2.
#[test]
fn open_ds_without_ds2_skips_entrypoint_setup() {
    let (mut dsm, world) = new_dsm(vec![device("a.ds", 0x1, "Alpha")]);
    world.borrow_mut().get_groups = 0x1;
    let mut app = ident(9, 0x1, "APP");
    init_dsm(&mut dsm, &mut app, 0);
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut ident(0, 0, "Alpha")),
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert!(!world
        .borrow()
        .ds_calls
        .iter()
        .any(|call| call.1 == DAT_ENTRYPOINT));
}

// Not oracle-derived: a failed MSG_OPENDS adds no source and gives back the source Id.
#[test]
fn open_ds_open_failure_restores_source_id() {
    let (mut dsm, world) = new_dsm(vec![device("a.ds", 0x1, "Alpha")]);
    let mut app = ident(9, 0x1, "APP");
    init_dsm(&mut dsm, &mut app, 0);
    world.borrow_mut().open_rc = 1;
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut ident(0, 0, "Alpha")),
    );
    assert_eq!(
        (rc, dsm.condition_code()),
        (TWRC_FAILURE, TWCC_OPERATIONERROR)
    );
    assert_eq!(dsm.next_source_id(), 0);
    assert!(dsm.sources().is_empty());
}

// Not oracle-derived: CloseDS removes the source and reports the source's result.
#[test]
fn close_ds_removes_source_on_success_and_failure() {
    let (mut dsm, world) = session(0);
    let mut app = ident(9, 0, "APP");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_CLOSEDS,
        Data::Identity(&mut ident(5, 0, "SRC")),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_SUCCESS, TWCC_SUCCESS));
    assert!(dsm.sources().is_empty());
    assert!(world
        .borrow()
        .ds_calls
        .contains(&(DG_CONTROL, DAT_IDENTITY, MSG_CLOSEDS)));

    let (mut dsm, world) = session(0);
    world.borrow_mut().close_rc = 1;
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_CLOSEDS,
        Data::Identity(&mut ident(5, 0, "SRC")),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_SEQERROR));
    assert!(dsm.sources().is_empty());
}

// Not oracle-derived: the device cursor walks the list, then reports the end of the list.
#[test]
fn identity_getters_walk_the_device_list() {
    let (mut dsm, _world) = new_dsm(vec![
        device("a.ds", 0x1, "Alpha"),
        device("b.ds", 0x2, "Beta"),
    ]);
    let mut app = ident(9, 0x3, "APP");
    let mut out = ident(0, 0, "");

    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETDEFAULT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, out.product_name.as_str()), (TWRC_SUCCESS, "Alpha"));

    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETFIRST,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, out.product_name.as_str()), (TWRC_SUCCESS, "Alpha"));

    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETNEXT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, out.product_name.as_str()), (TWRC_SUCCESS, "Beta"));

    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETNEXT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_ENDOFLIST, TWCC_SUCCESS));
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETNEXT,
        Data::Identity(&mut out),
    );
    assert_eq!(rc, TWRC_ENDOFLIST);

    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETFIRST,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, out.product_name.as_str()), (TWRC_SUCCESS, "Alpha"));
}

// Not oracle-derived: an empty device list is NODS for GetFirst and end-of-list for GetNext.
#[test]
fn identity_getters_on_empty_list() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut app = ident(9, 0, "APP");
    let mut out = ident(0, 0, "");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETFIRST,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_NODS));
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETNEXT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_ENDOFLIST, TWCC_SUCCESS));
}

// Not oracle-derived: the user-select dialog is offered the matching devices and the DSM parent.
#[test]
fn user_select_cancel_offers_matching_devices_with_parent() {
    let (mut dsm, world) = new_dsm(vec![
        device("a.ds", 0x1, "Alpha"),
        device("b.ds", 0x2, "Beta"),
    ]);
    let mut app = ident(9, 0x2, "APP");
    init_dsm(&mut dsm, &mut app, 0x77);
    let mut out = ident(0, 0, "");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_USERSELECT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_CANCEL, TWCC_SUCCESS));
    assert_eq!(world.borrow().offers, vec![(0x77, vec!["Beta".to_owned()])]);
    assert_eq!(out, ident(0, 0, ""));
}

// Not oracle-derived: with no device sharing a group, the dialog is offered an empty list.
#[test]
fn user_select_with_no_shared_group_offers_nothing() {
    let (mut dsm, world) = new_dsm(vec![device("a.ds", 0x1, "Alpha")]);
    let mut app = ident(9, 0x2, "APP");
    init_dsm(&mut dsm, &mut app, 0x77);
    let mut out = ident(0, 0, "");
    let rc = dsm.entry(
        &mut app,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_USERSELECT,
        Data::Identity(&mut out),
    );
    assert_eq!(rc, TWRC_CANCEL);
    assert_eq!(world.borrow().offers, vec![(0x77, Vec::<String>::new())]);
}

// Not oracle-derived: the FakeSource answers MSG_GET without identity data by reporting its
// result and leaving the identity alone. Covers the non-identity branch of the test source.
#[test]
fn fake_source_get_without_identity_data_leaves_identity_alone() {
    let world = shared_world();
    world.borrow_mut().get_groups = 0x7;
    let mut host = FakeHost::new(world.clone());
    let mut source = host.load_source("a.ds").expect("a.ds loads");
    assert_eq!(
        source.entry(None, DG_CONTROL, DAT_IDENTITY, MSG_GET, Data::Null),
        TWRC_SUCCESS
    );
    assert_eq!(
        world.borrow().ds_calls,
        vec![(DG_CONTROL, DAT_IDENTITY, MSG_GET)]
    );
}

// Not oracle-derived: with no callback and no messages queued, a DS-owned message queues and
// posts to the thread queue only (no parent, UI or event window set).
#[test]
fn control_null_queues_when_callback_proc_is_null() {
    let (mut dsm, world) = session(0);
    dsm.sources_mut()[0].callback = Callback {
        proc_addr: 0,
        ref_con: 0x9,
    };
    let mut src = ident(5, 0, "SRC");
    queue_xferready(&mut dsm, &mut src);
    assert!(world.borrow().callbacks.is_empty());
    assert_eq!(dsm.sources()[0].pending.len(), 1);
}

// Not oracle-derived: a callback addressed to a destination passes that destination's Id.
#[test]
fn callback_receives_destination_id_when_addressed() {
    let (mut dsm, world) = session(0);
    dsm.sources_mut()[0].callback = Callback {
        proc_addr: 0x5000,
        ref_con: 0xabc,
    };
    let mut src = ident(5, 0, "SRC");
    let dest = ident(5, 0, "SRC");
    let rc = dsm.entry(
        &mut src,
        Some(&dest),
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert_eq!(
        world.borrow().callbacks,
        vec![(0x5000, 0xabc, Some(5), MSG_XFERREADY)]
    );
}

// Not oracle-derived: a callback that runs ControlNull again is rejected.
#[test]
fn nested_application_callback_is_bad_protocol() {
    let (mut dsm, world) = session(0);
    dsm.sources_mut()[0].callback = Callback {
        proc_addr: 0x5000,
        ref_con: 0,
    };
    dsm.processing_callback = true;
    let mut src = ident(5, 0, "SRC");
    let rc = dsm.entry(
        &mut src,
        None,
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADPROTOCOL));
    assert!(world.borrow().callbacks.is_empty());
}

// Not oracle-derived: the C code sets the condition code only when the callback's result is not
// success, so a successful callback leaves the previous condition code alone.
#[test]
fn successful_callback_keeps_condition_code() {
    let (mut dsm, world) = session(0);
    dsm.sources_mut()[0].callback = Callback {
        proc_addr: 0x5000,
        ref_con: 0,
    };
    let mut app = ident(9, 0, "APP");
    dsm.entry(&mut app, None, DG_CONTROL, DAT_STATUS, MSG_SET, Data::Null);
    assert_eq!(dsm.condition_code(), TWCC_BADPROTOCOL);

    let mut src = ident(5, 0, "SRC");
    let rc = dsm.entry(
        &mut src,
        None,
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert_eq!(dsm.condition_code(), TWCC_BADPROTOCOL);
    assert_eq!(world.borrow().callbacks.len(), 1);
}

fn posts_for(parent: usize, ui: usize, event: usize) -> Vec<usize> {
    let (mut dsm, _world) = session(parent);
    dsm.sources_mut()[0].ui_window = ui;
    dsm.sources_mut()[0].event_window = event;
    let mut src = ident(5, 0, "SRC");
    queue_xferready(&mut dsm, &mut src);
    dsm.posted_messages().to_vec()
}

// Not oracle-derived: each window is posted once, in parent, UI, event, thread order.
#[test]
fn control_null_posts_each_window_once() {
    assert_eq!(posts_for(0, 0, 0), vec![0]);
    assert_eq!(posts_for(0x10, 0x10, 0), vec![0x10, 0]);
    assert_eq!(posts_for(0x10, 0x20, 0x20), vec![0x10, 0x20, 0]);
    assert_eq!(posts_for(0x10, 0, 0x10), vec![0x10, 0]);
    assert_eq!(posts_for(0, 0, 0x30), vec![0x30, 0]);
    assert_eq!(posts_for(0, 0x20, 0), vec![0x20, 0]);
    assert_eq!(posts_for(0x10, 0x20, 0x30), vec![0x10, 0x20, 0x30, 0]);
}

// Not oracle-derived: a pending message with a window that already has a peeked event posts no
// window message, and the window is recorded as the source's event window.
#[test]
fn process_event_with_pending_and_window_found_posts_nothing() {
    let (mut dsm, world) = session(0x10);
    let mut src = ident(5, 0, "SRC");
    queue_xferready(&mut dsm, &mut src);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    queue_xferready(&mut dsm, &mut src);
    let before = dsm.posted_messages().len();
    world.borrow_mut().peek_found = true;
    let mut event = event_for(0xc001, 0x40);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Event(&mut event),
    );
    assert_eq!((rc, event.tw_message), (TWRC_DSEVENT, MSG_XFERREADY));
    assert_eq!(dsm.posted_messages().len(), before);
    assert_eq!(world.borrow().peeks, vec![(0x40, 0xc001)]);
    assert_eq!(dsm.sources()[0].event_window, 0x40);
}

// Not oracle-derived: a pending message with a window that has no peeked event posts that window.
#[test]
fn process_event_with_pending_and_window_not_found_posts_window() {
    let (mut dsm, _world) = session(0x10);
    let mut src = ident(5, 0, "SRC");
    queue_xferready(&mut dsm, &mut src);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    queue_xferready(&mut dsm, &mut src);
    let before = dsm.posted_messages().len();
    let mut event = event_for(0xc001, 0x40);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Event(&mut event),
    );
    assert_eq!(rc, TWRC_DSEVENT);
    assert_eq!(dsm.posted_messages().len(), before + 1);
    assert_eq!(dsm.posted_messages().last(), Some(&0x40));
}

// Not oracle-derived: with nothing pending, the event answer is MSG_NULL and nothing is peeked.
#[test]
fn process_event_with_nothing_pending_answers_null() {
    let (mut dsm, world) = session(0);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    let mut event = event_for(0xc001, 0x40);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Event(&mut event),
    );
    assert_eq!((rc, event.tw_message), (TWRC_DSEVENT, MSG_NULL));
    assert!(world.borrow().peeks.is_empty());
    assert_eq!(dsm.sources()[0].event_window, 0x40);
}

// Not oracle-derived: an event that is not the DSM's event message is forwarded to the source,
// and a zero window leaves the event window alone.
#[test]
fn process_event_other_message_is_forwarded() {
    let (mut dsm, world) = session(0);
    world.borrow_mut().fwd_rc = 7;
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    let mut event = event_for(0xdead, 0);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Event(&mut event),
    );
    assert_eq!((rc, event.tw_message), (7, 0x55));
    assert_eq!(dsm.sources()[0].event_window, 0);
    assert!(world
        .borrow()
        .ds_calls
        .contains(&(DG_CONTROL, DAT_EVENT, MSG_PROCESSEVENT)));
}

// Not oracle-derived: ProcessEvent without a TW_EVENT is a bad value.
#[test]
fn process_event_without_event_data_is_bad_value() {
    let (mut dsm, _world) = session(0);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_FAILURE, TWCC_BADVALUE));
}

// Not oracle-derived: ENABLEDS and ENABLEDSUIONLY record the parent window as the UI window.
#[test]
fn enable_ds_records_ui_window() {
    let (mut dsm, world) = session(0);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_USERINTERFACE,
        MSG_ENABLEDS,
        Data::UserInterface { hparent: 0x90 },
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert_eq!(dsm.sources()[0].ui_window, 0x90);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_USERINTERFACE,
        MSG_ENABLEDSUIONLY,
        Data::UserInterface { hparent: 0x91 },
    );
    assert_eq!(rc, TWRC_SUCCESS);
    assert_eq!(dsm.sources()[0].ui_window, 0x91);
    assert_eq!(world.borrow().ds_calls.len(), 2);
}

// Not oracle-derived: the UI window is only recorded for a control enable message with UI data.
#[test]
fn enable_ds_without_ui_data_keeps_window() {
    let (mut dsm, _world) = session(0);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_USERINTERFACE,
        MSG_ENABLEDS,
        Data::Null,
    );
    dsm.entry(
        &mut app,
        Some(&dest),
        DG_IMAGE,
        DAT_USERINTERFACE,
        MSG_ENABLEDS,
        Data::UserInterface { hparent: 0x92 },
    );
    dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_ENABLEDS,
        Data::UserInterface { hparent: 0x93 },
    );
    dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_USERINTERFACE,
        MSG_GET,
        Data::UserInterface { hparent: 0x94 },
    );
    assert_eq!(dsm.sources()[0].ui_window, 0);
}

// Not oracle-derived: REGISTER_CALLBACK for DAT_CALLBACK2 stores the callback.
#[test]
fn register_callback_accepts_callback2() {
    let (mut dsm, _world) = session(0);
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_CALLBACK2,
        MSG_REGISTER_CALLBACK,
        Data::Callback {
            proc_addr: 0x6000,
            ref_con: 0x1,
        },
    );
    assert_eq!((rc, dsm.condition_code()), (TWRC_SUCCESS, TWCC_SUCCESS));
    assert_eq!(dsm.sources()[0].callback.proc_addr, 0x6000);
}

// Not oracle-derived: REGISTER_CALLBACK is forwarded when the data, DAT or DG does not match.
#[test]
fn register_callback_forwards_other_requests() {
    let (mut dsm, world) = session(0);
    world.borrow_mut().fwd_rc = 7;
    let dest = ident(5, 0, "SRC");
    let mut app = ident(9, 0, "APP");
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_CALLBACK,
        MSG_REGISTER_CALLBACK,
        Data::Null,
    );
    assert_eq!(rc, 7);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_REGISTER_CALLBACK,
        Data::Callback {
            proc_addr: 1,
            ref_con: 1,
        },
    );
    assert_eq!(rc, 7);
    let rc = dsm.entry(
        &mut app,
        Some(&dest),
        DG_IMAGE,
        DAT_CALLBACK,
        MSG_REGISTER_CALLBACK,
        Data::Callback {
            proc_addr: 1,
            ref_con: 1,
        },
    );
    assert_eq!(rc, 7);
    assert_eq!(dsm.sources()[0].callback.proc_addr, 0);
}
