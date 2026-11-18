//! Property storage of NCrypt objects, and the argument rules of NCryptGetProperty and NCryptSetProperty.

use crate::wide::{wide_eq, wide_str};
use crate::{
    Status, ERROR_SUCCESS, NTE_BUFFER_TOO_SMALL, NTE_INVALID_HANDLE, NTE_INVALID_PARAMETER,
    NTE_NOT_SUPPORTED,
};

pub const ALGORITHM_PROPERTY: &str = "Algorithm Name";
pub const ALGORITHM_GROUP_PROPERTY: &str = "Algorithm Group";
pub const LENGTHS_PROPERTY: &str = "Lengths";
pub const BLOCK_LENGTH_PROPERTY: &str = "Block Length";
pub const SIGNATURE_LENGTH_PROPERTY: &str = "SignatureLength";
pub const EXPORT_POLICY_PROPERTY: &str = "Export Policy";
pub const KEY_USAGE_PROPERTY: &str = "Key Usage";
pub const KEY_TYPE_PROPERTY: &str = "Key Type";
pub const LENGTH_PROPERTY: &str = "Length";
pub const PUBLIC_KEY_LENGTH_PROPERTY: &str = "PublicKeyLength";
pub const PROVIDER_HANDLE_PROPERTY: &str = "Provider Handle";

/// The named values of one object. Names compare up to their first NUL.
#[derive(Default)]
pub struct PropertyStore {
    entries: Vec<(Vec<u16>, Vec<u8>)>,
}

impl PropertyStore {
    /// Stores the value under the name, replacing an earlier value with the same name.
    pub fn set(&mut self, name: &[u16], value: &[u8]) {
        match self.position(name) {
            Some(index) => self.entries[index].1 = value.to_vec(),
            None => self.entries.push((wide_str(name).to_vec(), value.to_vec())),
        }
    }

    pub fn get(&self, name: &[u16]) -> Option<&[u8]> {
        self.position(name)
            .map(|index| self.entries[index].1.as_slice())
    }

    fn position(&self, name: &[u16]) -> Option<usize> {
        self.entries
            .iter()
            .position(|(key, _)| key.as_slice() == wide_str(name))
    }
}

/// The outcome of a property read: the size reported to the caller, the status and the bytes copied out.
pub struct PropertyRead<'a> {
    pub size: Option<usize>,
    pub status: Status,
    pub value: &'a [u8],
}

/// NCryptGetProperty over an optional object. A missing object is an invalid handle and a missing name is an invalid parameter.
/// Without an output buffer only the size is reported. A buffer shorter than the value is refused.
pub fn get_property<'a>(
    store: Option<&'a PropertyStore>,
    name: &[u16],
    output_len: Option<usize>,
) -> PropertyRead<'a> {
    let Some(store) = store else {
        return PropertyRead {
            size: None,
            status: NTE_INVALID_HANDLE,
            value: &[],
        };
    };
    let Some(value) = store.get(name) else {
        return PropertyRead {
            size: None,
            status: NTE_INVALID_PARAMETER,
            value: &[],
        };
    };
    let size = Some(value.len());
    match output_len {
        None => PropertyRead {
            size,
            status: ERROR_SUCCESS,
            value: &[],
        },
        Some(len) if len < value.len() => PropertyRead {
            size,
            status: NTE_BUFFER_TOO_SMALL,
            value: &[],
        },
        Some(_) => PropertyRead {
            size,
            status: ERROR_SUCCESS,
            value,
        },
    }
}

/// NCryptSetProperty over an optional object. The Provider Handle property cannot be set.
pub fn set_property(store: Option<&mut PropertyStore>, name: &[u16], value: &[u8]) -> Status {
    let Some(store) = store else {
        return NTE_INVALID_HANDLE;
    };
    if wide_eq(name, PROVIDER_HANDLE_PROPERTY) {
        return NTE_NOT_SUPPORTED;
    }
    store.set(name, value);
    ERROR_SUCCESS
}
