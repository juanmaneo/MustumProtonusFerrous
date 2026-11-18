//! Replays the commands of the Wine oracle (`target/canon/colorcnv`) against this crate.
//! The vectors are the verbatim output of Wine's `color_converter.c` under ASan/UBSan.

use super::*;

pub(super) const AUDIO: Guid = mf_subtype(0x7364_7561);

const NAMES: &[(&str, Guid)] = &[
    ("AYUV", MEDIASUBTYPE_AYUV),
    ("I420", MEDIASUBTYPE_I420),
    ("IYUV", MEDIASUBTYPE_IYUV),
    ("NV11", MEDIASUBTYPE_NV11),
    ("NV12", MEDIASUBTYPE_NV12),
    ("UYVY", MEDIASUBTYPE_UYVY),
    ("V216", MEDIASUBTYPE_V216),
    ("V410", MEDIASUBTYPE_V410),
    ("Y41P", MEDIASUBTYPE_Y41P),
    ("Y41T", MEDIASUBTYPE_Y41T),
    ("Y42T", MEDIASUBTYPE_Y42T),
    ("YUY2", MEDIASUBTYPE_YUY2),
    ("YV12", MEDIASUBTYPE_YV12),
    ("YVU9", MEDIASUBTYPE_YVU9),
    ("YVYU", MEDIASUBTYPE_YVYU),
    ("RGB8", MEDIASUBTYPE_RGB8),
    ("RGB555", MEDIASUBTYPE_RGB555),
    ("RGB565", MEDIASUBTYPE_RGB565),
    ("RGB24", MEDIASUBTYPE_RGB24),
    ("RGB32", MEDIASUBTYPE_RGB32),
    ("MFVideoFormat_RGB8", mf_subtype(41)),
    ("MFVideoFormat_RGB555", mf_subtype(24)),
    ("MFVideoFormat_RGB565", mf_subtype(23)),
    ("MFVideoFormat_RGB24", mf_subtype(20)),
    ("MFVideoFormat_RGB32", mf_subtype(22)),
];
const KINDS: &[(&str, ConverterKind)] = &[
    ("conv", ConverterKind::Converter),
    ("resz", ConverterKind::Resizer),
];

const FORMATS: &[(&str, Guid)] = &[("vi", FORMAT_VIDEO_INFO), ("mf", FORMAT_MF_VIDEO_FORMAT)];

fn lookup<T: Copy>(table: &[(&str, T)], name: &str) -> T {
    table
        .iter()
        .find(|(known, _)| *known == name)
        .map(|(_, value)| *value)
        .expect("name is in the oracle table")
}

fn int(text: &str) -> i64 {
    match text.strip_prefix("0x") {
        Some(hex) => i64::from_str_radix(hex, 16).expect("hex number"),
        None => text.parse().expect("decimal number"),
    }
}

fn canonical(guid: &Guid) -> String {
    let Guid {
        data1,
        data2,
        data3,
        data4: [a, b, c, d, e, f, g, h],
    } = *guid;
    format!(
        "{data1:08x}-{data2:04x}-{data3:04x}-{a:02x}{b:02x}-{c:02x}{d:02x}{e:02x}{f:02x}{g:02x}{h:02x}"
    )
}

fn write_guid(bytes: &mut [u8], at: usize, guid: &Guid) {
    bytes[at..at + 4].copy_from_slice(&guid.data1.to_le_bytes());
    bytes[at + 4..at + 6].copy_from_slice(&guid.data2.to_le_bytes());
    bytes[at + 6..at + 8].copy_from_slice(&guid.data3.to_le_bytes());
    bytes[at + 8..at + 16].copy_from_slice(&guid.data4);
}

/// A `VIDEOINFOHEADER` (`bmi` = 48) or `VIDEOINFOHEADER2` (`bmi` = 72) with the given header.
pub(super) fn header_bytes(
    bmi: usize,
    width: u32,
    height: i32,
    compression: u32,
    image: u32,
) -> Vec<u8> {
    let mut bytes = vec![0; 256];
    bytes[bmi + 4..bmi + 8].copy_from_slice(&width.to_le_bytes());
    bytes[bmi + 8..bmi + 12].copy_from_slice(&height.to_le_bytes());
    bytes[bmi + 16..bmi + 20].copy_from_slice(&compression.to_le_bytes());
    bytes[bmi + 20..bmi + 24].copy_from_slice(&image.to_le_bytes());
    bytes
}

fn video_info_bytes(width: u32, height: i32, compression: u32, image: u32) -> Vec<u8> {
    header_bytes(48, width, height, compression, image)
}

pub(super) fn mf_format_bytes(width: u32, height: u32, flags: u64, subtype: &Guid) -> Vec<u8> {
    let mut bytes = vec![0; 256];
    bytes[8..12].copy_from_slice(&width.to_le_bytes());
    bytes[12..16].copy_from_slice(&height.to_le_bytes());
    bytes[112..120].copy_from_slice(&flags.to_le_bytes());
    write_guid(&mut bytes, 120, subtype);
    bytes
}

pub(super) fn media<'a>(subtype: Guid, format_type: Guid, format: &'a [u8]) -> DmoMediaType<'a> {
    DmoMediaType {
        major_type: MEDIATYPE_VIDEO,
        subtype,
        format_type,
        format,
    }
}

fn outcome(result: Result<u32, HResult>) -> String {
    let hr = result.as_ref().err().copied().unwrap_or(S_OK);
    let suffix = result.map_or(String::new(), |size| format!(" size={size}"));
    format!("hr=0x{hr:08x}{suffix}")
}

pub(super) fn mf_image_size(_: &Guid, width: u32, height: u32) -> Result<u32, HResult> {
    Ok(width * height * 2)
}

fn fill_text(words: &[&str]) -> String {
    let format = VideoFormat {
        width: int(words[2]) as u32,
        height: int(words[3]) as u32,
        video_flags: int(words[5]) as u64,
        subtype: lookup(NAMES, words[1]),
    };
    fill(&format, int(words[4]) as i32, int(words[6]) as usize).map_or(
        "ret=-1".to_string(),
        |filled| {
            let planes = filled
                .planes
                .map(|plane| plane.map_or("-".to_string(), |offset| offset.to_string()));
            let strides = filled.strides.map(|stride| stride.to_string());
            format!(
                "size={} planes={} strides={}",
                filled.size,
                planes.join(","),
                strides.join(",")
            )
        },
    )
}

fn pixfmt_text(words: &[&str]) -> String {
    words[1..]
        .iter()
        .map(|name| {
            let pixel = pixel_format_from_video_subtype(&lookup(NAMES, name));
            pixel.name().unwrap_or("none")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn video_text(result: Result<VideoFormat, HResult>) -> String {
    let hr = result.as_ref().err().copied().unwrap_or(S_OK);
    result.map_or(format!("hr=0x{hr:08x}"), |format| {
        format!(
            "hr=0x{hr:08x} w={} h={} flags={:#x} sub={}",
            format.width,
            format.height,
            format.video_flags,
            canonical(&format.subtype)
        )
    })
}

fn vidfmt_text(words: &[&str]) -> String {
    let subtype = lookup(NAMES, words[2]);
    let (width, height) = (int(words[3]) as u32, int(words[4]) as i32);
    let bytes = if words[1] == "vi" {
        video_info_bytes(width, height, int(words[5]) as u32, 0)
    } else {
        mf_format_bytes(width, height as u32, int(words[5]) as u64, &subtype)
    };
    let result = init_video_format(&media(subtype, lookup(FORMATS, words[1]), &bytes));
    video_text(result)
}

fn check_text(words: &[&str]) -> String {
    let kind = lookup(KINDS, words[1]);
    let subtype = lookup(NAMES, words[3]);
    let length = int(words[5]) as usize;
    let (width, height, image) = (
        int(words[6]) as u32,
        int(words[7]) as i32,
        int(words[8]) as u32,
    );
    let bytes = if words[4] == "vi" {
        video_info_bytes(width, height, 0, image)
    } else {
        mf_format_bytes(width, height as u32, 0, &subtype)
    };
    let output = media(subtype, lookup(FORMATS, words[4]), &bytes[..length]);
    outcome(check_dmo_media_type(
        kind.output_types(),
        &output,
        mf_image_size,
    ))
}

fn align_text(words: &[&str]) -> String {
    let planes: [Option<usize>; 4] = std::array::from_fn(|i| {
        let text = words[3 + i];
        (text != "-").then(|| int(text) as usize)
    });
    let strides: [i32; 4] = std::array::from_fn(|i| int(words[7 + i]) as i32);
    let aligned = check_arrays_alignment(
        &planes,
        &strides,
        int(words[1]) as usize,
        int(words[2]) as usize,
    );
    u32::from(aligned).to_string()
}

fn avail_text(words: &[&str]) -> String {
    let kind = lookup(KINDS, words[1]);
    let result = available_subtype(
        kind.input_types(),
        int(words[2]) as u32,
        int(words[3]) as u32,
    );
    let hr = result.as_ref().err().copied().unwrap_or(S_OK);
    let sub = result.map_or("-".to_string(), |guid| canonical(&guid));
    format!("hr=0x{hr:08x} sub={sub}")
}

fn setout_text(words: &[&str]) -> String {
    let kind = lookup(KINDS, words[1]);
    let input = lookup(NAMES, words[2]);
    let output_subtype = lookup(NAMES, words[3]);
    let index = int(words[4]) as u32;
    let (width, height, image) = (
        int(words[5]) as u32,
        int(words[6]) as i32,
        int(words[7]) as u32,
    );
    let bytes = video_info_bytes(width, height, 0, image);
    let output = media(output_subtype, FORMAT_VIDEO_INFO, &bytes[..88]);
    outcome(check_output_type(
        kind,
        index,
        &input,
        &output,
        mf_image_size,
    ))
}

fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0] {
        "fill" => fill_text(&words),
        "pixfmt" => pixfmt_text(&words),
        "dmosub" => canonical(&get_dmo_subtype(&lookup(NAMES, words[1]))),
        "vidfmt" => vidfmt_text(&words),
        "check" => check_text(&words),
        "align" => align_text(&words),
        "mfres" => format!("0x{:08x}", mf_result_from_dmo(int(words[1]) as u32)),
        "avail" => avail_text(&words),
        _ => setout_text(&words),
    }
}

const VECTORS: &[(&str, &str)] = &[
    (
        "fill AYUV 3 2 0 0 4096",
        "size=24 planes=0,-,-,- strides=12,0,0,0",
    ),
    (
        "fill I420 5 3 0 0 4096",
        "size=27 planes=0,15,21,- strides=5,3,3,0",
    ),
    (
        "fill NV11 6 3 0 0 4096",
        "size=30 planes=0,18,24,- strides=6,2,2,0",
    ),
    (
        "fill NV12 5 4 0 0 4096",
        "size=32 planes=0,20,-,- strides=5,6,0,0",
    ),
    (
        "fill UYVY 5 2 0 0 4096",
        "size=24 planes=0,-,-,- strides=12,0,0,0",
    ),
    (
        "fill YUY2 4 2 0 0 4096",
        "size=16 planes=0,-,-,- strides=8,0,0,0",
    ),
    (
        "fill YVYU 4 2 0 0 4096",
        "size=16 planes=0,-,-,- strides=8,0,0,0",
    ),
    (
        "fill RGB24 3 2 0 0 4096",
        "size=18 planes=0,-,-,- strides=9,0,0,0",
    ),
    (
        "fill RGB32 3 2 0 0 4096",
        "size=24 planes=0,-,-,- strides=12,0,0,0",
    ),
    (
        "fill RGB555 3 2 0 0 4096",
        "size=12 planes=0,-,-,- strides=6,0,0,0",
    ),
    (
        "fill RGB8 5 2 0 0 4096",
        "size=10 planes=0,-,-,- strides=5,0,0,0",
    ),
    (
        "fill NV12 5 4 8 0 4096",
        "size=32 planes=0,32,-,- strides=8,9,0,0",
    ),
    (
        "fill RGB32 3 2 -16 0x80000 4096",
        "size=24 planes=16,-,-,- strides=-16,0,0,0",
    ),
    (
        "pixfmt AYUV I420 IYUV NV11 NV12 UYVY YUY2 YV12 YVU9 YVYU Y41P Y41T Y42T",
        "ayuv yuv420p yuv420p yuv411p nv12 uyvy422 yuyv422 yuv420p none yvyu422 none none none",
    ),
    (
        "pixfmt V410 V216 RGB8 RGB555 RGB565 RGB24 RGB32",
        "none none rgb8 rgb555le rgb565le bgr24 bgr0",
    ),
    (
        "dmosub MFVideoFormat_RGB32",
        "e436eb7e-524f-11ce-9f53-0020af0ba770",
    ),
    ("dmosub YUY2", "32595559-0000-0010-8000-00aa00389b71"),
    (
        "vidfmt vi RGB32 4 2 0",
        "hr=0x00000000 w=4 h=2 flags=0x80000 sub=e436eb7e-524f-11ce-9f53-0020af0ba770",
    ),
    (
        "vidfmt mf MFVideoFormat_RGB32 4 2 0x80000",
        "hr=0x00000000 w=4 h=2 flags=0x80000 sub=e436eb7e-524f-11ce-9f53-0020af0ba770",
    ),
    (
        "check conv video YUY2 vi 88 4 2 16",
        "hr=0x00000000 size=16",
    ),
    (
        "check conv video RGB32 mf 168 4 2 0",
        "hr=0x00000000 size=16",
    ),
    ("align 32 4 0 16 20 - 8 4 4 0", "1"),
    ("mfres 0x80040201", "0xc00d36b3"),
    (
        "avail resz 0 11",
        "hr=0x00000000 sub=32315659-0000-0010-8000-00aa00389b71",
    ),
    ("setout resz YUY2 YUY2 0 4 2 16", "hr=0x00000000 size=16"),
];

#[test]
fn oracle_vectors_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "{command}");
    }
}
