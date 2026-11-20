//! Twenty ASan/UBSan-clean rows from the verbatim Wine methods in
//! `target/canon/xolehlp/verbatim.inc`. Each oracle row calls both methods and
//! records the copied value's FNV-1a checksum and timeout.

use crate::{TransactionOptions, XactOptions, DESCRIPTION_CAPACITY, S_OK};

const TIMEOUTS: [u32; 20] = [
    0,
    1,
    2,
    3,
    7,
    15,
    255,
    256,
    65_535,
    65_536,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_fffe,
    0xffff_ffff,
    0x1234_5678,
    0x8765_4321,
    0xdead_beef,
    0x0102_0304,
    0xabcd_ef01,
    0x4000_0000,
];

/// `(timeout, FNV-1a of the copied XACTOPT bytes)` from the C oracle.
const ORACLE_ROWS: [(u32, u32); 20] = [
    (0, 0x96c3_a0fd),
    (1, 0xdb9f_e41c),
    (2, 0xb911_f4b7),
    (3, 0xe5ec_19fe),
    (7, 0x8e72_9fda),
    (15, 0x180a_1a7a),
    (255, 0xb613_fa02),
    (256, 0x5a18_8842),
    (65_535, 0x3a3b_dd9b),
    (65_536, 0x2ae6_1924),
    (0x7fff_ffff, 0x1a27_6fd9),
    (0x8000_0000, 0x6589_44d5),
    (0xffff_fffe, 0xfb0d_3f90),
    (0xffff_ffff, 0xa1b4_8ea9),
    (0x1234_5678, 0xd751_6c1d),
    (0x8765_4321, 0x927e_90ad),
    (0xdead_beef, 0xeb5d_ee0b),
    (0x0102_0304, 0x4053_badd),
    (0xabcd_ef01, 0x0438_22fd),
    (0x4000_0000, 0xac63_2e05),
];

fn checksum(options: &XactOptions) -> u32 {
    options
        .timeout
        .to_le_bytes()
        .into_iter()
        .chain(options.description)
        .fold(2_166_136_261, |hash, byte| {
            (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
        })
}

#[test]
fn oracle_rows_match_wine_transaction_options_copy() {
    for (row, &(expected_timeout, expected_checksum)) in ORACLE_ROWS.iter().enumerate() {
        let input = XactOptions {
            timeout: TIMEOUTS[row],
            description: core::array::from_fn(|index| (index * 13 + row * 37) as u8),
        };
        let mut state = TransactionOptions::new(XactOptions {
            timeout: !TIMEOUTS[row],
            description: [0xa5; DESCRIPTION_CAPACITY],
        });
        assert_eq!(state.set_options(Some(&input)), S_OK, "set row {row}");

        let mut output = XactOptions {
            timeout: 0,
            description: [0x5a; DESCRIPTION_CAPACITY],
        };
        assert_eq!(state.get_options(Some(&mut output)), S_OK, "get row {row}");
        assert_eq!(output.timeout, expected_timeout, "timeout row {row}");
        assert_eq!(checksum(&output), expected_checksum, "checksum row {row}");
        assert_eq!(output, input, "full value row {row}");
    }
}
