//! Edge-case tests for the ported msxml6 records.
//!
//! These cover the lookup misses, the uniqueness of the coclass table, and the GUID text form at
//! its extremes. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn export_lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: the oracle only looked up the names that the spec declares.
    assert_eq!(export("dllcanunloadnow"), None);
    assert_eq!(export("DllCanUnloadNow "), None);
    assert_eq!(export(""), None);
    assert_eq!(
        export("DllGetClassObject").map(|entry| entry.kind),
        Some(ExportKind::Forward {
            module: "msxml3",
            function: "DllGetClassObject",
        })
    );
}

#[test]
fn coclass_lookup_is_exact_and_excludes_the_library() {
    // Not oracle-derived: the oracle only looked up coclass names. MSXML2 is the library, not a
    // coclass, so it is not found here.
    assert_eq!(coclass("MSXML2"), None);
    assert_eq!(coclass("domdocument60"), None);
    assert_eq!(coclass(""), None);
    assert_eq!(
        coclass("XMLHTTP60").map(|entry| entry.threading),
        Some(Threading::Apartment)
    );
}

#[test]
fn coclass_names_clsids_and_progids_are_unique() {
    // Not oracle-derived: a structural check on the table built from the oracle rows.
    for (i, first) in COCLASSES.iter().enumerate() {
        for second in &COCLASSES[i + 1..] {
            assert_ne!(first.name, second.name);
            assert_ne!(first.progid, second.progid);
            assert_ne!(first.clsid, second.clsid);
        }
    }
}

#[test]
fn guid_text_form_at_the_extremes() {
    // Not oracle-derived: the all-zero and all-ones GUIDs, and a GUID whose leading bytes are
    // zero, checked against the 8-4-4-4-12 lowercase form that uuid(...) uses.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(
        zero.to_uuid_string(),
        "00000000-0000-0000-0000-000000000000"
    );

    let ones = Guid {
        data1: u32::MAX,
        data2: u16::MAX,
        data3: u16::MAX,
        data4: [0xff; 8],
    };
    assert_eq!(
        ones.to_uuid_string(),
        "ffffffff-ffff-ffff-ffff-ffffffffffff"
    );

    let padded = Guid {
        data1: 1,
        data2: 2,
        data3: 3,
        data4: [0, 1, 2, 3, 4, 5, 6, 0x0f],
    };
    assert_eq!(
        padded.to_uuid_string(),
        "00000001-0002-0003-0001-02030405060f"
    );
}
