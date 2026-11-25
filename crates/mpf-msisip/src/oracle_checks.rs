//! Oracle-derived test vectors for the ported msisip logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `MsiSIPGetSignedDataMsg` and `MsiSIPIsMyTypeOfFile` from `dlls/msisip/main.c`
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/msisip/` and
//! running the harness. The harness prints 25 rows. Row 20
//! (`wrapped_header_overflow`) is flagged by AddressSanitizer: Wine copies the
//! 6-byte wrapped header into a 5-byte buffer. That row is excluded, so 24 rows remain.

use super::*;

/// Byte the harness fills the output buffer with.
const FILL: u8 = 0xee;
/// `*pdwEncodingType` before the call. Only a successful copy writes it.
const UNSET_ENCODING: u32 = 0x0bad_f00d;
/// `*pcbSignedDataMsg` before the call when `pbSignedDataMsg` is NULL.
const NULL_PCB: u32 = 0xcafe;
/// `*pdwEncodingType` the harness prints after a successful copy.
const PRINTED_ENCODING: u32 = 0x0001_0001;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

/// What the harness prints for one call of `MsiSIPGetSignedDataMsg`.
#[derive(Debug, PartialEq, Eq)]
struct Outputs {
    ret: bool,
    pcb: u32,
    err: u32,
    enc: u32,
    /// Hex of the buffer after the call, or `None` for a NULL buffer.
    buf: Option<String>,
}

fn from_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Runs the port the way the harness runs the C. `buf_len` is the buffer size
/// (the input `*pcbSignedDataMsg`), and `None` is a NULL buffer.
fn run(stream: &str, buf_len: Option<usize>) -> Outputs {
    let stream = from_hex(stream);
    let mut buf = vec![FILL; buf_len.unwrap_or(0)];
    let outcome = get_signed_data_msg(&stream, buf_len.map(|_| buf.as_mut_slice()));
    let pcb_in = buf_len.map_or(NULL_PCB, |n| n as u32);
    let buf = buf_len.map(|_| to_hex(&buf));
    match outcome {
        SignedDataMsg::Invalid => Outputs {
            ret: false,
            pcb: pcb_in,
            err: 0,
            enc: UNSET_ENCODING,
            buf,
        },
        SignedDataMsg::SizeQuery(size) => Outputs {
            ret: true,
            pcb: size,
            err: 0,
            enc: UNSET_ENCODING,
            buf,
        },
        SignedDataMsg::InsufficientBuffer(size) => Outputs {
            ret: false,
            pcb: size,
            err: ERROR_INSUFFICIENT_BUFFER,
            enc: UNSET_ENCODING,
            buf,
        },
        SignedDataMsg::Copied { size } => Outputs {
            ret: true,
            pcb: size,
            err: 0,
            enc: ENCODING_TYPE,
            buf,
        },
    }
}

/// A size query: `ret` TRUE and `*pcb` set. Nothing else is touched.
fn size_query(pcb: u32) -> Outputs {
    Outputs {
        ret: true,
        pcb,
        err: 0,
        enc: UNSET_ENCODING,
        buf: None,
    }
}

fn copied(pcb: u32, buf: &str) -> Outputs {
    Outputs {
        ret: true,
        pcb,
        err: 0,
        enc: PRINTED_ENCODING,
        buf: Some(buf.into()),
    }
}

fn failed(pcb: u32, buf: Option<&str>) -> Outputs {
    Outputs {
        ret: false,
        pcb,
        err: 0,
        enc: UNSET_ENCODING,
        buf: buf.map(Into::into),
    }
}

fn insufficient(pcb: u32, buf: &str) -> Outputs {
    Outputs {
        ret: false,
        pcb,
        err: ERROR_INSUFFICIENT_BUFFER,
        enc: UNSET_ENCODING,
        buf: Some(buf.into()),
    }
}

fn check(name: &str, stream: &str, buf_len: Option<usize>, expected: Outputs) {
    assert_eq!(run(stream, buf_len), expected, "oracle row {name}");
}

/// Formats a GUID the way the harness's `printf` does.
fn guid_text(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{}-{}",
        guid.data1,
        guid.data2,
        guid.data3,
        to_hex(&d[..2]),
        to_hex(&d[2..])
    )
}

/// A class GUID as the harness builds it: only `Data1` differs between rows.
fn clsid(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0,
        data3: 0,
        data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
    }
}

/// The subject text the harness prints, or `None` when the subject is untouched.
fn reported_subject(clsid: &Guid) -> Option<String> {
    msi_subject_for_clsid(clsid).map(|subject| guid_text(&subject))
}

#[test]
fn oracle_get_signed_data_msg_rows() {
    check("query_short_form", "3003010203ff", None, size_query(5));
    check("query_truncated_body", "30820100", None, size_query(0x104));
    check("query_len4", "308400000100", None, size_query(0x106));
    check("query_nonminimal_len", "30820005", None, size_query(9));
    check(
        "len_octets_over_4",
        "3085000000010000",
        None,
        failed(NULL_PCB, None),
    );
    check(
        "indefinite_length",
        "30800102",
        None,
        failed(NULL_PCB, None),
    );
    check("bad_tag", "3103010203", None, failed(NULL_PCB, None));
    check(
        "one_byte_stream",
        "30",
        Some(4),
        failed(4, Some("eeeeeeee")),
    );
    check(
        "long_length_truncated",
        "308201",
        Some(8),
        failed(8, Some("eeeeeeeeeeeeeeee")),
    );
    check(
        "copy_short_exact",
        "3003aabbcc",
        Some(5),
        copied(5, "3003aabbcc"),
    );
    check(
        "copy_long_roomy",
        "308102aabb",
        Some(16),
        copied(5, "308102aabbeeeeeeeeeeeeeeeeeeeeee"),
    );
    check(
        "copy_zero_length_long",
        "308100",
        Some(3),
        copied(3, "308100"),
    );
    check(
        "insufficient_short",
        "3003aabbcc",
        Some(4),
        insufficient(5, "eeeeeeee"),
    );
    check(
        "insufficient_long",
        "308102aabb",
        Some(3),
        insufficient(5, "eeeeee"),
    );
    check(
        "content_truncated",
        "3003aa",
        Some(8),
        failed(8, Some("3003aaeeeeeeeeee")),
    );
    check(
        "header_only_long",
        "308102",
        Some(8),
        failed(8, Some("308102eeeeeeeeee")),
    );
    check("empty_stream", "", Some(4), failed(4, Some("eeeeeeee")));
    check("wrapped_query", "3084ffffffff", None, size_query(5));
    check(
        "wrapped_insufficient",
        "3084ffffffff",
        Some(3),
        insufficient(5, "eeeeee"),
    );
    check(
        "wrapped_partial_body",
        "3084ffffffff0102030405060708090a",
        Some(16),
        failed(16, Some("3084ffffffff0102030405060708090a")),
    );
}

#[test]
fn oracle_msi_subject_rows() {
    let subject = "000c10f1-0000-0000-c000-000000000046";
    assert_eq!(guid_text(&MY_SUBJECT), subject);
    assert_eq!(
        reported_subject(&clsid(0x000c_1082)).as_deref(),
        Some(subject)
    );
    assert_eq!(
        reported_subject(&clsid(0x000c_1084)).as_deref(),
        Some(subject)
    );
    assert_eq!(
        reported_subject(&clsid(0x000c_1086)).as_deref(),
        Some(subject)
    );
    // near_miss_data1: the harness leaves its 0xee fill in the subject.
    assert_eq!(reported_subject(&clsid(0x000c_1085)), None);
}
