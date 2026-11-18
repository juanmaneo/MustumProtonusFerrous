//! Replays the commands of the Wine oracle (`target/canon/msvproc`) against this crate.
//! The vectors are the verbatim output of Wine's `video_processor.c` helpers under ASan/UBSan.
//! Plane addresses are written as offsets from the oracle's buffer base, and `-` is a NULL plane.

use crate::aperture::{content_rect, frame_dimensions, update_aperture};
use crate::format::{pixel_format_from_subtype, PixelFormat};
use crate::layout::{apply_pitch, arrays_aligned};
use crate::{Offset, VideoArea, VideoInfo};

const PIXEL_FORMATS: &[(&str, PixelFormat)] = &[
    ("none", PixelFormat::None),
    ("yuv420p", PixelFormat::Yuv420p),
    ("yuv411p", PixelFormat::Yuv411p),
    ("nv12", PixelFormat::Nv12),
    ("yuyv422", PixelFormat::Yuyv422),
    ("yvyu422", PixelFormat::Yvyu422),
    ("uyvy422", PixelFormat::Uyvy422),
    ("ayuv", PixelFormat::Ayuv),
    ("y216le", PixelFormat::Y216),
    ("rgba", PixelFormat::Rgba),
    ("bgra", PixelFormat::Bgra),
    ("bgr24", PixelFormat::Bgr24),
    ("bgr0", PixelFormat::Bgr0),
    ("rgb555le", PixelFormat::Rgb555),
    ("rgb565le", PixelFormat::Rgb565),
    ("rgb8", PixelFormat::Rgb8),
];

/// The oracle's base address, a multiple of 64 like its buffer.
const BASE: usize = 0x10_0000;

fn int(text: &str) -> i32 {
    text.parse().expect("integer")
}

fn address(token: &str) -> usize {
    if token == "-" {
        0
    } else {
        BASE.wrapping_add_signed(int(token) as isize)
    }
}

fn offset_text(address: usize) -> String {
    if address == 0 {
        "-".to_string()
    } else {
        (address as isize - BASE as isize).to_string()
    }
}

/// The six words `ox oy cx cy width height` describe one `MFVideoInfo`.
fn info(words: &[&str]) -> VideoInfo {
    VideoInfo {
        width: words[4].parse().expect("width"),
        height: words[5].parse().expect("height"),
        minimum_display_aperture: VideoArea {
            offset_x: Offset {
                fract: 0,
                value: int(words[0]) as i16,
            },
            offset_y: Offset {
                fract: 0,
                value: int(words[1]) as i16,
            },
            cx: int(words[2]),
            cy: int(words[3]),
        },
    }
}

fn aperture_text(info: &VideoInfo) -> String {
    let area = &info.minimum_display_aperture;
    format!(
        "{} {} {} {}",
        area.offset_x.value, area.offset_y.value, area.cx, area.cy
    )
}

fn subtype_text(words: &[&str]) -> String {
    let data1 = match words[1] {
        "fourcc" => u32::from_le_bytes(words[2].as_bytes().try_into().expect("four characters")),
        _ => words[2].parse().expect("D3DFMT value"),
    };
    let format = pixel_format_from_subtype(data1);
    let name = PIXEL_FORMATS
        .iter()
        .find(|(_, known)| *known == format)
        .expect("format is named")
        .0;
    format!("fmt={name}")
}

fn frame_text(words: &[&str]) -> String {
    let (width, height) = frame_dimensions(&info(&words[1..7]));
    format!("frame={width}x{height}")
}

fn content_text(words: &[&str]) -> String {
    let rect = content_rect(&info(&words[1..7]));
    format!(
        "rect={} {} {} {}",
        rect.left, rect.top, rect.right, rect.bottom
    )
}

fn update_text(words: &[&str]) -> String {
    let mut input = info(&words[1..7]);
    let mut output = info(&words[7..13]);
    let mismatch = update_aperture(&mut input, &mut output);
    format!(
        "in={} out={} mismatch={}",
        aperture_text(&input),
        aperture_text(&output),
        u8::from(mismatch)
    )
}

fn pitch_text(words: &[&str]) -> String {
    let planes = [1, 2, 3, 4].map(|i| address(words[i]));
    let strides = [5, 6, 7, 8].map(|i| int(words[i]));
    let (planes, strides) = apply_pitch(
        planes,
        strides,
        int(words[9]),
        int(words[10]) as u32,
        int(words[11]) != 0,
    );
    format!(
        "planes={} strides={},{},{},{}",
        planes.map(offset_text).join(","),
        strides[0],
        strides[1],
        strides[2],
        strides[3]
    )
}

fn aligned_text(words: &[&str]) -> String {
    let planes = [1, 2, 3, 4].map(|i| address(words[i]));
    let strides = [5, 6, 7, 8].map(|i| int(words[i]));
    let aligned = arrays_aligned(planes, strides, int(words[9]), int(words[10]));
    format!("aligned={}", u8::from(aligned))
}

fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0] {
        "subtype" => subtype_text(&words),
        "frame" => frame_text(&words),
        "content" => content_text(&words),
        "update" => update_text(&words),
        "pitch" => pitch_text(&words),
        _ => aligned_text(&words),
    }
}

const VECTORS: &[(&str, &str)] = &[
    ("subtype fourcc I420", "fmt=yuv420p"),
    ("subtype fourcc NV11", "fmt=yuv411p"),
    ("subtype fourcc Y216", "fmt=y216le"),
    ("subtype fourcc 420O", "fmt=none"),
    ("subtype fourcc P208", "fmt=none"),
    ("subtype d3d 32", "fmt=rgba"),
    ("subtype d3d 24", "fmt=rgb555le"),
    ("subtype d3d 99", "fmt=none"),
    ("frame 8 4 624 472 640 480", "frame=632x476"),
    ("frame -2 0 0 3 4 5", "frame=-2x3"),
    ("content 0 0 0 0 1920 1080", "rect=0 0 1920 1080"),
    ("content 16 8 1888 1064 1920 1080", "rect=16 8 1904 1072"),
    (
        "update 0 0 0 0 640 480 0 0 0 0 640 480",
        "in=0 0 640 480 out=0 0 640 480 mismatch=0",
    ),
    (
        "update 16 8 624 464 640 480 0 0 0 0 640 480",
        "in=16 8 624 464 out=16 8 624 464 mismatch=1",
    ),
    (
        "pitch 0 15 21 - 5 3 3 0 8 3 0",
        "planes=0,24,20,- strides=8,4,4,0",
    ),
    (
        "pitch 0 - - - 20 0 0 0 0 4 1",
        "planes=60,-,-,- strides=-20,0,0,0",
    ),
    (
        "pitch 0 - - - 20 0 0 0 -20 4 1",
        "planes=60,-,-,- strides=-20,0,0,0",
    ),
    (
        "pitch 0 20 - - 5 6 0 0 10 4 0",
        "planes=0,40,-,- strides=10,12,0,0",
    ),
    ("aligned 0 - - - 16 16 0 0 32 16", "aligned=1"),
    ("aligned 0 8 - - 16 16 0 0 32 16", "aligned=0"),
    ("aligned 0 - - - 16 -16 0 0 32 16", "aligned=1"),
    ("aligned 0 - - - 16 16 0 0 24 16", "aligned=0"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "{command}");
    }
}
