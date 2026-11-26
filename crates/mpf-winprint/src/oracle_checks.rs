//! Oracle-derived test vectors for the ported `printproc.c` logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `EnumPrintProcessorDatatypesW`, `OpenPrintProcessor`, `ClosePrintProcessor` and
//! `get_handle_data` from `dlls/winprint/printproc.c` with
//! `gcc -fshort-wchar -fsanitize=address,undefined -g` under `target/canon/winprint/`.
//! Each build (`-m64` and `-m32`) printed 22 rows. This file uses the 22 x64 rows plus the two
//! x86 enumeration rows (`E7x86`, `E8x86`) that show the 4-byte pointer layout: 24 vectors.
//! ASan and UBSan reported nothing, so no row was excluded.

use super::*;

/// One `EnumPrintProcessorDatatypesW` row. `needed` and `no` are `None` when the argument was
/// passed as NULL. `Some(v)` means it was passed and must hold `v` afterwards. The harness
/// starts `needed` at 0xdeadbeef and `no` at 99.
struct EnumRow {
    name: &'static str,
    arch: Arch,
    level: u32,
    size: u32,
    buf: bool,
    needed: Option<u32>,
    no: Option<u32>,
    result: Result<(), u32>,
    /// Offsets of the two `pName` values from the buffer start, on success.
    offsets: Option<(usize, usize)>,
}

const ENUM_ROWS: &[EnumRow] = &[
    EnumRow {
        name: "E1_level0_size0_nobuf",
        arch: Arch::X64,
        level: 0,
        size: 0,
        buf: false,
        needed: Some(34),
        no: Some(0),
        result: Err(87),
        offsets: None,
    },
    EnumRow {
        name: "E2_level2_size34_buf",
        arch: Arch::X64,
        level: 2,
        size: 34,
        buf: true,
        needed: Some(34),
        no: Some(0),
        result: Err(87),
        offsets: None,
    },
    EnumRow {
        name: "E3_level1_size100_nobuf",
        arch: Arch::X64,
        level: 1,
        size: 100,
        buf: false,
        needed: Some(34),
        no: Some(0),
        result: Err(87),
        offsets: None,
    },
    EnumRow {
        name: "E4_level1_size0_nobuf",
        arch: Arch::X64,
        level: 1,
        size: 0,
        buf: false,
        needed: Some(34),
        no: Some(0),
        result: Err(122),
        offsets: None,
    },
    EnumRow {
        name: "E5_level1_size33_buf",
        arch: Arch::X64,
        level: 1,
        size: 33,
        buf: true,
        needed: Some(34),
        no: Some(0),
        result: Err(122),
        offsets: None,
    },
    EnumRow {
        name: "E6_level1_size34_buf",
        arch: Arch::X64,
        level: 1,
        size: 34,
        buf: true,
        needed: Some(34),
        no: Some(2),
        result: Ok(()),
        offsets: Some((16, 24)),
    },
    EnumRow {
        name: "E7_level1_size25_buf",
        arch: Arch::X64,
        level: 1,
        size: 25,
        buf: true,
        needed: Some(34),
        no: Some(0),
        result: Err(122),
        offsets: None,
    },
    EnumRow {
        name: "E8_level1_size26_buf",
        arch: Arch::X64,
        level: 1,
        size: 26,
        buf: true,
        needed: Some(34),
        no: Some(0),
        result: Err(122),
        offsets: None,
    },
    EnumRow {
        name: "E9_null_needed",
        arch: Arch::X64,
        level: 1,
        size: 34,
        buf: true,
        needed: None,
        no: Some(99),
        result: Err(87),
        offsets: None,
    },
    EnumRow {
        name: "E10_null_no",
        arch: Arch::X64,
        level: 1,
        size: 34,
        buf: true,
        needed: Some(0xdead_beef),
        no: None,
        result: Err(87),
        offsets: None,
    },
    EnumRow {
        name: "E7x86_level1_size25_buf",
        arch: Arch::X86,
        level: 1,
        size: 25,
        buf: true,
        needed: Some(26),
        no: Some(0),
        result: Err(122),
        offsets: None,
    },
    EnumRow {
        name: "E8x86_level1_size26_buf",
        arch: Arch::X86,
        level: 1,
        size: 26,
        buf: true,
        needed: Some(26),
        no: Some(2),
        result: Ok(()),
        offsets: Some((8, 16)),
    },
];

/// One `OpenPrintProcessor` row. `datatype` is `None` for a NULL `pDatatype`. It is ignored
/// when `open_data` is false.
struct OpenRow {
    name: &'static str,
    port: bool,
    open_data: bool,
    datatype: Option<&'static str>,
    result: Result<(), u32>,
}

const OPEN_ROWS: &[OpenRow] = &[
    OpenRow {
        name: "O1_null_port",
        port: false,
        open_data: true,
        datatype: Some("RAW"),
        result: Err(87),
    },
    OpenRow {
        name: "O2_null_open_data",
        port: true,
        open_data: false,
        datatype: None,
        result: Err(87),
    },
    OpenRow {
        name: "O3_null_datatype",
        port: true,
        open_data: true,
        datatype: None,
        result: Err(87),
    },
    OpenRow {
        name: "O4_RAW",
        port: true,
        open_data: true,
        datatype: Some("RAW"),
        result: Ok(()),
    },
    OpenRow {
        name: "O5_TEXT",
        port: true,
        open_data: true,
        datatype: Some("TEXT"),
        result: Ok(()),
    },
    OpenRow {
        name: "O6_lowercase_raw",
        port: true,
        open_data: true,
        datatype: Some("raw"),
        result: Err(1804),
    },
    OpenRow {
        name: "O7_empty",
        port: true,
        open_data: true,
        datatype: Some(""),
        result: Err(1804),
    },
    OpenRow {
        name: "O8_RAW_then_NUL_junk",
        port: true,
        open_data: true,
        datatype: Some("RAW\0junk"),
        result: Ok(()),
    },
    OpenRow {
        name: "O9_RAWX",
        port: true,
        open_data: true,
        datatype: Some("RAWX"),
        result: Err(1804),
    },
];

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn code(r: Result<(), PpError>) -> Result<(), u32> {
    r.map_err(PpError::win32_code)
}

fn read_ptr(buf: &[u8], at: usize, ptr: usize) -> usize {
    let mut bytes = [0u8; 8];
    bytes[..ptr].copy_from_slice(&buf[at..at + ptr]);
    u64::from_le_bytes(bytes) as usize
}

fn read_wide(buf: &[u8], at: usize) -> String {
    let units: Vec<u16> = (at..buf.len())
        .step_by(2)
        .map(|i| u16::from_le_bytes([buf[i], buf[i + 1]]))
        .take_while(|&u| u != 0)
        .collect();
    String::from_utf16(&units).unwrap()
}

#[test]
fn oracle_enum_print_processor_datatypes_w_rows() {
    for row in ENUM_ROWS {
        let mut storage = vec![0u8; row.size as usize];
        let mut needed = 0xdead_beef;
        let mut no = 99;
        let datatypes = if row.buf {
            Some(storage.as_mut_slice())
        } else {
            None
        };
        let needed_arg = if row.needed.is_some() {
            Some(&mut needed)
        } else {
            None
        };
        let no_arg = if row.no.is_some() {
            Some(&mut no)
        } else {
            None
        };

        let got = enum_print_processor_datatypes_w(
            row.arch, row.level, row.size, datatypes, needed_arg, no_arg,
        );

        assert_eq!(code(got), row.result, "{}", row.name);
        if let Some(expected) = row.needed {
            assert_eq!(needed, expected, "{} needed", row.name);
        }
        if let Some(expected) = row.no {
            assert_eq!(no, expected, "{} no", row.name);
        }
        if let Some((off0, off1)) = row.offsets {
            let ptr = row.arch.pointer_size();
            assert_eq!(read_ptr(&storage, 0, ptr), off0, "{} off0", row.name);
            assert_eq!(read_ptr(&storage, ptr, ptr), off1, "{} off1", row.name);
            assert_eq!(read_wide(&storage, off0), "RAW", "{} s0", row.name);
            assert_eq!(read_wide(&storage, off1), "TEXT", "{} s1", row.name);
        }
    }
}

#[test]
fn oracle_check_open_print_processor_rows() {
    let port_units = wide("LPT1:");
    for row in OPEN_ROWS {
        let port = if row.port {
            Some(port_units.as_slice())
        } else {
            None
        };
        let datatype_units = row.datatype.map(wide);
        let open_data = OpenData {
            datatype: datatype_units.as_deref(),
        };
        let open = if row.open_data {
            Some(&open_data)
        } else {
            None
        };

        assert_eq!(
            code(check_open_print_processor(port, open)),
            row.result,
            "{}",
            row.name
        );
    }
}

#[test]
fn oracle_check_handle_data_rows() {
    // C1_null: ClosePrintProcessor(NULL) gives ret=0 err=6.
    assert_eq!(code(check_handle_data(None)), Err(6));
    // C2_bad_magic: a struct whose magic is 0 gives ret=0 err=6.
    assert_eq!(code(check_handle_data(Some(0))), Err(6));
    // C3_valid: a handle from OpenPrintProcessor gives ret=1 err=0.
    assert_eq!(code(check_handle_data(Some(PP_MAGIC))), Ok(()));
}
