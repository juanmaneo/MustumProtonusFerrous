//! Oracle-derived test vectors for the ported qdvd logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/qdvd/` (`qdvd_main.c`,
//! `graph.c`, `navigator.c`, and the `winecrt0` forwarders) under `target/canon/qdvd/` with
//! `gcc -fsanitize=address,undefined -g`, and running the harness. The harness prints 125 rows:
//! one per `E_NOTIMPL` export, one per forwarder, and one per exercised path. ASan and UBSan
//! reported nothing, so no row was excluded. Each check quotes its row above it.

use super::*;

/// One `stub|<label>|<hr>` row: the label, the Rust export, and the expected `HRESULT`.
type StubRow = (&'static str, fn() -> Hresult, u32);

/// The `E_NOTIMPL` exports, from the `stub|<label>|<hr>` rows. `control2.` and `info2.` are the
/// `IDvdControl2` and `IDvdInfo2` methods, and `graph.` the `IDvdGraphBuilder` methods.
#[rustfmt::skip]
const STUB_ROWS: [StubRow; 82] = [
    ("control2.PlayTitle", control2::play_title, 0x8000_4001),
    ("control2.PlayChapterInTitle", control2::play_chapter_in_title, 0x8000_4001),
    ("control2.PlayTimeInTitle", control2::play_time_in_title, 0x8000_4001),
    ("control2.Stop", control2::stop, 0x8000_4001),
    ("control2.ReturnFromSubmenu", control2::return_from_submenu, 0x8000_4001),
    ("control2.PlayAtTime", control2::play_at_time, 0x8000_4001),
    ("control2.PlayChapter", control2::play_chapter, 0x8000_4001),
    ("control2.PlayPrevChapter", control2::play_prev_chapter, 0x8000_4001),
    ("control2.ReplayChapter", control2::replay_chapter, 0x8000_4001),
    ("control2.PlayNextChapter", control2::play_next_chapter, 0x8000_4001),
    ("control2.PlayForwards", control2::play_forwards, 0x8000_4001),
    ("control2.PlayBackwards", control2::play_backwards, 0x8000_4001),
    ("control2.ShowMenu", control2::show_menu, 0x8000_4001),
    ("control2.Resume", control2::resume, 0x8000_4001),
    ("control2.SelectRelativeButton", control2::select_relative_button, 0x8000_4001),
    ("control2.ActivateButton", control2::activate_button, 0x8000_4001),
    ("control2.SelectButton", control2::select_button, 0x8000_4001),
    ("control2.SelectAndActivateButton", control2::select_and_activate_button, 0x8000_4001),
    ("control2.StillOff", control2::still_off, 0x8000_4001),
    ("control2.Pause", control2::pause, 0x8000_4001),
    ("control2.SelectAudioStream", control2::select_audio_stream, 0x8000_4001),
    ("control2.SelectSubpictureStream", control2::select_subpicture_stream, 0x8000_4001),
    ("control2.SetSubpictureState", control2::set_subpicture_state, 0x8000_4001),
    ("control2.SelectAngle", control2::select_angle, 0x8000_4001),
    ("control2.SelectParentalLevel", control2::select_parental_level, 0x8000_4001),
    ("control2.SelectParentalCountry", control2::select_parental_country, 0x8000_4001),
    ("control2.SelectKaraokeAudioPresentationMode", control2::select_karaoke_audio_presentation_mode, 0x8000_4001),
    ("control2.SelectVideoModePreference", control2::select_video_mode_preference, 0x8000_4001),
    ("control2.SetDVDDirectory", control2::set_dvd_directory, 0x8000_4001),
    ("control2.ActivateAtPosition", control2::activate_at_position, 0x8000_4001),
    ("control2.SelectAtPosition", control2::select_at_position, 0x8000_4001),
    ("control2.PlayChaptersAutoStop", control2::play_chapters_auto_stop, 0x8000_4001),
    ("control2.AcceptParentalLevelChange", control2::accept_parental_level_change, 0x8000_4001),
    ("control2.SetOption", control2::set_option, 0x8000_4001),
    ("control2.SetState", control2::set_state, 0x8000_4001),
    ("control2.PlayPeriodInTitleAutoStop", control2::play_period_in_title_auto_stop, 0x8000_4001),
    ("control2.SetGRPM", control2::set_grpm, 0x8000_4001),
    ("control2.SelectDefaultMenuLanguage", control2::select_default_menu_language, 0x8000_4001),
    ("control2.SelectDefaultAudioLanguage", control2::select_default_audio_language, 0x8000_4001),
    ("control2.SelectDefaultSubpictureLanguage", control2::select_default_subpicture_language, 0x8000_4001),
    ("info2.GetCurrentDomain", info2::get_current_domain, 0x8000_4001),
    ("info2.GetCurrentLocation", info2::get_current_location, 0x8000_4001),
    ("info2.GetTotalTitleTime", info2::get_total_title_time, 0x8000_4001),
    ("info2.GetCurrentButton", info2::get_current_button, 0x8000_4001),
    ("info2.GetCurrentAngle", info2::get_current_angle, 0x8000_4001),
    ("info2.GetCurrentAudio", info2::get_current_audio, 0x8000_4001),
    ("info2.GetCurrentSubpicture", info2::get_current_subpicture, 0x8000_4001),
    ("info2.GetCurrentUOPS", info2::get_current_uops, 0x8000_4001),
    ("info2.GetAllSPRMs", info2::get_all_sprms, 0x8000_4001),
    ("info2.GetAllGPRMs", info2::get_all_gprms, 0x8000_4001),
    ("info2.GetAudioLanguage", info2::get_audio_language, 0x8000_4001),
    ("info2.GetSubpictureLanguage", info2::get_subpicture_language, 0x8000_4001),
    ("info2.GetTitleAttributes", info2::get_title_attributes, 0x8000_4001),
    ("info2.GetVMGAttributes", info2::get_vmg_attributes, 0x8000_4001),
    ("info2.GetVideoAttributes", info2::get_video_attributes, 0x8000_4001),
    ("info2.GetAudioAttributes", info2::get_audio_attributes, 0x8000_4001),
    ("info2.GetKaraokeAttributes", info2::get_karaoke_attributes, 0x8000_4001),
    ("info2.GetSubpictureAttributes", info2::get_subpicture_attributes, 0x8000_4001),
    ("info2.GetCurrentVolumeInfo", info2::get_current_volume_info, 0x8000_4001),
    ("info2.GetDVDTextNumberOfLanguages", info2::get_dvd_text_number_of_languages, 0x8000_4001),
    ("info2.GetDVDTextLanguageInfo", info2::get_dvd_text_language_info, 0x8000_4001),
    ("info2.GetDVDTextStringAsNative", info2::get_dvd_text_string_as_native, 0x8000_4001),
    ("info2.GetDVDTextStringAsUnicode", info2::get_dvd_text_string_as_unicode, 0x8000_4001),
    ("info2.GetPlayerParentalLevel", info2::get_player_parental_level, 0x8000_4001),
    ("info2.GetNumberOfChapters", info2::get_number_of_chapters, 0x8000_4001),
    ("info2.GetTitleParentalLevels", info2::get_title_parental_levels, 0x8000_4001),
    ("info2.GetDVDDirectory", info2::get_dvd_directory, 0x8000_4001),
    ("info2.IsAudioStreamEnabled", info2::is_audio_stream_enabled, 0x8000_4001),
    ("info2.GetDiscID", info2::get_disc_id, 0x8000_4001),
    ("info2.GetState", info2::get_state, 0x8000_4001),
    ("info2.GetMenuLanguages", info2::get_menu_languages, 0x8000_4001),
    ("info2.GetButtonAtPosition", info2::get_button_at_position, 0x8000_4001),
    ("info2.GetCmdFromEvent", info2::get_cmd_from_event, 0x8000_4001),
    ("info2.GetDefaultMenuLanguage", info2::get_default_menu_language, 0x8000_4001),
    ("info2.GetDefaultAudioLanguage", info2::get_default_audio_language, 0x8000_4001),
    ("info2.SelectDefaultSubpictureLanguage", info2::select_default_subpicture_language, 0x8000_4001),
    ("info2.GetDecoderCaps", info2::get_decoder_caps, 0x8000_4001),
    ("info2.GetButtonRect", info2::get_button_rect, 0x8000_4001),
    ("info2.IsSubpictureStreamEnabled", info2::is_subpicture_stream_enabled, 0x8000_4001),
    ("graph.GetFiltergraph", graph::get_filtergraph, 0x8000_4001),
    ("graph.GetDvdInterface", graph::get_dvd_interface, 0x8000_4001),
    ("graph.RenderDvdVideoVolume", graph::render_dvd_video_volume, 0x8000_4001),
];

/// `create_instance` on the graph builder factory: the gate, then `graph_builder_create`, then
/// the `QueryInterface` for `iid` on the new object, which the C code releases afterwards. Returns
/// the number of objects created and the interface or failure the C code reports.
fn create_instance(
    has_outer: bool,
    iid: &Guid,
) -> (u32, Result<graph::GraphBuilderInterface, Hresult>) {
    if let Some(hr) = class_factory_create_instance_gate(has_outer, iid) {
        return (0, Err(hr));
    }
    (1, graph::GraphBuilder::create().inner_query_interface(iid))
}

#[test]
fn oracle_stub_rows() {
    for (label, export, expected) in STUB_ROWS {
        assert_eq!(export() as u32, expected, "{label}");
    }
}

#[test]
fn oracle_dll_rows() {
    // dll|CanUnloadNow|0x00000001
    assert_eq!(dll_can_unload_now() as u32, 0x0000_0001);
    // dll|DllRegisterServer|reg=0x00000000|0x00000000
    assert_eq!(dll_register_server(|| S_OK) as u32, 0x0000_0000);
    // dll|DllRegisterServer|reg=0x80004005|0x80004005
    assert_eq!(dll_register_server(|| E_FAIL) as u32, 0x8000_4005);
    // dll|DllUnregisterServer|unreg=0x00000000|0x00000000
    assert_eq!(dll_unregister_server(|| S_OK) as u32, 0x0000_0000);
    // dll|DllUnregisterServer|unreg=0x80004005|0x80004005
    assert_eq!(dll_unregister_server(|| E_FAIL) as u32, 0x8000_4005);
}

#[test]
fn oracle_get_class_object_rows() {
    // getclassobject|GB|IUnknown|0x00000000|GB_FACTORY
    assert!(
        dll_get_class_object(&CLSID_DVDGRAPHBUILDER, &IID_IUNKNOWN)
            == Ok(ClassFactory::GraphBuilder)
    );
    // getclassobject|GB|IClassFactory|0x00000000|GB_FACTORY
    assert!(
        dll_get_class_object(&CLSID_DVDGRAPHBUILDER, &IID_ICLASSFACTORY)
            == Ok(ClassFactory::GraphBuilder)
    );
    // getclassobject|GB|IDvdControl2|0x80004002|NULL
    assert!(dll_get_class_object(&CLSID_DVDGRAPHBUILDER, &IID_IDVDCONTROL2) == Err(E_NOINTERFACE));
    // getclassobject|NAV|IUnknown|0x00000000|NAV_FACTORY
    assert!(
        dll_get_class_object(&CLSID_DVDNAVIGATOR, &IID_IUNKNOWN) == Ok(ClassFactory::Navigator)
    );
    // getclassobject|NAV|IClassFactory|0x00000000|NAV_FACTORY
    assert!(
        dll_get_class_object(&CLSID_DVDNAVIGATOR, &IID_ICLASSFACTORY)
            == Ok(ClassFactory::Navigator)
    );
    // getclassobject|NAV|IDvdControl2|0x80004002|NULL
    assert!(dll_get_class_object(&CLSID_DVDNAVIGATOR, &IID_IDVDCONTROL2) == Err(E_NOINTERFACE));
    // getclassobject|UNKNOWN|IUnknown|0x80040111|UNTOUCHED
    // The harness passes &IID_IUnknown as the CLSID here, which no class is registered under.
    assert!(dll_get_class_object(&IID_IUNKNOWN, &IID_IUNKNOWN) == Err(CLASS_E_CLASSNOTAVAILABLE));
    // getclassobject|UNKNOWN|IClassFactory|0x80040111|UNTOUCHED
    assert!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_ICLASSFACTORY) == Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    // getclassobject|UNKNOWN|IDvdControl2|0x80040111|UNTOUCHED
    assert!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_IDVDCONTROL2) == Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_class_factory_rows() {
    // factory|AddRef|2
    assert_eq!(class_factory_add_ref(), 2);
    // factory|Release|1
    assert_eq!(class_factory_release(), 1);
    // factory|LockServer|1|0x00000000
    assert_eq!(class_factory_lock_server(true) as u32, 0x0000_0000);
    // factory|LockServer|0|0x00000000
    assert_eq!(class_factory_lock_server(false) as u32, 0x0000_0000);
}

#[test]
fn oracle_create_instance_rows() {
    // createinstance|NULL|IUnknown|0x00000000|created=1|INNER
    assert!(create_instance(false, &IID_IUNKNOWN) == (1, Ok(graph::GraphBuilderInterface::Inner)));
    // createinstance|NULL|IDvdGraphBuilder|0x00000000|created=1|DVD
    assert!(
        create_instance(false, &IID_IDVDGRAPHBUILDER)
            == (1, Ok(graph::GraphBuilderInterface::DvdGraphBuilder))
    );
    // createinstance|NULL|IDvdControl2|0x80004002|created=1|NULL
    assert!(create_instance(false, &IID_IDVDCONTROL2) == (1, Err(E_NOINTERFACE)));
    // createinstance|SET|IUnknown|0x00000000|created=1|INNER
    assert!(create_instance(true, &IID_IUNKNOWN) == (1, Ok(graph::GraphBuilderInterface::Inner)));
    // createinstance|SET|IDvdGraphBuilder|0x80004002|created=0|NULL
    assert!(create_instance(true, &IID_IDVDGRAPHBUILDER) == (0, Err(E_NOINTERFACE)));
}

#[test]
fn oracle_graph_builder_rows() {
    use graph::{GraphBuilder, GraphBuilderInterface};

    // graph|create|-|1
    let mut builder = GraphBuilder::create();
    assert_eq!(builder.refcount(), 1);
    // graph|inner_addref|-|2
    assert_eq!(builder.inner_add_ref(), 2);
    // graph|inner_release|-|1
    assert_eq!(builder.inner_release(), 1);
    // graph|inner_qi_iunknown|0x00000000|2
    assert!(builder.inner_query_interface(&IID_IUNKNOWN) == Ok(GraphBuilderInterface::Inner));
    assert_eq!(builder.refcount(), 2);
    // graph|inner_qi_dvd|0x00000000|3
    assert!(
        builder.inner_query_interface(&IID_IDVDGRAPHBUILDER)
            == Ok(GraphBuilderInterface::DvdGraphBuilder)
    );
    assert_eq!(builder.refcount(), 3);
    // graph|inner_qi_control2|0x80004002|3
    assert!(builder.inner_query_interface(&IID_IDVDCONTROL2) == Err(E_NOINTERFACE));
    assert_eq!(builder.refcount(), 3);
    // graph|dvd_addref|-|4
    assert_eq!(builder.dvd_add_ref(), 4);
    // graph|dvd_release|-|3
    assert_eq!(builder.dvd_release(), 3);
    // graph|dvd_qi_iunknown|0x00000000|4
    assert!(builder.dvd_query_interface(&IID_IUNKNOWN) == Ok(GraphBuilderInterface::Inner));
    assert_eq!(builder.refcount(), 4);
    // graph|inner_release|-|3
    assert_eq!(builder.inner_release(), 3);
    // graph|inner_release|-|2
    assert_eq!(builder.inner_release(), 2);
    // graph|inner_release|-|1
    assert_eq!(builder.inner_release(), 1);
    // graph|inner_release|-|0
    assert_eq!(builder.inner_release(), 0);
}

#[test]
fn oracle_navigator_rows() {
    // navigator|query_interface|IDvdControl2|0x00000000|CONTROL2
    assert!(navigator_query_interface(&IID_IDVDCONTROL2) == Ok(NavigatorInterface::DvdControl2));
    // navigator|query_interface|IDvdInfo2|0x00000000|INFO2
    assert!(navigator_query_interface(&IID_IDVDINFO2) == Ok(NavigatorInterface::DvdInfo2));
    // navigator|query_interface|IUnknown|0x80004002|UNTOUCHED
    assert!(navigator_query_interface(&IID_IUNKNOWN) == Err(E_NOINTERFACE));
    // navigator|query_interface|IDvdGraphBuilder|0x80004002|UNTOUCHED
    assert!(navigator_query_interface(&IID_IDVDGRAPHBUILDER) == Err(E_NOINTERFACE));
    // navigator|get_pin|0|NULL
    assert!(navigator_get_pin(0).is_none());
    // navigator|get_pin|1|NULL
    assert!(navigator_get_pin(1).is_none());
    // navigator|get_pin|4294967295|NULL
    assert!(navigator_get_pin(u32::MAX).is_none());
}
