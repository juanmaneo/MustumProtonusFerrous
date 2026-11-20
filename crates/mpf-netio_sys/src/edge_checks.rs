use super::pending::{find_wait, select_slot, Slot};

#[test]
fn empty_and_busy_slot_slices_have_no_selection() {
    assert_eq!(select_slot::<char>(&[], &'A'), None);
    assert_eq!(
        select_slot(
            &[Slot {
                in_use: true,
                callback: Some('A'),
            }],
            &'A'
        ),
        None
    );
}

#[test]
fn a_free_slot_without_callback_is_selected() {
    assert_eq!(
        select_slot(
            &[
                Slot {
                    in_use: true,
                    callback: Some('A'),
                },
                Slot {
                    in_use: false,
                    callback: None,
                },
            ],
            &'B'
        ),
        Some(1)
    );
}

#[test]
fn duplicate_waits_choose_the_first_and_empty_waits_do_not_match() {
    let waits = [Some(4), None, Some(4)];
    assert_eq!(find_wait(&waits, &4), Some(0));
    assert_eq!(find_wait(&waits, &5), None);
    assert_eq!(find_wait::<u8>(&[], &4), None);
}
