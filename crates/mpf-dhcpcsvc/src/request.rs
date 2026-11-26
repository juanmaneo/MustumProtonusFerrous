//! `DhcpRequestParams`: argument checks, the mountmgr query, and how its result is reported.

use crate::adapter::{get_adapter_name, Os};
use crate::api::{ERROR_INVALID_PARAMETER, ERROR_MORE_DATA, ERROR_OUTOFMEMORY};
use crate::layout::{
    collect_reply, fill_request, more_data_buflen, query_dhcp_request_params_ioctl, query_size,
    request_size, ReceivedParam,
};

/// The mountmgr calls that `DhcpRequestParams` makes.
pub trait MountMgr {
    /// `CreateFileW(MOUNTMGR_DOS_DEVICE_NAME, ...)`. `Err` carries `GetLastError()`.
    fn open(&mut self) -> Result<(), u32>;
    /// `CloseHandle` on the handle that `open` returned.
    fn close(&mut self);
    /// `calloc(1, len)`. `None` when the allocation fails.
    fn alloc_zeroed(&mut self, len: usize) -> Option<Vec<u8>>;
    /// `DeviceIoControl` with `buf` as both input and output. The driver's reply is in `buf` even
    /// when the call fails with `ERROR_MORE_DATA`. `Err` carries `GetLastError()`.
    fn ioctl(&mut self, code: u32, buf: &mut [u8]) -> Result<(), u32>;
}

/// `DhcpRequestParams`. `adapter` and `buflen` are required, as in the C code. `buf` receives the
/// option bytes when present. On success `*buflen` holds the byte total, and on `ERROR_MORE_DATA`
/// it holds the size the caller needs.
pub fn request_params<O: Os + MountMgr>(
    os: &mut O,
    adapter: Option<&[u16]>,
    recv_ids: &[u32],
    buf: Option<&mut [u8]>,
    buflen: Option<&mut u32>,
) -> Result<Vec<ReceivedParam>, u32> {
    let (Some(adapter), Some(buflen)) = (adapter, buflen) else {
        return Err(ERROR_INVALID_PARAMETER);
    };
    let name = get_adapter_name(os, adapter)?;
    os.open()?;
    let result = query_params(os, &name, recv_ids, buflen, buf);
    os.close();
    result
}

/// The part of `DhcpRequestParams` that runs while the mountmgr handle is open.
fn query_params<O: MountMgr>(
    os: &mut O,
    name: &[u8],
    recv_ids: &[u32],
    buflen: &mut u32,
    buf: Option<&mut [u8]>,
) -> Result<Vec<ReceivedParam>, u32> {
    let size = request_size(recv_ids.len(), *buflen).ok_or(ERROR_INVALID_PARAMETER)?;
    let mut query = os.alloc_zeroed(size as usize).ok_or(ERROR_OUTOFMEMORY)?;
    fill_request(&mut query, name, recv_ids);
    if let Err(code) = os.ioctl(query_dhcp_request_params_ioctl(), &mut query) {
        if code == ERROR_MORE_DATA {
            *buflen = more_data_buflen(query_size(&query), recv_ids.len());
        }
        return Err(code);
    }
    let reply = collect_reply(&query, recv_ids.len(), buf)?;
    *buflen = reply.buflen;
    Ok(reply.params)
}
