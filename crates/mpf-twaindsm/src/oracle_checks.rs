//! Oracle-derived test vectors for the ported Source Manager logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/twaindsm/dsm_ctrl.c`
//! and `dlls/twaindsm/twain32_main.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/twaindsm/`, and running the harness. The harness calls the C handlers directly,
//! or through `DSM_Entry` where a row names it. Each test here drives the same scenario through
//! `Dsm::entry` and checks the same fields as the printed row. The 25 printed rows are the
//! reference, and none was excluded: ASan and UBSan reported no error for any row.

use crate::test_host::*;
use crate::*;

fn closes(world: &std::rc::Rc<std::cell::RefCell<World>>) -> usize {
    world
        .borrow()
        .ds_calls
        .iter()
        .filter(|&&call| call == (DG_CONTROL, DAT_IDENTITY, MSG_CLOSEDS))
        .count()
}

// row 01 OpenDSM first: rc=0 twcc=0 init=1 groups=0x30000001 event=0xc001 parent=0x1234
#[test]
fn row01_open_dsm_first() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, DF_APP2 | 0x1, "APP");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    assert_eq!(
        (rc, dsm.condition_code(), dsm.is_initialized()),
        (0, 0, true)
    );
    assert_eq!(origin.supported_groups, 0x3000_0001);
    assert_eq!(dsm.event_message(), 0xc001);
    assert_eq!(dsm.parent(), 0x1234);
}

// row 02 OpenDSM again: rc=1 twcc=11 init=1 groups=0x0 event=0xc001 parent=0x5678
#[test]
fn row02_open_dsm_again() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1111),
    );
    world.borrow_mut().register_message = 0xc002;
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x5678),
    );
    assert_eq!(
        (rc, dsm.condition_code(), dsm.is_initialized()),
        (1, 11, true)
    );
    assert_eq!(origin.supported_groups, 0);
    assert_eq!(dsm.event_message(), 0xc001);
    assert_eq!(dsm.parent(), 0x5678);
}

// row 03 CloseDSM not open: rc=1 twcc=11 init=0 parent=0x0
#[test]
fn row03_close_dsm_not_open() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_CLOSEDSM,
        Data::Null,
    );
    assert_eq!(
        (rc, dsm.condition_code(), dsm.is_initialized()),
        (1, 11, false)
    );
    assert_eq!(dsm.parent(), 0);
}

// row 04 CloseDSM two sources: rc=0 twcc=0 init=0 sources=0 closes=2 parent=0x0
#[test]
fn row04_close_dsm_two_sources() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    adopt(&mut dsm, &world, 0x10, "S1");
    adopt(&mut dsm, &world, 0x11, "S2");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_CLOSEDSM,
        Data::Null,
    );
    assert_eq!(
        (rc, dsm.condition_code(), dsm.is_initialized()),
        (0, 0, false)
    );
    assert_eq!(dsm.sources().len(), 0);
    assert_eq!(closes(&world), 2);
    assert_eq!(dsm.parent(), 0);
}

// row 05 GetDSMStatus: rc=0 twcc=0 status=9
#[test]
fn row05_get_dsm_status() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    // Sets the condition code to TWCC_BADPROTOCOL, as the C harness does by assignment.
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_STATUS,
        MSG_SET,
        Data::Null,
    );
    let mut status = 0;
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_STATUS,
        MSG_GET,
        Data::Status(&mut status),
    );
    assert_eq!((rc, dsm.condition_code(), status), (0, 0, 9));
}

// row 06 GetEntrypoint NULL: rc=1 twcc=10
#[test]
fn row06_get_entrypoint_null() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_ENTRYPOINT,
        MSG_GET,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (1, 10));
}

// row 07 GetEntrypoint size=43: rc=1 twcc=10
#[test]
fn row07_get_entrypoint_too_small() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_ENTRYPOINT,
        MSG_GET,
        Data::Entrypoint { size: 43 },
    );
    assert_eq!((rc, dsm.condition_code()), (1, 10));
}

// row 08 GetEntrypoint size=sizeof(44): rc=0 twcc=0
#[test]
fn row08_get_entrypoint_size() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let size = TW_ENTRYPOINT_SIZE;
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_ENTRYPOINT,
        MSG_GET,
        Data::Entrypoint { size },
    );
    assert_eq!((rc, dsm.condition_code()), (0, 0));
}

// row 09 ControlNull bad msg: rc=1 twcc=9 pending=0 posts=0
#[test]
fn row09_control_null_bad_msg() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(5, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    let rc = dsm.entry(&mut origin, None, DG_CONTROL, DAT_NULL, 0x0999, Data::Null);
    assert_eq!((rc, dsm.condition_code()), (1, 9));
    assert_eq!(dsm.sources()[0].pending.len(), 0);
    assert_eq!(dsm.posted_messages().len(), 0);
}

// row 10 ControlNull xferready: rc=0 twcc=0 pending=1 posts=4[0x1111,0x2222,0x3333,0x0]
#[test]
fn row10_control_null_xferready() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(5, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1111),
    );
    dsm.sources_mut()[0].ui_window = 0x2222;
    dsm.sources_mut()[0].event_window = 0x3333;
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (0, 0));
    assert_eq!(dsm.sources()[0].pending.len(), 1);
    assert_eq!(dsm.posted_messages(), &[0x1111, 0x2222, 0x3333, 0]);
}

// row 11 ControlNull callback rc=3: rc=3 twcc=9 cb_calls=1 cb_msg=0x101 cb_ref=0xabc pending=0
#[test]
fn row11_control_null_callback() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(5, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    dsm.sources_mut()[0].callback = Callback {
        proc_addr: 0x5000,
        ref_con: 0xabc,
    };
    world.borrow_mut().callback_rc = 3;
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_NULL,
        MSG_XFERREADY,
        Data::Null,
    );
    assert_eq!((rc, dsm.condition_code()), (3, 9));
    let world = world.borrow();
    assert_eq!(world.callbacks.len(), 1);
    assert_eq!(world.callbacks[0].1, 0xabc);
    assert_eq!(world.callbacks[0].3, MSG_XFERREADY);
    assert_eq!(dsm.sources()[0].pending.len(), 0);
}

// row 12 ProcessEvent matched pending: rc=4 twcc=0 tw_message=0x101 pending=0 event_window=0x0
#[test]
fn row12_process_event_matched_pending() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let dest = ident(5, 0, "");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    adopt(&mut dsm, &world, 5, "SRC");
    dsm.sources_mut()[0].pending.push_back(MSG_XFERREADY);
    let mut event = TwEvent {
        msg: Msg {
            message: 0xc001,
            hwnd: 0,
        },
        tw_message: 0xffff,
    };
    let rc = dsm.entry(
        &mut origin,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Event(&mut event),
    );
    assert_eq!((rc, dsm.condition_code()), (4, 0));
    assert_eq!(event.tw_message, MSG_XFERREADY);
    assert_eq!(dsm.sources()[0].pending.len(), 0);
    assert_eq!(dsm.sources()[0].event_window, 0);
}

// row 13 ProcessEvent other msg: rc=0 twcc=0 tw_message=0xffff pending=1 event_window=0x5555
//   ds_calls=1 dat=0x2 msg=0x601 posts=1[0x5555]
#[test]
fn row13_process_event_other_msg() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let dest = ident(5, 0, "");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    adopt(&mut dsm, &world, 5, "SRC");
    dsm.sources_mut()[0].pending.push_back(MSG_XFERREADY);
    let mut event = TwEvent {
        msg: Msg {
            message: 0x1234,
            hwnd: 0x5555,
        },
        tw_message: 0xffff,
    };
    let rc = dsm.entry(
        &mut origin,
        Some(&dest),
        DG_CONTROL,
        DAT_EVENT,
        MSG_PROCESSEVENT,
        Data::Event(&mut event),
    );
    assert_eq!((rc, dsm.condition_code()), (0, 0));
    assert_eq!(event.tw_message, 0xffff);
    assert_eq!(dsm.sources()[0].pending.len(), 1);
    assert_eq!(dsm.sources()[0].event_window, 0x5555);
    assert_eq!(
        world.borrow().ds_calls,
        vec![(DG_CONTROL, DAT_EVENT, MSG_PROCESSEVENT)]
    );
    assert_eq!(dsm.posted_messages(), &[0x5555]);
}

// row 14 IdentityGet first/next/next: rc=0,0,7 names=A,B, twcc=0
#[test]
fn row14_identity_get_first_next() {
    let devices = vec![device("a.ds", 1, "A"), device("b.ds", 1, "B")];
    let (mut dsm, _world) = new_dsm(devices);
    let mut origin = ident(7, 0, "APP");
    let (mut out, mut out2, mut out3) = (ident(0, 0, ""), ident(0, 0, ""), ident(0, 0, ""));
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETFIRST,
        Data::Identity(&mut out),
    );
    let rc2 = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETNEXT,
        Data::Identity(&mut out2),
    );
    let rc3 = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETNEXT,
        Data::Identity(&mut out3),
    );
    assert_eq!((rc, rc2, rc3), (0, 0, 7));
    assert_eq!(
        (out.product_name.as_str(), out2.product_name.as_str()),
        ("A", "B")
    );
    assert_eq!(out3.product_name, "");
    assert_eq!(dsm.condition_code(), 0);
}

// row 15 IdentityGetDefault empty: rc=1 twcc=3
#[test]
fn row15_identity_get_default_empty() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let mut out = ident(0, 0, "");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_GETDEFAULT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, dsm.condition_code()), (1, 3));
}

// row 16 OpenDS not open: rc=1 twcc=11 init=0 sources=0
#[test]
fn row16_open_ds_not_open() {
    let (mut dsm, _world) = new_dsm(vec![device("a.ds", 1, "A")]);
    let mut origin = ident(7, 0, "APP");
    let mut ident_a = ident(0, 0, "A");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut ident_a),
    );
    assert_eq!(
        (rc, dsm.condition_code(), dsm.is_initialized()),
        (1, 11, false)
    );
    assert_eq!(dsm.sources().len(), 0);
}

// row 17 OpenDS B: rc=0 twcc=0 id=0 groups=0x40000001 next_id=1 sources=1 src=B src_groups=0x0
//   ds_calls=3 entrypoint_sets=1
#[test]
fn row17_open_ds_b() {
    let devices = vec![device("a.ds", 1, "A"), device("b.ds", 1, "B")];
    let (mut dsm, world) = new_dsm(devices);
    let mut origin = ident(7, 0, "APP");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    world.borrow_mut().get_groups = DF_DS2 | 0x1;
    let mut ident_b = ident(0, 0, "B");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut ident_b),
    );
    assert_eq!((rc, dsm.condition_code()), (0, 0));
    assert_eq!((ident_b.id, ident_b.supported_groups), (0, 0x4000_0001));
    assert_eq!(dsm.next_source_id(), 1);
    assert_eq!(dsm.sources().len(), 1);
    assert_eq!(dsm.sources()[0].identity.product_name, "B");
    assert_eq!(dsm.sources()[0].identity.supported_groups, 0);
    let world = world.borrow();
    assert_eq!(world.ds_calls.len(), 3);
    let entrypoint_sets = world
        .ds_calls
        .iter()
        .filter(|call| call.1 == DAT_ENTRYPOINT)
        .count();
    assert_eq!(entrypoint_sets, 1);
}

// row 18 OpenDS MSG_GET fails: rc=1 twcc=5 id=0 next_id=0 sources=0 ds_calls=1
#[test]
fn row18_open_ds_msg_get_fails() {
    let (mut dsm, world) = new_dsm(vec![device("a.ds", 1, "A")]);
    let mut origin = ident(7, 0, "APP");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    world.borrow_mut().get_rc = 1;
    let mut ident_a = ident(0, 0, "A");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut ident_a),
    );
    assert_eq!((rc, dsm.condition_code()), (1, 5));
    assert_eq!(
        (ident_a.id, dsm.next_source_id(), dsm.sources().len()),
        (0, 0, 0)
    );
    assert_eq!(world.borrow().ds_calls.len(), 1);
}

// row 19 OpenDS LoadLibrary fails: rc=1 twcc=5 id=119 next_id=0 sources=0
#[test]
fn row19_open_ds_load_fails() {
    let (mut dsm, _world) = new_dsm(vec![device("bad.ds", 1, "A")]);
    let mut origin = ident(7, 0, "APP");
    dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_PARENT,
        MSG_OPENDSM,
        Data::Parent(0x1234),
    );
    let mut ident_x = ident(0x77, 0, "");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_OPENDS,
        Data::Identity(&mut ident_x),
    );
    assert_eq!((rc, dsm.condition_code()), (1, 5));
    assert_eq!(
        (ident_x.id, dsm.next_source_id(), dsm.sources().len()),
        (119, 0, 0)
    );
}

// row 20 CloseDS unknown id: rc=1 twcc=3 sources=1
#[test]
fn row20_close_ds_unknown_id() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    let mut ident_9 = ident(9, 0, "");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_CLOSEDS,
        Data::Identity(&mut ident_9),
    );
    assert_eq!((rc, dsm.condition_code(), dsm.sources().len()), (1, 3, 1));
}

// row 21 DSM_Entry unknown dest: rc=1 twcc=12
#[test]
fn row21_dsm_entry_unknown_dest() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    let dest = ident(9, 0, "");
    let mut status = 0;
    let rc = dsm.entry(
        &mut origin,
        Some(&dest),
        DG_CONTROL,
        DAT_STATUS,
        MSG_GET,
        Data::Status(&mut status),
    );
    assert_eq!((rc, dsm.condition_code()), (1, 12));
}

// row 22 DSM_Entry DG_IMAGE no dest: rc=1 twcc=9
#[test]
fn row22_dsm_entry_dg_image_no_dest() {
    let (mut dsm, _world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    let rc = dsm.entry(&mut origin, None, 2, 0x0101, MSG_GET, Data::Null);
    assert_eq!((rc, dsm.condition_code()), (1, 9));
}

// row 23 DSM_Entry forward: rc=2 twcc=0 ds_calls=1 dat=0x101 msg=1
#[test]
fn row23_dsm_entry_forward() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    let dest = ident(5, 0, "");
    world.borrow_mut().fwd_rc = 2;
    let rc = dsm.entry(&mut origin, Some(&dest), 2, 0x0101, MSG_GET, Data::Null);
    assert_eq!((rc, dsm.condition_code()), (2, 0));
    assert_eq!(world.borrow().ds_calls, vec![(2, 0x0101, MSG_GET)]);
}

// row 24 DSM_Entry register callback: rc=0 twcc=0 ds_calls=0 cb=0x7777 ref=0x8888
#[test]
fn row24_dsm_entry_register_callback() {
    let (mut dsm, world) = new_dsm(Vec::new());
    let mut origin = ident(7, 0, "APP");
    adopt(&mut dsm, &world, 5, "SRC");
    let dest = ident(5, 0, "");
    let rc = dsm.entry(
        &mut origin,
        Some(&dest),
        DG_CONTROL,
        DAT_CALLBACK2,
        MSG_REGISTER_CALLBACK,
        Data::Callback {
            proc_addr: 0x7777,
            ref_con: 0x8888,
        },
    );
    assert_eq!((rc, dsm.condition_code()), (0, 0));
    assert_eq!(world.borrow().ds_calls.len(), 0);
    let callback = &dsm.sources()[0].callback;
    assert_eq!((callback.proc_addr, callback.ref_con), (0x7777, 0x8888));
}

// row 25 UserSelect ok: rc=0 twcc=0 selected=B groups=0x2
#[test]
fn row25_user_select_ok() {
    let devices = vec![
        device("a.ds", 0x1, "A"),
        device("b.ds", 0x2, "B"),
        device("c.ds", 0x4, "C"),
    ];
    let (mut dsm, world) = new_dsm(devices);
    let mut origin = ident(7, 0x6, "APP");
    world.borrow_mut().selection = Some(0);
    let mut out = ident(0, 0, "");
    let rc = dsm.entry(
        &mut origin,
        None,
        DG_CONTROL,
        DAT_IDENTITY,
        MSG_USERSELECT,
        Data::Identity(&mut out),
    );
    assert_eq!((rc, dsm.condition_code()), (0, 0));
    assert_eq!(
        (out.product_name.as_str(), out.supported_groups),
        ("B", 0x2)
    );
    assert_eq!(
        world.borrow().offers,
        vec![(0, vec!["B".to_owned(), "C".to_owned()])]
    );
}
