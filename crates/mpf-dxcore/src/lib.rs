//! Deterministic adapter-property logic from Wine's `dlls/dxcore/dxcore.c`.
//!
//! Ported: supported adapter property size lookup and `GetProperty` validation
//! and serialization for LUID, driver version and description, hardware ID,
//! memory values, and the `IsHardware` byte. The adapter values are supplied
//! by the caller, so the port does not enumerate hardware or depend on Wine's
//! wined3d adapter layer.
//!
//! Skipped: COM interface and reference-counting methods, adapter/factory/list
//! creation and enumeration (including LUID lookup), and state/notification
//! methods. Those require COM, OS or hardware interaction, or are Wine stubs.
//!
//! For safety, a driver description slice without an embedded NUL is treated
//! as a complete string and gets a trailing NUL in the output. Wine receives a
//! fixed, NUL-terminated description buffer from wined3d.

/// HRESULT returned for a supported operation.
pub type HResult = i32;

/// `S_OK`.
pub const S_OK: HResult = 0;
/// `E_POINTER`.
pub const E_POINTER: HResult = 0x8000_4003u32 as i32;
/// `E_INVALIDARG`.
pub const E_INVALIDARG: HResult = 0x8007_0057u32 as i32;
/// `DXGI_ERROR_INVALID_CALL`.
pub const DXGI_ERROR_INVALID_CALL: HResult = 0x887A_0001u32 as i32;

/// `DXCoreAdapterProperty` numeric values used by Wine.
pub mod property {
    /// Adapter LUID, an 8-byte `LUID`.
    pub const INSTANCE_LUID: u32 = 0;
    /// Driver version, an 8-byte `LARGE_INTEGER`.
    pub const DRIVER_VERSION: u32 = 1;
    /// NUL-terminated driver description.
    pub const DRIVER_DESCRIPTION: u32 = 2;
    /// Vendor, device, subsystem, and revision IDs.
    pub const HARDWARE_ID: u32 = 3;
    /// Dedicated video memory in bytes.
    pub const DEDICATED_ADAPTER_MEMORY: u32 = 7;
    /// Dedicated system memory (Wine returns zero).
    pub const DEDICATED_SYSTEM_MEMORY: u32 = 8;
    /// Shared system memory in bytes.
    pub const SHARED_SYSTEM_MEMORY: u32 = 9;
    /// Wine currently reports every adapter as hardware.
    pub const IS_HARDWARE: u32 = 11;
}

/// The adapter fields read by Wine's deterministic `GetProperty` cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdapterProperties<'a> {
    /// Native byte representation of the adapter LUID.
    pub instance_luid: [u8; 8],
    /// Native byte representation of the driver version.
    pub driver_version: [u8; 8],
    /// Hardware identity fields, in vendor/device/subsystem/revision order.
    pub hardware_id: [u32; 4],
    /// Dedicated video memory in bytes.
    pub dedicated_adapter_memory: u64,
    /// Shared system memory in bytes.
    pub shared_system_memory: u64,
    /// Driver description bytes; ends at the first NUL if present.
    pub driver_description: &'a [u8],
}

fn description_len(description: &[u8]) -> usize {
    description
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(description.len())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KnownProperty {
    InstanceLuid,
    DriverVersion,
    DriverDescription,
    HardwareId,
    DedicatedAdapterMemory,
    DedicatedSystemMemory,
    SharedSystemMemory,
    IsHardware,
}

impl KnownProperty {
    fn from_raw(property: u32) -> Result<Self, HResult> {
        use property::*;

        match property {
            INSTANCE_LUID => Ok(Self::InstanceLuid),
            DRIVER_VERSION => Ok(Self::DriverVersion),
            DRIVER_DESCRIPTION => Ok(Self::DriverDescription),
            HARDWARE_ID => Ok(Self::HardwareId),
            DEDICATED_ADAPTER_MEMORY => Ok(Self::DedicatedAdapterMemory),
            DEDICATED_SYSTEM_MEMORY => Ok(Self::DedicatedSystemMemory),
            SHARED_SYSTEM_MEMORY => Ok(Self::SharedSystemMemory),
            IS_HARDWARE => Ok(Self::IsHardware),
            _ => Err(DXGI_ERROR_INVALID_CALL),
        }
    }

    fn size(self, description: &[u8]) -> usize {
        match self {
            Self::InstanceLuid | Self::DriverVersion => 8,
            Self::DriverDescription => description_len(description) + 1,
            Self::HardwareId => 16,
            Self::DedicatedAdapterMemory
            | Self::DedicatedSystemMemory
            | Self::SharedSystemMemory => 8,
            Self::IsHardware => 1,
        }
    }
}

/// Returns the number of bytes Wine reports for a supported property.
///
/// Unknown and recognized-but-unimplemented properties return
/// `DXGI_ERROR_INVALID_CALL`.
pub fn get_property_size(property: u32, description: &[u8]) -> Result<usize, HResult> {
    KnownProperty::from_raw(property).map(|property| property.size(description))
}

/// Writes a property into `buffer`, matching Wine's `GetProperty` behavior.
///
/// A missing buffer returns `E_POINTER` before property validation. For a
/// non-empty undersized description buffer Wine zeroes its first byte before
/// returning `E_INVALIDARG`; other undersized buffers are left untouched.
pub fn get_property(
    adapter: &AdapterProperties<'_>,
    property: u32,
    buffer: Option<&mut [u8]>,
) -> HResult {
    let Some(buffer) = buffer else {
        return E_POINTER;
    };
    let property = match KnownProperty::from_raw(property) {
        Ok(property) => property,
        Err(error) => return error,
    };
    let size = property.size(adapter.driver_description);

    if property == KnownProperty::DriverDescription && !buffer.is_empty() {
        buffer[0] = 0;
    }
    if buffer.len() < size {
        return E_INVALIDARG;
    }

    match property {
        KnownProperty::InstanceLuid => buffer[..8].copy_from_slice(&adapter.instance_luid),
        KnownProperty::DriverVersion => buffer[..8].copy_from_slice(&adapter.driver_version),
        KnownProperty::DriverDescription => {
            let len = description_len(adapter.driver_description);
            buffer[..len].copy_from_slice(&adapter.driver_description[..len]);
            buffer[len] = 0;
        }
        KnownProperty::HardwareId => {
            for (index, value) in adapter.hardware_id.iter().enumerate() {
                let start = index * 4;
                buffer[start..start + 4].copy_from_slice(&value.to_ne_bytes());
            }
        }
        KnownProperty::DedicatedAdapterMemory => {
            buffer[..8].copy_from_slice(&adapter.dedicated_adapter_memory.to_ne_bytes());
        }
        KnownProperty::DedicatedSystemMemory => {
            buffer[..8].copy_from_slice(&0_u64.to_ne_bytes());
        }
        KnownProperty::SharedSystemMemory => {
            buffer[..8].copy_from_slice(&adapter.shared_system_memory.to_ne_bytes());
        }
        KnownProperty::IsHardware => buffer[0] = 1,
    }

    S_OK
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
