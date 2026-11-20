//! Edge cases that the oracle rows do not reach: an empty table, the largest id, a cursor past the
//! end, a payload that owns memory, and a stored zero magic, which Wine cannot tell from a missing
//! cookie.

use crate::CookieTable;

#[test]
fn empty_table_has_no_cookies() {
    let mut table: CookieTable<usize> = CookieTable::default();
    assert_eq!(table.get_magic(1), None);
    assert_eq!(table.get_data(1), None);
    assert_eq!(table.remove(1), None);
    let mut index = 0;
    assert_eq!(table.enumerate(0x10, &mut index), 0);
    assert_eq!(index, 0);
}

#[test]
fn largest_id_is_missing() {
    let mut table = CookieTable::new();
    table.generate(0x10, 7usize);
    assert_eq!(table.get_magic(u32::MAX), None);
    assert_eq!(table.get_data(u32::MAX), None);
    assert_eq!(table.remove(u32::MAX), None);
}

#[test]
fn zero_id_is_missing_on_empty_table() {
    let mut table: CookieTable<usize> = CookieTable::new();
    assert_eq!(table.get_magic(0), None);
    assert_eq!(table.get_data(0), None);
    assert_eq!(table.remove(0), None);
}

#[test]
fn generated_ids_are_never_zero() {
    let mut table = CookieTable::new();
    for i in 0..100usize {
        assert_ne!(table.generate(0x10, i), 0);
    }
}

#[test]
fn removed_id_cannot_be_removed_twice() {
    let mut table = CookieTable::new();
    let id = table.generate(0x10, 5usize);
    assert_eq!(table.remove(id), Some(5));
    assert_eq!(table.remove(id), None);
    assert_eq!(table.generate(0x20, 6), id);
    assert_eq!(table.get_magic(id), Some(0x20));
}

#[test]
fn cursor_past_the_end_is_left_alone() {
    let mut table = CookieTable::new();
    table.generate(0x10, 1usize);
    let mut index = u32::MAX;
    assert_eq!(table.enumerate(0x10, &mut index), 0);
    assert_eq!(index, u32::MAX);
}

#[test]
fn owned_payload_is_moved_out_on_remove() {
    let mut table = CookieTable::new();
    let id = table.generate(0x10, String::from("sink"));
    assert_eq!(table.get_data(id).map(String::as_str), Some("sink"));
    assert_eq!(table.remove(id), Some(String::from("sink")));
    assert_eq!(table.get_data(id), None);
}

#[test]
fn stored_zero_magic_reads_as_present() {
    let mut table = CookieTable::new();
    let id = table.generate(0, 9usize);
    assert_eq!(table.get_magic(id), Some(0));
    let mut index = 0;
    assert_eq!(table.enumerate(0, &mut index), id);
}
