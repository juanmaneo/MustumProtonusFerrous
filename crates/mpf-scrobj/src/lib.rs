//! Port of the portable logic in Wine's `dlls/scrobj/scrobj.c` and `dlls/scrobj/scrobj.spec`.
//!
//! Ported:
//! - Script host and member lookup: [`find_script_host`], [`lookup_script_properties`],
//!   [`lookup_object_members`] and [`property_accessor_names`], over [`ScriptletMember`],
//!   [`ObjectMember`], [`ScriptReference`] and [`MemberType`].
//! - XML helpers: [`is_xml_name`], [`is_case_xml_name`], [`expect_no_attributes`], the
//!   [`NodeReader`] that stands in for `IXmlReader`, and [`parse_property_flags`], which is the
//!   `MEMBER_PROPERTY` case of `parse_scriptlet_public`. `PROP_GETTER` and `PROP_SETTER` are the
//!   flag bits it sets.
//! - COM methods whose body is a fixed return value: the `E_NOTIMPL` methods of the global and
//!   scriptlet `IDispatchEx` objects and of the typelib object, and `scriptlet_factory_LockServer`
//!   (`S_OK`). See [`com`].
//!
//! Skipped:
//! - `DllMain`, `DllInstall`, `DllGetClassObject` and the registry code in `register_scriptlet`
//!   and `unregister_scriptlet`: they depend on the OS loader, the registry and the factory
//!   objects.
//! - `parse_scriptlet_registration`, the rest of `parse_scriptlet_public`, `parse_scriptlet_script`
//!   and `parse_scriptlet_file`: they read the XML through `IXmlReader` and download the file with
//!   urlmon. Only the property case is ported.
//! - The script engine (`create_script_host`, `init_script_host`, `parse_scripts`, the
//!   `IActiveScriptSite` callbacks) and the COM dispatch (`scriptlet_InvokeEx`,
//!   `global_GetIDsOfNames`, the typelib `Invoke`).
//! - `scriptlet_typelib_get_GUID`: it calls `CoCreateGuid` and `StringFromGUID2`. The C code tests
//!   the `StringFromGUID2` result with `FAILED`, but that call returns a character count, so the
//!   test never fails for a count of 0.
//! - The `scrobj.spec` export table: all five entries are `-private`, and none is a `@ stub`.
//! - Wine's `_memicmp` is modelled byte by byte, so only ASCII letters fold, as in the C code.
//!   Non-ASCII case folding is not modelled.

mod com;
mod lookup;
mod xml;

pub use com::*;
pub use lookup::{
    find_script_host, lookup_object_members, lookup_script_properties, property_accessor_names,
    DispId, MemberType, ObjectMember, ScriptReference, ScriptletMember,
};
pub use xml::{
    expect_no_attributes, is_case_xml_name, is_xml_name, parse_property_flags, NodeReader, XmlNode,
    PROP_GETTER, PROP_SETTER,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
