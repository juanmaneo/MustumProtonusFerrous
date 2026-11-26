//! Edge-case tests for the ported oledb32 logic.
//!
//! These cover every arm of the C switches, both sides of each `||` and `&&`, the parser's
//! branches, and the derived trait functions. Expectations that are not oracle-derived are
//! marked as such in a comment. Their values come from reading the Wine C source.

use crate::class_object::{
    FactoryKind, Guid, ObjOut, CLASS_E_CLASSNOTAVAILABLE, CLSID_EXTENDEDERRORINFO,
    CLSID_MSDAINITIALIZE, CLSID_OLEDB_CONVERSIONLIBRARY, CLSID_OLEDB_ROWPOSITIONLIBRARY,
    E_NOINTERFACE, IID_ICLASSFACTORY, IID_IUNKNOWN, S_OK,
};
use crate::dbtype::{
    DBTYPE_BOOL, DBTYPE_BSTR, DBTYPE_BYREF_WSTR, DBTYPE_BYTES, DBTYPE_CY, DBTYPE_DATE,
    DBTYPE_DBDATE, DBTYPE_DBTIME, DBTYPE_DBTIMESTAMP, DBTYPE_DECIMAL, DBTYPE_EMPTY,
    DBTYPE_FILETIME, DBTYPE_GUID, DBTYPE_I1, DBTYPE_I2, DBTYPE_I4, DBTYPE_I8, DBTYPE_NULL,
    DBTYPE_NUMERIC, DBTYPE_R4, DBTYPE_R8, DBTYPE_STR, DBTYPE_UI1, DBTYPE_UI2, DBTYPE_UI4,
    DBTYPE_UI8, DBTYPE_VARIANT, DBTYPE_WSTR,
};
use crate::{
    array_type, cf_query_interface, class_for_clsid, common_class, dbtype_class, dbtype_length,
    dll_can_unload_now, dll_get_class_object, dll_main, parse_init_string, DbProp,
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
fn dbtype_length_covers_every_arm() {
    // Not oracle-derived: each expectation is the value of the matching arm in convert.c's
    // get_length, on x86-64.
    let cases: &[(u16, usize)] = &[
        (DBTYPE_I1, 1),
        (DBTYPE_UI1, 1),
        (DBTYPE_I2, 2),
        (DBTYPE_UI2, 2),
        (DBTYPE_BOOL, 2),
        (DBTYPE_I4, 4),
        (DBTYPE_UI4, 4),
        (DBTYPE_R4, 4),
        (DBTYPE_I8, 8),
        (DBTYPE_UI8, 8),
        (DBTYPE_R8, 8),
        (DBTYPE_DATE, 8),
        (DBTYPE_DBDATE, 6),
        (DBTYPE_DBTIMESTAMP, 16),
        (DBTYPE_CY, 8),
        (DBTYPE_BSTR, 8),
        (DBTYPE_FILETIME, 8),
        (DBTYPE_GUID, 16),
        (DBTYPE_NUMERIC, 19),
        (DBTYPE_BYTES, 0),
        (DBTYPE_WSTR, 0),
        (DBTYPE_STR, 0),
        (DBTYPE_BYREF_WSTR, 0),
        (DBTYPE_VARIANT, 24),
        // The default arm: unknown or unsized types give 0.
        (DBTYPE_EMPTY, 0),
        (DBTYPE_DBTIME, 0),
        (0x4000, 0),
    ];
    for &(ty, len) in cases {
        assert_eq!(dbtype_length(ty), len, "get_length({ty})");
    }
}

#[test]
fn dbtype_class_covers_every_arm() {
    // Not oracle-derived: each expectation is the return value of the matching arm in
    // convert.c's get_dbtype_class. Unlisted types map to themselves.
    let cases: &[(u16, u16)] = &[
        (DBTYPE_I2, DBTYPE_I2),
        (DBTYPE_R4, DBTYPE_I2),
        (DBTYPE_R8, DBTYPE_I2),
        (DBTYPE_I1, DBTYPE_I2),
        (DBTYPE_UI1, DBTYPE_I2),
        (DBTYPE_UI2, DBTYPE_I2),
        (DBTYPE_I4, DBTYPE_I4),
        (DBTYPE_UI4, DBTYPE_I4),
        (DBTYPE_I8, DBTYPE_I8),
        (DBTYPE_UI8, DBTYPE_I8),
        (DBTYPE_BSTR, DBTYPE_BSTR),
        (DBTYPE_STR, DBTYPE_BSTR),
        (DBTYPE_WSTR, DBTYPE_BSTR),
        (DBTYPE_DBDATE, DBTYPE_DBDATE),
        (DBTYPE_DBTIME, DBTYPE_DBDATE),
        (DBTYPE_DBTIMESTAMP, DBTYPE_DBDATE),
        (DBTYPE_EMPTY, DBTYPE_EMPTY),
        (DBTYPE_BOOL, DBTYPE_BOOL),
        (DBTYPE_VARIANT, DBTYPE_VARIANT),
        (DBTYPE_NUMERIC, DBTYPE_NUMERIC),
    ];
    for &(ty, class) in cases {
        assert_eq!(dbtype_class(ty), class, "get_dbtype_class({ty})");
    }
}

#[test]
fn common_class_members_and_non_members() {
    // Not oracle-derived: membership from the switch in convert.c's common_class.
    for ty in [
        DBTYPE_EMPTY,
        DBTYPE_NULL,
        DBTYPE_I2,
        DBTYPE_I4,
        DBTYPE_BSTR,
        DBTYPE_BOOL,
        DBTYPE_VARIANT,
        DBTYPE_I8,
        DBTYPE_CY,
        DBTYPE_DECIMAL,
        DBTYPE_NUMERIC,
    ] {
        assert!(common_class(ty), "common_class({ty})");
    }
    for ty in [
        DBTYPE_I1,
        DBTYPE_R4,
        DBTYPE_DATE,
        DBTYPE_WSTR,
        DBTYPE_DBDATE,
    ] {
        assert!(!common_class(ty), "common_class({ty})");
    }
}

#[test]
fn array_type_range_boundaries() {
    // Not oracle-derived: the bounds DBTYPE_I2 (2) and DBTYPE_UI4 (19) from convert.c.
    assert!(!array_type(0));
    assert!(!array_type(1));
    assert!(array_type(2));
    assert!(array_type(12));
    assert!(array_type(19));
    assert!(!array_type(20));
}

#[test]
fn class_for_clsid_requires_every_field() {
    // Not oracle-derived: a CLSID that differs from CLSID_MSDAINITIALIZE only in its last
    // byte is not a known class.
    let mut near = CLSID_MSDAINITIALIZE;
    near.data4[7] = 0x28;
    assert_eq!(class_for_clsid(&near), None);
    assert_eq!(
        class_for_clsid(&CLSID_MSDAINITIALIZE),
        Some(FactoryKind::DataInit)
    );
}

#[test]
fn cf_query_interface_refuses_near_miss_iids() {
    // Not oracle-derived: an IID that differs from IID_IUnknown in its last byte is refused.
    let mut near = IID_IUNKNOWN;
    near.data4[7] = 0x47;
    assert_eq!(cf_query_interface(&near), E_NOINTERFACE);
    assert_eq!(cf_query_interface(&IID_IUNKNOWN), S_OK);
    assert_eq!(cf_query_interface(&IID_ICLASSFACTORY), S_OK);
}

#[test]
fn unknown_clsid_is_refused_for_any_iid() {
    // Not oracle-derived: the unknown-CLSID path ignores the IID.
    for iid in [IID_IUNKNOWN, IID_ICLASSFACTORY] {
        assert_eq!(
            dll_get_class_object(&GUID_NULL_FOR_TEST, &iid),
            (CLASS_E_CLASSNOTAVAILABLE, ObjOut::Null)
        );
    }
}

const GUID_NULL_FOR_TEST: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

#[test]
fn every_known_class_accepts_iclassfactory() {
    // Not oracle-derived: the four classes not covered by an oracle row for IClassFactory.
    let cases = [
        (CLSID_OLEDB_CONVERSIONLIBRARY, FactoryKind::Convert),
        (CLSID_MSDAINITIALIZE, FactoryKind::DataInit),
        (CLSID_EXTENDEDERRORINFO, FactoryKind::ErrorInfo),
        (CLSID_OLEDB_ROWPOSITIONLIBRARY, FactoryKind::RowPos),
    ];
    for (clsid, kind) in cases {
        assert_eq!(
            dll_get_class_object(&clsid, &IID_ICLASSFACTORY),
            (S_OK, ObjOut::Factory(kind))
        );
    }
}

#[test]
fn dll_main_and_can_unload_are_fixed() {
    // Not oracle-derived: DllMain returns TRUE after its switch for every reason. Reasons 0,
    // 2 and 3 are the process detach, thread attach and thread detach values.
    for reason in [0, 1, 2, 3] {
        assert!(dll_main(reason), "DllMain({reason})");
    }
    assert_eq!(dll_can_unload_now(), 1);
}

#[test]
fn parse_empty_and_without_equals() {
    // Not oracle-derived: no '=' means no pair, so the loop ends at once.
    assert_eq!(parse(""), pairs(&[]));
    assert_eq!(parse("abc"), pairs(&[]));
    assert_eq!(parse(";;;"), pairs(&[]));
}

#[test]
fn parse_stops_at_first_nul() {
    // Not oracle-derived: the C string ends at its first NUL. The text after it is ignored.
    assert_eq!(parse("a=1\0b=2"), pairs(&[("a", "1")]));
    assert_eq!(parse("\0a=1"), pairs(&[]));
}

#[test]
fn parse_drops_only_exact_provider_names() {
    // Not oracle-derived: Provider is dropped case-insensitively, but only on an exact match.
    assert_eq!(parse("provider=x;a=1"), pairs(&[("a", "1")]));
    assert_eq!(parse("PrOvIdEr=x;a=1"), pairs(&[("a", "1")]));
    assert_eq!(parse("Provider =x"), pairs(&[("Provider ", "x")]));
    assert_eq!(parse("Providers=1"), pairs(&[("Providers", "1")]));
    assert_eq!(parse("Provide=1"), pairs(&[("Provide", "1")]));
    assert_eq!(parse("Providex=1"), pairs(&[("Providex", "1")]));
}

#[test]
fn parse_skips_leading_space_classes() {
    // Not oracle-derived: iswspace is space and \t through \r. The C code skips these before
    // a name, and only these.
    assert_eq!(parse("\t\n\u{b}\u{c}\r a=1"), pairs(&[("a", "1")]));
    assert_eq!(parse("\u{8}a=1"), pairs(&[("\u{8}a", "1")]));
    assert_eq!(parse("\u{e}a=1"), pairs(&[("\u{e}a", "1")]));
}

#[test]
fn parse_quoted_values() {
    // Not oracle-derived: quote handling, from datainit.c's quoted-value branch.
    assert_eq!(parse("a=\"x;y\""), pairs(&[("a", "x;y")]));
    assert_eq!(parse("a='x\"y'"), pairs(&[("a", "x\"y")]));
    assert_eq!(parse("a=\"x'"), pairs(&[("a", "x'")]));
    assert_eq!(parse("a=\""), pairs(&[("a", "")]));
    assert_eq!(parse("a=\"\""), pairs(&[("a", "")]));
    assert_eq!(parse("a=\"v\""), pairs(&[("a", "v")]));
    assert_eq!(parse("a=\"v\";"), pairs(&[("a", "v")]));
    assert_eq!(parse("a='v';b=2"), pairs(&[("a", "v"), ("b", "2")]));
    assert_eq!(parse("a=\"v\"x"), pairs(&[("a", "v")]));
}

#[test]
fn parse_unquoted_values() {
    // Not oracle-derived: values end at ';' and keep their blanks. '=' inside a value is
    // ordinary text.
    assert_eq!(parse("a=b=c"), pairs(&[("a", "b=c")]));
    assert_eq!(parse("a=1;"), pairs(&[("a", "1")]));
    assert_eq!(parse("a=1;;"), pairs(&[("a", "1")]));
    assert_eq!(parse("=v"), pairs(&[("", "v")]));
    assert_eq!(parse("a=1;;b=2"), pairs(&[("a", "1"), (";b", "2")]));
    assert_eq!(parse("a=1 ;b=2"), pairs(&[("a", "1 "), ("b", "2")]));
    assert_eq!(parse("a b = c"), pairs(&[("a b ", " c")]));
}

#[test]
fn derived_impls_are_exercised() {
    // Not oracle-derived: calls the derived Clone and Debug impls, which the assertions above
    // do not reach, so the coverage gate sees them.
    let guid = CLSID_MSDAINITIALIZE;
    assert_eq!(guid.clone(), guid);
    assert!(format!("{guid:?}").contains("data1"));

    let kind = FactoryKind::RowPos;
    assert_eq!(kind.clone(), kind);
    assert!(format!("{kind:?}").contains("RowPos"));

    let out = ObjOut::Factory(FactoryKind::DataLinks);
    assert_eq!(out.clone(), out);
    assert!(format!("{out:?}").contains("DataLinks"));

    let prop = DbProp {
        name: vec![0x61],
        value: vec![0x31],
    };
    assert_eq!(prop.clone(), prop);
    assert!(format!("{prop:?}").contains("name"));
}
