/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msvcp_win.dll` in `dlls/msvcp_win/Makefile.in`.
const MSVCP_WIN_DLL_NAME: &str = "msvcp_win.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// The `@ stub` export `??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z`.
pub fn getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_ieaax_waebv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??$_Getvals@_W@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z")
}

/// The `@ stub` export `??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z`.
pub fn getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaax_waebv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??$_Getvals@_W@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z")
}

/// The `@ stub` export `??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z`.
pub fn getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_ieaax_waebv_locinfo_1_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??$_Getvals@_W@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@IEAAX_WAEBV_Locinfo@1@@Z")
}

/// The `@ stub` export `??0?$_Yarn@G@std@@QEAA@AEBV01@@Z`.
pub fn stub_0_yarn_g_std_qeaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0?$_Yarn@G@std@@QEAA@AEBV01@@Z")
}

/// The `@ stub` export `??0?$_Yarn@G@std@@QEAA@PEBG@Z`.
pub fn stub_0_yarn_g_std_qeaa_pebg_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0?$_Yarn@G@std@@QEAA@PEBG@Z")
}

/// The `@ stub` export `??0?$_Yarn@G@std@@QEAA@XZ`.
pub fn stub_0_yarn_g_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0?$_Yarn@G@std@@QEAA@XZ")
}

/// The `@ stub` export `??0?$_Yarn@_W@std@@QEAA@AEBV01@@Z`.
pub fn stub_0_yarn_w_std_qeaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0?$_Yarn@_W@std@@QEAA@AEBV01@@Z")
}

/// The `@ stub` export `??0?$_Yarn@_W@std@@QEAA@PEB_W@Z`.
pub fn stub_0_yarn_w_std_qeaa_peb_w_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0?$_Yarn@_W@std@@QEAA@PEB_W@Z")
}

/// The `@ stub` export `??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_iostream_du_char_traits_d_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_iostream_gu_char_traits_g_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_iostream_wu_char_traits_w_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_istream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_istream_du_char_traits_d_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_istream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_istream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_istream_gu_char_traits_g_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_istream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_istream_wu_char_traits_w_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_ostream_du_char_traits_d_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_ostream_gu_char_traits_g_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z`.
pub fn stub_0_basic_ostream_wu_char_traits_w_std_std_ieaa_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAA@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@AEBV01@@Z`.
pub fn stub_0_basic_streambuf_du_char_traits_d_std_std_ieaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@AEBV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@AEBV01@@Z`.
pub fn stub_0_basic_streambuf_gu_char_traits_g_std_std_ieaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAA@AEBV01@@Z",
    )
}

/// The `@ stub` export `??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@AEBV01@@Z`.
pub fn stub_0_basic_streambuf_wu_char_traits_w_std_std_ieaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAA@AEBV01@@Z",
    )
}

/// The `@ stub` export `??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAA@PEBD_K@Z`.
pub fn stub_0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaa_pebd_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAA@PEBD_K@Z",
    )
}

/// The `@ stub` export `??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z`.
pub fn stub_0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_aebv_locinfo_1_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@AEBV_Locinfo@1@_K@Z")
}

/// The `@ stub` export `??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z`.
pub fn stub_0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??0?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAA@_K@Z",
    )
}

/// The `@ stub` export `??0_Facet_base@std@@QEAA@AEBV01@@Z`.
pub fn stub_0_facet_base_std_qeaa_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0_Facet_base@std@@QEAA@AEBV01@@Z")
}

/// The `@ stub` export `??0_Facet_base@std@@QEAA@XZ`.
pub fn stub_0_facet_base_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0_Facet_base@std@@QEAA@XZ")
}

/// The `@ stub` export `??0_UShinit@std@@QEAA@XZ`.
pub fn stub_0_ushinit_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0_UShinit@std@@QEAA@XZ")
}

/// The `@ stub` export `??0time_base@std@@QEAA@_K@Z`.
pub fn stub_0time_base_std_qeaa_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??0time_base@std@@QEAA@_K@Z")
}

/// The `@ stub` export `??1?$_Yarn@G@std@@QEAA@XZ`.
pub fn stub_1_yarn_g_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??1?$_Yarn@G@std@@QEAA@XZ")
}

/// The `@ stub` export `??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEAA@XZ`.
pub fn stub_1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meaa_xz() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??1?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEAA@XZ",
    )
}

/// The `@ stub` export `??1_Facet_base@std@@UEAA@XZ`.
pub fn stub_1_facet_base_std_ueaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??1_Facet_base@std@@UEAA@XZ")
}

/// The `@ stub` export `??1_UShinit@std@@QEAA@XZ`.
pub fn stub_1_ushinit_std_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??1_UShinit@std@@QEAA@XZ")
}

/// The `@ stub` export `??1time_base@std@@UEAA@XZ`.
pub fn stub_1time_base_std_ueaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??1time_base@std@@UEAA@XZ")
}

/// The `@ stub` export `??4?$_Iosb@H@std@@QEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_iosb_h_std_qeaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$_Iosb@H@std@@QEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$_Iosb@H@std@@QEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_iosb_h_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??4?$_Iosb@H@std@@QEAAAEAV01@AEBV01@@Z")
}

/// The `@ stub` export `??4?$_Yarn@G@std@@QEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_yarn_g_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??4?$_Yarn@G@std@@QEAAAEAV01@AEBV01@@Z")
}

/// The `@ stub` export `??4?$_Yarn@G@std@@QEAAAEAV01@PEBG@Z`.
pub fn stub_4_yarn_g_std_qeaaaeav01_pebg_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??4?$_Yarn@G@std@@QEAAAEAV01@PEBG@Z")
}

/// The `@ stub` export `??4?$_Yarn@_W@std@@QEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_yarn_w_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$_Yarn@_W@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_iostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_iostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_iostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_iostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_iostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_iostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_istream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_istream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_istream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_istream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_istream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_istream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_istream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_istream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_ostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_ostream@DU?$char_traits@D@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_ostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_ostream@GU?$char_traits@G@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z`.
pub fn stub_4_basic_ostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_ostream@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@$$QEAV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_basic_streambuf_du_char_traits_d_std_std_ieaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAAAEAV01@AEBV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_basic_streambuf_gu_char_traits_g_std_std_ieaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_streambuf@GU?$char_traits@G@std@@@std@@IEAAAEAV01@AEBV01@@Z",
    )
}

/// The `@ stub` export `??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_basic_streambuf_wu_char_traits_w_std_std_ieaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4?$basic_streambuf@_WU?$char_traits@_W@std@@@std@@IEAAAEAV01@AEBV01@@Z",
    )
}

/// The `@ stub` export `??4_Crt_new_delete@std@@QEAAAEAU01@$$QEAU01@@Z`.
pub fn stub_4_crt_new_delete_std_qeaaaeau01_qeau01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4_Crt_new_delete@std@@QEAAAEAU01@$$QEAU01@@Z",
    )
}

/// The `@ stub` export `??4_Crt_new_delete@std@@QEAAAEAU01@AEBU01@@Z`.
pub fn stub_4_crt_new_delete_std_qeaaaeau01_aebu01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4_Crt_new_delete@std@@QEAAAEAU01@AEBU01@@Z",
    )
}

/// The `@ stub` export `??4_Facet_base@std@@QEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_facet_base_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4_Facet_base@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// The `@ stub` export `??4_Init_locks@std@@QEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_init_locks_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??4_Init_locks@std@@QEAAAEAV01@AEBV01@@Z",
    )
}

/// The `@ stub` export `??4_UShinit@std@@QEAAAEAV01@AEBV01@@Z`.
pub fn stub_4_ushinit_std_qeaaaeav01_aebv01_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??4_UShinit@std@@QEAAAEAV01@AEBV01@@Z")
}

/// The `@ stub` export `??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAG@Z`.
pub fn stub_5_basic_istream_gu_char_traits_g_std_std_qeaaaeav01_aeag_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??5?$basic_istream@GU?$char_traits@G@std@@@std@@QEAAAEAV01@AEAG@Z",
    )
}

/// The `@ stub` export `??_7?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@6B@`.
pub fn _7_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_6b() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??_7?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@6B@",
    )
}

/// The `@ stub` export `??_7time_base@std@@6B@`.
pub fn _7time_base_std_6b() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??_7time_base@std@@6B@")
}

/// The `@ stub` export `??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ`.
pub fn _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "??_F?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEAAXXZ",
    )
}

/// The `@ stub` export `??_Ftime_base@std@@QEAAXXZ`.
pub fn _ftime_base_std_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "??_Ftime_base@std@@QEAAXXZ")
}

/// The `@ stub` export `?CaptureCallstack@platform@details@Concurrency@@YA_KPEAPEAX_K1@Z`.
pub fn capturecallstack_platform_details_concurrency_ya_kpeapeax_k1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?CaptureCallstack@platform@details@Concurrency@@YA_KPEAPEAX_K1@Z",
    )
}

/// The `@ stub` export `?_Atexit@@YAXP6AXXZ@Z`.
pub fn _atexit_yaxp6axxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Atexit@@YAXP6AXXZ@Z")
}

/// The `@ stub` export `?_C_str@?$_Yarn@G@std@@QEBAPEBGXZ`.
pub fn _c_str_yarn_g_std_qebapebgxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_C_str@?$_Yarn@G@std@@QEBAPEBGXZ")
}

/// The `@ stub` export `?_Empty@?$_Yarn@G@std@@QEBA_NXZ`.
pub fn _empty_yarn_g_std_qeba_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Empty@?$_Yarn@G@std@@QEBA_NXZ")
}

/// The `@ stub` export `?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z`.
pub fn _fput_num_put_dv_ostreambuf_iterator_du_char_traits_d_std_std_std_aeba_av_ostreambuf_iterator_du_char_traits_d_std_2_v32_aeavios_base_2_dpebd_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Fput@?$num_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@DU?$char_traits@D@std@@@2@V32@AEAVios_base@2@DPEBD_K@Z")
}

/// The `@ stub` export `?_Fput@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@GU?$char_traits@G@std@@@2@V32@AEAVios_base@2@GPEBD_K@Z`.
pub fn _fput_num_put_gv_ostreambuf_iterator_gu_char_traits_g_std_std_std_aeba_av_ostreambuf_iterator_gu_char_traits_g_std_2_v32_aeavios_base_2_gpebd_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Fput@?$num_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@GU?$char_traits@G@std@@@2@V32@AEAVios_base@2@GPEBD_K@Z")
}

/// The `@ stub` export `?_Fput@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@2@V32@AEAVios_base@2@_WPEBD_K@Z`.
pub fn _fput_num_put_wv_ostreambuf_iterator_wu_char_traits_w_std_std_std_aeba_av_ostreambuf_iterator_wu_char_traits_w_std_2_v32_aeavios_base_2_wpebd_k_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Fput@?$num_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBA?AV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@2@V32@AEAVios_base@2@_WPEBD_K@Z")
}

/// The `@ stub` export `?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z`.
pub fn _getcat_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_sa_kpeapebvfacet_locale_2_pebv42_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getcat@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@SA_KPEAPEBVfacet@locale@2@PEBV42@@Z")
}

/// The `@ stub` export `?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@1AEAVios_base@2@PEAH@Z`.
pub fn _getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_2_1aeavios_base_2_peah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@1AEAVios_base@2@PEAH@Z")
}

/// The `@ stub` export `?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@1AEAVios_base@2@PEAH@Z`.
pub fn _getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_2_1aeavios_base_2_peah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@1AEAVios_base@2@PEAH@Z")
}

/// The `@ stub` export `?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@1AEAVios_base@2@PEAH@Z`.
pub fn _getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_2_1aeavios_base_2_peah_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@1AEAVios_base@2@PEAH@Z")
}

/// The `@ stub` export `?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBD@Z`.
pub fn _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getfmt@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBD@Z")
}

/// The `@ stub` export `?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@0HHAEAHAEBV?$ctype@D@2@@Z`.
pub fn _getint_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahaeav_istreambuf_iterator_du_char_traits_d_std_2_0hhaeahaebv_ctype_d_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getint@?$time_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@2@0HHAEAHAEBV?$ctype@D@2@@Z")
}

/// The `@ stub` export `?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@0HHAEAHAEBV?$ctype@G@2@@Z`.
pub fn _getint_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahaeav_istreambuf_iterator_gu_char_traits_g_std_2_0hhaeahaebv_ctype_g_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getint@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@0HHAEAHAEBV?$ctype@G@2@@Z")
}

/// The `@ stub` export `?_Getint@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@0HHAEAHAEBV?$ctype@_W@2@@Z`.
pub fn _getint_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahaeav_istreambuf_iterator_wu_char_traits_w_std_2_0hhaeahaebv_ctype_w_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Getint@?$time_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@2@0HHAEAHAEBV?$ctype@_W@2@@Z")
}

/// The `@ stub` export `?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z`.
pub fn _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaaxaebv_locinfo_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Init@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@IEAAXAEBV_Locinfo@2@@Z")
}

/// The `@ stub` export `?_Init_cnt@_UShinit@std@@0HA`.
pub fn _init_cnt_ushinit_std_0ha() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Init_cnt@_UShinit@std@@0HA")
}

/// The `@ stub` export `?_MP_Add@std@@YAXQEA_K_K@Z`.
pub fn _mp_add_std_yaxqea_k_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_MP_Add@std@@YAXQEA_K_K@Z")
}

/// The `@ stub` export `?_MP_Get@std@@YA_KQEA_K@Z`.
pub fn _mp_get_std_ya_kqea_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_MP_Get@std@@YA_KQEA_K@Z")
}

/// The `@ stub` export `?_MP_Mul@std@@YAXQEA_K_K1@Z`.
pub fn _mp_mul_std_yaxqea_k_k1_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_MP_Mul@std@@YAXQEA_K_K1@Z")
}

/// The `@ stub` export `?_MP_Rem@std@@YAXQEA_K_K@Z`.
pub fn _mp_rem_std_yaxqea_k_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_MP_Rem@std@@YAXQEA_K_K@Z")
}

/// The `@ stub` export `?_ReportUnobservedException@details@Concurrency@@YAXXZ`.
pub fn _reportunobservedexception_details_concurrency_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?_ReportUnobservedException@details@Concurrency@@YAXXZ",
    )
}

/// The `@ stub` export `?_Rng_abort@std@@YAXPEBD@Z`.
pub fn _rng_abort_std_yaxpebd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Rng_abort@std@@YAXPEBD@Z")
}

/// The `@ stub` export `?_Src@?1??_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB`.
pub fn _src_1_getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getffld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB`.
pub fn _src_1_getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getffld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB`.
pub fn _src_1_getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getffld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB`.
pub fn _src_1_getffldx_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB`.
pub fn _src_1_getffldx_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB`.
pub fn _src_1_getffldx_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getffldx@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1AEAVios_base@3@PEAH@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1HAEBVlocale@3@@Z@4QBDB`.
pub fn _src_1_getifld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1haebvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@DU?$char_traits@D@std@@@3@1HAEBVlocale@3@@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1HAEBVlocale@3@@Z@4QBDB`.
pub fn _src_1_getifld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1haebvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@GU?$char_traits@G@std@@@3@1HAEBVlocale@3@@Z@4QBDB")
}

/// The `@ stub` export `?_Src@?1??_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1HAEBVlocale@3@@Z@4QBDB`.
pub fn _src_1_getifld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1haebvlocale_3_z_4qbdb(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Src@?1??_Getifld@?$num_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@AEBAHPEADAEAV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@3@1HAEBVlocale@3@@Z@4QBDB")
}

/// The `@ stub` export `?_Throw_Cpp_error@std@@YAXH@Z`.
pub fn _throw_cpp_error_std_yaxh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Throw_Cpp_error@std@@YAXH@Z")
}

/// The `@ stub` export `?_Tidy@?$_Yarn@G@std@@AEAAXXZ`.
pub fn _tidy_yarn_g_std_aeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Tidy@?$_Yarn@G@std@@AEAAXXZ")
}

/// The `@ stub` export `?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEAAXXZ`.
pub fn _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aeaaxxz() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?_Tidy@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@AEAAXXZ",
    )
}

/// The `@ stub` export `?_W_Gettnames@_Locinfo@std@@QEBA?AV_Timevec@2@XZ`.
pub fn _w_gettnames_locinfo_std_qeba_av_timevec_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?_W_Gettnames@_Locinfo@std@@QEBA?AV_Timevec@2@XZ",
    )
}

/// The `@ stub` export `?_Xoverflow_error@std@@YAXPEBD@Z`.
pub fn _xoverflow_error_std_yaxpebd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?_Xoverflow_error@std@@YAXPEBD@Z")
}

/// The `@ stub` export `?c_str@?$_Yarn@G@std@@QEBAPEBGXZ`.
pub fn c_str_yarn_g_std_qebapebgxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?c_str@?$_Yarn@G@std@@QEBAPEBGXZ")
}

/// The `@ stub` export `?c_str@?$_Yarn@_W@std@@QEBAPEB_WXZ`.
pub fn c_str_yarn_w_std_qebapeb_wxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?c_str@?$_Yarn@_W@std@@QEBAPEB_WXZ")
}

/// The `@ stub` export `?classic_table@?$ctype@D@std@@SAPEBFXZ`.
pub fn classic_table_ctype_d_std_sapebfxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?classic_table@?$ctype@D@std@@SAPEBFXZ")
}

/// The `@ stub` export `?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AW4dateorder@time_base@2@XZ`.
pub fn date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AW4dateorder@time_base@2@XZ")
}

/// The `@ stub` export `?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AW4dateorder@time_base@2@XZ`.
pub fn do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_aw4dateorder_time_base_2_xz(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_date_order@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AW4dateorder@time_base@2@XZ")
}

/// The `@ stub` export `?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z`.
pub fn do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z")
}

/// The `@ stub` export `?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?do_get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@MEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?empty@?$_Yarn@G@std@@QEBA_NXZ`.
pub fn empty_yarn_g_std_qeba_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?empty@?$_Yarn@G@std@@QEBA_NXZ")
}

/// The `@ stub` export `?empty@?$_Yarn@_W@std@@QEBA_NXZ`.
pub fn empty_yarn_w_std_qeba_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?empty@?$_Yarn@_W@std@@QEBA_NXZ")
}

/// The `@ stub` export `?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z`.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@DD@Z")
}

/// The `@ stub` export `?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBG4@Z`.
pub fn get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebg4_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@PEBG4@Z")
}

/// The `@ stub` export `?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get_date@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get_monthname@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?get_new_handler@std@@YAP6AXXZXZ`.
pub fn get_new_handler_std_yap6axxzxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get_new_handler@std@@YAP6AXXZXZ")
}

/// The `@ stub` export `?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get_time@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get_weekday@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z`.
pub fn get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?get_year@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@QEBA?AV?$istreambuf_iterator@GU?$char_traits@G@std@@@2@V32@0AEAVios_base@2@AEAHPEAUtm@@@Z")
}

/// The `@ stub` export `?id@?$messages@D@std@@2V0locale@2@A`.
pub fn id_messages_d_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?id@?$messages@D@std@@2V0locale@2@A")
}

/// The `@ stub` export `?id@?$messages@G@std@@2V0locale@2@A`.
pub fn id_messages_g_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?id@?$messages@G@std@@2V0locale@2@A")
}

/// The `@ stub` export `?id@?$messages@_W@std@@2V0locale@2@A`.
pub fn id_messages_w_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?id@?$messages@_W@std@@2V0locale@2@A")
}

/// The `@ stub` export `?id@?$money_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_money_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$money_get@DV?$istreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$money_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_money_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$money_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$money_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_money_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?id@?$money_get@_WV?$istreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@2V0locale@2@A")
}

/// The `@ stub` export `?id@?$money_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_money_put_dv_ostreambuf_iterator_du_char_traits_d_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$money_put@DV?$ostreambuf_iterator@DU?$char_traits@D@std@@@std@@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$money_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_money_put_gv_ostreambuf_iterator_gu_char_traits_g_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$money_put@GV?$ostreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$money_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_money_put_wv_ostreambuf_iterator_wu_char_traits_w_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?id@?$money_put@_WV?$ostreambuf_iterator@_WU?$char_traits@_W@std@@@std@@@std@@2V0locale@2@A")
}

/// The `@ stub` export `?id@?$moneypunct@D$00@std@@2V0locale@2@A`.
pub fn id_moneypunct_d_00_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$moneypunct@D$00@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$moneypunct@D$0A@@std@@2V0locale@2@A`.
pub fn id_moneypunct_d_0a_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$moneypunct@D$0A@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$moneypunct@G$00@std@@2V0locale@2@A`.
pub fn id_moneypunct_g_00_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$moneypunct@G$00@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$moneypunct@G$0A@@std@@2V0locale@2@A`.
pub fn id_moneypunct_g_0a_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$moneypunct@G$0A@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$moneypunct@_W$00@std@@2V0locale@2@A`.
pub fn id_moneypunct_w_00_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$moneypunct@_W$00@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$moneypunct@_W$0A@@std@@2V0locale@2@A`.
pub fn id_moneypunct_w_0a_std_2v0locale_2_a() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$moneypunct@_W$0A@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?id@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@2V0locale@2@A`.
pub fn id_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_2v0locale_2_a(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?id@?$time_get@GV?$istreambuf_iterator@GU?$char_traits@G@std@@@std@@@std@@2V0locale@2@A",
    )
}

/// The `@ stub` export `?intl@?$moneypunct@D$00@std@@2_NB`.
pub fn intl_moneypunct_d_00_std_2_nb() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?intl@?$moneypunct@D$00@std@@2_NB")
}

/// The `@ stub` export `?intl@?$moneypunct@D$0A@@std@@2_NB`.
pub fn intl_moneypunct_d_0a_std_2_nb() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?intl@?$moneypunct@D$0A@@std@@2_NB")
}

/// The `@ stub` export `?intl@?$moneypunct@G$00@std@@2_NB`.
pub fn intl_moneypunct_g_00_std_2_nb() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?intl@?$moneypunct@G$00@std@@2_NB")
}

/// The `@ stub` export `?intl@?$moneypunct@G$0A@@std@@2_NB`.
pub fn intl_moneypunct_g_0a_std_2_nb() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?intl@?$moneypunct@G$0A@@std@@2_NB")
}

/// The `@ stub` export `?intl@?$moneypunct@_W$00@std@@2_NB`.
pub fn intl_moneypunct_w_00_std_2_nb() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?intl@?$moneypunct@_W$00@std@@2_NB")
}

/// The `@ stub` export `?intl@?$moneypunct@_W$0A@@std@@2_NB`.
pub fn intl_moneypunct_w_0a_std_2_nb() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?intl@?$moneypunct@_W$0A@@std@@2_NB")
}

/// The `@ stub` export `?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAX$$QEAV12@@Z`.
pub fn move_basic_ios_du_char_traits_d_std_std_qeaax_qeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAX$$QEAV12@@Z",
    )
}

/// The `@ stub` export `?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXAEAV12@@Z`.
pub fn move_basic_ios_du_char_traits_d_std_std_qeaaxaeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?move@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXAEAV12@@Z",
    )
}

/// The `@ stub` export `?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAX$$QEAV12@@Z`.
pub fn move_basic_ios_gu_char_traits_g_std_std_qeaax_qeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAX$$QEAV12@@Z",
    )
}

/// The `@ stub` export `?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXAEAV12@@Z`.
pub fn move_basic_ios_gu_char_traits_g_std_std_qeaaxaeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?move@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXAEAV12@@Z",
    )
}

/// The `@ stub` export `?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAX$$QEAV12@@Z`.
pub fn move_basic_ios_wu_char_traits_w_std_std_qeaax_qeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAX$$QEAV12@@Z",
    )
}

/// The `@ stub` export `?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXAEAV12@@Z`.
pub fn move_basic_ios_wu_char_traits_w_std_std_qeaaxaeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCP_WIN_DLL_NAME,
        "?move@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXAEAV12@@Z",
    )
}

/// The `@ stub` export `?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z`.
pub fn set_rdbuf_basic_ios_du_char_traits_d_std_std_qeaaxpeav_basic_streambuf_du_char_traits_d_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?set_rdbuf@?$basic_ios@DU?$char_traits@D@std@@@std@@QEAAXPEAV?$basic_streambuf@DU?$char_traits@D@std@@@2@@Z")
}

/// The `@ stub` export `?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXPEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z`.
pub fn set_rdbuf_basic_ios_gu_char_traits_g_std_std_qeaaxpeav_basic_streambuf_gu_char_traits_g_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?set_rdbuf@?$basic_ios@GU?$char_traits@G@std@@@std@@QEAAXPEAV?$basic_streambuf@GU?$char_traits@G@std@@@2@@Z")
}

/// The `@ stub` export `?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXPEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z`.
pub fn set_rdbuf_basic_ios_wu_char_traits_w_std_std_qeaaxpeav_basic_streambuf_wu_char_traits_w_std_2_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "?set_rdbuf@?$basic_ios@_WU?$char_traits@_W@std@@@std@@QEAAXPEAV?$basic_streambuf@_WU?$char_traits@_W@std@@@2@@Z")
}

/// The `@ stub` export `_Chmod`.
pub fn _chmod() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Chmod")
}

/// The `@ stub` export `_Hard_links`.
pub fn _hard_links() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Hard_links")
}

/// The `@ stub` export `_LExp`.
pub fn _lexp() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_LExp")
}

/// The `@ stub` export `_Mtx_timedlock`.
pub fn _mtx_timedlock() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Mtx_timedlock")
}

/// The `@ stub` export `_Mtxdst`.
pub fn _mtxdst() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Mtxdst")
}

/// The `@ stub` export `_Mtxinit`.
pub fn _mtxinit() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Mtxinit")
}

/// The `@ stub` export `_Mtxlock`.
pub fn _mtxlock() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Mtxlock")
}

/// The `@ stub` export `_Mtxunlock`.
pub fn _mtxunlock() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Mtxunlock")
}

/// The `@ stub` export `_Statvfs`.
pub fn _statvfs() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Statvfs")
}

/// The `@ stub` export `_Symlink_get`.
pub fn _symlink_get() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Symlink_get")
}

/// The `@ stub` export `_Thrd_abort`.
pub fn _thrd_abort() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Thrd_abort")
}

/// The `@ stub` export `_Thrd_exit`.
pub fn _thrd_exit() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_Thrd_exit")
}

/// The `@ stub` export `_WStod`.
pub fn _wstod() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_WStod")
}

/// The `@ stub` export `_WStodx`.
pub fn _wstodx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_WStodx")
}

/// The `@ stub` export `_WStof`.
pub fn _wstof() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_WStof")
}

/// The `@ stub` export `_WStofx`.
pub fn _wstofx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_WStofx")
}

/// The `@ stub` export `_WStold`.
pub fn _wstold() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_WStold")
}

/// The `@ stub` export `_WStoldx`.
pub fn _wstoldx() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "_WStoldx")
}

/// The `@ stub` export `__Wcrtomb_lk`.
pub fn _wcrtomb_lk() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "__Wcrtomb_lk")
}

/// The `@ stub` export `__set_stl_sync_api_mode`.
pub fn _set_stl_sync_api_mode() -> StubException {
    wine_spec_unimplemented_stub(MSVCP_WIN_DLL_NAME, "__set_stl_sync_api_mode")
}
