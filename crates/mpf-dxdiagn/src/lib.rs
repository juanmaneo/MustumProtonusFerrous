//! Port of the portable logic in Wine's `dlls/dxdiagn`.
//!
//! dxdiagn is primarily a COM provider that queries WMI, the registry, and the
//! filesystem. The deterministic, OS-free logic ported here is:
//! - `depth_for_pixelformat`: D3DFORMAT -> bit depth (provider.c)
//! - `vendor_manufacturer`: PCI vendor ID -> manufacturer string (provider.c)
//! - `format_guid`: GUID -> canonical string (provider.c)
//! - `resolve_child_path`: dotted container path resolution (container.c)
//!
//! Everything else (COM class factories, refcounting, WMI/registry/file
//! queries, DirectX enumeration) needs OS resources and is skipped.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// D3DFORMAT values used by `depth_for_pixelformat` (d3d9types.h).
pub const D3DFMT_P8: u32 = 41;
pub const D3DFMT_X1R5G5B5: u32 = 24;
pub const D3DFMT_R5G6B5: u32 = 23;
pub const D3DFMT_X8R8G8B8: u32 = 22;

/// Resolves a `D3DFORMAT` to a bit depth (provider.c `depth_for_pixelformat`).
///
/// Unknown formats fall back to 32 bpp, matching Wine's default branch.
pub fn depth_for_pixelformat(format: u32) -> u32 {
    match format {
        D3DFMT_P8 => 8,
        D3DFMT_X1R5G5B5 => 15,
        D3DFMT_R5G6B5 => 16,
        D3DFMT_X8R8G8B8 => 32,
        _ => 32,
    }
}

/// Maps a PCI vendor ID to its manufacturer string (provider.c
/// `vendor_id_to_manufacturer_string`). Unknown IDs return `"Unknown"`.
pub fn vendor_manufacturer(vendor_id: u32) -> &'static str {
    match vendor_id {
        0x1002 => "ATI Technologies Inc.",
        0x10de => "NVIDIA",
        0x15ad => "VMware",
        0x1af4 => "Red Hat",
        0x8086 => "Intel Corporation",
        _ => "Unknown",
    }
}

/// Formats a GUID as `%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x`
/// with lowercase hex (provider.c `guid_to_string`).
pub fn format_guid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> String {
    format!(
        "{data1:08x}-{data2:04x}-{data3:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data4[0], data4[1], data4[2], data4[3], data4[4], data4[5], data4[6], data4[7],
    )
}

/// A node in the DXDiag container tree (container.c `IDxDiagContainerImpl_Container`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container {
    pub name: String,
    pub children: Vec<Container>,
}

/// Error returned by `resolve_child_path` (Wine returns `E_INVALIDARG`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    NotFound,
}

/// Finds a direct child by exact name (Wine compares with `lstrcmpW`).
fn find_child<'a>(cont: &'a Container, name: &str) -> Result<&'a Container, ContainerError> {
    cont.children
        .iter()
        .find(|child| child.name == name)
        .ok_or(ContainerError::NotFound)
}

/// Resolves a dotted container path against a container tree, mirroring
/// container.c `IDxDiagContainerImpl_GetChildContainer`.
///
/// - An empty path on a leaf container returns that container.
/// - Segments are separated by `.`; a single trailing `.` is ignored
///   (`"a.b."` resolves like `"a.b"`).
/// - Matching is exact and case-sensitive.
/// - An empty segment (e.g. `".."`, or a lone `"."`) is looked up as a child
///   named `""`, as Wine does.
pub fn resolve_child_path<'a>(
    cont: &'a Container,
    path: &str,
) -> Result<&'a Container, ContainerError> {
    if path.is_empty() && cont.children.is_empty() {
        return Ok(cont);
    }

    let mut current = cont;
    let mut remaining = path;
    while let Some(dot) = remaining.find('.') {
        let segment = &remaining[..dot];
        let rest = &remaining[dot + 1..];
        if rest.is_empty() {
            // Lone terminating period: the final segment is what precedes it.
            remaining = segment;
            break;
        }
        current = find_child(current, segment)?;
        remaining = rest;
    }

    find_child(current, remaining)
}
