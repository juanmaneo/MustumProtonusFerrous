//! Port of the unix-side call table in Wine's `dlls/ctapi32/unixlib.c` and
//! `unixlib.h`: slot order, the WoW64 `ct_data` entry, and the `attach`/`detach`
//! status codes.

/// Windows `NTSTATUS`.
pub type NtStatus = i32;

pub const STATUS_SUCCESS: NtStatus = 0;
pub const STATUS_DLL_NOT_FOUND: NtStatus = 0xC000_0135_u32 as NtStatus;
pub const STATUS_ENTRYPOINT_NOT_FOUND: NtStatus = 0xC000_0139_u32 as NtStatus;

/// Number of slots in the unix call table (`unix_funcs_count`).
pub const UNIX_FUNCS_COUNT: usize = 5;

/// Slot index of each entry in the unix call table (`enum ctapi_funcs`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum UnixFunc {
    Attach,
    Detach,
    CtInit,
    CtData,
    CtClose,
}

/// Function stored in a table slot. The WoW64 table differs only in `CtData`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entry {
    Attach,
    Detach,
    CtInit,
    CtData,
    Wow64CtData,
    CtClose,
}

/// The function stored at `slot` in `__wine_unix_call_funcs` (`wow64 == false`) or
/// `__wine_unix_call_wow64_funcs` (`wow64 == true`).
pub fn entry(wow64: bool, slot: UnixFunc) -> Entry {
    match (wow64, slot) {
        (_, UnixFunc::Attach) => Entry::Attach,
        (_, UnixFunc::Detach) => Entry::Detach,
        (_, UnixFunc::CtInit) => Entry::CtInit,
        (false, UnixFunc::CtData) => Entry::CtData,
        (true, UnixFunc::CtData) => Entry::Wow64CtData,
        (_, UnixFunc::CtClose) => Entry::CtClose,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CtInitParams {
    pub ctn: u16,
    pub pn: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CtDataParams<Ptr> {
    pub ctn: u16,
    pub dad: Ptr,
    pub sad: Ptr,
    pub lenc: u16,
    pub command: Ptr,
    pub lenr: Ptr,
    pub response: Ptr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CtCloseParams {
    pub ctn: u16,
}

/// The `ct_data` arguments as a 32-bit WoW64 caller passes them (`PTR32` fields).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wow64CtDataParams {
    pub ctn: u16,
    pub dad: u32,
    pub sad: u32,
    pub lenc: u16,
    pub command: u32,
    pub lenr: u32,
    pub response: u32,
}

/// The low-level CT-API library that `attach` resolves and the unix calls forward to.
pub trait CtLibrary {
    type Ptr: Copy;

    fn ct_init(&self, params: CtInitParams) -> i8;
    fn ct_data(&self, params: CtDataParams<Self::Ptr>) -> i8;
    fn ct_close(&self, params: CtCloseParams) -> i8;
}

/// Port of `attach`. `handle` is the `dlopen` result (`None` if it failed) and the
/// closure answers `dlsym` for a symbol name.
pub fn attach<F: FnMut(&str) -> bool>(handle: Option<F>) -> NtStatus {
    let Some(mut dlsym) = handle else {
        return STATUS_DLL_NOT_FOUND;
    };
    for symbol in ["CT_init", "CT_data", "CT_close"] {
        if !dlsym(symbol) {
            return STATUS_ENTRYPOINT_NOT_FOUND;
        }
    }
    STATUS_SUCCESS
}

/// Port of `detach`. The `dlclose` call has no portable effect and is not modelled.
pub fn detach() -> NtStatus {
    STATUS_SUCCESS
}

pub fn unix_ct_init<L: CtLibrary>(lib: &L, params: CtInitParams) -> NtStatus {
    NtStatus::from(lib.ct_init(params))
}

pub fn unix_ct_data<L: CtLibrary>(lib: &L, params: CtDataParams<L::Ptr>) -> NtStatus {
    NtStatus::from(lib.ct_data(params))
}

pub fn unix_ct_close<L: CtLibrary>(lib: &L, params: CtCloseParams) -> NtStatus {
    NtStatus::from(lib.ct_close(params))
}

/// Port of `wow64_ct_data`: widens each 32-bit pointer with `ULongToPtr`, then forwards
/// to the native `ct_data`.
pub fn wow64_ct_data<L>(lib: &L, params: Wow64CtDataParams) -> NtStatus
where
    L: CtLibrary,
    L::Ptr: From<u32>,
{
    unix_ct_data(
        lib,
        CtDataParams {
            ctn: params.ctn,
            dad: L::Ptr::from(params.dad),
            sad: L::Ptr::from(params.sad),
            lenc: params.lenc,
            command: L::Ptr::from(params.command),
            lenr: L::Ptr::from(params.lenr),
            response: L::Ptr::from(params.response),
        },
    )
}
