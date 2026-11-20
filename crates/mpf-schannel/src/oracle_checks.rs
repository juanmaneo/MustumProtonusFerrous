//! Rows replayed from target/canon/schannel/oracle.txt, produced by compiling the verbatim
//! SpUserModeInitialize, SpLsaModeInitialize, SpGetInfoUnified and SpGetInfoSChannel bodies
//! from Wine's usermode.c and lsamode.c with ASan/UBSan. The run produced no sanitizer
//! reports, so no rows were excluded.

use super::{
    sp_get_info_schannel, sp_get_info_unified, sp_lsa_mode_initialize, sp_user_mode_initialize,
    PackageInfo, Status, SCHANNEL_CAPABILITIES,
};

fn info(name: &'static str, comment: &'static str) -> PackageInfo {
    PackageInfo {
        f_capabilities: SCHANNEL_CAPABILITIES,
        w_version: 1,
        w_rpc_id: 14,
        cb_max_token: 0x4000,
        name,
        comment,
    }
}

#[test]
fn oracle_user_mode_rows() {
    let rows: [(&str, u32, (Status, u32, u32)); 12] = [
        ("0 zero", 0x0000_0000, (Status::InvalidParameter, 0, 0)),
        ("1 one", 0x0000_0001, (Status::InvalidParameter, 0, 0)),
        (
            "2 interface version",
            0x0001_0000,
            (Status::Success, 0x1_0000, 2),
        ),
        ("3 version 2", 0x0002_0000, (Status::InvalidParameter, 0, 0)),
        ("4 version 8", 0x0080_0000, (Status::InvalidParameter, 0, 0)),
        (
            "5 low 16 bits",
            0x0000_FFFF,
            (Status::InvalidParameter, 0, 0),
        ),
        (
            "6 interface plus one",
            0x0001_0001,
            (Status::InvalidParameter, 0, 0),
        ),
        (
            "7 interface plus 0x1ffff",
            0x0001_FFFF,
            (Status::InvalidParameter, 0, 0),
        ),
        (
            "8 interface plus 0x100",
            0x0001_0100,
            (Status::InvalidParameter, 0, 0),
        ),
        (
            "9 signed max",
            0x7FFF_FFFF,
            (Status::InvalidParameter, 0, 0),
        ),
        ("10 high bit", 0x8000_0000, (Status::InvalidParameter, 0, 0)),
        ("11 all ones", 0xFFFF_FFFF, (Status::InvalidParameter, 0, 0)),
    ];

    assert_eq!(rows.len(), 12);
    for (name, input, expected) in rows {
        assert_eq!(
            sp_user_mode_initialize(input),
            expected,
            "oracle row {name}"
        );
    }
}

#[test]
fn oracle_lsa_mode_rows() {
    let expected = (Status::Success, 0x0080_0000, 2);
    for (name, input) in [
        ("12 lsa zero", 0x0000_0000u32),
        ("13 lsa interface version", 0x0001_0000),
        ("14 lsa all ones", 0xFFFF_FFFF),
    ] {
        assert_eq!(
            sp_lsa_mode_initialize(),
            expected,
            "oracle row {name} input {input:#x}"
        );
    }
}

#[test]
fn oracle_info_rows() {
    assert_eq!(
        sp_get_info_unified(),
        (
            Status::Success,
            info(
                "Microsoft Unified Security Protocol Provider",
                "Microsoft Unified Security Protocol Provider",
            ),
        ),
        "oracle row 15 info-unified"
    );
    assert_eq!(
        sp_get_info_schannel(),
        (
            Status::Success,
            info("Schannel", "Schannel Security Package"),
        ),
        "oracle row 16 info-schannel"
    );
    assert_eq!(SCHANNEL_CAPABILITIES, 0x0001_07B3, "oracle capability mask");
}
