/// `TAPIERR_REQUESTFAILED` from `include/tapi.h`, returned when the current location cannot be read.
const TAPIERR_REQUESTFAILED: u32 = 0xFFFF_FFF0;

/// The size of the `buf[200]` that `tapiGetLocationInfoW` reads each location value into. A value
/// that does not fit is not used.
const VALUE_BUFFER_BYTES: usize = 200;

/// `lstrcpynW(citycode, buf, 8)` copies at most 7 characters.
const CITY_CODE_MAX: usize = 7;

/// `swprintf(countrycode, 8, u"%u", ...)` writes at most 7 characters.
const COUNTRY_CODE_MAX: usize = 7;

/// A registry value that the location lookup reads. `Sz` holds the UTF-16 units of `REG_SZ`
/// data, including the terminating NUL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegData {
    Dword(u32),
    Sz(Vec<u16>),
}

/// The registry state that `tapiGetLocationInfoW` reads, under
/// `Software\Microsoft\Windows\CurrentVersion\Telephony\Locations`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationRegistry {
    /// Whether the `Locations` key opens.
    pub key_opens: bool,
    /// The `CurrentID` value, or `None` when it is missing.
    pub current_id: Option<RegData>,
    /// Whether the `Location<CurrentID>` subkey opens.
    pub subkey_opens: bool,
    /// The `AreaCode` value of the subkey, or `None` when it is missing.
    pub area_code: Option<RegData>,
    /// The `Country` value of the subkey, or `None` when it is missing.
    pub country: Option<RegData>,
}

/// The strings that `tapiGetLocationInfoW` writes. A string is `None` when its out-parameter is NULL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationInfo {
    pub country: Option<Vec<u16>>,
    pub city: Option<Vec<u16>>,
}

/// The strings that `tapiGetLocationInfoA` writes, after `WideCharToMultiByte`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnsiLocationInfo {
    pub country: Option<Vec<u8>>,
    pub city: Option<Vec<u8>>,
}

/// `tapiGetLocationInfoW` from `dlls/tapi32/assisted.c`. The lookup fails unless the `Locations`
/// key opens, `CurrentID` is a `REG_DWORD`, and the `Location<CurrentID>` subkey opens. A value that
/// is missing or has the wrong type gives an empty string, not a failure.
pub fn tapi_get_location_info_w(
    reg: &LocationRegistry,
    want_country: bool,
    want_city: bool,
) -> Result<LocationInfo, u32> {
    let current_id_is_dword = matches!(reg.current_id, Some(RegData::Dword(_)));
    if !(reg.key_opens && current_id_is_dword && reg.subkey_opens) {
        return Err(TAPIERR_REQUESTFAILED);
    }
    Ok(LocationInfo {
        country: want_country.then(|| country_code_units(reg.country.as_ref())),
        city: want_city.then(|| area_code_units(reg.area_code.as_ref())),
    })
}

/// `tapiGetLocationInfoA` from `dlls/tapi32/assisted.c`: the W lookup, with each string converted
/// by `to_ansi`.
pub fn tapi_get_location_info_a(
    reg: &LocationRegistry,
    want_country: bool,
    want_city: bool,
) -> Result<AnsiLocationInfo, u32> {
    let info = tapi_get_location_info_w(reg, want_country, want_city)?;
    Ok(AnsiLocationInfo {
        country: info.country.as_deref().map(to_ansi),
        city: info.city.as_deref().map(to_ansi),
    })
}

/// The `AreaCode` string. Data over the buffer size, or data of another type, is empty.
fn area_code_units(value: Option<&RegData>) -> Vec<u16> {
    match value {
        Some(RegData::Sz(units)) if units.len() * 2 <= VALUE_BUFFER_BYTES => units
            .iter()
            .take(CITY_CODE_MAX)
            .take_while(|&&unit| unit != 0)
            .copied()
            .collect(),
        _ => Vec::new(),
    }
}

/// The `Country` string, the decimal value of a `REG_DWORD`. Any other value is empty.
fn country_code_units(value: Option<&RegData>) -> Vec<u16> {
    match value {
        Some(RegData::Dword(country)) => country
            .to_string()
            .encode_utf16()
            .take(COUNTRY_CODE_MAX)
            .collect(),
        _ => Vec::new(),
    }
}

/// `WideCharToMultiByte(CP_ACP, ...)` for one string. Only ASCII is mapped. Any other unit becomes
/// `?`, which is the default character. Code page tables are not modelled.
fn to_ansi(units: &[u16]) -> Vec<u8> {
    units
        .iter()
        .map(|&unit| if unit < 0x80 { unit as u8 } else { b'?' })
        .collect()
}
