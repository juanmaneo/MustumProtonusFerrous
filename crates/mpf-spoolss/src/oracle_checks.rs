use super::{alloc_spl_str, winspool_table_ready, TABLE_LEN};

type AllocationRow<'a> = (Option<&'a [u16]>, Option<&'a [u16]>);

#[test]
fn matches_twenty_one_sanitized_wine_oracle_rows() {
    let table_rows = [
        (false, 0x1ff, false),
        (true, 0x1ff, true),
        (true, 0x000, false),
        (true, 0x1fe, false),
        (true, 0x1fd, false),
        (true, 0x1fb, false),
        (true, 0x1f7, false),
        (true, 0x1ef, false),
        (true, 0x1df, false),
        (true, 0x1bf, false),
        (true, 0x17f, false),
        (true, 0x0ff, false),
    ];
    for (loaded, mask, expected) in table_rows {
        let slots = core::array::from_fn(|index| mask & (1 << index) != 0);
        assert_eq!(winspool_table_ready(loaded, slots), expected);
    }

    let allocation_rows: [AllocationRow<'_>; 9] = [
        (None, None),
        (Some(&[0]), Some(&[0])),
        (Some(&[65, 0]), Some(&[65, 0])),
        (Some(&[65, 66, 67, 0]), Some(&[65, 66, 67, 0])),
        (Some(&[65, 0, 66, 0]), Some(&[65, 0])),
        (Some(&[0x03a9, 0]), Some(&[0x03a9, 0])),
        (Some(&[0xd83d, 0xde00, 0]), Some(&[0xd83d, 0xde00, 0])),
        (Some(&[65, 0, 66, 67, 0]), Some(&[65, 0])),
        (Some(&[0xffff, 0]), Some(&[0xffff, 0])),
    ];
    for (input, expected) in allocation_rows {
        assert_eq!(alloc_spl_str(input).as_deref(), expected);
    }

    assert_eq!(table_rows.len() + allocation_rows.len(), 21);
    assert_eq!(TABLE_LEN, 9);
}
