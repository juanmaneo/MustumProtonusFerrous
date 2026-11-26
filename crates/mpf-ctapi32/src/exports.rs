//! Port of the `CT_init`, `CT_data` and `CT_close` exports in Wine's
//! `dlls/ctapi32/ctapi32.c`. Each forwards to its unix call and returns the
//! `NTSTATUS` narrowed to `IS8`.

use crate::unixlib::{
    unix_ct_close, unix_ct_data, unix_ct_init, CtCloseParams, CtDataParams, CtInitParams, CtLibrary,
};

pub fn ct_init<L: CtLibrary>(lib: &L, ctn: u16, pn: u16) -> i8 {
    unix_ct_init(lib, CtInitParams { ctn, pn }) as i8
}

pub fn ct_data<L: CtLibrary>(lib: &L, params: CtDataParams<L::Ptr>) -> i8 {
    unix_ct_data(lib, params) as i8
}

pub fn ct_close<L: CtLibrary>(lib: &L, ctn: u16) -> i8 {
    unix_ct_close(lib, CtCloseParams { ctn }) as i8
}
