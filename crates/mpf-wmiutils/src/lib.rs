//! wmiutils subset for MPF, ported from Wine dlls/wmiutils/path.c and statuscode.c.
//!
//! Ported: IWbemPath::SetText parsing of the server, namespaces, class and keys;
//! IWbemPath::GetText for every flag; GetInfo; the server, namespace and class
//! accessors; IWbemPathKeyList GetCount and RemoveAllKeys; and the text of
//! IWbemStatusCodeText::GetErrorCodeText and GetFacilityCodeText.
//!
//! Skipped: COM object creation and reference counting, the critical sections,
//! the scope accessors, IsRelative and the other comparison helpers,
//! CreateClassPart and DeleteClassPart, IWbemPathKeyList GetKey, SetKey and
//! GetKeyList, and WbemStatusCodeText_create.
//!
//! Divergences: these inputs make the C code read or write past a key, so they
//! are rejected with WBEM_E_INVALID_PARAMETER: a dotted key without '=', an empty
//! dotted key list, and an empty piece in a dotted key list (a trailing or
//! doubled comma). A single key after '=' reads only its first piece, as in C.
//!
//! Kept from C: the length GetText reports for an implied key counts the '.'
//! even when the text has none, and a namespace region is split on separators up
//! to the first ':', even inside a key value.
//!
//! Whitespace is the ASCII set 0x09 to 0x0d and 0x20, the C locale of iswspace.
//! Text is UTF-16 code units, as in Wine.

mod build;
mod parse;
mod path;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use path::ObjectPath;

pub const WBEM_E_INVALID_PARAMETER: i32 = 0x8004_1008u32 as i32;
pub const WBEM_E_NOT_AVAILABLE: i32 = 0x8004_1009u32 as i32;
pub const WBEM_E_INVALID_OBJECT_PATH: i32 = 0x8004_103Au32 as i32;

pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: u64 = 0x1;
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: u64 = 0x2;
pub const WBEMPATH_INFO_IS_CLASS_REF: u64 = 0x4;
pub const WBEMPATH_INFO_IS_INST_REF: u64 = 0x8;
pub const WBEMPATH_INFO_HAS_SUBSCOPES: u64 = 0x10;
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: u64 = 0x80;
pub const WBEMPATH_INFO_V1_COMPLIANT: u64 = 0x200;
pub const WBEMPATH_INFO_V2_COMPLIANT: u64 = 0x400;
pub const WBEMPATH_INFO_CIM_COMPLIANT: u64 = 0x800;
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: u64 = 0x4000;
pub const WBEMPATH_INFO_PATH_HAD_SERVER: u64 = 0x20000;

pub const WBEMPATH_GET_RELATIVE_ONLY: i32 = 0x2;
pub const WBEMPATH_GET_SERVER_TOO: i32 = 0x4;
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: i32 = 0x8;
pub const WBEMPATH_GET_NAMESPACE_ONLY: i32 = 0x10;
pub const WBEMPATH_GET_ORIGINAL: i32 = 0x20;

pub(crate) const BACKSLASH: u16 = b'\\' as u16;
pub(crate) const SLASH: u16 = b'/' as u16;
pub(crate) const COLON: u16 = b':' as u16;
pub(crate) const DOT: u16 = b'.' as u16;
pub(crate) const EQ: u16 = b'=' as u16;
pub(crate) const COMMA: u16 = b',' as u16;

/// The text IWbemStatusCodeText::GetErrorCodeText gives for `res`. The LCID and
/// the flags are ignored, as in Wine.
pub fn error_code_text(res: i32) -> String {
    format!("Error code: 0x{:08x}", res as u32)
}

/// The text IWbemStatusCodeText::GetFacilityCodeText gives for `res`. The LCID
/// and the flags are ignored, as in Wine.
pub fn facility_code_text(res: i32) -> String {
    format!("Facility code: 0x{:08x}", res as u32)
}
