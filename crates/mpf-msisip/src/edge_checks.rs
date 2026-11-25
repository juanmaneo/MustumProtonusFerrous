//! Boundary and branch tests for the ported msisip logic.
//!
//! Each test says whether its expectation is oracle-derived. Expectations that
//! are not are read from the Wine control flow, or are the port's documented
//! deviation where Wine writes past the buffer.

use super::*;

const FILL: u8 = 0xee;

#[test]
fn der_length_boundaries() {
    // Not oracle-derived: the short form tops out at 0x7f, and 0x80 needs one length octet.
    assert_eq!(
        der::parse_der_header(&[0x30, 0x00]),
        Some(der::DerHeader {
            header_len: 2,
            content_len: 0
        })
    );
    assert_eq!(
        der::parse_der_header(&[0x30, 0x7f]),
        Some(der::DerHeader {
            header_len: 2,
            content_len: 0x7f
        })
    );
    assert_eq!(
        der::parse_der_header(&[0x30, 0x81, 0x80]),
        Some(der::DerHeader {
            header_len: 3,
            content_len: 0x80
        })
    );
    assert_eq!(
        der::parse_der_header(&[0x30, 0x81, 0xff]),
        Some(der::DerHeader {
            header_len: 3,
            content_len: 0xff
        })
    );
    assert_eq!(
        der::parse_der_header(&[0x30, 0x82, 0x01, 0x00]),
        Some(der::DerHeader {
            header_len: 4,
            content_len: 0x100
        })
    );
    assert_eq!(
        der::parse_der_header(&[0x30, 0x84, 0xff, 0xff, 0xff, 0xff]),
        Some(der::DerHeader {
            header_len: 6,
            content_len: u32::MAX
        })
    );
}

#[test]
fn der_rejects_malformed_headers() {
    // Not oracle-derived: one input per rejection branch of `parse_der_header`.
    // Empty and one-byte streams fail Wine's first two-byte read.
    assert_eq!(der::parse_der_header(&[]), None);
    assert_eq!(der::parse_der_header(&[0x30]), None);
    assert_eq!(der::parse_der_header(&[0x31, 0x00]), None);
    assert_eq!(der::parse_der_header(&[0x30, 0x80]), None);
    // Five length octets exceed the DWORD.
    assert_eq!(der::parse_der_header(&[0x30, 0x85, 0, 0, 0, 1, 0]), None);
    assert_eq!(der::parse_der_header(&[0x30, 0xff, 0, 0, 0, 0, 0]), None);
    // Two length octets announced, one present.
    assert_eq!(der::parse_der_header(&[0x30, 0x82, 0x01]), None);
}

#[test]
fn message_len_wraps_like_dword() {
    // Not oracle-derived: DWORD addition of the header length and the content length.
    let header = der::DerHeader {
        header_len: 2,
        content_len: 0x10,
    };
    assert_eq!(header.message_len(), 0x12);
    let header = der::DerHeader {
        header_len: 6,
        content_len: 0xffff_fffe,
    };
    assert_eq!(header.message_len(), 4);
}

#[test]
fn exact_size_buffer_copies_and_one_short_writes_nothing() {
    // Not oracle-derived: the oracle uses buffers 16 and 3 for this stream, not the exact size 5 and 4.
    let stream = [0x30, 0x81, 0x02, 0xaa, 0xbb];
    let mut exact = [FILL; 5];
    assert_eq!(
        get_signed_data_msg(&stream, Some(&mut exact)),
        SignedDataMsg::Copied { size: 5 }
    );
    assert_eq!(exact, [0x30, 0x81, 0x02, 0xaa, 0xbb]);

    let mut short = [FILL; 4];
    assert_eq!(
        get_signed_data_msg(&stream, Some(&mut short)),
        SignedDataMsg::InsufficientBuffer(5)
    );
    assert_eq!(short, [FILL; 4]);
}

#[test]
fn zero_length_message() {
    // Not oracle-derived: the oracle has no empty-content SEQUENCE.
    assert_eq!(
        get_signed_data_msg(&[0x30, 0x00], None),
        SignedDataMsg::SizeQuery(2)
    );
    let mut buf = [FILL; 2];
    assert_eq!(
        get_signed_data_msg(&[0x30, 0x00], Some(&mut buf)),
        SignedDataMsg::Copied { size: 2 }
    );
    assert_eq!(buf, [0x30, 0x00]);
}

#[test]
fn trailing_stream_bytes_are_not_copied() {
    // Not oracle-derived: Wine reads exactly dataBytes, so the 0xdd after the content is never read.
    let mut buf = [FILL; 6];
    assert_eq!(
        get_signed_data_msg(&[0x30, 0x03, 0xaa, 0xbb, 0xcc, 0xdd], Some(&mut buf)),
        SignedDataMsg::Copied { size: 5 }
    );
    assert_eq!(buf, [0x30, 0x03, 0xaa, 0xbb, 0xcc, FILL]);
}

#[test]
fn wrapped_header_only_buffer_writes_header() {
    // Not oracle-derived: a 6-byte buffer holds the wrapped header exactly. Wine reads no body
    // from the empty stream, so the call returns FALSE with the header written.
    let mut buf = [FILL; 6];
    assert_eq!(
        get_signed_data_msg(&[0x30, 0x84, 0xff, 0xff, 0xff, 0xff], Some(&mut buf)),
        SignedDataMsg::Invalid
    );
    assert_eq!(buf, [0x30, 0x84, 0xff, 0xff, 0xff, 0xff]);
}

#[test]
fn wrapped_buffer_shorter_than_header_is_insufficient() {
    // Not oracle-derived. Deviation: oracle row 20 (wrapped header into a 5-byte buffer) is
    // flagged by ASan, because Wine writes the 6-byte header past the end. The port reports
    // insufficient space and writes nothing.
    let mut buf = [FILL; 5];
    assert_eq!(
        get_signed_data_msg(&[0x30, 0x84, 0xff, 0xff, 0xff, 0xff], Some(&mut buf)),
        SignedDataMsg::InsufficientBuffer(5)
    );
    assert_eq!(buf, [FILL; 5]);
}

#[test]
fn wrapped_room_shorter_than_body_copies_room_only() {
    // Not oracle-derived. Deviation: Wine's read of the wrapped content length would write the
    // full 10-byte body into the 2 bytes left after the header. The port copies the 2 bytes that
    // fit and returns FALSE.
    let mut buf = [FILL; 8];
    let stream = [
        0x30, 0x84, 0xff, 0xff, 0xff, 0xff, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];
    assert_eq!(
        get_signed_data_msg(&stream, Some(&mut buf)),
        SignedDataMsg::Invalid
    );
    assert_eq!(buf, [0x30, 0x84, 0xff, 0xff, 0xff, 0xff, 1, 2]);
}

#[test]
fn msi_near_miss_clsids_are_rejected() {
    // Not oracle-derived beyond row 24 (data1 0x000c1085). Each field of the three MSI
    // classes is changed on its own, so every field is compared.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(msi_subject_for_clsid(&zero), None);
    assert_eq!(msi_subject_for_clsid(&MY_SUBJECT), None);
    for clsid in [CLSID_MSI_TRANSFORM, CLSID_MSI_DATABASE, CLSID_MSI_PATCH] {
        assert_eq!(
            msi_subject_for_clsid(&Guid {
                data1: clsid.data1 + 1,
                ..clsid
            }),
            None
        );
        assert_eq!(msi_subject_for_clsid(&Guid { data2: 1, ..clsid }), None);
        assert_eq!(msi_subject_for_clsid(&Guid { data3: 1, ..clsid }), None);
        let mut data4 = clsid.data4;
        data4[7] = 0x47;
        assert_eq!(msi_subject_for_clsid(&Guid { data4, ..clsid }), None);
    }
}
