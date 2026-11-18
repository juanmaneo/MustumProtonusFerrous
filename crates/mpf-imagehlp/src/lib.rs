//! Port of the byte-level logic in Wine's `dlls/imagehlp` (`integrity.c` and
//! `modify.c`) at commit `9f8c4ae4d35c7ff4153267aba9618c026f5f5583`.
//!
//! `imagehlp.spec` exports 109 functions: 74 forwarded to `dbghelp` (`-import`),
//! 9 `@ stub` entries and 26 named `stdcall` exports. This crate ports 4 of the
//! 26 named exports. Each one reads a byte slice supplied by the caller, in place
//! of a file handle or a mapped image, so none of them needs file or loader state.
//!
//! Ported (4 of 109):
//! - `CheckSumMappedFile`: [`check_sum_mapped_file`].
//! - `ImageEnumerateCertificates`: [`enumerate_certificates`].
//! - `ImageGetCertificateData`: [`certificate_data`].
//! - `ImageGetCertificateHeader`: [`certificate_header`].
//!
//! Skipped (105 of 109):
//! - 74 `-import` entries: forwarded to `dbghelp`, so `dlls/imagehlp` has no body for them.
//! - 9 `@ stub` entries with no body in the C source: `FindFileInPath`, `FindFileInSearchPath`, `MarkImageAsRunFromSwap`, `RemovePrivateCvSymbolicEx`, `SymEnumSym`, `SymEnumerateSymbolsW64`, `SymEnumerateSymbolsW`, `SymGetLineFromName64`, `SymGetLineFromName`.
//! - 10 `FIXME` stubs that return `ERROR_CALL_NOT_IMPLEMENTED`: `GetImageConfigInformation`, `GetImageUnusedHeaderBytes`, `SetImageConfigInformation`, `RemovePrivateCvSymbolic`, `RemoveRelocations`, `SplitSymbols`, `ReBaseImage`, `ReBaseImage64`, `UpdateDebugInfoFile`, `UpdateDebugInfoFileEx`. There is no logic to port.
//! - 12 file, mapping or image-state exports: `ImageAddCertificate`, `ImageRemoveCertificate`, `ImageGetDigestStream`, `ImageLoad`, `ImageUnload`, `MapAndLoad`, `UnMapAndLoad`, `MapFileAndCheckSumA`, `MapFileAndCheckSumW`, `BindImage`, `BindImageEx`, `TouchFileTimes`. They open, write, map or load files, and this crate has no file or loader layer.
//!
//! Divergences from C:
//! - Buffers are slices, and a NULL pointer is `None`. A buffer shorter than the data is an error, where C would overrun it.
//! - A read past the end of the image fails. The exception is the byte just past an odd-length checksum range: if the slice ends there, it reads as zero, like the zero-filled tail of a mapped page.
//! - [`check_sum_mapped_file`] returns `None` when `file_length` exceeds the slice, when the image has no NT headers (Wine returns NULL and writes neither output), or when the checksum field lies past the end of the slice.
//! - [`enumerate_certificates`] stores indices before it fails, as C does, and skips slots past the end of `indices`.
//! - Win32 last-error values are not modelled. [`CertificateError`] keeps the two codes a caller can act on, `ERROR_INVALID_PARAMETER` and `ERROR_INSUFFICIENT_BUFFER`, and `Failed` covers every other FALSE return.
//! - [`certificate_header`] returns the parsed fields instead of writing a `WIN_CERTIFICATE`. A short read returns `None`, where C may leave partial bytes in the output.

/// `CERT_SECTION_TYPE_ANY`: the `filter` for [`enumerate_certificates`] that matches every certificate type.
pub const CERT_SECTION_TYPE_ANY: u16 = 0xFF;

/// Bytes of a certificate that the C readers copy: `sizeof(WIN_CERTIFICATE) - sizeof(bCertificate)`, which is 11 on Win32.
const CERT_HEADER_LEN: u32 = 11;
/// The `PE\0\0` signature that starts the NT headers.
const PE_SIGNATURE: &[u8; 4] = b"PE\0\0";
/// Offset of `OptionalHeader.CheckSum` from the start of the NT headers.
const CHECKSUM_OFFSET: usize = 88;
/// Optional header magic for PE32 images.
const OPTIONAL_HEADER32_MAGIC: u16 = 0x10B;
/// Optional header magic for PE32+ images.
const OPTIONAL_HEADER64_MAGIC: u16 = 0x20B;

/// The result of [`check_sum_mapped_file`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MappedChecksum {
    /// Offset of the NT headers (`e_lfanew`) in the image.
    pub nt_offset: u32,
    /// The `CheckSum` value stored in the optional header.
    pub header_sum: u32,
    /// The checksum of the first `file_length` bytes.
    pub check_sum: u32,
}

/// Why a certificate reader failed. The first two variants are the Win32 errors that Wine sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateError {
    /// A required argument is missing (`ERROR_INVALID_PARAMETER`).
    InvalidParameter,
    /// The buffer cannot hold the certificate (`ERROR_INSUFFICIENT_BUFFER`).
    InsufficientBuffer,
    /// Any other FALSE return from Wine. Its last-error value is not modelled.
    Failed,
}

/// The fixed part of a certificate, as [`certificate_header`] reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CertificateHeader {
    /// `dwLength`: the certificate length, header included.
    pub length: u32,
    /// `wRevision`.
    pub revision: u16,
    /// `wCertificateType`.
    pub certificate_type: u16,
    /// The first three bytes of `bCertificate`.
    pub body_prefix: [u8; 3],
}

/// Ports `CheckSumMappedFile`: the checksum of the first `file_length` bytes of `image`.
///
/// The header's stored `CheckSum` is folded out of the sum, so the result does not depend on it.
pub fn check_sum_mapped_file(image: &[u8], file_length: u32) -> Option<MappedChecksum> {
    let length = file_length as usize;
    if length > image.len() {
        return None;
    }
    let nt = nt_header_offset(image)?;
    let header_sum = le32(window(image, nt + CHECKSUM_OFFSET, 4)?);
    let sum = subtract_half(word_sum(image, length), header_sum & 0xFFFF);
    let sum = subtract_half(sum, header_sum >> 16);
    Some(MappedChecksum {
        nt_offset: nt as u32,
        header_sum,
        check_sum: sum.wrapping_add(file_length),
    })
}

/// Ports `ImageEnumerateCertificates`: counts the certificates whose type matches `filter`.
///
/// Use [`CERT_SECTION_TYPE_ANY`] to match every type. The 0-based table index of
/// each match is stored in `indices`, in order, up to the slice's length; pass an
/// empty slice to count only. Returns `None` if the table cannot be walked. Indices
/// stored before that point stay stored, as in C.
pub fn enumerate_certificates(image: &[u8], filter: u16, indices: &mut [u32]) -> Option<u32> {
    let (sd_va, size) = security_directory(image)?;
    let mut count = 0u32;
    let mut index = 0u32;
    let mut offset = 0u32;
    while offset < size {
        let head = window(
            image,
            seek(sd_va.wrapping_add(offset))?,
            CERT_HEADER_LEN as usize,
        )?;
        let length = le32(head);
        if length < CERT_HEADER_LEN || length > size - offset {
            return None;
        }
        if filter == CERT_SECTION_TYPE_ANY || filter == le16(&head[6..]) {
            if let Some(slot) = indices.get_mut(count as usize) {
                *slot = index;
            }
            count += 1;
        }
        offset = next_offset(offset, length);
        index += 1;
    }
    Some(count)
}

/// Ports `ImageGetCertificateData`: copies certificate `index` into `buffer`.
///
/// `required` is set to the certificate's length whenever the certificate is found.
/// The length check runs before the buffer check, so a `required` that is too small is
/// reported even with no buffer, as in C. A `buffer` shorter than the certificate
/// returns [`CertificateError::InsufficientBuffer`] without writing, where C would
/// overrun it. If the image ends before the certificate does, the bytes that exist are
/// copied and [`CertificateError::Failed`] is returned.
pub fn certificate_data(
    image: &[u8],
    index: u32,
    buffer: Option<&mut [u8]>,
    required: Option<&mut u32>,
) -> Result<(), CertificateError> {
    let required = required.ok_or(CertificateError::InvalidParameter)?;
    let (position, size) = certificate_span(image, index).ok_or(CertificateError::Failed)?;
    if *required < size {
        *required = size;
        return Err(CertificateError::InsufficientBuffer);
    }
    let buffer = buffer.ok_or(CertificateError::InvalidParameter)?;
    *required = size;
    let len = size as usize;
    if buffer.len() < len {
        return Err(CertificateError::InsufficientBuffer);
    }
    let available = image.get(position..).unwrap_or_default();
    let copied = available.len().min(len);
    buffer[..copied].copy_from_slice(&available[..copied]);
    if copied == len {
        Ok(())
    } else {
        Err(CertificateError::Failed)
    }
}

/// Ports `ImageGetCertificateHeader`: reads the header of certificate `index`.
///
/// Returns `None` if there is no such certificate, or if it is too short to hold a header.
pub fn certificate_header(image: &[u8], index: u32) -> Option<CertificateHeader> {
    let (position, size) = certificate_span(image, index)?;
    if size < CERT_HEADER_LEN {
        return None;
    }
    let head = window(image, position, CERT_HEADER_LEN as usize)?;
    Some(CertificateHeader {
        length: le32(head),
        revision: le16(&head[4..]),
        certificate_type: le16(&head[6..]),
        body_prefix: [head[8], head[9], head[10]],
    })
}

/// The bytes `image[start..start + len]`, or `None` if any of them are missing.
fn window(image: &[u8], start: usize, len: usize) -> Option<&[u8]> {
    image.get(start..)?.get(..len)
}

/// The little-endian `u16` at the start of `bytes`.
fn le16(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]])
}

/// The little-endian `u32` at the start of `bytes`.
fn le32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// Converts a file position to an index into the image. `SetFilePointer` reads the
/// position as signed, so a position of 2^31 or more is a negative seek, which fails.
fn seek(position: u32) -> Option<usize> {
    usize::try_from(position as i32).ok()
}

/// Finds the NT headers the way `RtlImageNtHeader` does: the DOS header starts with
/// `MZ`, `e_lfanew` points at the `PE\0\0` signature. The optional header's magic is
/// not checked here.
fn nt_header_offset(image: &[u8]) -> Option<usize> {
    let dos = window(image, 0, 64)?;
    if &dos[..2] != b"MZ" {
        return None;
    }
    let nt = le32(&dos[60..]) as usize;
    (window(image, nt, 4)? == PE_SIGNATURE).then_some(nt)
}

/// The `(VirtualAddress, Size)` of the security data directory, read as
/// `GetNTHeaders` does. A PE32+ image must have all 264 bytes of its NT headers, a
/// PE32 image the first 248.
fn security_directory(image: &[u8]) -> Option<(u32, u32)> {
    let nt = nt_header_offset(image)?;
    let head = window(image, nt, 248)?;
    // The security entry's offset in the NT headers, and the NT header length for that format.
    let (entry, headers_len) = match le16(&head[24..]) {
        OPTIONAL_HEADER32_MAGIC => (152, 248),
        OPTIONAL_HEADER64_MAGIC => (168, 264),
        _ => return None,
    };
    window(image, nt, headers_len)?;
    Some((le32(&head[entry..]), le32(&head[entry + 4..])))
}

/// The `(position, length)` of certificate `index`, where `position` is the offset
/// of its `dwLength` in the image. Mirrors `IMAGEHLP_GetCertificateOffset`.
fn certificate_span(image: &[u8], index: u32) -> Option<(usize, u32)> {
    let (sd_va, size) = security_directory(image)?;
    let mut remaining = index;
    let mut offset = 0u32;
    loop {
        let position = seek(sd_va.wrapping_add(offset))?;
        let length = le32(window(image, position, 4)?);
        if length < 4 || length > size - offset {
            return None;
        }
        if remaining == 0 {
            return Some((position, length));
        }
        remaining -= 1;
        offset = next_offset(offset, length);
        if offset >= size {
            return None;
        }
    }
}

/// The offset of the certificate after the one at `offset`: its length, padded to a
/// multiple of 8, is added to `offset`. Wraps like the 32-bit C arithmetic.
fn next_offset(offset: u32, length: u32) -> u32 {
    offset
        .wrapping_add(length)
        .wrapping_add((8 - length % 8) % 8)
}

/// One step of the fold in `CheckSumMappedFile`: removes `half`, a 16-bit part of the
/// header checksum, from the 16-bit `sum`. When `half` is larger, the borrow wraps as
/// one's complement, which is `((sum - half) & 0xFFFF) - 1` in C.
fn subtract_half(sum: u32, half: u32) -> u32 {
    if sum >= half {
        sum - half
    } else {
        sum + 0xFFFF - half
    }
}

/// `CalcCheckSum`: the one's-complement sum of the little-endian words of
/// `image[..length]`. For an odd `length`, the last word also reads the byte at
/// `length`, which is zero if the slice ends there.
fn word_sum(image: &[u8], length: usize) -> u32 {
    let mut sum = 0u32;
    for word in 0..length.div_ceil(2) {
        let at = 2 * word;
        let lo = image.get(at).copied().unwrap_or(0);
        let hi = image.get(at + 1).copied().unwrap_or(0);
        sum += u32::from(u16::from_le_bytes([lo, hi]));
        // End-around carry. The sum stays at or below 0xFFFF, so one fold per word is exact.
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    sum
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod tests {
    use super::*;

    /// A PE32 image of `len` zero bytes, with `MZ`, `e_lfanew` = 0x40, the `PE\0\0`
    /// signature and the PE32 magic.
    fn pe_image(len: usize) -> Vec<u8> {
        let mut image = vec![0; len];
        image[..2].copy_from_slice(b"MZ");
        image[60..64].copy_from_slice(&0x40u32.to_le_bytes());
        image[0x40..0x44].copy_from_slice(PE_SIGNATURE);
        image[0x58..0x5A].copy_from_slice(&OPTIONAL_HEADER32_MAGIC.to_le_bytes());
        image
    }

    /// A PE32 image whose security directory lists one certificate of `length` bytes at
    /// 0x140, of type 2 and revision 0x200.
    fn signed_image(length: u32) -> Vec<u8> {
        let mut image = pe_image(0x140 + length as usize);
        image[0xD8..0xDC].copy_from_slice(&0x140u32.to_le_bytes());
        image[0xDC..0xE0].copy_from_slice(&length.to_le_bytes());
        image[0x140..0x144].copy_from_slice(&length.to_le_bytes());
        image[0x144..0x146].copy_from_slice(&0x200u16.to_le_bytes());
        image[0x146..0x148].copy_from_slice(&2u16.to_le_bytes());
        image
    }

    #[test]
    fn checksum_needs_the_whole_image_and_the_checksum_field() {
        assert!(check_sum_mapped_file(&pe_image(160), 160).is_some());
        assert_eq!(check_sum_mapped_file(&pe_image(160), 161), None);
        assert_eq!(check_sum_mapped_file(&pe_image(155), 155), None);
        assert!(check_sum_mapped_file(&pe_image(156), 156).is_some());
    }

    #[test]
    fn truncated_or_unmapped_headers_are_none() {
        assert_eq!(check_sum_mapped_file(b"MZ\0\0", 4), None);
        assert_eq!(check_sum_mapped_file(&[], 0), None);
        assert_eq!(certificate_header(b"MZ\0\0", 0), None);
        assert_eq!(
            enumerate_certificates(b"MZ\0\0", CERT_SECTION_TYPE_ANY, &mut []),
            None
        );
    }

    #[test]
    fn nt_offset_outside_the_image_is_none() {
        for lfanew in [0x1000u32, 0x8000_0040] {
            let mut image = pe_image(100);
            image[60..64].copy_from_slice(&lfanew.to_le_bytes());
            assert_eq!(check_sum_mapped_file(&image, 100), None);
            assert_eq!(
                enumerate_certificates(&image, CERT_SECTION_TYPE_ANY, &mut []),
                None
            );
            assert_eq!(certificate_header(&image, 0), None);
        }
    }

    #[test]
    fn short_buffer_is_insufficient_even_when_required_allows_it() {
        let image = signed_image(24);
        let mut buffer = [0xAA; 23];
        let mut required = 24;
        assert_eq!(
            certificate_data(&image, 0, Some(&mut buffer), Some(&mut required)),
            Err(CertificateError::InsufficientBuffer)
        );
        assert_eq!(required, 24);
        assert_eq!(buffer, [0xAA; 23]);
    }

    #[test]
    fn signed_image_round_trips_through_every_reader() {
        let image = signed_image(24);
        let header = certificate_header(&image, 0).expect("certificate header");
        assert_eq!(
            header,
            CertificateHeader {
                length: 24,
                revision: 0x200,
                certificate_type: 2,
                body_prefix: [0, 0, 0],
            }
        );
        assert_eq!(enumerate_certificates(&image, 2, &mut [0]), Some(1));
        let mut buffer = [0; 24];
        let mut required = 24;
        assert_eq!(
            certificate_data(&image, 0, Some(&mut buffer), Some(&mut required)),
            Ok(())
        );
        assert_eq!(buffer[..4], image[0x140..0x144]);
    }

    #[test]
    fn result_types_implement_their_traits() {
        let sum = MappedChecksum {
            nt_offset: 0x40,
            header_sum: 1,
            check_sum: 2,
        };
        assert_eq!(Clone::clone(&sum), sum);
        assert_eq!(
            format!("{sum:?}"),
            "MappedChecksum { nt_offset: 64, header_sum: 1, check_sum: 2 }"
        );

        let header = CertificateHeader {
            length: 24,
            revision: 0x200,
            certificate_type: 2,
            body_prefix: [1, 2, 3],
        };
        assert_eq!(Clone::clone(&header), header);
        assert_eq!(
            format!("{header:?}"),
            "CertificateHeader { length: 24, revision: 512, certificate_type: 2, body_prefix: [1, 2, 3] }"
        );

        let errors = [
            (CertificateError::InvalidParameter, "InvalidParameter"),
            (CertificateError::InsufficientBuffer, "InsufficientBuffer"),
            (CertificateError::Failed, "Failed"),
        ];
        for (error, name) in errors {
            assert_eq!(Clone::clone(&error), error);
            assert_eq!(format!("{error:?}"), name);
        }
        assert_ne!(CertificateError::Failed, CertificateError::InvalidParameter);
    }
}
