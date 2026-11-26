//! Oracle-derived test vectors for `copy_multisz`.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `copy_multiszA` and
//! `copy_multiszW` from `dlls/winscard/winscard.c`, with their `utf8_to_utf16`,
//! `utf16_to_ansi` and `utf8_to_ansi` helpers, under `target/canon/winscard/` with
//! `gcc -fsanitize=address,undefined -g`. The harness ran one process per row. 26 rows were
//! generated. Two are excluded: `A14` and `W12` (`ab\0cd\0`, a list with no final empty
//! string), where ASan reports a heap-buffer-overflow in Wine's loop. 24 rows remain.
//!
//! The `*_model` functions follow the harness's `MultiByteToWideChar` and
//! `WideCharToMultiByte` shims. Those shims are lenient: a malformed sequence becomes
//! U+FFFD and consumes one byte.

use super::*;

const SCARD_S_SUCCESS: i32 = 0;
const SCARD_E_INSUFFICIENT_BUFFER: i32 = 0x8010_0008_u32 as i32;

/// Decodes the UTF-8 sequence at the start of `s` (not empty) into `(code point, length)`.
fn decode_one(s: &[u8]) -> Option<(u32, usize)> {
    let lead = s[0];
    if lead < 0x80 {
        return Some((u32::from(lead), 1));
    }
    let (extra, min, bits) = match lead {
        0xC2..=0xDF => (1, 0x80, lead & 0x1F),
        0xE0..=0xEF => (2, 0x800, lead & 0x0F),
        0xF0..=0xF4 => (3, 0x1_0000, lead & 0x07),
        _ => return None,
    };
    let mut cp = u32::from(bits);
    for &b in s.get(1..=extra)? {
        if b & 0xC0 != 0x80 {
            return None;
        }
        cp = (cp << 6) | u32::from(b & 0x3F);
    }
    (cp >= min).then_some((cp, extra + 1))
}

/// Appends the UTF-16 units of `cp`, splitting code points of U+10000 and above.
fn push_utf16(cp: u32, units: &mut Vec<u16>) {
    if cp < 0x1_0000 {
        units.push(cp as u16);
    } else {
        let v = cp - 0x1_0000;
        units.push((0xD800 + (v >> 10)) as u16);
        units.push((0xDC00 + (v & 0x3FF)) as u16);
    }
}

/// Model of `MultiByteToWideChar(CP_UTF8)` for one NUL-terminated string. Returns the units
/// including the final NUL, or `None` if the input has no NUL.
pub(crate) fn utf8_to_utf16_model(src: &[u8]) -> Option<Vec<u16>> {
    let mut units = Vec::new();
    let mut i = 0;
    loop {
        if *src.get(i)? == 0 {
            units.push(0);
            return Some(units);
        }
        match decode_one(&src[i..]) {
            Some((cp, len)) => {
                push_utf16(cp, &mut units);
                i += len;
            }
            None => {
                units.push(0xFFFD);
                i += 1;
            }
        }
    }
}

/// Model of `WideCharToMultiByte(CP_ACP)`: units below 0x100 map to themselves, the rest to `?`.
pub(crate) fn utf16_to_ansi_model(units: &[u16]) -> Vec<u8> {
    units
        .iter()
        .map(|&u| u8::try_from(u).unwrap_or(b'?'))
        .collect()
}

/// `utf8_to_ansi`: the UTF-8 to ANSI conversion of one NUL-terminated string.
pub(crate) fn utf8_to_ansi_model(src: &[u8]) -> Option<Vec<u8>> {
    utf8_to_utf16_model(src).map(|units| utf16_to_ansi_model(&units))
}

/// The `SCARD_AUTOALLOCATE` / NULL / buffer mode a row was generated with.
pub(crate) enum Mode {
    Nothing,
    Query,
    Autoalloc,
    Buffer(usize),
}

/// One generated row: the input list, its mode, and the expected `(status, len, out)`.
struct Row<T: 'static> {
    label: &'static str,
    src: &'static [u8],
    mode: Mode,
    status: i32,
    len: usize,
    out: &'static [T],
}

/// Runs `copy_multisz` the way the harness's `run_a` and `run_w` do. Returns
/// `(status, len, out)`, where `out` holds the units the caller can read.
pub(crate) fn run<T>(
    src: &[u8],
    convert: &dyn Fn(&[u8]) -> Option<Vec<T>>,
    mode: &Mode,
) -> (i32, usize, Vec<T>)
where
    T: Copy + Default,
{
    let mut buf = Vec::new();
    if let Mode::Buffer(cap) = mode {
        buf.resize(*cap, T::default());
    }
    let dest = match mode {
        Mode::Nothing => None,
        Mode::Query => Some(Dest::Length),
        Mode::Autoalloc => Some(Dest::Allocate),
        Mode::Buffer(_) => Some(Dest::Buffer(&mut buf[..])),
    };
    match copy_multisz(src, convert, dest) {
        Ok(Written::Nothing) => (SCARD_S_SUCCESS, 0, Vec::new()),
        Ok(Written::Length(len)) => (SCARD_S_SUCCESS, len, Vec::new()),
        Ok(Written::Allocated(out)) => (SCARD_S_SUCCESS, out.len(), out),
        Ok(Written::Units(len)) => {
            buf.truncate(len);
            (SCARD_S_SUCCESS, len, buf)
        }
        Err(error) => {
            let len = match error {
                Error::InsufficientBuffer { required } => required,
                _ => 0,
            };
            (error.status(), len, Vec::new())
        }
    }
}

fn check_rows<T>(rows: &[Row<T>], convert: &dyn Fn(&[u8]) -> Option<Vec<T>>)
where
    T: Copy + Default + PartialEq + std::fmt::Debug + 'static,
{
    for row in rows {
        let got = run(row.src, convert, &row.mode);
        assert_eq!(
            got,
            (row.status, row.len, row.out.to_vec()),
            "{}",
            row.label
        );
    }
}

const ROWS_A: &[Row<u8>] = &[
    Row {
        label: "A1",
        src: b"\x00",
        mode: Mode::Nothing,
        status: SCARD_S_SUCCESS,
        len: 0,
        out: &[],
    },
    Row {
        label: "A2",
        src: b"\x00",
        mode: Mode::Query,
        status: SCARD_S_SUCCESS,
        len: 1,
        out: &[],
    },
    Row {
        label: "A3",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Query,
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[],
    },
    Row {
        label: "A4",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[0x61, 0x62, 0x00, 0x63, 0x64, 0x00, 0x00],
    },
    Row {
        label: "A5",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Buffer(7),
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[0x61, 0x62, 0x00, 0x63, 0x64, 0x00, 0x00],
    },
    Row {
        label: "A6",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Buffer(6),
        status: SCARD_E_INSUFFICIENT_BUFFER,
        len: 7,
        out: &[],
    },
    Row {
        label: "A7",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Buffer(0),
        status: SCARD_E_INSUFFICIENT_BUFFER,
        len: 7,
        out: &[],
    },
    Row {
        label: "A8",
        src: b"x\x00\x00",
        mode: Mode::Buffer(3),
        status: SCARD_S_SUCCESS,
        len: 3,
        out: &[0x78, 0x00, 0x00],
    },
    Row {
        label: "A9",
        src: b"\xc3\xa9\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 3,
        out: &[0xe9, 0x00, 0x00],
    },
    Row {
        label: "A10",
        src: b"\xe2\x82\xacab\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[0x3f, 0x61, 0x62, 0x00, 0x62, 0x00, 0x00],
    },
    Row {
        label: "A11",
        src: b"\xf0\x9f\x98\x80\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 6,
        out: &[0x3f, 0x3f, 0x00, 0x3f, 0x00, 0x00],
    },
    Row {
        label: "A12",
        src: b"\xff\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 3,
        out: &[0x3f, 0x00, 0x00],
    },
    Row {
        label: "A13",
        src: b"a\x00bb\x00ccc\x00\x00",
        mode: Mode::Buffer(12),
        status: SCARD_S_SUCCESS,
        len: 10,
        out: &[0x61, 0x00, 0x62, 0x62, 0x00, 0x63, 0x63, 0x63, 0x00, 0x00],
    },
];

const ROWS_W: &[Row<u16>] = &[
    Row {
        label: "W1",
        src: b"\x00",
        mode: Mode::Nothing,
        status: SCARD_S_SUCCESS,
        len: 0,
        out: &[],
    },
    Row {
        label: "W2",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Query,
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[],
    },
    Row {
        label: "W3",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[0x0061, 0x0062, 0x0000, 0x0063, 0x0064, 0x0000, 0x0000],
    },
    Row {
        label: "W4",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Buffer(6),
        status: SCARD_E_INSUFFICIENT_BUFFER,
        len: 7,
        out: &[],
    },
    Row {
        label: "W5",
        src: b"ab\x00cd\x00\x00",
        mode: Mode::Buffer(7),
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[0x0061, 0x0062, 0x0000, 0x0063, 0x0064, 0x0000, 0x0000],
    },
    Row {
        label: "W6",
        src: b"\xc3\xa9\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 3,
        out: &[0x00e9, 0x0000, 0x0000],
    },
    Row {
        label: "W7",
        src: b"\xe2\x82\xacab\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 7,
        out: &[0x20ac, 0x0061, 0x0062, 0x0000, 0x0062, 0x0000, 0x0000],
    },
    Row {
        label: "W8",
        src: b"\xf0\x9f\x98\x80\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 6,
        out: &[0xd83d, 0xde00, 0x0000, 0xfffd, 0x0000, 0x0000],
    },
    Row {
        label: "W9",
        src: b"\xff\x00\x00",
        mode: Mode::Autoalloc,
        status: SCARD_S_SUCCESS,
        len: 3,
        out: &[0xfffd, 0x0000, 0x0000],
    },
    Row {
        label: "W10",
        src: b"a\x00bb\x00ccc\x00\x00",
        mode: Mode::Buffer(10),
        status: SCARD_S_SUCCESS,
        len: 10,
        out: &[
            0x0061, 0x0000, 0x0062, 0x0062, 0x0000, 0x0063, 0x0063, 0x0063, 0x0000, 0x0000,
        ],
    },
    Row {
        label: "W11",
        src: b"x\x00\x00",
        mode: Mode::Query,
        status: SCARD_S_SUCCESS,
        len: 3,
        out: &[],
    },
];

#[test]
fn oracle_copy_multisz_a_rows() {
    check_rows(ROWS_A, &utf8_to_ansi_model);
}

#[test]
fn oracle_copy_multisz_w_rows() {
    check_rows(ROWS_W, &utf8_to_utf16_model);
}
