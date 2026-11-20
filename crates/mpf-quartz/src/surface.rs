//! `adjust_surface_size` in `dlls/quartz/vmr9.c`: rounds a texture surface up to powers of two, or
//! makes it square, as the device caps require.

/// `VMR9AllocFlag_TextureSurface`.
pub const VMR9_ALLOC_FLAG_TEXTURE_SURFACE: u32 = 0x4;
/// `D3DPTEXTURECAPS_POW2`.
pub const D3DPTEXTURECAPS_POW2: u32 = 0x2;
/// `D3DPTEXTURECAPS_SQUAREONLY`.
pub const D3DPTEXTURECAPS_SQUAREONLY: u32 = 0x20;

/// The fields of `VMR9AllocationInfo` that [adjust_surface_size] reads and writes.
pub struct AllocationInfo {
    pub flags: u32,
    pub width: u32,
    pub height: u32,
}

/// `adjust_surface_size`. Offscreen surfaces keep their size. `None` means the power of two above
/// a dimension does not fit in 32 bits.
pub fn adjust_surface_size(texture_caps: u32, info: AllocationInfo) -> Option<AllocationInfo> {
    if (info.flags & VMR9_ALLOC_FLAG_TEXTURE_SURFACE) == 0 {
        return Some(info);
    }

    let mut width = info.width;
    let mut height = info.height;
    if (texture_caps & D3DPTEXTURECAPS_POW2) != 0
        && (texture_caps & D3DPTEXTURECAPS_SQUAREONLY) == 0
    {
        width = width.checked_next_power_of_two()?;
        height = height.checked_next_power_of_two()?;
    }
    if (texture_caps & D3DPTEXTURECAPS_SQUAREONLY) != 0 {
        if height > width {
            width = height;
        } else {
            height = width;
        }
    }

    Some(AllocationInfo {
        flags: info.flags,
        width,
        height,
    })
}
