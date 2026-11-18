/// An input report as queued for a handle (`struct hid_report`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidReport {
    buffer: Vec<u8>,
}

impl HidReport {
    /// Copies `payload` into a report of `length` bytes and zero-fills the rest
    /// (`hid_report_create`).
    ///
    /// Returns `None` when `payload` is longer than `length`. C writes past the
    /// allocation in that case.
    pub fn new(payload: &[u8], length: usize) -> Option<Self> {
        if payload.len() > length {
            return None;
        }
        let mut buffer = vec![0; length];
        buffer[..payload.len()].copy_from_slice(payload);
        Some(Self { buffer })
    }

    /// The report bytes, `report->buffer`.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

/// One `HIDP_REPORT_IDS` entry of a device description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReportIds {
    pub report_id: u8,
    pub collection_number: u8,
    pub input_length: u16,
    pub output_length: u16,
    pub feature_length: u16,
}

/// The `HIDP_REPORT_TYPE` values that `find_report_with_type_and_id` matches on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    Input,
    Output,
    Feature,
}

/// `find_report_with_type_and_id`: the index of the first entry that has a
/// non-zero length for `report_type`.
///
/// `collection` 0 matches any collection. An entry with report ID 0 matches
/// any `id`. `any_id` ignores the ID altogether.
pub fn find_report_with_type_and_id(
    reports: &[ReportIds],
    collection: u8,
    report_type: ReportType,
    id: u8,
    any_id: bool,
) -> Option<usize> {
    reports.iter().position(|report| {
        let has_length = match report_type {
            ReportType::Input => report.input_length != 0,
            ReportType::Output => report.output_length != 0,
            ReportType::Feature => report.feature_length != 0,
        };
        (collection == 0 || collection == report.collection_number)
            && (any_id || report.report_id == 0 || report.report_id == id)
            && has_length
    })
}
