//! Per-thread COM state subset from Wine's `dlls/ole32/compobj.c`.
//!
//! `CoSetState` retains a reference to the supplied object, and `CoGetState`
//! returns another reference to the current thread's stored object. `Rc`
//! represents COM reference ownership without imposing `Send`/`Sync` on
//! apartment-bound objects. This does not expose the `IUnknown` ABI or model
//! failure to allocate Wine's COM TLS record.

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static STATE: RefCell<Option<Rc<dyn Any>>> = RefCell::new(None);
}

/// Replaces this thread's state object, retaining the new value and releasing
/// the old value when its final `Rc` is dropped. `None` clears the state.
pub fn co_set_state(state: Option<Rc<dyn Any>>) {
    STATE.with(|slot| *slot.borrow_mut() = state);
}

/// Returns a retained reference to this thread's state object, if any.
pub fn co_get_state() -> Option<Rc<dyn Any>> {
    STATE.with(|slot| slot.borrow().clone())
}

/// Compares two optional GUID byte arrays, with `None` representing the nil GUID.
pub fn is_equal_guid(a: Option<&[u8; 16]>, b: Option<&[u8; 16]>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, None) => true,
        (Some(a), None) => a.iter().all(|byte| *byte == 0),
        (None, Some(b)) => b.iter().all(|byte| *byte == 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_starts_empty_and_can_be_cleared() {
        co_set_state(None);
        assert!(co_get_state().is_none());
        co_set_state(Some(Rc::new(String::from("state"))));
        assert_eq!(
            co_get_state().unwrap().downcast_ref::<String>().unwrap(),
            "state"
        );
        co_set_state(None);
        assert!(co_get_state().is_none());
    }

    #[test]
    fn get_returns_a_retained_reference() {
        co_set_state(None);
        let value = Rc::new(String::from("retained"));
        co_set_state(Some(value.clone()));
        let fetched = co_get_state().unwrap();
        co_set_state(None);
        assert_eq!(fetched.downcast_ref::<String>().unwrap(), "retained");
        assert_eq!(Rc::strong_count(&value), 2);
    }

    #[test]
    fn replacing_state_releases_previous_slot_reference() {
        co_set_state(None);
        let first = Rc::new(String::from("first"));
        co_set_state(Some(first.clone()));
        let second = Rc::new(String::from("second"));
        co_set_state(Some(second.clone()));
        assert_eq!(Rc::strong_count(&first), 1);
        assert_eq!(
            co_get_state().unwrap().downcast_ref::<String>().unwrap(),
            "second"
        );
    }

    #[test]
    fn state_is_thread_local() {
        co_set_state(Some(Rc::new(String::from("parent"))));
        let child_state = std::thread::spawn(|| {
            assert!(co_get_state().is_none());
            co_set_state(Some(Rc::new(String::from("child"))));
            co_get_state()
                .unwrap()
                .downcast_ref::<String>()
                .unwrap()
                .clone()
        })
        .join()
        .unwrap();
        assert_eq!(child_state, "child");
        assert_eq!(
            co_get_state().unwrap().downcast_ref::<String>().unwrap(),
            "parent"
        );
        co_set_state(None);
    }

    #[test]
    fn equal_guid_handles_nil_and_identity() {
        let zero = [0; 16];
        let other = [1; 16];
        assert!(is_equal_guid(None, None));
        assert!(is_equal_guid(Some(&zero), None));
        assert!(is_equal_guid(None, Some(&zero)));
        assert!(is_equal_guid(Some(&zero), Some(&zero)));
        assert!(!is_equal_guid(Some(&other), None));
        assert!(!is_equal_guid(None, Some(&other)));
        assert!(!is_equal_guid(Some(&zero), Some(&other)));
    }
}
