//! The mountmgr query behind `DhcpRequestParams`: the layout of `struct mountmgr_dhcp_request_params`
//! (`include/ddk/mountmgr.h`), the `IOCTL_MOUNTMGR_QUERY_DHCP_REQUEST_PARAMS` code, and the reply
//! handling that `DhcpRequestParams` does after the ioctl.

use std::ops::Range;

use crate::api::{ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_DATA};

/// `FIELD_OFFSET(struct mountmgr_dhcp_request_params, params)`: `size`, `count`, `unix_name[16]`.
const HEADER_LEN: u64 = 24;
/// `sizeof(params[0])`: `{ id, offset, size }`.
const ENTRY_LEN: u64 = 12;
/// `FIELD_OFFSET(struct mountmgr_dhcp_request_params, unix_name)`.
const NAME_OFFSET: usize = 8;

/// `MOUNTMGRCONTROLTYPE`, the character `'m'`.
const MOUNTMGR_CONTROL_TYPE: u32 = 0x6d;
const METHOD_BUFFERED: u32 = 0;
/// `FILE_READ_ACCESS | FILE_WRITE_ACCESS`.
const FILE_READ_WRITE_ACCESS: u32 = 0x3;

/// A received option, as `DHCPAPI_PARAMS` reports it. `data` is the range of the caller's buffer
/// that holds the option's bytes.
#[derive(Debug, PartialEq, Eq)]
pub struct ReceivedParam {
    pub option_id: u32,
    /// Always `false`: the C code marks `IsVendor` as FIXME.
    pub is_vendor: bool,
    pub data: Option<Range<usize>>,
}

/// A successful reply: the received options, and the byte total that `*buflen` receives.
#[derive(Debug, PartialEq, Eq)]
pub struct Reply {
    pub params: Vec<ReceivedParam>,
    pub buflen: u32,
}

/// One `params[i]` entry as the driver writes it.
struct Entry {
    id: u32,
    offset: u32,
    size: u32,
}

/// `CTL_CODE(device, function, method, access)`.
fn ctl_code(device: u32, function: u32, method: u32, access: u32) -> u32 {
    (device << 16) | (access << 14) | (function << 2) | method
}

/// `IOCTL_MOUNTMGR_QUERY_DHCP_REQUEST_PARAMS`.
pub fn query_dhcp_request_params_ioctl() -> u32 {
    ctl_code(
        MOUNTMGR_CONTROL_TYPE,
        64,
        METHOD_BUFFERED,
        FILE_READ_WRITE_ACCESS,
    )
}

/// Byte offset of `params[index]` in the query.
pub fn params_offset(index: usize) -> u64 {
    HEADER_LEN + ENTRY_LEN * index as u64
}

/// Size of the query for `recv_count` options and a `buflen`-byte buffer, or `None` when that
/// does not fit the `DWORD` size that `DeviceIoControl` takes.
pub fn request_size(recv_count: usize, buflen: u32) -> Option<u32> {
    u32::try_from(params_offset(recv_count) + u64::from(buflen)).ok()
}

/// `*buflen` after `ERROR_MORE_DATA`: the driver's `size` minus the header and entries, with the
/// same 32-bit wrap as the C arithmetic.
pub fn more_data_buflen(size: u32, recv_count: usize) -> u32 {
    size.wrapping_sub(params_offset(recv_count) as u32)
}

/// The driver's `size` field of a query buffer.
pub fn query_size(query: &[u8]) -> u32 {
    get_u32(query, 0)
}

/// Writes `count`, the adapter name and the option ids into a zeroed query buffer. `name` must be
/// shorter than `IF_NAMESIZE`, so that the zeroed byte after it is its terminating NUL.
pub fn fill_request(query: &mut [u8], name: &[u8], recv_ids: &[u32]) {
    put_u32(query, 4, recv_ids.len() as u32);
    query[NAME_OFFSET..NAME_OFFSET + name.len()].copy_from_slice(name);
    for (i, &id) in recv_ids.iter().enumerate() {
        put_u32(query, params_offset(i) as usize, id);
    }
}

/// Checks the driver's reply to a query for `recv_count` options, and copies the received data
/// into `buf`. Without `buf` only the total is reported, as the C code does for a NULL buffer.
pub fn collect_reply(
    reply: &[u8],
    recv_count: usize,
    buf: Option<&mut [u8]>,
) -> Result<Reply, u32> {
    if (reply.len() as u64) < params_offset(recv_count) {
        return Err(ERROR_INVALID_DATA);
    }
    let count = get_u32(reply, 4) as usize;
    if count > recv_count {
        return Err(ERROR_INVALID_DATA);
    }
    let entries: Vec<Entry> = (0..count).map(|i| read_entry(reply, i)).collect();
    let total: u64 = entries.iter().map(|e| u64::from(e.size)).sum();
    let buflen = u32::try_from(total).map_err(|_| ERROR_INVALID_DATA)?;
    let Some(buf) = buf else {
        return Ok(Reply {
            params: Vec::new(),
            buflen,
        });
    };
    if total > buf.len() as u64 {
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    let sources = entries
        .iter()
        .map(|e| source_range(reply, e))
        .collect::<Result<Vec<_>, _>>()?;
    let mut params = Vec::with_capacity(entries.len());
    let mut dst = 0;
    for (entry, source) in entries.iter().zip(sources) {
        let data = source.map(|src| {
            let range = dst..dst + src.len();
            buf[range.clone()].copy_from_slice(&reply[src]);
            range
        });
        params.push(ReceivedParam {
            option_id: entry.id,
            is_vendor: false,
            data,
        });
        dst += entry.size as usize;
    }
    Ok(Reply { params, buflen })
}

/// Where `entry`'s data sits in the reply. An empty entry has no data, whatever its offset.
fn source_range(reply: &[u8], entry: &Entry) -> Result<Option<Range<usize>>, u32> {
    if entry.size == 0 {
        return Ok(None);
    }
    let end = u64::from(entry.offset) + u64::from(entry.size);
    if end > reply.len() as u64 {
        return Err(ERROR_INVALID_DATA);
    }
    Ok(Some(entry.offset as usize..end as usize))
}

fn read_entry(reply: &[u8], index: usize) -> Entry {
    let at = params_offset(index) as usize;
    Entry {
        id: get_u32(reply, at),
        offset: get_u32(reply, at + 4),
        size: get_u32(reply, at + 8),
    }
}

fn get_u32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn put_u32(bytes: &mut [u8], at: usize, value: u32) {
    bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
}
