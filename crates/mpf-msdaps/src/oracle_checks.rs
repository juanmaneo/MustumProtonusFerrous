//! Oracle-derived test vectors for the ported msdaps logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `db_type_size`
//! (`dlls/msdaps/row_server.c` lines 40-72), `CF_QueryInterface`, `CF_AddRef`, `CF_Release` and
//! `CF_LockServer` (`dlls/msdaps/main.c` lines 60-69, 71-95 and 114-118), and `DllGetClassObject`
//! (`dlls/msdaps/main.c` lines 137-166) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/msdaps/`. The 25 printed rows ran with no sanitizer report, so none is excluded.
//! Rows 16-22 compare the returned `HRESULT` and factory with the port's `Option` and `HRESULT`
//! results. The C-only side effects (the `addref` count and the `*obj` store) are not modelled.

use super::*;

/// `(db_type, var_len, size)` for rows 01-15, from `db_type_size(db_type, var_len)`.
const DB_TYPE_SIZE_ROWS: [(u16, usize, usize); 15] = [
    // row 01 DBTYPE_I1(16) var_len=99 -> 1
    (16, 99, 1),
    // row 02 DBTYPE_UI1(17) var_len=99 -> 1
    (17, 99, 1),
    // row 03 DBTYPE_I2(2) var_len=99 -> 2
    (2, 99, 2),
    // row 04 DBTYPE_UI2(18) var_len=99 -> 2
    (18, 99, 2),
    // row 05 DBTYPE_I4(3) var_len=99 -> 4
    (3, 99, 4),
    // row 06 DBTYPE_UI4(19) var_len=99 -> 4
    (19, 99, 4),
    // row 07 DBTYPE_I8(20) var_len=99 -> 8
    (20, 99, 8),
    // row 08 DBTYPE_UI8(21) var_len=99 -> 8
    (21, 99, 8),
    // row 09 DBTYPE_R8(5) var_len=99 -> 8
    (5, 99, 8),
    // row 10 DBTYPE_CY(6) var_len=99 -> 8
    (6, 99, 8),
    // row 11 DBTYPE_FILETIME(64) var_len=99 -> 8
    (64, 99, 8),
    // row 12 DBTYPE_BSTR(8) var_len=99 -> 8
    (8, 99, 8),
    // row 13 DBTYPE_GUID(72) var_len=99 -> 16
    (72, 99, 16),
    // row 14 DBTYPE_WSTR(130) var_len=37 -> 37
    (130, 37, 37),
    // row 15 default(0) var_len=5 -> 0
    (0, 5, 0),
];

#[test]
fn oracle_db_type_size_rows() {
    for &(db_type, var_len, size) in &DB_TYPE_SIZE_ROWS {
        assert_eq!(
            db_type_size(db_type, var_len),
            size,
            "db_type {db_type} var_len {var_len}"
        );
    }
}

#[test]
fn oracle_class_object_rows() {
    // row 16 DllGetClassObject(CLSID_wine_row_server) -> hr=0x00000000 obj=row_server_cf
    assert_eq!(
        class_object_for_clsid(&CLSID_WINE_ROW_SERVER),
        Some(ClassObject::RowServer)
    );
    // row 17 DllGetClassObject(CLSID_wine_row_proxy) -> hr=0x00000000 obj=row_proxy_cf
    assert_eq!(
        class_object_for_clsid(&CLSID_WINE_ROW_PROXY),
        Some(ClassObject::RowProxy)
    );
    // row 18 DllGetClassObject(CLSID_wine_rowset_server) -> hr=0x00000000 obj=rowset_server_cf
    assert_eq!(
        class_object_for_clsid(&CLSID_WINE_ROWSET_SERVER),
        Some(ClassObject::RowsetServer)
    );
    // row 19 DllGetClassObject(CLSID_wine_rowset_proxy) -> hr=0x00000000 obj=rowset_proxy_cf
    assert_eq!(
        class_object_for_clsid(&CLSID_WINE_ROWSET_PROXY),
        Some(ClassObject::RowsetProxy)
    );
}

#[test]
fn oracle_class_factory_query_interface_rows() {
    // row 20 CF_QueryInterface(IID_IUnknown) -> hr=0x00000000 addref=1 obj=iface
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), S_OK);
    // row 21 CF_QueryInterface(IID_IClassFactory) -> hr=0x00000000 addref=1 obj=iface
    assert_eq!(class_factory_query_interface(&IID_ICLASSFACTORY), S_OK);
    // row 22 CF_QueryInterface(CLSID_wine_row_server) -> hr=0x80004002 addref=0 obj=untouched
    assert_eq!(
        class_factory_query_interface(&CLSID_WINE_ROW_SERVER),
        E_NOINTERFACE
    );
}

#[test]
fn oracle_class_factory_constant_rows() {
    // row 23 CF_AddRef() -> 2
    assert_eq!(class_factory_add_ref(), 2);
    // row 24 CF_Release() -> 1
    assert_eq!(class_factory_release(), 1);
    // row 25 CF_LockServer(dolock=1) -> hr=0x00000000
    assert_eq!(class_factory_lock_server(true), S_OK);
}
