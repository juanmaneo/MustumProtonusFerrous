//! Headless subset of Wine's `dlls/hidparse.sys` (`main.c`).
//!
//! The ported functions compute from their arguments what the C bodies compute.
//! The parsed tables are vectors instead of Wine's flat preparsed buffer, and
//! nothing is allocated from a pool.
//!
//! Ported:
//! - `parse_descriptor`: the item reader, the global and local item state, the
//!   Input, Output and Feature value caps, the collection nodes and the report
//!   sizes.
//! - `parse_top_level_collections`: the top-level collection boundaries, as
//!   `top_level_collections`.
//! - `HidP_GetCollectionDescription`: `get_collection_description`, the
//!   collection and report ID tables.
//!
//! Skipped:
//! - `HidP_FreeCollectionDescription`, `ExAllocatePool`, `ExFreePool` and
//!   `free_parser_state`: ownership is Rust's.
//! - The flat preparsed buffer: its magic, offsets, pointer macros, `unknown`
//!   field and `caps_size`.
//! - `array_reserve`: its growth rule is kept, so a value list that cannot grow
//!   to hold its next index fails the parse as in C. No memory is reallocated.
//! - `debug_print_preparsed`, `debugstr_*`, and the TRACE, ERR and FIXME output.
//! - `DriverEntry`, a stub.
//! - The `ReportIDs` allocation counter, which only sizes the C buffer.
//!
//! Divergences from the C:
//! - An item whose last data byte lies at or past the end of the descriptor is
//!   rejected. C accepts one byte past the end there, and ASan reports the read.
//! - Products of report count and report size, and sums of start and length,
//!   can exceed 2^31, which overflows a signed `int` in C (undefined behaviour).
//!   Rust computes them in `u32` and wraps the sums.
//! - The `unknown` and `padding` fields of `hid_value_caps` are not modeled.

mod collection;
mod parser;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Parses a report descriptor into the preparsed data of one collection.
///
/// Returns `None` when the descriptor is malformed, as `parse_descriptor` in C does.
pub fn parse_descriptor(report_desc: &[u8]) -> Option<PreparsedData> {
    parser::parse_descriptor(report_desc)
}

/// Returns the start index of each top-level collection in a report descriptor.
pub fn top_level_collections(report_desc: &[u8]) -> Vec<usize> {
    parser::top_level_collections(report_desc)
}

/// Builds the device description (`HidP_GetCollectionDescription`) of a report descriptor.
pub fn get_collection_description(report_desc: &[u8]) -> Result<DeviceDesc, i32> {
    collection::get_collection_description(report_desc)
}

/// `HIDP_STATUS_INTERNAL_ERROR`, returned when a collection does not parse.
pub const HIDP_STATUS_INTERNAL_ERROR: i32 = 0xC011_0008u32 as i32;

/// `HID_VALUE_CAPS_ARRAY_HAS_MORE`: another usage of the same array item follows.
pub const HID_VALUE_CAPS_ARRAY_HAS_MORE: u32 = 0x01;
/// `HID_VALUE_CAPS_IS_CONSTANT`: the item is a constant (`Data/Constant` bit clear).
pub const HID_VALUE_CAPS_IS_CONSTANT: u32 = 0x02;
/// `HID_VALUE_CAPS_IS_BUTTON`: a one-bit or array item.
pub const HID_VALUE_CAPS_IS_BUTTON: u32 = 0x04;
/// `HID_VALUE_CAPS_IS_ABSOLUTE`: the item is absolute, not relative.
pub const HID_VALUE_CAPS_IS_ABSOLUTE: u32 = 0x08;
/// `HID_VALUE_CAPS_IS_RANGE`: the pending usages form a usage minimum and maximum.
pub const HID_VALUE_CAPS_IS_RANGE: u32 = 0x10;
/// `HID_VALUE_CAPS_IS_STRING_RANGE`: the string indices form a range.
pub const HID_VALUE_CAPS_IS_STRING_RANGE: u32 = 0x40;
/// `HID_VALUE_CAPS_IS_DESIGNATOR_RANGE`: the designator indices form a range.
pub const HID_VALUE_CAPS_IS_DESIGNATOR_RANGE: u32 = 0x80;

/// A `hid_value_caps`: one value of an Input, Output or Feature item, with the
/// global and local items in effect when it was declared.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValueCaps {
    pub usage_page: u16,
    pub report_id: u8,
    pub start_bit: u8,
    pub bit_size: u16,
    pub report_count: u16,
    pub start_byte: u16,
    pub total_bits: u16,
    pub bit_field: u32,
    pub end_byte: u16,
    pub link_collection: u16,
    pub link_usage_page: u16,
    pub link_usage: u16,
    pub flags: u32,
    pub usage_min: u16,
    pub usage_max: u16,
    pub string_min: u16,
    pub string_max: u16,
    pub designator_min: u16,
    pub designator_max: u16,
    pub data_index_min: u16,
    pub data_index_max: u16,
    pub null_value: u16,
    pub logical_min: i32,
    pub logical_max: i32,
    pub physical_min: i32,
    pub physical_max: i32,
    pub units: i32,
    pub units_exp: i32,
}

/// A `hid_collection_node`: a collection and its place in the collection tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionNode {
    pub usage_page: u16,
    pub usage: u16,
    pub parent: u16,
    pub number_of_children: u16,
    pub next_sibling: u16,
    pub first_child: u16,
    pub collection_type: u32,
}

/// The value caps and byte length of one report type (Input, Output or Feature).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportCaps {
    /// The value count, including the empty values that have no caps entry.
    pub count: u16,
    pub byte_length: u16,
    pub values: Vec<ValueCaps>,
}

/// The parsed form of one collection's descriptor (`hid_preparsed_data`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparsedData {
    pub usage_page: u16,
    pub usage: u16,
    pub input: ReportCaps,
    pub output: ReportCaps,
    pub feature: ReportCaps,
    pub collections: Vec<CollectionNode>,
}

/// A `HIDP_COLLECTION_DESC`, with its parsed descriptor moved in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionDesc {
    pub usage_page: u16,
    pub usage: u16,
    pub collection_number: u8,
    pub input_length: u16,
    pub output_length: u16,
    pub feature_length: u16,
    pub preparsed: PreparsedData,
}

/// A `HIDP_REPORT_IDS` entry: a report ID and its length per report type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportIdDesc {
    pub report_id: u8,
    pub collection_number: u8,
    pub input_length: u16,
    pub output_length: u16,
    pub feature_length: u16,
}

/// A `HIDP_DEVICE_DESC`: the collections and the report IDs of a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDesc {
    pub collections: Vec<CollectionDesc>,
    pub report_ids: Vec<ReportIdDesc>,
}
