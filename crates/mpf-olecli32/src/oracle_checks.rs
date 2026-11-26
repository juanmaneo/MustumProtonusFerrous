//! Oracle-derived test vectors for the ported olecli32 logic.
//!
//! Vectors come from the verbatim Wine C bodies in `dlls/olecli32/olecli_main.c` (lines 37-192)
//! and `__wine_spec_unimplemented_stub` in `dlls/ntdll/exception.c` (lines 504-516). They were
//! built with `gcc -fsanitize=address,undefined -g` under `target/canon/olecli32/` and run once.
//! The harness printed 59 rows: 18 for the 11 explicit C functions and 41 for the 41 `@ stub`
//! exports, one row per export. Row 05 (`OleRegisterClientDoc` with the counter at `LONG_MAX`)
//! is excluded because UBSan flags the signed overflow, so 58 rows remain.

use super::*;
use std::cell::Cell;

/// A stand-in `OLEOBJECT` whose `QueryType` slot stores `value` and returns `status`, like the
/// harness's `fake_query_type`. It counts its calls, as the harness counts `fake_calls`.
struct FakeObject {
    status: OleStatus,
    value: i32,
    calls: Cell<u32>,
}

impl OleObject for FakeObject {
    fn query_type(&self, xlong: &mut i32) -> OleStatus {
        self.calls.set(self.calls.get() + 1);
        *xlong = self.value;
        self.status
    }
}

#[test]
fn oracle_client_doc_rows() {
    // row 01 OleSavedClientDoc(hDoc=7) -> 0
    assert_eq!(ole_saved_client_doc(), OLE_OK);
    // row 06 OleRenameClientDoc(hDoc=3) -> 0
    assert_eq!(ole_rename_client_doc(), OLE_OK);
    // row 07 OleRevokeClientDoc(hServerDoc=0) -> 0
    assert_eq!(ole_revoke_client_doc(), OLE_OK);
}

#[test]
fn oracle_register_client_doc_rows() {
    // row 02 OleRegisterClientDoc(counter_in=0) -> ret=0 hRet=1 counter_out=1
    let mut registry = ClientDocRegistry { last_handle: 0 };
    let mut hret = 0;
    assert_eq!(ole_register_client_doc(&mut registry, &mut hret), OLE_OK);
    assert_eq!((hret, registry.last_handle), (1, 1));

    // row 03 OleRegisterClientDoc(counter_in=-5) -> ret=0 hRet=-4 counter_out=-4
    let mut registry = ClientDocRegistry { last_handle: -5 };
    let mut hret = 0;
    assert_eq!(ole_register_client_doc(&mut registry, &mut hret), OLE_OK);
    assert_eq!((hret, registry.last_handle), (-4, -4));

    // row 04 OleRegisterClientDoc(counter_in=0x7ffffffe) -> ret=0 hRet=2147483647 counter_out=2147483647
    let mut registry = ClientDocRegistry {
        last_handle: 0x7fff_fffe,
    };
    let mut hret = 0;
    assert_eq!(ole_register_client_doc(&mut registry, &mut hret), OLE_OK);
    assert_eq!((hret, registry.last_handle), (0x7fff_ffff, 0x7fff_ffff));
}

#[test]
fn oracle_clip_rows() {
    // row 08 OleCreateLinkFromClip(render=1,clipformat=2) -> 0
    assert_eq!(ole_create_link_from_clip(), OLE_OK);
    // row 09 OleQueryLinkFromClip(render=1,clipformat=2) -> 0
    assert_eq!(ole_query_link_from_clip(), OLE_OK);
    // row 10 OleQueryCreateFromClip(render=1,clipformat=2) -> 0
    assert_eq!(ole_query_create_from_clip(), OLE_OK);
    // row 11 OleCreateFromClip(render=1,clipformat=2) -> 0
    assert_eq!(ole_create_from_clip(), OLE_OK);
}

#[test]
fn oracle_set_host_names_row() {
    // row 12 OleSetHostNames(oleob=NULL) -> 0
    assert_eq!(ole_set_host_names(), OLE_OK);
}

#[test]
fn oracle_is_dc_meta_rows() {
    // row 13 OleIsDcMeta(GetObjectType=0) -> 0
    assert!(!ole_is_dc_meta(0));
    // row 14 OleIsDcMeta(GetObjectType=4) -> 1
    assert!(ole_is_dc_meta(4));
    // row 15 OleIsDcMeta(GetObjectType=3) -> 0
    assert!(!ole_is_dc_meta(3));
}

#[test]
fn oracle_query_type_rows() {
    // row 16 OleQueryType(oleob=NULL) -> ret=16 xlong=23130 calls=0
    let mut xlong = 0x5a5a;
    assert_eq!(ole_query_type(None, &mut xlong), OLE_NULL_OBJECT_STATUS);
    assert_eq!(xlong, 0x5a5a);

    // row 17 OleQueryType(status_in=0,value_in=42) -> ret=0 xlong=42 calls=1
    let object = FakeObject {
        status: OLE_OK,
        value: 42,
        calls: Cell::new(0),
    };
    let mut xlong = 0x5a5a;
    assert_eq!(ole_query_type(Some(&object), &mut xlong), OLE_OK);
    assert_eq!((xlong, object.calls.get()), (42, 1));

    // row 18 OleQueryType(status_in=16,value_in=-7) -> ret=16 xlong=-7 calls=1
    let object = FakeObject {
        status: 16,
        value: -7,
        calls: Cell::new(0),
    };
    let mut xlong = 0x5a5a;
    assert_eq!(ole_query_type(Some(&object), &mut xlong), 16);
    assert_eq!((xlong, object.calls.get()), (-7, 1));
}

#[test]
fn oracle_stub_rows() {
    // rows 19-59: each `@ stub` export raises code=0x80000100 flags=0x01 nparams=2
    // module=olecli32.dll function=<export name>. The harness records RtlRaiseException and
    // longjmps out, because the verbatim C loop never returns.
    type StubFn = fn() -> StubException;
    let rows: [(StubFn, &'static str); 41] = [
        (wep, "WEP"),
        (ole_delete, "OleDelete"),
        (ole_clone, "OleClone"),
        (ole_copy_from_link, "OleCopyFromLink"),
        (ole_equal, "OleEqual"),
        (ole_copy_to_clipboard, "OleCopyToClipboard"),
        (ole_set_target_device, "OleSetTargetDevice"),
        (ole_set_bounds, "OleSetBounds"),
        (ole_query_bounds, "OleQueryBounds"),
        (ole_draw, "OleDraw"),
        (ole_query_open, "OleQueryOpen"),
        (ole_activate, "OleActivate"),
        (ole_update, "OleUpdate"),
        (ole_reconnect, "OleReconnect"),
        (ole_get_link_update_options, "OleGetLinkUpdateOptions"),
        (ole_set_link_update_options, "OleSetLinkUpdateOptions"),
        (ole_enum_formats, "OleEnumFormats"),
        (ole_close, "OleClose"),
        (ole_get_data, "OleGetData"),
        (ole_set_data, "OleSetData"),
        (ole_query_protocol, "OleQueryProtocol"),
        (ole_query_out_of_date, "OleQueryOutOfDate"),
        (ole_object_convert, "OleObjectConvert"),
        (ole_create_from_template, "OleCreateFromTemplate"),
        (ole_query_release_status, "OleQueryReleaseStatus"),
        (ole_query_release_error, "OleQueryReleaseError"),
        (ole_query_release_method, "OleQueryReleaseMethod"),
        (ole_create_link_from_file, "OleCreateLinkFromFile"),
        (ole_release, "OleRelease"),
        (ole_revert_client_doc, "OleRevertClientDoc"),
        (ole_rename, "OleRename"),
        (ole_enum_objects, "OleEnumObjects"),
        (ole_query_name, "OleQueryName"),
        (ole_set_color_scheme, "OleSetColorScheme"),
        (ole_request_data, "OleRequestData"),
        (ole_lock_server, "OleLockServer"),
        (ole_unlock_server, "OleUnlockServer"),
        (ole_query_size, "OleQuerySize"),
        (ole_execute, "OleExecute"),
        (ole_create_invisible, "OleCreateInvisible"),
        (ole_query_client_version, "OleQueryClientVersion"),
    ];
    for (stub, function) in rows {
        assert_eq!(
            stub(),
            StubException {
                code: 0x8000_0100,
                flags: 0x01,
                number_parameters: 2,
                module: "olecli32.dll",
                function,
            }
        );
    }
}
