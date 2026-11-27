//! Oracle-derived test vectors for the msvcp120_app export table and stub records.
//!
//! Vectors were printed by `target/canon/msvcp120_app/harness.c`, which parses the verbatim
//! `msvcp120_app.spec` and `Makefile.in` from Wine's `dlls/msvcp120_app`, and runs the verbatim
//! `__wine_spec_unimplemented_stub` body (`dlls/ntdll/exception.c`) once per `@ stub` line. The
//! harness was built with `gcc -fsanitize=address,undefined -g` and reported no sanitizer output,
//! so no vector is excluded.
//!
//! The printed rows are one summary row, one module row and 395 stub rows, one per `@ stub` line
//! in spec order. The summary row covers all 3810 export lines: totals per kind and per arch, the
//! forward check, and a 64-bit FNV-1a digest of every `kind|arch|name|args` line. The digest stands
//! in for 3810 printed rows.

use super::*;

const ORACLE_TOTAL: usize = 3810;
const ORACLE_KIND_COUNTS: [(ExportKind, usize); 4] = [
    (ExportKind::Cdecl, 2288),
    (ExportKind::Thiscall, 967),
    (ExportKind::Extern, 160),
    (ExportKind::Stub, 395),
];
const ORACLE_ARCH_COUNTS: [(Option<Arch>, usize); 5] = [
    (None, 309),
    (Some(Arch::Arm), 1053),
    (Some(Arch::I386), 1053),
    (Some(Arch::Win32), 171),
    (Some(Arch::Win64), 1224),
];
const ORACLE_FORWARD_MISMATCHES: usize = 0;
const ORACLE_DIGEST: u64 = 0x9b4ac1f31c7ebed3;
const ORACLE_MODULE: &str = "msvcp120_app.dll";

/// A `@ stub` export: the helper the port maps it to, the export name the harness passed to
/// `__wine_spec_unimplemented_stub`, and the `-arch=` value on its spec line.
type StubRow = (fn() -> StubException, &'static str, Option<Arch>);

/// One row per `@ stub` line, in spec order. Every printed row has code `0x80000100`, flags `0x1`,
/// 2 parameters and module `msvcp120_app.dll`.
pub(crate) const STUB_ROWS: &[StubRow] = &[
    // row 001
    (
        _getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_iaax_wabv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IAAX_WABV_Locinfo@1@@Z",
        Some(Arch::Arm),
    ),
    // row 002
    (
        _getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_iaex_wabv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IAEX_WABV_Locinfo@1@@Z",
        Some(Arch::I386),
    ),
    // row 003
    (
        _getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_ieaax_waebv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z",
        Some(Arch::Win64),
    ),
    // row 004
    (
        _getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaax_wabv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAAX_WABV_Locinfo@1@@Z",
        Some(Arch::Arm),
    ),
    // row 005
    (
        _getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaex_wabv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAEX_WABV_Locinfo@1@@Z",
        Some(Arch::I386),
    ),
    // row 006
    (
        _getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaax_waebv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z",
        Some(Arch::Win64),
    ),
    // row 007
    (
        _getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_iaax_wabv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IAAX_WABV_Locinfo@1@@Z",
        Some(Arch::Arm),
    ),
    // row 008
    (
        _getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_iaex_wabv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IAEX_WABV_Locinfo@1@@Z",
        Some(Arch::I386),
    ),
    // row 009
    (
        _getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_ieaax_waebv_locinfo_1_z,
        "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z",
        Some(Arch::Win64),
    ),
    // row 010
    (
        _0_basic_iostream_du_char_traits_d_std_std_iaa_qav01_z,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 011
    (
        _0_basic_iostream_du_char_traits_d_std_std_iae_qav01_z,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 012
    (
        _0_basic_iostream_du_char_traits_d_std_std_ieaa_qeav01_z,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 013
    (
        _0_basic_iostream_gu_char_traits_g_std_std_iaa_qav01_z,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 014
    (
        _0_basic_iostream_gu_char_traits_g_std_std_iae_qav01_z,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 015
    (
        _0_basic_iostream_gu_char_traits_g_std_std_ieaa_qeav01_z,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 016
    (
        _0_basic_iostream_wu_char_traits_w_std_std_iaa_qav01_z,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 017
    (
        _0_basic_iostream_wu_char_traits_w_std_std_iae_qav01_z,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 018
    (
        _0_basic_iostream_wu_char_traits_w_std_std_ieaa_qeav01_z,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 019
    (
        _0_basic_istream_du_char_traits_d_std_std_iaa_qav01_z,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 020
    (
        _0_basic_istream_du_char_traits_d_std_std_iae_qav01_z,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 021
    (
        _0_basic_istream_du_char_traits_d_std_std_ieaa_qeav01_z,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 022
    (
        _0_basic_istream_gu_char_traits_g_std_std_iaa_qav01_z,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 023
    (
        _0_basic_istream_gu_char_traits_g_std_std_iae_qav01_z,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 024
    (
        _0_basic_istream_gu_char_traits_g_std_std_ieaa_qeav01_z,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 025
    (
        _0_basic_istream_wu_char_traits_w_std_std_iaa_qav01_z,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 026
    (
        _0_basic_istream_wu_char_traits_w_std_std_iae_qav01_z,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 027
    (
        _0_basic_istream_wu_char_traits_w_std_std_ieaa_qeav01_z,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 028
    (
        _0_basic_ostream_du_char_traits_d_std_std_iaa_qav01_z,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 029
    (
        _0_basic_ostream_du_char_traits_d_std_std_iae_qav01_z,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 030
    (
        _0_basic_ostream_du_char_traits_d_std_std_ieaa_qeav01_z,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 031
    (
        _0_basic_ostream_gu_char_traits_g_std_std_iaa_qav01_z,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 032
    (
        _0_basic_ostream_gu_char_traits_g_std_std_iae_qav01_z,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 033
    (
        _0_basic_ostream_gu_char_traits_g_std_std_ieaa_qeav01_z,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 034
    (
        _0_basic_ostream_wu_char_traits_w_std_std_iaa_qav01_z,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAA@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 035
    (
        _0_basic_ostream_wu_char_traits_w_std_std_iae_qav01_z,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAE@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 036
    (
        _0_basic_ostream_wu_char_traits_w_std_std_ieaa_qeav01_z,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 037
    (
        _0_basic_streambuf_du_char_traits_d_std_std_iaa_abv01_z,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAA@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 038
    (
        _0_basic_streambuf_du_char_traits_d_std_std_iae_abv01_z,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAE@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 039
    (
        _0_basic_streambuf_du_char_traits_d_std_std_ieaa_aebv01_z,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 040
    (
        _0_basic_streambuf_gu_char_traits_g_std_std_iaa_abv01_z,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAA@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 041
    (
        _0_basic_streambuf_gu_char_traits_g_std_std_iae_abv01_z,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAE@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 042
    (
        _0_basic_streambuf_gu_char_traits_g_std_std_ieaa_aebv01_z,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 043
    (
        _0_basic_streambuf_wu_char_traits_w_std_std_iaa_abv01_z,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAA@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 044
    (
        _0_basic_streambuf_wu_char_traits_w_std_std_iae_abv01_z,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAE@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 045
    (
        _0_basic_streambuf_wu_char_traits_w_std_std_ieaa_aebv01_z,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 046
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaa_pbdi_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAA@PBDI@Z",
        Some(Arch::Arm),
    ),
    // row 047
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iae_pbdi_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAE@PBDI@Z",
        Some(Arch::I386),
    ),
    // row 048
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaa_pebd_k_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAA@PEBD_K@Z",
        Some(Arch::Win64),
    ),
    // row 049
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaa_abv_locinfo_1_i_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAA@ABV_Locinfo@1@I@Z",
        Some(Arch::Arm),
    ),
    // row 050
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qae_abv_locinfo_1_i_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@ABV_Locinfo@1@I@Z",
        Some(Arch::I386),
    ),
    // row 051
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_aebv_locinfo_1_k_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z",
        Some(Arch::Win64),
    ),
    // row 052
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaa_i_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAA@I@Z",
        Some(Arch::Arm),
    ),
    // row 053
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qae_i_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@I@Z",
        Some(Arch::I386),
    ),
    // row 054
    (
        _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_k_z,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z",
        Some(Arch::Win64),
    ),
    // row 055
    (
        _0_concurrent_queue_iterator_base_v4_details_concurrency_iaa_abv_concurrent_queue_base_v4_12_z,
        "??0_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAA@ABV_Concurrent_queue_base_v4@12@@Z",
        Some(Arch::Arm),
    ),
    // row 056
    (
        _0_concurrent_queue_iterator_base_v4_details_concurrency_iae_abv_concurrent_queue_base_v4_12_z,
        "??0_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAE@ABV_Concurrent_queue_base_v4@12@@Z",
        Some(Arch::I386),
    ),
    // row 057
    (
        _0_concurrent_queue_iterator_base_v4_details_concurrency_ieaa_aebv_concurrent_queue_base_v4_12_z,
        "??0_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAA@AEBV_Concurrent_queue_base_v4@12@@Z",
        Some(Arch::Win64),
    ),
    // row 058
    (
        _0_facet_base_std_qaa_abv01_z,
        "??0_Facet_base@std@@QAA@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 059
    (
        _0_facet_base_std_qae_abv01_z,
        "??0_Facet_base@std@@QAE@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 060
    (
        _0_facet_base_std_qeaa_aebv01_z,
        "??0_Facet_base@std@@QEAA@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 061
    (
        _0_facet_base_std_qaa_xz,
        "??0_Facet_base@std@@QAA@XZ",
        Some(Arch::Arm),
    ),
    // row 062
    (
        _0_facet_base_std_qae_xz,
        "??0_Facet_base@std@@QAE@XZ",
        Some(Arch::I386),
    ),
    // row 063
    (
        _0_facet_base_std_qeaa_xz,
        "??0_Facet_base@std@@QEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 064
    (
        _0_ushinit_std_qaa_xz,
        "??0_UShinit@std@@QAA@XZ",
        Some(Arch::Arm),
    ),
    // row 065
    (
        _0_ushinit_std_qae_xz,
        "??0_UShinit@std@@QAE@XZ",
        Some(Arch::I386),
    ),
    // row 066
    (
        _0_ushinit_std_qeaa_xz,
        "??0_UShinit@std@@QEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 067
    (
        _0agent_concurrency_qaa_aavschedulegroup_1_z,
        "??0agent@Concurrency@@QAA@AAVScheduleGroup@1@@Z",
        Some(Arch::Arm),
    ),
    // row 068
    (
        _0agent_concurrency_qae_aavschedulegroup_1_z,
        "??0agent@Concurrency@@QAE@AAVScheduleGroup@1@@Z",
        Some(Arch::I386),
    ),
    // row 069
    (
        _0agent_concurrency_qeaa_aeavschedulegroup_1_z,
        "??0agent@Concurrency@@QEAA@AEAVScheduleGroup@1@@Z",
        Some(Arch::Win64),
    ),
    // row 070
    (
        _0agent_concurrency_qaa_aavscheduler_1_z,
        "??0agent@Concurrency@@QAA@AAVScheduler@1@@Z",
        Some(Arch::Arm),
    ),
    // row 071
    (
        _0agent_concurrency_qae_aavscheduler_1_z,
        "??0agent@Concurrency@@QAE@AAVScheduler@1@@Z",
        Some(Arch::I386),
    ),
    // row 072
    (
        _0agent_concurrency_qeaa_aeavscheduler_1_z,
        "??0agent@Concurrency@@QEAA@AEAVScheduler@1@@Z",
        Some(Arch::Win64),
    ),
    // row 073
    (
        _0agent_concurrency_qaa_xz,
        "??0agent@Concurrency@@QAA@XZ",
        Some(Arch::Arm),
    ),
    // row 074
    (
        _0agent_concurrency_qae_xz,
        "??0agent@Concurrency@@QAE@XZ",
        Some(Arch::I386),
    ),
    // row 075
    (
        _0agent_concurrency_qeaa_xz,
        "??0agent@Concurrency@@QEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 076
    (
        _0time_base_std_qaa_i_z,
        "??0time_base@std@@QAA@I@Z",
        Some(Arch::Arm),
    ),
    // row 077
    (
        _0time_base_std_qae_i_z,
        "??0time_base@std@@QAE@I@Z",
        Some(Arch::I386),
    ),
    // row 078
    (
        _0time_base_std_qeaa_k_z,
        "??0time_base@std@@QEAA@_K@Z",
        Some(Arch::Win64),
    ),
    // row 079
    (
        _1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_maa_xz,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MAA@XZ",
        Some(Arch::Arm),
    ),
    // row 080
    (
        _1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mae_xz,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MAE@XZ",
        Some(Arch::I386),
    ),
    // row 081
    (
        _1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meaa_xz,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 082
    (
        _1_concurrent_queue_iterator_base_v4_details_concurrency_iaa_xz,
        "??1_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAA@XZ",
        Some(Arch::Arm),
    ),
    // row 083
    (
        _1_concurrent_queue_iterator_base_v4_details_concurrency_iae_xz,
        "??1_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAE@XZ",
        Some(Arch::I386),
    ),
    // row 084
    (
        _1_concurrent_queue_iterator_base_v4_details_concurrency_ieaa_xz,
        "??1_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 085
    (
        _1_facet_base_std_uaa_xz,
        "??1_Facet_base@std@@UAA@XZ",
        Some(Arch::Arm),
    ),
    // row 086
    (
        _1_facet_base_std_uae_xz,
        "??1_Facet_base@std@@UAE@XZ",
        Some(Arch::I386),
    ),
    // row 087
    (
        _1_facet_base_std_ueaa_xz,
        "??1_Facet_base@std@@UEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 088
    (
        _1_ushinit_std_qaa_xz,
        "??1_UShinit@std@@QAA@XZ",
        Some(Arch::Arm),
    ),
    // row 089
    (
        _1_ushinit_std_qae_xz,
        "??1_UShinit@std@@QAE@XZ",
        Some(Arch::I386),
    ),
    // row 090
    (
        _1_ushinit_std_qeaa_xz,
        "??1_UShinit@std@@QEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 091
    (
        _1agent_concurrency_uaa_xz,
        "??1agent@Concurrency@@UAA@XZ",
        Some(Arch::Arm),
    ),
    // row 092
    (
        _1agent_concurrency_uae_xz,
        "??1agent@Concurrency@@UAE@XZ",
        Some(Arch::I386),
    ),
    // row 093
    (
        _1agent_concurrency_ueaa_xz,
        "??1agent@Concurrency@@UEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 094
    (
        _1time_base_std_uaa_xz,
        "??1time_base@std@@UAA@XZ",
        Some(Arch::Arm),
    ),
    // row 095
    (
        _1time_base_std_uae_xz,
        "??1time_base@std@@UAE@XZ",
        Some(Arch::I386),
    ),
    // row 096
    (
        _1time_base_std_ueaa_xz,
        "??1time_base@std@@UEAA@XZ",
        Some(Arch::Win64),
    ),
    // row 097
    (
        _4_iosb_h_std_qaaaav01_abv01_z,
        "??4?$_Iosb@H@std@@QAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 098
    (
        _4_iosb_h_std_qaeaav01_abv01_z,
        "??4?$_Iosb@H@std@@QAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 099
    (
        _4_iosb_h_std_qeaaaeav01_aebv01_z,
        "??4?$_Iosb@H@std@@QEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 100
    (
        _4_basic_iostream_du_char_traits_d_std_std_iaaaav01_qav01_z,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 101
    (
        _4_basic_iostream_du_char_traits_d_std_std_iaeaav01_qav01_z,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 102
    (
        _4_basic_iostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 103
    (
        _4_basic_iostream_gu_char_traits_g_std_std_iaaaav01_qav01_z,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 104
    (
        _4_basic_iostream_gu_char_traits_g_std_std_iaeaav01_qav01_z,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 105
    (
        _4_basic_iostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 106
    (
        _4_basic_iostream_wu_char_traits_w_std_std_iaaaav01_qav01_z,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 107
    (
        _4_basic_iostream_wu_char_traits_w_std_std_iaeaav01_qav01_z,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 108
    (
        _4_basic_iostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 109
    (
        _4_basic_istream_du_char_traits_d_std_std_iaaaav01_qav01_z,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 110
    (
        _4_basic_istream_du_char_traits_d_std_std_iaeaav01_qav01_z,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 111
    (
        _4_basic_istream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 112
    (
        _4_basic_istream_gu_char_traits_g_std_std_iaaaav01_qav01_z,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 113
    (
        _4_basic_istream_gu_char_traits_g_std_std_iaeaav01_qav01_z,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 114
    (
        _4_basic_istream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 115
    (
        _4_basic_istream_wu_char_traits_w_std_std_iaaaav01_qav01_z,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 116
    (
        _4_basic_istream_wu_char_traits_w_std_std_iaeaav01_qav01_z,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 117
    (
        _4_basic_istream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 118
    (
        _4_basic_ostream_du_char_traits_d_std_std_iaaaav01_qav01_z,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 119
    (
        _4_basic_ostream_du_char_traits_d_std_std_iaeaav01_qav01_z,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 120
    (
        _4_basic_ostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 121
    (
        _4_basic_ostream_gu_char_traits_g_std_std_iaaaav01_qav01_z,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 122
    (
        _4_basic_ostream_gu_char_traits_g_std_std_iaeaav01_qav01_z,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 123
    (
        _4_basic_ostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 124
    (
        _4_basic_ostream_wu_char_traits_w_std_std_iaaaav01_qav01_z,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAAAAV01@$$QAV01@@Z",
        Some(Arch::Arm),
    ),
    // row 125
    (
        _4_basic_ostream_wu_char_traits_w_std_std_iaeaav01_qav01_z,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAEAAV01@$$QAV01@@Z",
        Some(Arch::I386),
    ),
    // row 126
    (
        _4_basic_ostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
        Some(Arch::Win64),
    ),
    // row 127
    (
        _4_basic_streambuf_du_char_traits_d_std_std_iaaaav01_abv01_z,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 128
    (
        _4_basic_streambuf_du_char_traits_d_std_std_iaeaav01_abv01_z,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 129
    (
        _4_basic_streambuf_du_char_traits_d_std_std_ieaaaeav01_aebv01_z,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 130
    (
        _4_basic_streambuf_gu_char_traits_g_std_std_iaaaav01_abv01_z,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 131
    (
        _4_basic_streambuf_gu_char_traits_g_std_std_iaeaav01_abv01_z,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 132
    (
        _4_basic_streambuf_gu_char_traits_g_std_std_ieaaaeav01_aebv01_z,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 133
    (
        _4_basic_streambuf_wu_char_traits_w_std_std_iaaaav01_abv01_z,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 134
    (
        _4_basic_streambuf_wu_char_traits_w_std_std_iaeaav01_abv01_z,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 135
    (
        _4_basic_streambuf_wu_char_traits_w_std_std_ieaaaeav01_aebv01_z,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 136
    (
        _4_facet_base_std_qaaaav01_abv01_z,
        "??4_Facet_base@std@@QAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 137
    (
        _4_facet_base_std_qaeaav01_abv01_z,
        "??4_Facet_base@std@@QAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 138
    (
        _4_facet_base_std_qeaaaeav01_aebv01_z,
        "??4_Facet_base@std@@QEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 139
    (
        _4_init_locks_std_qaaaav01_abv01_z,
        "??4_Init_locks@std@@QAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 140
    (
        _4_init_locks_std_qaeaav01_abv01_z,
        "??4_Init_locks@std@@QAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 141
    (
        _4_init_locks_std_qeaaaeav01_aebv01_z,
        "??4_Init_locks@std@@QEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 142
    (
        _4_ushinit_std_qaaaav01_abv01_z,
        "??4_UShinit@std@@QAAAAV01@ABV01@@Z",
        Some(Arch::Arm),
    ),
    // row 143
    (
        _4_ushinit_std_qaeaav01_abv01_z,
        "??4_UShinit@std@@QAEAAV01@ABV01@@Z",
        Some(Arch::I386),
    ),
    // row 144
    (
        _4_ushinit_std_qeaaaeav01_aebv01_z,
        "??4_UShinit@std@@QEAAAEAV01@AEBV01@@Z",
        Some(Arch::Win64),
    ),
    // row 145
    (
        _5_basic_istream_gu_char_traits_g_std_std_qaaaav01_aag_z,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QAAAAV01@AAG@Z",
        Some(Arch::Arm),
    ),
    // row 146
    (
        _5_basic_istream_gu_char_traits_g_std_std_qaeaav01_aag_z,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QAEAAV01@AAG@Z",
        Some(Arch::I386),
    ),
    // row 147
    (
        _5_basic_istream_gu_char_traits_g_std_std_qeaaaeav01_aeag_z,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAG@Z",
        Some(Arch::Win64),
    ),
    // row 148
    (
        _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaaxxz,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAAXXZ",
        Some(Arch::Arm),
    ),
    // row 149
    (
        _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaexxz,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAEXXZ",
        Some(Arch::I386),
    ),
    // row 150
    (
        _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaaxxz,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ",
        Some(Arch::Win64),
    ),
    // row 151
    (
        _ftime_base_std_qaaxxz,
        "??_Ftime_base@std@@QAAXXZ",
        Some(Arch::Arm),
    ),
    // row 152
    (
        _ftime_base_std_qaexxz,
        "??_Ftime_base@std@@QAEXXZ",
        Some(Arch::I386),
    ),
    // row 153
    (
        _ftime_base_std_qeaaxxz,
        "??_Ftime_base@std@@QEAAXXZ",
        Some(Arch::Win64),
    ),
    // row 154
    (
        nfs_allocate_details_concurrency_yapaxiipax_z,
        "?NFS_Allocate@details@Concurrency@@YAPAXIIPAX@Z",
        Some(Arch::Win32),
    ),
    // row 155
    (
        nfs_allocate_details_concurrency_yapeax_k0peax_z,
        "?NFS_Allocate@details@Concurrency@@YAPEAX_K0PEAX@Z",
        Some(Arch::Win64),
    ),
    // row 156
    (
        nfs_free_details_concurrency_yaxpax_z,
        "?NFS_Free@details@Concurrency@@YAXPAX@Z",
        Some(Arch::Win32),
    ),
    // row 157
    (
        nfs_free_details_concurrency_yaxpeax_z,
        "?NFS_Free@details@Concurrency@@YAXPEAX@Z",
        Some(Arch::Win64),
    ),
    // row 158
    (
        nfs_getlinesize_details_concurrency_yaixz,
        "?NFS_GetLineSize@details@Concurrency@@YAIXZ",
        Some(Arch::Win32),
    ),
    // row 159
    (
        nfs_getlinesize_details_concurrency_ya_kxz,
        "?NFS_GetLineSize@details@Concurrency@@YA_KXZ",
        Some(Arch::Win64),
    ),
    // row 160
    (
        _advance_concurrent_queue_iterator_base_v4_details_concurrency_iaaxxz,
        "?_Advance@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAAXXZ",
        Some(Arch::Arm),
    ),
    // row 161
    (
        _advance_concurrent_queue_iterator_base_v4_details_concurrency_iaexxz,
        "?_Advance@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAEXXZ",
        Some(Arch::I386),
    ),
    // row 162
    (
        _advance_concurrent_queue_iterator_base_v4_details_concurrency_ieaaxxz,
        "?_Advance@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAAXXZ",
        Some(Arch::Win64),
    ),
    // row 163
    (
        _assign_concurrent_queue_iterator_base_v4_details_concurrency_iaaxabv123_z,
        "?_Assign@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAAXABV123@@Z",
        Some(Arch::Arm),
    ),
    // row 164
    (
        _assign_concurrent_queue_iterator_base_v4_details_concurrency_iaexabv123_z,
        "?_Assign@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAEXABV123@@Z",
        Some(Arch::I386),
    ),
    // row 165
    (
        _assign_concurrent_queue_iterator_base_v4_details_concurrency_ieaaxaebv123_z,
        "?_Assign@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAAXAEBV123@@Z",
        Some(Arch::Win64),
    ),
    // row 166
    (
        _atexit_yaxp6axxz_z,
        "?_Atexit@@YAXP6AXXZ@Z",
        None,
    ),
    // row 167
    (
        _future_error_map_std_yapbdh_z,
        "?_Future_error_map@std@@YAPBDH@Z",
        Some(Arch::Win32),
    ),
    // row 168
    (
        _future_error_map_std_yapebdh_z,
        "?_Future_error_map@std@@YAPEBDH@Z",
        Some(Arch::Win64),
    ),
    // row 169
    (
        _getcat_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_saipapbvfacet_locale_2_pbv42_z,
        "?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SAIPAPBVfacet@locale@2@PBV42@@Z",
        Some(Arch::Win32),
    ),
    // row 170
    (
        _getcat_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_sa_kpeapebvfacet_locale_2_pebv42_z,
        "?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z",
        Some(Arch::Win64),
    ),
    // row 171
    (
        _getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_2_1aavios_base_2_pah_z,
        "?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@1AAVios_base@2@PAH@Z",
        Some(Arch::Win32),
    ),
    // row 172
    (
        _getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_2_1aeavios_base_2_peah_z,
        "?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@1AEAVios_base@2@PEAH@Z",
        Some(Arch::Win64),
    ),
    // row 173
    (
        _getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_2_1aavios_base_2_pah_z,
        "?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@1AAVios_base@2@PAH@Z",
        Some(Arch::Win32),
    ),
    // row 174
    (
        _getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_2_1aeavios_base_2_peah_z,
        "?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@1AEAVios_base@2@PEAH@Z",
        Some(Arch::Win64),
    ),
    // row 175
    (
        _getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_2_1aavios_base_2_pah_z,
        "?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@1AAVios_base@2@PAH@Z",
        Some(Arch::Win32),
    ),
    // row 176
    (
        _getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_2_1aeavios_base_2_peah_z,
        "?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@1AEAVios_base@2@PEAH@Z",
        Some(Arch::Win64),
    ),
    // row 177
    (
        _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbd_z,
        "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBD@Z",
        Some(Arch::Arm),
    ),
    // row 178
    (
        _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ibe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbd_z,
        "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBD@Z",
        Some(Arch::I386),
    ),
    // row 179
    (
        _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebd_z,
        "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBD@Z",
        Some(Arch::Win64),
    ),
    // row 180
    (
        _getint_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahaav_istreambuf_iterator_du_char_traits_d_std_2_0hhaahabv_ctype_d_2_z,
        "?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@0HHAAHABV?$ctype@D@2@@Z",
        Some(Arch::Win32),
    ),
    // row 181
    (
        _getint_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahaeav_istreambuf_iterator_du_char_traits_d_std_2_0hhaeahaebv_ctype_d_2_z,
        "?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@0HHAEAHAEBV?$ctype@D@2@@Z",
        Some(Arch::Win64),
    ),
    // row 182
    (
        _getint_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahaav_istreambuf_iterator_gu_char_traits_g_std_2_0hhaahabv_ctype_g_2_z,
        "?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@0HHAAHABV?$ctype@G@2@@Z",
        Some(Arch::Win32),
    ),
    // row 183
    (
        _getint_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahaeav_istreambuf_iterator_gu_char_traits_g_std_2_0hhaeahaebv_ctype_g_2_z,
        "?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@0HHAEAHAEBV?$ctype@G@2@@Z",
        Some(Arch::Win64),
    ),
    // row 184
    (
        _getint_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahaav_istreambuf_iterator_wu_char_traits_w_std_2_0hhaahabv_ctype_w_2_z,
        "?_Getint@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@0HHAAHABV?$ctype@_W@2@@Z",
        Some(Arch::Win32),
    ),
    // row 185
    (
        _getint_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahaeav_istreambuf_iterator_wu_char_traits_w_std_2_0hhaeahaebv_ctype_w_2_z,
        "?_Getint@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@0HHAEAHAEBV?$ctype@_W@2@@Z",
        Some(Arch::Win64),
    ),
    // row 186
    (
        _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaaxabv_locinfo_2_z,
        "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAAXABV_Locinfo@2@@Z",
        Some(Arch::Arm),
    ),
    // row 187
    (
        _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaexabv_locinfo_2_z,
        "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAEXABV_Locinfo@2@@Z",
        Some(Arch::I386),
    ),
    // row 188
    (
        _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaaxaebv_locinfo_2_z,
        "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z",
        Some(Arch::Win64),
    ),
    // row 189
    (
        _mp_add_std_yaxqa_k_k_z,
        "?_MP_Add@std@@YAXQA_K_K@Z",
        Some(Arch::Win32),
    ),
    // row 190
    (
        _mp_add_std_yaxqea_k_k_z,
        "?_MP_Add@std@@YAXQEA_K_K@Z",
        Some(Arch::Win64),
    ),
    // row 191
    (
        _mp_get_std_ya_kqa_k_z,
        "?_MP_Get@std@@YA_KQA_K@Z",
        Some(Arch::Win32),
    ),
    // row 192
    (
        _mp_get_std_ya_kqea_k_z,
        "?_MP_Get@std@@YA_KQEA_K@Z",
        Some(Arch::Win64),
    ),
    // row 193
    (
        _mp_mul_std_yaxqa_k_k1_z,
        "?_MP_Mul@std@@YAXQA_K_K1@Z",
        Some(Arch::Win32),
    ),
    // row 194
    (
        _mp_mul_std_yaxqea_k_k1_z,
        "?_MP_Mul@std@@YAXQEA_K_K1@Z",
        Some(Arch::Win64),
    ),
    // row 195
    (
        _mp_rem_std_yaxqa_k_k_z,
        "?_MP_Rem@std@@YAXQA_K_K@Z",
        Some(Arch::Win32),
    ),
    // row 196
    (
        _mp_rem_std_yaxqea_k_k_z,
        "?_MP_Rem@std@@YAXQEA_K_K@Z",
        Some(Arch::Win64),
    ),
    // row 197
    (
        _rng_abort_std_yaxpbd_z,
        "?_Rng_abort@std@@YAXPBD@Z",
        Some(Arch::Win32),
    ),
    // row 198
    (
        _rng_abort_std_yaxpebd_z,
        "?_Rng_abort@std@@YAXPEBD@Z",
        Some(Arch::Win64),
    ),
    // row 199
    (
        _src_1_getffldx_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_3_1aavios_base_3_pah_z_4qbdb,
        "?_Src@?1??_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AAVios_base@3@PAH@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 200
    (
        _src_1_getffldx_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb,
        "?_Src@?1??_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 201
    (
        _src_1_getffldx_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_3_1aavios_base_3_pah_z_4qbdb,
        "?_Src@?1??_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AAVios_base@3@PAH@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 202
    (
        _src_1_getffldx_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb,
        "?_Src@?1??_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 203
    (
        _src_1_getffldx_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_3_1aavios_base_3_pah_z_4qbdb,
        "?_Src@?1??_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AAVios_base@3@PAH@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 204
    (
        _src_1_getffldx_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb,
        "?_Src@?1??_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 205
    (
        _src_1_getifld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_3_1habvlocale_3_z_4qbdb,
        "?_Src@?1??_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1HABVlocale@3@@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 206
    (
        _src_1_getifld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1haebvlocale_3_z_4qbdb,
        "?_Src@?1??_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1HAEBVlocale@3@@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 207
    (
        _src_1_getifld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_3_1habvlocale_3_z_4qbdb,
        "?_Src@?1??_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1HABVlocale@3@@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 208
    (
        _src_1_getifld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1haebvlocale_3_z_4qbdb,
        "?_Src@?1??_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1HAEBVlocale@3@@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 209
    (
        _src_1_getifld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_3_1habvlocale_3_z_4qbdb,
        "?_Src@?1??_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1HABVlocale@3@@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 210
    (
        _src_1_getifld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1haebvlocale_3_z_4qbdb,
        "?_Src@?1??_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1HAEBVlocale@3@@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 211
    (
        _src_3_getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_3_1aavios_base_3_pah_z_4qbdb,
        "?_Src@?3??_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AAVios_base@3@PAH@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 212
    (
        _src_3_getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb,
        "?_Src@?3??_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 213
    (
        _src_3_getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_3_1aavios_base_3_pah_z_4qbdb,
        "?_Src@?3??_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AAVios_base@3@PAH@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 214
    (
        _src_3_getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb,
        "?_Src@?3??_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 215
    (
        _src_3_getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_3_1aavios_base_3_pah_z_4qbdb,
        "?_Src@?3??_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AAVios_base@3@PAH@Z@4QBDB",
        Some(Arch::Win32),
    ),
    // row 216
    (
        _src_3_getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb,
        "?_Src@?3??_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB",
        Some(Arch::Win64),
    ),
    // row 217
    (
        _throw_cpp_error_std_yaxh_z,
        "?_Throw_Cpp_error@std@@YAXH@Z",
        None,
    ),
    // row 218
    (
        _throw_lock_error_threads_stdext_yaxxz,
        "?_Throw_lock_error@threads@stdext@@YAXXZ",
        None,
    ),
    // row 219
    (
        _throw_resource_error_threads_stdext_yaxxz,
        "?_Throw_resource_error@threads@stdext@@YAXXZ",
        None,
    ),
    // row 220
    (
        _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aaaxxz,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AAAXXZ",
        Some(Arch::Arm),
    ),
    // row 221
    (
        _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aaexxz,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AAEXXZ",
        Some(Arch::I386),
    ),
    // row 222
    (
        _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aeaaxxz,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEAAXXZ",
        Some(Arch::Win64),
    ),
    // row 223
    (
        _w_gettnames_locinfo_std_qba_av_timevec_2_xz,
        "?_W_Gettnames@_Locinfo@std@@QBA?AV_Timevec@2@XZ",
        Some(Arch::Arm),
    ),
    // row 224
    (
        _w_gettnames_locinfo_std_qbe_av_timevec_2_xz,
        "?_W_Gettnames@_Locinfo@std@@QBE?AV_Timevec@2@XZ",
        Some(Arch::I386),
    ),
    // row 225
    (
        _w_gettnames_locinfo_std_qeba_av_timevec_2_xz,
        "?_W_Gettnames@_Locinfo@std@@QEBA?AV_Timevec@2@XZ",
        Some(Arch::Win64),
    ),
    // row 226
    (
        _xoverflow_error_std_yaxpbd_z,
        "?_Xoverflow_error@std@@YAXPBD@Z",
        Some(Arch::Win32),
    ),
    // row 227
    (
        _xoverflow_error_std_yaxpebd_z,
        "?_Xoverflow_error@std@@YAXPEBD@Z",
        Some(Arch::Win64),
    ),
    // row 228
    (
        cancel_agent_concurrency_qaa_nxz,
        "?cancel@agent@Concurrency@@QAA_NXZ",
        Some(Arch::Arm),
    ),
    // row 229
    (
        cancel_agent_concurrency_qae_nxz,
        "?cancel@agent@Concurrency@@QAE_NXZ",
        Some(Arch::I386),
    ),
    // row 230
    (
        cancel_agent_concurrency_qeaa_nxz,
        "?cancel@agent@Concurrency@@QEAA_NXZ",
        Some(Arch::Win64),
    ),
    // row 231
    (
        classic_table_ctype_d_std_sapbfxz,
        "?classic_table@?$ctype@D@std@@SAPBFXZ",
        Some(Arch::Win32),
    ),
    // row 232
    (
        classic_table_ctype_d_std_sapebfxz,
        "?classic_table@?$ctype@D@std@@SAPEBFXZ",
        Some(Arch::Win64),
    ),
    // row 233
    (
        date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_aw4dateorder_time_base_2_xz,
        "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AW4dateorder@time_base@2@XZ",
        Some(Arch::Arm),
    ),
    // row 234
    (
        date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_aw4dateorder_time_base_2_xz,
        "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AW4dateorder@time_base@2@XZ",
        Some(Arch::I386),
    ),
    // row 235
    (
        date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_aw4dateorder_time_base_2_xz,
        "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AW4dateorder@time_base@2@XZ",
        Some(Arch::Win64),
    ),
    // row 236
    (
        do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_aw4dateorder_time_base_2_xz,
        "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AW4dateorder@time_base@2@XZ",
        Some(Arch::Arm),
    ),
    // row 237
    (
        do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_aw4dateorder_time_base_2_xz,
        "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AW4dateorder@time_base@2@XZ",
        Some(Arch::I386),
    ),
    // row 238
    (
        do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_aw4dateorder_time_base_2_xz,
        "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AW4dateorder@time_base@2@XZ",
        Some(Arch::Win64),
    ),
    // row 239
    (
        do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z,
        "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z",
        Some(Arch::Arm),
    ),
    // row 240
    (
        do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z,
        "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z",
        Some(Arch::I386),
    ),
    // row 241
    (
        do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z,
        "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z",
        Some(Arch::Win64),
    ),
    // row 242
    (
        do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 243
    (
        do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 244
    (
        do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 245
    (
        do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 246
    (
        do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 247
    (
        do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 248
    (
        do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 249
    (
        do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 250
    (
        do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 251
    (
        do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 252
    (
        do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 253
    (
        do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 254
    (
        do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 255
    (
        do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 256
    (
        do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 257
    (
        done_agent_concurrency_iaa_nxz,
        "?done@agent@Concurrency@@IAA_NXZ",
        Some(Arch::Arm),
    ),
    // row 258
    (
        done_agent_concurrency_iae_nxz,
        "?done@agent@Concurrency@@IAE_NXZ",
        Some(Arch::I386),
    ),
    // row 259
    (
        done_agent_concurrency_ieaa_nxz,
        "?done@agent@Concurrency@@IEAA_NXZ",
        Some(Arch::Win64),
    ),
    // row 260
    (
        get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z,
        "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z",
        Some(Arch::Arm),
    ),
    // row 261
    (
        get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z,
        "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z",
        Some(Arch::I386),
    ),
    // row 262
    (
        get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z,
        "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z",
        Some(Arch::Win64),
    ),
    // row 263
    (
        get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbg4_z,
        "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBG4@Z",
        Some(Arch::Arm),
    ),
    // row 264
    (
        get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbg4_z,
        "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBG4@Z",
        Some(Arch::I386),
    ),
    // row 265
    (
        get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebg4_z,
        "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBG4@Z",
        Some(Arch::Win64),
    ),
    // row 266
    (
        get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 267
    (
        get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 268
    (
        get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 269
    (
        get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 270
    (
        get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 271
    (
        get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 272
    (
        get_new_handler_std_yap6axxzxz,
        "?get_new_handler@std@@YAP6AXXZXZ",
        None,
    ),
    // row 273
    (
        get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 274
    (
        get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 275
    (
        get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 276
    (
        get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 277
    (
        get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 278
    (
        get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 279
    (
        get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::Arm),
    ),
    // row 280
    (
        get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z,
        "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z",
        Some(Arch::I386),
    ),
    // row 281
    (
        get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
        "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z",
        Some(Arch::Win64),
    ),
    // row 282
    (
        move_basic_ios_du_char_traits_d_std_std_qaax_qav12_z,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAAX$$QAV12@@Z",
        Some(Arch::Arm),
    ),
    // row 283
    (
        move_basic_ios_du_char_traits_d_std_std_qaex_qav12_z,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAEX$$QAV12@@Z",
        Some(Arch::I386),
    ),
    // row 284
    (
        move_basic_ios_du_char_traits_d_std_std_qeaax_qeav12_z,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAX$$QEAV12@@Z",
        Some(Arch::Win64),
    ),
    // row 285
    (
        move_basic_ios_du_char_traits_d_std_std_qaaxaav12_z,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAAXAAV12@@Z",
        Some(Arch::Arm),
    ),
    // row 286
    (
        move_basic_ios_du_char_traits_d_std_std_qaexaav12_z,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAEXAAV12@@Z",
        Some(Arch::I386),
    ),
    // row 287
    (
        move_basic_ios_du_char_traits_d_std_std_qeaaxaeav12_z,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXAEAV12@@Z",
        Some(Arch::Win64),
    ),
    // row 288
    (
        move_basic_ios_gu_char_traits_g_std_std_qaax_qav12_z,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAAX$$QAV12@@Z",
        Some(Arch::Arm),
    ),
    // row 289
    (
        move_basic_ios_gu_char_traits_g_std_std_qaex_qav12_z,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAEX$$QAV12@@Z",
        Some(Arch::I386),
    ),
    // row 290
    (
        move_basic_ios_gu_char_traits_g_std_std_qeaax_qeav12_z,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAX$$QEAV12@@Z",
        Some(Arch::Win64),
    ),
    // row 291
    (
        move_basic_ios_gu_char_traits_g_std_std_qaaxaav12_z,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAAXAAV12@@Z",
        Some(Arch::Arm),
    ),
    // row 292
    (
        move_basic_ios_gu_char_traits_g_std_std_qaexaav12_z,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAEXAAV12@@Z",
        Some(Arch::I386),
    ),
    // row 293
    (
        move_basic_ios_gu_char_traits_g_std_std_qeaaxaeav12_z,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXAEAV12@@Z",
        Some(Arch::Win64),
    ),
    // row 294
    (
        move_basic_ios_wu_char_traits_w_std_std_qaax_qav12_z,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAAX$$QAV12@@Z",
        Some(Arch::Arm),
    ),
    // row 295
    (
        move_basic_ios_wu_char_traits_w_std_std_qaex_qav12_z,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAEX$$QAV12@@Z",
        Some(Arch::I386),
    ),
    // row 296
    (
        move_basic_ios_wu_char_traits_w_std_std_qeaax_qeav12_z,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAX$$QEAV12@@Z",
        Some(Arch::Win64),
    ),
    // row 297
    (
        move_basic_ios_wu_char_traits_w_std_std_qaaxaav12_z,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAAXAAV12@@Z",
        Some(Arch::Arm),
    ),
    // row 298
    (
        move_basic_ios_wu_char_traits_w_std_std_qaexaav12_z,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAEXAAV12@@Z",
        Some(Arch::I386),
    ),
    // row 299
    (
        move_basic_ios_wu_char_traits_w_std_std_qeaaxaeav12_z,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXAEAV12@@Z",
        Some(Arch::Win64),
    ),
    // row 300
    (
        set_rdbuf_basic_ios_du_char_traits_d_std_std_qaaxpav_basic_streambuf_du_char_traits_d_std_2_z,
        "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QAAXPAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z",
        Some(Arch::Arm),
    ),
    // row 301
    (
        set_rdbuf_basic_ios_du_char_traits_d_std_std_qaexpav_basic_streambuf_du_char_traits_d_std_2_z,
        "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QAEXPAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z",
        Some(Arch::I386),
    ),
    // row 302
    (
        set_rdbuf_basic_ios_du_char_traits_d_std_std_qeaaxpeav_basic_streambuf_du_char_traits_d_std_2_z,
        "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z",
        Some(Arch::Win64),
    ),
    // row 303
    (
        set_rdbuf_basic_ios_gu_char_traits_g_std_std_qaaxpav_basic_streambuf_gu_char_traits_g_std_2_z,
        "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QAAXPAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z",
        Some(Arch::Arm),
    ),
    // row 304
    (
        set_rdbuf_basic_ios_gu_char_traits_g_std_std_qaexpav_basic_streambuf_gu_char_traits_g_std_2_z,
        "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QAEXPAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z",
        Some(Arch::I386),
    ),
    // row 305
    (
        set_rdbuf_basic_ios_gu_char_traits_g_std_std_qeaaxpeav_basic_streambuf_gu_char_traits_g_std_2_z,
        "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXPEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z",
        Some(Arch::Win64),
    ),
    // row 306
    (
        set_rdbuf_basic_ios_wu_char_traits_w_std_std_qaaxpav_basic_streambuf_wu_char_traits_w_std_2_z,
        "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAAXPAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z",
        Some(Arch::Arm),
    ),
    // row 307
    (
        set_rdbuf_basic_ios_wu_char_traits_w_std_std_qaexpav_basic_streambuf_wu_char_traits_w_std_2_z,
        "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAEXPAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z",
        Some(Arch::I386),
    ),
    // row 308
    (
        set_rdbuf_basic_ios_wu_char_traits_w_std_std_qeaaxpeav_basic_streambuf_wu_char_traits_w_std_2_z,
        "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXPEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z",
        Some(Arch::Win64),
    ),
    // row 309
    (
        start_agent_concurrency_qaa_nxz,
        "?start@agent@Concurrency@@QAA_NXZ",
        Some(Arch::Arm),
    ),
    // row 310
    (
        start_agent_concurrency_qae_nxz,
        "?start@agent@Concurrency@@QAE_NXZ",
        Some(Arch::I386),
    ),
    // row 311
    (
        start_agent_concurrency_qeaa_nxz,
        "?start@agent@Concurrency@@QEAA_NXZ",
        Some(Arch::Win64),
    ),
    // row 312
    (
        status_agent_concurrency_qaa_aw4agent_status_2_xz,
        "?status@agent@Concurrency@@QAA?AW4agent_status@2@XZ",
        Some(Arch::Arm),
    ),
    // row 313
    (
        status_agent_concurrency_qae_aw4agent_status_2_xz,
        "?status@agent@Concurrency@@QAE?AW4agent_status@2@XZ",
        Some(Arch::I386),
    ),
    // row 314
    (
        status_agent_concurrency_qeaa_aw4agent_status_2_xz,
        "?status@agent@Concurrency@@QEAA?AW4agent_status@2@XZ",
        Some(Arch::Win64),
    ),
    // row 315
    (
        status_port_agent_concurrency_qaapav_isource_w4agent_status_concurrency_2_xz,
        "?status_port@agent@Concurrency@@QAAPAV?$ISource@W4agent_status@Concurrency@@@2@XZ",
        Some(Arch::Arm),
    ),
    // row 316
    (
        status_port_agent_concurrency_qaepav_isource_w4agent_status_concurrency_2_xz,
        "?status_port@agent@Concurrency@@QAEPAV?$ISource@W4agent_status@Concurrency@@@2@XZ",
        Some(Arch::I386),
    ),
    // row 317
    (
        status_port_agent_concurrency_qeaapeav_isource_w4agent_status_concurrency_2_xz,
        "?status_port@agent@Concurrency@@QEAAPEAV?$ISource@W4agent_status@Concurrency@@@2@XZ",
        Some(Arch::Win64),
    ),
    // row 318
    (
        wait_agent_concurrency_sa_aw4agent_status_2_pav12_i_z,
        "?wait@agent@Concurrency@@SA?AW4agent_status@2@PAV12@I@Z",
        Some(Arch::Win32),
    ),
    // row 319
    (
        wait_agent_concurrency_sa_aw4agent_status_2_peav12_i_z,
        "?wait@agent@Concurrency@@SA?AW4agent_status@2@PEAV12@I@Z",
        Some(Arch::Win64),
    ),
    // row 320
    (
        wait_for_all_agent_concurrency_saxipapav12_paw4agent_status_2_i_z,
        "?wait_for_all@agent@Concurrency@@SAXIPAPAV12@PAW4agent_status@2@I@Z",
        Some(Arch::Win32),
    ),
    // row 321
    (
        wait_for_all_agent_concurrency_sax_kpeapeav12_peaw4agent_status_2_i_z,
        "?wait_for_all@agent@Concurrency@@SAX_KPEAPEAV12@PEAW4agent_status@2@I@Z",
        Some(Arch::Win64),
    ),
    // row 322
    (
        wait_for_one_agent_concurrency_saxipapav12_aaw4agent_status_2_aaii_z,
        "?wait_for_one@agent@Concurrency@@SAXIPAPAV12@AAW4agent_status@2@AAII@Z",
        Some(Arch::Win32),
    ),
    // row 323
    (
        wait_for_one_agent_concurrency_sax_kpeapeav12_aeaw4agent_status_2_aea_ki_z,
        "?wait_for_one@agent@Concurrency@@SAX_KPEAPEAV12@AEAW4agent_status@2@AEA_KI@Z",
        Some(Arch::Win64),
    ),
    // row 324
    (
        _dint,
        "_Dint",
        None,
    ),
    // row 325
    (
        _dnorm,
        "_Dnorm",
        None,
    ),
    // row 326
    (
        _dtento,
        "_Dtento",
        None,
    ),
    // row 327
    (
        _dunscale,
        "_Dunscale",
        None,
    ),
    // row 328
    (
        _fdint,
        "_FDint",
        None,
    ),
    // row 329
    (
        _fdnorm,
        "_FDnorm",
        None,
    ),
    // row 330
    (
        _fdtento,
        "_FDtento",
        None,
    ),
    // row 331
    (
        _fdunscale,
        "_FDunscale",
        None,
    ),
    // row 332
    (
        _fplsw,
        "_FPlsw",
        None,
    ),
    // row 333
    (
        _fpmsw,
        "_FPmsw",
        None,
    ),
    // row 334
    (
        _fxp_addh,
        "_FXp_addh",
        None,
    ),
    // row 335
    (
        _fxp_addx,
        "_FXp_addx",
        None,
    ),
    // row 336
    (
        _fxp_getw,
        "_FXp_getw",
        None,
    ),
    // row 337
    (
        _fxp_invx,
        "_FXp_invx",
        None,
    ),
    // row 338
    (
        _fxp_ldexpx,
        "_FXp_ldexpx",
        None,
    ),
    // row 339
    (
        _fxp_movx,
        "_FXp_movx",
        None,
    ),
    // row 340
    (
        _fxp_mulh,
        "_FXp_mulh",
        None,
    ),
    // row 341
    (
        _fxp_mulx,
        "_FXp_mulx",
        None,
    ),
    // row 342
    (
        _fxp_setn,
        "_FXp_setn",
        None,
    ),
    // row 343
    (
        _fxp_setw,
        "_FXp_setw",
        None,
    ),
    // row 344
    (
        _fxp_sqrtx,
        "_FXp_sqrtx",
        None,
    ),
    // row 345
    (
        _fxp_subx,
        "_FXp_subx",
        None,
    ),
    // row 346
    (
        _ldint,
        "_LDint",
        None,
    ),
    // row 347
    (
        _ldtento,
        "_LDtento",
        None,
    ),
    // row 348
    (
        _ldunscale,
        "_LDunscale",
        None,
    ),
    // row 349
    (
        _lexp,
        "_LExp",
        None,
    ),
    // row 350
    (
        _lplsw,
        "_LPlsw",
        None,
    ),
    // row 351
    (
        _lpmsw,
        "_LPmsw",
        None,
    ),
    // row 352
    (
        _lpoly,
        "_LPoly",
        None,
    ),
    // row 353
    (
        _lxp_addh,
        "_LXp_addh",
        None,
    ),
    // row 354
    (
        _lxp_addx,
        "_LXp_addx",
        None,
    ),
    // row 355
    (
        _lxp_getw,
        "_LXp_getw",
        None,
    ),
    // row 356
    (
        _lxp_invx,
        "_LXp_invx",
        None,
    ),
    // row 357
    (
        _lxp_ldexpx,
        "_LXp_ldexpx",
        None,
    ),
    // row 358
    (
        _lxp_movx,
        "_LXp_movx",
        None,
    ),
    // row 359
    (
        _lxp_mulh,
        "_LXp_mulh",
        None,
    ),
    // row 360
    (
        _lxp_mulx,
        "_LXp_mulx",
        None,
    ),
    // row 361
    (
        _lxp_setn,
        "_LXp_setn",
        None,
    ),
    // row 362
    (
        _lxp_setw,
        "_LXp_setw",
        None,
    ),
    // row 363
    (
        _lxp_sqrtx,
        "_LXp_sqrtx",
        None,
    ),
    // row 364
    (
        _lxp_subx,
        "_LXp_subx",
        None,
    ),
    // row 365
    (
        _mtx_timedlock,
        "_Mtx_timedlock",
        None,
    ),
    // row 366
    (
        _mtxdst,
        "_Mtxdst",
        None,
    ),
    // row 367
    (
        _mtxinit,
        "_Mtxinit",
        None,
    ),
    // row 368
    (
        _mtxlock,
        "_Mtxlock",
        None,
    ),
    // row 369
    (
        _mtxunlock,
        "_Mtxunlock",
        None,
    ),
    // row 370
    (
        _once,
        "_Once",
        None,
    ),
    // row 371
    (
        _plsw,
        "_Plsw",
        None,
    ),
    // row 372
    (
        _pmsw,
        "_Pmsw",
        None,
    ),
    // row 373
    (
        _poly,
        "_Poly",
        None,
    ),
    // row 374
    (
        _stoflt,
        "_Stoflt",
        None,
    ),
    // row 375
    (
        _stopfx,
        "_Stopfx",
        None,
    ),
    // row 376
    (
        _stoxflt,
        "_Stoxflt",
        None,
    ),
    // row 377
    (
        _thrd_abort,
        "_Thrd_abort",
        None,
    ),
    // row 378
    (
        _thrd_exit,
        "_Thrd_exit",
        None,
    ),
    // row 379
    (
        _tss_create,
        "_Tss_create",
        None,
    ),
    // row 380
    (
        _tss_delete,
        "_Tss_delete",
        None,
    ),
    // row 381
    (
        _tss_get,
        "_Tss_get",
        None,
    ),
    // row 382
    (
        _tss_set,
        "_Tss_set",
        None,
    ),
    // row 383
    (
        _xp_addh,
        "_Xp_addh",
        None,
    ),
    // row 384
    (
        _xp_addx,
        "_Xp_addx",
        None,
    ),
    // row 385
    (
        _xp_getw,
        "_Xp_getw",
        None,
    ),
    // row 386
    (
        _xp_invx,
        "_Xp_invx",
        None,
    ),
    // row 387
    (
        _xp_ldexpx,
        "_Xp_ldexpx",
        None,
    ),
    // row 388
    (
        _xp_movx,
        "_Xp_movx",
        None,
    ),
    // row 389
    (
        _xp_mulh,
        "_Xp_mulh",
        None,
    ),
    // row 390
    (
        _xp_mulx,
        "_Xp_mulx",
        None,
    ),
    // row 391
    (
        _xp_setn,
        "_Xp_setn",
        None,
    ),
    // row 392
    (
        _xp_setw,
        "_Xp_setw",
        None,
    ),
    // row 393
    (
        _xp_sqrtx,
        "_Xp_sqrtx",
        None,
    ),
    // row 394
    (
        _xp_subx,
        "_Xp_subx",
        None,
    ),
    // row 395
    (
        _wcrtomb_lk,
        "__Wcrtomb_lk",
        None,
    ),
];

#[test]
fn oracle_export_table_summary_matches_harness() {
    assert_eq!(EXPORTS.len(), ORACLE_TOTAL);
    for (kind, expected) in ORACLE_KIND_COUNTS {
        assert_eq!(
            EXPORTS.iter().filter(|export| export.kind == kind).count(),
            expected
        );
    }
    for (arch, expected) in ORACLE_ARCH_COUNTS {
        assert_eq!(
            EXPORTS.iter().filter(|export| export.arch == arch).count(),
            expected
        );
    }
    let mismatches = EXPORTS
        .iter()
        .filter(|export| !forward_matches_spec(export))
        .count();
    assert_eq!(mismatches, ORACLE_FORWARD_MISMATCHES);
    assert_eq!(table_digest(), ORACLE_DIGEST);
}

/// A non-stub export forwards to `msvcp120.<name>`, and a stub export does not forward.
fn forward_matches_spec(export: &Export) -> bool {
    match export.kind {
        ExportKind::Stub => forward(export.name).is_none(),
        _ => {
            forward(export.name)
                == Some(Forward {
                    module: "msvcp120",
                    function: export.name,
                })
        }
    }
}

/// FNV-1a over the `kind|arch|name|args` line of each export, in spec order. The harness hashes
/// the same bytes, with `none` for a missing `-arch=`.
fn table_digest() -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for export in EXPORTS.iter() {
        let line = format!(
            "{}|{}|{}|{}\n",
            kind_text(export.kind),
            arch_text(export.arch),
            export.name,
            export.args
        );
        for byte in line.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

fn kind_text(kind: ExportKind) -> &'static str {
    match kind {
        ExportKind::Cdecl => "cdecl",
        ExportKind::Thiscall => "thiscall",
        ExportKind::Extern => "extern",
        ExportKind::Stub => "stub",
    }
}

fn arch_text(arch: Option<Arch>) -> &'static str {
    match arch {
        None => "none",
        Some(Arch::Arm) => "arm",
        Some(Arch::I386) => "i386",
        Some(Arch::Win32) => "win32",
        Some(Arch::Win64) => "win64",
    }
}

#[test]
fn oracle_stub_rows_match_harness() {
    assert_eq!(STUB_ROWS.len(), 395);
    for (index, &(helper, function, arch)) in STUB_ROWS.iter().enumerate() {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: ORACLE_MODULE,
                function,
            },
            "row {:03}",
            index + 1
        );
        assert_eq!(
            find_export(function),
            Some(Export {
                name: function,
                kind: ExportKind::Stub,
                arch,
                args: "",
            }),
            "row {:03}",
            index + 1
        );
    }
}
