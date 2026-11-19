//! The file header and the chunk walk of `parse_header` in `dlls/d3dxof/parsing.c`.
//!
//! The header is four fourccs: the magic `xof `, the version `0302` or `0303`, the format
//! (`bin `, `txt `, or the compressed `bzip` and `tzip`), and the float size `0032` or `0064`.
//! A compressed format then holds the little-endian size of the decompressed file, header
//! included, followed by pairs of little-endian decompressed and compressed chunk sizes, each
//! pair followed by its MSZIP chunk. The token lexer that reads the body is not ported.

use std::borrow::Cow;

use crate::mszip::decompress;

/// The size of the four-fourcc header. The declared size of a compressed file includes it.
const HEADER_SIZE: usize = 16;

/// A parsed file: whether its body is text, and the body, decompressed when its format is
/// compressed.
#[derive(Debug, PartialEq, Eq)]
pub struct Xof<'a> {
    pub txt: bool,
    pub body: Cow<'a, [u8]>,
}

/// The failures of [`parse_header`], with the values of Wine's `DXFILEERR_*` codes.
#[derive(Debug, PartialEq, Eq)]
pub enum HeaderError {
    /// `DXFILEERR_BADALLOC`: an MSZIP chunk does not inflate.
    BadAlloc,
    /// `DXFILEERR_BADFILETYPE`: a wrong magic or format, or a truncated or inconsistent file.
    BadFileType,
    /// `DXFILEERR_BADFILEVERSION`: a version other than `0302` or `0303`.
    BadFileVersion,
    /// `DXFILEERR_BADFILEFLOATSIZE`: a float size other than `0032` or `0064`.
    BadFileFloatSize,
}

impl HeaderError {
    /// The `HRESULT` that Wine returns: `MAKE_DDHRESULT` of the error code.
    pub fn hresult(&self) -> u32 {
        match self {
            Self::BadAlloc => 0x8876_0356,
            Self::BadFileType => 0x8876_035d,
            Self::BadFileVersion => 0x8876_035e,
            Self::BadFileFloatSize => 0x8876_035f,
        }
    }
}

/// Parses the header of `file`, and for `bzip` and `tzip` expands its chunks into the body.
pub fn parse_header(file: &[u8]) -> Result<Xof<'_>, HeaderError> {
    let (header, rest) = file
        .split_first_chunk::<HEADER_SIZE>()
        .ok_or(HeaderError::BadFileType)?;
    if &header[..4] != b"xof " {
        return Err(HeaderError::BadFileType);
    }
    if !matches!(&header[4..8], b"0302" | b"0303") {
        return Err(HeaderError::BadFileVersion);
    }
    let (txt, packed) = match &header[8..12] {
        b"bin " => (false, false),
        b"txt " => (true, false),
        b"bzip" => (false, true),
        b"tzip" => (true, true),
        _ => return Err(HeaderError::BadFileType),
    };
    if !matches!(&header[12..16], b"0032" | b"0064") {
        return Err(HeaderError::BadFileFloatSize);
    }
    let body = if packed {
        Cow::Owned(inflate_chunks(rest)?)
    } else {
        Cow::Borrowed(rest)
    };
    Ok(Xof { txt, body })
}

/// The chunk walk of `parse_header`: the declared size of the body, then each chunk with its
/// two sizes. The chunks must fill the declared size exactly.
fn inflate_chunks(rest: &[u8]) -> Result<Vec<u8>, HeaderError> {
    let (declared, mut chunks) = rest
        .split_first_chunk::<4>()
        .ok_or(HeaderError::BadFileType)?;
    let size = u32::from_le_bytes(*declared)
        .checked_sub(HEADER_SIZE as u32)
        .ok_or(HeaderError::BadFileType)? as usize;
    let mut body = Vec::new();
    while !chunks.is_empty() {
        let (sizes, after) = chunks
            .split_first_chunk::<4>()
            .ok_or(HeaderError::BadFileType)?;
        let decomp = usize::from(u16::from_le_bytes([sizes[0], sizes[1]]));
        let comp = usize::from(u16::from_le_bytes([sizes[2], sizes[3]]));
        let (packed, next) = after
            .split_at_checked(comp)
            .ok_or(HeaderError::BadFileType)?;
        let chunk = decompress(packed, decomp).map_err(|_| HeaderError::BadAlloc)?;
        if body.len() + chunk.len() > size {
            return Err(HeaderError::BadFileType);
        }
        body.extend_from_slice(&chunk);
        chunks = next;
    }
    if body.len() != size {
        return Err(HeaderError::BadFileType);
    }
    Ok(body)
}
