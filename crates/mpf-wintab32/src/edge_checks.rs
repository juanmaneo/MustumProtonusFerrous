//! Edge cases the oracle rows do not reach: extreme arithmetic that Wine leaves undefined (ported
//! with wrapping), zero-size queues and axes, shrunken queues, and the state transitions that do
//! not change anything.

use crate::context::{Context, CXO_CSRMESSAGES, CXO_MESSAGES, CXS_OBSCURED, CXS_ONTOP};
use crate::packet::{
    encode, encode_all, encoded_len, fill, Packet, PK_CONTEXT, PK_SERIAL_NUMBER, TPS_QUEUE_ERR,
};
use crate::queue::Queue;
use crate::scale::{map_point, mul_div, Axis};
use crate::Accepted;

fn packet(serial: u32) -> Packet {
    Packet {
        serial,
        ..Packet::default()
    }
}

#[test]
fn mul_div_rounds_negative_results_away_from_zero() {
    assert_eq!(mul_div(-1, 1, 2), -1);
    assert_eq!(mul_div(-5, 1, 2), -3);
}

#[test]
fn mul_div_reports_results_outside_the_int_range() {
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
    assert_eq!(mul_div(i32::MIN, 1, -1), -1);
}

#[test]
fn flat_y_extent_keeps_the_scaled_position() {
    let x_axis = Axis {
        in_org: 0,
        in_ext: 1000,
        out_org: 0,
        out_ext: 500,
    };
    let y_axis = Axis {
        in_org: 0,
        in_ext: 1000,
        out_org: 7,
        out_ext: 0,
    };
    assert_eq!(map_point(x_axis, y_axis, 500, 300), (250, 7));
}

#[test]
fn zero_capacity_refuses_without_flagging() {
    let mut queue = Queue::new(0);
    assert!(!queue.push(packet(1)));
    assert!(queue.packets().is_empty());
}

#[test]
fn shrunken_queue_refuses_until_it_drains() {
    let mut queue = Queue::new(3);
    for serial in 1..=3 {
        assert!(queue.push(packet(serial)));
    }
    queue.set_capacity(1);
    assert!(!queue.push(packet(4)));
    assert_eq!(queue.packets().len(), 3);
    assert_eq!(queue.packets()[2].status, TPS_QUEUE_ERR);

    assert_eq!(queue.take_front(2).len(), 2);
    assert!(!queue.push(packet(5)));
    assert_eq!(queue.packets()[0].status, TPS_QUEUE_ERR);

    queue.take_front(1);
    assert!(queue.push(packet(6)));
    assert_eq!(queue.packets(), &[packet(6)]);
}

#[test]
fn data_range_needs_both_serials_in_order() {
    let mut queue = Queue::new(5);
    for serial in 5..=9 {
        assert!(queue.push(packet(serial)));
    }
    assert_eq!(queue.data_range(3, 9), None);
    assert_eq!(queue.data_range(9, 1), None);
    assert_eq!(queue.data_range(5, 9), Some(0..5));
    assert_eq!(queue.data_range(9, 9), Some(4..5));
}

#[test]
fn enabling_an_enabled_context_keeps_its_status() {
    let mut context = Context::open(1, true, 0, 0);
    context.set_overlap(false);
    context.set_enabled(true);
    assert_eq!(context.status(), CXS_OBSCURED);
    assert!(context.enabled());
    context.set_overlap(true);
    assert_eq!(context.status(), CXS_ONTOP);
}

#[test]
fn cursor_message_needs_the_messages_option() {
    let on_cursor = |cursor: u32| Packet {
        cursor,
        ..Packet::default()
    };
    let mut context = Context::open(1, true, CXO_MESSAGES | CXO_CSRMESSAGES, 0x20);
    assert_eq!(
        context.accept(on_cursor(5)),
        Some(Accepted::Queued { csr_message: true })
    );

    let mut silent = Context::open(1, true, CXO_CSRMESSAGES, 0x20);
    assert_eq!(
        silent.accept(on_cursor(5)),
        Some(Accepted::Queued { csr_message: false })
    );
}

#[test]
fn only_a_queued_cursor_change_asks_for_a_message() {
    assert!(Accepted::Queued { csr_message: true }.csr_message());
    assert!(!Accepted::Queued { csr_message: false }.csr_message());
    assert!(!Accepted::Overflow.csr_message());
}

#[test]
fn disabled_context_takes_no_packets_and_keeps_its_cursor() {
    let mut context = Context::open(1, false, 0, 0);
    assert_eq!(context.accept(packet(1)), None);
    assert_eq!(context.active_cursor(), u32::MAX);
    assert!(context.queue().packets().is_empty());
}

#[test]
fn packet_length_counts_every_field() {
    assert_eq!(encoded_len(0), 0);
    assert_eq!(encoded_len(0x3fff), 76);
}

#[test]
fn context_handle_is_zero_extended_to_eight_bytes() {
    let stamped = Packet {
        context: u32::MAX,
        ..Packet::default()
    };
    assert_eq!(
        encode(PK_CONTEXT, &stamped),
        [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0]
    );
}

#[test]
fn fill_pads_the_unused_slots_with_zeros() {
    assert_eq!(
        fill(PK_SERIAL_NUMBER, &[packet(7)], 3),
        [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(fill(PK_SERIAL_NUMBER, &[], 0), Vec::<u8>::new());
}

#[test]
fn encode_all_lays_packets_out_in_order() {
    assert_eq!(
        encode_all(PK_SERIAL_NUMBER, &[packet(1), packet(2)]),
        [1, 0, 0, 0, 2, 0, 0, 0]
    );
}
