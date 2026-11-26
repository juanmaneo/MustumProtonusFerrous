//! `DirectPlayNATHelpCreate` from `dlls/dpnhpast/main.c`.

use crate::{Hresult, E_NOTIMPL};
use std::ffi::c_void;

/// `DirectPlayNATHelpCreate(pIID, ppvInterface)`. The stub never reads the GUID
/// and never stores through `ppvInterface`, so both arguments are ignored.
/// `None` stands for a NULL pointer; a GUID is passed as its 16 bytes.
pub fn direct_play_nat_help_create(
    _iid: Option<&[u8; 16]>,
    _interface: Option<&mut *mut c_void>,
) -> Hresult {
    E_NOTIMPL
}
