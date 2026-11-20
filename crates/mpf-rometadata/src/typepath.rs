//! CLR type-name splitting and comparison from `mdtables.c`: `type_split_name` and
//! `clr_type_path_equals`.
//!
//! Divergence: the C comparison asserts that an empty namespace has no first character, so a
//! name with a leading dot (`.X`) trips the assertion when it is compared. [`TypePath::equals`]
//! compares the two parts and does not assert.

/// A type name split at its last dot. A name with no dot has an empty namespace.
pub struct TypePath<'a> {
    pub namespace: &'a [u8],
    pub type_name: &'a [u8],
}

/// Splits `name` at its last `.` (C `type_split_name`).
pub fn split_type_name(name: &[u8]) -> TypePath<'_> {
    match name.iter().rposition(|&b| b == b'.') {
        Some(sep) => TypePath {
            namespace: &name[..sep],
            type_name: &name[sep + 1..],
        },
        None => TypePath {
            namespace: b"",
            type_name: name,
        },
    }
}

impl TypePath<'_> {
    /// Whether two split names have the same namespace and the same type name (C
    /// `clr_type_path_equals`).
    pub fn equals(&self, other: &TypePath<'_>) -> bool {
        self.namespace == other.namespace && self.type_name == other.type_name
    }
}
