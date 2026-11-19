//! Replays of Wine's oracle rows. The oracle compiles the verbatim C sources in
//! `target/canon/windowscodecs` with `-fsanitize=address,undefined`, runs them, and prints one line
//! per row. Each test rebuilds its row from the port and compares it with the printed text. Hashes
//! are FNV-1a over the little-endian bytes of each value, starting from `0xcbf29ce484222325`.

use super::*;

const FNV_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;
const PALETTE8: [u32; 8] = [
    0xff00_0000,
    0xffff_0000,
    0xff00_ff00,
    0xff00_00ff,
    0xffff_ffff,
    0xff80_8080,
    0xffff_ff00,
    0x0000_ffff,
];

pub(crate) fn fnv_byte(hash: u64, byte: u8) -> u64 {
    (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME)
}

pub(crate) fn fnv_u32(hash: u64, value: u32) -> u64 {
    value.to_le_bytes().into_iter().fold(hash, fnv_byte)
}

pub(crate) fn code<T>(result: Result<T, HResult>) -> HResult {
    result.err().unwrap_or(S_OK)
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn words(values: &[u32]) -> String {
    values
        .iter()
        .map(|value| format!("{value:08x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn prefixed_words(values: &[u32]) -> String {
    values
        .iter()
        .map(|value| format!("0x{value:08x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn list_line(tag: &str, values: &[u32]) -> String {
    let items: Vec<String> = values.iter().map(u32::to_string).collect();
    format!("{tag} {}", items.join(","))
}

pub(crate) fn info_line(info: &DdsInfo) -> String {
    format!(
        "w={} h={} depth={} mip={} array={} frames={} data={} bpb={} format={} dim={} alpha={} pf={} bpp={}",
        info.width,
        info.height,
        info.depth,
        info.mip_levels,
        info.array_size,
        info.frame_count,
        info.data_offset,
        info.bytes_per_block,
        info.format,
        info.dimension as u32,
        info.alpha_mode,
        info.pixel_format as u32,
        info.pixel_format_bpp,
    )
}

pub(crate) fn legacy_header(
    width: u32,
    height: u32,
    mip_map_count: u32,
    four_cc: u32,
) -> DdsHeader {
    DdsHeader {
        size: DDS_HEADER_SIZE,
        height,
        width,
        mip_map_count,
        pixel_format: DdsPixelFormat {
            flags: DDPF_FOURCC,
            four_cc,
            ..DdsPixelFormat::default()
        },
        ..DdsHeader::default()
    }
}

pub(crate) fn dx10_header(
    dxgi_format: DxgiFormat,
    resource_dimension: u32,
    misc_flag: u32,
    array_size: u32,
    misc_flags2: u32,
) -> DdsHeaderDxt10 {
    DdsHeaderDxt10 {
        dxgi_format,
        resource_dimension,
        misc_flag,
        array_size,
        misc_flags2,
    }
}

/// An 8x4 uncompressed header with 32-bit RGBA masks and no FourCC, as oracle row DDS13(e) builds.
pub(crate) fn rgb32_header() -> DdsHeader {
    DdsHeader {
        size: DDS_HEADER_SIZE,
        height: 4,
        width: 8,
        mip_map_count: 1,
        pixel_format: DdsPixelFormat {
            flags: DDPF_RGB,
            four_cc: 0,
            rgb_bit_count: 32,
            r_mask: 0xff,
            g_mask: 0xff00,
            b_mask: 0xff_0000,
            a_mask: 0xff00_0000,
        },
        ..DdsHeader::default()
    }
}

/// Lays out a DDS file the way `DDS_HEADER` and `DDS_HEADER_DXT10` sit in memory, then keeps the
/// first `len` bytes, so a short file can be built. The `ddspf.size` field is always 32.
pub(crate) fn file_bytes(
    magic: u32,
    header: &DdsHeader,
    dxt10: Option<&DdsHeaderDxt10>,
    len: usize,
) -> Vec<u8> {
    let mut file = vec![0_u8; 256];
    put(&mut file, 0, magic);
    put(&mut file, 4, header.size);
    put(&mut file, 12, header.height);
    put(&mut file, 16, header.width);
    put(&mut file, 24, header.depth);
    put(&mut file, 28, header.mip_map_count);
    put(&mut file, 76, 32);
    put(&mut file, 80, header.pixel_format.flags);
    put(&mut file, 84, header.pixel_format.four_cc);
    put(&mut file, 88, header.pixel_format.rgb_bit_count);
    put(&mut file, 92, header.pixel_format.r_mask);
    put(&mut file, 96, header.pixel_format.g_mask);
    put(&mut file, 100, header.pixel_format.b_mask);
    put(&mut file, 104, header.pixel_format.a_mask);
    put(&mut file, 112, header.caps2);
    if let Some(ext) = dxt10 {
        put(&mut file, 128, ext.dxgi_format);
        put(&mut file, 132, ext.resource_dimension);
        put(&mut file, 136, ext.misc_flag);
        put(&mut file, 140, ext.array_size);
        put(&mut file, 144, ext.misc_flags2);
    }
    file.truncate(len);
    file
}

fn put(file: &mut [u8], at: usize, value: u32) {
    file[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

pub(crate) fn predefined(wic: u32, add: bool) -> Vec<u32> {
    predefined_palette(PaletteType::from_wic(wic).unwrap(), add)
}

/// The hash that `PAL02` to `PAL04` print: for each type, then for `add` false and true, the entry
/// count and then every colour.
pub(crate) fn palette_hash(types: &[u32]) -> u64 {
    let mut hash = FNV_BASIS;
    for &wic in types {
        for add in [false, true] {
            let colors = predefined(wic, add);
            hash = fnv_u32(hash, colors.len() as u32);
            for color in colors {
                hash = fnv_u32(hash, color);
            }
        }
    }
    hash
}

#[test]
fn oracle_cnv01() {
    let hash = (0..=u16::MAX).fold(FNV_BASIS, |hash, value| {
        fnv_u32(hash, float_16_to_32(value).to_bits())
    });
    assert_eq!(
        format!("CNV01 f16 sweep 0x{hash:016x}"),
        "CNV01 f16 sweep 0xee8266aea060e545"
    );
}

#[test]
fn oracle_cnv02() {
    let bits: Vec<u32> = [0x3c00, 0x0001, 0x7c00, 0x8000, 0xc000]
        .into_iter()
        .map(|value| float_16_to_32(value).to_bits())
        .collect();
    assert_eq!(
        format!("CNV02 f16 {}", prefixed_words(&bits)),
        "CNV02 f16 0x3f800000 0x33800000 0x47800000 0x80000000 0xc0000000"
    );
}

#[test]
fn oracle_cnv03() {
    let explicit = rgb_to_palette_index([16, 200, 30], &PALETTE8);
    let mut hash = FNV_BASIS;
    for blue in (0..=255_u8).step_by(17) {
        for green in (0..=255_u8).step_by(17) {
            for red in (0..=255_u8).step_by(17) {
                let index = rgb_to_palette_index([blue, green, red], &PALETTE8);
                hash = fnv_u32(hash, index as u32);
            }
        }
    }
    assert_eq!(
        format!("CNV03 palette explicit {explicit} grid 0x{hash:016x}"),
        "CNV03 palette explicit 2 grid 0xa0dd6f2d9707df55"
    );
}

#[test]
fn oracle_pal01() {
    let counts: Vec<String> = (2..=12)
        .flat_map(|wic| [false, true].map(|add| predefined(wic, add).len().to_string()))
        .collect();
    assert_eq!(
        format!("PAL01 counts {}", counts.join(",")),
        "PAL01 counts 2,2,16,17,28,29,72,73,126,127,224,225,252,253,256,256,4,4,16,16,256,256"
    );
}

#[test]
fn oracle_pal02() {
    assert_eq!(
        format!("PAL02 0x{:016x}", palette_hash(&[0x2, 0xa, 0xb, 0xc])),
        "PAL02 0x78e2ec9ccc44ac79"
    );
}

#[test]
fn oracle_pal03() {
    assert_eq!(
        format!("PAL03 0x{:016x}", palette_hash(&[0x3, 0x4, 0x5, 0x6])),
        "PAL03 0x45e1528a4f6509a5"
    );
}

#[test]
fn oracle_pal04() {
    assert_eq!(
        format!("PAL04 0x{:016x}", palette_hash(&[0x7, 0x8, 0x9])),
        "PAL04 0x356decf08c258781"
    );
}

#[test]
fn oracle_dds01() {
    let hash = (0..=u16::MAX).fold(FNV_BASIS, |hash, color| {
        fnv_u32(hash, rgb565_to_argb(color, 0x5a))
    });
    assert_eq!(
        format!("DDS01 rgb565 sweep 0x{hash:016x}"),
        "DDS01 rgb565 sweep 0xd6a3ff4e20cbc8c5"
    );
}

#[test]
fn oracle_dds02() {
    let four_ccs = [
        FOURCC_DXT1,
        FOURCC_DXT2,
        FOURCC_DXT3,
        FOURCC_DXT4,
        FOURCC_DXT5,
        FOURCC_BC4U,
        FOURCC_DX10,
    ];
    let modes: Vec<u32> = four_ccs
        .into_iter()
        .map(|four_cc| alpha_mode_from_fourcc(four_cc) as u32)
        .collect();
    assert_eq!(
        list_line("DDS02 alpha", &modes),
        "DDS02 alpha 2,2,0,2,0,0,0"
    );
}

#[test]
fn oracle_dds03() {
    let dims: Vec<u32> = [(4, 3), (0, 2), (0, 3), (0, 4), (0, 9)]
        .into_iter()
        .map(|(misc_flag, resource_dimension)| {
            let dxt10 = dx10_header(DXGI_FORMAT_UNKNOWN, resource_dimension, misc_flag, 0, 0);
            dimension(&DdsHeader::default(), Some(&dxt10)) as u32
        })
        .collect();
    assert_eq!(
        list_line("DDS03 dim_dxt10", &dims),
        "DDS03 dim_dxt10 3,0,1,2,1"
    );
}

#[test]
fn oracle_dds04() {
    let dims: Vec<u32> = [
        DDSCAPS2_CUBEMAP,
        DDSCAPS2_VOLUME,
        0,
        DDSCAPS2_CUBEMAP | DDSCAPS2_VOLUME,
    ]
    .into_iter()
    .map(|caps2| {
        dimension(
            &DdsHeader {
                caps2,
                ..DdsHeader::default()
            },
            None,
        ) as u32
    })
    .collect();
    assert_eq!(
        list_line("DDS04 dim_legacy", &dims),
        "DDS04 dim_legacy 3,2,1,3"
    );
}

#[test]
fn oracle_dds05() {
    let counts = [
        frame_count(4, 3, 2, Dimension::Cube),
        frame_count(1, 5, 1, Dimension::Texture2D),
        frame_count(4, 3, 2, Dimension::Texture2D),
        frame_count(8, 4, 1, Dimension::Texture3D),
    ];
    assert_eq!(
        list_line("DDS05 frame_count", &counts),
        "DDS05 frame_count 84,5,14,15"
    );
}

#[test]
fn oracle_dds06() {
    let info = DdsInfo {
        depth: 4,
        mip_levels: 3,
        array_size: 2,
        frame_count: 14,
        dimension: Dimension::Texture2D,
        ..DdsInfo::default()
    };
    let frames: Vec<String> = (0..=14)
        .map(|index| {
            let frame = frame_dds_index(index, &info).unwrap();
            format!(
                "({},{},{})",
                frame.array_index, frame.mip_level, frame.slice_index
            )
        })
        .collect();
    assert_eq!(
        format!("DDS06 frame_index {}", frames.join(" ")),
        "DDS06 frame_index (0,0,0) (0,0,1) (0,0,2) (0,0,3) (0,1,0) (0,1,1) (0,2,0) (1,0,0) (1,0,1) (1,0,2) (1,0,3) (1,1,0) (1,1,1) (1,2,0) (2,0,0)"
    );
}

#[test]
fn oracle_dds07() {
    let hash = (0..=255_u32).fold(FNV_BASIS, |hash, format| {
        fnv_byte(hash, u8::from(is_compressed(format)))
    });
    assert_eq!(
        format!("DDS07 is_compressed sweep 0x{hash:016x}"),
        "DDS07 is_compressed sweep 0xaff09accc765783c"
    );
}

#[test]
fn oracle_dds08() {
    let hash = (0..=255_u32).fold(FNV_BASIS, |hash, format| {
        fnv_byte(hash, bytes_per_block_from_format(format) as u8)
    });
    assert_eq!(
        format!("DDS08 bytes_per_block sweep 0x{hash:016x}"),
        "DDS08 bytes_per_block sweep 0x3b627551492f6a25"
    );
}

#[test]
fn oracle_dds09() {
    let formats: Vec<u32> = [0x1c, 0x47, 0x43, 0]
        .into_iter()
        .map(|format| dxgi_to_wic(format) as u32)
        .collect();
    assert_eq!(
        list_line("DDS09 dxgi_to_wic", &formats),
        "DDS09 dxgi_to_wic 9,0,22,0"
    );
}

#[test]
fn oracle_dds10() {
    let info = dds_info(&legacy_header(8, 4, 1, FOURCC_DXT1), None);
    assert_eq!(
        format!("DDS10 {}", info_line(&info)),
        "DDS10 w=8 h=4 depth=1 mip=1 array=1 frames=1 data=128 bpb=8 format=71 dim=1 alpha=2 pf=1 bpp=32"
    );
}

#[test]
fn oracle_dds11() {
    let info = dds_info(
        &legacy_header(8, 4, 1, FOURCC_DX10),
        Some(&dx10_header(0x62, 3, 0, 3, 8)),
    );
    assert_eq!(
        format!("DDS11 {}", info_line(&info)),
        "DDS11 w=8 h=4 depth=1 mip=1 array=3 frames=3 data=148 bpb=16 format=98 dim=1 alpha=8 pf=2 bpp=32"
    );
}

#[test]
fn oracle_dds12() {
    let parsed = parse_header(&file_bytes(
        DDS_MAGIC,
        &legacy_header(8, 4, 1, FOURCC_DXT5),
        None,
        128,
    ));
    assert_eq!(
        format!(
            "DDS12 hr=0x{:08x} {}",
            code(parsed),
            info_line(&parsed.unwrap())
        ),
        "DDS12 hr=0x00000000 w=8 h=4 depth=1 mip=1 array=1 frames=1 data=128 bpb=16 format=77 dim=1 alpha=0 pf=2 bpp=32"
    );
}

#[test]
fn oracle_dds13() {
    let dxt5 = legacy_header(8, 4, 1, FOURCC_DXT5);
    let dx10 = legacy_header(8, 4, 1, FOURCC_DX10);
    let short_size = DdsHeader { size: 123, ..dxt5 };
    let codes = [
        code(parse_header(&file_bytes(0x1234_5678, &dxt5, None, 128))),
        code(parse_header(&file_bytes(DDS_MAGIC, &short_size, None, 128))),
        code(parse_header(&file_bytes(
            DDS_MAGIC,
            &dx10,
            Some(&dx10_header(0x47, 3, 0, 1, 0)),
            138,
        ))),
        code(parse_header(&file_bytes(
            DDS_MAGIC,
            &dx10,
            Some(&dx10_header(0x47, 3, 4, 1, 0)),
            148,
        ))),
        code(parse_header(&file_bytes(
            DDS_MAGIC,
            &rgb32_header(),
            None,
            128,
        ))),
        code(parse_header(&file_bytes(DDS_MAGIC, &short_size, None, 3))),
    ];
    assert_eq!(
        format!("DDS13 {}", prefixed_words(&codes)),
        "DDS13 0x88982f07 0x88982f61 0x88982f72 0x88982f61 0x88982f61 0x88982f72"
    );
}

#[test]
fn oracle_gif01() {
    let map = [
        Rgb {
            red: 1,
            green: 2,
            blue: 3,
        },
        Rgb {
            red: 0x10,
            green: 0x20,
            blue: 0x30,
        },
        Rgb {
            red: 0xaa,
            green: 0xbb,
            blue: 0xcc,
        },
        Rgb::default(),
    ];
    let extensions = [ExtensionBlock {
        function: GRAPHICS_EXT_FUNC_CODE,
        bytes: vec![0, 0, 0, 1, 0, 0, 2, 0],
    }];
    let mut colors = [0_u32; 4];
    copy_palette(Some(map.as_slice()), &extensions, 4, &mut colors);
    assert_eq!(
        format!("GIF01 palette {}", words(&colors)),
        "GIF01 palette ff010203 ff102030 00aabbcc ff000000"
    );
}

#[test]
fn oracle_gif02() {
    let extensions = [ExtensionBlock {
        function: GRAPHICS_EXT_FUNC_CODE,
        bytes: vec![0, 0, 0, 1, 0, 0, 0, 0],
    }];
    let mut colors = [0_u32; 5];
    copy_palette(None, &extensions, 5, &mut colors);
    assert_eq!(
        format!("GIF02 palette {}", words(&colors)),
        "GIF02 palette 00000000 ffffffff ff000000 ff000000 ff000000"
    );
}

#[test]
fn oracle_gif03() {
    let src: Vec<u8> = (0..88).collect();
    let mut dst = [0_u8; 55];
    let result = copy_interlaced_pixels(&src, 5, 11, 5, None, 5, &mut dst);
    assert_eq!(
        format!("GIF03 hr=0x{:08x} {}", code(result), hex(&dst)),
        "GIF03 hr=0x00000000 00010203041e1f2021220f1011121323242526270a0b0c0d0e28292a2b2c14151617182d2e2f303105060708093233343536191a1b1c1d"
    );
}

#[test]
fn oracle_gif04() {
    let src: Vec<u8> = (0..88).collect();
    let rect = WicRect {
        x: 1,
        y: 2,
        width: 3,
        height: 5,
    };
    let mut dst = [0_u8; 19];
    let result = copy_interlaced_pixels(&src, 5, 11, 8, Some(rect), 4, &mut dst);
    assert_eq!(
        format!("GIF04 hr=0x{:08x} {}", code(result), hex(&dst)),
        "GIF04 hr=0x00000000 191a1b00393a3b001112130041424300212223"
    );
}
