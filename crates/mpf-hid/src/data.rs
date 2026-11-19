//! Data lists: every value and button of a report as `HidpData` items, and the reverse.

use crate::array::{slot_bits, Slots};
use crate::bits::{copy_bits, get_bit, set_bit};
use crate::enumerate::{enumerate, report_id_of, Filter, Sink, Visit};
use crate::usage::set_offsets;
use crate::value::read_field;
use crate::{
    HidpData, PreparsedData, ReportType, Status, ValueCaps, HIDP_STATUS_BUFFER_TOO_SMALL,
    HIDP_STATUS_BUTTON_NOT_PRESSED, HIDP_STATUS_IS_VALUE_ARRAY, HIDP_STATUS_REPORT_DOES_NOT_EXIST,
};

impl HidpData {
    /// `On` is the low byte of `RawValue`.
    fn on(self) -> bool {
        self.raw_value & 0xff != 0
    }

    /// `RawValue` with `On` set, keeping the other bytes.
    fn set_on(self) -> u32 {
        (self.raw_value & !0xff) | 1
    }
}

/// Appends the data items of one cap to `sink`.
fn get_data_item(
    cap: &ValueCaps,
    slots: &Slots,
    report: &[u8],
    sink: &mut Sink<HidpData>,
) -> Result<(), Status> {
    if cap.bit_size == 0 {
        return Ok(());
    }
    if cap.is_array() || cap.is_button() {
        set_offsets(cap, slots, report, |offset| {
            if let Some(slot) = sink.next_slot() {
                slot.data_index = cap.data_index_min.wrapping_add(offset);
                slot.raw_value = slot.set_on();
            }
        });
    } else if cap.report_count == 1 {
        if let Some(slot) = sink.next_slot() {
            let mut raw = [0u8; 4];
            read_field(&mut raw, usize::from(cap.bit_size), report, cap.first_bit())?;
            *slot = HidpData {
                data_index: cap.data_index_min,
                raw_value: u32::from_le_bytes(raw),
            };
        }
    }
    Ok(())
}

/// `HidP_GetData`.
pub fn get_data(
    p: &PreparsedData,
    rt: ReportType,
    report: &[u8],
    out: &mut [HidpData],
) -> Result<usize, Status> {
    let filter = Filter {
        report_id: report_id_of(report)?,
        ..Filter::default()
    };
    let mut sink = Sink::new(out);
    enumerate(p, rt, report.len(), &filter, &mut |cap, slots| {
        get_data_item(cap, &slots, report, &mut sink)?;
        Ok(Visit::Continue)
    })?;
    sink.finish()
}

/// Writes the array index for `offset` into the first index byte that is free or
/// holds `raw`'s usage. A zero `raw` clears that usage and shifts the bytes after it.
fn set_array_data(
    cap: &ValueCaps,
    slots: &Slots,
    raw: u32,
    offset: u16,
    report: &mut [u8],
) -> Result<(), Status> {
    let lookup = slots.index(offset);
    let hit = slot_bits(cap).find(|&bit| {
        let index = report[bit / 8];
        !slots.contains(index) || (raw == 0 && i64::from(index) == lookup)
    });
    let Some(bit) = hit else {
        return Err(if raw != 0 {
            HIDP_STATUS_BUFFER_TOO_SMALL
        } else {
            HIDP_STATUS_BUTTON_NOT_PRESSED
        });
    };
    let byte = bit / 8;
    if raw != 0 {
        report[byte] = lookup as u8;
    } else if i64::from(report[byte]) != lookup {
        return Err(HIDP_STATUS_BUTTON_NOT_PRESSED);
    } else {
        let last =
            (cap.first_bit() + usize::from(cap.bit_size) * usize::from(cap.report_count) - 1) / 8;
        report.copy_within(byte + 1..=last, byte);
        report[last] = 0;
    }
    Ok(())
}

/// Writes one data item into `report`. Sets `found` when `cap` holds the item's index.
fn set_data_item(
    cap: &ValueCaps,
    slots: &Slots,
    item: HidpData,
    report: &mut [u8],
    found: &mut bool,
) -> Result<(), Status> {
    if cap.bit_size == 0 || !(cap.data_index_min..=cap.data_index_max).contains(&item.data_index) {
        return Ok(());
    }
    *found = true;
    let offset = item.data_index - cap.data_index_min;
    if cap.is_array() {
        if !cap.is_range() {
            return Err(HIDP_STATUS_IS_VALUE_ARRAY);
        }
        set_array_data(cap, slots, item.raw_value, offset, report)
    } else if cap.is_button() {
        let bit = cap.first_bit() + usize::from(offset) * usize::from(cap.bit_size);
        if item.on() {
            set_bit(report, bit, true);
        } else if !get_bit(report, bit) {
            return Err(HIDP_STATUS_BUTTON_NOT_PRESSED);
        } else {
            set_bit(report, bit, false);
        }
        Ok(())
    } else if cap.report_count == 1 {
        let start_bit = usize::from(cap.start_bit);
        let bits = usize::from(cap.bit_size);
        if start_bit + bits > 32 {
            return Err(HIDP_STATUS_BUFFER_TOO_SMALL);
        }
        copy_bits(
            report,
            8 * usize::from(cap.start_byte),
            &item.raw_value.to_le_bytes(),
            start_bit,
            bits,
        );
        Ok(())
    } else {
        Ok(())
    }
}

/// `HidP_SetData`. Items are written in order, and the first item that no cap holds
/// stops the walk.
pub fn set_data(
    p: &PreparsedData,
    rt: ReportType,
    items: &[HidpData],
    report: &mut [u8],
) -> Result<(), Status> {
    let report_id = report_id_of(report)?;
    let mut found = false;
    let mut status: Result<(), Status> = Ok(());
    for &item in items {
        let filter = Filter {
            report_id,
            ..Filter::default()
        };
        status = enumerate(p, rt, report.len(), &filter, &mut |cap, slots| {
            set_data_item(cap, &slots, item, report, &mut found)?;
            Ok(Visit::Continue)
        })
        .map(|_| ());
        if status.is_err() || !found {
            break;
        }
    }
    if !found {
        return Err(HIDP_STATUS_REPORT_DOES_NOT_EXIST);
    }
    status
}
