//! Pure printer DEVMODEW validation logic from Wine's `winspool.drv`.
//!
//! The port covers only `IsValidDevmodeW`: it checks the caller-visible byte
//! count, the fixed header, the declared DEVMODE plus driver-extra length, and
//! the size of each field selected by `dmFields`. Printer handles, encoding
//! conversion, and the ANSI FIXME stub are outside this crate's scope.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

const DM_FIELDS_END: usize = 76;
const DM_SIZE_OFFSET: usize = 68;
const DM_DRIVER_EXTRA_OFFSET: usize = 70;
const DM_FIELDS_OFFSET: usize = 72;

/// `(DM_* flag, byte offset past the field)` for every field `IsValidDevmodeW` checks.
const FIELD_SIZES: [(u32, usize); 30] = [
    (0x0000_0001, 78),  // DM_ORIENTATION
    (0x0000_0002, 80),  // DM_PAPERSIZE
    (0x0000_0004, 82),  // DM_PAPERLENGTH
    (0x0000_0008, 84),  // DM_PAPERWIDTH
    (0x0000_0010, 86),  // DM_SCALE
    (0x0000_0100, 88),  // DM_COPIES
    (0x0000_0200, 90),  // DM_DEFAULTSOURCE
    (0x0000_0400, 92),  // DM_PRINTQUALITY
    (0x0000_0020, 84),  // DM_POSITION
    (0x0000_0080, 88),  // DM_DISPLAYORIENTATION
    (0x2000_0000, 92),  // DM_DISPLAYFIXEDOUTPUT
    (0x0000_0800, 94),  // DM_COLOR
    (0x0000_1000, 96),  // DM_DUPLEX
    (0x0000_2000, 98),  // DM_YRESOLUTION
    (0x0000_4000, 100), // DM_TTOPTION
    (0x0000_8000, 102), // DM_COLLATE
    (0x0001_0000, 166), // DM_FORMNAME
    (0x0002_0000, 168), // DM_LOGPIXELS
    (0x0004_0000, 172), // DM_BITSPERPEL
    (0x0008_0000, 176), // DM_PELSWIDTH
    (0x0010_0000, 180), // DM_PELSHEIGHT
    (0x0020_0000, 184), // DM_DISPLAYFLAGS
    (0x0000_0040, 184), // DM_NUP
    (0x0040_0000, 188), // DM_DISPLAYFREQUENCY
    (0x0080_0000, 192), // DM_ICMMETHOD
    (0x0100_0000, 196), // DM_ICMINTENT
    (0x0200_0000, 200), // DM_MEDIATYPE
    (0x0400_0000, 204), // DM_DITHERTYPE
    (0x0800_0000, 216), // DM_PANNINGWIDTH
    (0x1000_0000, 220), // DM_PANNINGHEIGHT
];

/// Returns whether a DEVMODEW byte buffer satisfies Wine's `IsValidDevmodeW` checks.
///
/// `size` is the caller-supplied accessible byte count, equivalent to the
/// Win32 function's `SIZE_T` argument. The slice must contain the fixed header
/// even if that count claims a larger region. The DEVMODE words and `dmFields`
/// are read in Windows little-endian byte order. A missing pointer or any
/// failed size/field check returns `false`; this function never repairs or
/// mutates the buffer.
pub fn is_valid_devmode_w(buffer: Option<&[u8]>, size: usize) -> bool {
    let Some(dm) = buffer else {
        return false;
    };
    if size < DM_FIELDS_END || dm.len() < DM_FIELDS_END {
        return false;
    }

    let dm_size = u16::from_le_bytes([dm[DM_SIZE_OFFSET], dm[DM_SIZE_OFFSET + 1]]) as usize;
    let driver_extra =
        u16::from_le_bytes([dm[DM_DRIVER_EXTRA_OFFSET], dm[DM_DRIVER_EXTRA_OFFSET + 1]]) as usize;
    if dm_size < DM_FIELDS_END || size < dm_size + driver_extra {
        return false;
    }

    let fields = u32::from_le_bytes([
        dm[DM_FIELDS_OFFSET],
        dm[DM_FIELDS_OFFSET + 1],
        dm[DM_FIELDS_OFFSET + 2],
        dm[DM_FIELDS_OFFSET + 3],
    ]);
    FIELD_SIZES
        .iter()
        .all(|&(flag, required_size)| fields & flag == 0 || dm_size >= required_size)
}
