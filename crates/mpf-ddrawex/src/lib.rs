//! Port of the portable logic in Wine's `dlls/ddrawex/ddraw.c`.
//!
//! Ported:
//! - `DDSD_to_DDSD2` (`DDSURFACEDESC` to `DDSURFACEDESC2`): [`ddsd_to_ddsd2`]
//! - `DDSD2_to_DDSD` (`DDSURFACEDESC2` to `DDSURFACEDESC`, clearing
//!   `DDSD_TEXTURESTAGE`, `DDSD_FVF` and `DDSD_SRCVBHANDLE`): [`ddsd2_to_ddsd`]
//!
//! Skipped:
//! - COM vtables in `ddraw.c` and `surface.c` (`QueryInterface`, `AddRef`, `Release`,
//!   `CreateSurface`, `GetCaps`, `GetDisplayMode`, ...): they forward to the inner ddraw.dll object.
//! - `enum_modes_cb2`, `enum_surfaces_wrapper`, `enum_surfaces_cb2`, `enumsurfaces_wrap_cb`:
//!   callback adapters that hand COM surface pointers to application callbacks.
//! - `dd_get_inner`, `dds_get_outer`, `prepare_permanent_dc`: COM pointer identity,
//!   private data and GDI device contexts.
//! - `ddrawex_factory_create` and the class factory in `main.c`: COM object creation.
//! - `TRACE`/`WARN` output in `DDSD2_to_DDSD`: it does not change the returned descriptor.

/// `DDSURFACEDESC` size on the 32-bit ABI, written to `dwSize` by [`ddsd2_to_ddsd`].
pub const DDSURFACEDESC_SIZE: u32 = 108;
/// `DDSURFACEDESC2` size on the 32-bit ABI, written to `dwSize` by [`ddsd_to_ddsd2`].
pub const DDSURFACEDESC2_SIZE: u32 = 124;

/// `DDSURFACEDESC.dwFlags` bits from `ddraw.h`.
pub const DDSD_CAPS: u32 = 0x0000_0001;
pub const DDSD_HEIGHT: u32 = 0x0000_0002;
pub const DDSD_WIDTH: u32 = 0x0000_0004;
pub const DDSD_PITCH: u32 = 0x0000_0008;
pub const DDSD_BACKBUFFERCOUNT: u32 = 0x0000_0020;
pub const DDSD_ZBUFFERBITDEPTH: u32 = 0x0000_0040;
pub const DDSD_ALPHABITDEPTH: u32 = 0x0000_0080;
pub const DDSD_LPSURFACE: u32 = 0x0000_0800;
pub const DDSD_PIXELFORMAT: u32 = 0x0000_1000;
pub const DDSD_CKDESTOVERLAY: u32 = 0x0000_2000;
pub const DDSD_CKDESTBLT: u32 = 0x0000_4000;
pub const DDSD_CKSRCOVERLAY: u32 = 0x0000_8000;
pub const DDSD_CKSRCBLT: u32 = 0x0001_0000;
pub const DDSD_MIPMAPCOUNT: u32 = 0x0002_0000;
pub const DDSD_REFRESHRATE: u32 = 0x0004_0000;
pub const DDSD_LINEARSIZE: u32 = 0x0008_0000;
pub const DDSD_TEXTURESTAGE: u32 = 0x0010_0000;
pub const DDSD_FVF: u32 = 0x0020_0000;
pub const DDSD_SRCVBHANDLE: u32 = 0x0040_0000;
pub const DDSD_DEPTH: u32 = 0x0080_0000;
pub const DDSD_ALL: u32 = 0x00ff_f9ee;

/// `DDCOLORKEY`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ColorKey {
    pub low: u32,
    pub high: u32,
}

/// The `DDSURFACEDESC` / `DDSURFACEDESC2` fields that the two conversions read or write.
///
/// Both C structs keep these fields at the same offsets, so one type serves both
/// directions. The C unions that share storage are one field here, because the
/// conversions depend on that aliasing. Fields the conversions never touch are not
/// modeled; the C code zeroes them in the output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceDesc {
    /// `dwSize`.
    pub size: u32,
    /// `dwFlags`: which of the other fields are valid.
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    /// `lPitch` and `dwLinearSize`, one slot read and written as raw bits.
    pub pitch_or_linear_size: u32,
    pub back_buffer_count: u32,
    /// `dwMipMapCount`, `dwZBufferBitDepth` and `dwRefreshRate`, one slot.
    pub mip_map_count: u32,
    pub alpha_bit_depth: u32,
    /// `lpSurface`, an opaque pointer value copied verbatim.
    pub surface: usize,
    pub ck_dest_overlay: ColorKey,
    pub ck_dest_blt: ColorKey,
    pub ck_src_overlay: ColorKey,
    pub ck_src_blt: ColorKey,
    /// `ddpfPixelFormat` as its eight DWORDs, copied as one value.
    pub pixel_format: [u32; 8],
    /// `ddsCaps.dwCaps`.
    pub caps: u32,
}

/// `DDSD_to_DDSD2`: converts a `DDSURFACEDESC` to a `DDSURFACEDESC2`. `flags` is kept
/// as it is, so `DDSD_TEXTURESTAGE`, `DDSD_FVF` and `DDSD_SRCVBHANDLE` survive.
pub fn ddsd_to_ddsd2(input: &SurfaceDesc) -> SurfaceDesc {
    copy_fields(input, DDSURFACEDESC2_SIZE)
}

/// `DDSD2_to_DDSD`: converts a `DDSURFACEDESC2` to a `DDSURFACEDESC`. The three flags
/// that `DDSURFACEDESC` lacks are cleared from `flags` after the copy.
pub fn ddsd2_to_ddsd(input: &SurfaceDesc) -> SurfaceDesc {
    let mut out = copy_fields(input, DDSURFACEDESC_SIZE);
    out.flags &= !(DDSD_TEXTURESTAGE | DDSD_FVF | DDSD_SRCVBHANDLE);
    out
}

/// Shared body of both conversions: the C code zeroes the output, then copies each
/// field whose flag is set in the input. `lpSurface` is copied unconditionally.
fn copy_fields(input: &SurfaceDesc, size: u32) -> SurfaceDesc {
    let flags = input.flags;
    let mut out = SurfaceDesc {
        size,
        flags,
        surface: input.surface,
        ..SurfaceDesc::default()
    };
    if flags & DDSD_WIDTH != 0 {
        out.width = input.width;
    }
    if flags & DDSD_HEIGHT != 0 {
        out.height = input.height;
    }
    if flags & DDSD_PIXELFORMAT != 0 {
        out.pixel_format = input.pixel_format;
    }
    if flags & DDSD_CAPS != 0 {
        out.caps = input.caps;
    }
    if flags & DDSD_PITCH != 0 {
        out.pitch_or_linear_size = input.pitch_or_linear_size;
    }
    if flags & DDSD_BACKBUFFERCOUNT != 0 {
        out.back_buffer_count = input.back_buffer_count;
    }
    // `DDSD_ZBUFFERBITDEPTH` and `DDSD_REFRESHRATE` use the same slot as `DDSD_MIPMAPCOUNT`.
    if flags & DDSD_ZBUFFERBITDEPTH != 0 {
        out.mip_map_count = input.mip_map_count;
    }
    if flags & DDSD_ALPHABITDEPTH != 0 {
        out.alpha_bit_depth = input.alpha_bit_depth;
    }
    if flags & DDSD_CKDESTOVERLAY != 0 {
        out.ck_dest_overlay = input.ck_dest_overlay;
    }
    if flags & DDSD_CKDESTBLT != 0 {
        out.ck_dest_blt = input.ck_dest_blt;
    }
    if flags & DDSD_CKSRCOVERLAY != 0 {
        out.ck_src_overlay = input.ck_src_overlay;
    }
    if flags & DDSD_CKSRCBLT != 0 {
        out.ck_src_blt = input.ck_src_blt;
    }
    if flags & DDSD_MIPMAPCOUNT != 0 {
        out.mip_map_count = input.mip_map_count;
    }
    if flags & DDSD_REFRESHRATE != 0 {
        out.mip_map_count = input.mip_map_count;
    }
    if flags & DDSD_LINEARSIZE != 0 {
        out.pitch_or_linear_size = input.pitch_or_linear_size;
    }
    out
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
