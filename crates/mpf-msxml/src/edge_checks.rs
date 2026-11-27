//! Edge-case tests for the ported msxml logic.
//!
//! These cover the export table shape, the forward split at the last dot, the file-name rule and
//! the class lookup. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn export_table_is_in_spec_order() {
    // Not oracle-derived: pins the table to the four entries of `msxml.spec`, in order.
    let names = EXPORTS.map(|entry| entry.name);
    assert_eq!(
        names,
        [
            "DllCanUnloadNow",
            "DllGetClassObject",
            "DllRegisterServer",
            "DllUnregisterServer",
        ]
    );
}

#[test]
fn every_entry_is_found_by_its_own_name() {
    // Not oracle-derived: the lookup over the whole table, where the oracle rows sample it.
    for entry in EXPORTS {
        assert_eq!(export(entry.name), Some(entry));
    }
}

#[test]
fn only_dll_get_class_object_is_forwarded() {
    // Not oracle-derived: the forward column of the whole table, where the oracle rows sample it.
    let forwarded: Vec<&str> = EXPORTS
        .iter()
        .filter(|entry| entry.forward.is_some())
        .map(|entry| entry.name)
        .collect();
    assert_eq!(forwarded, ["DllGetClassObject"]);
}

#[test]
fn export_lookup_rejects_empty_and_prefixed_names() {
    // Not oracle-derived: an empty name and a prefix of a real name are both absent.
    assert_eq!(export(""), None);
    assert_eq!(export("DllCanUnloadNow2"), None);
    assert_eq!(export("DllCanUnload"), None);
}

#[test]
fn split_forward_takes_the_last_dot() {
    // Not oracle-derived: `strrchr` picks the last dot, and the oracle only has `a.b.c`.
    assert_eq!(split_forward("a.b.c"), Some(("a.b", "c")));
    // Not oracle-derived: a trailing dot leaves an empty function name.
    assert_eq!(split_forward("msxml3."), Some(("msxml3", "")));
    // Not oracle-derived: a leading dot leaves an empty module name.
    assert_eq!(split_forward(".x"), Some(("", "x")));
}

#[test]
fn split_forward_rejects_targets_without_a_dot() {
    // Not oracle-derived: winebuild only calls the split for dotted targets, and the C code
    // writes through a NULL `strrchr` result for the others. The port returns `None` instead.
    assert_eq!(split_forward("DllMain"), None);
    assert_eq!(split_forward(""), None);
}

#[test]
fn get_dll_name_appends_only_when_there_is_no_dot() {
    // Not oracle-derived: an empty name gets the suffix, and a name with a dot anywhere is kept.
    assert_eq!(get_dll_name(""), ".dll");
    assert_eq!(
        get_dll_name("msxml3.DllGetClassObject"),
        "msxml3.DllGetClassObject"
    );
    assert_eq!(get_dll_name(".x"), ".x");
}

#[test]
fn every_class_is_found_by_its_own_prog_id() {
    // Not oracle-derived: the lookup over the whole class table, where the oracle rows sample it.
    for class in COM_CLASSES {
        assert_eq!(com_class(class.prog_id), Some(class));
    }
}

#[test]
fn com_class_matches_the_whole_key_name() {
    // Not oracle-derived: a prefix of a key name and a different case are absent. The case rule is
    // a port choice; the reference script has no lookup of its own.
    assert_eq!(com_class("MSXML.DOMDocument.1"), None);
    assert_eq!(com_class("MSXML.DOMDOCUMENT"), None);
    assert_eq!(com_class("msxml.domdocument"), None);
}
