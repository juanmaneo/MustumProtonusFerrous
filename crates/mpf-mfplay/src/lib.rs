//! mfplay subset for MPF: media item position validation and conversion.
//!
//! Reference Wine's `dlls/mfplay` implements the Media Foundation playback API. Most of it works on
//! COM objects, media sessions, windows and critical sections. This crate ports the pure functions
//! that `player.c` applies to position values: `media_item_set_position`, `media_item_get_position`
//! and the start/stop ordering check in `media_item_SetStartStopPosition`. It works on GUIDs,
//! `PROPVARIANT`s and integers, with no COM objects, allocation, threads or handles.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `set_position`: validates the position type GUID, the XOR null check between format and
//!   position, the `VT_EMPTY`/`VT_I8` variant check, and rejects zero positions with
//!   `MF_E_OUT_OF_RANGE`.
//! - `get_position`: writes the `MFP_POSITIONTYPE_100NS` GUID and a `VT_I8` `PROPVARIANT` holding
//!   the value, unless the value is zero.
//! - `set_start_stop_position`: validates both positions and rejects `start > stop` with
//!   `MF_E_OUT_OF_RANGE`.
//!
//! Deviations from the C:
//! - `set_position` returns `Result<i64, Hresult>` where the C writes through an out-pointer and
//!   returns an `HRESULT`; the `Result` makes the out-pointer unobservable.
//! - `get_position` returns `(Guid, Option<PropVariant>)` where the C writes through out-pointers
//!   and returns early on a null format.
//! - `set_start_stop_position` returns `Result<(), Hresult>` where the C mutates an item struct
//!   and returns an `HRESULT`; the `Result` makes the mutation unobservable.
//!
//! Skipped (reason per item):
//! - Everything else in `player.c`: COM vtables, `IMFPMediaItem`/`IMFPMediaPlayer` methods,
//!   `MFStartup`/`MFShutdown`, `HWND` event windows, `CRITICAL_SECTION`, media sources, topologies,
//!   rate control and `DllMain`. They need COM objects, allocation, threads, handles or the media
//!   session.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `HRESULT`.
pub type Hresult = i32;

/// `E_INVALIDARG`.
pub const E_INVALIDARG: Hresult = 0x8007_0057u32 as i32;

/// `E_POINTER`.
pub const E_POINTER: Hresult = 0x8000_4003u32 as i32;

/// `MF_E_OUT_OF_RANGE`.
pub const MF_E_OUT_OF_RANGE: Hresult = 0xC00D_3702u32 as i32;

/// `MFP_POSITIONTYPE_100NS`: the all-zeros position type GUID.
pub const MFP_POSITIONTYPE_100NS: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// `VT_EMPTY`: a `PROPVARIANT` variant type meaning "no value".
pub const VT_EMPTY: u16 = 0;

/// `VT_I8`: a `PROPVARIANT` variant type meaning a signed 64-bit integer.
pub const VT_I8: u16 = 20;

/// A `PROPVARIANT` holding a `VT` tag and a signed 64-bit value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropVariant {
    pub vt: u16,
    pub h_val: i64,
}

/// `media_item_set_position`: validates `format`/`position` and converts a `VT_I8` position to a
/// `LARGE_INTEGER`.
///
/// Mirrors Wine's `media_item_set_position` in `dlls/mfplay/player.c`.
pub fn set_position(format: Option<&Guid>, position: Option<&PropVariant>) -> Result<i64, Hresult> {
    if let Some(f) = format {
        if *f != MFP_POSITIONTYPE_100NS {
            return Err(E_INVALIDARG);
        }
    }

    if format.is_some() ^ position.is_some() {
        return Err(E_POINTER);
    }

    if let Some(p) = position {
        if p.vt != VT_EMPTY && p.vt != VT_I8 {
            return Err(E_INVALIDARG);
        }
        if p.vt == VT_I8 {
            if p.h_val == 0 {
                return Err(MF_E_OUT_OF_RANGE);
            }
            return Ok(p.h_val);
        }
    }

    // Both arguments null, or a `VT_EMPTY` position: the C returns `S_OK` with `*ret` at zero.
    Ok(0)
}

/// `media_item_get_position`: writes the position type GUID and a `VT_I8` `PROPVARIANT` holding
/// `value`, unless `value` is zero.
///
/// Mirrors Wine's `media_item_get_position` in `dlls/mfplay/player.c`.
pub fn get_position(value: i64) -> (Guid, Option<PropVariant>) {
    let format = MFP_POSITIONTYPE_100NS;

    if value == 0 {
        return (format, None);
    }

    (
        format,
        Some(PropVariant {
            vt: VT_I8,
            h_val: value,
        }),
    )
}

/// `media_item_SetStartStopPosition`: validates both positions and rejects `start > stop`.
///
/// Mirrors Wine's `media_item_SetStartStopPosition` in `dlls/mfplay/player.c`.
pub fn set_start_stop_position(
    start_format: Option<&Guid>,
    start_position: Option<&PropVariant>,
    stop_format: Option<&Guid>,
    stop_position: Option<&PropVariant>,
) -> Result<(), Hresult> {
    let start = set_position(start_format, start_position)?;
    let stop = set_position(stop_format, stop_position)?;

    if start > stop {
        return Err(MF_E_OUT_OF_RANGE);
    }

    Ok(())
}
