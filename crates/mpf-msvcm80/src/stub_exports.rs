//! Helpers for the `@ stub` exports of `dlls/msvcm80/msvcm80.spec`, one per export line.
//!
//! Each helper is named after its export: the export name in lower case, without the leading
//! `?`, with each run of non-alphanumeric characters replaced by `_`. The leading underscores
//! are kept as they are, so `_Stodx` and `__Stodx` get different names. Both `-arch=win32` and
//! `-arch=win64` variants are present, because the spec lists both.

use super::stub::{wine_spec_unimplemented_stub, StubException, MSVCM80_DLL_NAME};

/// `@ stub -arch=win32` export.
pub fn docallbackindefaultdomain_crtimplementationdetails_yaxp6gjpax_z0_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?DoCallBackInDefaultDomain@<CrtImplementationDetails>@@YAXP6GJPAX@Z0@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn docallbackindefaultdomain_crtimplementationdetails_yaxp6ajpeax_z0_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?DoCallBackInDefaultDomain@<CrtImplementationDetails>@@YAXP6AJPEAX@Z0@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _addstd_ios_base_std_saxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Addstd@ios_base@std@@SAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _addstd_ios_base_std_saxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Addstd@ios_base@std@@SAXPEAV12@@Z")
}

/// `@ stub` export.
pub fn _atexit_yaxp6axxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Atexit@@YAXP6AXXZ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _badoff_func_std_yaabjxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_BADOFF_func@std@@YAABJXZ")
}

/// `@ stub -arch=win64` export.
pub fn _badoff_func_std_yaaeb_jxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_BADOFF_func@std@@YAAEB_JXZ")
}

/// `@ stub -arch=win32` export.
pub fn _cerr_func_std_yaaav_basic_ostream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Cerr_func@std@@YAAAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _cerr_func_std_yaaeav_basic_ostream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Cerr_func@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _cin_func_std_yaaav_basic_istream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Cin_func@std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _cin_func_std_yaaeav_basic_istream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Cin_func@std@@YAAEAV?$basic_istream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _clocptr_func_locimp_locale_std_caaapav123_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Clocptr_func@_Locimp@locale@std@@CAAAPAV123@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _clocptr_func_locimp_locale_std_caaeapeav123_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Clocptr_func@_Locimp@locale@std@@CAAEAPEAV123@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _clog_func_std_yaaav_basic_ostream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Clog_func@std@@YAAAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _clog_func_std_yaaeav_basic_ostream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Clog_func@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub` export.
pub fn _cosh_yannn_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Cosh@@YANNN@Z")
}

/// `@ stub -arch=win32` export.
pub fn _cout_func_std_yaaav_basic_ostream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Cout_func@std@@YAAAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _cout_func_std_yaaeav_basic_ostream_du_char_traits_d_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Cout_func@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _debug_message_std_yaxpbd0_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Debug_message@std@@YAXPBD0@Z")
}

/// `@ stub -arch=win64` export.
pub fn _debug_message_std_yaxpebd0_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Debug_message@std@@YAXPEBD0@Z")
}

/// `@ stub -arch=win32` export.
pub fn _dnorm_yafpag_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dnorm@@YAFPAG@Z")
}

/// `@ stub -arch=win64` export.
pub fn _dnorm_yafpeag_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dnorm@@YAFPEAG@Z")
}

/// `@ stub -arch=win32` export.
pub fn _dscale_yafpanj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dscale@@YAFPANJ@Z")
}

/// `@ stub -arch=win64` export.
pub fn _dscale_yafpeanj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dscale@@YAFPEANJ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _dtentox_yannjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dtentox@@YANNJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _dtentox_yannjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dtentox@@YANNJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _dtest_yafpan_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dtest@@YAFPAN@Z")
}

/// `@ stub -arch=win64` export.
pub fn _dtest_yafpean_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dtest@@YAFPEAN@Z")
}

/// `@ stub -arch=win32` export.
pub fn _dunscale_yafpafpan_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dunscale@@YAFPAFPAN@Z")
}

/// `@ stub -arch=win64` export.
pub fn _dunscale_yafpeafpean_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Dunscale@@YAFPEAFPEAN@Z")
}

/// `@ stub -arch=win32` export.
pub fn _exp_yafpannf_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Exp@@YAFPANNF@Z")
}

/// `@ stub -arch=win64` export.
pub fn _exp_yafpeannf_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Exp@@YAFPEANNF@Z")
}

/// `@ stub` export.
pub fn _fcosh_yammm_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FCosh@@YAMMM@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fdnorm_yafpag_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDnorm@@YAFPAG@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fdnorm_yafpeag_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDnorm@@YAFPEAG@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fdscale_yafpamj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDscale@@YAFPAMJ@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fdscale_yafpeamj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDscale@@YAFPEAMJ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fdtentox_yammjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDtentox@@YAMMJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fdtentox_yammjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDtentox@@YAMMJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fdtest_yafpam_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDtest@@YAFPAM@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fdtest_yafpeam_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDtest@@YAFPEAM@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fdunscale_yafpafpam_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDunscale@@YAFPAFPAM@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fdunscale_yafpeafpeam_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FDunscale@@YAFPEAFPEAM@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fexp_yafpammf_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FExp@@YAFPAMMF@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fexp_yafpeammf_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FExp@@YAFPEAMMF@Z")
}

/// `@ stub` export.
pub fn _fsinh_yammm_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_FSinh@@YAMMM@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fiopen_std_yapau_iobuf_pbdhh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fiopen@std@@YAPAU_iobuf@@PBDHH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fiopen_std_yapeau_iobuf_pebdhh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fiopen@std@@YAPEAU_iobuf@@PEBDHH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fiopen_std_yapau_iobuf_pbghh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fiopen@std@@YAPAU_iobuf@@PBGHH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fiopen_std_yapeau_iobuf_pebghh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fiopen@std@@YAPEAU_iobuf@@PEBGHH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fiopen_std_yapau_iobuf_pb_whh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fiopen@std@@YAPAU_iobuf@@PB_WHH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _fiopen_std_yapeau_iobuf_peb_whh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fiopen@std@@YAPEAU_iobuf@@PEB_WHH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _fpz_func_std_yaaa_jxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fpz_func@std@@YAAA_JXZ")
}

/// `@ stub -arch=win64` export.
pub fn _fpz_func_std_yaaea_jxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Fpz_func@std@@YAAEA_JXZ")
}

/// `@ stub` export.
pub fn _getcoll_ya_au_collvec_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getcoll@@YA?AU_Collvec@@XZ")
}

/// `@ stub` export.
pub fn _getctype_ya_au_ctypevec_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getctype@@YA?AU_Ctypevec@@XZ")
}

/// `@ stub` export.
pub fn _getcvt_ya_au_cvtvec_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getcvt@@YA?AU_Cvtvec@@XZ")
}

/// `@ stub` export.
pub fn _getdateorder_yahxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getdateorder@@YAHXZ")
}

/// `@ stub -arch=win32` export.
pub fn _getgloballocale_locale_std_capav_locimp_12_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Getgloballocale@locale@std@@CAPAV_Locimp@12@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _getgloballocale_locale_std_capeav_locimp_12_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _getwctype_yafgpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getwctype@@YAFGPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getwctype_yafgpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getwctype@@YAFGPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getwctype_yaf_wpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getwctype@@YAF_WPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _getwctype_yaf_wpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Getwctype@@YAF_WPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _getwctypes_yapbgpbg0pafpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Getwctypes@@YAPBGPBG0PAFPBU_Ctypevec@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _getwctypes_yapebgpebg0peafpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Getwctypes@@YAPEBGPEBG0PEAFPEBU_Ctypevec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _getwctypes_yapb_wpb_w0pafpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Getwctypes@@YAPB_WPB_W0PAFPBU_Ctypevec@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _getwctypes_yapeb_wpeb_w0peafpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Getwctypes@@YAPEB_WPEB_W0PEAFPEBU_Ctypevec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _id_cnt_func_id_locale_std_caaahxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Id_cnt_func@id@locale@std@@CAAAHXZ")
}

/// `@ stub -arch=win64` export.
pub fn _id_cnt_func_id_locale_std_caaeahxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Id_cnt_func@id@locale@std@@CAAEAHXZ")
}

/// `@ stub -arch=win32` export.
pub fn _id_func_codecvt_gdh_std_saaavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$codecvt@GDH@std@@SAAAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _id_func_codecvt_gdh_std_saaeavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$codecvt@GDH@std@@SAAEAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _id_func_codecvt_wdh_std_saaavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$codecvt@_WDH@std@@SAAAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _id_func_codecvt_wdh_std_saaeavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$codecvt@_WDH@std@@SAAEAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _id_func_ctype_d_std_saaavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$ctype@D@std@@SAAAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _id_func_ctype_d_std_saaeavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$ctype@D@std@@SAAEAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _id_func_ctype_g_std_saaavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$ctype@G@std@@SAAAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _id_func_ctype_g_std_saaeavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$ctype@G@std@@SAAEAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _id_func_ctype_w_std_saaavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$ctype@_W@std@@SAAAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _id_func_ctype_w_std_saaeavid_locale_2_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Id_func@?$ctype@_W@std@@SAAEAVid@locale@2@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _index_func_ios_base_std_caaahxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Index_func@ios_base@std@@CAAAHXZ")
}

/// `@ stub -arch=win64` export.
pub fn _index_func_ios_base_std_caaeahxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Index_func@ios_base@std@@CAAEAHXZ")
}

/// `@ stub -arch=win32` export.
pub fn _init_locale_std_capav_locimp_12_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Init@locale@std@@CAPAV_Locimp@12@XZ")
}

/// `@ stub -arch=win64` export.
pub fn _init_locale_std_capeav_locimp_12_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Init@locale@std@@CAPEAV_Locimp@12@XZ")
}

/// `@ stub -arch=win32` export.
pub fn _init_cnt_func_init_ios_base_std_caaahxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_cnt_func@Init@ios_base@std@@CAAAHXZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _init_cnt_func_init_ios_base_std_caaeahxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_cnt_func@Init@ios_base@std@@CAAEAHXZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _init_ctor_init_ios_base_std_caxpav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_ctor@Init@ios_base@std@@CAXPAV123@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _init_ctor_init_ios_base_std_caxpeav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_ctor@Init@ios_base@std@@CAXPEAV123@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _init_dtor_init_ios_base_std_caxpav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_dtor@Init@ios_base@std@@CAXPAV123@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _init_dtor_init_ios_base_std_caxpeav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_dtor@Init@ios_base@std@@CAXPEAV123@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _init_locks_ctor_init_locks_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_locks_ctor@_Init_locks@std@@CAXPAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _init_locks_ctor_init_locks_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_locks_ctor@_Init_locks@std@@CAXPEAV12@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _init_locks_dtor_init_locks_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_locks_dtor@_Init_locks@std@@CAXPAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _init_locks_dtor_init_locks_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Init_locks_dtor@_Init_locks@std@@CAXPEAV12@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _ios_base_dtor_ios_base_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Ios_base_dtor@ios_base@std@@CAXPAV12@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _ios_base_dtor_ios_base_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Ios_base_dtor@ios_base@std@@CAXPEAV12@@Z",
    )
}

/// `@ stub` export.
pub fn _lcosh_yaooo_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LCosh@@YAOOO@Z")
}

/// `@ stub -arch=win32` export.
pub fn _ldscale_yafpaoj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDscale@@YAFPAOJ@Z")
}

/// `@ stub -arch=win64` export.
pub fn _ldscale_yafpeaoj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDscale@@YAFPEAOJ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _ldtentox_yaoojpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDtentox@@YAOOJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _ldtentox_yaoojpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDtentox@@YAOOJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _ldtest_yafpao_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDtest@@YAFPAO@Z")
}

/// `@ stub -arch=win64` export.
pub fn _ldtest_yafpeao_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDtest@@YAFPEAO@Z")
}

/// `@ stub -arch=win32` export.
pub fn _ldunscale_yafpafpao_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDunscale@@YAFPAFPAO@Z")
}

/// `@ stub -arch=win64` export.
pub fn _ldunscale_yafpeafpeao_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LDunscale@@YAFPEAFPEAO@Z")
}

/// `@ stub -arch=win32` export.
pub fn _lexp_yafpaoof_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LExp@@YAFPAOOF@Z")
}

/// `@ stub -arch=win64` export.
pub fn _lexp_yafpeaoof_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LExp@@YAFPEAOOF@Z")
}

/// `@ stub -arch=win32` export.
pub fn _lpoly_yaoopboh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LPoly@@YAOOPBOH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _lpoly_yaoopeboh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LPoly@@YAOOPEBOH@Z")
}

/// `@ stub` export.
pub fn _lsinh_yaooo_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_LSinh@@YAOOO@Z")
}

/// `@ stub -arch=win32` export.
pub fn _locimp_addfac_locimp_locale_std_caxpav123_pavfacet_23_i_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locimp_Addfac@_Locimp@locale@std@@CAXPAV123@PAVfacet@23@I@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _locimp_addfac_locimp_locale_std_caxpeav123_peavfacet_23_k_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locimp_Addfac@_Locimp@locale@std@@CAXPEAV123@PEAVfacet@23@_K@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _locimp_ctor_locimp_locale_std_caxpav123_abv123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locimp_ctor@_Locimp@locale@std@@CAXPAV123@ABV123@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _locimp_ctor_locimp_locale_std_caxpeav123_aebv123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locimp_ctor@_Locimp@locale@std@@CAXPEAV123@AEBV123@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _locimp_dtor_locimp_locale_std_caxpav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locimp_dtor@_Locimp@locale@std@@CAXPAV123@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _locimp_dtor_locimp_locale_std_caxpeav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locimp_dtor@_Locimp@locale@std@@CAXPEAV123@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _locinfo_addcats_locinfo_std_saaav12_pav12_hpbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_Addcats@_Locinfo@std@@SAAAV12@PAV12@HPBD@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _locinfo_addcats_locinfo_std_saaeav12_peav12_hpebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_Addcats@_Locinfo@std@@SAAEAV12@PEAV12@HPEBD@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _locinfo_ctor_locinfo_std_saxpav12_hpbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPAV12@HPBD@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _locinfo_ctor_locinfo_std_saxpeav12_hpebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@HPEBD@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _locinfo_ctor_locinfo_std_saxpav12_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPAV12@PBD@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _locinfo_ctor_locinfo_std_saxpeav12_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@PEBD@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _locinfo_dtor_locinfo_std_saxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Locinfo_dtor@_Locinfo@std@@SAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _locinfo_dtor_locinfo_std_saxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Locinfo_dtor@_Locinfo@std@@SAXPEAV12@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _lockit_ctor_lockit_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_ctor@_Lockit@std@@CAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _lockit_ctor_lockit_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_ctor@_Lockit@std@@CAXPEAV12@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _lockit_ctor_lockit_std_caxpav12_h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_ctor@_Lockit@std@@CAXPAV12@H@Z")
}

/// `@ stub -arch=win64` export.
pub fn _lockit_ctor_lockit_std_caxpeav12_h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_ctor@_Lockit@std@@CAXPEAV12@H@Z")
}

/// `@ stub` export.
pub fn _lockit_ctor_lockit_std_saxh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_ctor@_Lockit@std@@SAXH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _lockit_dtor_lockit_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_dtor@_Lockit@std@@CAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _lockit_dtor_lockit_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_dtor@_Lockit@std@@CAXPEAV12@@Z")
}

/// `@ stub` export.
pub fn _lockit_dtor_lockit_std_saxh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Lockit_dtor@_Lockit@std@@SAXH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _makeloc_locimp_locale_std_capav123_abv_locinfo_3_hpav123_pbv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makeloc@_Locimp@locale@std@@CAPAV123@ABV_Locinfo@3@HPAV123@PBV23@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _makeloc_locimp_locale_std_capeav123_aebv_locinfo_3_hpeav123_pebv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makeloc@_Locimp@locale@std@@CAPEAV123@AEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _makeushloc_locimp_locale_std_caxabv_locinfo_3_hpav123_pbv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makeushloc@_Locimp@locale@std@@CAXABV_Locinfo@3@HPAV123@PBV23@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _makeushloc_locimp_locale_std_caxaebv_locinfo_3_hpeav123_pebv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makeushloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _makewloc_locimp_locale_std_caxabv_locinfo_3_hpav123_pbv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makewloc@_Locimp@locale@std@@CAXABV_Locinfo@3@HPAV123@PBV23@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _makewloc_locimp_locale_std_caxaebv_locinfo_3_hpeav123_pebv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makewloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _makexloc_locimp_locale_std_caxabv_locinfo_3_hpav123_pbv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makexloc@_Locimp@locale@std@@CAXABV_Locinfo@3@HPAV123@PBV23@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _makexloc_locimp_locale_std_caxaebv_locinfo_3_hpeav123_pebv23_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Makexloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mbrtowc_yahpagpbdipahpbu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mbrtowc@@YAHPAGPBDIPAHPBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mbrtowc_yahpeagpebd_kpeahpebu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mbrtowc@@YAHPEAGPEBD_KPEAHPEBU_Cvtvec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mbrtowc_yahpa_wpbdipahpbu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mbrtowc@@YAHPA_WPBDIPAHPBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mbrtowc_yahpea_wpebd_kpeahpebu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mbrtowc@@YAHPEA_WPEBD_KPEAHPEBU_Cvtvec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mtxdst_yaxpau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxdst@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _mtxdst_yaxpeau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxdst@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mtxinit_yaxpau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxinit@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _mtxinit_yaxpeau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxinit@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mtxlock_yaxpau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxlock@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _mtxlock_yaxpeau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxlock@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mtxunlock_yaxpau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxunlock@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _mtxunlock_yaxpeau_rtl_critical_section_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Mtxunlock@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _mutex_lock_mutex_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_Lock@_Mutex@std@@CAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mutex_lock_mutex_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_Lock@_Mutex@std@@CAXPEAV12@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mutex_unlock_mutex_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_Unlock@_Mutex@std@@CAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mutex_unlock_mutex_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_Unlock@_Mutex@std@@CAXPEAV12@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mutex_ctor_mutex_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_ctor@_Mutex@std@@CAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mutex_ctor_mutex_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_ctor@_Mutex@std@@CAXPEAV12@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _mutex_dtor_mutex_std_caxpav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_dtor@_Mutex@std@@CAXPAV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _mutex_dtor_mutex_std_caxpeav12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Mutex_dtor@_Mutex@std@@CAXPEAV12@@Z")
}

/// `@ stub` export.
pub fn _nomemory_std_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Nomemory@std@@YAXXZ")
}

/// `@ stub -arch=win32` export.
pub fn _once_yaxpajp6axxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Once@@YAXPAJP6AXXZ@Z")
}

/// `@ stub -arch=win64` export.
pub fn _once_yaxpeajp6axxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Once@@YAXPEAJP6AXXZ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _poly_yannpbnh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Poly@@YANNPBNH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _poly_yannpebnh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Poly@@YANNPEBNH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _setgloballocale_locale_std_caxpax_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Setgloballocale@locale@std@@CAXPAX@Z")
}

/// `@ stub -arch=win64` export.
pub fn _setgloballocale_locale_std_caxpeax_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Setgloballocale@locale@std@@CAXPEAX@Z")
}

/// `@ stub` export.
pub fn _sinh_yannn_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Sinh@@YANNN@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stod_yanpbdpapadj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stod@@YANPBDPAPADJ@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stod_yanpebdpeapeadj_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stod@@YANPEBDPEAPEADJ@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stodx_yanpbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stodx@@YANPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stodx_yanpebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stodx@@YANPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stof_yampbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stof@@YAMPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stof_yampebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stof@@YAMPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stoflt_yahpbd0papadqajh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoflt@@YAHPBD0PAPADQAJH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stoflt_yahpebd0peapeadqeajh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoflt@@YAHPEBD0PEAPEADQEAJH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stofx_yampbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stofx@@YAMPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stofx_yampebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stofx@@YAMPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stold_yaopbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stold@@YAOPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stold_yaopebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stold@@YAOPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stoldx_yaopbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoldx@@YAOPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stoldx_yaopebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoldx@@YAOPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stolx_yajpbdpapadhpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stolx@@YAJPBDPAPADHPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stolx_yajpebdpeapeadhpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stolx@@YAJPEBDPEAPEADHPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stopfx_yahpapbdpapad_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stopfx@@YAHPAPBDPAPAD@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stopfx_yahpeapebdpeapead_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stopfx@@YAHPEAPEBDPEAPEAD@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stoul_yakpbdpapadh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoul@@YAKPBDPAPADH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stoul_yakpebdpeapeadh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoul@@YAKPEBDPEAPEADH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stoulx_yakpbdpapadhpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoulx@@YAKPBDPAPADHPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stoulx_yakpebdpeapeadhpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoulx@@YAKPEBDPEAPEADHPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _stoxflt_yahpbd0papadqajh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoxflt@@YAHPBD0PAPADQAJH@Z")
}

/// `@ stub -arch=win64` export.
pub fn _stoxflt_yahpebd0peapeadqeajh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Stoxflt@@YAHPEBD0PEAPEADQEAJH@Z")
}

/// `@ stub -arch=win32` export.
pub fn _strcoll_yahpbd000pbu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Strcoll@@YAHPBD000PBU_Collvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _strcoll_yahpebd000pebu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Strcoll@@YAHPEBD000PEBU_Collvec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _strxfrm_yaipad0pbd1pbu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Strxfrm@@YAIPAD0PBD1PBU_Collvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _strxfrm_ya_kpead0pebd1pebu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Strxfrm@@YA_KPEAD0PEBD1PEBU_Collvec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _sync_func_ios_base_std_caaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Sync_func@ios_base@std@@CAAA_NXZ")
}

/// `@ stub -arch=win64` export.
pub fn _sync_func_ios_base_std_caaea_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Sync_func@ios_base@std@@CAAEA_NXZ")
}

/// `@ stub -arch=win32` export.
pub fn _throw_std_yaxabvexception_stdext_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Throw@std@@YAXABVexception@stdext@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _throw_std_yaxaebvexception_stdext_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Throw@std@@YAXAEBVexception@stdext@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _tolower_yahhpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Tolower@@YAHHPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _tolower_yahhpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Tolower@@YAHHPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _toupper_yahhpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Toupper@@YAHHPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _toupper_yahhpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Toupper@@YAHHPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _towlower_yaggpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towlower@@YAGGPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _towlower_yaggpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towlower@@YAGGPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _towlower_ya_w_wpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towlower@@YA_W_WPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _towlower_ya_w_wpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towlower@@YA_W_WPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _towupper_yaggpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towupper@@YAGGPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _towupper_yaggpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towupper@@YAGGPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _towupper_ya_w_wpbu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towupper@@YA_W_WPBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _towupper_ya_w_wpebu_ctypevec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Towupper@@YA_W_WPEBU_Ctypevec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _wcerr_func_std_yaaav_basic_ostream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcerr_func@std@@YAAAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wcerr_func_std_yaaeav_basic_ostream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcerr_func@std@@YAAEAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcerr_func_std_yaaav_basic_ostream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcerr_func@std@@YAAAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wcerr_func_std_yaaeav_basic_ostream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcerr_func@std@@YAAEAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcin_func_std_yaaav_basic_istream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcin_func@std@@YAAAV?$basic_istream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wcin_func_std_yaaeav_basic_istream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcin_func@std@@YAAEAV?$basic_istream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcin_func_std_yaaav_basic_istream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcin_func@std@@YAAAV?$basic_istream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wcin_func_std_yaaeav_basic_istream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcin_func@std@@YAAEAV?$basic_istream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wclog_func_std_yaaav_basic_ostream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wclog_func@std@@YAAAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wclog_func_std_yaaeav_basic_ostream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wclog_func@std@@YAAEAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wclog_func_std_yaaav_basic_ostream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wclog_func@std@@YAAAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wclog_func_std_yaaeav_basic_ostream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wclog_func@std@@YAAEAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcout_func_std_yaaav_basic_ostream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcout_func@std@@YAAAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wcout_func_std_yaaeav_basic_ostream_gu_char_traits_g_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcout_func@std@@YAAEAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcout_func_std_yaaav_basic_ostream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcout_func@std@@YAAAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win64` export.
pub fn _wcout_func_std_yaaeav_basic_ostream_wu_char_traits_w_std_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcout_func@std@@YAAEAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcrtomb_yahpadgpahpbu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcrtomb@@YAHPADGPAHPBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _wcrtomb_yahpeadgpeahpebu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcrtomb@@YAHPEADGPEAHPEBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _wcrtomb_yahpad_wpahpbu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcrtomb@@YAHPAD_WPAHPBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _wcrtomb_yahpead_wpeahpebu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcrtomb@@YAHPEAD_WPEAHPEBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _wcscoll_yahpbg000pbu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcscoll@@YAHPBG000PBU_Collvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _wcscoll_yahpebg000pebu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcscoll@@YAHPEBG000PEBU_Collvec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _wcscoll_yahpb_w000pbu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcscoll@@YAHPB_W000PBU_Collvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _wcscoll_yahpeb_w000pebu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcscoll@@YAHPEB_W000PEBU_Collvec@@@Z")
}

/// `@ stub -arch=win32` export.
pub fn _wcsxfrm_yaipag0pbg1pbu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcsxfrm@@YAIPAG0PBG1PBU_Collvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _wcsxfrm_ya_kpeag0pebg1pebu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcsxfrm@@YA_KPEAG0PEBG1PEBU_Collvec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _wcsxfrm_yaipa_w0pb_w1pbu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Wcsxfrm@@YAIPA_W0PB_W1PBU_Collvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn _wcsxfrm_ya_kpea_w0peb_w1pebu_collvec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_Wcsxfrm@@YA_KPEA_W0PEB_W1PEBU_Collvec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _xfsopen_std_yapau_iobuf_pb_w0h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Xfsopen@std@@YAPAU_iobuf@@PB_W0H@Z")
}

/// `@ stub -arch=win64` export.
pub fn _xfsopen_std_yapeau_iobuf_peb_w0h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Xfsopen@std@@YAPEAU_iobuf@@PEB_W0H@Z")
}

/// `@ stub` export.
pub fn _xinvarg_string_base_std_saxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Xinvarg@_String_base@std@@SAXXZ")
}

/// `@ stub` export.
pub fn _xlen_string_base_std_saxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Xlen@_String_base@std@@SAXXZ")
}

/// `@ stub` export.
pub fn _xran_string_base_std_saxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_Xran@_String_base@std@@SAXXZ")
}

/// `@ stub -arch=win32` export.
pub fn __stodx_yanpbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Stodx@@YANPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn __stodx_yanpebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Stodx@@YANPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn __stofx_yampbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Stofx@@YAMPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn __stofx_yampebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Stofx@@YAMPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn __stoldx_yaopbdpapadjpah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Stoldx@@YAOPBDPAPADJPAH@Z")
}

/// `@ stub -arch=win64` export.
pub fn __stoldx_yaopebdpeapeadjpeah_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Stoldx@@YAOPEBDPEAPEADJPEAH@Z")
}

/// `@ stub -arch=win32` export.
pub fn __wcrtomb_lk_yahpad_wpahpbu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__Wcrtomb_lk@@YAHPAD_WPAHPBU_Cvtvec@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn __wcrtomb_lk_yahpead_wpeahpebu_cvtvec_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?__Wcrtomb_lk@@YAHPEAD_WPEAHPEBU_Cvtvec@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn __get_default_appdomain_yajpapauiunknown_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?__get_default_appdomain@@YAJPAPAUIUnknown@@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn __get_default_appdomain_yajpeapeauiunknown_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?__get_default_appdomain@@YAJPEAPEAUIUnknown@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn __query_new_handler_m_yap6mhi_zxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__query_new_handler_m@@YAP6MHI@ZXZ")
}

/// `@ stub -arch=win64` export.
pub fn __query_new_handler_m_yap6mh_k_zxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__query_new_handler_m@@YAP6MH_K@ZXZ")
}

/// `@ stub -arch=win32` export.
pub fn __release_appdomain_yaxpauiunknown_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?__release_appdomain@@YAXPAUIUnknown@@@Z")
}

/// `@ stub -arch=win64` export.
pub fn __release_appdomain_yaxpeauiunknown_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?__release_appdomain@@YAXPEAUIUnknown@@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _beginthread_yaip6mxpax_zi0_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_beginthread@@YAIP6MXPAX@ZI0@Z")
}

/// `@ stub -arch=win64` export.
pub fn _beginthread_ya_kp6mxpeax_zi0_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_beginthread@@YA_KP6MXPEAX@ZI0@Z")
}

/// `@ stub -arch=win32` export.
pub fn _beginthreadex_yaipaxip6mi0_z0ipai_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_beginthreadex@@YAIPAXIP6MI0@Z0IPAI@Z")
}

/// `@ stub -arch=win64` export.
pub fn _beginthreadex_ya_kpeaxip6mi0_z0ipeai_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_beginthreadex@@YA_KPEAXIP6MI0@Z0IPEAI@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _fpieee_flt_yahkpau_exception_pointers_p6mhpau_fpieee_record_z_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_fpieee_flt@@YAHKPAU_EXCEPTION_POINTERS@@P6MHPAU_FPIEEE_RECORD@@@Z@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _fpieee_flt_yahkpeau_exception_pointers_p6mhpeau_fpieee_record_z_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_fpieee_flt@@YAHKPEAU_EXCEPTION_POINTERS@@P6MHPEAU_FPIEEE_RECORD@@@Z@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _set_invalid_parameter_handler_yap6axpb_w00ii_zh_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_set_invalid_parameter_handler@@YAP6AXPB_W00II@ZH@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _set_invalid_parameter_handler_yap6axpeb_w00i_k_zh_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_set_invalid_parameter_handler@@YAP6AXPEB_W00I_K@ZH@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _set_invalid_parameter_handler_yap6mxpb_w00ii_zp6mx000ii_z_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_set_invalid_parameter_handler@@YAP6MXPB_W00II@ZP6MX000II@Z@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn _set_invalid_parameter_handler_yap6mxpeb_w00i_k_zp6mx000i1_z_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?_set_invalid_parameter_handler@@YAP6MXPEB_W00I_K@ZP6MX000I1@Z@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn _set_new_handler_yap6mhi_zp6mhi_z_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_set_new_handler@@YAP6MHI@ZP6MHI@Z@Z")
}

/// `@ stub -arch=win64` export.
pub fn _set_new_handler_yap6mh_k_zp6mh0_z_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_set_new_handler@@YAP6MH_K@ZP6MH0@Z@Z")
}

/// `@ stub` export.
pub fn _set_purecall_handler_yap6axxzh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_set_purecall_handler@@YAP6AXXZH@Z")
}

/// `@ stub` export.
pub fn _set_purecall_handler_yap6mxxzp6mxxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_set_purecall_handler@@YAP6MXXZP6MXXZ@Z")
}

/// `@ stub` export.
pub fn _uncaught_exception_m_std_ya_nxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?_uncaught_exception_m@std@@YA_NXZ")
}

/// `@ stub -arch=win32` export.
pub fn classic_locale_std_saabv12_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?classic@locale@std@@SAABV12@XZ")
}

/// `@ stub -arch=win64` export.
pub fn classic_locale_std_saaebv12_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?classic@locale@std@@SAAEBV12@XZ")
}

/// `@ stub` export.
pub fn empty_locale_std_sa_av12_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?empty@locale@std@@SA?AV12@XZ")
}

/// `@ stub -arch=win32` export.
pub fn facet_register_facet_locale_std_caxpav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?facet_Register@facet@locale@std@@CAXPAV123@@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn facet_register_facet_locale_std_caxpeav123_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?facet_Register@facet@locale@std@@CAXPEAV123@@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn global_locale_std_sa_av12_abv12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?global@locale@std@@SA?AV12@ABV12@@Z")
}

/// `@ stub -arch=win64` export.
pub fn global_locale_std_sa_av12_aebv12_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?global@locale@std@@SA?AV12@AEBV12@@Z")
}

/// `@ stub` export.
pub fn resetiosflags_std_ya_au_smanip_h_1_h_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?resetiosflags@std@@YA?AU?$_Smanip@H@1@H@Z",
    )
}

/// `@ stub` export.
pub fn set_new_handler_std_yap6mxxzp6mxxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?set_new_handler@std@@YAP6MXXZP6MXXZ@Z")
}

/// `@ stub` export.
pub fn set_terminate_yap6mxxzp6mxxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?set_terminate@@YAP6MXXZP6MXXZ@Z")
}

/// `@ stub` export.
pub fn set_unexpected_yap6mxxzp6mxxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?set_unexpected@@YAP6MXXZP6MXXZ@Z")
}

/// `@ stub` export.
pub fn setbase_std_ya_au_smanip_h_1_h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?setbase@std@@YA?AU?$_Smanip@H@1@H@Z")
}

/// `@ stub` export.
pub fn setiosflags_std_ya_au_smanip_h_1_h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?setiosflags@std@@YA?AU?$_Smanip@H@1@H@Z")
}

/// `@ stub -arch=win32` export.
pub fn setprecision_std_ya_au_smanip_h_1_h_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?setprecision@std@@YA?AU?$_Smanip@H@1@H@Z",
    )
}

/// `@ stub -arch=win64` export.
pub fn setprecision_std_ya_au_smanip_j_1_j_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCM80_DLL_NAME,
        "?setprecision@std@@YA?AU?$_Smanip@_J@1@_J@Z",
    )
}

/// `@ stub -arch=win32` export.
pub fn setw_std_ya_au_smanip_h_1_h_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?setw@std@@YA?AU?$_Smanip@H@1@H@Z")
}

/// `@ stub -arch=win64` export.
pub fn setw_std_ya_au_smanip_j_1_j_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?setw@std@@YA?AU?$_Smanip@_J@1@_J@Z")
}

/// `@ stub` export.
pub fn signal_yap6mxh_zhh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?signal@@YAP6MXH@ZHH@Z")
}

/// `@ stub` export.
pub fn signal_yap6mxh_zhp6mxh_z_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "?signal@@YAP6MXH@ZHP6MXH@Z@Z")
}

/// `@ stub` export.
pub fn towctrans() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "towctrans")
}

/// `@ stub` export.
pub fn wctrans() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "wctrans")
}

/// `@ stub` export.
pub fn wctype() -> StubException {
    wine_spec_unimplemented_stub(MSVCM80_DLL_NAME, "wctype")
}
