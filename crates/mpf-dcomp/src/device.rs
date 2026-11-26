/// `E_NOTIMPL` from `winerror.h`.
pub const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;

/// `DCompositionCreateDevice`. The C body does not validate its arguments and does
/// not write `*device`, so the caller's value is left as it was.
pub fn dcomposition_create_device(
    _dxgi_device: Option<usize>,
    _iid: Option<&[u8; 16]>,
    _device: Option<&mut Option<usize>>,
) -> i32 {
    E_NOTIMPL
}

/// `DCompositionCreateDevice2`. Same behaviour as [`dcomposition_create_device`].
pub fn dcomposition_create_device2(
    _rendering_device: Option<usize>,
    _iid: Option<&[u8; 16]>,
    _device: Option<&mut Option<usize>>,
) -> i32 {
    E_NOTIMPL
}

/// `DCompositionCreateDevice3`. Same behaviour as [`dcomposition_create_device`].
pub fn dcomposition_create_device3(
    _rendering_device: Option<usize>,
    _iid: Option<&[u8; 16]>,
    _device: Option<&mut Option<usize>>,
) -> i32 {
    E_NOTIMPL
}
