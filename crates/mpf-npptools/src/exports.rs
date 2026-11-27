use crate::stub::{StubException, STUB_EXPORTS};

/// Whether `npptools.spec` exports `name`. Names are case-sensitive.
pub fn is_export(name: &str) -> bool {
    STUB_EXPORTS.iter().any(|(export, _)| *export == name)
}

/// The record that calling the export `name` raises, or `None` when the spec does not export it.
pub fn call_export(name: &str) -> Option<StubException> {
    STUB_EXPORTS
        .iter()
        .find(|(export, _)| *export == name)
        .map(|(_, stub)| stub())
}
