/// `hid_device_add_*_count` limit on the report size, in bits.
pub const REPORT_BIT_LIMIT: u32 = 0x8_0000;
/// Number of axes (absolute and relative) a report layout can hold.
pub const AXIS_LIMIT: usize = 64;
/// Number of absolute axes that carry a usage.
pub const ABS_AXIS_USAGE_LIMIT: usize = 32;

/// `(x, y)` of each hatswitch nibble. Values 9 to 15 are null, as in C.
const HAT_DIRECTIONS: [(i32, i32); 16] = [
    (0, 0),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
];

/// Nibble value indexed by `[y + 1][x + 1]`, as in `hatswitch_compose`.
const HAT_VALUES: [[u8; 3]; 3] = [[8, 1, 2], [7, 0, 3], [6, 5, 4]];

/// A `USAGE_AND_PAGE`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UsageAndPage {
    pub usage: u16,
    pub usage_page: u16,
}

/// The `struct hid_device_state` fields that describe the input report layout.
///
/// The report buffer itself is passed to the writers. Byte offsets are `u16`, as
/// in C, and the counters wrap like C's `USHORT` arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HidDeviceLayout {
    pub bit_size: u32,
    pub axis_byte_offsets: [u16; AXIS_LIMIT],
    pub axis_sizes: [u8; AXIS_LIMIT],
    pub abs_axis_usages: [UsageAndPage; ABS_AXIS_USAGE_LIMIT],
    pub abs_axis_count: u16,
    pub rel_axis_count: u16,
    pub hatswitch_start: u16,
    pub hatswitch_count: u16,
    pub button_start: u16,
    pub button_count: u16,
}

impl Default for HidDeviceLayout {
    fn default() -> Self {
        Self {
            bit_size: 0,
            axis_byte_offsets: [0; AXIS_LIMIT],
            axis_sizes: [0; AXIS_LIMIT],
            abs_axis_usages: [UsageAndPage::default(); ABS_AXIS_USAGE_LIMIT],
            abs_axis_count: 0,
            rel_axis_count: 0,
            hatswitch_start: 0,
            hatswitch_count: 0,
            button_start: 0,
            button_count: 0,
        }
    }
}

impl HidDeviceLayout {
    /// `hid_device_add_button_count`.
    pub fn add_button_count(&mut self, count: u8) -> bool {
        if !self.bit_size.is_multiple_of(8) && self.button_count == 0 {
            return false;
        }
        if self.bit_size + u32::from(count) > REPORT_BIT_LIMIT {
            return false;
        }
        if self.button_count == 0 {
            self.button_start = byte_offset(self.bit_size);
        }
        self.button_count = self.button_count.wrapping_add(u16::from(count));
        self.bit_size += u32::from(count);
        true
    }

    /// `hid_device_add_hatswitch_count`. Each pair of hats takes one byte.
    pub fn add_hatswitch_count(&mut self, count: u8) -> bool {
        if self.button_count != 0 {
            return false;
        }
        if !self.bit_size.is_multiple_of(8) {
            return false;
        }
        if self.bit_size + 4 * (u32::from(count) + 1) > REPORT_BIT_LIMIT {
            return false;
        }
        if self.hatswitch_count == 0 {
            self.hatswitch_start = byte_offset(self.bit_size);
        }
        self.hatswitch_count = self.hatswitch_count.wrapping_add(u16::from(count));
        self.bit_size += 4 * u32::from(count);
        if count % 2 == 1 {
            self.bit_size += 4;
        }
        true
    }

    /// `hid_device_add_axis_count`. `usages` gives one usage per absolute axis.
    pub fn add_axis_count(
        &mut self,
        rel: bool,
        size: u8,
        count: u8,
        usage_page: u16,
        usages: &[u16],
    ) -> bool {
        if !rel && self.rel_axis_count != 0 {
            return false;
        }
        if self.button_count != 0 || self.hatswitch_count != 0 {
            return false;
        }
        if !self.bit_size.is_multiple_of(8) {
            return false;
        }
        if !matches!(size, 8 | 16 | 32) {
            return false;
        }
        if self.bit_size + u32::from(size) * u32::from(count) > REPORT_BIT_LIMIT {
            return false;
        }
        let axes = usize::from(self.abs_axis_count) + usize::from(self.rel_axis_count);
        if axes + usize::from(count) > AXIS_LIMIT {
            return false;
        }

        let offset = byte_offset(self.bit_size);
        if rel {
            for i in 0..usize::from(count) {
                let axis = axes + i;
                self.axis_byte_offsets[axis] = axis_offset(offset, i, size);
                self.axis_sizes[axis] = size;
            }
            self.rel_axis_count += u16::from(count);
        } else {
            let abs = usize::from(self.abs_axis_count);
            if abs + usize::from(count) > ABS_AXIS_USAGE_LIMIT {
                return false;
            }
            for (i, &usage) in usages[..usize::from(count)].iter().enumerate() {
                let axis = abs + i;
                self.axis_byte_offsets[axis] = axis_offset(offset, i, size);
                self.axis_sizes[axis] = size;
                self.abs_axis_usages[axis] = UsageAndPage { usage, usage_page };
            }
            self.abs_axis_count += u16::from(count);
        }
        self.bit_size += u32::from(size) * u32::from(count);
        true
    }

    /// `hid_device_set_abs_axis`. Writes `value` truncated to the axis size, little-endian.
    pub fn set_abs_axis(&self, report: &mut [u8], index: u32, value: i32) -> bool {
        if index >= u32::from(self.abs_axis_count) {
            return false;
        }
        self.write_axis(report, index as usize, value)
    }

    /// `hid_device_set_rel_axis`. Relative axes follow the absolute ones.
    pub fn set_rel_axis(&self, report: &mut [u8], index: u32, value: i32) -> bool {
        if index >= u32::from(self.rel_axis_count) {
            return false;
        }
        let axis = usize::from(self.abs_axis_count) + index as usize;
        self.write_axis(report, axis, value)
    }

    /// `hid_device_set_button`.
    pub fn set_button(&self, report: &mut [u8], index: u32, is_set: bool) -> bool {
        if index >= u32::from(self.button_count) {
            return false;
        }
        let offset = usize::from(self.button_start) + (index / 8) as usize;
        let mask = 1u8 << (index % 8);
        if is_set {
            report[offset] |= mask;
        } else {
            report[offset] &= !mask;
        }
        true
    }

    /// `hid_device_set_hatswitch_x`.
    ///
    /// Accepts `index == hatswitch_count`, as C does. With an even count that index
    /// lies past the last hat byte, so the write can land outside the hat fields.
    pub fn set_hatswitch_x(&self, report: &mut [u8], index: u32, new_x: i32) -> bool {
        let Some(offset) = self.hat_offset(index) else {
            return false;
        };
        let (_, y) = hatswitch_decompose(report[offset], index);
        hatswitch_compose(new_x, y, &mut report[offset], index);
        true
    }

    /// `hid_device_set_hatswitch_y`. Same index rule as [`Self::set_hatswitch_x`].
    pub fn set_hatswitch_y(&self, report: &mut [u8], index: u32, new_y: i32) -> bool {
        let Some(offset) = self.hat_offset(index) else {
            return false;
        };
        let (x, _) = hatswitch_decompose(report[offset], index);
        hatswitch_compose(x, new_y, &mut report[offset], index);
        true
    }

    /// `hid_device_move_hatswitch`. Adds `(x, y)` to the current direction.
    pub fn move_hatswitch(&self, report: &mut [u8], index: u32, x: i32, y: i32) -> bool {
        let Some(offset) = self.hat_offset(index) else {
            return false;
        };
        let (old_x, old_y) = hatswitch_decompose(report[offset], index);
        hatswitch_compose(
            old_x.wrapping_add(x),
            old_y.wrapping_add(y),
            &mut report[offset],
            index,
        );
        true
    }

    fn write_axis(&self, report: &mut [u8], axis: usize, value: i32) -> bool {
        let offset = usize::from(self.axis_byte_offsets[axis]);
        match self.axis_sizes[axis] {
            8 => report[offset] = value as u8,
            16 => report[offset..offset + 2].copy_from_slice(&(value as u16).to_le_bytes()),
            32 => report[offset..offset + 4].copy_from_slice(&value.to_le_bytes()),
            _ => return false,
        }
        true
    }

    fn hat_offset(&self, index: u32) -> Option<usize> {
        if index > u32::from(self.hatswitch_count) {
            return None;
        }
        Some(usize::from(self.hatswitch_start) + (index / 2) as usize)
    }
}

fn byte_offset(bit_size: u32) -> u16 {
    (bit_size / 8) as u16
}

fn axis_offset(base: u16, i: usize, size: u8) -> u16 {
    base + (i * usize::from(size) / 8) as u16
}

/// `hid_device_determine_axis_size`: the smallest of 8, 16 and 32 bits that holds the range.
pub fn determine_axis_size(min: i32, max: i32) -> u8 {
    if (min >= 0 && max <= 255) || (min >= -128 && max <= 127) {
        return 8;
    }
    if (min >= 0 && max <= 65535) || (min >= -32768 && max <= 32767) {
        return 16;
    }
    32
}

/// `hatswitch_decompose`: the `(x, y)` direction of the hat at `index` in `value`.
pub fn hatswitch_decompose(value: u8, index: u32) -> (i32, i32) {
    let nibble = if index % 2 == 1 {
        value >> 4
    } else {
        value & 0x0f
    };
    HAT_DIRECTIONS[usize::from(nibble)]
}

/// `hatswitch_compose`: stores the direction `(x, y)` in the nibble of hat `index`.
pub fn hatswitch_compose(x: i32, y: i32, value: &mut u8, index: u32) {
    let new_value = HAT_VALUES[(y.signum() + 1) as usize][(x.signum() + 1) as usize];
    if index % 2 == 1 {
        *value = (*value & 0x0f) | (new_value << 4);
    } else {
        *value = (*value & 0xf0) | new_value;
    }
}
