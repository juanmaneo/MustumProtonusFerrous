//! Return values of the `dpnhupnp` exports, from `winerror.h` and `main.c`.

pub type Hresult = i32;

pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;

/// Stand-in for `GUID`. The stubs never read it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `DirectPlayNATHelpCreate(pIID, ppv)`: `FIXME` stub. `ppv` is left unwritten.
pub fn direct_play_nat_help_create(_piid: Option<&Guid>, _ppv: Option<&mut usize>) -> Hresult {
    E_NOTIMPL
}

/// `DllGetClassObject(rclsid, riid, ppv)`: `FIXME` stub. `ppv` is left unwritten.
pub fn dll_get_class_object(
    _rclsid: Option<&Guid>,
    _riid: Option<&Guid>,
    _ppv: Option<&mut usize>,
) -> Hresult {
    E_FAIL
}
