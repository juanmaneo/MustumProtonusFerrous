//! Oracle-derived test vectors for the ported rtworkq logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/rtworkq/queue.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/rtworkq/` and running the harness. It prints 24 rows. UBSan flags
//! one of them, `due_overflow` (`timeout * 1000 * 10` with timeout 2^60), so that row
//! is excluded and 23 vectors remain. `edge_checks.rs` covers the wrapped result and
//! marks it as not oracle-derived.

use super::*;

/// The slot table as the harness leaves it: slot 1 freed at generation 2, slot 2 live
/// at generation 9, and every other slot zeroed.
fn harness_slots() -> [UserQueueSlot; 124] {
    let mut slots = [UserQueueSlot {
        refcount: 0,
        generation: 0,
    }; 124];
    slots[1] = UserQueueSlot {
        refcount: 0,
        generation: 2,
    };
    slots[2] = UserQueueSlot {
        refcount: 1,
        generation: 9,
    };
    slots
}

/// The harness prints a system queue as its `system_queues` index, or -1 for none.
fn sys_index(queue: Option<SystemQueue>) -> i32 {
    queue.map_or(-1, |q| q as i32)
}

/// The harness prints each dispatch as the item name, or `-` for none.
fn render(dispatches: &[Option<char>]) -> String {
    dispatches
        .iter()
        .map(|d| d.unwrap_or('-').to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn submission(item: char, result: char, reply: Option<char>) -> Submission<char> {
    Submission {
        item,
        result,
        reply,
    }
}

#[test]
fn oracle_item_key_rows() {
    // gen_wait_first | generate_item_key(0x82000000), call 1
    assert_eq!(
        ItemKeyGenerator::default().generate(0x8200_0000),
        0x8200_0000_0000_0001
    );
    // kind_wait_precedence | key=0xc200000000000001
    assert_eq!(item_kind(0xc200_0000_0000_0001), ItemKind::Wait);
    // kind_unknown | key=0x0000000000000005
    assert_eq!(item_kind(0x0000_0000_0000_0005), ItemKind::Unknown);
}

#[test]
fn oracle_system_queue_rows() {
    // sys_io | get_system_queue(3) -> index 2
    assert_eq!(sys_index(system_queue(3)), 2);
    // sys_do_not_use | get_system_queue(6) -> index 0
    assert_eq!(sys_index(system_queue(6)), 0);
    // sys_long_function | get_system_queue(7) -> index 6
    assert_eq!(sys_index(system_queue(7)), 6);
    // sys_user_queue | get_system_queue(0x00050001) -> none
    assert_eq!(sys_index(system_queue(0x0005_0001)), -1);
    // sys_type_io | system queue id 3 -> multithreaded
    assert_eq!(
        system_queue(3).map(system_queue_type),
        Some(RTWQ_MULTITHREADED_WORKQUEUE)
    );
}

#[test]
fn oracle_user_queue_rows() {
    let slots = harness_slots();
    // lookup_live | id=0x00070009, slot 2 refcount 1 gen 9 -> slot 2
    assert_eq!(find_user_queue(0x0007_0009, &slots), Some(2));
    // lookup_stale_generation | id=0x00070008 -> none
    assert_eq!(find_user_queue(0x0007_0008, &slots), None);
    // lookup_freed_slot | id=0x00060002, slot 1 refcount 0 -> none
    assert_eq!(find_user_queue(0x0006_0002, &slots), None);
    // encode_slot2_gen9 | entry=user_queues[2] -> 0x00070009
    assert_eq!(encode_user_queue_id(2, 9), 0x0007_0009);
    // generation_wrap | 0xfffe -> 0x0001
    assert_eq!(next_queue_generation(0xfffe), 0x0001);
    // generation_unreachable_ffff | 0xffff -> 0x0000 (the WORD wraps before the compare)
    assert_eq!(next_queue_generation(0xffff), 0x0000);
}

#[test]
fn oracle_priority_and_thread_rows() {
    // prio_zero | priority=0 -> NORMAL
    assert_eq!(callback_priority(0), CallbackPriority::Normal);
    // prio_negative | priority=-5 -> LOW
    assert_eq!(callback_priority(-5), CallbackPriority::Low);
    // prio_positive | priority=7 -> HIGH
    assert_eq!(callback_priority(7), CallbackPriority::High);
    // threads_window | queue_type=1 -> 1
    assert_eq!(pool_max_threads(RTWQ_WINDOW_WORKQUEUE), 1);
}

#[test]
fn oracle_due_time_rows() {
    // due_one_second | timeout=1000 -> low=0x00989680 high=0x00000000
    assert_eq!(
        due_time(1000),
        FileTime {
            low: 0x0098_9680,
            high: 0x0000_0000
        }
    );
    // due_negative | timeout=-1000 -> low=0xff676980 high=0xffffffff
    assert_eq!(
        due_time(-1000),
        FileTime {
            low: 0xff67_6980,
            high: 0xffff_ffff
        }
    );
}

#[test]
fn oracle_serial_queue_rows() {
    // serial_plain_fifo | submit A, submit B, finish A, finish B -> A - B -
    let mut queue = SerialQueue::default();
    let steps = [
        queue.submit(submission('A', 'a', None)),
        queue.submit(submission('B', 'b', None)),
        queue.finish('A'),
        queue.finish('B'),
    ];
    assert_eq!(render(&steps), "A - B -");

    // serial_reply_ack | submit A (reply), submit B, ack C with A's reply, finish B -> A - B -
    let mut queue = SerialQueue::default();
    let steps = [
        queue.submit(submission('A', 'a', Some('r'))),
        queue.submit(submission('B', 'b', None)),
        queue.submit(submission('C', 'r', None)),
        queue.finish('B'),
    ];
    assert_eq!(render(&steps), "A - B -");

    // serial_ack_idle | submit A (reply), finish A, ack B with A's reply on empty queue, finish B -> A - B -
    let mut queue = SerialQueue::default();
    let steps = [
        queue.submit(submission('A', 'a', Some('r'))),
        queue.finish('A'),
        queue.submit(submission('B', 'r', None)),
        queue.finish('B'),
    ];
    assert_eq!(render(&steps), "A - B -");
}
