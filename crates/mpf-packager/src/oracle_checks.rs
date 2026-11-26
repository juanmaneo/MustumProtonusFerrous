//! Oracle-derived test vectors for the ported packager logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/packager/packager_main.c`
//! (the `OleObject_*`, `PersistStorage_*` and `PackageCF_*` functions, `DllGetClassObject`, and the
//! `discard_string` helper) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/packager/`, and running the harness. It printed 67 rows, one per export plus
//! extra rows for the branches. ASan and UBSan reported nothing, so no rows were excluded. Each
//! test quotes the printed row above its assertions.

use super::*;

fn pkg(ref_count: i32) -> Package {
    Package {
        ref_count,
        ..Package::default()
    }
}

/// Runs `discard_string` over `bytes` the way the C harness drives `IStream_Read`: one byte per
/// read, `eof_hr` once the bytes run out, and a failed read at offset `err.0` that returns
/// `err.1`. Returns the result and the number of bytes consumed.
fn run_discard(bytes: &[u8], eof_hr: HRESULT, err: Option<(usize, HRESULT)>) -> (HRESULT, usize) {
    let mut pos = 0;
    let hr = discard_string(|chr: &mut u8| {
        if let Some((at, err_hr)) = err {
            if pos == at {
                return (err_hr, 0);
            }
        }
        match bytes.get(pos) {
            Some(&byte) => {
                *chr = byte;
                pos += 1;
                (S_OK, 1)
            }
            None => (eof_hr, 0),
        }
    });
    (hr, pos)
}

#[test]
fn oracle_hresult_values() {
    // hr= fields in the rows below: 0x80004001, 0x80004002, 0x80004005, 0x8007000e,
    // 0x80070057 and 0x80040111.
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_FAIL as u32, 0x8000_4005);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000e);
    assert_eq!(E_INVALIDARG as u32, 0x8007_0057);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
}

#[test]
fn oracle_ole_object_query_interface() {
    // OleObject_QueryInterface|IID_IUnknown|hr=0|obj=IOleObject|addref=1
    let mut p = pkg(1);
    assert_eq!(
        query_interface(&mut p, &IID_IUNKNOWN),
        Ok(OleInterface::IOleObject)
    );
    assert_eq!(p.ref_count, 2);
    // OleObject_QueryInterface|IID_IPersistStorage|hr=0|obj=IPersistStorage|addref=1
    let mut p = pkg(1);
    assert_eq!(
        query_interface(&mut p, &IID_IPERSIST_STORAGE),
        Ok(OleInterface::IPersistStorage)
    );
    assert_eq!(p.ref_count, 2);
    // OleObject_QueryInterface|IID_IClassFactory|hr=0x80004002|obj=NULL|addref=0
    let mut p = pkg(1);
    assert_eq!(
        query_interface(&mut p, &IID_ICLASS_FACTORY),
        Err(E_NOINTERFACE)
    );
    assert_eq!(p.ref_count, 1);
}

#[test]
fn oracle_ole_object_add_ref() {
    // OleObject_AddRef|ref=1|ret=2
    assert_eq!(add_ref(&mut pkg(1)), 2);
}

#[test]
fn oracle_ole_object_release() {
    // OleObject_Release|ref=2|ret=1|release_site=0|deletefile=0|heapfree=0
    assert_eq!(
        release(&mut pkg(2)),
        ReleaseOutcome {
            ref_count: 1,
            freed: false,
            client_site_released: false,
            file_deleted: false,
        }
    );
    // OleObject_Release|ref=1,site,file|ret=0|release_site=1|deletefile=1|heapfree=1
    let mut p = Package {
        ref_count: 1,
        client_site: true,
        has_filename: true,
    };
    assert_eq!(
        release(&mut p),
        ReleaseOutcome {
            ref_count: 0,
            freed: true,
            client_site_released: true,
            file_deleted: true,
        }
    );
    // OleObject_Release|ref=1,empty|ret=0|release_site=0|deletefile=0|heapfree=1
    assert_eq!(
        release(&mut pkg(1)),
        ReleaseOutcome {
            ref_count: 0,
            freed: true,
            client_site_released: false,
            file_deleted: false,
        }
    );
}

#[test]
fn oracle_ole_object_set_client_site() {
    // OleObject_SetClientSite|prev=NULL,new=site|hr=0|addref_site=1|release_site=0|clientsite_null=0
    let mut p = pkg(1);
    assert_eq!(
        set_client_site(&mut p, true),
        ClientSiteChange {
            hr: S_OK,
            released_previous: false,
            added_ref: true,
        }
    );
    assert!(p.client_site);
    // OleObject_SetClientSite|prev=site,new=NULL|hr=0|addref_site=0|release_site=1|clientsite_null=1
    let mut p = Package {
        ref_count: 1,
        client_site: true,
        has_filename: false,
    };
    assert_eq!(
        set_client_site(&mut p, false),
        ClientSiteChange {
            hr: S_OK,
            released_previous: true,
            added_ref: false,
        }
    );
    assert!(!p.client_site);
}

#[test]
fn oracle_ole_object_not_implemented_methods() {
    // OleObject_GetClientSite|hr=0x80004001
    assert_eq!(get_client_site(), E_NOTIMPL);
    // OleObject_SetHostNames|hr=0x80004001
    assert_eq!(set_host_names(), E_NOTIMPL);
    // OleObject_SetMoniker|hr=0x80004001
    assert_eq!(set_moniker(), E_NOTIMPL);
    // OleObject_GetMoniker|hr=0x80004001
    assert_eq!(get_moniker(), E_NOTIMPL);
    // OleObject_InitFromData|hr=0x80004001
    assert_eq!(init_from_data(), E_NOTIMPL);
    // OleObject_GetClipboardData|hr=0x80004001
    assert_eq!(get_clipboard_data(), E_NOTIMPL);
    // OleObject_EnumVerbs|hr=0x80004001
    assert_eq!(enum_verbs(), E_NOTIMPL);
    // OleObject_Update|hr=0x80004001
    assert_eq!(update(), E_NOTIMPL);
    // OleObject_IsUpToDate|hr=0x80004001
    assert_eq!(is_up_to_date(), E_NOTIMPL);
    // OleObject_GetUserClassID|hr=0x80004001
    assert_eq!(get_user_class_id(), E_NOTIMPL);
    // OleObject_GetUserType|hr=0x80004001
    assert_eq!(get_user_type(), E_NOTIMPL);
    // OleObject_SetExtent|hr=0x80004001
    assert_eq!(set_extent(), E_NOTIMPL);
    // OleObject_GetExtent|hr=0x80004001
    assert_eq!(get_extent(), E_NOTIMPL);
    // OleObject_Advise|hr=0x80004001
    assert_eq!(advise(), E_NOTIMPL);
    // OleObject_Unadvise|hr=0x80004001
    assert_eq!(unadvise(), E_NOTIMPL);
    // OleObject_EnumAdvise|hr=0x80004001
    assert_eq!(enum_advise(), E_NOTIMPL);
    // OleObject_SetColorScheme|hr=0x80004001
    assert_eq!(set_color_scheme(), E_NOTIMPL);
}

#[test]
fn oracle_ole_object_close() {
    // OleObject_Close|save=0|hr=0
    assert_eq!(close(0), S_OK);
    // OleObject_Close|save=1|hr=0
    assert_eq!(close(1), S_OK);
    // OleObject_Close|save=2|hr=0
    assert_eq!(close(2), S_OK);
}

#[test]
fn oracle_ole_object_do_verb() {
    // OleObject_DoVerb|verb=0|hr=0|shellexec=1
    assert_eq!(do_verb(0), S_OK);
    // OleObject_DoVerb|verb=1|hr=0x80070057|shellexec=0
    assert_eq!(do_verb(1), E_INVALIDARG);
    // OleObject_DoVerb|verb=-1|hr=0x80070057|shellexec=0
    assert_eq!(do_verb(-1), E_INVALIDARG);
}

#[test]
fn oracle_ole_object_get_misc_status() {
    // OleObject_GetMiscStatus|status=NULL|hr=0x80070057
    assert_eq!(get_misc_status(false), Err(E_INVALIDARG));
    // OleObject_GetMiscStatus|status=ptr|hr=0|value=0x2
    assert_eq!(get_misc_status(true), Ok(0x2));
    assert_eq!(OLEMISC_ONLYICONIC, 0x2);
}

#[test]
fn oracle_persist_storage_query_interface() {
    // PersistStorage_QueryInterface|IID_IPersistStorage|hr=0|obj=IPersistStorage
    assert_eq!(
        persist_storage_query_interface(&mut pkg(1), &IID_IPERSIST_STORAGE),
        Ok(OleInterface::IPersistStorage)
    );
    // PersistStorage_QueryInterface|IID_IClassFactory|hr=0x80004002|obj=NULL
    assert_eq!(
        persist_storage_query_interface(&mut pkg(1), &IID_ICLASS_FACTORY),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn oracle_persist_storage_refcount() {
    // PersistStorage_AddRef|ref=1|ret=2
    assert_eq!(persist_storage_add_ref(&mut pkg(1)), 2);
    // PersistStorage_Release|ref=2|ret=1
    assert_eq!(persist_storage_release(&mut pkg(2)).ref_count, 1);
}

#[test]
fn oracle_persist_storage_not_implemented_methods() {
    // PersistStorage_GetClassID|hr=0x80004001
    assert_eq!(get_class_id(), E_NOTIMPL);
    // PersistStorage_IsDirty|hr=0x80004001
    assert_eq!(is_dirty(), E_NOTIMPL);
    // PersistStorage_InitNew|hr=0x80004001
    assert_eq!(init_new(), E_NOTIMPL);
    // PersistStorage_Save|hr=0x80004001
    assert_eq!(save(), E_NOTIMPL);
    // PersistStorage_SaveCompleted|hr=0x80004001
    assert_eq!(save_completed(), E_NOTIMPL);
    // PersistStorage_HandsOffStorage|hr=0x80004001
    assert_eq!(hands_off_storage(), E_NOTIMPL);
}

#[test]
fn oracle_discard_string() {
    // discard_string|ab NUL, eof=E_FAIL|hr=0|consumed=3
    assert_eq!(run_discard(b"ab\0", E_FAIL, None), (S_OK, 3));
    // discard_string|a then eof S_OK|hr=0x80004005|consumed=1
    assert_eq!(run_discard(b"a", S_OK, None), (E_FAIL, 1));
    // discard_string|a then eof S_FALSE|hr=0x80004005|consumed=1
    assert_eq!(run_discard(b"a", S_FALSE, None), (E_FAIL, 1));
    // discard_string|empty, eof E_FAIL|hr=0x80004005|consumed=0
    assert_eq!(run_discard(b"", E_FAIL, None), (E_FAIL, 0));
    // discard_string|empty, eof S_OK|hr=0x80004005|consumed=0
    assert_eq!(run_discard(b"", S_OK, None), (E_FAIL, 0));
    // discard_string|empty, eof 2|hr=0x2|consumed=0
    assert_eq!(run_discard(b"", 2, None), (2, 0));
    // discard_string|NUL only|hr=0|consumed=1
    assert_eq!(run_discard(b"\0", S_OK, None), (S_OK, 1));
    // discard_string|xy, read error at 1|hr=0x80004005|consumed=1
    assert_eq!(run_discard(b"xy", S_OK, Some((1, E_FAIL))), (E_FAIL, 1));
    // discard_string|ab, read error S_FALSE at 0|hr=0x80004005|consumed=0
    assert_eq!(run_discard(b"ab", S_OK, Some((0, S_FALSE))), (E_FAIL, 0));
    // discard_string|abc NUL|hr=0|consumed=4
    assert_eq!(run_discard(b"abc\0", S_OK, None), (S_OK, 4));
}

#[test]
fn oracle_class_factory_query_interface() {
    // PackageCF_QueryInterface|IID_IUnknown|hr=0|obj=PackageCF|addref=1
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), S_OK);
    // PackageCF_QueryInterface|IID_IClassFactory|hr=0|obj=PackageCF
    assert_eq!(class_factory_query_interface(&IID_ICLASS_FACTORY), S_OK);
    // PackageCF_QueryInterface|IID_IOleObject|hr=0x80004002|obj=NULL
    assert_eq!(
        class_factory_query_interface(&IID_IOLE_OBJECT),
        E_NOINTERFACE
    );
}

#[test]
fn oracle_class_factory_refcount() {
    // PackageCF_AddRef|ret=2
    assert_eq!(class_factory_add_ref(), 2);
    // PackageCF_Release|ret=1
    assert_eq!(class_factory_release(), 1);
}

#[test]
fn oracle_class_factory_create_instance() {
    // PackageCF_CreateInstance|iid=IOleObject|hr=0|obj=IOleObject
    assert_eq!(
        create_instance(true, &IID_IOLE_OBJECT),
        Ok(OleInterface::IOleObject)
    );
    // PackageCF_CreateInstance|iid=IPersistStorage|hr=0|obj=IPersistStorage
    assert_eq!(
        create_instance(true, &IID_IPERSIST_STORAGE),
        Ok(OleInterface::IPersistStorage)
    );
    // PackageCF_CreateInstance|iid=IClassFactory|hr=0x80004002|obj=NULL
    assert_eq!(
        create_instance(true, &IID_ICLASS_FACTORY),
        Err(E_NOINTERFACE)
    );
    // PackageCF_CreateInstance|alloc=fail|hr=0x8007000e|obj=unchanged
    assert_eq!(create_instance(false, &IID_IOLE_OBJECT), Err(E_OUTOFMEMORY));
}

#[test]
fn oracle_class_factory_lock_server() {
    // PackageCF_LockServer|hr=0
    assert_eq!(lock_server(true), S_OK);
}

#[test]
fn oracle_dll_get_class_object() {
    // DllGetClassObject|CLSID_Package,IID_IClassFactory|hr=0|obj=PackageCF
    assert_eq!(
        dll_get_class_object(&CLSID_PACKAGE, &IID_ICLASS_FACTORY),
        S_OK
    );
    // DllGetClassObject|CLSID_Package,IID_IOleObject|hr=0x80004002|obj=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_PACKAGE, &IID_IOLE_OBJECT),
        E_NOINTERFACE
    );
    // DllGetClassObject|other CLSID|hr=0x80040111|obj=NULL
    assert_eq!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_ICLASS_FACTORY),
        CLASS_E_CLASSNOTAVAILABLE
    );
}
