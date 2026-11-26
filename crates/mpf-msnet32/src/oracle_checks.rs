//! Oracle-derived test vectors for the ported msnet32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `MSNET32_57`
//! (`dlls/msnet32/msnet_main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/msnet32/`, and running the
//! harness. The harness printed 99 rows: 95 `@ stub` rows, one per export, and 4 `MSNET32_57` rows
//! with different argument vectors. ASan and UBSan reported nothing, so no row is excluded.

use super::*;
use core::ffi::c_void;
use std::ptr::null_mut;

/// Each `@ stub` export with the record the harness captured: the helper, then code, flags,
/// number of parameters, module and function.
type StubRow = (
    fn() -> StubException,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

const STUB_ROWS: &[StubRow] = &[
    (ordinal_1, 0x8000_0100, 0x01, 2, "msnet32.dll", "1"),
    (ordinal_2, 0x8000_0100, 0x01, 2, "msnet32.dll", "2"),
    (ordinal_3, 0x8000_0100, 0x01, 2, "msnet32.dll", "3"),
    (ordinal_4, 0x8000_0100, 0x01, 2, "msnet32.dll", "4"),
    (ordinal_5, 0x8000_0100, 0x01, 2, "msnet32.dll", "5"),
    (ordinal_6, 0x8000_0100, 0x01, 2, "msnet32.dll", "6"),
    (ordinal_7, 0x8000_0100, 0x01, 2, "msnet32.dll", "7"),
    (ordinal_8, 0x8000_0100, 0x01, 2, "msnet32.dll", "8"),
    (ordinal_9, 0x8000_0100, 0x01, 2, "msnet32.dll", "9"),
    (ordinal_10, 0x8000_0100, 0x01, 2, "msnet32.dll", "10"),
    (ordinal_11, 0x8000_0100, 0x01, 2, "msnet32.dll", "11"),
    (ordinal_12, 0x8000_0100, 0x01, 2, "msnet32.dll", "12"),
    (ordinal_13, 0x8000_0100, 0x01, 2, "msnet32.dll", "13"),
    (ordinal_14, 0x8000_0100, 0x01, 2, "msnet32.dll", "14"),
    (ordinal_15, 0x8000_0100, 0x01, 2, "msnet32.dll", "15"),
    (ordinal_16, 0x8000_0100, 0x01, 2, "msnet32.dll", "16"),
    (ordinal_17, 0x8000_0100, 0x01, 2, "msnet32.dll", "17"),
    (ordinal_18, 0x8000_0100, 0x01, 2, "msnet32.dll", "18"),
    (ordinal_19, 0x8000_0100, 0x01, 2, "msnet32.dll", "19"),
    (ordinal_20, 0x8000_0100, 0x01, 2, "msnet32.dll", "20"),
    (ordinal_21, 0x8000_0100, 0x01, 2, "msnet32.dll", "21"),
    (ordinal_22, 0x8000_0100, 0x01, 2, "msnet32.dll", "22"),
    (ordinal_23, 0x8000_0100, 0x01, 2, "msnet32.dll", "23"),
    (ordinal_24, 0x8000_0100, 0x01, 2, "msnet32.dll", "24"),
    (ordinal_25, 0x8000_0100, 0x01, 2, "msnet32.dll", "25"),
    (ordinal_26, 0x8000_0100, 0x01, 2, "msnet32.dll", "26"),
    (ordinal_27, 0x8000_0100, 0x01, 2, "msnet32.dll", "27"),
    (ordinal_28, 0x8000_0100, 0x01, 2, "msnet32.dll", "28"),
    (ordinal_29, 0x8000_0100, 0x01, 2, "msnet32.dll", "29"),
    (ordinal_30, 0x8000_0100, 0x01, 2, "msnet32.dll", "30"),
    (ordinal_31, 0x8000_0100, 0x01, 2, "msnet32.dll", "31"),
    (ordinal_32, 0x8000_0100, 0x01, 2, "msnet32.dll", "32"),
    (ordinal_33, 0x8000_0100, 0x01, 2, "msnet32.dll", "33"),
    (ordinal_34, 0x8000_0100, 0x01, 2, "msnet32.dll", "34"),
    (ordinal_35, 0x8000_0100, 0x01, 2, "msnet32.dll", "35"),
    (ordinal_36, 0x8000_0100, 0x01, 2, "msnet32.dll", "36"),
    (ordinal_37, 0x8000_0100, 0x01, 2, "msnet32.dll", "37"),
    (ordinal_38, 0x8000_0100, 0x01, 2, "msnet32.dll", "38"),
    (ordinal_39, 0x8000_0100, 0x01, 2, "msnet32.dll", "39"),
    (ordinal_40, 0x8000_0100, 0x01, 2, "msnet32.dll", "40"),
    (ordinal_41, 0x8000_0100, 0x01, 2, "msnet32.dll", "41"),
    (ordinal_42, 0x8000_0100, 0x01, 2, "msnet32.dll", "42"),
    (ordinal_43, 0x8000_0100, 0x01, 2, "msnet32.dll", "43"),
    (ordinal_44, 0x8000_0100, 0x01, 2, "msnet32.dll", "44"),
    (ordinal_45, 0x8000_0100, 0x01, 2, "msnet32.dll", "45"),
    (ordinal_46, 0x8000_0100, 0x01, 2, "msnet32.dll", "46"),
    (ordinal_47, 0x8000_0100, 0x01, 2, "msnet32.dll", "47"),
    (ordinal_48, 0x8000_0100, 0x01, 2, "msnet32.dll", "48"),
    (ordinal_49, 0x8000_0100, 0x01, 2, "msnet32.dll", "49"),
    (ordinal_50, 0x8000_0100, 0x01, 2, "msnet32.dll", "50"),
    (ordinal_51, 0x8000_0100, 0x01, 2, "msnet32.dll", "51"),
    (ordinal_52, 0x8000_0100, 0x01, 2, "msnet32.dll", "52"),
    (ordinal_53, 0x8000_0100, 0x01, 2, "msnet32.dll", "53"),
    (ordinal_54, 0x8000_0100, 0x01, 2, "msnet32.dll", "54"),
    (ordinal_55, 0x8000_0100, 0x01, 2, "msnet32.dll", "55"),
    (ordinal_56, 0x8000_0100, 0x01, 2, "msnet32.dll", "56"),
    (ordinal_58, 0x8000_0100, 0x01, 2, "msnet32.dll", "58"),
    (ordinal_59, 0x8000_0100, 0x01, 2, "msnet32.dll", "59"),
    (ordinal_60, 0x8000_0100, 0x01, 2, "msnet32.dll", "60"),
    (ordinal_61, 0x8000_0100, 0x01, 2, "msnet32.dll", "61"),
    (ordinal_62, 0x8000_0100, 0x01, 2, "msnet32.dll", "62"),
    (ordinal_63, 0x8000_0100, 0x01, 2, "msnet32.dll", "63"),
    (ordinal_64, 0x8000_0100, 0x01, 2, "msnet32.dll", "64"),
    (ordinal_65, 0x8000_0100, 0x01, 2, "msnet32.dll", "65"),
    (ordinal_66, 0x8000_0100, 0x01, 2, "msnet32.dll", "66"),
    (ordinal_67, 0x8000_0100, 0x01, 2, "msnet32.dll", "67"),
    (ordinal_68, 0x8000_0100, 0x01, 2, "msnet32.dll", "68"),
    (ordinal_69, 0x8000_0100, 0x01, 2, "msnet32.dll", "69"),
    (ordinal_70, 0x8000_0100, 0x01, 2, "msnet32.dll", "70"),
    (ordinal_71, 0x8000_0100, 0x01, 2, "msnet32.dll", "71"),
    (ordinal_72, 0x8000_0100, 0x01, 2, "msnet32.dll", "72"),
    (ordinal_73, 0x8000_0100, 0x01, 2, "msnet32.dll", "73"),
    (ordinal_74, 0x8000_0100, 0x01, 2, "msnet32.dll", "74"),
    (ordinal_75, 0x8000_0100, 0x01, 2, "msnet32.dll", "75"),
    (ordinal_76, 0x8000_0100, 0x01, 2, "msnet32.dll", "76"),
    (ordinal_77, 0x8000_0100, 0x01, 2, "msnet32.dll", "77"),
    (ordinal_78, 0x8000_0100, 0x01, 2, "msnet32.dll", "78"),
    (ordinal_79, 0x8000_0100, 0x01, 2, "msnet32.dll", "79"),
    (ordinal_80, 0x8000_0100, 0x01, 2, "msnet32.dll", "80"),
    (ordinal_81, 0x8000_0100, 0x01, 2, "msnet32.dll", "81"),
    (ordinal_82, 0x8000_0100, 0x01, 2, "msnet32.dll", "82"),
    (ordinal_83, 0x8000_0100, 0x01, 2, "msnet32.dll", "83"),
    (ordinal_84, 0x8000_0100, 0x01, 2, "msnet32.dll", "84"),
    (ordinal_85, 0x8000_0100, 0x01, 2, "msnet32.dll", "85"),
    (ordinal_86, 0x8000_0100, 0x01, 2, "msnet32.dll", "86"),
    (ordinal_87, 0x8000_0100, 0x01, 2, "msnet32.dll", "87"),
    (ordinal_88, 0x8000_0100, 0x01, 2, "msnet32.dll", "88"),
    (ordinal_89, 0x8000_0100, 0x01, 2, "msnet32.dll", "89"),
    (ordinal_90, 0x8000_0100, 0x01, 2, "msnet32.dll", "90"),
    (ordinal_91, 0x8000_0100, 0x01, 2, "msnet32.dll", "91"),
    (ordinal_92, 0x8000_0100, 0x01, 2, "msnet32.dll", "92"),
    (ordinal_93, 0x8000_0100, 0x01, 2, "msnet32.dll", "93"),
    (ordinal_94, 0x8000_0100, 0x01, 2, "msnet32.dll", "94"),
    (ordinal_95, 0x8000_0100, 0x01, 2, "msnet32.dll", "95"),
    (ordinal_96, 0x8000_0100, 0x01, 2, "msnet32.dll", "96"),
];

#[test]
fn oracle_stub_export_rows() {
    for &(export, code, flags, number_parameters, module, function) in STUB_ROWS {
        assert_eq!(
            export(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}

#[test]
fn oracle_msnet32_57_rows() {
    // Rows 57 to 60 of the harness output, as (a1, a2, a3, a4, a5). Every row returns -1.
    let mut slot = 0u8;
    let ptr = (&mut slot as *mut u8).cast::<c_void>();
    assert_eq!(msnet32_57(0, 0, null_mut(), 0, null_mut()), -1);
    assert_eq!(msnet32_57(1, 2, ptr, 3, ptr), -1);
    assert_eq!(msnet32_57(i32::MIN, i32::MAX, ptr, -1, null_mut()), -1);
    assert_eq!(msnet32_57(4660, -4660, null_mut(), 65535, ptr), -1);
}
