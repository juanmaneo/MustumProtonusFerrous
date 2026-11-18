//! version subset for MPF: the headless logic of `dlls/version/version.c`.
//!
//! Reference Wine's `dlls/version` implements only `VerInstallFileA/W`; the
//! other exports forward to `dlls/kernelbase`. This crate ports the pure
//! decision helpers that `VerInstallFileA` applies to its inputs. They work on
//! byte buffers and slices, with no file, registry, or heap access. Paths are
//! ANSI byte slices without a trailing NUL.
//!
//! Ported (pure logic, verified against the reference by table tests):
//! - `error_to_vif` (`_error2vif`): maps a Win32 error to a `VIF_*` bit.
//! - `lz_copy_error_to_vif`: the `LZCopy` error switch in `VerInstallFileA`.
//! - `fixed_file_info` (`_fetch_versioninfo`, locate step): reads the
//!   `VS_FIXEDFILEINFO` from a version block, including the UTF-16 marker check.
//! - `compare_file_versions`: the version and type mismatch bits of `VerInstallFileA`.
//! - `temp_file_tail` (the `strrchr` step): the tail of a temp path, from its last backslash.
//! - `temp_file_outcome` (the `tmpfilelen` check): the buffer-size result for the temp name.
//!
//! Partial: `VerInstallFileA` and `VerInstallFileW` are not exported as functions
//! here. What remains of them is file orchestration. A caller would combine the
//! helpers above into the `xret` value.
//!
//! Skipped exports (none is fully ported; reason per line):
//! - `GetFileVersionInfoA`: reads version resources from a file; in `dlls/kernelbase`.
//! - `GetFileVersionInfoW`: reads version resources from a file; in `dlls/kernelbase`.
//! - `GetFileVersionInfoExA`: reads version resources from a file; in `dlls/kernelbase`.
//! - `GetFileVersionInfoExW`: reads version resources from a file; in `dlls/kernelbase`.
//! - `GetFileVersionInfoSizeA`: queries a file on disk; in `dlls/kernelbase`.
//! - `GetFileVersionInfoSizeW`: queries a file on disk; in `dlls/kernelbase`.
//! - `GetFileVersionInfoSizeExA`: queries a file on disk; in `dlls/kernelbase`.
//! - `GetFileVersionInfoSizeExW`: queries a file on disk; in `dlls/kernelbase`.
//! - `VerFindFileA`: searches directories for a file; in `dlls/kernelbase`.
//! - `VerFindFileW`: searches directories for a file; in `dlls/kernelbase`.
//! - `VerLanguageNameA`: needs locale data; in `dlls/kernelbase/locale.c`.
//! - `VerLanguageNameW`: needs locale data; in `dlls/kernelbase/locale.c`.
//! - `VerQueryValueA`: parses a version block; pure, but in `dlls/kernelbase/version.c`.
//! - `VerQueryValueW`: parses a version block; pure, but in `dlls/kernelbase/version.c`.
//! - `VerInstallFileA`: partial (helpers above); file I/O, temp files, and moves are skipped.
//! - `VerInstallFileW`: partial (helpers above); `WideCharToMultiByte` and file I/O are skipped.
//!
//! Also skipped, not exported: the `_fetch_versioninfo` retry loop (it calls
//! `GetFileVersionInfoA` and the process heap), the `VerQueryValue` translation
//! block (its body is a comment, so it has no effect), and the `TRACE`, `WARN`,
//! and `FIXME` logging.

/// `VIF_TEMPFILE`: the temp file name was returned to the caller.
pub const VIF_TEMPFILE: u32 = 0x0000_0001;
/// `VIF_MISMATCH`: the source and destination differ.
pub const VIF_MISMATCH: u32 = 0x0000_0002;
/// `VIF_SRCOLD`: the source is older than the destination.
pub const VIF_SRCOLD: u32 = 0x0000_0004;
/// `VIF_DIFFTYPE`: the file type or subtype differs.
pub const VIF_DIFFTYPE: u32 = 0x0000_0020;
/// `VIF_OUTOFSPACE`: writing the temp file failed for lack of space.
pub const VIF_OUTOFSPACE: u32 = 0x0000_0100;
/// `VIF_ACCESSVIOLATION`: access to the destination was denied.
pub const VIF_ACCESSVIOLATION: u32 = 0x0000_0200;
/// `VIF_SHARINGVIOLATION`: the destination is in use by another process.
pub const VIF_SHARINGVIOLATION: u32 = 0x0000_0400;
/// `VIF_OUTOFMEMORY`: an LZ buffer could not be allocated.
pub const VIF_OUTOFMEMORY: u32 = 0x0000_8000;
/// `VIF_CANNOTREADSRC`: the source file could not be read.
pub const VIF_CANNOTREADSRC: u32 = 0x0001_0000;
/// `VIF_BUFFTOOSMALL`: the caller's temp-name buffer is too small.
pub const VIF_BUFFTOOSMALL: u32 = 0x0004_0000;

/// `VS_FFI_SIGNATURE`, the magic value of a `VS_FIXEDFILEINFO`.
pub const VS_FFI_SIGNATURE: u32 = 0xfeef_04bd;

const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_SHARING_VIOLATION: u32 = 32;

const LZERROR_BADINHANDLE: i32 = -1;
const LZERROR_BADOUTHANDLE: i32 = -2;
const LZERROR_READ: i32 = -3;
const LZERROR_WRITE: i32 = -4;
const LZERROR_GLOBALLOC: i32 = -5;
const LZERROR_GLOBLOCK: i32 = -6;
const LZERROR_BADVALUE: i32 = -7;
const LZERROR_UNKNOWNALG: i32 = -8;

/// Offset of `VS_FIXEDFILEINFO` in a version block (`buf + 0x14`).
const FFI_OFFSET: usize = 0x14;
/// Offset used when the block's marker word is `UNICODE_MARKER` (`buf + 0x28`).
const UNICODE_FFI_OFFSET: usize = 0x28;
/// The value the reference reads at `FFI_OFFSET` to detect a UTF-16 block.
const UNICODE_MARKER: u32 = 0x004f_0049;
/// Size of `VS_FIXEDFILEINFO`: thirteen DWORDs.
const FFI_LEN: usize = 13 * 4;

/// `_error2vif`: the `VIF_*` bit for a Win32 error, or 0 when there is none.
pub fn error_to_vif(error: u32) -> u32 {
    match error {
        ERROR_ACCESS_DENIED => VIF_ACCESSVIOLATION,
        ERROR_SHARING_VIOLATION => VIF_SHARINGVIOLATION,
        _ => 0,
    }
}

/// The `LZCopy` error switch in `VerInstallFileA`. Non-error and unknown values
/// map to 0, which the reference also ignores.
pub fn lz_copy_error_to_vif(ret: i32) -> u32 {
    match ret {
        LZERROR_BADINHANDLE | LZERROR_READ | LZERROR_BADVALUE | LZERROR_UNKNOWNALG => {
            VIF_CANNOTREADSRC
        }
        LZERROR_BADOUTHANDLE | LZERROR_WRITE => VIF_OUTOFSPACE,
        LZERROR_GLOBALLOC | LZERROR_GLOBLOCK => VIF_OUTOFMEMORY,
        _ => 0,
    }
}

/// `VS_FIXEDFILEINFO`: thirteen little-endian DWORDs, in file order.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FixedFileInfo {
    pub signature: u32,
    pub struc_version: u32,
    pub file_version_ms: u32,
    pub file_version_ls: u32,
    pub product_version_ms: u32,
    pub product_version_ls: u32,
    pub file_flags_mask: u32,
    pub file_flags: u32,
    pub file_os: u32,
    pub file_type: u32,
    pub file_subtype: u32,
    pub file_date_ms: u32,
    pub file_date_ls: u32,
}

impl FixedFileInfo {
    fn from_words(words: [u32; 13]) -> Self {
        let [signature, struc_version, file_version_ms, file_version_ls, product_version_ms, product_version_ls, file_flags_mask, file_flags, file_os, file_type, file_subtype, file_date_ms, file_date_ls] =
            words;
        Self {
            signature,
            struc_version,
            file_version_ms,
            file_version_ls,
            product_version_ms,
            product_version_ls,
            file_flags_mask,
            file_flags,
            file_os,
            file_type,
            file_subtype,
            file_date_ms,
            file_date_ls,
        }
    }
}

/// The locate step of `_fetch_versioninfo`: reads the `VS_FIXEDFILEINFO` from a
/// version block. The block holds a UTF-16 marker word at `0x14` (`0x004f0049`
/// in the reference's check); when it is set, the structure is at `0x28`. The
/// signature is not checked, because the reference only logs a warning when it
/// is wrong. Returns `None` when the block is too short to hold the structure.
pub fn fixed_file_info(block: &[u8]) -> Option<FixedFileInfo> {
    let marker = block.get(FFI_OFFSET..FFI_OFFSET + 4)?;
    let at = if le_u32(marker) == UNICODE_MARKER {
        UNICODE_FFI_OFFSET
    } else {
        FFI_OFFSET
    };
    let bytes = block.get(at..at + FFI_LEN)?;
    let words = std::array::from_fn(|i| le_u32(&bytes[4 * i..]));
    Some(FixedFileInfo::from_words(words))
}

/// The version and type comparison in `VerInstallFileA`. A destination newer than
/// the source sets `VIF_MISMATCH | VIF_SRCOLD`. Differing file type or subtype
/// sets `VIF_MISMATCH | VIF_DIFFTYPE`.
pub fn compare_file_versions(dest: &FixedFileInfo, src: &FixedFileInfo) -> u32 {
    let mut xret = 0;
    if (dest.file_version_ms, dest.file_version_ls) > (src.file_version_ms, src.file_version_ls) {
        xret |= VIF_MISMATCH | VIF_SRCOLD;
    }
    if dest.file_type != src.file_type || dest.file_subtype != src.file_subtype {
        xret |= VIF_MISMATCH | VIF_DIFFTYPE;
    }
    xret
}

/// The temp-name tail from `strrchr(tmpfn, '\\')`, as `VerInstallFileA` uses
/// it. The tail includes the backslash. A path with no backslash is returned
/// whole.
pub fn temp_file_tail(path: &[u8]) -> &[u8] {
    let last = path.iter().rposition(|&c| c == b'\\').unwrap_or(0);
    &path[last..]
}

/// Result of the `tmpfilelen` check in `VerInstallFileA`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempFile<'a> {
    /// The buffer is smaller than the name: the caller deletes the temp file.
    BufferTooSmall,
    /// `name` is copied to the caller's buffer, and `len` (with the NUL) is returned.
    Copied { name: &'a [u8], len: u32 },
}

impl TempFile<'_> {
    /// The `VIF_*` bit that the reference ORs into the install result.
    pub fn flag(&self) -> u32 {
        match self {
            TempFile::BufferTooSmall => VIF_BUFFTOOSMALL,
            TempFile::Copied { .. } => VIF_TEMPFILE,
        }
    }
}

/// The `tmpfilelen` check in `VerInstallFileA`, for a non-zero `xret`. The
/// reference tests `tmpfilelen < strlen(tail)`, so a buffer exactly as long as
/// the tail passes. Its `strcpy` then writes one byte past that buffer. This port
/// keeps the same rule and reports `tail.len() + 1` as the length, but it cannot
/// write into the caller's buffer.
pub fn temp_file_outcome(tail: &[u8], tmpfilelen: u32) -> TempFile<'_> {
    if (tmpfilelen as usize) < tail.len() {
        TempFile::BufferTooSmall
    } else {
        TempFile::Copied {
            name: tail,
            len: (tail.len() + 1) as u32,
        }
    }
}

/// Reads a little-endian DWORD from the first four bytes of `bytes`.
fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[cfg(test)]
mod tests {
    // Table rows are vectors printed by the Wine reference oracle: verbatim
    // extracts of dlls/version/version.c, built with ASan and UBSan.
    use super::*;

    fn ver(words: [u32; 4]) -> FixedFileInfo {
        FixedFileInfo {
            file_version_ms: words[0],
            file_version_ls: words[1],
            file_type: words[2],
            file_subtype: words[3],
            ..Default::default()
        }
    }

    /// Builds a version block the way the oracle driver does: fields at `at`,
    /// with the UTF-16 marker at `0x14` when `at` is `0x28`.
    fn version_block(at: usize, words: [u32; 13]) -> Vec<u8> {
        let mut block = vec![0u8; 0x80];
        if at == UNICODE_FFI_OFFSET {
            block[FFI_OFFSET..FFI_OFFSET + 4].copy_from_slice(&UNICODE_MARKER.to_le_bytes());
        }
        for (i, word) in words.iter().enumerate() {
            block[at + 4 * i..at + 4 * i + 4].copy_from_slice(&word.to_le_bytes());
        }
        block
    }

    #[test]
    fn error_to_vif_matches_oracle() {
        const ROWS: [(u32, u32); 4] = [(0, 0x0), (5, 0x200), (32, 0x400), (33, 0x0)];
        for (error, flags) in ROWS {
            assert_eq!(error_to_vif(error), flags, "error {error}");
        }
    }

    #[test]
    fn lz_copy_error_to_vif_matches_oracle() {
        const ROWS: [(i32, u32); 4] = [(-1, 0x1_0000), (-2, 0x100), (-5, 0x8000), (-9, 0x0)];
        for (ret, flags) in ROWS {
            assert_eq!(lz_copy_error_to_vif(ret), flags, "ret {ret}");
        }
    }

    #[test]
    fn fixed_file_info_matches_oracle() {
        const ROWS: [(usize, [u32; 13]); 4] = [
            (
                0x14,
                [
                    0xfeef_04bd,
                    0x0001_0000,
                    0x0001_0002,
                    0x0000_0003,
                    0x0001_0002,
                    0x0000_0003,
                    0x0000_003f,
                    0x0000_0000,
                    0x0004_0004,
                    0x0000_0001,
                    0x0000_0000,
                    0x01d0_a000,
                    0x1234_5678,
                ],
            ),
            (
                0x28,
                [
                    0xfeef_04bd,
                    0x0001_0000,
                    0x0002_0000,
                    0x0000_0004,
                    0x0002_0000,
                    0x0000_0004,
                    0x0000_003f,
                    0x0000_0002,
                    0x0004_0004,
                    0x0000_0002,
                    0x0000_0007,
                    0x0000_0000,
                    0xffff_ffff,
                ],
            ),
            (
                0x14,
                [
                    0x1234_5678,
                    0x0000_0000,
                    0x0000_0001,
                    0x0000_0002,
                    0x0000_0003,
                    0x0000_0004,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                ],
            ),
            (
                0x28,
                [
                    0xfeef_04bd,
                    0x0001_0000,
                    0x0000_0005,
                    0x0000_0000,
                    0x0000_0005,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0000,
                    0x0000_0003,
                    0x0000_0001,
                    0x0000_0000,
                    0x0000_0000,
                ],
            ),
        ];
        for (at, words) in ROWS {
            let block = version_block(at, words);
            assert_eq!(
                fixed_file_info(&block),
                Some(FixedFileInfo::from_words(words)),
                "at {at:#x}"
            );
        }
    }

    #[test]
    fn fixed_file_info_rejects_short_blocks() {
        let words = [VS_FFI_SIGNATURE; 13];
        assert_eq!(fixed_file_info(&[]), None);
        let normal = version_block(FFI_OFFSET, words);
        assert_eq!(fixed_file_info(&normal[..0x17]), None);
        assert_eq!(fixed_file_info(&normal[..0x47]), None);
        assert!(fixed_file_info(&normal[..0x48]).is_some());
        let unicode = version_block(UNICODE_FFI_OFFSET, words);
        assert_eq!(fixed_file_info(&unicode[..0x5b]), None);
        assert!(fixed_file_info(&unicode[..0x5c]).is_some());
    }

    #[test]
    fn compare_file_versions_matches_oracle() {
        const ROWS: [([u32; 4], [u32; 4], u32); 6] = [
            ([1, 2, 1, 0], [1, 2, 1, 0], 0x0),
            ([2, 0, 1, 0], [1, 9, 1, 0], 0x6),
            ([1, 5, 1, 0], [1, 4, 1, 0], 0x6),
            ([1, 4, 1, 0], [1, 5, 1, 0], 0x0),
            ([1, 2, 1, 0], [1, 2, 3, 0], 0x22),
            ([1, 2, 1, 4], [1, 2, 1, 0], 0x22),
        ];
        for (dest, src, flags) in ROWS {
            assert_eq!(
                compare_file_versions(&ver(dest), &ver(src)),
                flags,
                "dest {dest:?} src {src:?}"
            );
        }
    }

    #[test]
    fn temp_file_tail_matches_oracle() {
        const ROWS: [(&[u8], usize); 3] = [
            (b"C:\\WINDOWS\\ver1.tmp", 10),
            (b"ver1.tmp", 0),
            (b"a\\b\\c", 3),
        ];
        for (path, tmplast) in ROWS {
            assert_eq!(temp_file_tail(path), &path[tmplast..]);
        }
    }

    #[test]
    fn temp_file_outcome_matches_oracle() {
        const ROWS: [(&[u8], u32, TempFile<'static>, u32); 4] = [
            (
                b"\\ver1.tmp",
                64,
                TempFile::Copied {
                    name: b"\\ver1.tmp",
                    len: 10,
                },
                0x7,
            ),
            (
                b"\\ver1.tmp",
                9,
                TempFile::Copied {
                    name: b"\\ver1.tmp",
                    len: 10,
                },
                0x7,
            ),
            (b"\\ver1.tmp", 8, TempFile::BufferTooSmall, 0x4_0006),
            (b"ver", 2, TempFile::BufferTooSmall, 0x4_0006),
        ];
        for (tail, tmpfilelen, expected, xret) in ROWS {
            let got = temp_file_outcome(tail, tmpfilelen);
            assert_eq!(got, expected, "tmpfilelen {tmpfilelen}");
            assert_eq!(VIF_MISMATCH | VIF_SRCOLD | got.flag(), xret);
        }
    }
}
