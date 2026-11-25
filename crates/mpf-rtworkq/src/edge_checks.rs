//! Edge-case tests for the ported rtworkq logic.
//!
//! These cover the boundaries of system queue routing, handle decoding, key
//! classification, and the serial queue's acknowledgement rules. Expectations that are
//! not oracle-derived say so in a comment.

use super::*;

fn slots_with(index: usize, slot: UserQueueSlot) -> [UserQueueSlot; 124] {
    let mut slots = [UserQueueSlot {
        refcount: 0,
        generation: 0,
    }; 124];
    slots[index] = slot;
    slots
}

fn ft(low: u32, high: u32) -> FileTime {
    FileTime { low, high }
}

fn submission(item: char, result: char, reply: Option<char>) -> Submission<char> {
    Submission {
        item,
        result,
        reply,
    }
}

#[test]
fn system_queue_boundaries() {
    // Not oracle-derived: the id switch in get_system_queue. Undefined ids fall back to
    // the standard queue, including 0 and 0xffff, which are below the private mask.
    for id in [0, 0x0000_0006, 0x0000_0008, 0xffff] {
        assert_eq!(system_queue(id), Some(SystemQueue::Standard), "id {id:#x}");
    }
    assert_eq!(system_queue(1), Some(SystemQueue::Standard));
    assert_eq!(system_queue(2), Some(SystemQueue::Rt));
    assert_eq!(system_queue(3), Some(SystemQueue::Io));
    assert_eq!(system_queue(4), Some(SystemQueue::Timer));
    assert_eq!(system_queue(5), Some(SystemQueue::Multithreaded));
    assert_eq!(system_queue(7), Some(SystemQueue::LongFunction));
}

#[test]
fn private_mask_marks_user_queues() {
    // Not oracle-derived: any bit in the high word makes the id a user queue handle.
    assert!(!is_user_queue(0));
    assert!(!is_user_queue(0xffff));
    for id in [0x0001_0000, 0x0005_0001, 0x8000_0003, 0xffff_ffff] {
        assert!(is_user_queue(id), "id {id:#x}");
        assert_eq!(system_queue(id), None, "id {id:#x}");
    }
}

#[test]
fn system_queue_type_per_variant() {
    // Not oracle-derived: the grab_queue switch lists IO, MULTITHREADED and LONG_FUNCTION.
    assert_eq!(
        system_queue_type(SystemQueue::Standard),
        RTWQ_STANDARD_WORKQUEUE
    );
    assert_eq!(system_queue_type(SystemQueue::Rt), RTWQ_STANDARD_WORKQUEUE);
    assert_eq!(
        system_queue_type(SystemQueue::Timer),
        RTWQ_STANDARD_WORKQUEUE
    );
    assert_eq!(
        system_queue_type(SystemQueue::Io),
        RTWQ_MULTITHREADED_WORKQUEUE
    );
    assert_eq!(
        system_queue_type(SystemQueue::Multithreaded),
        RTWQ_MULTITHREADED_WORKQUEUE
    );
    assert_eq!(
        system_queue_type(SystemQueue::LongFunction),
        RTWQ_MULTITHREADED_WORKQUEUE
    );
}

#[test]
fn system_queue_type_over_all_ids() {
    // Not oracle-derived: every id below the private mask, checked against the
    // grab_queue switch on the ids 3 (IO), 5 (MULTITHREADED) and 7 (LONG_FUNCTION).
    for id in 0..=0xffff_u32 {
        let queue = system_queue(id).expect("ids below the private mask are system queues");
        let expected = if matches!(id, 3 | 5 | 7) {
            RTWQ_MULTITHREADED_WORKQUEUE
        } else {
            RTWQ_STANDARD_WORKQUEUE
        };
        assert_eq!(system_queue_type(queue), expected, "id {id:#x}");
    }
}

#[test]
fn pool_max_threads_by_queue_type() {
    // Not oracle-derived: pool_queue_init only special-cases standard and window queues.
    assert_eq!(pool_max_threads(RTWQ_STANDARD_WORKQUEUE), 1);
    assert_eq!(pool_max_threads(RTWQ_WINDOW_WORKQUEUE), 1);
    assert_eq!(pool_max_threads(RTWQ_MULTITHREADED_WORKQUEUE), 4);
    assert_eq!(pool_max_threads(3), 4);
    assert_eq!(pool_max_threads(u32::MAX), 4);
}

#[test]
fn callback_priority_sign_boundaries() {
    // Not oracle-derived: the sign tests in pool_queue_submit at +1, -1 and the extremes.
    assert_eq!(callback_priority(1), CallbackPriority::High);
    assert_eq!(callback_priority(-1), CallbackPriority::Low);
    assert_eq!(callback_priority(i32::MIN), CallbackPriority::Low);
    assert_eq!(callback_priority(i32::MAX), CallbackPriority::High);
}

#[test]
fn due_time_boundaries() {
    // Not oracle-derived: values computed with python3 using wrapping 64-bit arithmetic.
    assert_eq!(due_time(0), ft(0, 0));
    assert_eq!(due_time(-1), ft(0xffff_d8f0, 0xffff_ffff));
    assert_eq!(due_time(500_000_000), ft(0x2739_5000, 0x0000_048c));
    assert_eq!(due_time(1 << 40), ft(0, 0x0027_1000));
    // 922337203685477 * 10^4 is the largest scaled value that fits in i64.
    assert_eq!(due_time(922_337_203_685_477), ft(0xffff_e950, 0x7fff_ffff));
    // One more step sets the sign bit; the port wraps, as the x86-64 build does.
    assert_eq!(due_time(922_337_203_685_478), ft(0x0000_1060, 0x8000_0000));
    assert_eq!(due_time(i64::MIN), ft(0, 0));
    // The oracle's excluded row: 2^60 * 10^4 is a multiple of 2^64, so it wraps to zero.
    assert_eq!(due_time(1 << 60), ft(0, 0));
}

#[test]
fn item_key_packs_mask_and_key() {
    // Not oracle-derived: get_item_key puts the mask in the high word.
    assert_eq!(item_key(0, 0), 0);
    assert_eq!(item_key(0x8000_0000, 1), 0x8000_0000_0000_0001);
    assert_eq!(item_key(u32::MAX, u32::MAX), u64::MAX);
}

#[test]
fn item_key_generator_starts_at_one_and_wraps() {
    // Not oracle-derived: InterlockedIncrement from zero, then wrapping past i32::MAX.
    let mut generator = ItemKeyGenerator::default();
    assert_eq!(generator.generate(0), 1);
    assert_eq!(generator.generate(0), 2);

    let mut generator = ItemKeyGenerator { next: i32::MAX };
    assert_eq!(generator.generate(0x8000_0000), 0x8000_0000_8000_0000);
    assert_eq!(generator.generate(0x8000_0000), 0x8000_0000_8000_0001);
}

#[test]
fn item_kind_high_word_boundaries() {
    // Not oracle-derived: the high-word masks in queue_cancel_item.
    assert_eq!(item_kind(0x8000_0000_0000_0000), ItemKind::Scheduled);
    assert_eq!(item_kind(0x8100_0000_0000_0000), ItemKind::Scheduled);
    assert_eq!(item_kind(0x8200_0000_0000_0000), ItemKind::Wait);
    assert_eq!(item_kind(0xffff_ffff_0000_0000), ItemKind::Wait);
    assert_eq!(item_kind(0x0200_0000_0000_0000), ItemKind::Unknown);
    // Only the high word is tested.
    assert_eq!(item_kind(0x0000_0000_8200_0000), ItemKind::Unknown);
}

#[test]
fn decode_handle_bounds() {
    // Not oracle-derived: slots run from 0 to 123, which are high words 5 to 128.
    assert_eq!(decode_user_queue_id(0x0000_0000), None);
    assert_eq!(decode_user_queue_id(0x0004_0000), None);
    assert_eq!(decode_user_queue_id(0x0005_0000), Some((0, 0)));
    assert_eq!(decode_user_queue_id(0x0005_ffff), Some((0, 0xffff)));
    assert_eq!(decode_user_queue_id(0x0080_0001), Some((123, 1)));
    assert_eq!(decode_user_queue_id(0x0081_0000), None);
    assert_eq!(decode_user_queue_id(0xffff_0000), None);
}

#[test]
fn encode_decode_round_trip() {
    // Not oracle-derived: the first and last handles, and every slot round-trips.
    assert_eq!(encode_user_queue_id(0, 1), 0x0005_0001);
    assert_eq!(encode_user_queue_id(123, 0xffff), 0x0080_ffff);
    for slot in 0..124 {
        assert_eq!(
            decode_user_queue_id(encode_user_queue_id(slot, 7)),
            Some((slot, 7))
        );
    }
}

#[test]
fn find_user_queue_liveness_and_generation() {
    // Not oracle-derived: get_queue_obj tests the refcount for nonzero and the
    // generation for equality.
    // A negative refcount is still nonzero, so the slot is live.
    let slots = slots_with(
        0,
        UserQueueSlot {
            refcount: -1,
            generation: 3,
        },
    );
    assert_eq!(find_user_queue(0x0005_0003, &slots), Some(0));

    let live = slots_with(
        123,
        UserQueueSlot {
            refcount: 1,
            generation: 7,
        },
    );
    assert_eq!(find_user_queue(0x0080_0007, &live), Some(123));
    assert_eq!(find_user_queue(0x0080_0006, &live), None);
    // A zero refcount is a free slot even when the generation matches.
    let freed = slots_with(
        123,
        UserQueueSlot {
            refcount: 0,
            generation: 7,
        },
    );
    assert_eq!(find_user_queue(0x0080_0007, &freed), None);
    // A table shorter than the slot index has no entry for it.
    assert_eq!(find_user_queue(0x0080_0007, &live[..10]), None);
    // An id that does not decode never reaches the table.
    assert_eq!(find_user_queue(0x0004_0000, &live), None);
}

#[test]
fn next_generation_steps() {
    // Not oracle-derived: ordinary increments below the wrap point.
    assert_eq!(next_queue_generation(0), 1);
    assert_eq!(next_queue_generation(1), 2);
    assert_eq!(next_queue_generation(0xfffd), 0xfffe);
}

#[test]
fn serial_ack_only_matches_the_head() {
    // Not oracle-derived: serial_queue_is_ack_token compares only against the head.
    let mut queue = SerialQueue::default();
    // A is dispatched and carries reply token 'r'. Queue: [A(r)].
    assert_eq!(queue.submit(submission('A', 'a', Some('r'))), Some('A'));
    // B carries reply token 'x' and waits behind A. Queue: [A(r), B(x)].
    assert_eq!(queue.submit(submission('B', 'b', Some('x'))), None);
    // C's result equals B's token, but B is not the head, so C is queued, not an ack.
    assert_eq!(queue.submit(submission('C', 'x', None)), None);
    // D acknowledges A: A is removed and B is dispatched. D is not queued.
    assert_eq!(queue.submit(submission('D', 'r', None)), Some('B'));
    // E acknowledges B: B is removed and C is dispatched.
    assert_eq!(queue.submit(submission('E', 'x', None)), Some('C'));
    assert_eq!(queue.finish('C'), None);
}

#[test]
fn serial_finish_removes_one_pending_item() {
    // Not oracle-derived. Wine finishes only the head; finishing B from the middle is a
    // model extension, and the model returns the head as the next item.
    let mut queue = SerialQueue::default();
    assert_eq!(queue.submit(submission('A', 'a', None)), Some('A'));
    assert_eq!(queue.submit(submission('B', 'b', None)), None);
    assert_eq!(queue.submit(submission('C', 'c', None)), None);
    // Queue: [A, C].
    assert_eq!(queue.finish('B'), Some('A'));
    // Queue: [C].
    assert_eq!(queue.finish('A'), Some('C'));
    // Not pending: the queue is unchanged.
    assert_eq!(queue.finish('Z'), Some('C'));
    assert_eq!(queue.finish('C'), None);
}

#[test]
fn serial_regular_head_is_never_acknowledged() {
    // Not oracle-derived: a regular item has a NULL reply_result, and results are never
    // NULL, so a regular head cannot match any result.
    let mut queue = SerialQueue::default();
    assert_eq!(queue.submit(submission('A', 'a', None)), Some('A'));
    // B's result equals A's result, not a reply token, so B is queued.
    assert_eq!(queue.submit(submission('B', 'a', None)), None);
    assert_eq!(queue.finish('A'), Some('B'));
}

#[test]
fn serial_ack_with_nothing_queued_behind() {
    // Not oracle-derived: an ack that empties the queue dispatches nothing.
    let mut queue = SerialQueue::default();
    assert_eq!(queue.submit(submission('A', 'a', Some('r'))), Some('A'));
    assert_eq!(queue.submit(submission('C', 'r', None)), None);
    // The queue is empty again, so the next submission is dispatched at once.
    assert_eq!(queue.submit(submission('D', 'd', None)), Some('D'));
}

#[test]
fn derived_impls_are_callable() {
    // Calls the derived impls directly; llvm-cov counts them as functions. Not
    // oracle-derived: these only check the Clone and Debug output shape.
    let queue = SystemQueue::Io;
    assert_eq!(Clone::clone(&queue), SystemQueue::Io);
    assert!(format!("{queue:?}").contains("Io"));

    let priority = CallbackPriority::Low;
    assert_eq!(Clone::clone(&priority), CallbackPriority::Low);
    assert!(format!("{priority:?}").contains("Low"));

    let kind = ItemKind::Scheduled;
    assert_eq!(Clone::clone(&kind), ItemKind::Scheduled);
    assert!(format!("{kind:?}").contains("Scheduled"));

    let time = ft(1, 2);
    assert_eq!(Clone::clone(&time), time);
    assert!(format!("{time:?}").contains("high: 2"));

    let slot = UserQueueSlot {
        refcount: 1,
        generation: 4,
    };
    assert_eq!(Clone::clone(&slot), slot);
    assert!(format!("{slot:?}").contains("generation: 4"));
}
