//! Port of the portable logic in Wine's `dlls/dpnet/address.c`.
//!
//! Ported:
//! - Component store: `AddComponent` validation and upsert ([`Address::add_component`])
//! - URL text: `GetURLW` ([`Address::get_url`])
//! - Lookups: `GetComponentByName` ([`Address::get_component_by_name`]),
//!   `GetComponentByIndex` ([`Address::get_component_by_index`]),
//!   `GetNumComponents` ([`Address::num_components`])
//! - `GUID` in-memory layout used by `GetComponentByName` and `GetComponentByIndex`: [`Guid`]
//!
//! Skipped:
//! - Stub exports that return an error without doing work: `BuildFromURLW/A`,
//!   `SetEqual`, `IsEqual`, `Clear`, `Get/SetUserData`, `Get/SetDevice`,
//!   `BuildFromDirectPlay4Address`.
//! - `GetSP`/`SetSP`: thin accessors that call `AddComponent` for the provider key.
//! - `Duplicate`: allocates a COM object and replays `AddComponent` into it.
//! - `GetURLA`: converts through `WideCharToMultiByte` with the OS code page.
//! - COM plumbing (`QueryInterface`, `AddRef`, `Release`, `DPNET_CreateDirectPlay8Address`)
//!   and `debugstr_SP`, which is debug output only.
//! - Output-pointer NULL checks: replaced by `Option` and slices.
//! - The other `dlls/dpnet` sources (client, server, peer, lobbied app, thread pool,
//!   DLL entry points): COM, networking, threads and registry.
//!
//! Deviations from the C code, all on paths the C code leaves undefined:
//! - `GetComponentByIndex` with `index == count` returns `DoesNotExist`. The C bounds
//!   check is `>`, so it reads one slot past the array.
//! - `GetComponentByIndex` with a name capacity equal to the name length succeeds, and
//!   the name is returned without its NUL. The C code writes the NUL one WCHAR past the
//!   caller's buffer.
//! - `AddComponent` rejects a STRING or ANSI payload with no NUL terminator. The C code
//!   reads past the buffer to find one.
//! - `GetURLW` has no 1024-WCHAR stack buffer limit. The C code overflows it on long
//!   addresses.
//! - Name comparison is by code unit. Wine's `lstrcmpW` calls `CompareStringW` with
//!   flags 0, which is locale-aware.
//! - ANSI values render one byte per code unit. This is exact for ASCII only; the C
//!   `%hs` conversion depends on the active code page.

mod address;

pub use address::{AddError, Address, IndexError, NameError, Retrieved};

/// `DPNA_DATATYPE_*` from `dplay8.h`.
pub const DPNA_DATATYPE_STRING: u32 = 0x0000_0001;
pub const DPNA_DATATYPE_DWORD: u32 = 0x0000_0002;
pub const DPNA_DATATYPE_GUID: u32 = 0x0000_0003;
pub const DPNA_DATATYPE_BINARY: u32 = 0x0000_0004;
pub const DPNA_DATATYPE_STRING_ANSI: u32 = 0x0000_0005;

/// `GUID` with the Windows field layout. [`Guid::to_le_bytes`] and
/// [`Guid::from_le_bytes`] use the in-memory byte order of a little-endian host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub fn from_le_bytes(bytes: [u8; 16]) -> Self {
        let mut data4 = [0u8; 8];
        data4.copy_from_slice(&bytes[8..]);
        Self {
            data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_le_bytes([bytes[4], bytes[5]]),
            data3: u16::from_le_bytes([bytes[6], bytes[7]]),
            data4,
        }
    }

    pub fn to_le_bytes(self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.data1.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.data2.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.data3.to_le_bytes());
        bytes[8..].copy_from_slice(&self.data4);
        bytes
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
