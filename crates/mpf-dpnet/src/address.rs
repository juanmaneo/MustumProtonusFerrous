//! Component store behind `IDirectPlay8Address`: `AddComponent` validation and
//! upsert, `GetURLW` text, and the lookups by name and by index.

use crate::{
    Guid, DPNA_DATATYPE_BINARY, DPNA_DATATYPE_DWORD, DPNA_DATATYPE_GUID, DPNA_DATATYPE_STRING,
    DPNA_DATATYPE_STRING_ANSI,
};

/// `DPNA_HEADER`, the prefix of every URL.
const HEADER: &str = "x-directplay:/";

/// Typed payload of one component. `None` is an unknown type: the C code keeps
/// its size and type but copies no data.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    None,
    Dword(u32),
    Guid(Guid),
    /// UTF-16LE bytes including the terminating NUL.
    String(Vec<u8>),
    /// Bytes including the terminating NUL.
    Ansi(Vec<u8>),
    Binary(Vec<u8>),
}

impl Data {
    /// The bytes `GetComponentByName` and `GetComponentByIndex` copy out.
    fn bytes(&self) -> Vec<u8> {
        match self {
            Data::None => Vec::new(),
            Data::Dword(value) => value.to_le_bytes().to_vec(),
            Data::Guid(guid) => guid.to_le_bytes().to_vec(),
            Data::String(bytes) | Data::Ansi(bytes) | Data::Binary(bytes) => bytes.clone(),
        }
    }
}

/// `struct component`. The name is stored up to its first NUL.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Component {
    name: Vec<u16>,
    datatype: u32,
    size: usize,
    data: Data,
}

impl Component {
    fn retrieve(&self) -> Retrieved {
        Retrieved {
            datatype: self.datatype,
            size: self.size,
            bytes: self.data.bytes(),
        }
    }

    /// Appends `name=value` as `GetURLW` formats it. Binary and unknown types
    /// append nothing, as in the C `default:` branch.
    fn render(&self, out: &mut Vec<u16>) {
        let value: Vec<u16> = match &self.data {
            Data::None | Data::Binary(_) => return,
            Data::Dword(value) => (*value as i32).to_string().encode_utf16().collect(),
            Data::Guid(guid) => guid_text(guid).encode_utf16().collect(),
            Data::String(bytes) => utf16_until_nul(bytes),
            Data::Ansi(bytes) => bytes
                .iter()
                .take_while(|&&byte| byte != 0)
                .map(|&byte| u16::from(byte))
                .collect(),
        };
        out.extend_from_slice(&self.name);
        out.push(u16::from(b'='));
        out.extend_from_slice(&value);
    }
}

/// `%7B`-escaped GUID text as `GetURLW` writes it.
fn guid_text(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "%7B{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}%7D",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

fn utf16_until_nul(bytes: &[u8]) -> Vec<u16> {
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|unit| u16::from_le_bytes(*unit))
        .take_while(|&unit| unit != 0)
        .collect()
}

/// The part of a name before its first NUL, as `wcsdup` and `lstrcmpW` see it.
fn until_nul(units: &[u16]) -> &[u16] {
    match units.iter().position(|&unit| unit == 0) {
        Some(end) => &units[..end],
        None => units,
    }
}

/// The component list of one `IDirectPlay8Address`, in insertion order.
#[derive(Debug, Default)]
pub struct Address {
    components: Vec<Component>,
}

/// Failures of `AddComponent`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddError {
    /// `lpvData` is NULL (`DPNERR_INVALIDPOINTER`).
    InvalidPointer,
    /// The size does not fit the declared type (`DPNERR_INVALIDPARAM`).
    InvalidParam,
}

/// Failures of `GetComponentByName`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameError {
    DoesNotExist,
    /// `size` is the length the caller needs (`DPNERR_BUFFERTOOSMALL`).
    BufferTooSmall {
        size: usize,
    },
}

/// Failures of `GetComponentByIndex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexError {
    DoesNotExist,
    /// `name_len` counts the terminating NUL, as the C output does.
    BufferTooSmall {
        name_len: usize,
        size: usize,
        datatype: u32,
    },
}

/// Output of a successful or size-reporting component lookup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Retrieved {
    pub datatype: u32,
    pub size: usize,
    /// The bytes copied out. Empty for unknown types, which copy nothing.
    pub bytes: Vec<u8>,
}

impl Address {
    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    /// `AddComponent`. A component with the same name is replaced in place;
    /// otherwise the component is appended.
    pub fn add_component(
        &mut self,
        name: &[u16],
        datatype: u32,
        data: Option<&[u8]>,
    ) -> Result<(), AddError> {
        let Some(data) = data else {
            return Err(AddError::InvalidPointer);
        };
        let payload = match datatype {
            DPNA_DATATYPE_DWORD => {
                let Ok(bytes) = <[u8; 4]>::try_from(data) else {
                    return Err(AddError::InvalidParam);
                };
                Data::Dword(u32::from_le_bytes(bytes))
            }
            DPNA_DATATYPE_GUID => {
                let Ok(bytes) = <[u8; 16]>::try_from(data) else {
                    return Err(AddError::InvalidParam);
                };
                Data::Guid(Guid::from_le_bytes(bytes))
            }
            DPNA_DATATYPE_STRING => {
                let Some(units) = data
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .position(|unit| *unit == [0, 0])
                else {
                    return Err(AddError::InvalidParam);
                };
                if (units + 1) * 2 != data.len() {
                    return Err(AddError::InvalidParam);
                }
                Data::String(data.to_vec())
            }
            DPNA_DATATYPE_STRING_ANSI => {
                let Some(len) = data.iter().position(|&byte| byte == 0) else {
                    return Err(AddError::InvalidParam);
                };
                if len + 1 != data.len() {
                    return Err(AddError::InvalidParam);
                }
                Data::Ansi(data.to_vec())
            }
            DPNA_DATATYPE_BINARY => Data::Binary(data.to_vec()),
            _ => Data::None,
        };

        let name = until_nul(name).to_vec();
        let size = data.len();
        match self.components.iter_mut().find(|entry| entry.name == name) {
            Some(entry) => {
                entry.datatype = datatype;
                entry.size = size;
                entry.data = payload;
            }
            None => self.components.push(Component {
                name,
                datatype,
                size,
                data: payload,
            }),
        }
        Ok(())
    }

    /// `GetURLW`. Returns the NUL-terminated URL when `capacity` code units fit it.
    /// Otherwise returns the required capacity, including the NUL, as `Err`.
    pub fn get_url(&self, capacity: usize) -> Result<Vec<u16>, usize> {
        let mut body = Vec::new();
        for entry in &self.components {
            // The C code tests `position`, the length so far, so the separator
            // follows any non-empty body even when the next entry renders nothing.
            if !body.is_empty() {
                body.push(u16::from(b';'));
            }
            entry.render(&mut body);
        }
        let mut url: Vec<u16> = HEADER.encode_utf16().collect();
        url.extend_from_slice(&body);
        url.push(0);
        if capacity >= url.len() {
            Ok(url)
        } else {
            Err(url.len())
        }
    }

    /// `GetComponentByName`. The name is compared up to its first NUL.
    pub fn get_component_by_name(
        &self,
        name: &[u16],
        capacity: usize,
    ) -> Result<Retrieved, NameError> {
        let name = until_nul(name);
        let Some(entry) = self.components.iter().find(|entry| entry.name == name) else {
            return Err(NameError::DoesNotExist);
        };
        let retrieved = entry.retrieve();
        if capacity < retrieved.size {
            return Err(NameError::BufferTooSmall {
                size: retrieved.size,
            });
        }
        Ok(retrieved)
    }

    /// `GetComponentByIndex`. The name is returned without its NUL; the caller
    /// appends one when it has room for it.
    pub fn get_component_by_index(
        &self,
        index: usize,
        name_capacity: usize,
        buf_capacity: usize,
    ) -> Result<(Vec<u16>, Retrieved), IndexError> {
        let Some(entry) = self.components.get(index) else {
            return Err(IndexError::DoesNotExist);
        };
        let retrieved = entry.retrieve();
        if buf_capacity < retrieved.size || name_capacity < entry.name.len() {
            return Err(IndexError::BufferTooSmall {
                name_len: entry.name.len() + 1,
                size: retrieved.size,
                datatype: retrieved.datatype,
            });
        }
        Ok((entry.name.clone(), retrieved))
    }
}
