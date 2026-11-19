//! hid subset for MPF: the deterministic logic of Wine's `dlls/hid/hidp.c`.
//!
//! The preparsed descriptor is a `PreparsedData` value and each report is a byte
//! slice. The functions answer the `HidP` queries and pack report bytes the way the
//! C functions do. No device, IOCTL, file, or flat preparsed blob is involved.
//!
//! Ported (table tests against a verbatim Wine C oracle, see `oracle_checks.rs`):
//! - Usage values: `get_usage_value`, `get_usage_value_array` and
//!   `get_scaled_usage_value` (the `HidP_Get*` readers), and `set_usage_value`,
//!   `set_usage_value_array` and `set_scaled_usage_value` (the `HidP_Set*` writers).
//! - Usage lists: `get_usages` and `get_usages_ex` list the set buttons; `set_usages`
//!   and `unset_usages` set and clear them, including buttons stored as array indices.
//! - Data lists: `get_data` reads every value and button into `HidpData` items, and
//!   `set_data` writes them back.
//! - Caps: `get_caps`, `get_value_caps`, `get_button_caps`,
//!   `get_specific_value_caps`, `get_specific_button_caps`,
//!   `max_usage_list_length`, `max_data_list_length` and
//!   `initialize_report_for_id`.
//!
//! Divergences from the C bodies:
//! - Integers are 32 bits, as `ULONG` and `LONG` are on Windows. The oracle harness is
//!   built for LP64, where they are 64 bits, so its rows stay in the 32-bit range.
//! - Where C would trap or read past a buffer, the port returns an error or treats the
//!   cap as empty. A scaled set or unscale with `physical_min == physical_max != 0`
//!   returns `BAD_LOG_PHY_VALUES` (C divides by zero). A scalar field wider than 32
//!   bits returns `BUFFER_TOO_SMALL`, and so does a `set_data` scalar whose
//!   `start_bit + bit_size` exceeds 32 (C reads past `RawValue`). `HAS_MORE` on the
//!   last cap is clamped to the end of the list. A zero `bit_size` in sign extension
//!   returns the value unchanged.
//! - A usage or data range with `usage_min > usage_max` is empty. C may wrap around
//!   it. A zero-length array field at `start_bit` 0 is empty, where C's end bit wraps.
//! - A cap whose bits lie past the end of the report panics. C reads past the buffer.
//! - When an output slice is too small, the list functions return `BUFFER_TOO_SMALL`
//!   and no partial list. C fills the slice and reports the count it needed.
//!   `set_data` does not return the number of items it processed.
//! - Caps outputs are `ValueCaps` copies, not the `HIDP_VALUE_CAPS` union. A range cap
//!   keeps its `report_count`. `IsAlias`, `Units`, the link usages, and the string and
//!   designator ranges are not modelled.
//! - `copy_bits` copies one bit at a time. The C version reads two bytes at a time,
//!   which ASan flags at the end of a buffer.
//!
//! Kept from C, not fixed (the oracle rows record both):
//! - Scalar `set_data` writes the field at bit 0 of its start byte, not at `start_bit`.
//! - `unset_usages` on an array clears the first index byte that falls in the cap's
//!   range, whatever usage was named.
//!
//! Skipped:
//! - `HidP_GetLinkCollectionNodes`: the link node table is not modelled, so
//!   `PreparsedData` records only the node count.
//! - `HidP_TranslateUsagesToI8042ScanCodes`: a FIXME stub that returns
//!   `STATUS_NOT_IMPLEMENTED`.
//! - `hidd.c`: only `DeviceIoControl` wrappers.
//! - The preparsed-data magic check, and the `TRACE` and `FIXME` logging.

mod array;
mod bits;
mod caps;
mod data;
mod enumerate;
mod usage;
mod value;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use caps::{
    get_button_caps, get_caps, get_specific_button_caps, get_specific_value_caps, get_value_caps,
    initialize_report_for_id, max_data_list_length, max_usage_list_length,
};
pub use data::{get_data, set_data};
pub use usage::{get_usages, get_usages_ex, set_usages, unset_usages};
pub use value::{
    get_scaled_usage_value, get_usage_value, get_usage_value_array, set_scaled_usage_value,
    set_usage_value, set_usage_value_array,
};

/// An `NTSTATUS` value, as the `HIDP_STATUS_*` codes of `ddk/hidpi.h` are.
pub type Status = i32;

/// `HIDP_STATUS_SUCCESS`.
pub const HIDP_STATUS_SUCCESS: Status = 0x0011_0000;
/// `HIDP_STATUS_INVALID_REPORT_LENGTH`: an empty report, or one whose length is not the
/// length of its report type.
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: Status = 0xC011_0003u32 as i32;
/// `HIDP_STATUS_USAGE_NOT_FOUND`.
pub const HIDP_STATUS_USAGE_NOT_FOUND: Status = 0xC011_0004u32 as i32;
/// `HIDP_STATUS_VALUE_OUT_OF_RANGE`.
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: Status = 0xC011_0005u32 as i32;
/// `HIDP_STATUS_BAD_LOG_PHY_VALUES`.
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: Status = 0xC011_0006u32 as i32;
/// `HIDP_STATUS_BUFFER_TOO_SMALL`.
pub const HIDP_STATUS_BUFFER_TOO_SMALL: Status = 0xC011_0007u32 as i32;
/// `HIDP_STATUS_INCOMPATIBLE_REPORT_ID`.
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: Status = 0xC011_000Au32 as i32;
/// `HIDP_STATUS_NOT_VALUE_ARRAY`.
pub const HIDP_STATUS_NOT_VALUE_ARRAY: Status = 0xC011_000Bu32 as i32;
/// `HIDP_STATUS_IS_VALUE_ARRAY`.
pub const HIDP_STATUS_IS_VALUE_ARRAY: Status = 0xC011_000Cu32 as i32;
/// `HIDP_STATUS_BUTTON_NOT_PRESSED`.
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: Status = 0xC011_000Fu32 as i32;
/// `HIDP_STATUS_REPORT_DOES_NOT_EXIST`.
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: Status = 0xC011_0010u32 as i32;

/// The report type of a query: `HidP_Input`, `HidP_Output` or `HidP_Feature`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportType {
    Input,
    Output,
    Feature,
}

/// Wine's `hid_value_caps`: one value or button cap, with its usage range, its
/// location in the report, and its logical and physical ranges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ValueCaps {
    pub usage_page: u16,
    pub usage_min: u16,
    pub usage_max: u16,
    pub report_id: u8,
    pub start_byte: u16,
    pub start_bit: u8,
    pub bit_size: u16,
    pub report_count: u16,
    pub bit_field: u32,
    pub flags: u32,
    pub data_index_min: u16,
    pub data_index_max: u16,
    pub link_collection: u16,
    pub logical_min: i32,
    pub logical_max: i32,
    pub physical_min: i32,
    pub physical_max: i32,
}

impl ValueCaps {
    /// `HID_VALUE_CAPS_IS_ARRAY`: the field holds usage index bytes, not one value.
    pub fn is_array(&self) -> bool {
        self.bit_field & 2 == 0
    }

    /// `HID_VALUE_CAPS_IS_BUTTON`.
    pub fn is_button(&self) -> bool {
        self.flags & 4 != 0
    }

    /// `HID_VALUE_CAPS_IS_RANGE`.
    pub fn is_range(&self) -> bool {
        self.flags & 0x10 != 0
    }

    /// `HID_VALUE_CAPS_ARRAY_HAS_MORE`: the next cap continues this cap's index range.
    pub fn has_more(&self) -> bool {
        self.flags & 1 != 0
    }

    /// A cap with a usage. Wine skips the caps whose usages are both zero.
    pub fn is_used(&self) -> bool {
        self.usage_min != 0 || self.usage_max != 0
    }

    /// The bit position of the field in the report, counted from byte 0.
    pub fn first_bit(&self) -> usize {
        8 * usize::from(self.start_byte) + usize::from(self.start_bit)
    }
}

/// Wine's `struct hid_report_caps` for one report type: the report length and the
/// value caps in descriptor order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReportCaps {
    pub byte_length: u16,
    pub values: Vec<ValueCaps>,
}

/// The preparsed data of one device: the usage page and usage, the link collection
/// count, and the caps of each report type.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PreparsedData {
    pub usage_page: u16,
    pub usage: u16,
    pub link_collection_nodes: u16,
    pub input: ReportCaps,
    pub output: ReportCaps,
    pub feature: ReportCaps,
}

impl PreparsedData {
    fn report(&self, rt: ReportType) -> &ReportCaps {
        match rt {
            ReportType::Input => &self.input,
            ReportType::Output => &self.output,
            ReportType::Feature => &self.feature,
        }
    }
}

/// `HIDP_DATA`: one data index with its raw value. On a button, `raw_value` is the
/// `On` flag in its low byte.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HidpData {
    pub data_index: u16,
    pub raw_value: u32,
}

/// `USAGE_AND_PAGE`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsageAndPage {
    pub usage_page: u16,
    pub usage: u16,
}

/// `HIDP_CAPS`: the device totals that `HidP_GetCaps` reports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    pub usage: u16,
    pub usage_page: u16,
    pub input_report_byte_length: u16,
    pub output_report_byte_length: u16,
    pub feature_report_byte_length: u16,
    pub number_link_collection_nodes: u16,
    pub number_input_button_caps: u16,
    pub number_input_value_caps: u16,
    pub number_input_data_indices: u16,
    pub number_output_button_caps: u16,
    pub number_output_value_caps: u16,
    pub number_output_data_indices: u16,
    pub number_feature_button_caps: u16,
    pub number_feature_value_caps: u16,
    pub number_feature_data_indices: u16,
}
