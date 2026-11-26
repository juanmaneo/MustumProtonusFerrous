//! Port of the portable logic in Wine's `dlls/atl/registrar.c`.
//!
//! Ported:
//! - `get_word`: the script tokenizer, with `'...'` quoting, `''` escapes and the
//!   single-character words `{`, `}` and `=` (`script.rs`)
//! - `do_preprocess`: `%NAME%` and `%%` expansion, newest replacement first (`preprocess.rs`):
//!   [`Registrar::add_replacement`], [`Registrar::clear_replacements`]
//! - `do_process_root_key`, `do_process_key`: root lookup, the `NoRemove`, `ForceRemove`,
//!   `val` and `Delete` keywords, typed values and nested blocks, for both the register
//!   and unregister passes (`walk.rs`, `registry.rs`)
//! - `string_register`: [`Registrar::string_register`], including the unregister pass that
//!   runs after a failed register
//! - `HRESULT_FROM_WIN32`: [`Error::hresult`]
//! - The `d` value's `wcstoul(s, NULL, 10)` and the `b` value's hex decoding (`codec.rs`)
//!
//! Skipped:
//! - `IRegistrar` COM vtable, `QueryInterface`, `AddRef`, `Release`: COM plumbing.
//! - `resource_register`, `file_register` and their `IRegistrar` methods: `LoadLibraryExW`,
//!   `FindResourceW`, `CreateFileW`.
//! - The `Reg*W` calls: the caller supplies them through [`Registry`].
//! - `strbuf` and its `malloc`/`realloc` calls: `Vec` replaces them.
//!
//! Deviations from the C code:
//! - An unclosed block is an `Exception`. The C loop spins forever at the end of input.
//! - Whitespace and case folding are ASCII-only, not locale-aware.

mod codec;
mod error;
mod preprocess;
mod registry;
mod roots;
mod script;
mod text;
mod walk;

pub use error::Error;
pub use registry::{Registry, Value};
pub use roots::RootKey;

use preprocess::Replacement;

/// `ATLRegistrar`: the replacement list that `%NAME%` expands through.
#[derive(Default)]
pub struct Registrar {
    replacements: Vec<Replacement>,
}

impl Registrar {
    /// `Registrar_AddReplacement`. The key and the item are cut at their first NUL.
    pub fn add_replacement(&mut self, key: &[u16], item: &[u16]) {
        self.replacements.push(Replacement {
            key: text::until_nul(key).to_vec(),
            item: text::until_nul(item).to_vec(),
        });
    }

    /// `Registrar_ClearReplacements`.
    pub fn clear_replacements(&mut self) {
        self.replacements.clear();
    }

    /// `string_register`: runs the script in `data` as a register pass, or as an unregister
    /// pass when `register` is false. A failed register pass is followed by an unregister
    /// pass, whose result is ignored.
    pub fn string_register<R: Registry>(
        &self,
        reg: &mut R,
        data: &[u16],
        register: bool,
    ) -> Result<(), Error> {
        let script = preprocess::preprocess(text::until_nul(data), &self.replacements)?;
        let result = walk::process_root_keys(reg, &script, register);
        if register && result.is_err() {
            let _ = walk::process_root_keys(reg, &script, false);
        }
        result
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
