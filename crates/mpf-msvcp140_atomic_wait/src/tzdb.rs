/// `z->ver` in `__std_tzdb_get_time_zones`.
const TZDB_VERSION: &str = "2022g";

/// `enum tzdb_error`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TzdbError {
    Success = 0,
    Win = 1,
    Icu = 2,
}

/// `struct tzdb_time_zones`. Each name is the C buffer, including its terminating zero byte.
/// `links` has one NULL slot for each zone the first pass counted. The C code never fills them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TzdbTimeZones {
    pub error: TzdbError,
    pub ver: &'static str,
    pub names: Vec<Vec<u8>>,
    pub links: Vec<Option<Vec<u8>>>,
}

/// `struct tzdb_current_zone`. `name` is `None` (NULL) when `error` is `Win`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TzdbCurrentZone {
    pub error: TzdbError,
    pub name: Option<Vec<u8>>,
}

/// `struct tzdb_leap_second`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TzdbLeapSecond {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub negative: u16,
    pub reserved: u16,
}

/// `__std_tzdb_get_time_zones`. `enumerate(index)` stands for `EnumDynamicTimeZoneInformation`:
/// `Some(StandardName)` when it returns 0, and `None` when it returns an error. The first pass
/// counts the zones up to the first error. The second pass copies their names and stops at the
/// first error, as the C loop does.
pub fn std_tzdb_get_time_zones(enumerate: &dyn Fn(u32) -> Option<Vec<u16>>) -> TzdbTimeZones {
    let mut count = 0;
    while enumerate(count).is_some() {
        count += 1;
    }
    let mut names = Vec::new();
    for index in 0..count {
        let Some(standard_name) = enumerate(index) else {
            break;
        };
        names.push(copy_wide_name(&standard_name));
    }
    TzdbTimeZones {
        error: TzdbError::Success,
        ver: TZDB_VERSION,
        names,
        links: vec![None; count as usize],
    }
}

/// `__std_tzdb_get_current_zone`. `get_dynamic_time_zone` stands for
/// `GetDynamicTimeZoneInformation`: `Some(StandardName)` on success, and `None` when it returns
/// `TIME_ZONE_ID_INVALID`.
pub fn std_tzdb_get_current_zone(
    get_dynamic_time_zone: &dyn Fn() -> Option<Vec<u16>>,
) -> TzdbCurrentZone {
    match get_dynamic_time_zone() {
        Some(standard_name) => TzdbCurrentZone {
            error: TzdbError::Success,
            name: Some(copy_wide_name(&standard_name)),
        },
        None => TzdbCurrentZone {
            error: TzdbError::Win,
            name: None,
        },
    }
}

/// `__std_tzdb_get_leap_seconds`. Wine has no leap second table: the C function writes 0 to
/// `*new_size` and returns NULL. The port returns the table (`None`) and the new size. `prev_size`
/// is unused, as in the C code.
pub fn std_tzdb_get_leap_seconds(_prev_size: usize) -> (Option<Vec<TzdbLeapSecond>>, usize) {
    (None, 0)
}

/// Copies a WCHAR name the way the C loop does: a `malloc(wcslen + 1)` buffer is filled by storing
/// each WCHAR as a `char`, which keeps only the low byte, until a stored byte is zero. A WCHAR whose
/// low byte is zero therefore ends the name early. The C code leaves the bytes after that point
/// indeterminate. The port zero-fills them.
fn copy_wide_name(standard_name: &[u16]) -> Vec<u8> {
    let mut name = vec![0; wide_len(standard_name) + 1];
    for (slot, &wc) in name.iter_mut().zip(standard_name) {
        *slot = wc as u8;
        if *slot == 0 {
            break;
        }
    }
    name
}

fn wide_len(standard_name: &[u16]) -> usize {
    standard_name
        .iter()
        .position(|&wc| wc == 0)
        .unwrap_or(standard_name.len())
}
