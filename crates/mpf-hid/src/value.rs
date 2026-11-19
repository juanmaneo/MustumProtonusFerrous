//! Usage values: the `HidP_*UsageValue*` readers and writers, and the scaling between
//! a logical value and its physical value.

use crate::bits::copy_bits;
use crate::enumerate::{enumerate, nonzero, report_id_of, Filter, Visit};
use crate::{
    PreparsedData, ReportType, Status, ValueCaps, HIDP_STATUS_BAD_LOG_PHY_VALUES,
    HIDP_STATUS_BUFFER_TOO_SMALL, HIDP_STATUS_VALUE_OUT_OF_RANGE,
};

/// Copies a field of `count` bits from `report` into `out`, zero-filled first.
pub(crate) fn read_field(
    out: &mut [u8],
    count: usize,
    report: &[u8],
    first: usize,
) -> Result<(), Status> {
    if count.div_ceil(8) > out.len() {
        return Err(HIDP_STATUS_BUFFER_TOO_SMALL);
    }
    out.fill(0);
    copy_bits(out, 0, report, first, count);
    Ok(())
}

/// Copies the low `count` bits of `value` into a field of `report`.
pub(crate) fn write_field(
    report: &mut [u8],
    first: usize,
    count: usize,
    value: &[u8],
) -> Result<(), Status> {
    if count.div_ceil(8) > value.len() {
        return Err(HIDP_STATUS_BUFFER_TOO_SMALL);
    }
    copy_bits(report, first, value, 0, count);
    Ok(())
}

/// The first bit and the width of the field of one usage of a scalar cap.
fn scalar_bits(cap: &ValueCaps, usage: u16) -> (usize, usize) {
    let offset = usize::from(usage - cap.usage_min) * usize::from(cap.bit_size);
    (cap.first_bit() + offset, usize::from(cap.bit_size))
}

/// The first bit and the width of the whole index array of an array cap.
fn array_bits(cap: &ValueCaps) -> (usize, usize) {
    (
        cap.first_bit(),
        usize::from(cap.bit_size) * usize::from(cap.report_count),
    )
}

/// `sign_extend`: a field of `bit_size` bits, read as signed when the logical minimum
/// is negative.
pub(crate) fn sign_extend(value: u32, cap: &ValueCaps) -> i32 {
    let sign = 1u32
        .checked_shl(u32::from(cap.bit_size).wrapping_sub(1))
        .unwrap_or(0);
    if sign <= 1 || cap.logical_min >= 0 {
        return value as i32;
    }
    (i64::from(value) - (i64::from(value & sign) << 1)) as i32
}

/// `MulDiv`: `a * b / c` rounded to nearest, or -1 on a zero divisor or overflow.
pub(crate) fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 {
        (-i64::from(a), -i64::from(c))
    } else {
        (i64::from(a), i64::from(c))
    };
    let product = a * i64::from(b);
    let half = c / 2;
    let ret = if (a < 0) == (b < 0) {
        (product + half) / c
    } else {
        (product - half) / c
    };
    if ret > i64::from(i32::MAX) || ret < -i64::from(i32::MAX) {
        -1
    } else {
        ret as i32
    }
}

/// The physical value of a logical value, as `get_scaled_usage_value` reports it.
fn scale_value(cap: &ValueCaps, logical: i32) -> Result<i32, Status> {
    if cap.logical_min > cap.logical_max || cap.physical_min > cap.physical_max {
        return Err(HIDP_STATUS_BAD_LOG_PHY_VALUES);
    }
    if !(cap.logical_min..=cap.logical_max).contains(&logical) {
        return Err(HIDP_STATUS_VALUE_OUT_OF_RANGE);
    }
    if cap.physical_min == 0 && cap.physical_max == 0 {
        return Ok(logical);
    }
    Ok(cap.physical_min.wrapping_add(mul_div(
        logical.wrapping_sub(cap.logical_min),
        cap.physical_max.wrapping_sub(cap.physical_min),
        cap.logical_max.wrapping_sub(cap.logical_min),
    )))
}

/// The inverse of `scale_value`, with the rounding that `set_scaled_usage_value` does.
fn unscale_value(cap: &ValueCaps, physical: i32) -> Result<i32, Status> {
    if cap.logical_min > cap.logical_max || cap.physical_min > cap.physical_max {
        return Err(HIDP_STATUS_BAD_LOG_PHY_VALUES);
    }
    if cap.physical_min == 0 && cap.physical_max == 0 {
        return Ok(physical);
    }
    let log_range = cap
        .logical_max
        .wrapping_sub(cap.logical_min)
        .wrapping_add(1)
        / 2;
    let phy_range = cap
        .physical_max
        .wrapping_sub(cap.physical_min)
        .wrapping_add(1)
        / 2;
    if phy_range == 0 {
        return Err(HIDP_STATUS_BAD_LOG_PHY_VALUES);
    }
    let offset = log_range
        .wrapping_mul(physical.wrapping_sub(cap.physical_min))
        .wrapping_div(phy_range);
    Ok(cap.logical_min.wrapping_add(offset))
}

fn value_filter(
    report: &[u8],
    array: bool,
    page: u16,
    co: u16,
    usage: u16,
) -> Result<Filter, Status> {
    Ok(Filter {
        values: true,
        array,
        page: Some(page),
        collection: nonzero(co),
        usage: Some(usage),
        report_id: report_id_of(report)?,
        ..Filter::default()
    })
}

/// `HidP_GetUsageValue`.
pub fn get_usage_value(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    report: &[u8],
) -> Result<u32, Status> {
    let mut value = [0u8; 4];
    let filter = value_filter(report, false, page, co, usage)?;
    enumerate(p, rt, report.len(), &filter, &mut |cap, _| {
        let (first, count) = scalar_bits(cap, usage);
        read_field(&mut value, count, report, first)?;
        Ok(Visit::Stop)
    })?;
    Ok(u32::from_le_bytes(value))
}

/// `HidP_GetUsageValueArray`.
pub fn get_usage_value_array(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    value: &mut [u8],
    report: &[u8],
) -> Result<(), Status> {
    let filter = value_filter(report, true, page, co, usage)?;
    enumerate(p, rt, report.len(), &filter, &mut |cap, _| {
        let (first, count) = array_bits(cap);
        read_field(value, count, report, first)?;
        Ok(Visit::Stop)
    })?;
    Ok(())
}

/// `HidP_GetScaledUsageValue`.
pub fn get_scaled_usage_value(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    report: &[u8],
) -> Result<i32, Status> {
    let mut result = 0;
    let filter = value_filter(report, false, page, co, usage)?;
    enumerate(p, rt, report.len(), &filter, &mut |cap, _| {
        let (first, count) = scalar_bits(cap, usage);
        let mut raw = [0u8; 4];
        read_field(&mut raw, count, report, first)?;
        result = scale_value(cap, sign_extend(u32::from_le_bytes(raw), cap))?;
        Ok(Visit::Stop)
    })?;
    Ok(result)
}

/// `HidP_SetUsageValue`.
pub fn set_usage_value(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    value: u32,
    report: &mut [u8],
) -> Result<(), Status> {
    let filter = value_filter(report, false, page, co, usage)?;
    enumerate(p, rt, report.len(), &filter, &mut |cap, _| {
        let (first, count) = scalar_bits(cap, usage);
        write_field(report, first, count, &value.to_le_bytes())?;
        Ok(Visit::Stop)
    })?;
    Ok(())
}

/// `HidP_SetUsageValueArray`.
pub fn set_usage_value_array(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    value: &[u8],
    report: &mut [u8],
) -> Result<(), Status> {
    let filter = value_filter(report, true, page, co, usage)?;
    enumerate(p, rt, report.len(), &filter, &mut |cap, _| {
        let (first, count) = array_bits(cap);
        write_field(report, first, count, value)?;
        Ok(Visit::Stop)
    })?;
    Ok(())
}

/// `HidP_SetScaledUsageValue`.
pub fn set_scaled_usage_value(
    p: &PreparsedData,
    rt: ReportType,
    page: u16,
    co: u16,
    usage: u16,
    value: i32,
    report: &mut [u8],
) -> Result<(), Status> {
    let filter = value_filter(report, false, page, co, usage)?;
    enumerate(p, rt, report.len(), &filter, &mut |cap, _| {
        let (first, count) = scalar_bits(cap, usage);
        let scaled = unscale_value(cap, value)?;
        write_field(report, first, count, &scaled.to_le_bytes())?;
        Ok(Visit::Stop)
    })?;
    Ok(())
}
