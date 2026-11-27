//! Oracle-derived test vectors for the ported icu stub records.
//!
//! Vectors were captured by compiling the verbatim `__wine_spec_unimplemented_stub` body
//! (`dlls/ntdll/exception.c`) with winebuild-style shims for the six `@ stub` exports of
//! `dlls/icu/icu.spec`, under `target/canon/icu/`, with `gcc -fsanitize=address,undefined -g`.
//! The harness replaces `RtlRaiseException` with a recorder that longjmps out, because the
//! verbatim `for (;;)` loop would otherwise never return. All 15 printed rows are kept: ASan and
//! UBSan reported nothing, so no vector is excluded.

use super::*;

/// The record every harness row prints: code 0x80000100, flags 0x1 and two parameters.
fn raised(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module,
        function,
    }
}

#[test]
fn oracle_stub_export_rows() {
    // row 01: export CloseDefaultICUGroupingLetters -> icu.dll, CloseDefaultICUGroupingLetters
    assert_eq!(
        close_default_icu_grouping_letters(),
        raised("icu.dll", "CloseDefaultICUGroupingLetters")
    );

    // row 02: export GetDefaultICUGroupingLetters -> icu.dll, GetDefaultICUGroupingLetters
    assert_eq!(
        get_default_icu_grouping_letters(),
        raised("icu.dll", "GetDefaultICUGroupingLetters")
    );

    // row 03: export GetICUGroupingLetter -> icu.dll, GetICUGroupingLetter
    assert_eq!(
        get_icu_grouping_letter(),
        raised("icu.dll", "GetICUGroupingLetter")
    );

    // row 04: export SortCloseHandle -> icu.dll, SortCloseHandle
    assert_eq!(sort_close_handle(), raised("icu.dll", "SortCloseHandle"));

    // row 05: export SortGetHandle -> icu.dll, SortGetHandle
    assert_eq!(sort_get_handle(), raised("icu.dll", "SortGetHandle"));

    // row 06: export SortGetSearchKey -> icu.dll, SortGetSearchKey
    assert_eq!(sort_get_search_key(), raised("icu.dll", "SortGetSearchKey"));
}

#[test]
fn oracle_direct_call_rows() {
    // row 07: direct module="" function="" -> p0= p1=
    assert_eq!(wine_spec_unimplemented_stub("", ""), raised("", ""));

    // row 08: direct module="icu.dll" function="" -> p0=icu.dll p1=
    assert_eq!(
        wine_spec_unimplemented_stub("icu.dll", ""),
        raised("icu.dll", "")
    );

    // row 09: direct module="" function="u_strlen" -> p0= p1=u_strlen
    assert_eq!(
        wine_spec_unimplemented_stub("", "u_strlen"),
        raised("", "u_strlen")
    );

    // row 10: direct module="icu.dll" function="u_strlen" -> p0=icu.dll p1=u_strlen
    assert_eq!(
        wine_spec_unimplemented_stub("icu.dll", "u_strlen"),
        raised("icu.dll", "u_strlen")
    );

    // row 11: direct module="ICU.DLL" function="SortGetHandle" -> p0=ICU.DLL p1=SortGetHandle
    assert_eq!(
        wine_spec_unimplemented_stub("ICU.DLL", "SortGetHandle"),
        raised("ICU.DLL", "SortGetHandle")
    );

    // row 12: direct module="icu.dll" function="sortgethandle" -> p0=icu.dll p1=sortgethandle
    assert_eq!(
        wine_spec_unimplemented_stub("icu.dll", "sortgethandle"),
        raised("icu.dll", "sortgethandle")
    );

    // row 13: direct module="ntdll.dll" function="RtlRaiseException" -> p0=ntdll.dll p1=RtlRaiseException
    assert_eq!(
        wine_spec_unimplemented_stub("ntdll.dll", "RtlRaiseException"),
        raised("ntdll.dll", "RtlRaiseException")
    );

    // row 14: direct module="icu.dll" function="ucol_getSortKey" -> p0=icu.dll p1=ucol_getSortKey
    assert_eq!(
        wine_spec_unimplemented_stub("icu.dll", "ucol_getSortKey"),
        raised("icu.dll", "ucol_getSortKey")
    );

    // row 15: direct module="icu.dll.so" function="GetICUGroupingLetter" -> p0=icu.dll.so p1=GetICUGroupingLetter
    assert_eq!(
        wine_spec_unimplemented_stub("icu.dll.so", "GetICUGroupingLetter"),
        raised("icu.dll.so", "GetICUGroupingLetter")
    );
}
