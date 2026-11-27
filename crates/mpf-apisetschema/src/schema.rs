//! The `apiset` records of a spec file, as `parse_spec_apiset` in `tools/winebuild/parser.c` adds them.

use crate::hash::apiset_hash;
use crate::pool::StringPool;
use crate::tokens::Tokenizer;

/// `ARRAY_SIZE(entry->values) - 1`: `struct apiset_entry` has four value slots, and one stays spare.
const MAX_VALUES: usize = 3;

/// The errors that make winebuild fail. Wine reports each one and keeps parsing; this port stops at the first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecError {
    /// A line whose first token is not `apiset`. Wine reports these as "Expected ordinal declaration".
    UnknownDirective,
    /// A line with no name, or a name that is not followed by `=`.
    Syntax,
    /// A name with no `-`.
    NoHyphen,
    /// A name that is already defined.
    DuplicateName,
    /// A name whose hash equals the hash of an earlier name.
    DuplicateHash,
    /// More than three values.
    TooManyValues,
}

/// One `apiset` record: `struct apiset_entry`. Offsets index the string pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name_off: usize,
    pub name_len: usize,
    pub hash: u32,
    pub hash_len: usize,
    pub values: Vec<Value>,
}

/// One value of a record: `struct apiset_value`. `name_off` and `name_len` are 0 when the value has no `name:` part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
    pub name_off: usize,
    pub name_len: usize,
    pub val_off: usize,
    pub val_len: usize,
}

/// The `apiset` records of a spec file, in declaration order, and the string pool they point into.
#[derive(Debug, Default)]
pub struct Schema {
    pub entries: Vec<Entry>,
    pub(crate) pool: StringPool,
}

impl Schema {
    /// `parse_spec` for a data-only spec file: each line in turn, stopping at the first error.
    pub fn parse_spec(text: &[u8]) -> Result<Self, SpecError> {
        let mut schema = Self::default();
        for line in text.split(|&byte| byte == b'\n') {
            schema.parse_line(line)?;
        }
        Ok(schema)
    }

    /// One line of `parse_spec()`, without the ordinal and `@` declarations.
    pub fn parse_line(&mut self, line: &[u8]) -> Result<(), SpecError> {
        let mut tokens = Tokenizer::new(line);
        let Some(directive) = tokens.next_token() else {
            return Ok(());
        };
        if directive != b"apiset" {
            return Err(SpecError::UnknownDirective);
        }
        let name = tokens.next_token().ok_or(SpecError::Syntax)?;
        let (hash, hash_len) = apiset_hash(&name).ok_or(SpecError::NoHyphen)?;
        self.check_unique(&name, hash)?;
        if tokens.next_token().as_deref() != Some(b"=".as_slice()) {
            return Err(SpecError::Syntax);
        }
        let mut values = Vec::new();
        while let Some(value) = tokens.next_token() {
            values.push(value);
        }
        self.add_record(&name, hash, hash_len, &values)
    }

    /// The duplicate checks of `parse_spec_apiset`, in Wine's order: for each existing record, the name first, then the hash.
    fn check_unique(&self, name: &[u8], hash: u32) -> Result<(), SpecError> {
        for entry in &self.entries {
            if self.name_of(entry) == name {
                return Err(SpecError::DuplicateName);
            }
            if entry.hash == hash {
                return Err(SpecError::DuplicateHash);
            }
        }
        Ok(())
    }

    fn name_of(&self, entry: &Entry) -> &[u8] {
        &self.pool.bytes()[entry.name_off..entry.name_off + entry.name_len]
    }

    /// `add_apiset` and `add_apiset_value`: adds the name, then the values. Nothing changes when a value is rejected.
    fn add_record(
        &mut self,
        name: &[u8],
        hash: u32,
        hash_len: usize,
        values: &[Vec<u8>],
    ) -> Result<(), SpecError> {
        if values.len() > MAX_VALUES {
            return Err(SpecError::TooManyValues);
        }
        let name_off = self.pool.add(name, name.len());
        let values = values.iter().map(|value| self.add_value(value)).collect();
        self.entries.push(Entry {
            name_off,
            name_len: name.len(),
            hash,
            hash_len,
            values,
        });
        Ok(())
    }

    /// `add_apiset_value`: a `name:value` token splits at its first `:`.
    fn add_value(&mut self, value: &[u8]) -> Value {
        match value.iter().position(|&byte| byte == b':') {
            Some(sep) => {
                let name_off = self.pool.add(value, sep);
                let val = &value[sep + 1..];
                Value {
                    name_off,
                    name_len: sep,
                    val_off: self.pool.add(val, val.len()),
                    val_len: val.len(),
                }
            }
            None => Value {
                name_off: 0,
                name_len: 0,
                val_off: self.pool.add(value, value.len()),
                val_len: value.len(),
            },
        }
    }
}
