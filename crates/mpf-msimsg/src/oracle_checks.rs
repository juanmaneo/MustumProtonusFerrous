//! Oracle-derived test vectors for the msimsg reboot prompt resource.
//!
//! `dlls/msimsg` has no C functions, so the rows describe the resource rather than return
//! values. Wine's `wrc` 11.19 (the system build; the reference tree is 11.22) compiled the
//! verbatim `dlls/msimsg/msimsg.rc` to a `.res` under `target/canon/msimsg/`. The harness
//! `harness.c` (`gcc -fsanitize=address,undefined -g`) parsed that `.res` and printed the 15
//! rows below. No row was excluded: ASan and UBSan reported nothing.

use super::*;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn oracle_reboot_prompt_rows() {
    let entry = reboot_prompt_resource();

    // row 01 type_id = 10
    assert_eq!(entry.type_id, 10);
    assert_eq!(RT_RCDATA, 10);

    // row 02 name_id = 1703
    assert_eq!(entry.name_id, 1703);
    assert_eq!(REBOOT_PROMPT_ID, 1703);

    // row 03 lang_id = 0x0409
    assert_eq!(entry.lang_id, 0x0409);

    // row 04 LANG_ENGLISH = 0x09
    assert_eq!(LANG_ENGLISH, 0x09);

    // row 05 SUBLANG_DEFAULT = 0x01
    assert_eq!(SUBLANG_DEFAULT, 0x01);

    // row 06 MAKELANGID(LANG_ENGLISH, SUBLANG_DEFAULT) = 0x0409
    assert_eq!(make_lang_id(LANG_ENGLISH, SUBLANG_DEFAULT), 0x0409);

    // row 07 data_size = 134
    assert_eq!(entry.data.len(), 134);

    // row 08 text_len = 132
    assert_eq!(REBOOT_PROMPT.len(), 132);

    // row 09 data[text_len] = 0x00
    assert_eq!(entry.data[132], 0x00);

    // row 10 data[text_len+1] = 0x00
    assert_eq!(entry.data[133], 0x00);

    // row 11 data[0..8] = 596f75206d757374
    assert_eq!(hex(&entry.data[..8]), "596f75206d757374");

    // row 12 placeholder_offset("[2]") = 67
    assert_eq!(REBOOT_PROMPT.find("[2]"), Some(67));

    // row 13 data[text_len-1] = 0x2e
    assert_eq!(entry.data[131], 0x2e);

    // row 14 text
    assert_eq!(
        REBOOT_PROMPT,
        "You must simulate a Windows reboot to complete the installation of [2]. Click Yes to do this now or No if you want to restart later."
    );

    // row 15 data_hex
    assert_eq!(
        hex(&entry.data),
        "596f75206d7573742073696d756c61746520612057696e646f7773207265626f6f7420746f20636f6d706c6574652074686520696e7374616c6c6174696f6e206f66205b325d2e20436c69636b2059657320746f20646f2074686973206e6f77206f72204e6f20696620796f752077616e7420746f2072657374617274206c617465722e0000"
    );
}
