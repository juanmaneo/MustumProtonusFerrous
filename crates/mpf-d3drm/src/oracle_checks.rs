//! Checks the port against rows recorded from Wine's verbatim `dlls/d3drm` C, built with ASan and
//! UBSan, with no reports. Each row is `(commands, replies)`: the `|`-separated commands run in
//! order, each on its own input, and the `|`-separated replies must match exactly.
//!
//! The commands use the oracle's input format. Floats are the hex of their IEEE bits, and the
//! byte arguments are hex, `-` for empty, or `g<len>` for the generated pattern.

use crate::animation::{index_max, index_min, insert_position, lookup_key, range, Key};
use crate::math::{
    color_alpha, color_blue, color_green, color_red, color_rgba, matrix_from_quaternion,
    quaternion_from_rotation, quaternion_multiply, quaternion_slerp, vector_add, vector_cross,
    vector_dot, vector_modulus, vector_normalize, vector_reflect, vector_rotate, vector_scale,
    vector_subtract,
};
use crate::texture::{load_indexed, load_rgb24, palettise, Palettised, Rgb24Image};
use crate::transform::{matrix_multiply_affine, matrix_set_rotation, vector_transform_affine};
use crate::{Matrix, Quaternion, Vector};

#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    // A: colour constructor: in range, clamped, 1 - 2^-24, and zero channels
    ("rgba 3f800000 00000000 3f000000 3e000000|rgba 40000000 bf800000 3d4ccccd 3e4ccccd|rgba 3f7fffff 00000000 00000000 00000000|rgba 00000000 3f000000 3e800000 3e800000", "1fff007f|33ff000c|00fe0000|3f007f3f"),
    // B: colour getters
    ("alpha ff000000|red 00ff0000|green 0000ff00|blue 000000ff|alpha 7f000000|red 00800000", "3f800000|3f800000|3f800000|3f800000|3efefeff|3f008081"),
    // C: quaternion multiply
    ("qmul 3f000000 3f800000 c0000000 3e800000 3fc00000 bf400000 3f000000 40400000|qmul 3f3504f4 3f3504f4 00000000 00000000 3f000000 3f000000 3f000000 3f000000", "3fe00000 c0a00000 c0be0000 3f600000|00000000 3f3504f4 00000000 3f3504f4"),
    // D: rotation matrix from a quaternion
    ("qmatrix 3f800000 00000000 00000000 00000000|qmatrix 3f19999a 3ef5c28f beb851ec 3f23d70a", "3f800000 00000000 00000000 00000000 00000000 3f800000 00000000 00000000 00000000 00000000 3f800000 00000000 00000000 00000000 00000000 3f800000|bda09030 bf8e8a72 3e3ac70c 00000000 3ed844d0 be8f5c28 bf84b5dd 00000000 3f85f070 3debedfc 3e8f5c28 00000000 00000000 00000000 00000000 3f800000"),
    // E: quaternion from a rotation
    ("qrot 00000000 00000000 3f800000 40490fdb|qrot 40000000 40400000 bf800000 3f000000", "b33bbd2e 00000000 00000000 3f800000 00000000 00000000 3f800000|3f780aa5 3e076ab4 3e4b200e bd876ab4 3f08d677 3f4d41b2 be88d677"),
    // F: slerp, spherical weights
    ("slerp 3f800000 00000000 00000000 00000000 00000000 3f800000 00000000 00000000 3f000000|slerp 3f19999a 3f4ccccd 00000000 00000000 bf19999a 00000000 3f4ccccd 00000000 3f000000", "3f3504f3 3f3504f3 00000000 00000000|3f3a4473 3ef85b43 bef85b43 00000000"),
    // G: slerp, linear weights and the sign flip
    ("slerp 3f800000 00000000 00000000 00000000 3f7ffff8 00000000 00000000 3a83126f 3e800000|slerp 3f800000 00000000 00000000 00000000 bf800000 00000000 00000000 00000000 3e800000", "3f7ffffe 00000000 00000000 3983126f|3f800000 00000000 00000000 00000000"),
    // H: vector add, subtract, cross and dot
    ("vadd 3f800000 40000000 40400000 40800000 40a00000 40c00000|vsub 3f800000 40000000 40400000 40800000 40a00000 40c00000|vcross 3f800000 40000000 40400000 40800000 40a00000 40c00000|vdot 3f800000 40000000 40400000 40800000 40a00000 40c00000", "40a00000 40e00000 41100000|c0400000 c0400000 c0400000|c0400000 40c00000 c0400000|42000000"),
    // I: vector modulus and normalise, with zero and tiny inputs
    ("vmod 40400000 00000000 40800000|vnorm 40400000 00000000 40800000|vnorm 00000000 00000000 00000000|vnorm 0da24260 00000000 00000000|vnorm 1e3ce508 00000000 00000000", "40a00000|3f19999a 00000000 3f4ccccd|3f800000 00000000 00000000|3f800000 00000000 00000000|3f800016 00000000 00000000"),
    // J: vector reflect and scale
    ("vreflect 3f800000 bf800000 00000000 00000000 3f800000 00000000|vscale 3f800000 40000000 40400000 3f000000", "bf800000 bf800000 80000000|3f000000 3f800000 3fc00000"),
    // K: vector rotate
    ("vrot 3f800000 00000000 00000000 00000000 00000000 40000000 3fc90fdb|vrot 3f000000 bf800000 40000000 3f800000 3f800000 00000000 40000000", "00000000 3f800000 00000000 00000000 00000000 3f800000|3ea1be60 bf08bc50 bf48bf15 3f3504f3 3f3504f3 00000000"),
    // L: affine matrix product
    ("mmul 3f800000 40000000 40400000 41100000 00000000 3f800000 40800000 41000000 40a00000 00000000 3f800000 40e00000 40800000 40a00000 40c00000 3f800000 40000000 00000000 3f800000 c0400000 3f800000 3f800000 00000000 bf800000 00000000 40400000 3f800000 c0000000 3f800000 c0000000 40400000 40a00000", "40800000 41300000 40800000 00000000 3f800000 41500000 40800000 00000000 41200000 40400000 40c00000 00000000 41600000 41a80000 41500000 3f800000"),
    // M: rotation matrix about an axis
    ("mrot 3f800000 3f800000 00000000 3f860a92|mrot 00000000 00000000 c0800000 bfc90fdb", "3f3fffff 3e7fffff bf1cc471 00000000 3e7fffff 3f3fffff 3f1cc471 00000000 3f1cc471 bf1cc471 3effffff 00000000 00000000 00000000 00000000 3f800000 3f3504f3 3f3504f3 00000000|b33bbd2e 3f800000 00000000 00000000 bf800000 b33bbd2e 80000000 00000000 80000000 00000000 3f7fffff 00000000 00000000 00000000 00000000 3f800000 00000000 00000000 bf800000"),
    // N: affine vector transform
    ("vxform 3f800000 40000000 40400000 3f800000 40000000 40400000 41100000 00000000 3f800000 40800000 41000000 40a00000 00000000 3f800000 40e00000 40800000 40a00000 40c00000 3f800000", "41a00000 41100000 41a00000"),
    // O: lookup_key: exact, between keys, and past both ends
    ("alookup 5 00000000 3f800000 40000000 40800000 41000000 40000000|alookup 5 00000000 3f800000 40000000 40800000 41000000 40400000|alookup 5 00000000 3f800000 40000000 40800000 41000000 00000000|alookup 5 00000000 3f800000 40000000 40800000 41000000 41000000|alookup 5 00000000 3f800000 40000000 40800000 41000000 bf800000", "2|3|0|4|0"),
    // P: index_min and index_max over a run of equal times
    ("aindmin 5 3f800000 40000000 40000000 40000000 40400000 40000000|aindmin 5 3f800000 40000000 40000000 40000000 40400000 3f800000|aindmin 5 3f800000 40000000 40000000 40000000 40400000 40400000|aindmin 5 3f800000 40000000 40000000 40000000 40400000 00000000|aindmax 5 3f800000 40000000 40000000 40000000 40400000 40000000|aindmax 5 3f800000 40000000 40000000 40000000 40400000 3f800000|aindmax 5 3f800000 40000000 40000000 40000000 40400000 40400000|aindmax 5 3f800000 40000000 40000000 40000000 40400000 00000000", "1|0|4|0|3|0|4|0"),
    // Q: insert_position: before, inside, past and between keys
    ("apos 3 3f800000 40400000 40a00000 40000000|apos 3 3f800000 40400000 40a00000 40400000|apos 3 3f800000 40400000 40a00000 00000000|apos 3 3f800000 40400000 40a00000 41100000|apos 3 3f800000 40400000 40a00000 40800000", "0|1|0|3|2"),
    // R: range, including empty, reversed and wrapped counts
    ("arange 4 00000000 3f800000 40000000 40400000 40000000 40400000|arange 4 00000000 3f800000 40000000 40400000 3f800000 40000000|arange 4 00000000 3f800000 40000000 40400000 00000000 40400000|arange 4 00000000 3f800000 40000000 40400000 40400000 40400000|arange 4 00000000 3f800000 40000000 40400000 40a00000 40c00000|arange 4 00000000 3f800000 40000000 40400000 c0000000 bf800000|arange 0 00000000 3f800000", "2 0|1 1|0 4|3 18446744073709551614|none|none|none"),
    // S: palettise with duplicate colours, both flips
    ("palette 3 2 0 0a141e0a141e28323c46505a28323c0a141e|palette 3 2 1 0a141e0a141e28323c46505a28323c0a141e", "pal 3 4 1e140a3c32285a5046 403aee0ec94c19c5 8|pal 3 4 5a50463c32281e140a 271a6d5e59e459d9 8"),
    // T: more than 256 colours: palettise refuses, load expands
    ("palette 17 300 0 g15300|load 17 300 0 g15300", "none|bgrx 68 3992c52721c4f893 20400"),
    // U: load of at most 256 colours palettises
    ("load 2 2 1 g12|load 3 2 0 g18", "pal 4 4 bb844d622af36f370015dea6 9683344b866520ad 8|pal 6 4 6f370015dea6bb844d622af308d199af7740 988769560d90721e 8"),
    // V: load_indexed copies rows, both flips and empty
    ("indexed 2 3 1 g6|indexed 3 2 0 000102030405|indexed 0 3 0 -", "2eacf74c31031b50 6|a54ac5bf0ea60dde 6|cbf29ce484222325 0"),
    // W: size overflow guards
    ("load 9223372036854775807 2 0 -|indexed 9223372036854775808 2 0 -|palette 9223372036854775807 2 0 -", "err|err|none"),
];

#[test]
fn oracle_rows_match_wine() {
    for (commands, expected) in ROWS {
        assert_eq!(replay(commands), *expected, "commands: {commands}");
    }
}

#[test]
fn unknown_command_is_bad() {
    assert_eq!(run_command("bogus 1 2"), "bad-op");
    assert_eq!(run_command(""), "bad-op");
}

fn replay(commands: &str) -> String {
    commands
        .split('|')
        .map(run_command)
        .collect::<Vec<_>>()
        .join("|")
}

fn run_command(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    let op = words.first().copied().unwrap_or("");
    let a: &[&str] = words.get(1..).unwrap_or_default();
    match op {
        "rgba" => format!(
            "{:08x}",
            color_rgba(float(a[0]), float(a[1]), float(a[2]), float(a[3]))
        ),
        "alpha" => float_reply(color_alpha(hex(a[0]))),
        "red" => float_reply(color_red(hex(a[0]))),
        "green" => float_reply(color_green(hex(a[0]))),
        "blue" => float_reply(color_blue(hex(a[0]))),
        "qmul" => quaternion_reply(quaternion_multiply(
            quaternion(&a[0..4]),
            quaternion(&a[4..8]),
        )),
        "qmatrix" => matrix_reply(&matrix_from_quaternion(quaternion(&a[0..4]))),
        "qrot" => {
            let mut axis = vector(&a[0..3]);
            let q = quaternion_from_rotation(&mut axis, float(a[3]));
            format!("{} {}", quaternion_reply(q), vector_reply(axis))
        }
        "slerp" => quaternion_reply(quaternion_slerp(
            quaternion(&a[0..4]),
            quaternion(&a[4..8]),
            float(a[8]),
        )),
        "vadd" => vector_reply(vector_add(vector(&a[0..3]), vector(&a[3..6]))),
        "vsub" => vector_reply(vector_subtract(vector(&a[0..3]), vector(&a[3..6]))),
        "vcross" => vector_reply(vector_cross(vector(&a[0..3]), vector(&a[3..6]))),
        "vdot" => float_reply(vector_dot(vector(&a[0..3]), vector(&a[3..6]))),
        "vmod" => float_reply(vector_modulus(vector(&a[0..3]))),
        "vnorm" => {
            let mut v = vector(&a[0..3]);
            vector_normalize(&mut v);
            vector_reply(v)
        }
        "vreflect" => vector_reply(vector_reflect(vector(&a[0..3]), vector(&a[3..6]))),
        "vrot" => {
            let mut axis = vector(&a[3..6]);
            let r = vector_rotate(vector(&a[0..3]), &mut axis, float(a[6]));
            format!("{} {}", vector_reply(r), vector_reply(axis))
        }
        "vscale" => vector_reply(vector_scale(vector(&a[0..3]), float(a[3]))),
        "mmul" => matrix_reply(&matrix_multiply_affine(
            &matrix(&a[0..16]),
            &matrix(&a[16..32]),
        )),
        "mrot" => {
            let mut axis = vector(&a[0..3]);
            let m = matrix_set_rotation(&mut axis, float(a[3]));
            format!("{} {}", matrix_reply(&m), vector_reply(axis))
        }
        "vxform" => vector_reply(vector_transform_affine(
            vector(&a[0..3]),
            &matrix(&a[3..19]),
        )),
        "alookup" => lookup_key(&keys(a), query(a, 0)).to_string(),
        "aindmin" => index_min(&keys(a), query(a, 0)).to_string(),
        "aindmax" => index_max(&keys(a), query(a, 0)).to_string(),
        "apos" => insert_position(&keys(a), query(a, 0)).to_string(),
        "arange" => match range(&keys(a), query(a, 0), query(a, 1)) {
            Some((first, count)) => format!("{first} {count}"),
            None => "none".to_string(),
        },
        "palette" => match palettise(&bytes(a[3]), num(a[0]), num(a[1]), flag(a[2])) {
            Some(image) => palette_reply(&image),
            None => "none".to_string(),
        },
        "load" => match load_rgb24(&bytes(a[3]), num(a[0]), num(a[1]), flag(a[2])) {
            Ok(Rgb24Image::Palette(image)) => palette_reply(&image),
            Ok(Rgb24Image::Bgrx { pitch, pixels }) => {
                format!("bgrx {pitch} {:016x} {}", fnv1a(&pixels), pixels.len())
            }
            Err(_) => "err".to_string(),
        },
        "indexed" => match load_indexed(&bytes(a[3]), num(a[0]), num(a[1]), flag(a[2])) {
            Ok(pixels) => format!("{:016x} {}", fnv1a(&pixels), pixels.len()),
            Err(_) => "err".to_string(),
        },
        _ => "bad-op".to_string(),
    }
}

fn float_reply(value: f32) -> String {
    format!("{:08x}", value.to_bits())
}

fn floats(values: &[f32]) -> String {
    values
        .iter()
        .map(|&value| float_reply(value))
        .collect::<Vec<_>>()
        .join(" ")
}

fn vector_reply(v: Vector) -> String {
    floats(&[v.x, v.y, v.z])
}

fn quaternion_reply(q: Quaternion) -> String {
    floats(&[q.s, q.v.x, q.v.y, q.v.z])
}

fn matrix_reply(m: &Matrix) -> String {
    floats(m.as_flattened())
}

fn palette_reply(image: &Palettised) -> String {
    let colours: String = image
        .palette
        .iter()
        .map(|&[r, g, b]| format!("{r:02x}{g:02x}{b:02x}"))
        .collect();
    format!(
        "pal {} {} {colours} {:016x} {}",
        image.palette.len(),
        image.pitch,
        fnv1a(&image.indices),
        image.indices.len()
    )
}

/// FNV-1a, 64-bit, as the C harness hashes the bytes that a call produced.
fn fnv1a(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn num(text: &str) -> usize {
    text.parse().expect("decimal number")
}

fn flag(text: &str) -> bool {
    num(text) != 0
}

fn hex(text: &str) -> u32 {
    u32::from_str_radix(text, 16).expect("hex word")
}

fn float(text: &str) -> f32 {
    f32::from_bits(hex(text))
}

fn vector(a: &[&str]) -> Vector {
    Vector::new(float(a[0]), float(a[1]), float(a[2]))
}

fn quaternion(a: &[&str]) -> Quaternion {
    Quaternion::new(float(a[0]), vector(&a[1..]))
}

fn matrix(a: &[&str]) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| float(a[4 * i + j])))
}

/// The keys of an animation command: their count, then their times.
fn keys(a: &[&str]) -> Vec<Key<()>> {
    a[1..=num(a[0])]
        .iter()
        .map(|text| Key {
            time: float(text),
            value: (),
        })
        .collect()
}

/// The `i`th query time, which follows the keys.
fn query(a: &[&str], i: usize) -> f32 {
    float(a[num(a[0]) + 1 + i])
}

fn bytes(text: &str) -> Vec<u8> {
    match text.strip_prefix('g') {
        Some(len) => generated(num(len)),
        None if text == "-" => Vec::new(),
        None => unhex(text),
    }
}

/// The oracle's generated pattern: `((i * 2654435761 + 12345) >> 16) & 0xff`.
fn generated(len: usize) -> Vec<u8> {
    (0..len)
        .map(|i| ((i as u64 * 2654435761 + 12345) >> 16) as u8)
        .collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex byte"))
        .collect()
}
