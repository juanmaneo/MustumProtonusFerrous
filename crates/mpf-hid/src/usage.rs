//! Usage lists: the buttons that a report sets, and the functions that set and clear them.

use crate::array::{slot_bits, Slots};
use crate::bits::{get_bit, set_bit};
use crate::enumerate::{enumerate, nonzero, report_id_of, Filter, Sink, Visit};
use crate::{
    PreparsedData, ReportType, Status, UsageAndPage, ValueCaps, HIDP_STATUS_BUFFER_TOO_SMALL,
    HIDP_STATUS_BUTTON_NOT_PRESSED,
};

/// Calls `emit` with the usage offset of each usage that `report` sets in `cap`.
pub(crate) fn set_offsets(
    cap: &ValueCaps,
    slots: &Slots,
    report: &[u8],
    mut emit: impl FnMut(u16),
) {
    if cap.is_array() {
        for bit in slot_bits(cap) {
            let index = report[bit / 8];
            if slots.contains(index) {
                emit(slots.offset(index));
            }
        }
    } else {
        let first = cap.first_bit();
        let span = i64::from(cap.usage_max) - i64::from(cap.usage_min) + 1;
        for offset in 0..usize::try_from(span).unwrap_or(0) {
            if get_bit(report, first + offset) {
                emit(offset as u16);
            }
        }
    }
}

/// Lists the usages that `report` sets among the buttons that `filter` matches.
fn list<T>(
    p: &PreparsedData,
    rt: ReportType,
    filter: &Filter,
    report: &[u8],
    out: &mut [T],
    item: impl Fn(&ValueCaps, u16) -> T,
) -> Result<usize, Status> {
    let mut sink = Sink::new(out);
    enumerate(p, rt, report.len(), filter, &mut |cap, slots| {
        set_offsets(cap, &slots, report, |offset| {
            if let Some(slot) = sink.next_slot() {
                *slot = item(cap, offset);
            }
        });
        Ok(Visit::Continue)
    })?;
    sink.finish()
}

/// `HidP_GetUsages`.
pub fn get_usages(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    report: &[u8],
    out: &mut [u16],
) -> Result<usize, Status> {
    let filter = Filter {
        buttons: true,
        page: Some(page),
        collection: nonzero(co),
        report_id: report_id_of(report)?,
        ..Filter::default()
    };
    list(p, rt, &filter, report, out, |cap, offset| {
        cap.usage_min.wrapping_add(offset)
    })
}

/// `HidP_GetUsagesEx`.
pub fn get_usages_ex(
    p: &PreparsedData,
    rt: ReportType,
    co: u16,
    report: &[u8],
    out: &mut [UsageAndPage],
) -> Result<usize, Status> {
    let filter = Filter {
        buttons: true,
        collection: nonzero(co),
        report_id: report_id_of(report)?,
        ..Filter::default()
    };
    list(p, rt, &filter, report, out, |cap, offset| UsageAndPage {
        usage_page: cap.usage_page,
        usage: cap.usage_min.wrapping_add(offset),
    })
}

/// Sets one usage in `cap`. An array cap stores the usage's index in the first free byte.
fn set_usage(cap: &ValueCaps, slots: &Slots, usage: u16, report: &mut [u8]) -> Result<(), Status> {
    let offset = usage - cap.usage_min;
    if cap.is_array() {
        for bit in slot_bits(cap) {
            if report[bit / 8] == 0 {
                report[bit / 8] = slots.index(offset) as u8;
                return Ok(());
            }
        }
        return Err(HIDP_STATUS_BUFFER_TOO_SMALL);
    }
    set_bit(report, cap.first_bit() + usize::from(offset), true);
    Ok(())
}

/// Clears one usage in `cap`. An array cap clears the first index byte in its range,
/// whatever usage was named. Returns whether a usage was cleared.
fn unset_usage(cap: &ValueCaps, slots: &Slots, usage: u16, report: &mut [u8]) -> bool {
    if cap.is_array() {
        for bit in slot_bits(cap) {
            if slots.contains(report[bit / 8]) {
                report[bit / 8] = 0;
                return true;
            }
        }
        return false;
    }
    let bit = cap.first_bit() + usize::from(usage - cap.usage_min);
    let was_set = get_bit(report, bit);
    set_bit(report, bit, false);
    was_set
}

/// `HidP_SetUsages`.
pub fn set_usages(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usages: &[u16],
    report: &mut [u8],
) -> Result<(), Status> {
    let report_id = report_id_of(report)?;
    for &usage in usages {
        let filter = Filter {
            buttons: true,
            page: Some(page),
            collection: nonzero(co),
            usage: Some(usage),
            report_id,
            ..Filter::default()
        };
        enumerate(p, rt, report.len(), &filter, &mut |cap, slots| {
            set_usage(cap, &slots, usage, report)?;
            Ok(Visit::Stop)
        })?;
    }
    Ok(())
}

/// `HidP_UnsetUsages`.
pub fn unset_usages(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usages: &[u16],
    report: &mut [u8],
) -> Result<(), Status> {
    let report_id = report_id_of(report)?;
    let mut found = false;
    for &usage in usages {
        let filter = Filter {
            buttons: true,
            page: Some(page),
            collection: nonzero(co),
            usage: Some(usage),
            report_id,
            ..Filter::default()
        };
        enumerate(p, rt, report.len(), &filter, &mut |cap, slots| {
            found |= unset_usage(cap, &slots, usage, report);
            Ok(Visit::Stop)
        })?;
    }
    if found {
        Ok(())
    } else {
        Err(HIDP_STATUS_BUTTON_NOT_PRESSED)
    }
}
