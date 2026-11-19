//! Index bytes of array fields. A cap whose `HAS_MORE` flag is set shares its index
//! range with the caps that follow it, so the range is counted from the end of the chain.

use crate::ValueCaps;

/// The index range of one array cap, as `i64` so that an empty usage range stays empty.
pub(crate) struct Slots {
    min: i64,
    max: i64,
}

impl Slots {
    pub(crate) fn contains(&self, index: u8) -> bool {
        (self.min..=self.max).contains(&i64::from(index))
    }

    /// The usage offset of `index`. Only meaningful when `contains(index)`.
    pub(crate) fn offset(&self, index: u8) -> u16 {
        (i64::from(index) - self.min) as u16
    }

    /// The index byte of the usage at `offset`.
    pub(crate) fn index(&self, offset: u16) -> i64 {
        self.min + i64::from(offset)
    }
}

/// The index range of the array cap at `i`, as `hidp.c` derives it.
pub(crate) fn slots(values: &[ValueCaps], i: usize) -> Slots {
    let mut end = i;
    while values[end].has_more() && end + 1 < values.len() {
        end += 1;
    }
    let min = (end - i + 1) as i64;
    let cap = &values[i];
    let max = min + i64::from(cap.usage_max) - i64::from(cap.usage_min);
    Slots { min, max }
}

/// The bit position of each index byte of an array cap.
pub(crate) fn slot_bits(cap: &ValueCaps) -> impl Iterator<Item = usize> {
    let first = cap.first_bit();
    (first..first + usize::from(cap.bit_size) * usize::from(cap.report_count)).step_by(8)
}
