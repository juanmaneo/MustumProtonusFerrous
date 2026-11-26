//! Edge-case tests for the ported DDSD conversions.
//!
//! These cover each flag guarding its field on its own, the flags that
//! `ddsd2_to_ddsd` clears, and the `dwSize` values. Tests marked "Not oracle-derived"
//! take their expectations from reading `ddraw.c`, not from the harness output.

use super::*;

const SURFACE: usize = 0x7700_8800;

/// A descriptor with every modeled field set to a distinct nonzero value.
fn full() -> SurfaceDesc {
    SurfaceDesc {
        size: DDSURFACEDESC2_SIZE,
        flags: 0,
        height: 0x11,
        width: 0x22,
        pitch_or_linear_size: 0x33,
        back_buffer_count: 0x44,
        mip_map_count: 0x55,
        alpha_bit_depth: 0x66,
        surface: SURFACE,
        ck_dest_overlay: ColorKey {
            low: 0x81,
            high: 0x82,
        },
        ck_dest_blt: ColorKey {
            low: 0x83,
            high: 0x84,
        },
        ck_src_overlay: ColorKey {
            low: 0x85,
            high: 0x86,
        },
        ck_src_blt: ColorKey {
            low: 0x87,
            high: 0x88,
        },
        pixel_format: [0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98],
        caps: 0xa1,
    }
}

/// Runs `convert` on `full()` with only `flag` set, and compares against a result
/// that has `size`, `flags` and `lpSurface` plus the one field `set` writes.
fn check_single_flag(
    flag: u32,
    convert: fn(&SurfaceDesc) -> SurfaceDesc,
    size: u32,
    set: fn(&mut SurfaceDesc),
) {
    let mut input = full();
    input.flags = flag;
    let mut expected = SurfaceDesc {
        size,
        flags: flag,
        surface: SURFACE,
        ..SurfaceDesc::default()
    };
    set(&mut expected);
    assert_eq!(convert(&input), expected);
}

#[test]
fn zero_flags_copy_only_size_flags_and_surface() {
    // Not oracle-derived: C memsets the output, then copies `lpSurface` unconditionally.
    let input = full();
    assert_eq!(
        ddsd_to_ddsd2(&input),
        SurfaceDesc {
            size: DDSURFACEDESC2_SIZE,
            surface: SURFACE,
            ..SurfaceDesc::default()
        }
    );
    assert_eq!(
        ddsd2_to_ddsd(&input),
        SurfaceDesc {
            size: DDSURFACEDESC_SIZE,
            surface: SURFACE,
            ..SurfaceDesc::default()
        }
    );
}

#[test]
fn all_flags_copy_every_field() {
    // Not oracle-derived: the output is `full()` with the flag word and size changed.
    let mut input = full();
    input.flags = DDSD_ALL;
    // DDSD_ALL (0x00fff9ee) has no DDSD_CAPS bit, so `caps` is not copied.
    let mut expected = full();
    expected.flags = DDSD_ALL;
    expected.caps = 0;
    assert_eq!(ddsd_to_ddsd2(&input), expected);
    assert_eq!(
        ddsd2_to_ddsd(&input),
        SurfaceDesc {
            size: DDSURFACEDESC_SIZE,
            flags: DDSD_ALL & !(DDSD_TEXTURESTAGE | DDSD_FVF | DDSD_SRCVBHANDLE),
            ..expected
        }
    );
}

#[test]
fn single_flags_to_ddsd2_copy_only_their_field() {
    // Not oracle-derived: each expectation is the field guarded by that flag in `ddraw.c`.
    let size = DDSURFACEDESC2_SIZE;
    check_single_flag(DDSD_CAPS, ddsd_to_ddsd2, size, |d| d.caps = 0xa1);
    check_single_flag(DDSD_HEIGHT, ddsd_to_ddsd2, size, |d| d.height = 0x11);
    check_single_flag(DDSD_WIDTH, ddsd_to_ddsd2, size, |d| d.width = 0x22);
    check_single_flag(DDSD_PITCH, ddsd_to_ddsd2, size, |d| {
        d.pitch_or_linear_size = 0x33
    });
    check_single_flag(DDSD_BACKBUFFERCOUNT, ddsd_to_ddsd2, size, |d| {
        d.back_buffer_count = 0x44
    });
    check_single_flag(DDSD_ZBUFFERBITDEPTH, ddsd_to_ddsd2, size, |d| {
        d.mip_map_count = 0x55
    });
    check_single_flag(DDSD_ALPHABITDEPTH, ddsd_to_ddsd2, size, |d| {
        d.alpha_bit_depth = 0x66
    });
    check_single_flag(DDSD_PIXELFORMAT, ddsd_to_ddsd2, size, |d| {
        d.pixel_format = full().pixel_format
    });
    check_single_flag(DDSD_CKDESTOVERLAY, ddsd_to_ddsd2, size, |d| {
        d.ck_dest_overlay = full().ck_dest_overlay
    });
    check_single_flag(DDSD_CKDESTBLT, ddsd_to_ddsd2, size, |d| {
        d.ck_dest_blt = full().ck_dest_blt
    });
    check_single_flag(DDSD_CKSRCOVERLAY, ddsd_to_ddsd2, size, |d| {
        d.ck_src_overlay = full().ck_src_overlay
    });
    check_single_flag(DDSD_CKSRCBLT, ddsd_to_ddsd2, size, |d| {
        d.ck_src_blt = full().ck_src_blt
    });
    check_single_flag(DDSD_MIPMAPCOUNT, ddsd_to_ddsd2, size, |d| {
        d.mip_map_count = 0x55
    });
    check_single_flag(DDSD_REFRESHRATE, ddsd_to_ddsd2, size, |d| {
        d.mip_map_count = 0x55
    });
    check_single_flag(DDSD_LINEARSIZE, ddsd_to_ddsd2, size, |d| {
        d.pitch_or_linear_size = 0x33
    });
    // Flags that only set bits in `flags`: nothing else is copied.
    for flag in [
        DDSD_LPSURFACE,
        DDSD_TEXTURESTAGE,
        DDSD_FVF,
        DDSD_SRCVBHANDLE,
        DDSD_DEPTH,
    ] {
        check_single_flag(flag, ddsd_to_ddsd2, size, |_| {});
    }
}

#[test]
fn single_flags_to_ddsd_copy_only_their_field() {
    // Not oracle-derived: same as above, with the 108-byte size and the flag
    // mask applied. The C union means ZBUFFERBITDEPTH writes the mip-map slot.
    let size = DDSURFACEDESC_SIZE;
    check_single_flag(DDSD_CAPS, ddsd2_to_ddsd, size, |d| d.caps = 0xa1);
    check_single_flag(DDSD_ZBUFFERBITDEPTH, ddsd2_to_ddsd, size, |d| {
        d.mip_map_count = 0x55
    });
    check_single_flag(DDSD_REFRESHRATE, ddsd2_to_ddsd, size, |d| {
        d.mip_map_count = 0x55
    });
    check_single_flag(DDSD_LINEARSIZE, ddsd2_to_ddsd, size, |d| {
        d.pitch_or_linear_size = 0x33
    });
}

#[test]
fn ddsd2_to_ddsd_clears_only_the_three_flags_it_lacks() {
    // Not oracle-derived: `DDSD_DEPTH` is not in the strip list in `ddraw.c`, so it stays.
    let mut input = full();
    input.flags = DDSD_TEXTURESTAGE | DDSD_FVF | DDSD_SRCVBHANDLE | DDSD_CAPS | DDSD_DEPTH;
    assert_eq!(ddsd_to_ddsd2(&input).flags, input.flags);
    assert_eq!(ddsd2_to_ddsd(&input).flags, DDSD_CAPS | DDSD_DEPTH);
}

#[test]
fn size_is_fixed_by_direction_not_by_input() {
    // Not oracle-derived: C overwrites `dwSize` from `sizeof(*out)`, ignoring the input value.
    let mut input = full();
    input.size = 0;
    assert_eq!(ddsd_to_ddsd2(&input).size, 124);
    assert_eq!(ddsd2_to_ddsd(&input).size, 108);
    input.size = 0xdead_beef;
    assert_eq!(ddsd_to_ddsd2(&input).size, 124);
    assert_eq!(ddsd2_to_ddsd(&input).size, 108);
}

#[test]
fn conversions_round_trip_modeled_fields() {
    // Not oracle-derived: converting down and back up keeps the fields whose flags
    // are set, and `dwSize` is reset to the DDSURFACEDESC2 size.
    let mut input = full();
    input.flags = DDSD_WIDTH | DDSD_HEIGHT | DDSD_CAPS | DDSD_PIXELFORMAT | DDSD_LPSURFACE;
    let down = ddsd2_to_ddsd(&input);
    let up = ddsd_to_ddsd2(&down);
    assert_eq!(up.size, DDSURFACEDESC2_SIZE);
    assert_eq!(up.width, input.width);
    assert_eq!(up.height, input.height);
    assert_eq!(up.caps, input.caps);
    assert_eq!(up.pixel_format, input.pixel_format);
    assert_eq!(up.surface, input.surface);
}
