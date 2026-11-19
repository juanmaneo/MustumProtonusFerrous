use crate::build;
use crate::parse;
use crate::{
    DOT, WBEMPATH_INFO_ANON_LOCAL_MACHINE, WBEMPATH_INFO_CIM_COMPLIANT,
    WBEMPATH_INFO_HAS_IMPLIED_KEY, WBEMPATH_INFO_HAS_MACHINE_NAME, WBEMPATH_INFO_HAS_SUBSCOPES,
    WBEMPATH_INFO_IS_CLASS_REF, WBEMPATH_INFO_IS_INST_REF, WBEMPATH_INFO_PATH_HAD_SERVER,
    WBEMPATH_INFO_SERVER_NAMESPACE_ONLY, WBEMPATH_INFO_V1_COMPLIANT, WBEMPATH_INFO_V2_COMPLIANT,
    WBEM_E_INVALID_OBJECT_PATH, WBEM_E_INVALID_PARAMETER, WBEM_E_NOT_AVAILABLE,
};

const SERVER_FLAGS: u64 = WBEMPATH_INFO_PATH_HAD_SERVER
    | WBEMPATH_INFO_V1_COMPLIANT
    | WBEMPATH_INFO_V2_COMPLIANT
    | WBEMPATH_INFO_CIM_COMPLIANT;
const NAMESPACE_FLAGS: u64 =
    WBEMPATH_INFO_V1_COMPLIANT | WBEMPATH_INFO_V2_COMPLIANT | WBEMPATH_INFO_CIM_COMPLIANT;
const CLASS_FLAGS: u64 = WBEMPATH_INFO_V2_COMPLIANT | WBEMPATH_INFO_CIM_COMPLIANT;

/// A WBEM object path, the state behind Wine's IWbemPath.
///
/// Lengths are UTF-16 code units. The accessors that copy text follow the C
/// convention: `capacity` is the caller's buffer length, a call with
/// `capacity` no greater than the text only reports the length, and a larger
/// `capacity` needs `out`, which must hold at least `capacity` units.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObjectPath {
    pub(crate) text: Vec<u16>,
    pub(crate) server: Option<Vec<u16>>,
    pub(crate) namespaces: Vec<Vec<u16>>,
    pub(crate) class: Option<Vec<u16>>,
    pub(crate) keys: Vec<Key>,
    pub(crate) flags: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Key {
    pub(crate) name: Vec<u16>,
    pub(crate) value: Vec<u16>,
}

impl ObjectPath {
    /// Replaces the path with the parse of `text`, cut at the first NUL. Mode 0
    /// and a missing text are rejected before anything is cleared.
    pub fn set_text(&mut self, mode: u32, text: Option<&[u16]>) -> Result<(), i32> {
        if mode == 0 {
            return Err(WBEM_E_INVALID_PARAMETER);
        }
        let text = c_str(text.ok_or(WBEM_E_INVALID_PARAMETER)?);
        *self = ObjectPath::default();
        if text.is_empty() {
            return Ok(());
        }
        *self = parse::parse_text(text)?;
        self.text = text.to_vec();
        Ok(())
    }

    /// The text for `flags`, with the length to allocate (including the NUL).
    pub fn get_text(
        &self,
        flags: i32,
        capacity: usize,
        out: Option<&mut [u16]>,
    ) -> Result<usize, i32> {
        let (text, len) = build::build_path(self, flags);
        if capacity > len {
            let out = out.ok_or(WBEM_E_INVALID_PARAMETER)?;
            out[..text.len()].copy_from_slice(&text);
            out[text.len()] = 0;
        }
        Ok(len + 1)
    }

    pub fn get_info(&self, info: u32) -> Result<u64, i32> {
        if info != 0 {
            return Err(WBEM_E_INVALID_PARAMETER);
        }
        let mut flags = self.flags;
        flags |= match self.server.as_deref() {
            None | Some([DOT]) => WBEMPATH_INFO_ANON_LOCAL_MACHINE,
            Some(_) => WBEMPATH_INFO_HAS_MACHINE_NAME,
        };
        match self.class.as_ref() {
            None => flags |= WBEMPATH_INFO_SERVER_NAMESPACE_ONLY,
            Some(_) => {
                flags |= WBEMPATH_INFO_HAS_SUBSCOPES;
                if self.keys.is_empty() {
                    flags |= WBEMPATH_INFO_IS_CLASS_REF;
                } else {
                    if self.keys.iter().any(|key| key.name.is_empty()) {
                        flags |= WBEMPATH_INFO_HAS_IMPLIED_KEY;
                    }
                    flags |= WBEMPATH_INFO_IS_INST_REF;
                }
            }
        }
        Ok(flags)
    }

    /// `None` clears the server and its compliance flags.
    pub fn set_server(&mut self, name: Option<&[u16]>) {
        match name {
            Some(name) => {
                self.server = Some(c_str(name).to_vec());
                self.flags |= SERVER_FLAGS;
            }
            None => {
                self.server = None;
                self.flags &= !SERVER_FLAGS;
            }
        }
    }

    pub fn get_server(&self, capacity: usize, out: Option<&mut [u16]>) -> Result<usize, i32> {
        copy_out(self.server.as_deref(), WBEM_E_NOT_AVAILABLE, capacity, out)
    }

    pub fn get_namespace_count(&self) -> usize {
        self.namespaces.len()
    }

    /// Inserts `name` at `idx`, which may equal the current count.
    pub fn set_namespace_at(&mut self, idx: usize, name: Option<&[u16]>) -> Result<(), i32> {
        let name = name.ok_or(WBEM_E_INVALID_PARAMETER)?;
        if idx > self.namespaces.len() {
            return Err(WBEM_E_INVALID_PARAMETER);
        }
        self.namespaces.insert(idx, c_str(name).to_vec());
        self.flags |= NAMESPACE_FLAGS;
        Ok(())
    }

    pub fn get_namespace_at(
        &self,
        idx: usize,
        capacity: usize,
        out: Option<&mut [u16]>,
    ) -> Result<usize, i32> {
        copy_out(
            self.namespaces.get(idx).map(Vec::as_slice),
            WBEM_E_INVALID_PARAMETER,
            capacity,
            out,
        )
    }

    pub fn remove_namespace_at(&mut self, idx: usize) -> Result<(), i32> {
        if idx >= self.namespaces.len() {
            return Err(WBEM_E_INVALID_PARAMETER);
        }
        self.namespaces.remove(idx);
        Ok(())
    }

    pub fn remove_all_namespaces(&mut self) {
        self.namespaces.clear();
    }

    pub fn set_class_name(&mut self, name: Option<&[u16]>) -> Result<(), i32> {
        let name = name.ok_or(WBEM_E_INVALID_PARAMETER)?;
        self.class = Some(c_str(name).to_vec());
        self.flags |= CLASS_FLAGS;
        Ok(())
    }

    pub fn get_class_name(&self, capacity: usize, out: Option<&mut [u16]>) -> Result<usize, i32> {
        copy_out(
            self.class.as_deref(),
            WBEM_E_INVALID_OBJECT_PATH,
            capacity,
            out,
        )
    }

    pub fn get_key_count(&self) -> usize {
        self.keys.len()
    }

    /// Only flags 0 are accepted, as in IWbemPathKeyList::RemoveAllKeys.
    pub fn remove_all_keys(&mut self, flags: u32) -> Result<(), i32> {
        if flags != 0 {
            return Err(WBEM_E_INVALID_PARAMETER);
        }
        self.keys.clear();
        Ok(())
    }
}

fn c_str(text: &[u16]) -> &[u16] {
    match text.iter().position(|&c| c == 0) {
        Some(end) => &text[..end],
        None => text,
    }
}

/// The copy-out rule shared by the getters: a size query returns the length
/// plus the NUL, and the buffer is only written when `capacity` holds the text.
fn copy_out(
    value: Option<&[u16]>,
    missing: i32,
    capacity: usize,
    out: Option<&mut [u16]>,
) -> Result<usize, i32> {
    if capacity > 0 && out.is_none() {
        return Err(WBEM_E_INVALID_PARAMETER);
    }
    let value = value.ok_or(missing)?;
    if let Some(out) = out {
        if capacity > value.len() {
            out[..value.len()].copy_from_slice(value);
            out[value.len()] = 0;
        }
    }
    Ok(value.len() + 1)
}
