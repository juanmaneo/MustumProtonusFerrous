use crate::stub::{d3d12_get_interface, d3d12_sdk_version, StubException};

/// The `d3d12core.spec` export table, in spec order. Both entries are `@ stub`.
pub const EXPORTS: [&str; 2] = ["D3D12GetInterface", "D3D12SDKVersion"];

/// The record that calling the export `name` raises, or `None` when the spec does not export it.
/// Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    match name {
        "D3D12GetInterface" => Some(d3d12_get_interface()),
        "D3D12SDKVersion" => Some(d3d12_sdk_version()),
        _ => None,
    }
}
