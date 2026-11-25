use crate::{TIME_BYTES, TIME_MS, TIME_SAMPLES};

/// `MMTIME`: the `wType` word and the 8-byte `u` union, kept as raw memory. On a little-endian
/// host the first dword of `u` is `cb`, `ms`, `sample` or `ticks`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MmTime {
    pub wtype: u32,
    pub u: [u8; 8],
}

impl MmTime {
    fn dword(&self) -> u32 {
        u32::from_le_bytes([self.u[0], self.u[1], self.u[2], self.u[3]])
    }

    fn set_dword(&mut self, value: u32) {
        self.u[..4].copy_from_slice(&value.to_le_bytes());
    }
}

/// The rate fields of `WAVEMAPDATA` that `wodGetPosition` reads. "Outer" is the client
/// format and "inner" is the device format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rates {
    pub avg_speed_outer: u32,
    pub avg_speed_inner: u32,
    pub samples_per_sec_outer: u32,
    pub samples_per_sec_inner: u32,
}

/// `MulDiv` from `dlls/kernelbase/main.c`: `a * b / c`, rounded to nearest with halves away
/// from zero. Returns `-1` for a zero divisor or a result outside `-i32::MAX..=i32::MAX`.
///
/// Wine negates a negative divisor with C `-`, which is undefined for `i32::MIN`. This port
/// wraps instead, so `i32::MIN` stays negative and the division still runs.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (a, c) = if c < 0 {
        (a.wrapping_neg(), c.wrapping_neg())
    } else {
        (a, c)
    };
    let product = i64::from(a) * i64::from(b);
    let half = i64::from(c / 2);
    let ret = if (a < 0 && b < 0) || (a >= 0 && b >= 0) {
        (product + half) / i64::from(c)
    } else {
        (product - half) / i64::from(c)
    };
    let limit = i64::from(i32::MAX);
    if (-limit..=limit).contains(&ret) {
        ret as i32
    } else {
        -1
    }
}

/// The time query that `wodGetPosition` passes to the device. A `TIME_MS` request is sent
/// as `TIME_BYTES`, because the millisecond value is derived from the byte count. Any other
/// request is sent unchanged.
pub fn position_query(req: &MmTime) -> MmTime {
    let mut query = *req;
    if req.wtype == TIME_MS {
        query.wtype = TIME_BYTES;
    }
    query
}

/// `wodGetPosition` / `widGetPosition` (identical bodies), after the device call. `reply` is
/// what the device wrote back for the query from [`position_query`] applied to `req`. The
/// result is the caller's `lpTime`. Bytes that a conversion leaves alone come from `req`.
///
/// # Panics
///
/// Panics where the C code divides by zero: a zero `samples_per_sec_outer` in the byte
/// branch, and a zero `avg_speed_inner` with a `TIME_BYTES` reply that needs alignment.
pub fn get_position(rates: &Rates, req: &MmTime, reply: &MmTime) -> MmTime {
    let mut out = *req;
    if reply.wtype == TIME_BYTES {
        let mut cb = reply.dword();
        let inner_per_outer = rates.samples_per_sec_inner / rates.samples_per_sec_outer;
        if inner_per_outer > 0 {
            let inner_bytes_per_sample = rates.avg_speed_inner / rates.samples_per_sec_inner;
            let inner_bytes_per_outer_sample = inner_bytes_per_sample.wrapping_mul(inner_per_outer);
            let remainder = cb % inner_bytes_per_outer_sample;
            if remainder > 0 {
                cb = cb
                    .wrapping_sub(remainder)
                    .wrapping_add(inner_bytes_per_outer_sample);
            }
        }
        out.set_dword(mul_div(
            cb as i32,
            rates.avg_speed_outer as i32,
            rates.avg_speed_inner as i32,
        ) as u32);
        if req.wtype == TIME_MS {
            out.set_dword(mul_div(out.dword() as i32, 1000, rates.avg_speed_outer as i32) as u32);
        } else {
            out.wtype = TIME_BYTES;
        }
    } else if req.wtype == TIME_SAMPLES && reply.wtype == TIME_SAMPLES {
        out.set_dword(mul_div(
            reply.dword() as i32,
            rates.samples_per_sec_outer as i32,
            rates.samples_per_sec_inner as i32,
        ) as u32);
    } else {
        out.u = reply.u;
    }
    out
}
