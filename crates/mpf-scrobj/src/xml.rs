//! XML helpers from `dlls/scrobj/scrobj.c`: the name checks, the attribute check, and the
//! `MEMBER_PROPERTY` case of `parse_scriptlet_public`.
//!
//! The C code reads through `IXmlReader`. [`NodeReader`] replays a fixed list of nodes instead,
//! so the parser runs without COM.

use crate::com::{E_FAIL, E_NOTIMPL, HRESULT};

/// `PROP_GETTER` from `scrobj.c`: the property has a `get` element.
pub const PROP_GETTER: u32 = 1;

/// `PROP_SETTER` from `scrobj.c`: the property has a `put` element.
pub const PROP_SETTER: u32 = 2;

/// One node as `IXmlReader` reports it. `Element` carries the qualified name and whether the
/// element is empty (`<x/>`). `Other` stands for every other node type, such as text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlNode<'a> {
    Element { name: &'a [u16], empty: bool },
    EndElement,
    Whitespace,
    ProcessingInstruction,
    Other,
}

/// Stands in for the `IXmlReader` that `scrobj.c` reads from. [`next_node`](Self::next_node)
/// skips whitespace and processing instructions, as `next_xml_node` does.
#[derive(Clone, Debug)]
pub struct NodeReader<'a> {
    nodes: &'a [XmlNode<'a>],
    pos: usize,
}

impl<'a> NodeReader<'a> {
    pub fn new(nodes: &'a [XmlNode<'a>]) -> Self {
        Self { nodes, pos: 0 }
    }

    /// Returns the next node that is not whitespace or a processing instruction. Reading past
    /// the end fails with `E_FAIL`, which is what the C caller sees from a failed `Read`.
    pub fn next_node(&mut self) -> Result<XmlNode<'a>, HRESULT> {
        loop {
            let node = *self.nodes.get(self.pos).ok_or(E_FAIL)?;
            self.pos += 1;
            match node {
                XmlNode::Whitespace | XmlNode::ProcessingInstruction => continue,
                _ => return Ok(node),
            }
        }
    }

    /// Reads the next node and requires it to be an end element, as `expect_end_element` does.
    pub fn expect_end_element(&mut self) -> Result<(), HRESULT> {
        match self.next_node() {
            Ok(XmlNode::EndElement) => Ok(()),
            _ => Err(E_FAIL),
        }
    }
}

/// Returns whether the qualified name equals `name` exactly, as `is_xml_name` does. `None`
/// stands for a failed `GetQualifiedName`.
pub fn is_xml_name(qname: Option<&[u16]>, name: &str) -> bool {
    qname.is_some_and(|qname| qname.iter().copied().eq(name.encode_utf16()))
}

/// Returns whether the qualified name equals `name` ignoring case, as `is_case_xml_name` does.
/// The C code calls `_memicmp` on the raw bytes, so each byte folds only if it is an ASCII
/// letter. A non-ASCII code unit can therefore match an ASCII name when its low byte matches
/// after folding.
pub fn is_case_xml_name(qname: Option<&[u16]>, name: &str) -> bool {
    let name: Vec<u16> = name.encode_utf16().collect();
    qname.is_some_and(|qname| qname.len() == name.len() && memicmp(qname, &name))
}

fn memicmp(a: &[u16], b: &[u16]) -> bool {
    a.iter()
        .flat_map(|unit| unit.to_le_bytes())
        .zip(b.iter().flat_map(|unit| unit.to_le_bytes()))
        .all(|(x, y)| x.eq_ignore_ascii_case(&y))
}

/// Checks an element for attributes, as `expect_no_attributes` does. `attribute_count` is the
/// result of `GetAttributeCount`, where `Err` is a failed `HRESULT` that is passed through. Any
/// attribute is `E_FAIL`.
pub fn expect_no_attributes(attribute_count: Result<u32, HRESULT>) -> Result<(), HRESULT> {
    match attribute_count? {
        0 => Ok(()),
        _ => Err(E_FAIL),
    }
}

/// Parses the child elements of a `<property>` element, as the `MEMBER_PROPERTY` case of
/// `parse_scriptlet_public` does. `empty` is whether the property element itself is empty. The
/// result is `PROP_GETTER`, `PROP_SETTER` or both. A property with neither is `E_NOTIMPL`,
/// matching the C code.
pub fn parse_property_flags(empty: bool, reader: &mut NodeReader<'_>) -> Result<u32, HRESULT> {
    if empty {
        return Ok(0);
    }
    let mut flags = 0;
    loop {
        match reader.next_node()? {
            XmlNode::EndElement => break,
            XmlNode::Element {
                name,
                empty: element_empty,
            } => {
                if is_case_xml_name(Some(name), "get") {
                    if (flags & PROP_GETTER) != 0 {
                        return Err(E_FAIL);
                    }
                    flags |= PROP_GETTER;
                } else if is_case_xml_name(Some(name), "put") {
                    if (flags & PROP_SETTER) != 0 {
                        return Err(E_FAIL);
                    }
                    flags |= PROP_SETTER;
                } else {
                    return Err(E_FAIL);
                }
                if !element_empty {
                    reader.expect_end_element()?;
                }
            }
            _ => return Err(E_FAIL),
        }
    }
    if flags == 0 {
        return Err(E_NOTIMPL);
    }
    Ok(flags)
}
