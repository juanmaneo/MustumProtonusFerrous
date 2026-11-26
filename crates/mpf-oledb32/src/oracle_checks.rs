//! Oracle-derived test vectors for the ported oledb32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `main.c`, `convert.c` and
//! `datainit.c`, plus winecrt0's `DllCanUnloadNow`, with
//! `gcc -std=gnu11 -fsanitize=address,undefined -g` under `target/canon/oledb32/`, and running
//! the harness. The harness prints 25 rows (R01 to R25). ASan and UBSan flagged no row, so none
//! is excluded.

use crate::class_object::{
    FactoryKind, Guid, ObjOut, CLASS_E_CLASSNOTAVAILABLE, CLSID_DATALINKS, CLSID_EXTENDEDERRORINFO,
    CLSID_MSDAINITIALIZE, CLSID_OLEDB_CONVERSIONLIBRARY, CLSID_OLEDB_ROWPOSITIONLIBRARY,
    E_NOINTERFACE, IID_ICLASSFACTORY, IID_IUNKNOWN, S_FALSE, S_OK,
};
use crate::dbtype::{
    DBTYPE_BSTR, DBTYPE_DBDATE, DBTYPE_DBTIMESTAMP, DBTYPE_I1, DBTYPE_I8, DBTYPE_NUMERIC,
    DBTYPE_STR, DBTYPE_VARIANT,
};
use crate::{
    array_type, cf_add_ref, cf_lock_server, cf_release, common_class, dbtype_class, dbtype_length,
    dll_can_unload_now, dll_get_class_object, dll_main, parse_init_string,
};

/// `GUID_NULL`, from the C harness.
const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// The "other" IID the harness passes, `0c733a8b-2a1c-11ce-ade5-00aa0044773d`.
const IID_OTHER: Guid = Guid {
    data1: 0x0c73_3a8b,
    data2: 0x2a1c,
    data3: 0x11ce,
    data4: [0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d],
};

fn parse(init: &str) -> Vec<(String, String)> {
    let wide: Vec<u16> = init.encode_utf16().collect();
    parse_init_string(&wide)
        .into_iter()
        .map(|p| {
            (
                String::from_utf16(&p.name).unwrap(),
                String::from_utf16(&p.value).unwrap(),
            )
        })
        .collect()
}

fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
    list.iter()
        .map(|&(n, v)| (n.to_string(), v.to_string()))
        .collect()
}

#[test]
fn oracle_main_rows() {
    // R01 DllCanUnloadNow hr=0x00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);
    // R02 DllMain(attach) = 1
    assert!(dll_main(1));
    // R03 CF AddRef = 2
    assert_eq!(cf_add_ref(), 2);
    // R04 CF Release = 1
    assert_eq!(cf_release(), 1);
    // R05 CF LockServer(TRUE) hr=0x00000000
    assert_eq!(cf_lock_server(true), S_OK);
    // R06 GCO(GUID_NULL,IUnknown) hr=0x80040111 obj=NULL
    assert_eq!(
        dll_get_class_object(&GUID_NULL, &IID_IUNKNOWN),
        (CLASS_E_CLASSNOTAVAILABLE, ObjOut::Null)
    );
    // R07 GCO(CONVERSIONLIBRARY,IUnknown) hr=0x00000000 obj=convert
    assert_eq!(
        dll_get_class_object(&CLSID_OLEDB_CONVERSIONLIBRARY, &IID_IUNKNOWN),
        (S_OK, ObjOut::Factory(FactoryKind::Convert))
    );
    // R08 GCO(DataLinks,IClassFactory) hr=0x00000000 obj=dslocator
    assert_eq!(
        dll_get_class_object(&CLSID_DATALINKS, &IID_ICLASSFACTORY),
        (S_OK, ObjOut::Factory(FactoryKind::DataLinks))
    );
    // R09 GCO(MSDAINITIALIZE,other) hr=0x80004002 obj=untouched
    assert_eq!(
        dll_get_class_object(&CLSID_MSDAINITIALIZE, &IID_OTHER),
        (E_NOINTERFACE, ObjOut::Untouched)
    );
    // R10 GCO(EXTENDEDERRORINFO,IUnknown) hr=0x00000000 obj=errorinfo
    assert_eq!(
        dll_get_class_object(&CLSID_EXTENDEDERRORINFO, &IID_IUNKNOWN),
        (S_OK, ObjOut::Factory(FactoryKind::ErrorInfo))
    );
    // R11 GCO(ROWPOSITIONLIBRARY,IUnknown) hr=0x00000000 obj=rowpos
    assert_eq!(
        dll_get_class_object(&CLSID_OLEDB_ROWPOSITIONLIBRARY, &IID_IUNKNOWN),
        (S_OK, ObjOut::Factory(FactoryKind::RowPos))
    );
}

#[test]
fn oracle_convert_rows() {
    // R12 get_length(16) = 1
    assert_eq!(dbtype_length(DBTYPE_I1), 1);
    // R13 get_length(133) = 6
    assert_eq!(dbtype_length(DBTYPE_DBDATE), 6);
    // R14 get_length(12) = 24
    assert_eq!(dbtype_length(DBTYPE_VARIANT), 24);
    // R15 get_length(8) = 8
    assert_eq!(dbtype_length(DBTYPE_BSTR), 8);
    // R16 get_dbtype_class(135) = 133
    assert_eq!(dbtype_class(DBTYPE_DBTIMESTAMP), 133);
    // R17 get_dbtype_class(129) = 8
    assert_eq!(dbtype_class(DBTYPE_STR), 8);
    // R18 common_class(131) = 1
    assert!(common_class(DBTYPE_NUMERIC));
    // R19 array_type(20) = 0
    assert!(!array_type(DBTYPE_I8));
}

#[test]
fn oracle_parse_rows() {
    // R20 PARSE in="PROVIDER=X;Data Source=s" n=1 out=[Data Source]=[s]
    assert_eq!(
        parse("PROVIDER=X;Data Source=s"),
        pairs(&[("Data Source", "s")])
    );
    // R21 PARSE in="  a = 1 ; b="x;y";c='q'" n=3 out=[a ]=[ 1 ]|[b]=[x;y]|[c]=[q]
    assert_eq!(
        parse("  a = 1 ; b=\"x;y\";c='q'"),
        pairs(&[("a ", " 1 "), ("b", "x;y"), ("c", "q")])
    );
    // R22 PARSE in="a="unterminated;b=2" n=1 out=[a]=[unterminated;b=2]
    assert_eq!(
        parse("a=\"unterminated;b=2"),
        pairs(&[("a", "unterminated;b=2")])
    );
    // R23 PARSE in="k="v"junk=w" n=2 out=[k]=[v]|[junk]=[w]
    assert_eq!(parse("k=\"v\"junk=w"), pairs(&[("k", "v"), ("junk", "w")]));
    // R24 PARSE in="x=" n=1 out=[x]=[]
    assert_eq!(parse("x="), pairs(&[("x", "")]));
    // R25 PARSE in="x=1;;y=2" n=2 out=[x]=[1]|[;y]=[2]
    assert_eq!(parse("x=1;;y=2"), pairs(&[("x", "1"), (";y", "2")]));
}
