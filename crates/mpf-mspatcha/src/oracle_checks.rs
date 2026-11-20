//! PA19 VLI outputs from the verbatim Wine readers in the ASan/UBSan oracle.

use crate::{read_svli, read_uvli, VliError};

type UnsignedRow<'a> = (&'a [u8], Result<(u64, usize), VliError>);
type SignedRow<'a> = (&'a [u8], Result<(i64, usize), VliError>);

#[test]
fn unsigned_oracle_rows() {
    let rows: &[UnsignedRow<'_>] = &[
        (&[], Err(VliError::Empty)),
        (&[0x80], Ok((0, 1))),
        (&[0x81], Ok((1, 1))),
        (&[0x7f, 0x80], Ok((127, 2))),
        (&[0x00, 0x81], Ok((128, 2))),
        (&[0x7f, 0xff, 0x80], Ok((16_383, 2))),
        (&[0x00, 0x00, 0x81], Ok((16_384, 3))),
        (&[0xff, 0x00], Ok((127, 1))),
        (&[0x7f, 0x7f, 0x7f, 0x80], Ok((2_097_151, 4))),
        (
            &[0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0xff],
            Ok((i64::MAX as u64, 9)),
        ),
        (&[0x00], Err(VliError::Unterminated)),
        (
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            Err(VliError::Unterminated),
        ),
    ];

    for (bytes, expected) in rows {
        assert_eq!(read_uvli(bytes), *expected, "input={bytes:02x?}");
    }
}

#[test]
fn signed_oracle_rows() {
    let rows: &[SignedRow<'_>] = &[
        (&[], Err(VliError::Empty)),
        (&[0x80], Ok((0, 1))),
        (&[0xc0], Ok((0, 1))),
        (&[0x01, 0x80], Ok((1, 2))),
        (&[0x41, 0x80], Ok((-1, 2))),
        (&[0x7f, 0x80], Ok((-63, 2))),
        (&[0x00, 0x81], Ok((64, 2))),
        (&[0x40, 0x81], Ok((-64, 2))),
        (&[0x7f, 0xff, 0x80], Ok((-8_191, 2))),
        (
            &[0x3f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0xff],
            Ok((i64::MAX / 2, 9)),
        ),
        (
            &[0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0xff],
            Ok((-(i64::MAX / 2), 9)),
        ),
        (&[0x00], Err(VliError::Unterminated)),
    ];

    for (bytes, expected) in rows {
        assert_eq!(read_svli(bytes), *expected, "input={bytes:02x?}");
    }
}
