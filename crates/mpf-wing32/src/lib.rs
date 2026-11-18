//! Headless subset of Wine's `dlls/wing32/wing32.c` WinG API.
//!
//! Ported:
//! - `WinGRecommendDIBFormat`: `recommend_dib_format`; writes the 320x1, 8-bpp, `BI_RGB` header and fails on a null pointer.
//!
//! Skipped (each one is a GDI call on live device, bitmap, or palette state, with no logic of its own):
//! - `WinGCreateDC`: `CreateCompatibleDC(0)`.
//! - `WinGCreateBitmap`: `CreateDIBSection`.
//! - `WinGGetDIBPointer`: `GetObjectW` on a live bitmap handle.
//! - `WinGSetDIBColorTable` and `WinGGetDIBColorTable`: `SetDIBColorTable` and `GetDIBColorTable` on a device context.
//! - `WinGCreateHalftonePalette`: `GetDC(0)` and `CreateHalftonePalette`.
//! - `WinGCreateHalftoneBrush`: `CreateSolidBrush(color)`; the device context and brush type are ignored.
//! - `WinGStretchBlt`: sets `COLORONCOLOR`, calls `StretchBlt`, then restores the mode of the device context.
//! - `WinGBitBlt`: `BitBlt` with `SRCCOPY`.

/// `BI_RGB`, the uncompressed `biCompression` value.
pub const BI_RGB: u32 = 0;
/// `sizeof(BITMAPINFOHEADER)`, the `biSize` that `WinGRecommendDIBFormat` writes.
pub const BITMAPINFOHEADER_SIZE: u32 = 40;

/// The `BITMAPINFOHEADER` fields, with the Win32 field widths.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: i32,
    pub y_pels_per_meter: i32,
    pub clr_used: u32,
    pub clr_important: u32,
}

/// Fills `header` with the 320x1, 8-bpp, uncompressed format that WinG recommends and returns `true`.
///
/// Returns `false` and leaves nothing behind when `header` is `None`. Prior field values are overwritten, never read.
/// Only `bmiHeader` is written; the color table that follows it in `BITMAPINFO` is untouched.
pub fn recommend_dib_format(header: Option<&mut BitmapInfoHeader>) -> bool {
    let Some(header) = header else {
        return false;
    };
    *header = BitmapInfoHeader {
        size: BITMAPINFOHEADER_SIZE,
        width: 320,
        height: 1,
        planes: 1,
        bit_count: 8,
        compression: BI_RGB,
        size_image: 0,
        x_pels_per_meter: 0,
        y_pels_per_meter: 0,
        clr_used: 0,
        clr_important: 0,
    };
    true
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size_matches_win32_layout() {
        assert_eq!(std::mem::size_of::<BitmapInfoHeader>(), 40);
        assert_eq!(
            BITMAPINFOHEADER_SIZE as usize,
            std::mem::size_of::<BitmapInfoHeader>()
        );
    }

    #[test]
    fn null_header_fails() {
        assert!(!recommend_dib_format(None));
    }

    #[test]
    fn recommended_header_is_320_by_1_at_8_bpp() {
        let mut header = BitmapInfoHeader {
            size: 0,
            width: -7,
            height: 99,
            planes: 3,
            bit_count: 24,
            compression: 3,
            size_image: 5,
            x_pels_per_meter: 6,
            y_pels_per_meter: 7,
            clr_used: 8,
            clr_important: 9,
        };
        assert!(recommend_dib_format(Some(&mut header)));
        assert_eq!(header.width, 320);
        assert_eq!(header.height, 1);
        assert_eq!(header.bit_count, 8);
        assert_eq!(header.compression, BI_RGB);
        assert_eq!(header.size, 40);
    }
}
