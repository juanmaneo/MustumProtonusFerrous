//! Headless port of the pure string helpers in Wine's dlls/hhctrl.ocx, the HtmlHelp viewer.
//!
//! Ported:
//! - get_node_name from stream.c: [node_name].
//! - get_attr from stream.c: [attr_value].
//! - skip_schema from chm.c: [strip_schema].
//! - is_chm from help.c: [is_chm_url].
//! - decode_html from help.c, up to its code page conversion: [decode_entities], with the
//!   entity table and find_html_symbol.
//!
//! Differences from the C code:
//! - [strip_schema] folds ASCII letters only. wcsnicmp also folds other letters through
//!   towlower, and the result for those depends on the locale.
//! - [decode_entities] returns bytes. decode_html passes them to MultiByteToWideChar with the
//!   caller's code page, and the port leaves that conversion to the caller.
//! - Numeric references such as `&#65;` are never decoded. help.c passes the `#` to strtol,
//!   which converts nothing, so the reference is copied through. The port keeps that behaviour.
//! - [attr_value] searches the whole slice. get_attr stops at the first NUL byte, because it
//!   works on a C string. The inputs here do not contain NUL bytes.
//! - [node_name] ends at the end of the slice when the tag has no `>` or whitespace. The C code
//!   reads the NUL that next_node appends.
//!
//! Skipped:
//! - Every function that needs the OS, files, COM, windows, the registry or the network, and the
//!   FIXME and TRACE logging.

mod attr;
mod entity;
mod node;
mod schema;

pub use attr::attr_value;
pub use entity::decode_entities;
pub use node::node_name;
pub use schema::{is_chm_url, strip_schema};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
