//! Script host and member lookup from `dlls/scrobj/scrobj.c`.
//!
//! The C code asks each script host's `IDispatchEx` for a name. Here the host list is a count
//! and the lookup is a closure from `(host index, name)` to a dispatch ID, so the logic runs
//! without COM.

use crate::com::{E_FAIL, HRESULT};

/// `DISPID` from `include/oaidl.h`.
pub type DispId = i32;

/// `struct script_reference`. The C struct holds a host pointer; this port holds the host's
/// index in the object's host list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScriptReference {
    pub host: usize,
    pub id: DispId,
}

/// `MEMBER_METHOD` and `MEMBER_PROPERTY` from `scrobj.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberType {
    Method,
    Property,
}

/// One member declared by a scriptlet, as `struct scriptlet_member` holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptletMember {
    pub kind: MemberType,
    pub name: Vec<u16>,
}

/// One member of a scriptlet instance after lookup, as `struct scriptlet_instance` holds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectMember {
    Method {
        name: Vec<u16>,
        script: ScriptReference,
    },
    Property {
        name: Vec<u16>,
        get: ScriptReference,
        put: ScriptReference,
    },
}

/// Returns the index of the first host whose language equals `language`, as `find_script_host`
/// does with `wcscmp`.
pub fn find_script_host(languages: &[&[u16]], language: &[u16]) -> Option<usize> {
    languages.iter().position(|host| *host == language)
}

/// Looks `name` up on each host in turn and returns the first hit, as `lookup_script_properties`
/// does. `E_FAIL` means no host knows the name.
pub fn lookup_script_properties<F>(
    hosts: usize,
    get_dispid: F,
    name: &[u16],
) -> Result<ScriptReference, HRESULT>
where
    F: Fn(usize, &[u16]) -> Option<DispId>,
{
    (0..hosts)
        .find_map(|host| get_dispid(host, name).map(|id| ScriptReference { host, id }))
        .ok_or(E_FAIL)
}

/// Resolves every member of a factory against the script hosts, as `lookup_object_properties`
/// does. A property looks up `get_<name>` and then `put_<name>`. If the getter is missing, the
/// setter is not looked up. Any failure aborts the whole lookup, so the returned error is the
/// one from the first member that failed.
pub fn lookup_object_members<F>(
    hosts: usize,
    get_dispid: F,
    members: &[ScriptletMember],
) -> Result<Vec<ObjectMember>, HRESULT>
where
    F: Fn(usize, &[u16]) -> Option<DispId>,
{
    members
        .iter()
        .map(|member| match member.kind {
            MemberType::Method => {
                let script = lookup_script_properties(hosts, &get_dispid, &member.name)?;
                Ok(ObjectMember::Method {
                    name: member.name.clone(),
                    script,
                })
            }
            MemberType::Property => {
                let (get_name, put_name) = property_accessor_names(&member.name);
                let get = lookup_script_properties(hosts, &get_dispid, &get_name)?;
                let put = lookup_script_properties(hosts, &get_dispid, &put_name)?;
                Ok(ObjectMember::Property {
                    name: member.name.clone(),
                    get,
                    put,
                })
            }
        })
        .collect()
}

/// Returns the `get_` and `put_` accessor names for a property, as `lookup_object_properties`
/// builds them.
pub fn property_accessor_names(name: &[u16]) -> (Vec<u16>, Vec<u16>) {
    (accessor_name("get_", name), accessor_name("put_", name))
}

fn accessor_name(prefix: &str, name: &[u16]) -> Vec<u16> {
    prefix.encode_utf16().chain(name.iter().copied()).collect()
}
