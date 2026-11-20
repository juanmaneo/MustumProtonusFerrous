//! Null-pointer and copy-isolation checks for the Wine method semantics.

use crate::{TransactionOptions, XactOptions, DESCRIPTION_CAPACITY, E_INVALIDARG, S_OK};

fn options(timeout: u32, byte: u8) -> XactOptions {
    XactOptions {
        timeout,
        description: [byte; DESCRIPTION_CAPACITY],
    }
}

#[test]
fn set_rejects_null_without_changing_state() {
    let initial = options(0x1234_5678, 0xa5);
    let mut state = TransactionOptions::new(initial);
    assert_eq!(state.set_options(None), E_INVALIDARG);

    let mut fetched = options(0, 0);
    assert_eq!(state.get_options(Some(&mut fetched)), S_OK);
    assert_eq!(fetched, initial);
}

#[test]
fn get_rejects_null_destination() {
    let state = TransactionOptions::new(options(u32::MAX, 0xff));
    assert_eq!(state.get_options(None), E_INVALIDARG);
}

#[test]
fn set_and_get_copy_all_description_bytes_independently() {
    let original = XactOptions {
        timeout: u32::MAX,
        description: core::array::from_fn(|index| index as u8),
    };
    let mut state = TransactionOptions::new(options(0, 0));
    assert_eq!(state.set_options(Some(&original)), S_OK);

    let mut fetched = options(0, 0x5a);
    assert_eq!(state.get_options(Some(&mut fetched)), S_OK);
    assert_eq!(fetched, original);

    fetched.description[0] ^= 0xff;
    assert_eq!(state.get_options(Some(&mut fetched)), S_OK);
    assert_eq!(fetched, original);
}
