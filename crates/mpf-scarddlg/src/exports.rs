/// The `scarddlg.spec` export table, in spec order: each ordinal with its name. Every entry is `@ stub`.
pub const EXPORTS: [(u16, &str); 5] = [
    (1, "GetOpenCardNameA"),
    (2, "GetOpenCardNameW"),
    (3, "SCardDlgExtendedError"),
    (4, "SCardUIDlgSelectCardA"),
    (5, "SCardUIDlgSelectCardW"),
];

/// The ordinal of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_ordinal(name: &str) -> Option<u16> {
    EXPORTS
        .iter()
        .find(|(_, n)| *n == name)
        .map(|&(ordinal, _)| ordinal)
}
