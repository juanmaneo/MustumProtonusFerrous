//! Port of the export table and the `@ stub` records in Wine's
//! `dlls/msvcp_win/msvcp_win.spec`, for the x86_64 build.
//!
//! `dlls/msvcp_win` has no C sources. Its exports are forwards to `msvcp140` and `@ stub`
//! entries, so the port covers the export table and the stub records.
//!
//! Ported:
//! - The x86_64 export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]. The 1203 `@ cdecl`
//!   and 121 `@ extern` entries forward to `msvcp140` under the same name.
//! - The 168 `@ stub` exports, each modelled as the record `__wine_spec_unimplemented_stub`
//!   raises: [`wine_spec_unimplemented_stub`], [`StubException`], [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`], and one helper per export. A helper is named after its export
//!   in snake_case: lowercase, runs of non-alphanumeric characters become `_`, the C++ `?` marks
//!   are dropped, a leading `_` is kept, and a name that would start with a digit (a mangled
//!   constructor such as `??0?$_Yarn@G@std@@QEAA@XZ`) gets a `stub_` prefix.
//!
//! Skipped:
//! - The 2268 entries that only the i386, ARM and 32-bit builds export (`-arch=i386`,
//!   `-arch=arm` and `-arch=win32`). The x86_64 build exports the entries with no `-arch` flag
//!   and those with `-arch=win64`.
//! - The argument lists in the spec, and the `-ret64` and `-norelay` flags. winebuild uses them
//!   for relay thunks of functions the DLL defines, and it emits no thunk for a forward.
//! - The forwarded bodies: `msvcp140` implements them.
//! - The `__cdecl` calling convention and the `for (;;)` loop around `RtlRaiseException`: the
//!   port returns the record instead.
//! - Ordinals: every export has a name.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    _7_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_6b, _7time_base_std_6b,
    _atexit_yaxp6axxz_z, _c_str_yarn_g_std_qebapebgxz, _chmod, _empty_yarn_g_std_qeba_nxz,
    _f_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaaxxz,
    _fput_num_put_dv_ostreambuf_iterator_du_char_traits_d_std_std_std_aeba_av_ostreambuf_iterator_du_char_traits_d_std_2_v32_aeavios_base_2_dpebd_k_z,
    _fput_num_put_gv_ostreambuf_iterator_gu_char_traits_g_std_std_std_aeba_av_ostreambuf_iterator_gu_char_traits_g_std_2_v32_aeavios_base_2_gpebd_k_z,
    _fput_num_put_wv_ostreambuf_iterator_wu_char_traits_w_std_std_std_aeba_av_ostreambuf_iterator_wu_char_traits_w_std_2_v32_aeavios_base_2_wpebd_k_z,
    _ftime_base_std_qeaaxxz,
    _getcat_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_sa_kpeapebvfacet_locale_2_pebv42_z,
    _getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_2_1aeavios_base_2_peah_z,
    _getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_2_1aeavios_base_2_peah_z,
    _getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_2_1aeavios_base_2_peah_z,
    _getfmt_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebd_z,
    _getint_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahaeav_istreambuf_iterator_du_char_traits_d_std_2_0hhaeahaebv_ctype_d_2_z,
    _getint_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahaeav_istreambuf_iterator_gu_char_traits_g_std_2_0hhaeahaebv_ctype_g_2_z,
    _getint_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahaeav_istreambuf_iterator_wu_char_traits_w_std_2_0hhaeahaebv_ctype_w_2_z,
    _hard_links, _init_cnt_ushinit_std_0ha,
    _init_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaaxaebv_locinfo_2_z,
    _lexp, _mp_add_std_yaxqea_k_k_z, _mp_get_std_ya_kqea_k_z, _mp_mul_std_yaxqea_k_k1_z,
    _mp_rem_std_yaxqea_k_k_z, _mtx_timedlock, _mtxdst, _mtxinit, _mtxlock, _mtxunlock,
    _reportunobservedexception_details_concurrency_yaxxz, _rng_abort_std_yaxpebd_z,
    _set_stl_sync_api_mode,
    _src_1_getffld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb,
    _src_1_getffld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb,
    _src_1_getffld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb,
    _src_1_getffldx_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1aeavios_base_3_peah_z_4qbdb,
    _src_1_getffldx_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1aeavios_base_3_peah_z_4qbdb,
    _src_1_getffldx_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1aeavios_base_3_peah_z_4qbdb,
    _src_1_getifld_num_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_aebahpeadaeav_istreambuf_iterator_du_char_traits_d_std_3_1haebvlocale_3_z_4qbdb,
    _src_1_getifld_num_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aebahpeadaeav_istreambuf_iterator_gu_char_traits_g_std_3_1haebvlocale_3_z_4qbdb,
    _src_1_getifld_num_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_aebahpeadaeav_istreambuf_iterator_wu_char_traits_w_std_3_1haebvlocale_3_z_4qbdb,
    _statvfs, _symlink_get, _thrd_abort, _thrd_exit, _throw_cpp_error_std_yaxh_z,
    _tidy_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_aeaaxxz,
    _tidy_yarn_g_std_aeaaxxz, _w_gettnames_locinfo_std_qeba_av_timevec_2_xz, _wcrtomb_lk, _wstod,
    _wstodx, _wstof, _wstofx, _wstold, _wstoldx, _xoverflow_error_std_yaxpebd_z,
    c_str_yarn_g_std_qebapebgxz, c_str_yarn_w_std_qebapeb_wxz,
    capturecallstack_platform_details_concurrency_ya_kpeapeax_k1_z,
    classic_table_ctype_d_std_sapebfxz,
    date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_aw4dateorder_time_base_2_xz,
    do_date_order_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_aw4dateorder_time_base_2_xz,
    do_get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    do_get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    do_get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z,
    do_get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    do_get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    do_get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    empty_yarn_g_std_qeba_nxz, empty_yarn_w_std_qeba_nxz,
    get_date_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    get_monthname_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    get_new_handler_std_yap6axxzxz,
    get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_dd_z,
    get_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_pebg4_z,
    get_time_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    get_weekday_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    get_year_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeba_av_istreambuf_iterator_gu_char_traits_g_std_2_v32_0aeavios_base_2_aeahpeautm_z,
    getvals_w_time_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_ieaax_waebv_locinfo_1_z,
    getvals_w_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaax_waebv_locinfo_1_z,
    getvals_w_time_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_ieaax_waebv_locinfo_1_z,
    id_messages_d_std_2v0locale_2_a, id_messages_g_std_2v0locale_2_a,
    id_messages_w_std_2v0locale_2_a,
    id_money_get_dv_istreambuf_iterator_du_char_traits_d_std_std_std_2v0locale_2_a,
    id_money_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_2v0locale_2_a,
    id_money_get_wv_istreambuf_iterator_wu_char_traits_w_std_std_std_2v0locale_2_a,
    id_money_put_dv_ostreambuf_iterator_du_char_traits_d_std_std_std_2v0locale_2_a,
    id_money_put_gv_ostreambuf_iterator_gu_char_traits_g_std_std_std_2v0locale_2_a,
    id_money_put_wv_ostreambuf_iterator_wu_char_traits_w_std_std_std_2v0locale_2_a,
    id_moneypunct_d_00_std_2v0locale_2_a, id_moneypunct_d_0a_std_2v0locale_2_a,
    id_moneypunct_g_00_std_2v0locale_2_a, id_moneypunct_g_0a_std_2v0locale_2_a,
    id_moneypunct_w_00_std_2v0locale_2_a, id_moneypunct_w_0a_std_2v0locale_2_a,
    id_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_2v0locale_2_a,
    intl_moneypunct_d_00_std_2_nb, intl_moneypunct_d_0a_std_2_nb, intl_moneypunct_g_00_std_2_nb,
    intl_moneypunct_g_0a_std_2_nb, intl_moneypunct_w_00_std_2_nb, intl_moneypunct_w_0a_std_2_nb,
    move_basic_ios_du_char_traits_d_std_std_qeaax_qeav12_z,
    move_basic_ios_du_char_traits_d_std_std_qeaaxaeav12_z,
    move_basic_ios_gu_char_traits_g_std_std_qeaax_qeav12_z,
    move_basic_ios_gu_char_traits_g_std_std_qeaaxaeav12_z,
    move_basic_ios_wu_char_traits_w_std_std_qeaax_qeav12_z,
    move_basic_ios_wu_char_traits_w_std_std_qeaaxaeav12_z,
    set_rdbuf_basic_ios_du_char_traits_d_std_std_qeaaxpeav_basic_streambuf_du_char_traits_d_std_2_z,
    set_rdbuf_basic_ios_gu_char_traits_g_std_std_qeaaxpeav_basic_streambuf_gu_char_traits_g_std_2_z,
    set_rdbuf_basic_ios_wu_char_traits_w_std_std_qeaaxpeav_basic_streambuf_wu_char_traits_w_std_2_z,
    stub_0_basic_iostream_du_char_traits_d_std_std_ieaa_qeav01_z,
    stub_0_basic_iostream_gu_char_traits_g_std_std_ieaa_qeav01_z,
    stub_0_basic_iostream_wu_char_traits_w_std_std_ieaa_qeav01_z,
    stub_0_basic_istream_du_char_traits_d_std_std_ieaa_qeav01_z,
    stub_0_basic_istream_gu_char_traits_g_std_std_ieaa_qeav01_z,
    stub_0_basic_istream_wu_char_traits_w_std_std_ieaa_qeav01_z,
    stub_0_basic_ostream_du_char_traits_d_std_std_ieaa_qeav01_z,
    stub_0_basic_ostream_gu_char_traits_g_std_std_ieaa_qeav01_z,
    stub_0_basic_ostream_wu_char_traits_w_std_std_ieaa_qeav01_z,
    stub_0_basic_streambuf_du_char_traits_d_std_std_ieaa_aebv01_z,
    stub_0_basic_streambuf_gu_char_traits_g_std_std_ieaa_aebv01_z,
    stub_0_basic_streambuf_wu_char_traits_w_std_std_ieaa_aebv01_z,
    stub_0_facet_base_std_qeaa_aebv01_z, stub_0_facet_base_std_qeaa_xz,
    stub_0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_ieaa_pebd_k_z,
    stub_0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_aebv_locinfo_1_k_z,
    stub_0_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_qeaa_k_z,
    stub_0_ushinit_std_qeaa_xz, stub_0_yarn_g_std_qeaa_aebv01_z, stub_0_yarn_g_std_qeaa_pebg_z,
    stub_0_yarn_g_std_qeaa_xz, stub_0_yarn_w_std_qeaa_aebv01_z, stub_0_yarn_w_std_qeaa_peb_w_z,
    stub_0time_base_std_qeaa_k_z, stub_1_facet_base_std_ueaa_xz,
    stub_1_time_get_gv_istreambuf_iterator_gu_char_traits_g_std_std_std_meaa_xz,
    stub_1_ushinit_std_qeaa_xz, stub_1_yarn_g_std_qeaa_xz, stub_1time_base_std_ueaa_xz,
    stub_4_basic_iostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_iostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_iostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_istream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_istream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_istream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_ostream_du_char_traits_d_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_ostream_gu_char_traits_g_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_ostream_wu_char_traits_w_std_std_ieaaaeav01_qeav01_z,
    stub_4_basic_streambuf_du_char_traits_d_std_std_ieaaaeav01_aebv01_z,
    stub_4_basic_streambuf_gu_char_traits_g_std_std_ieaaaeav01_aebv01_z,
    stub_4_basic_streambuf_wu_char_traits_w_std_std_ieaaaeav01_aebv01_z,
    stub_4_crt_new_delete_std_qeaaaeau01_aebu01_z, stub_4_crt_new_delete_std_qeaaaeau01_qeau01_z,
    stub_4_facet_base_std_qeaaaeav01_aebv01_z, stub_4_init_locks_std_qeaaaeav01_aebv01_z,
    stub_4_iosb_h_std_qeaaaeav01_aebv01_z, stub_4_iosb_h_std_qeaaaeav01_qeav01_z,
    stub_4_ushinit_std_qeaaaeav01_aebv01_z, stub_4_yarn_g_std_qeaaaeav01_aebv01_z,
    stub_4_yarn_g_std_qeaaaeav01_pebg_z, stub_4_yarn_w_std_qeaaaeav01_aebv01_z,
    stub_5_basic_istream_gu_char_traits_g_std_std_qeaaaeav01_aeag_z, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
