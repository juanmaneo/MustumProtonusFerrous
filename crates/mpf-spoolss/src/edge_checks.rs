use super::{alloc_spl_str, winspool_table_ready, TABLE_LEN};

#[test]
fn allocation_handles_null_and_terminator_edges() {
    assert_eq!(alloc_spl_str(None), None);
    assert_eq!(alloc_spl_str(Some(&[0])), Some(vec![0]));
    assert_eq!(alloc_spl_str(Some(&[0x03a9, 0])), Some(vec![0x03a9, 0]));
    assert_eq!(alloc_spl_str(Some(&[65, 0, 66, 0])), Some(vec![65, 0]));
}

#[test]
#[should_panic(expected = "src must be NUL-terminated")]
fn allocation_requires_a_terminator() {
    let _ = alloc_spl_str(Some(&[65, 66]));
}

#[test]
fn table_readiness_requires_load_and_every_slot() {
    assert_eq!(TABLE_LEN, 9);
    assert!(!winspool_table_ready(false, [true; TABLE_LEN]));
    assert!(winspool_table_ready(true, [true; TABLE_LEN]));
    assert!(!winspool_table_ready(true, [false; TABLE_LEN]));
}
