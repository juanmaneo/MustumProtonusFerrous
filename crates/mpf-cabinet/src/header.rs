//! The cabinet header of `fdi.c`: `FDI_read_entries` (the CFHEADER with its
//! reserved sizes and its prev and next names), `FDI_read_string` and
//! `FDIIsCabinet`.

use crate::Error;

const CFHEAD_SIZE: usize = 0x24;
const CFHEADEXT_SIZE: usize = 0x04;
const CAB_SIZE: usize = 0x08;
const MINOR_VERSION: usize = 0x18;
const MAJOR_VERSION: usize = 0x19;
const NUM_FOLDERS: usize = 0x1A;
const NUM_FILES: usize = 0x1C;
const FLAGS: usize = 0x1E;
const SET_ID: usize = 0x20;
const CABINET_INDEX: usize = 0x22;

const PREV_CABINET: u16 = 0x0001;
const NEXT_CABINET: u16 = 0x0002;
const RESERVE_PRESENT: u16 = 0x0004;

/// The reserved sizes that a cabinet declares when `RESERVE_PRESENT` is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reserve {
    /// Bytes to skip after the reserved-sizes block.
    pub header: u16,
    /// Reserved bytes in each folder.
    pub folder: u8,
    /// Reserved bytes in each data block.
    pub data: u8,
}

/// A previous or next cabinet name and its optional info string. The strings
/// exclude their NUL terminators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link<'a> {
    pub name: &'a [u8],
    pub info: Option<&'a [u8]>,
}

/// The header fields that `FDI_read_entries` keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CabinetHeader<'a> {
    pub cab_size: i32,
    pub folders: u16,
    pub files: u16,
    pub set_id: u16,
    pub index: u16,
    pub reserve: Option<Reserve>,
    pub prev: Option<Link<'a>>,
    pub next: Option<Link<'a>>,
}

/// `FDI_read_entries`: parses the CFHEADER at the start of `data`, then its
/// reserved sizes and its prev and next names when the flags announce them.
/// Format versions newer than 1.3 are rejected. Wine's failed seek past the
/// reserved header is not modeled: a slice has no seek to fail, so a skip
/// past the end only fails when the bytes that follow are read.
pub fn read_header(data: &[u8]) -> Result<CabinetHeader<'_>, Error> {
    let head = data.get(..CFHEAD_SIZE).ok_or(Error::NotACabinet)?;
    if &head[..4] != b"MSCF" {
        return Err(Error::NotACabinet);
    }
    let major = head[MAJOR_VERSION];
    if major > 1 || (major == 1 && head[MINOR_VERSION] > 3) {
        return Err(Error::UnknownVersion);
    }

    let flags = le_u16(head, FLAGS);
    let cab_size = le_i32(head, CAB_SIZE);
    let mut pos = CFHEAD_SIZE;

    let reserve = if (flags & RESERVE_PRESENT) != 0 {
        let ext = data
            .get(pos..pos + CFHEADEXT_SIZE)
            .ok_or(Error::CorruptCabinet)?;
        let header = le_u16(ext, 0);
        pos += CFHEADEXT_SIZE + usize::from(header);
        Some(Reserve {
            header,
            folder: ext[2],
            data: ext[3],
        })
    } else {
        None
    };

    let prev = if (flags & PREV_CABINET) != 0 {
        Some(read_link(data, &mut pos, cab_size)?)
    } else {
        None
    };
    let next = if (flags & NEXT_CABINET) != 0 {
        Some(read_link(data, &mut pos, cab_size)?)
    } else {
        None
    };

    Ok(CabinetHeader {
        cab_size,
        folders: le_u16(head, NUM_FOLDERS),
        files: le_u16(head, NUM_FILES),
        set_id: le_u16(head, SET_ID),
        index: le_u16(head, CABINET_INDEX),
        reserve,
        prev,
        next,
    })
}

/// `FDIIsCabinet`: [`read_header`] with the next cabinet cleared. The next
/// name is still parsed, so a corrupt one still fails.
pub fn is_cabinet(data: &[u8]) -> Result<CabinetHeader<'_>, Error> {
    let mut header = read_header(data)?;
    header.next = None;
    Ok(header)
}

/// A name, then its optional info string. A missing name is corruption.
fn read_link<'a>(data: &'a [u8], pos: &mut usize, cab_size: i32) -> Result<Link<'a>, Error> {
    let name = read_string(data, pos, cab_size).ok_or(Error::CorruptCabinet)?;
    let info = read_string(data, pos, cab_size);
    Ok(Link { name, info })
}

/// `FDI_read_string`: the bytes from `pos` up to the next NUL. The search stops
/// at `cab_size` when that is a non-negative offset past `pos`, and otherwise
/// runs to the end of `data`. On success `pos` moves past the NUL. On failure
/// it stays where it was.
fn read_string<'a>(data: &'a [u8], pos: &mut usize, cab_size: i32) -> Option<&'a [u8]> {
    let start = *pos;
    let end = match usize::try_from(cab_size) {
        Ok(size) if size >= start => size.min(data.len()),
        _ => data.len(),
    };
    let text = data.get(start..end)?;
    let nul = text.iter().position(|&b| b == 0)?;
    *pos = start + nul + 1;
    Some(&text[..nul])
}

fn le_u16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn le_i32(bytes: &[u8], at: usize) -> i32 {
    i32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}
