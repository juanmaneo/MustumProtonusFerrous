//! The exports of `dlls/msnet32/msnet32.spec`: `MSNET32_57`, the one explicit C function, and the
//! 95 `@ stub` exports, one helper each.
//!
//! A `@ stub` export has no name, so winebuild passes its ordinal as the function argument. The
//! helper's `function` field is that ordinal written in decimal.

use crate::stub::{wine_spec_unimplemented_stub, StubException};
use core::ffi::c_void;

/// The module name `winebuild` passes to stubs, from `MODULE = msnet32.dll` in `dlls/msnet32/Makefile.in`.
const MSNET32_DLL_NAME: &str = "msnet32.dll";

/// `MSNET32_57` from `dlls/msnet32/msnet_main.c`, ordinal 57. The C body traces its arguments and
/// returns `-1` (`FAILURE`), so the port ignores them too.
pub fn msnet32_57(_a1: i32, _a2: i32, _a3: *mut c_void, _a4: i32, _a5: *mut c_void) -> i32 {
    -1
}

/// Ordinal 1, `@ stub` in `msnet32.spec`.
pub fn ordinal_1() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "1")
}

/// Ordinal 2, `@ stub` in `msnet32.spec`.
pub fn ordinal_2() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "2")
}

/// Ordinal 3, `@ stub` in `msnet32.spec`.
pub fn ordinal_3() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "3")
}

/// Ordinal 4, `@ stub` in `msnet32.spec`.
pub fn ordinal_4() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "4")
}

/// Ordinal 5, `@ stub` in `msnet32.spec`.
pub fn ordinal_5() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "5")
}

/// Ordinal 6, `@ stub` in `msnet32.spec`.
pub fn ordinal_6() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "6")
}

/// Ordinal 7, `@ stub` in `msnet32.spec`.
pub fn ordinal_7() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "7")
}

/// Ordinal 8, `@ stub` in `msnet32.spec`.
pub fn ordinal_8() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "8")
}

/// Ordinal 9, `@ stub` in `msnet32.spec`.
pub fn ordinal_9() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "9")
}

/// Ordinal 10, `@ stub` in `msnet32.spec`.
pub fn ordinal_10() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "10")
}

/// Ordinal 11, `@ stub` in `msnet32.spec`.
pub fn ordinal_11() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "11")
}

/// Ordinal 12, `@ stub` in `msnet32.spec`.
pub fn ordinal_12() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "12")
}

/// Ordinal 13, `@ stub` in `msnet32.spec`.
pub fn ordinal_13() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "13")
}

/// Ordinal 14, `@ stub` in `msnet32.spec`.
pub fn ordinal_14() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "14")
}

/// Ordinal 15, `@ stub` in `msnet32.spec`.
pub fn ordinal_15() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "15")
}

/// Ordinal 16, `@ stub` in `msnet32.spec`.
pub fn ordinal_16() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "16")
}

/// Ordinal 17, `@ stub` in `msnet32.spec`.
pub fn ordinal_17() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "17")
}

/// Ordinal 18, `@ stub` in `msnet32.spec`.
pub fn ordinal_18() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "18")
}

/// Ordinal 19, `@ stub` in `msnet32.spec`.
pub fn ordinal_19() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "19")
}

/// Ordinal 20, `@ stub` in `msnet32.spec`.
pub fn ordinal_20() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "20")
}

/// Ordinal 21, `@ stub` in `msnet32.spec`.
pub fn ordinal_21() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "21")
}

/// Ordinal 22, `@ stub` in `msnet32.spec`.
pub fn ordinal_22() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "22")
}

/// Ordinal 23, `@ stub` in `msnet32.spec`.
pub fn ordinal_23() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "23")
}

/// Ordinal 24, `@ stub` in `msnet32.spec`.
pub fn ordinal_24() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "24")
}

/// Ordinal 25, `@ stub` in `msnet32.spec`.
pub fn ordinal_25() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "25")
}

/// Ordinal 26, `@ stub` in `msnet32.spec`.
pub fn ordinal_26() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "26")
}

/// Ordinal 27, `@ stub` in `msnet32.spec`.
pub fn ordinal_27() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "27")
}

/// Ordinal 28, `@ stub` in `msnet32.spec`.
pub fn ordinal_28() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "28")
}

/// Ordinal 29, `@ stub` in `msnet32.spec`.
pub fn ordinal_29() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "29")
}

/// Ordinal 30, `@ stub` in `msnet32.spec`.
pub fn ordinal_30() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "30")
}

/// Ordinal 31, `@ stub` in `msnet32.spec`.
pub fn ordinal_31() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "31")
}

/// Ordinal 32, `@ stub` in `msnet32.spec`.
pub fn ordinal_32() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "32")
}

/// Ordinal 33, `@ stub` in `msnet32.spec`.
pub fn ordinal_33() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "33")
}

/// Ordinal 34, `@ stub` in `msnet32.spec`.
pub fn ordinal_34() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "34")
}

/// Ordinal 35, `@ stub` in `msnet32.spec`.
pub fn ordinal_35() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "35")
}

/// Ordinal 36, `@ stub` in `msnet32.spec`.
pub fn ordinal_36() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "36")
}

/// Ordinal 37, `@ stub` in `msnet32.spec`.
pub fn ordinal_37() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "37")
}

/// Ordinal 38, `@ stub` in `msnet32.spec`.
pub fn ordinal_38() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "38")
}

/// Ordinal 39, `@ stub` in `msnet32.spec`.
pub fn ordinal_39() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "39")
}

/// Ordinal 40, `@ stub` in `msnet32.spec`.
pub fn ordinal_40() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "40")
}

/// Ordinal 41, `@ stub` in `msnet32.spec`.
pub fn ordinal_41() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "41")
}

/// Ordinal 42, `@ stub` in `msnet32.spec`.
pub fn ordinal_42() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "42")
}

/// Ordinal 43, `@ stub` in `msnet32.spec`.
pub fn ordinal_43() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "43")
}

/// Ordinal 44, `@ stub` in `msnet32.spec`.
pub fn ordinal_44() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "44")
}

/// Ordinal 45, `@ stub` in `msnet32.spec`.
pub fn ordinal_45() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "45")
}

/// Ordinal 46, `@ stub` in `msnet32.spec`.
pub fn ordinal_46() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "46")
}

/// Ordinal 47, `@ stub` in `msnet32.spec`.
pub fn ordinal_47() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "47")
}

/// Ordinal 48, `@ stub` in `msnet32.spec`.
pub fn ordinal_48() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "48")
}

/// Ordinal 49, `@ stub` in `msnet32.spec`.
pub fn ordinal_49() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "49")
}

/// Ordinal 50, `@ stub` in `msnet32.spec`.
pub fn ordinal_50() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "50")
}

/// Ordinal 51, `@ stub` in `msnet32.spec`.
pub fn ordinal_51() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "51")
}

/// Ordinal 52, `@ stub` in `msnet32.spec`.
pub fn ordinal_52() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "52")
}

/// Ordinal 53, `@ stub` in `msnet32.spec`.
pub fn ordinal_53() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "53")
}

/// Ordinal 54, `@ stub` in `msnet32.spec`.
pub fn ordinal_54() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "54")
}

/// Ordinal 55, `@ stub` in `msnet32.spec`.
pub fn ordinal_55() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "55")
}

/// Ordinal 56, `@ stub` in `msnet32.spec`.
pub fn ordinal_56() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "56")
}

/// Ordinal 58, `@ stub` in `msnet32.spec`.
pub fn ordinal_58() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "58")
}

/// Ordinal 59, `@ stub` in `msnet32.spec`.
pub fn ordinal_59() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "59")
}

/// Ordinal 60, `@ stub` in `msnet32.spec`.
pub fn ordinal_60() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "60")
}

/// Ordinal 61, `@ stub` in `msnet32.spec`.
pub fn ordinal_61() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "61")
}

/// Ordinal 62, `@ stub` in `msnet32.spec`.
pub fn ordinal_62() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "62")
}

/// Ordinal 63, `@ stub` in `msnet32.spec`.
pub fn ordinal_63() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "63")
}

/// Ordinal 64, `@ stub` in `msnet32.spec`.
pub fn ordinal_64() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "64")
}

/// Ordinal 65, `@ stub` in `msnet32.spec`.
pub fn ordinal_65() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "65")
}

/// Ordinal 66, `@ stub` in `msnet32.spec`.
pub fn ordinal_66() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "66")
}

/// Ordinal 67, `@ stub` in `msnet32.spec`.
pub fn ordinal_67() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "67")
}

/// Ordinal 68, `@ stub` in `msnet32.spec`.
pub fn ordinal_68() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "68")
}

/// Ordinal 69, `@ stub` in `msnet32.spec`.
pub fn ordinal_69() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "69")
}

/// Ordinal 70, `@ stub` in `msnet32.spec`.
pub fn ordinal_70() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "70")
}

/// Ordinal 71, `@ stub` in `msnet32.spec`.
pub fn ordinal_71() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "71")
}

/// Ordinal 72, `@ stub` in `msnet32.spec`.
pub fn ordinal_72() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "72")
}

/// Ordinal 73, `@ stub` in `msnet32.spec`.
pub fn ordinal_73() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "73")
}

/// Ordinal 74, `@ stub` in `msnet32.spec`.
pub fn ordinal_74() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "74")
}

/// Ordinal 75, `@ stub` in `msnet32.spec`.
pub fn ordinal_75() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "75")
}

/// Ordinal 76, `@ stub` in `msnet32.spec`.
pub fn ordinal_76() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "76")
}

/// Ordinal 77, `@ stub` in `msnet32.spec`.
pub fn ordinal_77() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "77")
}

/// Ordinal 78, `@ stub` in `msnet32.spec`.
pub fn ordinal_78() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "78")
}

/// Ordinal 79, `@ stub` in `msnet32.spec`.
pub fn ordinal_79() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "79")
}

/// Ordinal 80, `@ stub` in `msnet32.spec`.
pub fn ordinal_80() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "80")
}

/// Ordinal 81, `@ stub` in `msnet32.spec`.
pub fn ordinal_81() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "81")
}

/// Ordinal 82, `@ stub` in `msnet32.spec`.
pub fn ordinal_82() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "82")
}

/// Ordinal 83, `@ stub` in `msnet32.spec`.
pub fn ordinal_83() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "83")
}

/// Ordinal 84, `@ stub` in `msnet32.spec`.
pub fn ordinal_84() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "84")
}

/// Ordinal 85, `@ stub` in `msnet32.spec`.
pub fn ordinal_85() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "85")
}

/// Ordinal 86, `@ stub` in `msnet32.spec`.
pub fn ordinal_86() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "86")
}

/// Ordinal 87, `@ stub` in `msnet32.spec`.
pub fn ordinal_87() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "87")
}

/// Ordinal 88, `@ stub` in `msnet32.spec`.
pub fn ordinal_88() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "88")
}

/// Ordinal 89, `@ stub` in `msnet32.spec`.
pub fn ordinal_89() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "89")
}

/// Ordinal 90, `@ stub` in `msnet32.spec`.
pub fn ordinal_90() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "90")
}

/// Ordinal 91, `@ stub` in `msnet32.spec`.
pub fn ordinal_91() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "91")
}

/// Ordinal 92, `@ stub` in `msnet32.spec`.
pub fn ordinal_92() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "92")
}

/// Ordinal 93, `@ stub` in `msnet32.spec`.
pub fn ordinal_93() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "93")
}

/// Ordinal 94, `@ stub` in `msnet32.spec`.
pub fn ordinal_94() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "94")
}

/// Ordinal 95, `@ stub` in `msnet32.spec`.
pub fn ordinal_95() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "95")
}

/// Ordinal 96, `@ stub` in `msnet32.spec`.
pub fn ordinal_96() -> StubException {
    wine_spec_unimplemented_stub(MSNET32_DLL_NAME, "96")
}
