//! Oracle-derived vectors for `get_compression_types`.
//!
//! Captured by compiling the verbatim `DPVOICE_GetCompressionTypes` body from
//! `dlls/dpvoice/server.c`, together with the `DVCOMPRESSIONINFO` and `DVERR_*`
//! definitions from `include/dvoice.h`, using
//! `gcc -fsanitize=address,undefined -g -fshort-wchar` (Wine's wide-char ABI).
//! The harness ran natively (Win64 layout) and with `-m32` (Win32 layout), 12 rows
//! each, 24 rows in total. ASan and UBSan reported nothing, so no row was excluded.
//! Each row prints the post-call buffer. The `lpszName` field is printed as an
//! offset from `pData`, because absolute addresses vary between runs. These tests
//! therefore pass `data_addr` 0.

use super::*;

struct Row {
    name: &'static str,
    abi: Abi,
    flags: u32,
    data_len: Option<usize>,
    size_in: Option<u32>,
    num_in: Option<u32>,
    hr: u32,
    size_out: Option<u32>,
    num_out: Option<u32>,
    out: &'static str,
}

fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

const ROWS: &[Row] = &[
    Row { name: "exact_write", abi: Abi::Win64, flags: 0, data_len: Some(82), size_in: Some(82), num_in: Some(7), hr: 0x00000000, size_out: Some(82), num_out: Some(1), out: "50000000d42fe18db37cce48a7e89c47a22e8ac500000000300000000000000000000000000000000000000000fa00004d0053002d00500043004d0020003600340020006b006200690074002f0073000000" },
    Row { name: "larger_write", abi: Abi::Win64, flags: 0, data_len: Some(92), size_in: Some(92), num_in: Some(7), hr: 0x00000000, size_out: Some(82), num_out: Some(1), out: "50000000d42fe18db37cce48a7e89c47a22e8ac500000000300000000000000000000000000000000000000000fa00004d0053002d00500043004d0020003600340020006b006200690074002f0073000000aaaaaaaaaaaaaaaaaaaa" },
    Row { name: "one_short", abi: Abi::Win64, flags: 0, data_len: Some(81), size_in: Some(81), num_in: Some(7), hr: 0x8015001e, size_out: Some(82), num_out: Some(1), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "zero_size_buffer", abi: Abi::Win64, flags: 0, data_len: Some(0), size_in: Some(0), num_in: Some(7), hr: 0x8015001e, size_out: Some(82), num_out: Some(1), out: "" },
    Row { name: "null_data_exact", abi: Abi::Win64, flags: 0, data_len: None, size_in: Some(82), num_in: Some(7), hr: 0x80004003, size_out: Some(82), num_out: Some(1), out: "" },
    Row { name: "null_data_zero", abi: Abi::Win64, flags: 0, data_len: None, size_in: Some(0), num_in: Some(7), hr: 0x8015001e, size_out: Some(82), num_out: Some(1), out: "" },
    Row { name: "null_size_ptr", abi: Abi::Win64, flags: 0, data_len: Some(82), size_in: None, num_in: Some(7), hr: 0x80004003, size_out: None, num_out: Some(7), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "null_num_ptr", abi: Abi::Win64, flags: 0, data_len: Some(82), size_in: Some(82), num_in: None, hr: 0x80004003, size_out: Some(82), num_out: None, out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "flags_nonzero", abi: Abi::Win64, flags: 1, data_len: Some(82), size_in: Some(82), num_in: Some(7), hr: 0x80150078, size_out: Some(82), num_out: Some(7), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "flags_high_bit", abi: Abi::Win64, flags: 2147483648, data_len: Some(82), size_in: Some(82), num_in: Some(7), hr: 0x80150078, size_out: Some(82), num_out: Some(7), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "flags_null_data", abi: Abi::Win64, flags: 1, data_len: None, size_in: Some(82), num_in: Some(7), hr: 0x80150078, size_out: Some(82), num_out: Some(7), out: "" },
    Row { name: "size_max_null_data", abi: Abi::Win64, flags: 0, data_len: None, size_in: Some(4294967295), num_in: Some(7), hr: 0x80004003, size_out: Some(82), num_out: Some(1), out: "" },
    Row { name: "exact_write", abi: Abi::Win32, flags: 0, data_len: Some(70), size_in: Some(70), num_in: Some(7), hr: 0x00000000, size_out: Some(70), num_out: Some(1), out: "50000000d42fe18db37cce48a7e89c47a22e8ac524000000000000000000000000fa00004d0053002d00500043004d0020003600340020006b006200690074002f0073000000" },
    Row { name: "larger_write", abi: Abi::Win32, flags: 0, data_len: Some(80), size_in: Some(80), num_in: Some(7), hr: 0x00000000, size_out: Some(70), num_out: Some(1), out: "50000000d42fe18db37cce48a7e89c47a22e8ac524000000000000000000000000fa00004d0053002d00500043004d0020003600340020006b006200690074002f0073000000aaaaaaaaaaaaaaaaaaaa" },
    Row { name: "one_short", abi: Abi::Win32, flags: 0, data_len: Some(69), size_in: Some(69), num_in: Some(7), hr: 0x8015001e, size_out: Some(70), num_out: Some(1), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "zero_size_buffer", abi: Abi::Win32, flags: 0, data_len: Some(0), size_in: Some(0), num_in: Some(7), hr: 0x8015001e, size_out: Some(70), num_out: Some(1), out: "" },
    Row { name: "null_data_exact", abi: Abi::Win32, flags: 0, data_len: None, size_in: Some(70), num_in: Some(7), hr: 0x80004003, size_out: Some(70), num_out: Some(1), out: "" },
    Row { name: "null_data_zero", abi: Abi::Win32, flags: 0, data_len: None, size_in: Some(0), num_in: Some(7), hr: 0x8015001e, size_out: Some(70), num_out: Some(1), out: "" },
    Row { name: "null_size_ptr", abi: Abi::Win32, flags: 0, data_len: Some(70), size_in: None, num_in: Some(7), hr: 0x80004003, size_out: None, num_out: Some(7), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "null_num_ptr", abi: Abi::Win32, flags: 0, data_len: Some(70), size_in: Some(70), num_in: None, hr: 0x80004003, size_out: Some(70), num_out: None, out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "flags_nonzero", abi: Abi::Win32, flags: 1, data_len: Some(70), size_in: Some(70), num_in: Some(7), hr: 0x80150078, size_out: Some(70), num_out: Some(7), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "flags_high_bit", abi: Abi::Win32, flags: 2147483648, data_len: Some(70), size_in: Some(70), num_in: Some(7), hr: 0x80150078, size_out: Some(70), num_out: Some(7), out: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" },
    Row { name: "flags_null_data", abi: Abi::Win32, flags: 1, data_len: None, size_in: Some(70), num_in: Some(7), hr: 0x80150078, size_out: Some(70), num_out: Some(7), out: "" },
    Row { name: "size_max_null_data", abi: Abi::Win32, flags: 0, data_len: None, size_in: Some(4294967295), num_in: Some(7), hr: 0x80004003, size_out: Some(70), num_out: Some(1), out: "" },
];

#[test]
fn oracle_get_compression_types_rows() {
    for row in ROWS {
        let mut data = row.data_len.map(|n| vec![0xaau8; n]);
        let mut size = row.size_in;
        let mut num = row.num_in;
        let hr = get_compression_types(
            data.as_deref_mut(),
            size.as_mut(),
            num.as_mut(),
            row.flags,
            0,
            row.abi,
        );
        assert_eq!(hr as u32, row.hr, "{}: hr", row.name);
        assert_eq!(size, row.size_out, "{}: size", row.name);
        assert_eq!(num, row.num_out, "{}: num", row.name);
        assert_eq!(
            data.unwrap_or_default(),
            hex(row.out),
            "{}: buffer",
            row.name
        );
    }
}
