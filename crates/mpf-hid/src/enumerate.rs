//! Walks the caps that match a filter in descriptor order, as `enum_value_caps` does.

use crate::array::{slots, Slots};
use crate::{
    PreparsedData, ReportType, Status, ValueCaps, HIDP_STATUS_BUFFER_TOO_SMALL,
    HIDP_STATUS_INCOMPATIBLE_REPORT_ID, HIDP_STATUS_INVALID_REPORT_LENGTH,
    HIDP_STATUS_NOT_VALUE_ARRAY, HIDP_STATUS_USAGE_NOT_FOUND,
};

/// `struct caps_filter`. `None` matches any value of that field.
#[derive(Default)]
pub(crate) struct Filter {
    pub(crate) buttons: bool,
    pub(crate) values: bool,
    pub(crate) array: bool,
    pub(crate) page: Option<u16>,
    pub(crate) collection: Option<u16>,
    pub(crate) usage: Option<u16>,
    pub(crate) report_id: u8,
}

impl Filter {
    fn matches(&self, cap: &ValueCaps) -> bool {
        if !cap.is_used() {
            return false;
        }
        if self.buttons && !cap.is_button() {
            return false;
        }
        if self.values && cap.is_button() {
            return false;
        }
        if self.page.is_some_and(|page| page != cap.usage_page) {
            return false;
        }
        if self.collection.is_some_and(|co| co != cap.link_collection) {
            return false;
        }
        self.usage
            .is_none_or(|usage| (cap.usage_min..=cap.usage_max).contains(&usage))
    }
}

/// Whether the walk goes on after a visited cap.
pub(crate) enum Visit {
    Continue,
    Stop,
}

/// The caps of one report type. A nonzero `report_len` must equal the report length.
pub(crate) fn report_caps(
    p: &PreparsedData,
    rt: ReportType,
    report_len: usize,
) -> Result<&[ValueCaps], Status> {
    let report = p.report(rt);
    if report_len != 0 && report_len != usize::from(report.byte_length) {
        return Err(HIDP_STATUS_INVALID_REPORT_LENGTH);
    }
    Ok(&report.values)
}

/// Visits each matching cap with its index range. Caps whose report ID differs from a
/// nonzero `filter.report_id` are skipped, and that sets the error when nothing matched.
/// Returns how many caps were visited. The visitor is a trait object so that the walk is
/// compiled once rather than once per caller.
pub(crate) fn enumerate(
    p: &PreparsedData,
    rt: ReportType,
    report_len: usize,
    filter: &Filter,
    visit: &mut dyn FnMut(&ValueCaps, Slots) -> Result<Visit, Status>,
) -> Result<usize, Status> {
    let values = report_caps(p, rt, report_len)?;
    let mut count = 0;
    let mut incompatible = false;
    for (i, cap) in values.iter().enumerate() {
        if !filter.matches(cap) {
            continue;
        }
        if filter.report_id != 0 && cap.report_id != filter.report_id {
            incompatible = true;
        } else if filter.array && (cap.is_range() || cap.report_count <= 1) {
            return Err(HIDP_STATUS_NOT_VALUE_ARRAY);
        } else {
            count += 1;
            if matches!(visit(cap, slots(values, i))?, Visit::Stop) {
                break;
            }
        }
    }
    if count == 0 {
        return Err(if incompatible {
            HIDP_STATUS_INCOMPATIBLE_REPORT_ID
        } else {
            HIDP_STATUS_USAGE_NOT_FOUND
        });
    }
    Ok(count)
}

/// A caller's output slice. Every item is counted, and `finish` reports the overflow.
pub(crate) struct Sink<'a, T> {
    out: &'a mut [T],
    count: usize,
}

impl<'a, T> Sink<'a, T> {
    pub(crate) fn new(out: &'a mut [T]) -> Self {
        Sink { out, count: 0 }
    }

    /// The slot for the next item, or `None` when the output is full.
    pub(crate) fn next_slot(&mut self) -> Option<&mut T> {
        let slot = self.out.get_mut(self.count);
        self.count += 1;
        slot
    }

    pub(crate) fn finish(self) -> Result<usize, Status> {
        if self.count > self.out.len() {
            Err(HIDP_STATUS_BUFFER_TOO_SMALL)
        } else {
            Ok(self.count)
        }
    }
}

/// The report ID that the first byte of a report selects.
pub(crate) fn report_id_of(report: &[u8]) -> Result<u8, Status> {
    report
        .first()
        .copied()
        .ok_or(HIDP_STATUS_INVALID_REPORT_LENGTH)
}

/// A filter value that the C code treats as "any" when it is zero.
pub(crate) fn nonzero(value: u16) -> Option<u16> {
    (value != 0).then_some(value)
}
