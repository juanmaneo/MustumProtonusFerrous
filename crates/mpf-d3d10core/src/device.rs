//! `D3D10CoreCreateDevice` control flow.

use crate::{failed, Hresult, E_FAIL, S_OK};

/// `D3D10CoreCreateDevice`. `create_dxgi_device` stands for `DXGID3D10CreateDevice`, called
/// with the feature level as a one-element array. `query_id3d10_device` stands for the
/// `IUnknown_QueryInterface` call for `IID_ID3D10Device`, which runs only after creation succeeded.
pub fn d3d10_core_create_device(
    flags: u32,
    feature_level: u32,
    create_dxgi_device: impl FnOnce(u32, &[u32]) -> Hresult,
    query_id3d10_device: impl FnOnce() -> Hresult,
) -> Hresult {
    let hr = create_dxgi_device(flags, &[feature_level]);
    if failed(hr) {
        return hr;
    }
    if failed(query_id3d10_device()) {
        return E_FAIL;
    }
    S_OK
}
