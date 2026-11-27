//! Port of the portable logic in Wine's `dlls/msimsg/msimsg.rc`.
//!
//! `dlls/msimsg` has no C sources and no `.spec` file. It is a resource-only DLL built with
//! `-Wl,-noentry`, so it exports nothing and has no `@ stub` records to model. The port covers
//! the one resource it carries.
//!
//! Ported:
//! - `1703 RCDATA` reboot prompt: [`reboot_prompt_resource`], [`reboot_prompt_rcdata`],
//!   [`ResourceEntry`], [`REBOOT_PROMPT`], [`REBOOT_PROMPT_ID`] and [`RT_RCDATA`]
//! - `LANGUAGE LANG_ENGLISH, SUBLANG_DEFAULT`: [`make_lang_id`], [`REBOOT_PROMPT_LANG_ID`],
//!   [`LANG_ENGLISH`] and [`SUBLANG_DEFAULT`], with the values from `include/winnt.rh` and the
//!   `MAKELANGID` macro from `include/winnt.h`
//!
//! Skipped:
//! - The `.res` container layout (header entry, DWORD padding, DataVersion, MemoryFlags,
//!   Version and Characteristics). The resource compiler writes it and the loader reads it;
//!   this DLL never parses it.
//! - Substitution of the `[2]` placeholder. This DLL contains no code that fills it in, so the
//!   text is ported verbatim.

mod resource;

pub use resource::{
    make_lang_id, reboot_prompt_rcdata, reboot_prompt_resource, ResourceEntry, LANG_ENGLISH,
    REBOOT_PROMPT, REBOOT_PROMPT_ID, REBOOT_PROMPT_LANG_ID, RT_RCDATA, SUBLANG_DEFAULT,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
