//! The caps queries: `HidP_GetCaps`, the value and button caps, the list lengths, and
//! `HidP_InitializeReportForID`.

use crate::enumerate::{enumerate, nonzero, report_caps, Filter, Sink, Visit};
use crate::{
    Caps, PreparsedData, ReportType, Status, ValueCaps, HIDP_STATUS_INVALID_REPORT_LENGTH,
    HIDP_STATUS_REPORT_DOES_NOT_EXIST,
};

/// The totals of one report type that `HidP_GetCaps` reports.
#[derive(Default)]
struct Counts {
    buttons: u16,
    values: u16,
    data_indices: u16,
}

fn counts(values: &[ValueCaps]) -> Counts {
    let mut counts = Counts::default();
    for cap in values.iter().filter(|cap| cap.is_used()) {
        if cap.is_button() {
            counts.buttons = counts.buttons.wrapping_add(1);
        } else {
            counts.values = counts.values.wrapping_add(1);
        }
        let indices = if cap.is_range() {
            cap.data_index_max
                .wrapping_sub(cap.data_index_min)
                .wrapping_add(1)
        } else {
            1
        };
        counts.data_indices = counts.data_indices.wrapping_add(indices);
    }
    counts
}

/// `HidP_GetCaps`.
pub fn get_caps(p: &PreparsedData) -> Caps {
    let input = counts(&p.input.values);
    let output = counts(&p.output.values);
    let feature = counts(&p.feature.values);
    Caps {
        usage: p.usage,
        usage_page: p.usage_page,
        input_report_byte_length: p.input.byte_length,
        output_report_byte_length: p.output.byte_length,
        feature_report_byte_length: p.feature.byte_length,
        number_link_collection_nodes: p.link_collection_nodes,
        number_input_button_caps: input.buttons,
        number_input_value_caps: input.values,
        number_input_data_indices: input.data_indices,
        number_output_button_caps: output.buttons,
        number_output_value_caps: output.values,
        number_output_data_indices: output.data_indices,
        number_feature_button_caps: feature.buttons,
        number_feature_value_caps: feature.values,
        number_feature_data_indices: feature.data_indices,
    }
}

/// `HidP_MaxUsageListLength`. A page of 0 matches any page.
pub fn max_usage_list_length(p: &PreparsedData, rt: ReportType, page: u16) -> u32 {
    let filter = Filter {
        buttons: true,
        page: nonzero(page),
        ..Filter::default()
    };
    let mut count = 0u32;
    let _ = enumerate(p, rt, 0, &filter, &mut |cap, _| {
        count = count.wrapping_add(u32::from(cap.report_count));
        Ok(Visit::Continue)
    });
    count
}

/// `HidP_MaxDataListLength`.
pub fn max_data_list_length(p: &PreparsedData, rt: ReportType) -> u32 {
    let mut count = 0u32;
    let _ = enumerate(p, rt, 0, &Filter::default(), &mut |cap, _| {
        let items = if cap.is_range() || cap.is_button() {
            u32::from(cap.report_count)
        } else {
            1
        };
        count = count.wrapping_add(items);
        Ok(Visit::Continue)
    });
    count
}

/// Caps are matched on a page, collection and usage that are each "any" when zero.
fn specific_filter(page: u16, co: u16, usage: u16) -> Filter {
    Filter {
        page: nonzero(page),
        collection: nonzero(co),
        usage: nonzero(usage),
        ..Filter::default()
    }
}

fn specific_caps(
    p: &PreparsedData,
    rt: ReportType,
    filter: &Filter,
    out: &mut [ValueCaps],
) -> Result<usize, Status> {
    let mut sink = Sink::new(out);
    enumerate(p, rt, 0, filter, &mut |cap, _| {
        if let Some(slot) = sink.next_slot() {
            *slot = *cap;
        }
        Ok(Visit::Continue)
    })?;
    sink.finish()
}

/// `HidP_GetSpecificValueCaps`.
pub fn get_specific_value_caps(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    out: &mut [ValueCaps],
) -> Result<usize, Status> {
    let filter = Filter {
        values: true,
        ..specific_filter(page, co, usage)
    };
    specific_caps(p, rt, &filter, out)
}

/// `HidP_GetSpecificButtonCaps`.
pub fn get_specific_button_caps(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    out: &mut [ValueCaps],
) -> Result<usize, Status> {
    let filter = Filter {
        buttons: true,
        ..specific_filter(page, co, usage)
    };
    specific_caps(p, rt, &filter, out)
}

/// `HidP_GetValueCaps`.
pub fn get_value_caps(
    p: &PreparsedData,
    rt: ReportType,
    out: &mut [ValueCaps],
) -> Result<usize, Status> {
    get_specific_value_caps(p, rt, 0, 0, 0, out)
}

/// `HidP_GetButtonCaps`.
pub fn get_button_caps(
    p: &PreparsedData,
    rt: ReportType,
    out: &mut [ValueCaps],
) -> Result<usize, Status> {
    get_specific_button_caps(p, rt, 0, 0, 0, out)
}

/// `HidP_InitializeReportForID`: clears `report` and sets its first byte to `report_id`.
pub fn initialize_report_for_id(
    p: &PreparsedData,
    rt: ReportType,
    report_id: u8,
    report: &mut [u8],
) -> Result<(), Status> {
    if report.is_empty() {
        return Err(HIDP_STATUS_INVALID_REPORT_LENGTH);
    }
    let values = report_caps(p, rt, report.len())?;
    if !values
        .iter()
        .any(|cap| cap.report_id == report_id && cap.is_used())
    {
        return Err(HIDP_STATUS_REPORT_DOES_NOT_EXIST);
    }
    report.fill(0);
    report[0] = report_id;
    Ok(())
}
