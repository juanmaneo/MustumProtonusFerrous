use crate::com::{E_INVALIDARG, HRESULT};

/// One `struct range` from `dlls/mfmediaengine/main.c`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Range {
    pub start: f64,
    pub end: f64,
}

/// The ranges behind `IMFMediaTimeRange` (`struct time_range` in `dlls/mfmediaengine/main.c`).
/// The COM reference count is not modelled, and a `Vec` stores the ranges in place of the C array.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimeRange {
    ranges: Vec<Range>,
}

impl TimeRange {
    /// The stored ranges, in the order `add_range` left them.
    pub fn ranges(&self) -> &[Range] {
        &self.ranges
    }

    /// `time_range_GetLength`. The count is narrowed to `DWORD` as the C return is.
    pub fn length(&self) -> u32 {
        self.ranges.len() as u32
    }

    /// `time_range_GetStart`: `E_INVALIDARG` for an index at or past the count.
    pub fn start(&self, idx: u32) -> Result<f64, HRESULT> {
        self.get(idx).map(|range| range.start)
    }

    /// `time_range_GetEnd`: `E_INVALIDARG` for an index at or past the count.
    pub fn end(&self, idx: u32) -> Result<f64, HRESULT> {
        self.get(idx).map(|range| range.end)
    }

    fn get(&self, idx: u32) -> Result<&Range, HRESULT> {
        self.ranges.get(idx as usize).ok_or(E_INVALIDARG)
    }

    /// `time_range_ContainsTime`. Both ends are inclusive.
    pub fn contains_time(&self, time: f64) -> bool {
        self.ranges
            .iter()
            .any(|range| time >= range.start && time <= range.end)
    }

    /// `time_range_AddRange`. The first stored range that contains the new one, or that the new one
    /// contains or intersects, absorbs it, and the later ranges are not checked. Otherwise the new
    /// range is appended, unsorted. `min` and `max` are Wine's macros rather than `f64::min` and
    /// `f64::max`, so a NaN bound picks the same operand as the C code.
    pub fn add_range(&mut self, start: f64, end: f64) {
        for range in &mut self.ranges {
            if range.start <= start && range.end >= end {
                return;
            }
            if range.start >= start && range.end <= end {
                range.start = start;
                range.end = end;
                return;
            }
            if (start >= range.start && start <= range.end)
                || (end >= range.start && end <= range.end)
            {
                range.start = if range.start < start {
                    range.start
                } else {
                    start
                };
                range.end = if range.end > end { range.end } else { end };
                return;
            }
        }
        self.ranges.push(Range { start, end });
    }

    /// `time_range_Clear`. The storage is kept, as the C code keeps its array allocated.
    pub fn clear(&mut self) {
        self.ranges.clear();
    }
}
