use crate::com::{
    Guid, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, HRESULT, IID_IOLE_OBJECT, IID_IPERSIST_STORAGE,
    IID_IUNKNOWN, S_OK,
};

/// `OLEMISC_ONLYICONIC` from `include/oleidl.idl`.
pub const OLEMISC_ONLYICONIC: u32 = 0x2;

/// The `struct Package` fields that the ported methods read or write. The `clientsite` pointer
/// and the `filename` buffer are modelled as presence flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Package {
    pub ref_count: i32,
    pub client_site: bool,
    pub has_filename: bool,
}

/// The interface that `OleObject_QueryInterface` hands back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OleInterface {
    IOleObject,
    IPersistStorage,
}

/// What `OleObject_Release` did to the package. The client-site release, the file deletion and
/// the `HeapFree` are reported as flags, and all three happen only when `freed` is set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReleaseOutcome {
    pub ref_count: i32,
    pub freed: bool,
    pub client_site_released: bool,
    pub file_deleted: bool,
}

/// What `OleObject_SetClientSite` released and acquired, and its return value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClientSiteChange {
    pub hr: HRESULT,
    pub released_previous: bool,
    pub added_ref: bool,
}

/// `OleObject_QueryInterface`. On success the C body calls `IUnknown_AddRef` on the returned
/// object, which is `OleObject_AddRef` on the same package, so the count goes up by one.
pub fn query_interface(pkg: &mut Package, riid: &Guid) -> Result<OleInterface, HRESULT> {
    let iface = if *riid == IID_IUNKNOWN || *riid == IID_IOLE_OBJECT {
        OleInterface::IOleObject
    } else if *riid == IID_IPERSIST_STORAGE {
        OleInterface::IPersistStorage
    } else {
        return Err(E_NOINTERFACE);
    };
    add_ref(pkg);
    Ok(iface)
}

/// `OleObject_AddRef`. The count wraps like `InterlockedIncrement`.
pub fn add_ref(pkg: &mut Package) -> i32 {
    pkg.ref_count = pkg.ref_count.wrapping_add(1);
    pkg.ref_count
}

/// `OleObject_Release`. The count wraps like `InterlockedDecrement`. When it reaches zero, the
/// client site is released and the file is deleted if one was set, then the object is freed.
pub fn release(pkg: &mut Package) -> ReleaseOutcome {
    pkg.ref_count = pkg.ref_count.wrapping_sub(1);
    let freed = pkg.ref_count == 0;
    ReleaseOutcome {
        ref_count: pkg.ref_count,
        freed,
        client_site_released: freed && pkg.client_site,
        file_deleted: freed && pkg.has_filename,
    }
}

/// `OleObject_SetClientSite`. The previous site is released and the new one is AddRef'd, as the
/// C body does.
pub fn set_client_site(pkg: &mut Package, new_site: bool) -> ClientSiteChange {
    let released_previous = pkg.client_site;
    pkg.client_site = new_site;
    ClientSiteChange {
        hr: S_OK,
        released_previous,
        added_ref: new_site,
    }
}

/// `OleObject_GetClientSite`, which is not implemented.
pub fn get_client_site() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_SetHostNames`, which is not implemented.
pub fn set_host_names() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_Close`. `OLECLOSE_SAVEIFDIRTY` and `OLECLOSE_PROMPTSAVE` only log a `WARN` in the
/// C body, so the port has nothing to record for any save option.
pub fn close(_save_option: u32) -> HRESULT {
    S_OK
}

/// `OleObject_SetMoniker`, which is not implemented.
pub fn set_moniker() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_GetMoniker`, which is not implemented.
pub fn get_moniker() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_InitFromData`, which is not implemented.
pub fn init_from_data() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_GetClipboardData`, which is not implemented.
pub fn get_clipboard_data() -> HRESULT {
    E_NOTIMPL
}

/// `do_activate_object`. The C body opens the file with `ShellExecuteW`, which the port does not
/// model, and returns `S_OK`.
fn do_activate_object() -> HRESULT {
    S_OK
}

/// `OleObject_DoVerb`. Verb 0 activates the object; every other verb is rejected.
pub fn do_verb(verb: i32) -> HRESULT {
    match verb {
        0 => do_activate_object(),
        _ => E_INVALIDARG,
    }
}

/// `OleObject_EnumVerbs`, which is not implemented.
pub fn enum_verbs() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_Update`, which is not implemented.
pub fn update() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_IsUpToDate`, which is not implemented.
pub fn is_up_to_date() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_GetUserClassID`, which is not implemented.
pub fn get_user_class_id() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_GetUserType`, which is not implemented.
pub fn get_user_type() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_SetExtent`, which is not implemented.
pub fn set_extent() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_GetExtent`, which is not implemented.
pub fn get_extent() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_Advise`, which is not implemented.
pub fn advise() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_Unadvise`, which is not implemented.
pub fn unadvise() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_EnumAdvise`, which is not implemented.
pub fn enum_advise() -> HRESULT {
    E_NOTIMPL
}

/// `OleObject_GetMiscStatus`. A missing status pointer is `E_INVALIDARG`; otherwise the status
/// is always `OLEMISC_ONLYICONIC`, whatever the aspect.
pub fn get_misc_status(has_status_out: bool) -> Result<u32, HRESULT> {
    if !has_status_out {
        return Err(E_INVALIDARG);
    }
    Ok(OLEMISC_ONLYICONIC)
}

/// `OleObject_SetColorScheme`, which is not implemented.
pub fn set_color_scheme() -> HRESULT {
    E_NOTIMPL
}
