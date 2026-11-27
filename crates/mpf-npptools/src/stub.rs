/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = npptools.dll` in `dlls/npptools/Makefile.in`.
pub const NPPTOOLS_DLL_NAME: &str = "npptools.dll";

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

/// Defines one helper per `@ stub` export, and the table of them in spec order.
macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/npptools/npptools.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(NPPTOOLS_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `npptools.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    clear_event_data => "ClearEventData",
    create_blob => "CreateBlob",
    create_npp_interface => "CreateNPPInterface",
    destroy_blob => "DestroyBlob",
    destroy_npp_blob_table => "DestroyNPPBlobTable",
    duplicate_blob => "DuplicateBlob",
    filter_npp_blob => "FilterNPPBlob",
    find_one_of => "FindOneOf",
    find_unknown_blob_categories => "FindUnknownBlobCategories",
    find_unknown_blob_tags => "FindUnknownBlobTags",
    get_bool_from_blob => "GetBoolFromBlob",
    get_class_id_from_blob => "GetClassIDFromBlob",
    get_dword_from_blob => "GetDwordFromBlob",
    get_mac_address_from_blob => "GetMacAddressFromBlob",
    get_npp_address_filter_from_blob => "GetNPPAddressFilterFromBlob",
    get_npp_blob_from_ui => "GetNPPBlobFromUI",
    get_npp_blob_table => "GetNPPBlobTable",
    get_npp_etype_sap_filter => "GetNPPEtypeSapFilter",
    get_npp_mac_type_as_number => "GetNPPMacTypeAsNumber",
    get_npp_pattern_filter_from_blob => "GetNPPPatternFilterFromBlob",
    get_npp_trigger_from_blob => "GetNPPTriggerFromBlob",
    get_network_info_from_blob => "GetNetworkInfoFromBlob",
    get_string_from_blob => "GetStringFromBlob",
    get_strings_from_blob => "GetStringsFromBlob",
    is_remote_npp => "IsRemoteNPP",
    lock_blob => "LockBlob",
    marshal_blob => "MarshalBlob",
    merge_blob => "MergeBlob",
    nm_add_used_entry => "NmAddUsedEntry",
    nm_heap_allocate => "NmHeapAllocate",
    nm_heap_free => "NmHeapFree",
    nm_heap_reallocate => "NmHeapReallocate",
    nm_heap_set_max_size => "NmHeapSetMaxSize",
    nm_heap_size => "NmHeapSize",
    nm_remove_used_entry => "NmRemoveUsedEntry",
    raise_nm_event => "RaiseNMEvent",
    read_blob_from_file => "ReadBlobFromFile",
    reg_create_blob_key => "RegCreateBlobKey",
    reg_open_blob_key => "RegOpenBlobKey",
    release_event_system => "ReleaseEventSystem",
    remove_from_blob => "RemoveFromBlob",
    select_npp_blob_from_table => "SelectNPPBlobFromTable",
    send_event => "SendEvent",
    set_bool_in_blob => "SetBoolInBlob",
    set_class_id_in_blob => "SetClassIDInBlob",
    set_dword_in_blob => "SetDwordInBlob",
    set_mac_address_in_blob => "SetMacAddressInBlob",
    set_npp_address_filter_in_blob => "SetNPPAddressFilterInBlob",
    set_npp_etype_sap_filter => "SetNPPEtypeSapFilter",
    set_npp_pattern_filter_in_blob => "SetNPPPatternFilterInBlob",
    set_npp_trigger_in_blob => "SetNPPTriggerInBlob",
    set_network_info_in_blob => "SetNetworkInfoInBlob",
    set_string_in_blob => "SetStringInBlob",
    subkey_exists => "SubkeyExists",
    un_marshal_blob => "UnMarshalBlob",
    unlock_blob => "UnlockBlob",
    write_blob_to_file => "WriteBlobToFile",
    write_cracked_blob_to_file => "WriteCrackedBlobToFile",
    recursive_delete_key => "recursiveDeleteKey",
    set_key_and_value => "setKeyAndValue",
}
