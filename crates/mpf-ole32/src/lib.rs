//! Per-thread COM state subset from Wine's `dlls/ole32/compobj.c`.
//!
//! `CoSetState` retains a reference to the supplied object, and `CoGetState`
//! returns another reference to the current thread's stored object. `Rc`
//! represents COM reference ownership without imposing `Send`/`Sync` on
//! apartment-bound objects. This does not expose the `IUnknown` ABI or model
//! failure to allocate Wine's COM TLS record.

use mpf_rpcrt4::Uuid;
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

/// Units in a braced GUID, including its terminator.
const CHARS_IN_GUID: usize = 39;

/// The two failure codes of `IIDFromString`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IidStringError {
    /// `E_INVALIDARG`: the text is not 38 units long before its terminator.
    InvalidArg,
    /// `CO_E_IIDSTRING`: the text has the right length but is not a braced GUID.
    IidString,
}

/// `CLSIDFromString` for the braced form. A null `s` gives the nil CLSID. Wine
/// falls back to a ProgID lookup in the registry, which this port does not
/// model, so a ProgID returns `None`.
pub fn clsid_from_string(s: Option<&[u16]>) -> Option<Uuid> {
    match s {
        None => Some(Uuid::default()),
        Some(s) => guid_from_braced(s),
    }
}

/// `IIDFromString`: parses a braced GUID that is exactly 38 units long. A null
/// `s` gives the nil IID.
pub fn iid_from_string(s: Option<&[u16]>) -> Result<Uuid, IidStringError> {
    let Some(s) = s else {
        return Ok(Uuid::default());
    };
    if wlen(s) != CHARS_IN_GUID - 1 {
        return Err(IidStringError::InvalidArg);
    }
    guid_from_braced(s).ok_or(IidStringError::IidString)
}

/// `StringFromGUID2`: writes `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` in uppercase
/// hex, followed by a terminator. Returns the 39 units written, or 0 for a null
/// `guid` or a `dst` shorter than 39 units.
pub fn string_from_guid2(guid: Option<&Uuid>, dst: &mut [u16]) -> usize {
    let Some(guid) = guid else {
        return 0;
    };
    if dst.len() < CHARS_IN_GUID {
        return 0;
    }
    let d = guid.data4;
    let text = format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    );
    let units: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    dst[..CHARS_IN_GUID].copy_from_slice(&units);
    CHARS_IN_GUID
}

/// `combase`'s `guid_from_string` for `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`,
/// with hex digits in either case. The NUL must follow the closing brace, and
/// text after that NUL is ignored.
fn guid_from_braced(s: &[u16]) -> Option<Uuid> {
    let text = s.get(..CHARS_IN_GUID)?;
    if text[0] != u16::from(b'{') || text[37] != u16::from(b'}') || text[38] != 0 {
        return None;
    }
    let hex = |start: usize, count: usize| {
        text[start..start + count].iter().try_fold(0u64, |acc, &c| {
            let digit = char::from_u32(u32::from(c))?.to_digit(16)?;
            Some((acc << 4) | u64::from(digit))
        })
    };
    let dash = |index: usize| (text[index] == u16::from(b'-')).then_some(());
    let data1 = hex(1, 8)? as u32;
    dash(9)?;
    let data2 = hex(10, 4)? as u16;
    dash(14)?;
    let data3 = hex(15, 4)? as u16;
    dash(19)?;
    dash(24)?;
    let mut data4 = [0u8; 8];
    for (k, byte) in data4.iter_mut().enumerate() {
        let at = 20 + 2 * k + usize::from(k >= 2);
        *byte = hex(at, 2)? as u8;
    }
    Some(Uuid {
        data1,
        data2,
        data3,
        data4,
    })
}

fn wlen(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn sample() -> Uuid {
        Uuid {
            data1: 0x0102_0304,
            data2: 0x0506,
            data3: 0x0708,
            data4: [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x0a],
        }
    }

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

    #[test]
    fn clsid_from_string_parses_braced_text_in_either_case() {
        assert_eq!(
            clsid_from_string(Some(&w("{01020304-0506-0708-090A-0B0C0D0E0F0A}"))),
            Some(sample())
        );
        assert_eq!(
            clsid_from_string(Some(&w("{ABCDEF01-2345-6789-ABCD-EF0123456789}"))),
            Some(Uuid {
                data1: 0xabcd_ef01,
                data2: 0x2345,
                data3: 0x6789,
                data4: [0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89],
            })
        );
        let mut trailing = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
        trailing.extend("junk".encode_utf16());
        assert_eq!(clsid_from_string(Some(&trailing)), Some(sample()));
        assert_eq!(clsid_from_string(None), Some(Uuid::default()));
    }

    #[test]
    fn clsid_from_string_rejects_non_braced_and_malformed_text() {
        let cases = [
            "",
            "01020304-0506-0708-090A-0B0C0D0E0F0A}",
            "{01020304-0506-0708-090A-0B0C0D0E0F0A",
            "(01020304-0506-0708-090A-0B0C0D0E0F0A}",
            "{01020304-0506-0708-090A-0B0C0D0E0F0A)",
            "{01020304-0506-0708-090A-0B0C0D0E0F0A}x",
            "{0102030G-0506-0708-090A-0B0C0D0E0F0A}",
            "{01020304_0506-0708-090A-0B0C0D0E0F0A}",
            "{01020304-050G-0708-090A-0B0C0D0E0F0A}",
            "{01020304-0506_0708-090A-0B0C0D0E0F0A}",
            "{01020304-0506-070G-090A-0B0C0D0E0F0A}",
            "{01020304-0506-0708_090A-0B0C0D0E0F0A}",
            "{01020304-0506-0708-090A_0B0C0D0E0F0A}",
            "{01020304-0506-0708-G90A-0B0C0D0E0F0A}",
            "{01020304-0506-0708-090A-0B0C0D0E0F0G}",
            "Interface.Name",
        ];
        for input in cases {
            assert_eq!(clsid_from_string(Some(&w(input))), None, "{input:?}");
        }
        let no_terminator = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
        assert_eq!(clsid_from_string(Some(&no_terminator[..38])), None);
    }

    #[test]
    fn guid_parser_rejects_lone_surrogate_units_in_digits() {
        let mut text = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
        text[1] = 0xd800;
        assert_eq!(clsid_from_string(Some(&text)), None);
    }

    #[test]
    fn iid_from_string_reports_length_and_format_failures_separately() {
        assert_eq!(iid_from_string(None), Ok(Uuid::default()));
        assert_eq!(
            iid_from_string(Some(&w("{01020304-0506-0708-090A-0B0C0D0E0F0A}"))),
            Ok(sample())
        );
        assert_eq!(
            iid_from_string(Some(&w("{01020304-0506-0708-090A-0B0C0D0E0F0A}x"))),
            Err(IidStringError::InvalidArg)
        );
        assert_eq!(
            iid_from_string(Some(&w("{01020304-0506-0708-090A-0B0C0D0E0F0A"))),
            Err(IidStringError::InvalidArg)
        );
        assert_eq!(
            iid_from_string(Some(&w("(01020304-0506-0708-090A-0B0C0D0E0F0A}"))),
            Err(IidStringError::IidString)
        );
        assert_eq!(
            iid_from_string(Some(&w("{01020304-0506-0708-090A-0B0C0D0E0F0G}"))),
            Err(IidStringError::IidString)
        );
        let no_terminator = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
        assert_eq!(
            iid_from_string(Some(&no_terminator[..38])),
            Err(IidStringError::IidString)
        );
    }

    #[test]
    fn string_from_guid2_writes_uppercase_braced_text() {
        let mut buf = [0xffffu16; 40];
        assert_eq!(string_from_guid2(Some(&sample()), &mut buf), 39);
        assert_eq!(buf[..39], w("{01020304-0506-0708-090A-0B0C0D0E0F0A}")[..]);
        assert_eq!(buf[39], 0xffff);

        let lower = clsid_from_string(Some(&w("{abcdef01-2345-6789-abcd-ef0123456789}")));
        assert_eq!(string_from_guid2(lower.as_ref(), &mut buf), 39);
        assert_eq!(buf[..39], w("{ABCDEF01-2345-6789-ABCD-EF0123456789}")[..]);

        assert_eq!(string_from_guid2(None, &mut buf), 0);
        assert_eq!(string_from_guid2(Some(&sample()), &mut buf[..38]), 0);
    }
}
