//! Edge cases that the oracle rows do not reach: out-of-range and NaN colour channels, empty
//! animation keys, the size guards of the texture helpers, and the derived trait impls.

use crate::animation::{index_max, index_min, insert_position, lookup_key, range, Key};
use crate::math::{color_rgb, color_rgba};
use crate::texture::{load_indexed, load_rgb24, palettise, ImageError, Palettised, Rgb24Image};
use crate::{Quaternion, Vector};

#[test]
fn colour_channels_clamp_and_nan_is_zero() {
    assert_eq!(color_rgb(0.0, 0.5, 1.0), 0xff00_7fff);
    assert_eq!(color_rgba(f32::NAN, 0.0, 0.0, 0.0), 0);
    assert_eq!(color_rgba(-1.0, 2.0, 0.0, -0.0), 0x0000_ff00);
}

#[test]
fn empty_keys_give_zero_and_no_range() {
    let keys: &[Key<()>] = &[];
    assert_eq!(lookup_key(keys, 1.0), 0);
    assert_eq!(index_min(keys, 1.0), 0);
    assert_eq!(index_max(keys, 1.0), 0);
    assert_eq!(insert_position(keys, 1.0), 0);
    assert_eq!(range(keys, 0.0, 1.0), None);
}

#[test]
fn palettise_refuses_overflowing_sizes() {
    assert_eq!(palettise(&[0; 8], 2, 2, false), None);
    assert_eq!(palettise(&[], usize::MAX, 0, false), None);
    assert_eq!(palettise(&[], 1 << 63, 2, false), None);
    assert_eq!(palettise(&[], usize::MAX / 3 + 1, 1, false), None);
}

#[test]
fn palettise_zero_sizes_give_empty_images() {
    assert_eq!(
        palettise(&[], 0, 3, false),
        Some(Palettised {
            pitch: 0,
            indices: Vec::new(),
            palette: Vec::new(),
        })
    );
    assert_eq!(
        palettise(&[], 3, 0, false),
        Some(Palettised {
            pitch: 4,
            indices: Vec::new(),
            palette: Vec::new(),
        })
    );
}

#[test]
fn more_than_256_colours_is_not_palettised() {
    let src: Vec<u8> = (0..257)
        .flat_map(|i: usize| [i as u8, (i >> 8) as u8, 0])
        .collect();
    assert_eq!(palettise(&src, 257, 1, false), None);
    assert_eq!(
        palettise(&src[..768], 256, 1, false).map(|image| image.palette.len()),
        Some(256)
    );
    let pixels: Vec<u8> = src
        .chunks(3)
        .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 0xff])
        .collect();
    assert_eq!(
        load_rgb24(&src, 257, 1, false),
        Ok(Rgb24Image::Bgrx {
            pitch: 1028,
            pixels,
        })
    );
}

#[test]
fn load_of_a_small_image_palettises() {
    assert_eq!(
        load_rgb24(&[1, 2, 3], 1, 1, false),
        Ok(Rgb24Image::Palette(Palettised {
            pitch: 4,
            indices: vec![0, 0xff, 0xff, 0xff],
            palette: vec![[3, 2, 1]],
        }))
    );
}

#[test]
fn load_rejects_overflowing_and_short_sources() {
    assert_eq!(
        load_rgb24(&[], usize::MAX, 0, false),
        Err(ImageError::TooLarge)
    );
    assert_eq!(
        load_rgb24(&[], 1 << 61, 2, false),
        Err(ImageError::TooLarge)
    );
    assert_eq!(
        load_rgb24(&[0; 2], 1, 1, false),
        Err(ImageError::ShortSource)
    );
    assert_eq!(
        load_indexed(&[], 1 << 63, 2, false),
        Err(ImageError::TooLarge)
    );
    assert_eq!(
        load_indexed(&[0; 3], 2, 2, false),
        Err(ImageError::ShortSource)
    );
    assert_eq!(load_indexed(&[], 0, 3, false), Ok(Vec::new()));
}

#[test]
fn derived_traits_on_vectors() {
    let v = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(Clone::clone(&v), v);
    assert_eq!(format!("{v:?}"), "Vector { x: 1.0, y: 2.0, z: 3.0 }");
    let q = Quaternion { s: 0.5, v };
    assert_eq!(Clone::clone(&q), q);
    assert_eq!(
        format!("{q:?}"),
        "Quaternion { s: 0.5, v: Vector { x: 1.0, y: 2.0, z: 3.0 } }"
    );
}

#[test]
fn derived_traits_on_keys() {
    let unit = Key {
        time: 0.5,
        value: (),
    };
    assert_eq!(Clone::clone(&unit), unit);
    assert_eq!(format!("{unit:?}"), "Key { time: 0.5, value: () }");
    let int_key = Key {
        time: 1.0,
        value: 7,
    };
    assert_eq!(Clone::clone(&int_key), int_key);
    assert_eq!(format!("{int_key:?}"), "Key { time: 1.0, value: 7 }");
}

#[test]
fn derived_traits_on_images() {
    let palette = Rgb24Image::Palette(Palettised {
        pitch: 0,
        indices: Vec::new(),
        palette: Vec::new(),
    });
    let bgrx = Rgb24Image::Bgrx {
        pitch: 4,
        pixels: vec![1, 2, 3, 255],
    };
    assert_eq!(
        format!("{palette:?}"),
        "Palette(Palettised { pitch: 0, indices: [], palette: [] })"
    );
    assert_eq!(
        format!("{bgrx:?}"),
        "Bgrx { pitch: 4, pixels: [1, 2, 3, 255] }"
    );
    assert_ne!(palette, bgrx);
    assert_eq!(
        bgrx,
        Rgb24Image::Bgrx {
            pitch: 4,
            pixels: vec![1, 2, 3, 255],
        }
    );
    assert_eq!(format!("{:?}", ImageError::TooLarge), "TooLarge");
    assert_eq!(format!("{:?}", ImageError::ShortSource), "ShortSource");
    assert_ne!(ImageError::TooLarge, ImageError::ShortSource);
}
