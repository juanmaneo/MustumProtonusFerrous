//! Oracle-derived test vectors for the ported msvcm80 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) and of the functions in `dlls/msvcm80/msvcm_main.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/msvcm80/`, and running the harness.
//! The harness printed one row per export line of `msvcm80.spec`, in spec order: 296
//! `@ stub` rows and 10 `@ cdecl` rows.
//! ASan and UBSan flagged none of them, so none are excluded.

use super::*;

/// The `@ stub` rows: the helper the port maps each export to, and the export name the harness
/// passed to `__wine_spec_unimplemented_stub`. Every printed row has code `0x80000100`, flags
/// `0x1`, 2 parameters and module `msvcm80.dll`.
type StubRow = (fn() -> StubException, &'static str);

pub(crate) const STUB_ROWS: &[StubRow] = &[
    // row 001 (-arch=win32)
    (
        docallbackindefaultdomain_crtimplementationdetails_yaxp6gjpax_z0_z,
        "?DoCallBackInDefaultDomain@<CrtImplementationDetails>@@YAXP6GJPAX@Z0@Z",
    ),
    // row 002 (-arch=win64)
    (
        docallbackindefaultdomain_crtimplementationdetails_yaxp6ajpeax_z0_z,
        "?DoCallBackInDefaultDomain@<CrtImplementationDetails>@@YAXP6AJPEAX@Z0@Z",
    ),
    // row 012 (-arch=win32)
    (
        _addstd_ios_base_std_saxpav12_z,
        "?_Addstd@ios_base@std@@SAXPAV12@@Z",
    ),
    // row 013 (-arch=win64)
    (
        _addstd_ios_base_std_saxpeav12_z,
        "?_Addstd@ios_base@std@@SAXPEAV12@@Z",
    ),
    // row 014
    (_atexit_yaxp6axxz_z, "?_Atexit@@YAXP6AXXZ@Z"),
    // row 015 (-arch=win32)
    (_badoff_func_std_yaabjxz, "?_BADOFF_func@std@@YAABJXZ"),
    // row 016 (-arch=win64)
    (_badoff_func_std_yaaeb_jxz, "?_BADOFF_func@std@@YAAEB_JXZ"),
    // row 017 (-arch=win32)
    (
        _cerr_func_std_yaaav_basic_ostream_du_char_traits_d_std_1_xz,
        "?_Cerr_func@std@@YAAAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 018 (-arch=win64)
    (
        _cerr_func_std_yaaeav_basic_ostream_du_char_traits_d_std_1_xz,
        "?_Cerr_func@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 019 (-arch=win32)
    (
        _cin_func_std_yaaav_basic_istream_du_char_traits_d_std_1_xz,
        "?_Cin_func@std@@YAAAV?$basic_istream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 020 (-arch=win64)
    (
        _cin_func_std_yaaeav_basic_istream_du_char_traits_d_std_1_xz,
        "?_Cin_func@std@@YAAEAV?$basic_istream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 021 (-arch=win32)
    (
        _clocptr_func_locimp_locale_std_caaapav123_xz,
        "?_Clocptr_func@_Locimp@locale@std@@CAAAPAV123@XZ",
    ),
    // row 022 (-arch=win64)
    (
        _clocptr_func_locimp_locale_std_caaeapeav123_xz,
        "?_Clocptr_func@_Locimp@locale@std@@CAAEAPEAV123@XZ",
    ),
    // row 023 (-arch=win32)
    (
        _clog_func_std_yaaav_basic_ostream_du_char_traits_d_std_1_xz,
        "?_Clog_func@std@@YAAAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 024 (-arch=win64)
    (
        _clog_func_std_yaaeav_basic_ostream_du_char_traits_d_std_1_xz,
        "?_Clog_func@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 025
    (_cosh_yannn_z, "?_Cosh@@YANNN@Z"),
    // row 026 (-arch=win32)
    (
        _cout_func_std_yaaav_basic_ostream_du_char_traits_d_std_1_xz,
        "?_Cout_func@std@@YAAAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 027 (-arch=win64)
    (
        _cout_func_std_yaaeav_basic_ostream_du_char_traits_d_std_1_xz,
        "?_Cout_func@std@@YAAEAV?$basic_ostream@DU?$char_traits@D@std@@@1@XZ",
    ),
    // row 028 (-arch=win32)
    (
        _debug_message_std_yaxpbd0_z,
        "?_Debug_message@std@@YAXPBD0@Z",
    ),
    // row 029 (-arch=win64)
    (
        _debug_message_std_yaxpebd0_z,
        "?_Debug_message@std@@YAXPEBD0@Z",
    ),
    // row 030 (-arch=win32)
    (_dnorm_yafpag_z, "?_Dnorm@@YAFPAG@Z"),
    // row 031 (-arch=win64)
    (_dnorm_yafpeag_z, "?_Dnorm@@YAFPEAG@Z"),
    // row 032 (-arch=win32)
    (_dscale_yafpanj_z, "?_Dscale@@YAFPANJ@Z"),
    // row 033 (-arch=win64)
    (_dscale_yafpeanj_z, "?_Dscale@@YAFPEANJ@Z"),
    // row 034 (-arch=win32)
    (_dtentox_yannjpah_z, "?_Dtentox@@YANNJPAH@Z"),
    // row 035 (-arch=win64)
    (_dtentox_yannjpeah_z, "?_Dtentox@@YANNJPEAH@Z"),
    // row 036 (-arch=win32)
    (_dtest_yafpan_z, "?_Dtest@@YAFPAN@Z"),
    // row 037 (-arch=win64)
    (_dtest_yafpean_z, "?_Dtest@@YAFPEAN@Z"),
    // row 038 (-arch=win32)
    (_dunscale_yafpafpan_z, "?_Dunscale@@YAFPAFPAN@Z"),
    // row 039 (-arch=win64)
    (_dunscale_yafpeafpean_z, "?_Dunscale@@YAFPEAFPEAN@Z"),
    // row 040 (-arch=win32)
    (_exp_yafpannf_z, "?_Exp@@YAFPANNF@Z"),
    // row 041 (-arch=win64)
    (_exp_yafpeannf_z, "?_Exp@@YAFPEANNF@Z"),
    // row 042
    (_fcosh_yammm_z, "?_FCosh@@YAMMM@Z"),
    // row 043 (-arch=win32)
    (_fdnorm_yafpag_z, "?_FDnorm@@YAFPAG@Z"),
    // row 044 (-arch=win64)
    (_fdnorm_yafpeag_z, "?_FDnorm@@YAFPEAG@Z"),
    // row 045 (-arch=win32)
    (_fdscale_yafpamj_z, "?_FDscale@@YAFPAMJ@Z"),
    // row 046 (-arch=win64)
    (_fdscale_yafpeamj_z, "?_FDscale@@YAFPEAMJ@Z"),
    // row 047 (-arch=win32)
    (_fdtentox_yammjpah_z, "?_FDtentox@@YAMMJPAH@Z"),
    // row 048 (-arch=win64)
    (_fdtentox_yammjpeah_z, "?_FDtentox@@YAMMJPEAH@Z"),
    // row 049 (-arch=win32)
    (_fdtest_yafpam_z, "?_FDtest@@YAFPAM@Z"),
    // row 050 (-arch=win64)
    (_fdtest_yafpeam_z, "?_FDtest@@YAFPEAM@Z"),
    // row 051 (-arch=win32)
    (_fdunscale_yafpafpam_z, "?_FDunscale@@YAFPAFPAM@Z"),
    // row 052 (-arch=win64)
    (_fdunscale_yafpeafpeam_z, "?_FDunscale@@YAFPEAFPEAM@Z"),
    // row 053 (-arch=win32)
    (_fexp_yafpammf_z, "?_FExp@@YAFPAMMF@Z"),
    // row 054 (-arch=win64)
    (_fexp_yafpeammf_z, "?_FExp@@YAFPEAMMF@Z"),
    // row 055
    (_fsinh_yammm_z, "?_FSinh@@YAMMM@Z"),
    // row 056 (-arch=win32)
    (
        _fiopen_std_yapau_iobuf_pbdhh_z,
        "?_Fiopen@std@@YAPAU_iobuf@@PBDHH@Z",
    ),
    // row 057 (-arch=win64)
    (
        _fiopen_std_yapeau_iobuf_pebdhh_z,
        "?_Fiopen@std@@YAPEAU_iobuf@@PEBDHH@Z",
    ),
    // row 058 (-arch=win32)
    (
        _fiopen_std_yapau_iobuf_pbghh_z,
        "?_Fiopen@std@@YAPAU_iobuf@@PBGHH@Z",
    ),
    // row 059 (-arch=win64)
    (
        _fiopen_std_yapeau_iobuf_pebghh_z,
        "?_Fiopen@std@@YAPEAU_iobuf@@PEBGHH@Z",
    ),
    // row 060 (-arch=win32)
    (
        _fiopen_std_yapau_iobuf_pb_whh_z,
        "?_Fiopen@std@@YAPAU_iobuf@@PB_WHH@Z",
    ),
    // row 061 (-arch=win64)
    (
        _fiopen_std_yapeau_iobuf_peb_whh_z,
        "?_Fiopen@std@@YAPEAU_iobuf@@PEB_WHH@Z",
    ),
    // row 062 (-arch=win32)
    (_fpz_func_std_yaaa_jxz, "?_Fpz_func@std@@YAAA_JXZ"),
    // row 063 (-arch=win64)
    (_fpz_func_std_yaaea_jxz, "?_Fpz_func@std@@YAAEA_JXZ"),
    // row 064
    (_getcoll_ya_au_collvec_xz, "?_Getcoll@@YA?AU_Collvec@@XZ"),
    // row 065
    (
        _getctype_ya_au_ctypevec_xz,
        "?_Getctype@@YA?AU_Ctypevec@@XZ",
    ),
    // row 066
    (_getcvt_ya_au_cvtvec_xz, "?_Getcvt@@YA?AU_Cvtvec@@XZ"),
    // row 067
    (_getdateorder_yahxz, "?_Getdateorder@@YAHXZ"),
    // row 068 (-arch=win32)
    (
        _getgloballocale_locale_std_capav_locimp_12_xz,
        "?_Getgloballocale@locale@std@@CAPAV_Locimp@12@XZ",
    ),
    // row 069 (-arch=win64)
    (
        _getgloballocale_locale_std_capeav_locimp_12_xz,
        "?_Getgloballocale@locale@std@@CAPEAV_Locimp@12@XZ",
    ),
    // row 070 (-arch=win32)
    (
        _getwctype_yafgpbu_ctypevec_z,
        "?_Getwctype@@YAFGPBU_Ctypevec@@@Z",
    ),
    // row 071 (-arch=win64)
    (
        _getwctype_yafgpebu_ctypevec_z,
        "?_Getwctype@@YAFGPEBU_Ctypevec@@@Z",
    ),
    // row 072 (-arch=win32)
    (
        _getwctype_yaf_wpbu_ctypevec_z,
        "?_Getwctype@@YAF_WPBU_Ctypevec@@@Z",
    ),
    // row 073 (-arch=win64)
    (
        _getwctype_yaf_wpebu_ctypevec_z,
        "?_Getwctype@@YAF_WPEBU_Ctypevec@@@Z",
    ),
    // row 074 (-arch=win32)
    (
        _getwctypes_yapbgpbg0pafpbu_ctypevec_z,
        "?_Getwctypes@@YAPBGPBG0PAFPBU_Ctypevec@@@Z",
    ),
    // row 075 (-arch=win64)
    (
        _getwctypes_yapebgpebg0peafpebu_ctypevec_z,
        "?_Getwctypes@@YAPEBGPEBG0PEAFPEBU_Ctypevec@@@Z",
    ),
    // row 076 (-arch=win32)
    (
        _getwctypes_yapb_wpb_w0pafpbu_ctypevec_z,
        "?_Getwctypes@@YAPB_WPB_W0PAFPBU_Ctypevec@@@Z",
    ),
    // row 077 (-arch=win64)
    (
        _getwctypes_yapeb_wpeb_w0peafpebu_ctypevec_z,
        "?_Getwctypes@@YAPEB_WPEB_W0PEAFPEBU_Ctypevec@@@Z",
    ),
    // row 078 (-arch=win32)
    (
        _id_cnt_func_id_locale_std_caaahxz,
        "?_Id_cnt_func@id@locale@std@@CAAAHXZ",
    ),
    // row 079 (-arch=win64)
    (
        _id_cnt_func_id_locale_std_caaeahxz,
        "?_Id_cnt_func@id@locale@std@@CAAEAHXZ",
    ),
    // row 080 (-arch=win32)
    (
        _id_func_codecvt_gdh_std_saaavid_locale_2_xz,
        "?_Id_func@?$codecvt@GDH@std@@SAAAVid@locale@2@XZ",
    ),
    // row 081 (-arch=win64)
    (
        _id_func_codecvt_gdh_std_saaeavid_locale_2_xz,
        "?_Id_func@?$codecvt@GDH@std@@SAAEAVid@locale@2@XZ",
    ),
    // row 082 (-arch=win32)
    (
        _id_func_codecvt_wdh_std_saaavid_locale_2_xz,
        "?_Id_func@?$codecvt@_WDH@std@@SAAAVid@locale@2@XZ",
    ),
    // row 083 (-arch=win64)
    (
        _id_func_codecvt_wdh_std_saaeavid_locale_2_xz,
        "?_Id_func@?$codecvt@_WDH@std@@SAAEAVid@locale@2@XZ",
    ),
    // row 084 (-arch=win32)
    (
        _id_func_ctype_d_std_saaavid_locale_2_xz,
        "?_Id_func@?$ctype@D@std@@SAAAVid@locale@2@XZ",
    ),
    // row 085 (-arch=win64)
    (
        _id_func_ctype_d_std_saaeavid_locale_2_xz,
        "?_Id_func@?$ctype@D@std@@SAAEAVid@locale@2@XZ",
    ),
    // row 086 (-arch=win32)
    (
        _id_func_ctype_g_std_saaavid_locale_2_xz,
        "?_Id_func@?$ctype@G@std@@SAAAVid@locale@2@XZ",
    ),
    // row 087 (-arch=win64)
    (
        _id_func_ctype_g_std_saaeavid_locale_2_xz,
        "?_Id_func@?$ctype@G@std@@SAAEAVid@locale@2@XZ",
    ),
    // row 088 (-arch=win32)
    (
        _id_func_ctype_w_std_saaavid_locale_2_xz,
        "?_Id_func@?$ctype@_W@std@@SAAAVid@locale@2@XZ",
    ),
    // row 089 (-arch=win64)
    (
        _id_func_ctype_w_std_saaeavid_locale_2_xz,
        "?_Id_func@?$ctype@_W@std@@SAAEAVid@locale@2@XZ",
    ),
    // row 090 (-arch=win32)
    (
        _index_func_ios_base_std_caaahxz,
        "?_Index_func@ios_base@std@@CAAAHXZ",
    ),
    // row 091 (-arch=win64)
    (
        _index_func_ios_base_std_caaeahxz,
        "?_Index_func@ios_base@std@@CAAEAHXZ",
    ),
    // row 092 (-arch=win32)
    (
        _init_locale_std_capav_locimp_12_xz,
        "?_Init@locale@std@@CAPAV_Locimp@12@XZ",
    ),
    // row 093 (-arch=win64)
    (
        _init_locale_std_capeav_locimp_12_xz,
        "?_Init@locale@std@@CAPEAV_Locimp@12@XZ",
    ),
    // row 094 (-arch=win32)
    (
        _init_cnt_func_init_ios_base_std_caaahxz,
        "?_Init_cnt_func@Init@ios_base@std@@CAAAHXZ",
    ),
    // row 095 (-arch=win64)
    (
        _init_cnt_func_init_ios_base_std_caaeahxz,
        "?_Init_cnt_func@Init@ios_base@std@@CAAEAHXZ",
    ),
    // row 096 (-arch=win32)
    (
        _init_ctor_init_ios_base_std_caxpav123_z,
        "?_Init_ctor@Init@ios_base@std@@CAXPAV123@@Z",
    ),
    // row 097 (-arch=win64)
    (
        _init_ctor_init_ios_base_std_caxpeav123_z,
        "?_Init_ctor@Init@ios_base@std@@CAXPEAV123@@Z",
    ),
    // row 098 (-arch=win32)
    (
        _init_dtor_init_ios_base_std_caxpav123_z,
        "?_Init_dtor@Init@ios_base@std@@CAXPAV123@@Z",
    ),
    // row 099 (-arch=win64)
    (
        _init_dtor_init_ios_base_std_caxpeav123_z,
        "?_Init_dtor@Init@ios_base@std@@CAXPEAV123@@Z",
    ),
    // row 100 (-arch=win32)
    (
        _init_locks_ctor_init_locks_std_caxpav12_z,
        "?_Init_locks_ctor@_Init_locks@std@@CAXPAV12@@Z",
    ),
    // row 101 (-arch=win64)
    (
        _init_locks_ctor_init_locks_std_caxpeav12_z,
        "?_Init_locks_ctor@_Init_locks@std@@CAXPEAV12@@Z",
    ),
    // row 102 (-arch=win32)
    (
        _init_locks_dtor_init_locks_std_caxpav12_z,
        "?_Init_locks_dtor@_Init_locks@std@@CAXPAV12@@Z",
    ),
    // row 103 (-arch=win64)
    (
        _init_locks_dtor_init_locks_std_caxpeav12_z,
        "?_Init_locks_dtor@_Init_locks@std@@CAXPEAV12@@Z",
    ),
    // row 104 (-arch=win32)
    (
        _ios_base_dtor_ios_base_std_caxpav12_z,
        "?_Ios_base_dtor@ios_base@std@@CAXPAV12@@Z",
    ),
    // row 105 (-arch=win64)
    (
        _ios_base_dtor_ios_base_std_caxpeav12_z,
        "?_Ios_base_dtor@ios_base@std@@CAXPEAV12@@Z",
    ),
    // row 106
    (_lcosh_yaooo_z, "?_LCosh@@YAOOO@Z"),
    // row 107 (-arch=win32)
    (_ldscale_yafpaoj_z, "?_LDscale@@YAFPAOJ@Z"),
    // row 108 (-arch=win64)
    (_ldscale_yafpeaoj_z, "?_LDscale@@YAFPEAOJ@Z"),
    // row 109 (-arch=win32)
    (_ldtentox_yaoojpah_z, "?_LDtentox@@YAOOJPAH@Z"),
    // row 110 (-arch=win64)
    (_ldtentox_yaoojpeah_z, "?_LDtentox@@YAOOJPEAH@Z"),
    // row 111 (-arch=win32)
    (_ldtest_yafpao_z, "?_LDtest@@YAFPAO@Z"),
    // row 112 (-arch=win64)
    (_ldtest_yafpeao_z, "?_LDtest@@YAFPEAO@Z"),
    // row 113 (-arch=win32)
    (_ldunscale_yafpafpao_z, "?_LDunscale@@YAFPAFPAO@Z"),
    // row 114 (-arch=win64)
    (_ldunscale_yafpeafpeao_z, "?_LDunscale@@YAFPEAFPEAO@Z"),
    // row 115 (-arch=win32)
    (_lexp_yafpaoof_z, "?_LExp@@YAFPAOOF@Z"),
    // row 116 (-arch=win64)
    (_lexp_yafpeaoof_z, "?_LExp@@YAFPEAOOF@Z"),
    // row 117 (-arch=win32)
    (_lpoly_yaoopboh_z, "?_LPoly@@YAOOPBOH@Z"),
    // row 118 (-arch=win64)
    (_lpoly_yaoopeboh_z, "?_LPoly@@YAOOPEBOH@Z"),
    // row 119
    (_lsinh_yaooo_z, "?_LSinh@@YAOOO@Z"),
    // row 120 (-arch=win32)
    (
        _locimp_addfac_locimp_locale_std_caxpav123_pavfacet_23_i_z,
        "?_Locimp_Addfac@_Locimp@locale@std@@CAXPAV123@PAVfacet@23@I@Z",
    ),
    // row 121 (-arch=win64)
    (
        _locimp_addfac_locimp_locale_std_caxpeav123_peavfacet_23_k_z,
        "?_Locimp_Addfac@_Locimp@locale@std@@CAXPEAV123@PEAVfacet@23@_K@Z",
    ),
    // row 122 (-arch=win32)
    (
        _locimp_ctor_locimp_locale_std_caxpav123_abv123_z,
        "?_Locimp_ctor@_Locimp@locale@std@@CAXPAV123@ABV123@@Z",
    ),
    // row 123 (-arch=win64)
    (
        _locimp_ctor_locimp_locale_std_caxpeav123_aebv123_z,
        "?_Locimp_ctor@_Locimp@locale@std@@CAXPEAV123@AEBV123@@Z",
    ),
    // row 124 (-arch=win32)
    (
        _locimp_dtor_locimp_locale_std_caxpav123_z,
        "?_Locimp_dtor@_Locimp@locale@std@@CAXPAV123@@Z",
    ),
    // row 125 (-arch=win64)
    (
        _locimp_dtor_locimp_locale_std_caxpeav123_z,
        "?_Locimp_dtor@_Locimp@locale@std@@CAXPEAV123@@Z",
    ),
    // row 126 (-arch=win32)
    (
        _locinfo_addcats_locinfo_std_saaav12_pav12_hpbd_z,
        "?_Locinfo_Addcats@_Locinfo@std@@SAAAV12@PAV12@HPBD@Z",
    ),
    // row 127 (-arch=win64)
    (
        _locinfo_addcats_locinfo_std_saaeav12_peav12_hpebd_z,
        "?_Locinfo_Addcats@_Locinfo@std@@SAAEAV12@PEAV12@HPEBD@Z",
    ),
    // row 128 (-arch=win32)
    (
        _locinfo_ctor_locinfo_std_saxpav12_hpbd_z,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPAV12@HPBD@Z",
    ),
    // row 129 (-arch=win64)
    (
        _locinfo_ctor_locinfo_std_saxpeav12_hpebd_z,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@HPEBD@Z",
    ),
    // row 130 (-arch=win32)
    (
        _locinfo_ctor_locinfo_std_saxpav12_pbd_z,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPAV12@PBD@Z",
    ),
    // row 131 (-arch=win64)
    (
        _locinfo_ctor_locinfo_std_saxpeav12_pebd_z,
        "?_Locinfo_ctor@_Locinfo@std@@SAXPEAV12@PEBD@Z",
    ),
    // row 132 (-arch=win32)
    (
        _locinfo_dtor_locinfo_std_saxpav12_z,
        "?_Locinfo_dtor@_Locinfo@std@@SAXPAV12@@Z",
    ),
    // row 133 (-arch=win64)
    (
        _locinfo_dtor_locinfo_std_saxpeav12_z,
        "?_Locinfo_dtor@_Locinfo@std@@SAXPEAV12@@Z",
    ),
    // row 134 (-arch=win32)
    (
        _lockit_ctor_lockit_std_caxpav12_z,
        "?_Lockit_ctor@_Lockit@std@@CAXPAV12@@Z",
    ),
    // row 135 (-arch=win64)
    (
        _lockit_ctor_lockit_std_caxpeav12_z,
        "?_Lockit_ctor@_Lockit@std@@CAXPEAV12@@Z",
    ),
    // row 136 (-arch=win32)
    (
        _lockit_ctor_lockit_std_caxpav12_h_z,
        "?_Lockit_ctor@_Lockit@std@@CAXPAV12@H@Z",
    ),
    // row 137 (-arch=win64)
    (
        _lockit_ctor_lockit_std_caxpeav12_h_z,
        "?_Lockit_ctor@_Lockit@std@@CAXPEAV12@H@Z",
    ),
    // row 138
    (
        _lockit_ctor_lockit_std_saxh_z,
        "?_Lockit_ctor@_Lockit@std@@SAXH@Z",
    ),
    // row 139 (-arch=win32)
    (
        _lockit_dtor_lockit_std_caxpav12_z,
        "?_Lockit_dtor@_Lockit@std@@CAXPAV12@@Z",
    ),
    // row 140 (-arch=win64)
    (
        _lockit_dtor_lockit_std_caxpeav12_z,
        "?_Lockit_dtor@_Lockit@std@@CAXPEAV12@@Z",
    ),
    // row 141
    (
        _lockit_dtor_lockit_std_saxh_z,
        "?_Lockit_dtor@_Lockit@std@@SAXH@Z",
    ),
    // row 142 (-arch=win32)
    (
        _makeloc_locimp_locale_std_capav123_abv_locinfo_3_hpav123_pbv23_z,
        "?_Makeloc@_Locimp@locale@std@@CAPAV123@ABV_Locinfo@3@HPAV123@PBV23@@Z",
    ),
    // row 143 (-arch=win64)
    (
        _makeloc_locimp_locale_std_capeav123_aebv_locinfo_3_hpeav123_pebv23_z,
        "?_Makeloc@_Locimp@locale@std@@CAPEAV123@AEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    ),
    // row 144 (-arch=win32)
    (
        _makeushloc_locimp_locale_std_caxabv_locinfo_3_hpav123_pbv23_z,
        "?_Makeushloc@_Locimp@locale@std@@CAXABV_Locinfo@3@HPAV123@PBV23@@Z",
    ),
    // row 145 (-arch=win64)
    (
        _makeushloc_locimp_locale_std_caxaebv_locinfo_3_hpeav123_pebv23_z,
        "?_Makeushloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    ),
    // row 146 (-arch=win32)
    (
        _makewloc_locimp_locale_std_caxabv_locinfo_3_hpav123_pbv23_z,
        "?_Makewloc@_Locimp@locale@std@@CAXABV_Locinfo@3@HPAV123@PBV23@@Z",
    ),
    // row 147 (-arch=win64)
    (
        _makewloc_locimp_locale_std_caxaebv_locinfo_3_hpeav123_pebv23_z,
        "?_Makewloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    ),
    // row 148 (-arch=win32)
    (
        _makexloc_locimp_locale_std_caxabv_locinfo_3_hpav123_pbv23_z,
        "?_Makexloc@_Locimp@locale@std@@CAXABV_Locinfo@3@HPAV123@PBV23@@Z",
    ),
    // row 149 (-arch=win64)
    (
        _makexloc_locimp_locale_std_caxaebv_locinfo_3_hpeav123_pebv23_z,
        "?_Makexloc@_Locimp@locale@std@@CAXAEBV_Locinfo@3@HPEAV123@PEBV23@@Z",
    ),
    // row 150 (-arch=win32)
    (
        _mbrtowc_yahpagpbdipahpbu_cvtvec_z,
        "?_Mbrtowc@@YAHPAGPBDIPAHPBU_Cvtvec@@@Z",
    ),
    // row 151 (-arch=win64)
    (
        _mbrtowc_yahpeagpebd_kpeahpebu_cvtvec_z,
        "?_Mbrtowc@@YAHPEAGPEBD_KPEAHPEBU_Cvtvec@@@Z",
    ),
    // row 152 (-arch=win32)
    (
        _mbrtowc_yahpa_wpbdipahpbu_cvtvec_z,
        "?_Mbrtowc@@YAHPA_WPBDIPAHPBU_Cvtvec@@@Z",
    ),
    // row 153 (-arch=win64)
    (
        _mbrtowc_yahpea_wpebd_kpeahpebu_cvtvec_z,
        "?_Mbrtowc@@YAHPEA_WPEBD_KPEAHPEBU_Cvtvec@@@Z",
    ),
    // row 154 (-arch=win32)
    (
        _mtxdst_yaxpau_rtl_critical_section_z,
        "?_Mtxdst@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 155 (-arch=win64)
    (
        _mtxdst_yaxpeau_rtl_critical_section_z,
        "?_Mtxdst@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 156 (-arch=win32)
    (
        _mtxinit_yaxpau_rtl_critical_section_z,
        "?_Mtxinit@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 157 (-arch=win64)
    (
        _mtxinit_yaxpeau_rtl_critical_section_z,
        "?_Mtxinit@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 158 (-arch=win32)
    (
        _mtxlock_yaxpau_rtl_critical_section_z,
        "?_Mtxlock@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 159 (-arch=win64)
    (
        _mtxlock_yaxpeau_rtl_critical_section_z,
        "?_Mtxlock@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 160 (-arch=win32)
    (
        _mtxunlock_yaxpau_rtl_critical_section_z,
        "?_Mtxunlock@@YAXPAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 161 (-arch=win64)
    (
        _mtxunlock_yaxpeau_rtl_critical_section_z,
        "?_Mtxunlock@@YAXPEAU_RTL_CRITICAL_SECTION@@@Z",
    ),
    // row 162 (-arch=win32)
    (
        _mutex_lock_mutex_std_caxpav12_z,
        "?_Mutex_Lock@_Mutex@std@@CAXPAV12@@Z",
    ),
    // row 163 (-arch=win64)
    (
        _mutex_lock_mutex_std_caxpeav12_z,
        "?_Mutex_Lock@_Mutex@std@@CAXPEAV12@@Z",
    ),
    // row 164 (-arch=win32)
    (
        _mutex_unlock_mutex_std_caxpav12_z,
        "?_Mutex_Unlock@_Mutex@std@@CAXPAV12@@Z",
    ),
    // row 165 (-arch=win64)
    (
        _mutex_unlock_mutex_std_caxpeav12_z,
        "?_Mutex_Unlock@_Mutex@std@@CAXPEAV12@@Z",
    ),
    // row 166 (-arch=win32)
    (
        _mutex_ctor_mutex_std_caxpav12_z,
        "?_Mutex_ctor@_Mutex@std@@CAXPAV12@@Z",
    ),
    // row 167 (-arch=win64)
    (
        _mutex_ctor_mutex_std_caxpeav12_z,
        "?_Mutex_ctor@_Mutex@std@@CAXPEAV12@@Z",
    ),
    // row 168 (-arch=win32)
    (
        _mutex_dtor_mutex_std_caxpav12_z,
        "?_Mutex_dtor@_Mutex@std@@CAXPAV12@@Z",
    ),
    // row 169 (-arch=win64)
    (
        _mutex_dtor_mutex_std_caxpeav12_z,
        "?_Mutex_dtor@_Mutex@std@@CAXPEAV12@@Z",
    ),
    // row 170
    (_nomemory_std_yaxxz, "?_Nomemory@std@@YAXXZ"),
    // row 171 (-arch=win32)
    (_once_yaxpajp6axxz_z, "?_Once@@YAXPAJP6AXXZ@Z"),
    // row 172 (-arch=win64)
    (_once_yaxpeajp6axxz_z, "?_Once@@YAXPEAJP6AXXZ@Z"),
    // row 173 (-arch=win32)
    (_poly_yannpbnh_z, "?_Poly@@YANNPBNH@Z"),
    // row 174 (-arch=win64)
    (_poly_yannpebnh_z, "?_Poly@@YANNPEBNH@Z"),
    // row 175 (-arch=win32)
    (
        _setgloballocale_locale_std_caxpax_z,
        "?_Setgloballocale@locale@std@@CAXPAX@Z",
    ),
    // row 176 (-arch=win64)
    (
        _setgloballocale_locale_std_caxpeax_z,
        "?_Setgloballocale@locale@std@@CAXPEAX@Z",
    ),
    // row 177
    (_sinh_yannn_z, "?_Sinh@@YANNN@Z"),
    // row 178 (-arch=win32)
    (_stod_yanpbdpapadj_z, "?_Stod@@YANPBDPAPADJ@Z"),
    // row 179 (-arch=win64)
    (_stod_yanpebdpeapeadj_z, "?_Stod@@YANPEBDPEAPEADJ@Z"),
    // row 180 (-arch=win32)
    (_stodx_yanpbdpapadjpah_z, "?_Stodx@@YANPBDPAPADJPAH@Z"),
    // row 181 (-arch=win64)
    (
        _stodx_yanpebdpeapeadjpeah_z,
        "?_Stodx@@YANPEBDPEAPEADJPEAH@Z",
    ),
    // row 182 (-arch=win32)
    (_stof_yampbdpapadjpah_z, "?_Stof@@YAMPBDPAPADJPAH@Z"),
    // row 183 (-arch=win64)
    (_stof_yampebdpeapeadjpeah_z, "?_Stof@@YAMPEBDPEAPEADJPEAH@Z"),
    // row 184 (-arch=win32)
    (_stoflt_yahpbd0papadqajh_z, "?_Stoflt@@YAHPBD0PAPADQAJH@Z"),
    // row 185 (-arch=win64)
    (
        _stoflt_yahpebd0peapeadqeajh_z,
        "?_Stoflt@@YAHPEBD0PEAPEADQEAJH@Z",
    ),
    // row 186 (-arch=win32)
    (_stofx_yampbdpapadjpah_z, "?_Stofx@@YAMPBDPAPADJPAH@Z"),
    // row 187 (-arch=win64)
    (
        _stofx_yampebdpeapeadjpeah_z,
        "?_Stofx@@YAMPEBDPEAPEADJPEAH@Z",
    ),
    // row 188 (-arch=win32)
    (_stold_yaopbdpapadjpah_z, "?_Stold@@YAOPBDPAPADJPAH@Z"),
    // row 189 (-arch=win64)
    (
        _stold_yaopebdpeapeadjpeah_z,
        "?_Stold@@YAOPEBDPEAPEADJPEAH@Z",
    ),
    // row 190 (-arch=win32)
    (_stoldx_yaopbdpapadjpah_z, "?_Stoldx@@YAOPBDPAPADJPAH@Z"),
    // row 191 (-arch=win64)
    (
        _stoldx_yaopebdpeapeadjpeah_z,
        "?_Stoldx@@YAOPEBDPEAPEADJPEAH@Z",
    ),
    // row 192 (-arch=win32)
    (_stolx_yajpbdpapadhpah_z, "?_Stolx@@YAJPBDPAPADHPAH@Z"),
    // row 193 (-arch=win64)
    (
        _stolx_yajpebdpeapeadhpeah_z,
        "?_Stolx@@YAJPEBDPEAPEADHPEAH@Z",
    ),
    // row 194 (-arch=win32)
    (_stopfx_yahpapbdpapad_z, "?_Stopfx@@YAHPAPBDPAPAD@Z"),
    // row 195 (-arch=win64)
    (_stopfx_yahpeapebdpeapead_z, "?_Stopfx@@YAHPEAPEBDPEAPEAD@Z"),
    // row 196 (-arch=win32)
    (_stoul_yakpbdpapadh_z, "?_Stoul@@YAKPBDPAPADH@Z"),
    // row 197 (-arch=win64)
    (_stoul_yakpebdpeapeadh_z, "?_Stoul@@YAKPEBDPEAPEADH@Z"),
    // row 198 (-arch=win32)
    (_stoulx_yakpbdpapadhpah_z, "?_Stoulx@@YAKPBDPAPADHPAH@Z"),
    // row 199 (-arch=win64)
    (
        _stoulx_yakpebdpeapeadhpeah_z,
        "?_Stoulx@@YAKPEBDPEAPEADHPEAH@Z",
    ),
    // row 200 (-arch=win32)
    (_stoxflt_yahpbd0papadqajh_z, "?_Stoxflt@@YAHPBD0PAPADQAJH@Z"),
    // row 201 (-arch=win64)
    (
        _stoxflt_yahpebd0peapeadqeajh_z,
        "?_Stoxflt@@YAHPEBD0PEAPEADQEAJH@Z",
    ),
    // row 202 (-arch=win32)
    (
        _strcoll_yahpbd000pbu_collvec_z,
        "?_Strcoll@@YAHPBD000PBU_Collvec@@@Z",
    ),
    // row 203 (-arch=win64)
    (
        _strcoll_yahpebd000pebu_collvec_z,
        "?_Strcoll@@YAHPEBD000PEBU_Collvec@@@Z",
    ),
    // row 204 (-arch=win32)
    (
        _strxfrm_yaipad0pbd1pbu_collvec_z,
        "?_Strxfrm@@YAIPAD0PBD1PBU_Collvec@@@Z",
    ),
    // row 205 (-arch=win64)
    (
        _strxfrm_ya_kpead0pebd1pebu_collvec_z,
        "?_Strxfrm@@YA_KPEAD0PEBD1PEBU_Collvec@@@Z",
    ),
    // row 206 (-arch=win32)
    (
        _sync_func_ios_base_std_caaa_nxz,
        "?_Sync_func@ios_base@std@@CAAA_NXZ",
    ),
    // row 207 (-arch=win64)
    (
        _sync_func_ios_base_std_caaea_nxz,
        "?_Sync_func@ios_base@std@@CAAEA_NXZ",
    ),
    // row 208 (-arch=win32)
    (
        _throw_std_yaxabvexception_stdext_z,
        "?_Throw@std@@YAXABVexception@stdext@@@Z",
    ),
    // row 209 (-arch=win64)
    (
        _throw_std_yaxaebvexception_stdext_z,
        "?_Throw@std@@YAXAEBVexception@stdext@@@Z",
    ),
    // row 210 (-arch=win32)
    (
        _tolower_yahhpbu_ctypevec_z,
        "?_Tolower@@YAHHPBU_Ctypevec@@@Z",
    ),
    // row 211 (-arch=win64)
    (
        _tolower_yahhpebu_ctypevec_z,
        "?_Tolower@@YAHHPEBU_Ctypevec@@@Z",
    ),
    // row 212 (-arch=win32)
    (
        _toupper_yahhpbu_ctypevec_z,
        "?_Toupper@@YAHHPBU_Ctypevec@@@Z",
    ),
    // row 213 (-arch=win64)
    (
        _toupper_yahhpebu_ctypevec_z,
        "?_Toupper@@YAHHPEBU_Ctypevec@@@Z",
    ),
    // row 214 (-arch=win32)
    (
        _towlower_yaggpbu_ctypevec_z,
        "?_Towlower@@YAGGPBU_Ctypevec@@@Z",
    ),
    // row 215 (-arch=win64)
    (
        _towlower_yaggpebu_ctypevec_z,
        "?_Towlower@@YAGGPEBU_Ctypevec@@@Z",
    ),
    // row 216 (-arch=win32)
    (
        _towlower_ya_w_wpbu_ctypevec_z,
        "?_Towlower@@YA_W_WPBU_Ctypevec@@@Z",
    ),
    // row 217 (-arch=win64)
    (
        _towlower_ya_w_wpebu_ctypevec_z,
        "?_Towlower@@YA_W_WPEBU_Ctypevec@@@Z",
    ),
    // row 218 (-arch=win32)
    (
        _towupper_yaggpbu_ctypevec_z,
        "?_Towupper@@YAGGPBU_Ctypevec@@@Z",
    ),
    // row 219 (-arch=win64)
    (
        _towupper_yaggpebu_ctypevec_z,
        "?_Towupper@@YAGGPEBU_Ctypevec@@@Z",
    ),
    // row 220 (-arch=win32)
    (
        _towupper_ya_w_wpbu_ctypevec_z,
        "?_Towupper@@YA_W_WPBU_Ctypevec@@@Z",
    ),
    // row 221 (-arch=win64)
    (
        _towupper_ya_w_wpebu_ctypevec_z,
        "?_Towupper@@YA_W_WPEBU_Ctypevec@@@Z",
    ),
    // row 222 (-arch=win32)
    (
        _wcerr_func_std_yaaav_basic_ostream_gu_char_traits_g_std_1_xz,
        "?_Wcerr_func@std@@YAAAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 223 (-arch=win64)
    (
        _wcerr_func_std_yaaeav_basic_ostream_gu_char_traits_g_std_1_xz,
        "?_Wcerr_func@std@@YAAEAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 224 (-arch=win32)
    (
        _wcerr_func_std_yaaav_basic_ostream_wu_char_traits_w_std_1_xz,
        "?_Wcerr_func@std@@YAAAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 225 (-arch=win64)
    (
        _wcerr_func_std_yaaeav_basic_ostream_wu_char_traits_w_std_1_xz,
        "?_Wcerr_func@std@@YAAEAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 226 (-arch=win32)
    (
        _wcin_func_std_yaaav_basic_istream_gu_char_traits_g_std_1_xz,
        "?_Wcin_func@std@@YAAAV?$basic_istream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 227 (-arch=win64)
    (
        _wcin_func_std_yaaeav_basic_istream_gu_char_traits_g_std_1_xz,
        "?_Wcin_func@std@@YAAEAV?$basic_istream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 228 (-arch=win32)
    (
        _wcin_func_std_yaaav_basic_istream_wu_char_traits_w_std_1_xz,
        "?_Wcin_func@std@@YAAAV?$basic_istream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 229 (-arch=win64)
    (
        _wcin_func_std_yaaeav_basic_istream_wu_char_traits_w_std_1_xz,
        "?_Wcin_func@std@@YAAEAV?$basic_istream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 230 (-arch=win32)
    (
        _wclog_func_std_yaaav_basic_ostream_gu_char_traits_g_std_1_xz,
        "?_Wclog_func@std@@YAAAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 231 (-arch=win64)
    (
        _wclog_func_std_yaaeav_basic_ostream_gu_char_traits_g_std_1_xz,
        "?_Wclog_func@std@@YAAEAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 232 (-arch=win32)
    (
        _wclog_func_std_yaaav_basic_ostream_wu_char_traits_w_std_1_xz,
        "?_Wclog_func@std@@YAAAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 233 (-arch=win64)
    (
        _wclog_func_std_yaaeav_basic_ostream_wu_char_traits_w_std_1_xz,
        "?_Wclog_func@std@@YAAEAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 234 (-arch=win32)
    (
        _wcout_func_std_yaaav_basic_ostream_gu_char_traits_g_std_1_xz,
        "?_Wcout_func@std@@YAAAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 235 (-arch=win64)
    (
        _wcout_func_std_yaaeav_basic_ostream_gu_char_traits_g_std_1_xz,
        "?_Wcout_func@std@@YAAEAV?$basic_ostream@GU?$char_traits@G@std@@@1@XZ",
    ),
    // row 236 (-arch=win32)
    (
        _wcout_func_std_yaaav_basic_ostream_wu_char_traits_w_std_1_xz,
        "?_Wcout_func@std@@YAAAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 237 (-arch=win64)
    (
        _wcout_func_std_yaaeav_basic_ostream_wu_char_traits_w_std_1_xz,
        "?_Wcout_func@std@@YAAEAV?$basic_ostream@_WU?$char_traits@_W@std@@@1@XZ",
    ),
    // row 238 (-arch=win32)
    (
        _wcrtomb_yahpadgpahpbu_cvtvec_z,
        "?_Wcrtomb@@YAHPADGPAHPBU_Cvtvec@@@Z",
    ),
    // row 239 (-arch=win64)
    (
        _wcrtomb_yahpeadgpeahpebu_cvtvec_z,
        "?_Wcrtomb@@YAHPEADGPEAHPEBU_Cvtvec@@@Z",
    ),
    // row 240 (-arch=win32)
    (
        _wcrtomb_yahpad_wpahpbu_cvtvec_z,
        "?_Wcrtomb@@YAHPAD_WPAHPBU_Cvtvec@@@Z",
    ),
    // row 241 (-arch=win64)
    (
        _wcrtomb_yahpead_wpeahpebu_cvtvec_z,
        "?_Wcrtomb@@YAHPEAD_WPEAHPEBU_Cvtvec@@@Z",
    ),
    // row 242 (-arch=win32)
    (
        _wcscoll_yahpbg000pbu_collvec_z,
        "?_Wcscoll@@YAHPBG000PBU_Collvec@@@Z",
    ),
    // row 243 (-arch=win64)
    (
        _wcscoll_yahpebg000pebu_collvec_z,
        "?_Wcscoll@@YAHPEBG000PEBU_Collvec@@@Z",
    ),
    // row 244 (-arch=win32)
    (
        _wcscoll_yahpb_w000pbu_collvec_z,
        "?_Wcscoll@@YAHPB_W000PBU_Collvec@@@Z",
    ),
    // row 245 (-arch=win64)
    (
        _wcscoll_yahpeb_w000pebu_collvec_z,
        "?_Wcscoll@@YAHPEB_W000PEBU_Collvec@@@Z",
    ),
    // row 246 (-arch=win32)
    (
        _wcsxfrm_yaipag0pbg1pbu_collvec_z,
        "?_Wcsxfrm@@YAIPAG0PBG1PBU_Collvec@@@Z",
    ),
    // row 247 (-arch=win64)
    (
        _wcsxfrm_ya_kpeag0pebg1pebu_collvec_z,
        "?_Wcsxfrm@@YA_KPEAG0PEBG1PEBU_Collvec@@@Z",
    ),
    // row 248 (-arch=win32)
    (
        _wcsxfrm_yaipa_w0pb_w1pbu_collvec_z,
        "?_Wcsxfrm@@YAIPA_W0PB_W1PBU_Collvec@@@Z",
    ),
    // row 249 (-arch=win64)
    (
        _wcsxfrm_ya_kpea_w0peb_w1pebu_collvec_z,
        "?_Wcsxfrm@@YA_KPEA_W0PEB_W1PEBU_Collvec@@@Z",
    ),
    // row 250 (-arch=win32)
    (
        _xfsopen_std_yapau_iobuf_pb_w0h_z,
        "?_Xfsopen@std@@YAPAU_iobuf@@PB_W0H@Z",
    ),
    // row 251 (-arch=win64)
    (
        _xfsopen_std_yapeau_iobuf_peb_w0h_z,
        "?_Xfsopen@std@@YAPEAU_iobuf@@PEB_W0H@Z",
    ),
    // row 252
    (
        _xinvarg_string_base_std_saxxz,
        "?_Xinvarg@_String_base@std@@SAXXZ",
    ),
    // row 253
    (
        _xlen_string_base_std_saxxz,
        "?_Xlen@_String_base@std@@SAXXZ",
    ),
    // row 254
    (
        _xran_string_base_std_saxxz,
        "?_Xran@_String_base@std@@SAXXZ",
    ),
    // row 255 (-arch=win32)
    (__stodx_yanpbdpapadjpah_z, "?__Stodx@@YANPBDPAPADJPAH@Z"),
    // row 256 (-arch=win64)
    (
        __stodx_yanpebdpeapeadjpeah_z,
        "?__Stodx@@YANPEBDPEAPEADJPEAH@Z",
    ),
    // row 257 (-arch=win32)
    (__stofx_yampbdpapadjpah_z, "?__Stofx@@YAMPBDPAPADJPAH@Z"),
    // row 258 (-arch=win64)
    (
        __stofx_yampebdpeapeadjpeah_z,
        "?__Stofx@@YAMPEBDPEAPEADJPEAH@Z",
    ),
    // row 259 (-arch=win32)
    (__stoldx_yaopbdpapadjpah_z, "?__Stoldx@@YAOPBDPAPADJPAH@Z"),
    // row 260 (-arch=win64)
    (
        __stoldx_yaopebdpeapeadjpeah_z,
        "?__Stoldx@@YAOPEBDPEAPEADJPEAH@Z",
    ),
    // row 261 (-arch=win32)
    (
        __wcrtomb_lk_yahpad_wpahpbu_cvtvec_z,
        "?__Wcrtomb_lk@@YAHPAD_WPAHPBU_Cvtvec@@@Z",
    ),
    // row 262 (-arch=win64)
    (
        __wcrtomb_lk_yahpead_wpeahpebu_cvtvec_z,
        "?__Wcrtomb_lk@@YAHPEAD_WPEAHPEBU_Cvtvec@@@Z",
    ),
    // row 263 (-arch=win32)
    (
        __get_default_appdomain_yajpapauiunknown_z,
        "?__get_default_appdomain@@YAJPAPAUIUnknown@@@Z",
    ),
    // row 264 (-arch=win64)
    (
        __get_default_appdomain_yajpeapeauiunknown_z,
        "?__get_default_appdomain@@YAJPEAPEAUIUnknown@@@Z",
    ),
    // row 265 (-arch=win32)
    (
        __query_new_handler_m_yap6mhi_zxz,
        "?__query_new_handler_m@@YAP6MHI@ZXZ",
    ),
    // row 266 (-arch=win64)
    (
        __query_new_handler_m_yap6mh_k_zxz,
        "?__query_new_handler_m@@YAP6MH_K@ZXZ",
    ),
    // row 267 (-arch=win32)
    (
        __release_appdomain_yaxpauiunknown_z,
        "?__release_appdomain@@YAXPAUIUnknown@@@Z",
    ),
    // row 268 (-arch=win64)
    (
        __release_appdomain_yaxpeauiunknown_z,
        "?__release_appdomain@@YAXPEAUIUnknown@@@Z",
    ),
    // row 269 (-arch=win32)
    (
        _beginthread_yaip6mxpax_zi0_z,
        "?_beginthread@@YAIP6MXPAX@ZI0@Z",
    ),
    // row 270 (-arch=win64)
    (
        _beginthread_ya_kp6mxpeax_zi0_z,
        "?_beginthread@@YA_KP6MXPEAX@ZI0@Z",
    ),
    // row 271 (-arch=win32)
    (
        _beginthreadex_yaipaxip6mi0_z0ipai_z,
        "?_beginthreadex@@YAIPAXIP6MI0@Z0IPAI@Z",
    ),
    // row 272 (-arch=win64)
    (
        _beginthreadex_ya_kpeaxip6mi0_z0ipeai_z,
        "?_beginthreadex@@YA_KPEAXIP6MI0@Z0IPEAI@Z",
    ),
    // row 273 (-arch=win32)
    (
        _fpieee_flt_yahkpau_exception_pointers_p6mhpau_fpieee_record_z_z,
        "?_fpieee_flt@@YAHKPAU_EXCEPTION_POINTERS@@P6MHPAU_FPIEEE_RECORD@@@Z@Z",
    ),
    // row 274 (-arch=win64)
    (
        _fpieee_flt_yahkpeau_exception_pointers_p6mhpeau_fpieee_record_z_z,
        "?_fpieee_flt@@YAHKPEAU_EXCEPTION_POINTERS@@P6MHPEAU_FPIEEE_RECORD@@@Z@Z",
    ),
    // row 275 (-arch=win32)
    (
        _set_invalid_parameter_handler_yap6axpb_w00ii_zh_z,
        "?_set_invalid_parameter_handler@@YAP6AXPB_W00II@ZH@Z",
    ),
    // row 276 (-arch=win64)
    (
        _set_invalid_parameter_handler_yap6axpeb_w00i_k_zh_z,
        "?_set_invalid_parameter_handler@@YAP6AXPEB_W00I_K@ZH@Z",
    ),
    // row 277 (-arch=win32)
    (
        _set_invalid_parameter_handler_yap6mxpb_w00ii_zp6mx000ii_z_z,
        "?_set_invalid_parameter_handler@@YAP6MXPB_W00II@ZP6MX000II@Z@Z",
    ),
    // row 278 (-arch=win64)
    (
        _set_invalid_parameter_handler_yap6mxpeb_w00i_k_zp6mx000i1_z_z,
        "?_set_invalid_parameter_handler@@YAP6MXPEB_W00I_K@ZP6MX000I1@Z@Z",
    ),
    // row 279 (-arch=win32)
    (
        _set_new_handler_yap6mhi_zp6mhi_z_z,
        "?_set_new_handler@@YAP6MHI@ZP6MHI@Z@Z",
    ),
    // row 280 (-arch=win64)
    (
        _set_new_handler_yap6mh_k_zp6mh0_z_z,
        "?_set_new_handler@@YAP6MH_K@ZP6MH0@Z@Z",
    ),
    // row 281
    (
        _set_purecall_handler_yap6axxzh_z,
        "?_set_purecall_handler@@YAP6AXXZH@Z",
    ),
    // row 282
    (
        _set_purecall_handler_yap6mxxzp6mxxz_z,
        "?_set_purecall_handler@@YAP6MXXZP6MXXZ@Z",
    ),
    // row 283
    (
        _uncaught_exception_m_std_ya_nxz,
        "?_uncaught_exception_m@std@@YA_NXZ",
    ),
    // row 284 (-arch=win32)
    (
        classic_locale_std_saabv12_xz,
        "?classic@locale@std@@SAABV12@XZ",
    ),
    // row 285 (-arch=win64)
    (
        classic_locale_std_saaebv12_xz,
        "?classic@locale@std@@SAAEBV12@XZ",
    ),
    // row 286
    (empty_locale_std_sa_av12_xz, "?empty@locale@std@@SA?AV12@XZ"),
    // row 287 (-arch=win32)
    (
        facet_register_facet_locale_std_caxpav123_z,
        "?facet_Register@facet@locale@std@@CAXPAV123@@Z",
    ),
    // row 288 (-arch=win64)
    (
        facet_register_facet_locale_std_caxpeav123_z,
        "?facet_Register@facet@locale@std@@CAXPEAV123@@Z",
    ),
    // row 289 (-arch=win32)
    (
        global_locale_std_sa_av12_abv12_z,
        "?global@locale@std@@SA?AV12@ABV12@@Z",
    ),
    // row 290 (-arch=win64)
    (
        global_locale_std_sa_av12_aebv12_z,
        "?global@locale@std@@SA?AV12@AEBV12@@Z",
    ),
    // row 291
    (
        resetiosflags_std_ya_au_smanip_h_1_h_z,
        "?resetiosflags@std@@YA?AU?$_Smanip@H@1@H@Z",
    ),
    // row 292
    (
        set_new_handler_std_yap6mxxzp6mxxz_z,
        "?set_new_handler@std@@YAP6MXXZP6MXXZ@Z",
    ),
    // row 293
    (
        set_terminate_yap6mxxzp6mxxz_z,
        "?set_terminate@@YAP6MXXZP6MXXZ@Z",
    ),
    // row 294
    (
        set_unexpected_yap6mxxzp6mxxz_z,
        "?set_unexpected@@YAP6MXXZP6MXXZ@Z",
    ),
    // row 295
    (
        setbase_std_ya_au_smanip_h_1_h_z,
        "?setbase@std@@YA?AU?$_Smanip@H@1@H@Z",
    ),
    // row 296
    (
        setiosflags_std_ya_au_smanip_h_1_h_z,
        "?setiosflags@std@@YA?AU?$_Smanip@H@1@H@Z",
    ),
    // row 297 (-arch=win32)
    (
        setprecision_std_ya_au_smanip_h_1_h_z,
        "?setprecision@std@@YA?AU?$_Smanip@H@1@H@Z",
    ),
    // row 298 (-arch=win64)
    (
        setprecision_std_ya_au_smanip_j_1_j_z,
        "?setprecision@std@@YA?AU?$_Smanip@_J@1@_J@Z",
    ),
    // row 299 (-arch=win32)
    (
        setw_std_ya_au_smanip_h_1_h_z,
        "?setw@std@@YA?AU?$_Smanip@H@1@H@Z",
    ),
    // row 300 (-arch=win64)
    (
        setw_std_ya_au_smanip_j_1_j_z,
        "?setw@std@@YA?AU?$_Smanip@_J@1@_J@Z",
    ),
    // row 301
    (signal_yap6mxh_zhh_z, "?signal@@YAP6MXH@ZHH@Z"),
    // row 302
    (signal_yap6mxh_zhp6mxh_z_z, "?signal@@YAP6MXH@ZHP6MXH@Z@Z"),
    // row 304
    (towctrans, "towctrans"),
    // row 305
    (wctrans, "wctrans"),
    // row 306
    (wctype, "wctype"),
];

#[test]
fn oracle_stub_rows() {
    for &(helper, function) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "msvcm80.dll",
                function,
            },
            "{function}"
        );
    }
}

/// The `@ cdecl` rows. Each C body only writes a `FIXME` trace, which the port does not model,
/// so the row checks that the helper returns.
#[test]
fn oracle_msvcm_main_rows() {
    // row 003: C trace "stub"
    crt_implementation_details_do_dll_language_support_validation();
    // row 004 (-arch=win32): C trace "0x1000: stub"
    crt_implementation_details_register_module_uninitializer(0x1000);
    // row 005 (-arch=win64): C trace "0x1000: stub"
    crt_implementation_details_register_module_uninitializer(0x1000);
    // row 006 (-arch=win32): C trace "0x1000: stub"
    crt_implementation_details_throw_module_load_exception(0x1000);
    // row 007 (-arch=win64): C trace "0x1000: stub"
    crt_implementation_details_throw_module_load_exception(0x1000);
    // row 008 (-arch=win32): C trace "0x1000 0x2000: stub"
    crt_implementation_details_throw_module_load_exception_inner(0x1000, 0x2000);
    // row 009 (-arch=win64): C trace "0x1000 0x2000: stub"
    crt_implementation_details_throw_module_load_exception_inner(0x1000, 0x2000);
    // row 010 (-arch=win32): C trace "0x1000 0x2000: stub"
    crt_implementation_details_throw_nested_module_load_exception(0x1000, 0x2000);
    // row 011 (-arch=win64): C trace "0x1000 0x2000: stub"
    crt_implementation_details_throw_nested_module_load_exception(0x1000, 0x2000);
    // row 303: C trace "0x1000: stub"
    setusermatherr_m(0x1000);
}
