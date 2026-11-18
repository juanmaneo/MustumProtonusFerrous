//! Headless subset of Wine's `dlls/msdmo` (`dmoreg.c` and `dmort.c`).
//!
//! The ported functions compute what Wine's C bodies compute from their
//! arguments. The registry, COM objects (`IEnumDMO`, the `pUnk` member of a
//! media type) and the `CoTaskMem` heap are not modelled.
//!
//! Ported:
//! - `array_reserve`: [`array_reserve`], the capacity growth policy; the `realloc` itself is skipped.
//! - `GUIDToString`: [`guid_to_string`], the registry key name of a DMO.
//! - `IsMediaTypeEqual` and `any_types_match`: [`is_media_type_equal`] and [`any_types_match`], the wildcard type filter of `IEnumDMO::Next`.
//! - `DMOGetTypes`: [`dmo_get_types`], the buffer sizes and returned counts; the registry reads go through a caller closure.
//! - `MoInitMediaType`: [`mo_init_media_type`].
//! - `MoCopyMediaType`: [`mo_copy_media_type`]; the `pUnk` `AddRef` is skipped.
//! - `MoFreeMediaType`: [`mo_free_media_type`]; the `pUnk` `Release` is skipped.
//!
//! Skipped:
//! - `MoCreateMediaType`, `MoDeleteMediaType`, `MoDuplicateMediaType`: `CoTaskMem` allocation wrappers around the ported init, free and copy.
//! - `DMORegister`, `DMOUnregister`, `DMOGetName`: registry reads and writes under `HKCR\DirectShow\MediaObjects`.
//! - `DMOEnum` and the `IEnumDMO` methods: a COM object walking registry keys.

/// Success HRESULT.
pub const S_OK: i32 = 0;
/// `E_POINTER`, returned for null pointer arguments.
pub const E_POINTER: i32 = 0x8000_4003u32 as i32;
/// `ERROR_SUCCESS`, the registry status of a complete read.
pub const ERROR_SUCCESS: u32 = 0;
/// `ERROR_MORE_DATA`, the registry status of a read into a too-small buffer.
pub const ERROR_MORE_DATA: u32 = 234;
/// `sizeof(DMO_PARTIAL_MEDIATYPE)`: two GUIDs.
pub const PARTIAL_MEDIATYPE_SIZE: u32 = 32;
/// The registry value that lists the input types of a DMO.
pub const INPUT_TYPES: &str = "InputTypes";
/// The registry value that lists the output types of a DMO.
pub const OUTPUT_TYPES: &str = "OutputTypes";

/// A Windows `GUID`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `GUID_NULL`, the wildcard in a partial media type.
pub const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// A `DMO_PARTIAL_MEDIATYPE`: a major type and a subtype.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PartialMediaType {
    pub type_: Guid,
    pub subtype: Guid,
}

/// A `DMO_MEDIA_TYPE` without its `pUnk` member.
///
/// `format` is `pbFormat`: `None` for a null pointer, otherwise a buffer that
/// Wine sizes to `cb_format` bytes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MediaType {
    pub majortype: Guid,
    pub subtype: Guid,
    pub fixed_size_samples: i32,
    pub temporal_compression: i32,
    pub sample_size: u32,
    pub formattype: Guid,
    pub cb_format: u32,
    pub format: Option<Vec<u8>>,
}

/// Returns the capacity `array_reserve` grows to so that `count` elements of `size` bytes fit, or `None` where it returns `FALSE`.
///
/// A capacity that already holds `count` is kept. Otherwise it is at least 8,
/// doubled while below `count` (as long as doubling cannot overflow), then
/// raised to `count`. A `count` whose byte size overflows 32 bits is refused.
/// Allocation failure is not modelled.
///
/// # Panics
///
/// Panics when `size` is 0, where Wine divides by zero.
pub fn array_reserve(capacity: u32, count: u32, size: u32) -> Option<u32> {
    if count <= capacity {
        return Some(capacity);
    }
    let max_capacity = u32::MAX / size;
    if count > max_capacity {
        return None;
    }
    let mut new_capacity = capacity.max(8);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    Some(new_capacity.max(count))
}

/// Formats a GUID the way `GUIDToString` does: upper-case hex, no braces.
pub fn guid_to_string(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// `IsMediaTypeEqual`: the types and the subtypes each match, with `GUID_NULL` on either side matching anything.
pub fn is_media_type_equal(mt1: &PartialMediaType, mt2: &PartialMediaType) -> bool {
    let matches = |a: &Guid, b: &Guid| a == b || *b == GUID_NULL || *a == GUID_NULL;
    matches(&mt1.type_, &mt2.type_) && matches(&mt1.subtype, &mt2.subtype)
}

/// `any_types_match`: whether some type of `a` is [`is_media_type_equal`] to some type of `b`.
pub fn any_types_match(a: &[PartialMediaType], b: &[PartialMediaType]) -> bool {
    a.iter()
        .any(|x| b.iter().any(|y| is_media_type_equal(x, y)))
}

/// The type count `DMOGetTypes` returns for one registry value read.
///
/// `status` and `size` are what `RegQueryValueExW` returned. A complete read
/// or `ERROR_MORE_DATA` yields the smaller of `requested` and the number of
/// whole types in `size`; any other status yields 0.
pub fn returned_type_count(requested: u32, status: u32, size: u32) -> u32 {
    if status == ERROR_SUCCESS || status == ERROR_MORE_DATA {
        requested.min(size / PARTIAL_MEDIATYPE_SIZE)
    } else {
        0
    }
}

/// The counts `DMOGetTypes` returns once the DMO's registry key is open.
///
/// `query(value_name, buffer_size)` stands for `RegQueryValueExW`: it is called
/// with [`INPUT_TYPES`] then [`OUTPUT_TYPES`] and the buffer size in bytes (the
/// count times [`PARTIAL_MEDIATYPE_SIZE`], wrapping like Wine's 32-bit
/// multiply), and returns the status and the size it reports. The caller opens
/// the key named by [`guid_to_string`]; when that fails Wine returns `E_FAIL`
/// and never calls this.
pub fn dmo_get_types<F>(input_count: u32, output_count: u32, mut query: F) -> (u32, u32)
where
    F: FnMut(&str, u32) -> (u32, u32),
{
    let mut read = |name: &str, count: u32| {
        let (status, size) = query(name, count.wrapping_mul(PARTIAL_MEDIATYPE_SIZE));
        returned_type_count(count, status, size)
    };
    let inputs = read(INPUT_TYPES, input_count);
    let outputs = read(OUTPUT_TYPES, output_count);
    (inputs, outputs)
}

/// `MoInitMediaType`: clears `media` and, when `cb_format` is non-zero, gives it a format buffer of that size.
///
/// Wine leaves the new buffer uninitialized; here it is zeroed. Returns
/// `E_POINTER` for a null `media`. Allocation failure is not modelled.
pub fn mo_init_media_type(media: Option<&mut MediaType>, cb_format: u32) -> i32 {
    let Some(media) = media else {
        return E_POINTER;
    };
    *media = MediaType::default();
    if cb_format > 0 {
        media.format = Some(vec![0; cb_format as usize]);
        media.cb_format = cb_format;
    }
    S_OK
}

/// `MoFreeMediaType`: drops the format buffer and leaves `cb_format` as it was.
///
/// Returns `E_POINTER` for a null `media`.
pub fn mo_free_media_type(media: Option<&mut MediaType>) -> i32 {
    let Some(media) = media else {
        return E_POINTER;
    };
    media.format = None;
    S_OK
}

/// `MoCopyMediaType`: copies every field of `src` into `dst`, and the first `cb_format` bytes of the format buffer when there is one and `cb_format` is non-zero.
///
/// A format buffer `dst` already holds is replaced: Wine overwrites the pointer
/// without freeing it, and here the old buffer is dropped.
/// Returns `E_POINTER` when either argument is null.
///
/// # Panics
///
/// Panics when the format buffer of `src` is shorter than `cb_format`, where
/// Wine reads past it.
pub fn mo_copy_media_type(dst: Option<&mut MediaType>, src: Option<&MediaType>) -> i32 {
    let (Some(dst), Some(src)) = (dst, src) else {
        return E_POINTER;
    };
    dst.majortype = src.majortype;
    dst.subtype = src.subtype;
    dst.formattype = src.formattype;
    dst.fixed_size_samples = src.fixed_size_samples;
    dst.temporal_compression = src.temporal_compression;
    dst.sample_size = src.sample_size;
    dst.cb_format = src.cb_format;
    dst.format = match &src.format {
        Some(format) if src.cb_format > 0 => Some(format[..src.cb_format as usize].to_vec()),
        _ => None,
    };
    S_OK
}

#[cfg(test)]
mod oracle_checks;
