//! Tests against Wine's oracle rows, then Rust-only edge cases.
//!
//! The rows are the replies of Wine's verbatim jxrlib.c bodies, built with
//! ASan and UBSan by the canon build script for this crate. The replay
//! function runs each command through the port and prints the result the way
//! the C harness does, so every row is compared as the exact text.

use super::*;
use crate::pixel_format::PIXEL_FORMAT_TABLE;
use std::fmt::Debug;
use std::str::FromStr;

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("bpp e3fed78f-e8db-4acf-84c1-e97f6136b327", "0"),
    ("bpp 6fddc324-4e03-4bfe-b185-3d77768dc917", "0"),
    ("stride 1 1", "1"),
    ("stride 7 3", "3"),
    ("stride 4294967295 128", "536870896"),
    ("read 0 16 16", "0"),
    ("read -2147467259 0 16", "-102"),
    ("read 0 3 16", "-102"),
    ("write 0 16 16", "0"),
    ("seek 0", "0 0"),
    ("seek -2147467259", "-102 -102"),
    ("frame 0", "0"),
    ("frame 1", "-2147467263"),
    ("init 1 0 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "-2003292412"),
    ("init 0 -2147024882 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "-2147024882"),
    ("init 0 0 0 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 0 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 1 3 0 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 1 3 1 0 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 0 64 32 1 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 0 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 0 0", "-2147467259"),
    ("init 0 0 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc917 1 64 32 1 42c00000 43964000 1 0", "-2147467259"),
    ("init 0 0 1 1 3 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90d 1 64 32 1 42c00000 43964000 1 0", "0 1 14 64 32 24 4058000000000000 4072c80000000000 0 0"),
    ("init 0 0 1 1 1 1 1 6fddc324-4e03-4bfe-b185-3d77768dc90f 1 -1 3 1 42900000 42900000 1 5", "0 1 14 4294967295 3 32 4052000000000000 4052000000000000 0 1"),
];

#[rustfmt::skip]
const TABLE: &[(&str, u32)] = &[
    ("6fddc324-4e03-4bfe-b185-3d77768dc91e", 128),
    ("6fddc324-4e03-4bfe-b185-3d77768dc919", 128),
    ("6fddc324-4e03-4bfe-b185-3d77768dc91b", 128),
    ("6fddc324-4e03-4bfe-b185-3d77768dc909", 16),
    ("6fddc324-4e03-4bfe-b185-3d77768dc90a", 16),
    ("6fddc324-4e03-4bfe-b185-3d77768dc90b", 16),
    ("6fddc324-4e03-4bfe-b185-3d77768dc913", 16),
    ("6fddc324-4e03-4bfe-b185-3d77768dc93e", 16),
    ("6fddc324-4e03-4bfe-b185-3d77768dc90c", 24),
    ("6fddc324-4e03-4bfe-b185-3d77768dc90d", 24),
    ("6fddc324-4e03-4bfe-b185-3d77768dc90e", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc914", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc90f", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc91c", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc93f", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc911", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc93d", 32),
    ("6fddc324-4e03-4bfe-b185-3d77768dc92c", 40),
    ("6fddc324-4e03-4bfe-b185-3d77768dc915", 48),
    ("6fddc324-4e03-4bfe-b185-3d77768dc912", 48),
    ("6fddc324-4e03-4bfe-b185-3d77768dc93b", 48),
    ("6fddc324-4e03-4bfe-b185-3d77768dc91f", 64),
    ("6fddc324-4e03-4bfe-b185-3d77768dc916", 64),
    ("6fddc324-4e03-4bfe-b185-3d77768dc91d", 64),
    ("6fddc324-4e03-4bfe-b185-3d77768dc93a", 64),
    ("6fddc324-4e03-4bfe-b185-3d77768dc92d", 80),
    ("6fddc324-4e03-4bfe-b185-3d77768dc908", 8),
    ("6fddc324-4e03-4bfe-b185-3d77768dc918", 96),
    ("6fddc324-4e03-4bfe-b185-3d77768dc905", 1),
];

const RGB24: &str = "6fddc324-4e03-4bfe-b185-3d77768dc90d";

fn parse_guid(text: &str) -> Guid {
    let hex = text.replace('-', "");
    assert_eq!(hex.len(), 32, "bad GUID {text}");
    let byte = |index: usize| u8::from_str_radix(&hex[2 * index..2 * index + 2], 16).unwrap();
    Guid {
        data1: u32::from_str_radix(&hex[0..8], 16).unwrap(),
        data2: u16::from_str_radix(&hex[8..12], 16).unwrap(),
        data3: u16::from_str_radix(&hex[12..16], 16).unwrap(),
        data4: std::array::from_fn(|index| byte(8 + index)),
    }
}

fn number<T: FromStr>(text: &str) -> T
where
    T::Err: Debug,
{
    text.parse().unwrap()
}

fn hex(text: &str) -> u32 {
    u32::from_str_radix(text, 16).unwrap()
}

fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split(' ').collect();
    match words[0] {
        "bpp" => pixel_format_bpp(&parse_guid(words[1])).to_string(),
        "stride" => frame_stride(number(words[1]), number(words[2])).to_string(),
        "read" | "write" => {
            transfer_status(number(words[1]), number(words[2]), number(words[3])).to_string()
        }
        "seek" => {
            let status = seek_status(number(words[1]));
            format!("{status} {status}")
        }
        "frame" => frame_index_hresult(number(words[1])).to_string(),
        "init" => replay_init(&words[1..]),
        other => panic!("unknown oracle command {other}"),
    }
}

fn replay_init(tokens: &[&str]) -> String {
    assert_eq!(tokens.len(), 16, "init takes 16 fields");
    let flag = |index: usize| tokens[index] == "1";
    let mut host = Replies {
        rewind_hr: number(tokens[1]),
        init_ok: flag(2),
        frame_count: flag(3).then_some(number(tokens[4])),
        select_ok: flag(5),
        pixel_format: flag(6).then(|| parse_guid(tokens[7])),
        size: flag(8).then(|| Dimensions {
            width: number(tokens[9]),
            height: number(tokens[10]),
        }),
        resolution: flag(11).then(|| Resolution {
            dpix: f32::from_bits(hex(tokens[12])),
            dpiy: f32::from_bits(hex(tokens[13])),
        }),
        color_context: flag(14).then_some(number(tokens[15])),
    };
    let mut decoder = WmpDecoder::default();
    if flag(0) {
        let _ = decoder.initialize(&mut Replies {
            rewind_hr: E_FAIL,
            ..Replies::default()
        });
    }
    match decoder.initialize(&mut host) {
        Ok(init) => format!(
            "0 {} {} {} {} {} {:016x} {:016x} {} {}",
            init.stat.frame_count,
            init.stat.flags,
            init.frame.width,
            init.frame.height,
            init.frame.bpp,
            init.frame.dpix.to_bits(),
            init.frame.dpiy.to_bits(),
            init.frame.num_colors,
            init.frame.num_color_contexts,
        ),
        Err(hr) => hr.to_string(),
    }
}

#[derive(Default)]
struct Replies {
    rewind_hr: i32,
    init_ok: bool,
    frame_count: Option<u32>,
    select_ok: bool,
    pixel_format: Option<Guid>,
    size: Option<Dimensions>,
    resolution: Option<Resolution>,
    color_context: Option<u32>,
}

impl JxrHost for Replies {
    fn rewind(&mut self) -> i32 {
        self.rewind_hr
    }
    fn initialize(&mut self) -> bool {
        self.init_ok
    }
    fn frame_count(&mut self) -> Option<u32> {
        self.frame_count
    }
    fn select_first_frame(&mut self) -> bool {
        self.select_ok
    }
    fn pixel_format(&mut self) -> Option<Guid> {
        self.pixel_format
    }
    fn size(&mut self) -> Option<Dimensions> {
        self.size
    }
    fn resolution(&mut self) -> Option<Resolution> {
        self.resolution
    }
    fn color_context_size(&mut self) -> Option<u32> {
        self.color_context
    }
}

fn working_host() -> Replies {
    Replies {
        init_ok: true,
        frame_count: Some(3),
        select_ok: true,
        pixel_format: Some(parse_guid(RGB24)),
        size: Some(Dimensions {
            width: 64,
            height: 32,
        }),
        resolution: Some(Resolution {
            dpix: 96.0,
            dpiy: 300.5,
        }),
        color_context: Some(0),
        ..Replies::default()
    }
}

#[test]
fn oracle_rows_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "{command}");
    }
}

#[test]
fn pixel_format_table_matches_wine_rows() {
    assert_eq!(PIXEL_FORMAT_TABLE.len(), TABLE.len());
    for ((guid, bpp), (text, expected)) in PIXEL_FORMAT_TABLE.iter().zip(TABLE) {
        assert_eq!(*guid, parse_guid(text));
        assert_eq!(*bpp, *expected);
    }
}

#[test]
fn lookup_matches_wine_table() {
    for (text, bpp) in TABLE {
        assert_eq!(pixel_format_bpp(&parse_guid(text)), *bpp, "{text}");
    }
}

#[test]
fn lookup_compares_every_field() {
    let base = parse_guid(RGB24);
    assert_eq!(pixel_format_bpp(&base), 24);
    assert_eq!(
        pixel_format_bpp(&Guid {
            data1: base.data1 ^ 1,
            ..base
        }),
        0
    );
    assert_eq!(
        pixel_format_bpp(&Guid {
            data2: base.data2 ^ 1,
            ..base
        }),
        0
    );
    assert_eq!(
        pixel_format_bpp(&Guid {
            data3: base.data3 ^ 1,
            ..base
        }),
        0
    );
    let mut data4 = base.data4;
    data4[0] ^= 0x80;
    assert_eq!(pixel_format_bpp(&Guid { data4, ..base }), 0);
    assert_eq!(pixel_format_bpp(&Guid::default()), 0);
}

#[test]
fn frame_stride_rounds_bits_up_to_whole_bytes() {
    assert_eq!(frame_stride(0, 24), 0);
    assert_eq!(frame_stride(8, 1), 1);
    assert_eq!(frame_stride(9, 1), 2);
    assert_eq!(frame_stride(64, 24), 192);
    assert_eq!(frame_stride(u32::MAX, 128), 536_870_896);
}

#[test]
fn transfer_is_file_io_on_error_or_short_count() {
    assert_eq!(transfer_status(0, 16, 16), WMP_ERR_SUCCESS);
    assert_eq!(transfer_status(0, 0, 0), WMP_ERR_SUCCESS);
    assert_eq!(transfer_status(1, 16, 16), WMP_ERR_SUCCESS);
    assert_eq!(transfer_status(0, 17, 16), WMP_ERR_FILE_IO);
    assert_eq!(transfer_status(E_FAIL, 16, 16), WMP_ERR_FILE_IO);
    assert_eq!(
        transfer_status(WINCODEC_ERR_WRONGSTATE, 0, 0),
        WMP_ERR_FILE_IO
    );
}

#[test]
fn seek_is_file_io_only_on_failure() {
    assert_eq!(seek_status(0), WMP_ERR_SUCCESS);
    assert_eq!(seek_status(E_NOTIMPL), WMP_ERR_FILE_IO);
}

#[test]
fn only_frame_zero_exists() {
    assert_eq!(frame_index_hresult(0), S_OK);
    assert_eq!(frame_index_hresult(1), E_NOTIMPL);
    assert_eq!(frame_index_hresult(u32::MAX), E_NOTIMPL);
}

#[test]
fn bound_stream_refuses_a_second_initialize() {
    let mut decoder = WmpDecoder::default();
    let mut failing = Replies {
        rewind_hr: E_FAIL,
        ..Replies::default()
    };
    assert_eq!(decoder.initialize(&mut failing).err(), Some(E_FAIL));
    assert_eq!(
        decoder.initialize(&mut working_host()).err(),
        Some(WINCODEC_ERR_WRONGSTATE)
    );
}

#[test]
fn debug_output_names_every_type() {
    let initialized = WmpDecoder::default()
        .initialize(&mut working_host())
        .unwrap();
    assert!(format!("{initialized:?}").starts_with("Initialized {"));
    assert!(format!("{:?}", initialized.stat).starts_with("DecoderStat {"));
    assert!(format!("{:?}", initialized.frame).starts_with("FrameInfo {"));
    assert!(format!(
        "{:?}",
        Dimensions {
            width: 1,
            height: 2
        }
    )
    .starts_with("Dimensions {"));
    assert!(format!(
        "{:?}",
        Resolution {
            dpix: 1.0,
            dpiy: 2.0
        }
    )
    .starts_with("Resolution {"));
    assert!(format!("{:?}", WmpDecoder::default()).starts_with("WmpDecoder {"));
    assert!(format!("{:?}", parse_guid(RGB24)).starts_with("Guid {"));
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn replay_rejects_unknown_commands() {
    replay("bogus 1");
}
