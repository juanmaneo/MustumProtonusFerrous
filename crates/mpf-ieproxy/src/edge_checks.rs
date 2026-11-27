//! Edge-case tests for the ported ieproxy logic.
//!
//! These cover the ordinal rule's boundaries, the lookups, the i386 suffix, the GUID formatting
//! and the constants. Not every expectation is oracle-derived. The ones that aren't say so in a
//! comment.

use super::*;

fn entry(name: &'static str, ordinal: Option<u32>) -> Export {
    Export {
        name,
        ordinal,
        stdcall: true,
        private: false,
        params: &[],
        delegate: None,
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from dlls/ieproxy/Makefile.in and the wrapper IDL files.
    assert_eq!(MODULE, "ieproxy.dll");
    assert_eq!(
        PS_FACTORY_BUFFER_CLSID.to_braced_string(),
        "{a4a1a128-768f-41e0-bf75-e4fddd701cba}"
    );
    assert_eq!(PS_FACTORY_BUFFER.name, "PSFactoryBuffer");
    assert_eq!(PS_FACTORY_BUFFER.clsid, PS_FACTORY_BUFFER_CLSID);
    assert_eq!(PS_FACTORY_BUFFER.threading, "both");
    assert_eq!(PS_FACTORY_BUFFER.interfaces, ["IPSFactoryBuffer"]);
}

#[test]
fn guid_formats_with_zero_padding() {
    // Not oracle-derived: the boundary values of the hex fields.
    let zero = Guid {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    assert_eq!(
        zero.to_braced_string(),
        "{00000000-0000-0000-0000-000000000000}"
    );
    let max = Guid {
        data1: u32::MAX,
        data2: u16::MAX,
        data3: u16::MAX,
        data4: [0xff; 8],
    };
    assert_eq!(
        max.to_braced_string(),
        "{ffffffff-ffff-ffff-ffff-ffffffffffff}"
    );
}

#[test]
fn spec_order_and_shape() {
    // Not oracle-derived: the names and their order are the lines of dlls/ieproxy/ieproxy.spec.
    let names: Vec<&str> = EXPORTS.iter().map(|e| e.name).collect();
    assert_eq!(
        names,
        [
            "GetProxyDllInfo",
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
        ]
    );
    // i386_stdcall_bytes counts four bytes per parameter. This pins the assumption that every
    // parameter type is `ptr`.
    assert!(EXPORTS.iter().flat_map(|e| e.params).all(|p| *p == "ptr"));
    assert!(EXPORTS.iter().all(|e| e.stdcall && e.private));
}

#[test]
fn assign_ordinals_boundaries() {
    // Not oracle-derived: the empty input, and the base when no entry is explicit.
    assert_eq!(assign_ordinals(&[]), Vec::<u32>::new());
    assert_eq!(
        assign_ordinals(&[entry("a", None), entry("b", None)]),
        [1, 2]
    );

    // Not oracle-derived: all explicit, returned unchanged, including out-of-order values.
    assert_eq!(
        assign_ordinals(&[entry("a", Some(9)), entry("b", Some(2))]),
        [9, 2]
    );

    // Not oracle-derived: `@` takes the base when it is free, even though an explicit entry
    // comes later in the spec.
    assert_eq!(
        assign_ordinals(&[entry("a", None), entry("b", Some(1))]),
        [2, 1]
    );

    // Not oracle-derived: `@` skips the explicit base and the ordinals that earlier `@` entries
    // took.
    assert_eq!(
        assign_ordinals(&[
            entry("a", None),
            entry("b", Some(2)),
            entry("c", None),
            entry("d", None),
        ]),
        [3, 2, 4, 5]
    );
}

#[test]
fn lookups_miss_cleanly() {
    // Not oracle-derived: ordinals 0 and 8 are outside the table, and name lookups are
    // case-sensitive.
    assert_eq!(export_name(0), None);
    assert_eq!(export_name(8), None);
    assert_eq!(export_ordinal("DllGetClassObject2"), None);
    assert_eq!(find_export("NoSuchExport"), None);
    assert_eq!(
        find_export("DllCanUnloadNow").map(|e| e.name),
        Some("DllCanUnloadNow")
    );
}

#[test]
fn i386_suffix_follows_stdcall_and_param_count() {
    // Not oracle-derived for the non-stdcall branch: the winebuild runs used only stdcall entries.
    assert_eq!(i386_stdcall_bytes(&entry("a", None)), Some(0));

    let cdecl = Export {
        stdcall: false,
        ..entry("b", None)
    };
    assert_eq!(i386_stdcall_bytes(&cdecl), None);

    let three = Export {
        params: &["ptr", "ptr", "ptr"],
        ..entry("c", None)
    };
    assert_eq!(i386_stdcall_bytes(&three), Some(12));
}
