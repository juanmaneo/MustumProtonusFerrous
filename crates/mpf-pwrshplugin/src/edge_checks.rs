//! Edge-case tests for the ported pwrshplugin stubs.
//!
//! These cover the constants, the stub record for names other than the 12 exports, the export
//! table, and the lookup of unknown names. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The oracle rows
    // print the same code and flags, but these are the header values.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of pwrshplugin.spec. The code,
    // flags and parameter count are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the twelve `@ stub` lines of dlls/pwrshplugin/pwrshplugin.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            "GetCLRVersionForPSVersion",
            "PerformWSManPluginReportCompletion",
            "WSManPluginCommand",
            "WSManPluginConnect",
            "WSManPluginReceive",
            "WSManPluginReleaseCommandContext",
            "WSManPluginReleaseShellContext",
            "WSManPluginSend",
            "WSManPluginShell",
            "WSManPluginShutdown",
            "WSManPluginSignal",
            "WSManPluginStartup",
        ]
    );
}

#[test]
fn every_export_resolves_to_its_own_stub() {
    // Not oracle-derived: each name in the table must map to a record for that same function,
    // with the module name from dlls/pwrshplugin/Makefile.in.
    for name in EXPORTS {
        let stub = export_stub(name).expect(name);
        assert_eq!(stub.function, name);
        assert_eq!(stub.module, "pwrshplugin.dll");
    }
}

#[test]
fn unknown_names_are_not_exports() {
    // Not oracle-derived: names the spec does not export. The lookup is case-sensitive, and
    // empty names and names with extra characters do not match.
    for name in [
        "",
        "getclrversionforpsversion",
        "WSManPluginCommand ",
        "WSManPluginCommandX",
        "DllMain",
    ] {
        assert_eq!(export_stub(name), None, "{name:?}");
    }
}
