//! `parse_descriptor` and `parse_top_level_collections`: the report descriptor
//! state machine of Wine's `main.c`, over a byte slice.

use crate::{
    CollectionNode, PreparsedData, ReportCaps, ValueCaps, HID_VALUE_CAPS_ARRAY_HAS_MORE,
    HID_VALUE_CAPS_IS_ABSOLUTE, HID_VALUE_CAPS_IS_BUTTON, HID_VALUE_CAPS_IS_CONSTANT,
    HID_VALUE_CAPS_IS_DESIGNATOR_RANGE, HID_VALUE_CAPS_IS_RANGE, HID_VALUE_CAPS_IS_STRING_RANGE,
};

const INPUT_DATA_CONST: u32 = 0x01;
const INPUT_ARRAY_VAR: u32 = 0x02;
const INPUT_ABS_REL: u32 = 0x04;
const INPUT_NULL: u32 = 0x40;

macro_rules! short_item {
    ($tag:expr, $item_type:expr) => {
        ($tag << 4) | ($item_type << 2)
    };
}

const ITEM_MASK: u8 = 0xFC;

const MAIN_INPUT: u8 = short_item!(0x8, 0);
const MAIN_OUTPUT: u8 = short_item!(0x9, 0);
const MAIN_FEATURE: u8 = short_item!(0xB, 0);
const MAIN_COLLECTION: u8 = short_item!(0xA, 0);
const MAIN_END_COLLECTION: u8 = short_item!(0xC, 0);

const GLOBAL_USAGE_PAGE: u8 = short_item!(0x0, 1);
const GLOBAL_LOGICAL_MINIMUM: u8 = short_item!(0x1, 1);
const GLOBAL_LOGICAL_MAXIMUM: u8 = short_item!(0x2, 1);
const GLOBAL_PHYSICAL_MINIMUM: u8 = short_item!(0x3, 1);
const GLOBAL_PHYSICAL_MAXIMUM: u8 = short_item!(0x4, 1);
const GLOBAL_UNIT_EXPONENT: u8 = short_item!(0x5, 1);
const GLOBAL_UNIT: u8 = short_item!(0x6, 1);
const GLOBAL_REPORT_SIZE: u8 = short_item!(0x7, 1);
const GLOBAL_REPORT_ID: u8 = short_item!(0x8, 1);
const GLOBAL_REPORT_COUNT: u8 = short_item!(0x9, 1);
const GLOBAL_PUSH: u8 = short_item!(0xA, 1);
const GLOBAL_POP: u8 = short_item!(0xB, 1);

const LOCAL_USAGE: u8 = short_item!(0x0, 2);
const LOCAL_USAGE_MINIMUM: u8 = short_item!(0x1, 2);
const LOCAL_USAGE_MAXIMUM: u8 = short_item!(0x2, 2);
const LOCAL_DESIGNATOR_INDEX: u8 = short_item!(0x3, 2);
const LOCAL_DESIGNATOR_MINIMUM: u8 = short_item!(0x4, 2);
const LOCAL_DESIGNATOR_MAXIMUM: u8 = short_item!(0x5, 2);
const LOCAL_STRING_INDEX: u8 = short_item!(0x7, 2);
const LOCAL_STRING_MINIMUM: u8 = short_item!(0x8, 2);
const LOCAL_STRING_MAXIMUM: u8 = short_item!(0x9, 2);
const LOCAL_DELIMITER: u8 = short_item!(0xA, 2);

/// One item of a descriptor: its prefix byte, its unsigned and signed value,
/// and the offset of the item after it.
struct Item {
    prefix: u8,
    value: u32,
    signed: i32,
    next: usize,
}

/// Reads the item at `pos`, or `None` when its data would run past the end of
/// `desc`. Its last data byte must lie inside `desc`.
fn read_item(desc: &[u8], pos: usize) -> Option<Item> {
    let prefix = desc[pos];
    let size = match prefix & 0x03 {
        3 => 4,
        size => usize::from(size),
    };
    let data = desc.get(pos + 1..pos + 1 + size)?;
    let mut bytes = [0u8; 4];
    bytes[..size].copy_from_slice(data);
    let value = u32::from_le_bytes(bytes);
    let signed = match size {
        1 => i32::from(value as u8 as i8),
        2 => i32::from(value as u16 as i16),
        _ => value as i32,
    };
    Some(Item {
        prefix,
        value,
        signed,
        next: pos + 1 + size,
    })
}

/// `array_reserve`: grows `array` as C does (to 32, then by 3/2) and reports
/// whether it now holds `index`. A list can fail when `index` jumps past the
/// grown size.
fn array_reserve(array: &mut Vec<ValueCaps>, index: usize) -> bool {
    if index < array.len() {
        return true;
    }
    let size = if array.is_empty() {
        32
    } else {
        array.len() * 3 / 2
    };
    array.resize(size, ValueCaps::default());
    size > index
}

/// Grows a stack or the collection list to hold `index`. C cannot fail here:
/// each index is at most one past the current size.
fn grow(array: &mut Vec<ValueCaps>, index: usize) {
    if index >= array.len() {
        array.resize(index + 1, ValueCaps::default());
    }
}

fn copy_global_items(dst: &mut ValueCaps, src: &ValueCaps) {
    dst.usage_page = src.usage_page;
    dst.logical_min = src.logical_min;
    dst.logical_max = src.logical_max;
    dst.physical_min = src.physical_min;
    dst.physical_max = src.physical_max;
    dst.units_exp = src.units_exp;
    dst.units = src.units;
    dst.bit_size = src.bit_size;
    dst.report_id = src.report_id;
    dst.report_count = src.report_count;
}

fn copy_collection_items(dst: &mut ValueCaps, src: &ValueCaps) {
    dst.link_collection = src.link_collection;
    dst.link_usage_page = src.link_usage_page;
    dst.link_usage = src.link_usage;
}

/// The state of one descriptor parse: `struct hid_parser_state`.
///
/// The global and collection stacks are separate here. C keeps both in one
/// array, but each stack writes its own fields, so the two behave the same.
struct Parser {
    usage: u16,
    usage_page: u16,
    link_nodes: u16,
    usages_page: [u16; 256],
    usages_min: [u16; 256],
    usages_max: [u16; 256],
    usages_size: usize,
    items: ValueCaps,
    global_stack: Vec<ValueCaps>,
    global_idx: usize,
    collection_stack: Vec<ValueCaps>,
    collection_idx: usize,
    collections: Vec<ValueCaps>,
    values: [Vec<ValueCaps>; 3],
    bit_size: [[u32; 256]; 3],
    byte_length: [u16; 3],
    caps_count: [u16; 3],
    empty_caps: [u16; 3],
    data_count: [u16; 3],
}

impl Parser {
    fn new() -> Self {
        Self {
            usage: 0,
            usage_page: 0,
            link_nodes: 0,
            usages_page: [0; 256],
            usages_min: [0; 256],
            usages_max: [0; 256],
            usages_size: 0,
            items: ValueCaps::default(),
            global_stack: Vec::new(),
            global_idx: 0,
            collection_stack: Vec::new(),
            collection_idx: 0,
            collections: Vec::new(),
            values: [Vec::new(), Vec::new(), Vec::new()],
            bit_size: [[0; 256]; 3],
            byte_length: [0; 3],
            caps_count: [0; 3],
            empty_caps: [0; 3],
            data_count: [0; 3],
        }
    }

    /// Applies one item. `false` makes the whole parse fail, as in C.
    fn apply(&mut self, item: Item) -> bool {
        self.items.bit_field = item.value;
        match item.prefix & ITEM_MASK {
            MAIN_INPUT => self.new_value_caps(0),
            MAIN_OUTPUT => self.new_value_caps(1),
            MAIN_FEATURE => self.new_value_caps(2),
            MAIN_END_COLLECTION => self.end_collection(),
            GLOBAL_POP => self.global_pop(),
            LOCAL_USAGE => self.local_usage((item.value >> 16) as u16, item.value as u16),
            // Wine does not implement delimiters, so the parse stops here.
            LOCAL_DELIMITER => false,
            _ => {
                self.apply_item(&item);
                true
            }
        }
    }

    /// The items that cannot fail: global and local state, and the collection
    /// start.
    fn apply_item(&mut self, item: &Item) {
        let value = item.value;
        match item.prefix & ITEM_MASK {
            MAIN_COLLECTION => self.new_collection(),
            GLOBAL_USAGE_PAGE => self.items.usage_page = value as u16,
            GLOBAL_LOGICAL_MINIMUM => self.items.logical_min = item.signed,
            GLOBAL_LOGICAL_MAXIMUM => self.items.logical_max = item.signed,
            GLOBAL_PHYSICAL_MINIMUM => self.items.physical_min = item.signed,
            GLOBAL_PHYSICAL_MAXIMUM => self.items.physical_max = item.signed,
            GLOBAL_UNIT_EXPONENT => self.items.units_exp = item.signed,
            GLOBAL_UNIT => self.items.units = item.signed,
            GLOBAL_REPORT_SIZE => self.items.bit_size = value as u16,
            GLOBAL_REPORT_ID => self.items.report_id = value as u8,
            GLOBAL_REPORT_COUNT => self.items.report_count = value as u16,
            GLOBAL_PUSH => self.global_push(),
            LOCAL_USAGE_MINIMUM => self.local_usage_min((value >> 16) as u16, value as u16),
            LOCAL_USAGE_MAXIMUM => self.local_usage_max((value >> 16) as u16, value as u16),
            LOCAL_DESIGNATOR_INDEX => {
                self.items.designator_min = value as u16;
                self.items.designator_max = value as u16;
                self.items.flags &= !HID_VALUE_CAPS_IS_DESIGNATOR_RANGE;
            }
            LOCAL_DESIGNATOR_MINIMUM => {
                self.items.designator_min = value as u16;
                self.items.flags |= HID_VALUE_CAPS_IS_DESIGNATOR_RANGE;
            }
            LOCAL_DESIGNATOR_MAXIMUM => {
                self.items.designator_max = value as u16;
                self.items.flags |= HID_VALUE_CAPS_IS_DESIGNATOR_RANGE;
            }
            LOCAL_STRING_INDEX => {
                self.items.string_min = value as u16;
                self.items.string_max = value as u16;
                self.items.flags &= !HID_VALUE_CAPS_IS_STRING_RANGE;
            }
            LOCAL_STRING_MINIMUM => {
                self.items.string_min = value as u16;
                self.items.flags |= HID_VALUE_CAPS_IS_STRING_RANGE;
            }
            LOCAL_STRING_MAXIMUM => {
                self.items.string_max = value as u16;
                self.items.flags |= HID_VALUE_CAPS_IS_STRING_RANGE;
            }
            _ => {}
        }
    }

    /// `reset_local_items`: keeps the global and collection items, clears the rest.
    fn reset_local_items(&mut self) {
        let mut items = ValueCaps::default();
        copy_global_items(&mut items, &self.items);
        copy_collection_items(&mut items, &self.items);
        self.items = items;
        self.usages_page = [0; 256];
        self.usages_min = [0; 256];
        self.usages_max = [0; 256];
        self.usages_size = 0;
    }

    fn global_push(&mut self) {
        grow(&mut self.global_stack, self.global_idx);
        copy_global_items(&mut self.global_stack[self.global_idx], &self.items);
        self.global_idx += 1;
    }

    fn global_pop(&mut self) -> bool {
        if self.global_idx == 0 {
            return false;
        }
        self.global_idx -= 1;
        copy_global_items(&mut self.items, &self.global_stack[self.global_idx]);
        true
    }

    fn page_or_global(&self, page: u16) -> u16 {
        if page == 0 {
            self.items.usage_page
        } else {
            page
        }
    }

    fn local_usage(&mut self, page: u16, usage: u16) -> bool {
        let page = self.page_or_global(page);
        if self.items.flags & HID_VALUE_CAPS_IS_RANGE != 0 {
            self.usages_size = 0;
        }
        let index = self.usages_size;
        self.usages_page[index] = page;
        self.usages_min[index] = usage;
        self.usages_max[index] = usage;
        self.items.usage_min = usage;
        self.items.usage_max = usage;
        self.items.flags &= !HID_VALUE_CAPS_IS_RANGE;
        self.usages_size += 1;
        self.usages_size <= 255
    }

    fn local_usage_min(&mut self, page: u16, usage: u16) {
        let page = self.page_or_global(page);
        if self.items.flags & HID_VALUE_CAPS_IS_RANGE == 0 {
            self.usages_max[0] = 0;
        }
        self.usages_page[0] = page;
        self.usages_min[0] = usage;
        self.items.usage_min = usage;
        self.items.flags |= HID_VALUE_CAPS_IS_RANGE;
        self.usages_size = 1;
    }

    fn local_usage_max(&mut self, page: u16, usage: u16) {
        let page = self.page_or_global(page);
        if self.items.flags & HID_VALUE_CAPS_IS_RANGE == 0 {
            self.usages_min[0] = 0;
        }
        self.usages_page[0] = page;
        self.usages_max[0] = usage;
        self.items.usage_max = usage;
        self.items.flags |= HID_VALUE_CAPS_IS_RANGE;
        self.usages_size = 1;
    }

    fn new_collection(&mut self) {
        let index = self.collection_idx;
        grow(&mut self.collection_stack, index);
        copy_collection_items(&mut self.collection_stack[index], &self.items);
        self.collection_idx += 1;

        self.items.usage_min = self.usages_min[0];
        self.items.usage_max = self.usages_max[0];
        let node = usize::from(self.link_nodes);
        grow(&mut self.collections, node);
        self.collections[node] = self.items;
        self.items.link_collection = self.link_nodes;
        self.items.link_usage_page = self.items.usage_page;
        self.items.link_usage = self.items.usage_min;
        if self.link_nodes == 0 {
            self.usage_page = self.items.usage_page;
            self.usage = self.items.usage_min;
        }
        self.link_nodes = self.link_nodes.wrapping_add(1);
        self.reset_local_items();
    }

    fn end_collection(&mut self) -> bool {
        if self.collection_idx == 0 {
            return false;
        }
        self.collection_idx -= 1;
        copy_collection_items(&mut self.items, &self.collection_stack[self.collection_idx]);
        self.reset_local_items();
        true
    }

    /// `parse_new_value_caps` for the report type `t` (0 Input, 1 Output, 2 Feature).
    fn new_value_caps(&mut self, t: usize) -> bool {
        let items = self.items;
        let usages_size = self.usages_size.max(1);

        let report = &mut self.bit_size[t][usize::from(items.report_id)];
        if *report == 0 {
            *report = 8;
        }
        *report = report.wrapping_add(u32::from(items.bit_size) * u32::from(items.report_count));
        let bits = *report;
        self.byte_length[t] = u32::from(self.byte_length[t]).max(bits.wrapping_add(7) / 8) as u16;

        if items.report_count == 0 {
            self.empty_caps[t] = self.empty_caps[t].wrapping_add(usages_size as u16);
            self.reset_local_items();
            return true;
        }

        let base = usize::from(self.caps_count[t]);
        if !array_reserve(&mut self.values[t], base + usages_size) {
            return false;
        }

        let is_array = items.bit_field & INPUT_ARRAY_VAR == 0;
        let mut start_bit = bits;
        if is_array {
            start_bit =
                start_bit.wrapping_sub(u32::from(items.report_count) * u32::from(items.bit_size));
        } else {
            self.items.report_count = items.report_count.wrapping_sub((usages_size - 1) as u16);
        }
        if items.bit_field & INPUT_ABS_REL == 0 {
            self.items.flags |= HID_VALUE_CAPS_IS_ABSOLUTE;
        }
        if items.bit_field & INPUT_DATA_CONST != 0 {
            self.items.flags |= HID_VALUE_CAPS_IS_CONSTANT;
        }
        if items.bit_size == 1 || is_array {
            self.items.flags |= HID_VALUE_CAPS_IS_BUTTON;
        }
        self.items.null_value = if is_array {
            self.items.logical_min as u16
        } else if items.bit_field & INPUT_NULL == 0 {
            0
        } else {
            1
        };

        self.items.data_index_max = self.data_count[t].wrapping_sub(1);
        for index in 0..usages_size {
            if !is_array {
                start_bit = start_bit.wrapping_sub(
                    u32::from(self.items.report_count) * u32::from(self.items.bit_size),
                );
            } else if index > 0 {
                self.items.flags |= HID_VALUE_CAPS_ARRAY_HAS_MORE;
            } else {
                self.items.flags &= !HID_VALUE_CAPS_ARRAY_HAS_MORE;
            }
            let slot = if is_array {
                usages_size - 1 - index
            } else {
                index
            };
            self.add_value_caps(t, slot, start_bit);
            if !is_array {
                self.items.report_count = 1;
            }
        }
        self.caps_count[t] = self.caps_count[t].wrapping_add(usages_size as u16);
        self.data_count[t] = self.items.data_index_max.wrapping_add(1);

        self.items.usage_page = items.usage_page;
        self.items.report_count = items.report_count;
        self.reset_local_items();
        true
    }

    /// `add_new_value_caps`: stores one value of the current item, for usage
    /// entry `usages_size - 1 - index`, at `values[t][caps_count[t] + index]`.
    fn add_value_caps(&mut self, t: usize, index: usize, start_bit: u32) {
        let entry = self.usages_size.max(1) - 1 - index;
        let slot = usize::from(self.caps_count[t]) + index;
        let usage_page = self.usages_page[entry];
        let usage_min = self.usages_min[entry];
        let usage_max = self.usages_max[entry];

        let items = &mut self.items;
        items.start_byte = (start_bit / 8) as u16;
        items.start_bit = (start_bit % 8) as u8;
        items.total_bits = items.report_count.wrapping_mul(items.bit_size);
        items.end_byte = (start_bit
            .wrapping_add(u32::from(items.total_bits))
            .wrapping_add(7)
            / 8) as u16;
        items.usage_page = usage_page;
        items.usage_min = usage_min;
        items.usage_max = usage_max;
        let count = if usage_max == 0 && usage_min == 0 {
            u16::MAX
        } else {
            usage_max.wrapping_sub(usage_min)
        };
        items.data_index_min = items.data_index_max.wrapping_add(1);
        items.data_index_max = items.data_index_min.wrapping_add(count);

        let mut value = *items;
        if value.flags & HID_VALUE_CAPS_IS_BUTTON != 0 {
            value.logical_min = if value.bit_field & INPUT_ARRAY_VAR != 0 {
                0
            } else {
                value.logical_max
            };
            value.logical_max = 0;
            value.physical_min = 0;
            value.physical_max = 0;
        }
        self.values[t][slot] = value;
    }

    /// `build_preparsed_data`: the caps lists and the collection tree.
    fn build(self) -> PreparsedData {
        let count = usize::from(self.link_nodes);
        let mut collections = Vec::with_capacity(count);
        for node in &self.collections[..count] {
            collections.push(CollectionNode {
                usage_page: node.usage_page,
                usage: node.usage_min,
                parent: node.link_collection,
                number_of_children: 0,
                next_sibling: 0,
                first_child: 0,
                collection_type: node.bit_field,
            });
        }
        for index in 1..count {
            let parent = usize::from(collections[index].parent);
            collections[index].next_sibling = collections[parent].first_child;
            collections[parent].first_child = index as u16;
            let children = collections[parent].number_of_children;
            collections[parent].number_of_children = children.wrapping_add(1);
        }
        PreparsedData {
            usage_page: self.usage_page,
            usage: self.usage,
            input: self.report_caps(0),
            output: self.report_caps(1),
            feature: self.report_caps(2),
            collections,
        }
    }

    fn report_caps(&self, t: usize) -> ReportCaps {
        let count = self.caps_count[t];
        ReportCaps {
            count: count.wrapping_add(self.empty_caps[t]),
            byte_length: self.byte_length[t],
            values: self.values[t][..usize::from(count)].to_vec(),
        }
    }
}

/// `parse_descriptor`: parses the descriptor of one collection. `None` is the
/// C failure: a truncated item, an unbalanced end collection, a pop without a
/// push, a delimiter, or a value list that cannot grow.
pub fn parse_descriptor(desc: &[u8]) -> Option<PreparsedData> {
    let mut state = Parser::new();
    let mut pos = 0;
    while pos < desc.len() {
        let item = read_item(desc, pos)?;
        pos = item.next;
        if !state.apply(item) {
            return None;
        }
    }
    Some(state.build())
}

/// `parse_top_level_collections`: the offsets where the top-level collections
/// start and end. The first offset is 0, and collection `i` spans
/// `bounds[i]..bounds[i + 1]`. Scanning stops at a truncated item.
pub fn top_level_collections(desc: &[u8]) -> Vec<usize> {
    let mut bounds = vec![0];
    let mut depth = 0u32;
    let mut pos = 0;
    while pos < desc.len() {
        let Some(item) = read_item(desc, pos) else {
            break;
        };
        pos = item.next;
        match item.prefix & ITEM_MASK {
            MAIN_COLLECTION => depth = depth.wrapping_add(1),
            MAIN_END_COLLECTION => {
                depth = depth.wrapping_sub(1);
                if depth == 0 {
                    bounds.push(pos);
                }
            }
            _ => {}
        }
    }
    bounds
}
