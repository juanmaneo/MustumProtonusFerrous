//! Helpers for the `@ stub` exports of `dlls/msvcp120_app/msvcp120_app.spec`, one per export line.
//!
//! Each helper is named after its export: the export name in lower case, without the leading
//! `?`, with each run of non-alphanumeric characters replaced by `_`. The leading underscores are
//! kept as they are, so `_Stodx` and `__Stodx` get different names. Every `-arch=` variant is
//! present, because the spec lists each one.

use super::stub::{wine_spec_unimplemented_stub, StubException, MSVCP120_APP_DLL_NAME};

/// `@ stub -arch=arm` export.
pub fn _getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_iaax_wabv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IAAX_WABV_Locinfo@1@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_iaex_wabv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IAEX_WABV_Locinfo@1@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_ieaax_waebv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z")
}

/// `@ stub -arch=arm` export.
pub fn _getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaax_wabv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAAX_WABV_Locinfo@1@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaex_wabv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAEX_WABV_Locinfo@1@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaax_waebv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z")
}

/// `@ stub -arch=arm` export.
pub fn _getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_iaax_wabv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IAAX_WABV_Locinfo@1@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_iaex_wabv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IAEX_WABV_Locinfo@1@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_ieaax_waebv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z")
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_iostream_du_char_traits_d_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_iostream_du_char_traits_d_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_iostream_du_char_traits_d_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_iostream_gu_char_traits_g_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_iostream_gu_char_traits_g_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_iostream_gu_char_traits_g_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_iostream_wu_char_traits_w_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_iostream_wu_char_traits_w_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_iostream_wu_char_traits_w_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_istream_du_char_traits_d_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_istream_du_char_traits_d_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_istream_du_char_traits_d_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_istream_gu_char_traits_g_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_istream_gu_char_traits_g_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_istream_gu_char_traits_g_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_istream_wu_char_traits_w_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_istream_wu_char_traits_w_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_istream_wu_char_traits_w_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_ostream_du_char_traits_d_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_ostream_du_char_traits_d_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_ostream_du_char_traits_d_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_ostream_gu_char_traits_g_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_ostream_gu_char_traits_g_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_ostream_gu_char_traits_g_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_ostream_wu_char_traits_w_std_std_iaa_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAA@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_ostream_wu_char_traits_w_std_std_iae_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAE@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_ostream_wu_char_traits_w_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_streambuf_du_char_traits_d_std_std_iaa_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAA@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_streambuf_du_char_traits_d_std_std_iae_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAE@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_streambuf_du_char_traits_d_std_std_ieaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_streambuf_gu_char_traits_g_std_std_iaa_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAA@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_streambuf_gu_char_traits_g_std_std_iae_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAE@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_streambuf_gu_char_traits_g_std_std_ieaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_basic_streambuf_wu_char_traits_w_std_std_iaa_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAA@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_basic_streambuf_wu_char_traits_w_std_std_iae_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAE@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_basic_streambuf_wu_char_traits_w_std_std_ieaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaa_pbdi_z() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAA@PBDI@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iae_pbdi_z() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAE@PBDI@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaa_pebd_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAA@PEBD_K@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaa_abv_locinfo_1_i_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAA@ABV_Locinfo@1@I@Z")
}

/// `@ stub -arch=i386` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qae_abv_locinfo_1_i_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@ABV_Locinfo@1@I@Z")
}

/// `@ stub -arch=win64` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_aebv_locinfo_1_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z")
}

/// `@ stub -arch=arm` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaa_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAA@I@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qae_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAE@I@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_k_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0_concurrent_queue_iterator_base_v4_details_concurrency_iaa_abv_concurrent_queue_base_v4_12_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAA@ABV_Concurrent_queue_base_v4@12@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _0_concurrent_queue_iterator_base_v4_details_concurrency_iae_abv_concurrent_queue_base_v4_12_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAE@ABV_Concurrent_queue_base_v4@12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _0_concurrent_queue_iterator_base_v4_details_concurrency_ieaa_aebv_concurrent_queue_base_v4_12_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAA@AEBV_Concurrent_queue_base_v4@12@@Z")
}

/// `@ stub -arch=arm` export.
pub fn _0_facet_base_std_qaa_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Facet_base@std@@QAA@ABV01@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _0_facet_base_std_qae_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Facet_base@std@@QAE@ABV01@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _0_facet_base_std_qeaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Facet_base@std@@QEAA@AEBV01@@Z")
}

/// `@ stub -arch=arm` export.
pub fn _0_facet_base_std_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Facet_base@std@@QAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _0_facet_base_std_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Facet_base@std@@QAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _0_facet_base_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_Facet_base@std@@QEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _0_ushinit_std_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_UShinit@std@@QAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _0_ushinit_std_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_UShinit@std@@QAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _0_ushinit_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0_UShinit@std@@QEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _0agent_concurrency_qaa_aavschedulegroup_1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0agent@Concurrency@@QAA@AAVScheduleGroup@1@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0agent_concurrency_qae_aavschedulegroup_1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0agent@Concurrency@@QAE@AAVScheduleGroup@1@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0agent_concurrency_qeaa_aeavschedulegroup_1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0agent@Concurrency@@QEAA@AEAVScheduleGroup@1@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0agent_concurrency_qaa_aavscheduler_1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0agent@Concurrency@@QAA@AAVScheduler@1@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _0agent_concurrency_qae_aavscheduler_1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0agent@Concurrency@@QAE@AAVScheduler@1@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _0agent_concurrency_qeaa_aeavscheduler_1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??0agent@Concurrency@@QEAA@AEAVScheduler@1@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _0agent_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0agent@Concurrency@@QAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _0agent_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0agent@Concurrency@@QAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _0agent_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0agent@Concurrency@@QEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _0time_base_std_qaa_i_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0time_base@std@@QAA@I@Z")
}

/// `@ stub -arch=i386` export.
pub fn _0time_base_std_qae_i_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0time_base@std@@QAE@I@Z")
}

/// `@ stub -arch=win64` export.
pub fn _0time_base_std_qeaa_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??0time_base@std@@QEAA@_K@Z")
}

/// `@ stub -arch=arm` export.
pub fn _1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_maa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MAA@XZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn _1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MAE@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEAA@XZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _1_concurrent_queue_iterator_base_v4_details_concurrency_iaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??1_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAA@XZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn _1_concurrent_queue_iterator_base_v4_details_concurrency_iae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??1_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAE@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _1_concurrent_queue_iterator_base_v4_details_concurrency_ieaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??1_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAA@XZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _1_facet_base_std_uaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1_Facet_base@std@@UAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _1_facet_base_std_uae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1_Facet_base@std@@UAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _1_facet_base_std_ueaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1_Facet_base@std@@UEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _1_ushinit_std_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1_UShinit@std@@QAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _1_ushinit_std_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1_UShinit@std@@QAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _1_ushinit_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1_UShinit@std@@QEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _1agent_concurrency_uaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1agent@Concurrency@@UAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _1agent_concurrency_uae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1agent@Concurrency@@UAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _1agent_concurrency_ueaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1agent@Concurrency@@UEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _1time_base_std_uaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1time_base@std@@UAA@XZ")
}

/// `@ stub -arch=i386` export.
pub fn _1time_base_std_uae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1time_base@std@@UAE@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _1time_base_std_ueaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??1time_base@std@@UEAA@XZ")
}

/// `@ stub -arch=arm` export.
pub fn _4_iosb_h_std_qaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??4?$_Iosb@H@std@@QAAAAV01@ABV01@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _4_iosb_h_std_qaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??4?$_Iosb@H@std@@QAEAAV01@ABV01@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _4_iosb_h_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$_Iosb@H@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_iostream_du_char_traits_d_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_iostream_du_char_traits_d_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_iostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_iostream_gu_char_traits_g_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_iostream_gu_char_traits_g_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_iostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_iostream_wu_char_traits_w_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_iostream_wu_char_traits_w_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_iostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_istream_du_char_traits_d_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_istream_du_char_traits_d_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_istream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_istream_gu_char_traits_g_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_istream_gu_char_traits_g_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_istream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_istream_wu_char_traits_w_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_istream_wu_char_traits_w_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_istream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_ostream_du_char_traits_d_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_ostream_du_char_traits_d_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_ostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_ostream_gu_char_traits_g_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_ostream_gu_char_traits_g_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_ostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_ostream_wu_char_traits_w_std_std_iaaaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAAAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_ostream_wu_char_traits_w_std_std_iaeaav01_qav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IAEAAV01@$$QAV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_ostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_streambuf_du_char_traits_d_std_std_iaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAAAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_streambuf_du_char_traits_d_std_std_iaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IAEAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_streambuf_du_char_traits_d_std_std_ieaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_streambuf_gu_char_traits_g_std_std_iaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAAAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_streambuf_gu_char_traits_g_std_std_iaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IAEAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_streambuf_gu_char_traits_g_std_std_ieaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_basic_streambuf_wu_char_traits_w_std_std_iaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAAAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_basic_streambuf_wu_char_traits_w_std_std_iaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IAEAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_basic_streambuf_wu_char_traits_w_std_std_ieaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_facet_base_std_qaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_Facet_base@std@@QAAAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_facet_base_std_qaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_Facet_base@std@@QAEAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_facet_base_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_Facet_base@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_init_locks_std_qaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_Init_locks@std@@QAAAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _4_init_locks_std_qaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_Init_locks@std@@QAEAAV01@ABV01@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _4_init_locks_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_Init_locks@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _4_ushinit_std_qaaaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??4_UShinit@std@@QAAAAV01@ABV01@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _4_ushinit_std_qaeaav01_abv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??4_UShinit@std@@QAEAAV01@ABV01@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _4_ushinit_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??4_UShinit@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _5_basic_istream_gu_char_traits_g_std_std_qaaaav01_aag_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QAAAAV01@AAG@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _5_basic_istream_gu_char_traits_g_std_std_qaeaav01_aag_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QAEAAV01@AAG@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _5_basic_istream_gu_char_traits_g_std_std_qeaaaeav01_aeag_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAG@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAAXXZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QAEXXZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _ftime_base_std_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??_Ftime_base@std@@QAAXXZ")
}

/// `@ stub -arch=i386` export.
pub fn _ftime_base_std_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??_Ftime_base@std@@QAEXXZ")
}

/// `@ stub -arch=win64` export.
pub fn _ftime_base_std_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "??_Ftime_base@std@@QEAAXXZ")
}

/// `@ stub -arch=win32` export.
pub fn nfs_allocate_details_concurrency_yapaxiipax_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?NFS_Allocate@details@Concurrency@@YAPAXIIPAX@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn nfs_allocate_details_concurrency_yapeax_k0peax_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?NFS_Allocate@details@Concurrency@@YAPEAX_K0PEAX@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn nfs_free_details_concurrency_yaxpax_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?NFS_Free@details@Concurrency@@YAXPAX@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn nfs_free_details_concurrency_yaxpeax_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?NFS_Free@details@Concurrency@@YAXPEAX@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn nfs_getlinesize_details_concurrency_yaixz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?NFS_GetLineSize@details@Concurrency@@YAIXZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn nfs_getlinesize_details_concurrency_ya_kxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?NFS_GetLineSize@details@Concurrency@@YA_KXZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _advance_concurrent_queue_iterator_base_v4_details_concurrency_iaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Advance@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAAXXZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn _advance_concurrent_queue_iterator_base_v4_details_concurrency_iaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Advance@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAEXXZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _advance_concurrent_queue_iterator_base_v4_details_concurrency_ieaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Advance@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAAXXZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _assign_concurrent_queue_iterator_base_v4_details_concurrency_iaaxabv123_z() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Assign@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAAXABV123@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn _assign_concurrent_queue_iterator_base_v4_details_concurrency_iaexabv123_z() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Assign@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IAEXABV123@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _assign_concurrent_queue_iterator_base_v4_details_concurrency_ieaaxaebv123_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Assign@_Concurrent_queue_iterator_base_v4@details@Concurrency@@IEAAXAEBV123@@Z",
    )
}

/// `@ stub` export.
pub fn _atexit_yaxp6axxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Atexit@@YAXP6AXXZ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _future_error_map_std_yapbdh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Future_error_map@std@@YAPBDH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _future_error_map_std_yapebdh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Future_error_map@std@@YAPEBDH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getcat_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_saipapbvfacet_locale_2_pbv42_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SAIPAPBVfacet@locale@2@PBV42@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getcat_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_sa_kpeapebvfacet_locale_2_pebv42_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_2_1aavios_base_2_pah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@1AAVios_base@2@PAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_2_1aeavios_base_2_peah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@1AEAVios_base@2@PEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_2_1aavios_base_2_pah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@1AAVios_base@2@PAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_2_1aeavios_base_2_peah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@1AEAVios_base@2@PEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_2_1aavios_base_2_pah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@1AAVios_base@2@PAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_2_1aeavios_base_2_peah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@1AEAVios_base@2@PEAH@Z")
}

/// `@ stub -arch=arm` export.
pub fn _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBD@Z")
}

/// `@ stub -arch=i386` export.
pub fn _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ibe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBD@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBD@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getint_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahaav_istreambuf_iterator_du_char_traits_d_std_2_0hhaahabv_ctype_d_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@0HHAAHABV?$ctype@D@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getint_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahaeav_istreambuf_iterator_du_char_traits_d_std_2_0hhaeahaebv_ctype_d_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@0HHAEAHAEBV?$ctype@D@2@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getint_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahaav_istreambuf_iterator_gu_char_traits_g_std_2_0hhaahabv_ctype_g_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@0HHAAHABV?$ctype@G@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getint_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahaeav_istreambuf_iterator_gu_char_traits_g_std_2_0hhaeahaebv_ctype_g_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@0HHAEAHAEBV?$ctype@G@2@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getint_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahaav_istreambuf_iterator_wu_char_traits_w_std_2_0hhaahabv_ctype_w_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getint@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@0HHAAHABV?$ctype@_W@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getint_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahaeav_istreambuf_iterator_wu_char_traits_w_std_2_0hhaeahaebv_ctype_w_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Getint@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@0HHAEAHAEBV?$ctype@_W@2@@Z")
}

/// `@ stub -arch=arm` export.
pub fn _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaaxabv_locinfo_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAAXABV_Locinfo@2@@Z")
}

/// `@ stub -arch=i386` export.
pub fn _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_iaexabv_locinfo_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IAEXABV_Locinfo@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaaxaebv_locinfo_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mp_add_std_yaxqa_k_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Add@std@@YAXQA_K_K@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mp_add_std_yaxqea_k_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Add@std@@YAXQEA_K_K@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mp_get_std_ya_kqa_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Get@std@@YA_KQA_K@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mp_get_std_ya_kqea_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Get@std@@YA_KQEA_K@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mp_mul_std_yaxqa_k_k1_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Mul@std@@YAXQA_K_K1@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mp_mul_std_yaxqea_k_k1_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Mul@std@@YAXQEA_K_K1@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mp_rem_std_yaxqa_k_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Rem@std@@YAXQA_K_K@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mp_rem_std_yaxqea_k_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_MP_Rem@std@@YAXQEA_K_K@Z")
}

/// `@ stub -arch=win32` export.
pub fn _rng_abort_std_yaxpbd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Rng_abort@std@@YAXPBD@Z")
}

/// `@ stub -arch=win64` export.
pub fn _rng_abort_std_yaxpebd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Rng_abort@std@@YAXPEBD@Z")
}

/// `@ stub -arch=win32` export.
pub fn _src_1_getffldx_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_3_1aavios_base_3_pah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AAVios_base@3@PAH@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_1_getffldx_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_1_getffldx_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_3_1aavios_base_3_pah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AAVios_base@3@PAH@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_1_getffldx_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_1_getffldx_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_3_1aavios_base_3_pah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AAVios_base@3@PAH@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_1_getffldx_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_1_getifld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_3_1habvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1HABVlocale@3@@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_1_getifld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1haebvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1HAEBVlocale@3@@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_1_getifld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_3_1habvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1HABVlocale@3@@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_1_getifld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1haebvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1HAEBVlocale@3@@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_1_getifld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_3_1habvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1HABVlocale@3@@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_1_getifld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1haebvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1HAEBVlocale@3@@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_3_getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_abahpadaav_istreambuf_iterator_du_char_traits_d_std_3_1aavios_base_3_pah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?3??_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AAVios_base@3@PAH@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_3_getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?3??_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_3_getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_abahpadaav_istreambuf_iterator_gu_char_traits_g_std_3_1aavios_base_3_pah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?3??_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AAVios_base@3@PAH@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_3_getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?3??_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// `@ stub -arch=win32` export.
pub fn _src_3_getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_abahpadaav_istreambuf_iterator_wu_char_traits_w_std_3_1aavios_base_3_pah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?3??_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@ABAHPADAAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AAVios_base@3@PAH@Z@4QBDB")
}

/// `@ stub -arch=win64` export.
pub fn _src_3_getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Src@?3??_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// `@ stub` export.
pub fn _throw_cpp_error_std_yaxh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Throw_Cpp_error@std@@YAXH@Z")
}

/// `@ stub` export.
pub fn _throw_lock_error_threads_stdext_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Throw_lock_error@threads@stdext@@YAXXZ",
    )
}

/// `@ stub` export.
pub fn _throw_resource_error_threads_stdext_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Throw_resource_error@threads@stdext@@YAXXZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aaaxxz() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AAAXXZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aaexxz() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AAEXXZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aeaaxxz() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEAAXXZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn _w_gettnames_locinfo_std_qba_av_timevec_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_W_Gettnames@_Locinfo@std@@QBA?AV_Timevec@2@XZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn _w_gettnames_locinfo_std_qbe_av_timevec_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_W_Gettnames@_Locinfo@std@@QBE?AV_Timevec@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _w_gettnames_locinfo_std_qeba_av_timevec_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?_W_Gettnames@_Locinfo@std@@QEBA?AV_Timevec@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _xoverflow_error_std_yaxpbd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Xoverflow_error@std@@YAXPBD@Z")
}

/// `@ stub -arch=win64` export.
pub fn _xoverflow_error_std_yaxpebd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?_Xoverflow_error@std@@YAXPEBD@Z")
}

/// `@ stub -arch=arm` export.
pub fn cancel_agent_concurrency_qaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?cancel@agent@Concurrency@@QAA_NXZ")
}

/// `@ stub -arch=i386` export.
pub fn cancel_agent_concurrency_qae_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?cancel@agent@Concurrency@@QAE_NXZ")
}

/// `@ stub -arch=win64` export.
pub fn cancel_agent_concurrency_qeaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?cancel@agent@Concurrency@@QEAA_NXZ")
}

/// `@ stub -arch=win32` export.
pub fn classic_table_ctype_d_std_sapbfxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?classic_table@?$ctype@D@std@@SAPBFXZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn classic_table_ctype_d_std_sapebfxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?classic_table@?$ctype@D@std@@SAPEBFXZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AW4dateorder@time_base@2@XZ")
}

/// `@ stub -arch=i386` export.
pub fn date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AW4dateorder@time_base@2@XZ")
}

/// `@ stub -arch=win64` export.
pub fn date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AW4dateorder@time_base@2@XZ")
}

/// `@ stub -arch=arm` export.
pub fn do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AW4dateorder@time_base@2@XZ")
}

/// `@ stub -arch=i386` export.
pub fn do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AW4dateorder@time_base@2@XZ")
}

/// `@ stub -arch=win64` export.
pub fn do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AW4dateorder@time_base@2@XZ")
}

/// `@ stub -arch=arm` export.
pub fn do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z")
}

/// `@ stub -arch=i386` export.
pub fn do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z")
}

/// `@ stub -arch=win64` export.
pub fn do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z")
}

/// `@ stub -arch=arm` export.
pub fn do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_mbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn done_agent_concurrency_iaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?done@agent@Concurrency@@IAA_NXZ")
}

/// `@ stub -arch=i386` export.
pub fn done_agent_concurrency_iae_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?done@agent@Concurrency@@IAE_NXZ")
}

/// `@ stub -arch=win64` export.
pub fn done_agent_concurrency_ieaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?done@agent@Concurrency@@IEAA_NXZ")
}

/// `@ stub -arch=arm` export.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@DD@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z")
}

/// `@ stub -arch=arm` export.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbg4_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBG4@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_pbg4_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@PBG4@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebg4_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBG4@Z")
}

/// `@ stub -arch=arm` export.
pub fn get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub` export.
pub fn get_new_handler_std_yap6axxzxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_new_handler@std@@YAP6AXXZXZ")
}

/// `@ stub -arch=arm` export.
pub fn get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=i386` export.
pub fn get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qbe_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aavios_base_2_aahpautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QBE?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AAVios_base@2@AAHPAUtm@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// `@ stub -arch=arm` export.
pub fn move_basic_ios_du_char_traits_d_std_std_qaax_qav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAAX$$QAV12@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn move_basic_ios_du_char_traits_d_std_std_qaex_qav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAEX$$QAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn move_basic_ios_du_char_traits_d_std_std_qeaax_qeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAX$$QEAV12@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn move_basic_ios_du_char_traits_d_std_std_qaaxaav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAAXAAV12@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn move_basic_ios_du_char_traits_d_std_std_qaexaav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QAEXAAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn move_basic_ios_du_char_traits_d_std_std_qeaaxaeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXAEAV12@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn move_basic_ios_gu_char_traits_g_std_std_qaax_qav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAAX$$QAV12@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn move_basic_ios_gu_char_traits_g_std_std_qaex_qav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAEX$$QAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn move_basic_ios_gu_char_traits_g_std_std_qeaax_qeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAX$$QEAV12@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn move_basic_ios_gu_char_traits_g_std_std_qaaxaav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAAXAAV12@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn move_basic_ios_gu_char_traits_g_std_std_qaexaav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QAEXAAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn move_basic_ios_gu_char_traits_g_std_std_qeaaxaeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXAEAV12@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn move_basic_ios_wu_char_traits_w_std_std_qaax_qav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAAX$$QAV12@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn move_basic_ios_wu_char_traits_w_std_std_qaex_qav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAEX$$QAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn move_basic_ios_wu_char_traits_w_std_std_qeaax_qeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAX$$QEAV12@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn move_basic_ios_wu_char_traits_w_std_std_qaaxaav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAAXAAV12@@Z",
    )
}

/// `@ stub -arch=i386` export.
pub fn move_basic_ios_wu_char_traits_w_std_std_qaexaav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAEXAAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn move_basic_ios_wu_char_traits_w_std_std_qeaaxaeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXAEAV12@@Z",
    )
}

/// `@ stub -arch=arm` export.
pub fn set_rdbuf_basic_ios_du_char_traits_d_std_std_qaaxpav_basic_streambuf_du_char_traits_d_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QAAXPAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z")
}

/// `@ stub -arch=i386` export.
pub fn set_rdbuf_basic_ios_du_char_traits_d_std_std_qaexpav_basic_streambuf_du_char_traits_d_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QAEXPAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn set_rdbuf_basic_ios_du_char_traits_d_std_std_qeaaxpeav_basic_streambuf_du_char_traits_d_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z")
}

/// `@ stub -arch=arm` export.
pub fn set_rdbuf_basic_ios_gu_char_traits_g_std_std_qaaxpav_basic_streambuf_gu_char_traits_g_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QAAXPAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z")
}

/// `@ stub -arch=i386` export.
pub fn set_rdbuf_basic_ios_gu_char_traits_g_std_std_qaexpav_basic_streambuf_gu_char_traits_g_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QAEXPAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn set_rdbuf_basic_ios_gu_char_traits_g_std_std_qeaaxpeav_basic_streambuf_gu_char_traits_g_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXPEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z")
}

/// `@ stub -arch=arm` export.
pub fn set_rdbuf_basic_ios_wu_char_traits_w_std_std_qaaxpav_basic_streambuf_wu_char_traits_w_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAAXPAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z")
}

/// `@ stub -arch=i386` export.
pub fn set_rdbuf_basic_ios_wu_char_traits_w_std_std_qaexpav_basic_streambuf_wu_char_traits_w_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QAEXPAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z")
}

/// `@ stub -arch=win64` export.
pub fn set_rdbuf_basic_ios_wu_char_traits_w_std_std_qeaaxpeav_basic_streambuf_wu_char_traits_w_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXPEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z")
}

/// `@ stub -arch=arm` export.
pub fn start_agent_concurrency_qaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?start@agent@Concurrency@@QAA_NXZ")
}

/// `@ stub -arch=i386` export.
pub fn start_agent_concurrency_qae_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?start@agent@Concurrency@@QAE_NXZ")
}

/// `@ stub -arch=win64` export.
pub fn start_agent_concurrency_qeaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "?start@agent@Concurrency@@QEAA_NXZ")
}

/// `@ stub -arch=arm` export.
pub fn status_agent_concurrency_qaa_aw4agent_status_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?status@agent@Concurrency@@QAA?AW4agent_status@2@XZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn status_agent_concurrency_qae_aw4agent_status_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?status@agent@Concurrency@@QAE?AW4agent_status@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn status_agent_concurrency_qeaa_aw4agent_status_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?status@agent@Concurrency@@QEAA?AW4agent_status@2@XZ",
    )
}

/// `@ stub -arch=arm` export.
pub fn status_port_agent_concurrency_qaapav_isource_w4agent_status_concurrency_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?status_port@agent@Concurrency@@QAAPAV?$ISource@W4agent_status@Concurrency@@@2@XZ",
    )
}

/// `@ stub -arch=i386` export.
pub fn status_port_agent_concurrency_qaepav_isource_w4agent_status_concurrency_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?status_port@agent@Concurrency@@QAEPAV?$ISource@W4agent_status@Concurrency@@@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn status_port_agent_concurrency_qeaapeav_isource_w4agent_status_concurrency_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?status_port@agent@Concurrency@@QEAAPEAV?$ISource@W4agent_status@Concurrency@@@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn wait_agent_concurrency_sa_aw4agent_status_2_pav12_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?wait@agent@Concurrency@@SA?AW4agent_status@2@PAV12@I@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn wait_agent_concurrency_sa_aw4agent_status_2_peav12_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?wait@agent@Concurrency@@SA?AW4agent_status@2@PEAV12@I@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn wait_for_all_agent_concurrency_saxipapav12_paw4agent_status_2_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?wait_for_all@agent@Concurrency@@SAXIPAPAV12@PAW4agent_status@2@I@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn wait_for_all_agent_concurrency_sax_kpeapeav12_peaw4agent_status_2_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?wait_for_all@agent@Concurrency@@SAX_KPEAPEAV12@PEAW4agent_status@2@I@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn wait_for_one_agent_concurrency_saxipapav12_aaw4agent_status_2_aaii_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?wait_for_one@agent@Concurrency@@SAXIPAPAV12@AAW4agent_status@2@AAII@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn wait_for_one_agent_concurrency_sax_kpeapeav12_aeaw4agent_status_2_aea_ki_z() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP120_APP_DLL_NAME,
        "?wait_for_one@agent@Concurrency@@SAX_KPEAPEAV12@AEAW4agent_status@2@AEA_KI@Z",
    )
}

/// `@ stub` export.
pub fn _dint() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Dint")
}

/// `@ stub` export.
pub fn _dnorm() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Dnorm")
}

/// `@ stub` export.
pub fn _dtento() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Dtento")
}

/// `@ stub` export.
pub fn _dunscale() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Dunscale")
}

/// `@ stub` export.
pub fn _fdint() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FDint")
}

/// `@ stub` export.
pub fn _fdnorm() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FDnorm")
}

/// `@ stub` export.
pub fn _fdtento() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FDtento")
}

/// `@ stub` export.
pub fn _fdunscale() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FDunscale")
}

/// `@ stub` export.
pub fn _fplsw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FPlsw")
}

/// `@ stub` export.
pub fn _fpmsw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FPmsw")
}

/// `@ stub` export.
pub fn _fxp_addh() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_addh")
}

/// `@ stub` export.
pub fn _fxp_addx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_addx")
}

/// `@ stub` export.
pub fn _fxp_getw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_getw")
}

/// `@ stub` export.
pub fn _fxp_invx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_invx")
}

/// `@ stub` export.
pub fn _fxp_ldexpx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_ldexpx")
}

/// `@ stub` export.
pub fn _fxp_movx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_movx")
}

/// `@ stub` export.
pub fn _fxp_mulh() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_mulh")
}

/// `@ stub` export.
pub fn _fxp_mulx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_mulx")
}

/// `@ stub` export.
pub fn _fxp_setn() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_setn")
}

/// `@ stub` export.
pub fn _fxp_setw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_setw")
}

/// `@ stub` export.
pub fn _fxp_sqrtx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_sqrtx")
}

/// `@ stub` export.
pub fn _fxp_subx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_FXp_subx")
}

/// `@ stub` export.
pub fn _ldint() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LDint")
}

/// `@ stub` export.
pub fn _ldtento() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LDtento")
}

/// `@ stub` export.
pub fn _ldunscale() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LDunscale")
}

/// `@ stub` export.
pub fn _lexp() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LExp")
}

/// `@ stub` export.
pub fn _lplsw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LPlsw")
}

/// `@ stub` export.
pub fn _lpmsw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LPmsw")
}

/// `@ stub` export.
pub fn _lpoly() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LPoly")
}

/// `@ stub` export.
pub fn _lxp_addh() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_addh")
}

/// `@ stub` export.
pub fn _lxp_addx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_addx")
}

/// `@ stub` export.
pub fn _lxp_getw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_getw")
}

/// `@ stub` export.
pub fn _lxp_invx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_invx")
}

/// `@ stub` export.
pub fn _lxp_ldexpx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_ldexpx")
}

/// `@ stub` export.
pub fn _lxp_movx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_movx")
}

/// `@ stub` export.
pub fn _lxp_mulh() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_mulh")
}

/// `@ stub` export.
pub fn _lxp_mulx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_mulx")
}

/// `@ stub` export.
pub fn _lxp_setn() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_setn")
}

/// `@ stub` export.
pub fn _lxp_setw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_setw")
}

/// `@ stub` export.
pub fn _lxp_sqrtx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_sqrtx")
}

/// `@ stub` export.
pub fn _lxp_subx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_LXp_subx")
}

/// `@ stub` export.
pub fn _mtx_timedlock() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Mtx_timedlock")
}

/// `@ stub` export.
pub fn _mtxdst() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Mtxdst")
}

/// `@ stub` export.
pub fn _mtxinit() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Mtxinit")
}

/// `@ stub` export.
pub fn _mtxlock() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Mtxlock")
}

/// `@ stub` export.
pub fn _mtxunlock() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Mtxunlock")
}

/// `@ stub` export.
pub fn _once() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Once")
}

/// `@ stub` export.
pub fn _plsw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Plsw")
}

/// `@ stub` export.
pub fn _pmsw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Pmsw")
}

/// `@ stub` export.
pub fn _poly() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Poly")
}

/// `@ stub` export.
pub fn _stoflt() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Stoflt")
}

/// `@ stub` export.
pub fn _stopfx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Stopfx")
}

/// `@ stub` export.
pub fn _stoxflt() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Stoxflt")
}

/// `@ stub` export.
pub fn _thrd_abort() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Thrd_abort")
}

/// `@ stub` export.
pub fn _thrd_exit() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Thrd_exit")
}

/// `@ stub` export.
pub fn _tss_create() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Tss_create")
}

/// `@ stub` export.
pub fn _tss_delete() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Tss_delete")
}

/// `@ stub` export.
pub fn _tss_get() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Tss_get")
}

/// `@ stub` export.
pub fn _tss_set() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Tss_set")
}

/// `@ stub` export.
pub fn _xp_addh() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_addh")
}

/// `@ stub` export.
pub fn _xp_addx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_addx")
}

/// `@ stub` export.
pub fn _xp_getw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_getw")
}

/// `@ stub` export.
pub fn _xp_invx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_invx")
}

/// `@ stub` export.
pub fn _xp_ldexpx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_ldexpx")
}

/// `@ stub` export.
pub fn _xp_movx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_movx")
}

/// `@ stub` export.
pub fn _xp_mulh() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_mulh")
}

/// `@ stub` export.
pub fn _xp_mulx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_mulx")
}

/// `@ stub` export.
pub fn _xp_setn() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_setn")
}

/// `@ stub` export.
pub fn _xp_setw() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_setw")
}

/// `@ stub` export.
pub fn _xp_sqrtx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_sqrtx")
}

/// `@ stub` export.
pub fn _xp_subx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "_Xp_subx")
}

/// `@ stub` export.
pub fn _wcrtomb_lk() -> StubException {
    wine_spec_unimplemented_stub(MSVCP120_APP_DLL_NAME, "__Wcrtomb_lk")
}
