//! The NT process-information chain that WTSEnumerateProcessesExW walks.
//!
//! Each SYSTEM_PROCESS_INFORMATION entry (the __WINESRC__ layout, win64) starts with NextEntryOffset. Its image name is a UNICODE_STRING whose Buffer is an absolute pointer into the same buffer. parse_process_list follows the offsets as Wine does, and process_info_size returns the size Wine allocates for the result.

pub(crate) const NEXT_ENTRY_OFFSET: usize = 0;
pub(crate) const PROCESS_NAME_LENGTH: usize = 56;
pub(crate) const PROCESS_NAME_BUFFER: usize = 64;
pub(crate) const UNIQUE_PROCESS_ID: usize = 80;
pub(crate) const SESSION_ID: usize = 100;
/// The fixed part of an entry: the struct up to and including SessionId.
pub(crate) const HEADER_LEN: usize = 104;
/// sizeof(WTS_PROCESS_INFOW) on win64.
pub(crate) const WTS_PROCESS_INFOW_SIZE: usize = 24;
/// offsetof(SID, SubAuthority[SID_MAX_SUB_AUTHORITIES]): the per-process SID space.
pub(crate) const SID_RESERVE_SIZE: usize = 68;
pub(crate) const WCHAR_SIZE: usize = 2;

/// One process from the NT list, as WTSEnumerateProcessesExW reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessEntry {
    pub session_id: u32,
    /// UniqueProcessId truncated to 32 bits, as the DWORD ProcessId field.
    pub process_id: u32,
    /// The image name as UTF-16 code units, without the terminator.
    pub name: Vec<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// An entry header runs past the end of the buffer.
    Truncated,
    /// A name pointer, or the name it covers, lies outside the buffer.
    NameOutOfRange,
    /// The name's byte length is odd. Wine would store the terminator misaligned.
    OddNameLength,
}

/// Walks the chain in buf, whose first byte is at address, and returns each process in order.
pub fn parse_process_list(buf: &[u8], address: u64) -> Result<Vec<ProcessEntry>, ParseError> {
    let mut entries = Vec::new();
    let mut offset = 0;
    loop {
        let header = buf
            .get(offset..offset + HEADER_LEN)
            .ok_or(ParseError::Truncated)?;
        let next = u32::from_le_bytes(field(header, NEXT_ENTRY_OFFSET));
        let name_len = usize::from(u16::from_le_bytes(field(header, PROCESS_NAME_LENGTH)));
        let name_ptr = u64::from_le_bytes(field(header, PROCESS_NAME_BUFFER));
        let process_id = u64::from_le_bytes(field(header, UNIQUE_PROCESS_ID)) as u32;
        let session_id = u32::from_le_bytes(field(header, SESSION_ID));
        entries.push(ProcessEntry {
            session_id,
            process_id,
            name: read_name(buf, address, name_ptr, name_len)?,
        });
        if next == 0 {
            return Ok(entries);
        }
        offset += next as usize;
    }
}

/// The size WTSEnumerateProcessesExW allocates for entries: each WTS_PROCESS_INFOW, its name with terminator, and its SID space.
pub fn process_info_size(entries: &[ProcessEntry]) -> usize {
    entries
        .iter()
        .map(|entry| {
            WTS_PROCESS_INFOW_SIZE + (entry.name.len() + 1) * WCHAR_SIZE + SID_RESERVE_SIZE
        })
        .sum()
}

/// Reads the len-byte name at ptr. A zero-length name is empty whatever ptr holds, since Wine's memcpy copies nothing.
fn read_name(buf: &[u8], address: u64, ptr: u64, len: usize) -> Result<Vec<u16>, ParseError> {
    if !len.is_multiple_of(WCHAR_SIZE) {
        return Err(ParseError::OddNameLength);
    }
    if len == 0 {
        return Ok(Vec::new());
    }
    let start = ptr.checked_sub(address).ok_or(ParseError::NameOutOfRange)?;
    let end = start.saturating_add(len as u64);
    if end > buf.len() as u64 {
        return Err(ParseError::NameOutOfRange);
    }
    let bytes = &buf[start as usize..end as usize];
    let (units, _) = bytes.as_chunks::<WCHAR_SIZE>();
    Ok(units.iter().map(|unit| u16::from_le_bytes(*unit)).collect())
}

/// The N bytes of bytes starting at at.
fn field<const N: usize>(bytes: &[u8], at: usize) -> [u8; N] {
    core::array::from_fn(|i| bytes[at + i])
}
