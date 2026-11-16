//! Dependency-free PE/COFF parser.
//!
//! Foundation of the loader in Mustum Protonus Ferrous (MPF): every Windows
//! module the port loads goes through this parser, mirroring what `loader/`
//! and `dlls/ntdll/loader.c` do in the reference Wine source.

use std::fmt;

pub const DOS_MAGIC: u16 = 0x5A4D; // "MZ"
pub const PE_SIGNATURE: u32 = 0x0000_4550; // "PE\0\0"
pub const OPT_MAGIC_PE32: u16 = 0x10B;
pub const OPT_MAGIC_PE32PLUS: u16 = 0x20B;

pub const MACHINE_UNKNOWN: u16 = 0x0000;
pub const MACHINE_I386: u16 = 0x014C;
pub const MACHINE_AMD64: u16 = 0x8664;
pub const MACHINE_ARM64: u16 = 0xAA64;

pub const DIR_EXPORT: usize = 0;
pub const DIR_IMPORT: usize = 1;
pub const DIR_BASERELOC: usize = 5;
pub const DIR_TLS: usize = 9;
pub const DIR_DELAY_IMPORT: usize = 13;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeError {
    Truncated,
    BadDosMagic,
    BadPeSignature,
    UnsupportedMagic(u16),
    BadTable,
}

impl fmt::Display for PeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeError::Truncated => write!(f, "file is truncated"),
            PeError::BadDosMagic => write!(f, "bad DOS magic (expected MZ)"),
            PeError::BadPeSignature => write!(f, "bad PE signature"),
            PeError::UnsupportedMagic(m) => {
                write!(f, "unsupported optional-header magic {m:#06x}")
            }
            PeError::BadTable => write!(f, "bad or out-of-range table"),
        }
    }
}

impl std::error::Error for PeError {}

fn u16_at(d: &[u8], o: usize) -> Result<u16, PeError> {
    let b = d
        .get(o..)
        .and_then(|s| s.get(..2))
        .ok_or(PeError::Truncated)?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

fn u32_at(d: &[u8], o: usize) -> Result<u32, PeError> {
    let b = d
        .get(o..)
        .and_then(|s| s.get(..4))
        .ok_or(PeError::Truncated)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn u64_at(d: &[u8], o: usize) -> Result<u64, PeError> {
    let b = d
        .get(o..)
        .and_then(|s| s.get(..8))
        .ok_or(PeError::Truncated)?;
    Ok(u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

pub fn machine_name(machine: u16) -> &'static str {
    match machine {
        MACHINE_I386 => "i386",
        MACHINE_AMD64 => "x86_64",
        MACHINE_ARM64 => "aarch64",
        _ => "unknown",
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CoffHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct DataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub characteristics: u32,
}

impl SectionHeader {
    pub fn name(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(8);
        std::str::from_utf8(&self.name[..end]).unwrap_or("")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OptionalHeader {
    Pe32 {
        image_base: u32,
        entry_point_rva: u32,
        size_of_image: u32,
        size_of_headers: u32,
        subsystem: u16,
        dll_characteristics: u16,
    },
    Pe32Plus {
        image_base: u64,
        entry_point_rva: u32,
        size_of_image: u32,
        size_of_headers: u32,
        subsystem: u16,
        dll_characteristics: u16,
    },
}

#[derive(Debug, Clone)]
pub struct ImportEntry {
    pub dll: String,
    pub function: String,
    pub ordinal: Option<u16>,
    /// RVA of the IAT slot that receives the resolved address.
    pub thunk_rva: u32,
}

#[derive(Debug, Clone)]
pub struct ExportEntry {
    pub name: String,
    pub ordinal: u16,
    pub rva: u32,
}

/// One base-relocation entry: `reloc_type` (3 = HIGHLOW, 10 = DIR64) and the
/// `offset` within the block's page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelocEntry {
    pub reloc_type: u8,
    pub offset: u16,
}

#[derive(Debug, Clone)]
pub struct RelocBlock {
    pub page_rva: u32,
    pub entries: Vec<RelocEntry>,
}

#[derive(Debug, Clone, Copy)]
pub struct TlsDirectory {
    pub start_address_of_raw_data: u64,
    pub end_address_of_raw_data: u64,
    pub address_of_index: u64,
    pub address_of_call_backs: u64,
    pub size_of_zero_fill: u32,
    pub characteristics: u32,
}

#[derive(Debug, Clone)]
pub struct DelayImport {
    pub dll: String,
    pub module_handle_rva: u32,
    pub iat_rva: u32,
    pub int_rva: u32,
    pub entries: Vec<ImportEntry>,
}

#[derive(Debug, Clone)]
pub struct PeImage<'a> {
    data: &'a [u8],
    pub coff: CoffHeader,
    pub optional: OptionalHeader,
    pub sections: Vec<SectionHeader>,
    pub data_directories: Vec<DataDirectory>,
}

impl<'a> PeImage<'a> {
    pub fn parse(data: &'a [u8]) -> Result<PeImage<'a>, PeError> {
        if u16_at(data, 0)? != DOS_MAGIC {
            return Err(PeError::BadDosMagic);
        }
        let pe_off = u32_at(data, 0x3C)? as usize;
        if u32_at(data, pe_off)? != PE_SIGNATURE {
            return Err(PeError::BadPeSignature);
        }
        let coff_off = pe_off + 4;
        let coff = CoffHeader {
            machine: u16_at(data, coff_off)?,
            number_of_sections: u16_at(data, coff_off + 2)?,
            size_of_optional_header: u16_at(data, coff_off + 16)?,
            characteristics: u16_at(data, coff_off + 18)?,
        };
        let opt_off = coff_off + 20;
        let magic = u16_at(data, opt_off)?;
        let (optional, dirs_off, num_dirs) = match magic {
            OPT_MAGIC_PE32 => {
                let entry_point_rva = u32_at(data, opt_off + 16)?;
                let image_base = u32_at(data, opt_off + 28)?;
                let size_of_image = u32_at(data, opt_off + 56)?;
                let size_of_headers = u32_at(data, opt_off + 60)?;
                let subsystem = u16_at(data, opt_off + 68)?;
                let dll_characteristics = u16_at(data, opt_off + 70)?;
                let n = u32_at(data, opt_off + 92)? as usize;
                let opt = OptionalHeader::Pe32 {
                    image_base,
                    entry_point_rva,
                    size_of_image,
                    size_of_headers,
                    subsystem,
                    dll_characteristics,
                };
                (opt, opt_off + 96, n)
            }
            OPT_MAGIC_PE32PLUS => {
                let entry_point_rva = u32_at(data, opt_off + 16)?;
                let image_base = u64_at(data, opt_off + 24)?;
                let size_of_image = u32_at(data, opt_off + 56)?;
                let size_of_headers = u32_at(data, opt_off + 60)?;
                let subsystem = u16_at(data, opt_off + 68)?;
                let dll_characteristics = u16_at(data, opt_off + 70)?;
                let n = u32_at(data, opt_off + 108)? as usize;
                let opt = OptionalHeader::Pe32Plus {
                    image_base,
                    entry_point_rva,
                    size_of_image,
                    size_of_headers,
                    subsystem,
                    dll_characteristics,
                };
                (opt, opt_off + 112, n)
            }
            other => return Err(PeError::UnsupportedMagic(other)),
        };
        let mut data_directories = Vec::new();
        for i in 0..num_dirs.min(16) {
            let off = dirs_off + i * 8;
            data_directories.push(DataDirectory {
                virtual_address: u32_at(data, off)?,
                size: u32_at(data, off + 4)?,
            });
        }
        let sec_off = opt_off + coff.size_of_optional_header as usize;
        let mut sections = Vec::new();
        for i in 0..coff.number_of_sections as usize {
            let o = sec_off + i * 40;
            let mut name = [0u8; 8];
            name.copy_from_slice(data.get(o..o + 8).ok_or(PeError::Truncated)?);
            sections.push(SectionHeader {
                name,
                virtual_size: u32_at(data, o + 8)?,
                virtual_address: u32_at(data, o + 12)?,
                size_of_raw_data: u32_at(data, o + 16)?,
                pointer_to_raw_data: u32_at(data, o + 20)?,
                characteristics: u32_at(data, o + 36)?,
            });
        }
        Ok(PeImage {
            data,
            coff,
            optional,
            sections,
            data_directories,
        })
    }

    pub fn machine(&self) -> u16 {
        self.coff.machine
    }

    pub fn is_64bit(&self) -> bool {
        matches!(self.optional, OptionalHeader::Pe32Plus { .. })
    }

    pub fn image_base(&self) -> u64 {
        match self.optional {
            OptionalHeader::Pe32 { image_base, .. } => image_base as u64,
            OptionalHeader::Pe32Plus { image_base, .. } => image_base,
        }
    }

    pub fn size_of_image(&self) -> u32 {
        match self.optional {
            OptionalHeader::Pe32 { size_of_image, .. }
            | OptionalHeader::Pe32Plus { size_of_image, .. } => size_of_image,
        }
    }

    pub fn size_of_headers(&self) -> u32 {
        match self.optional {
            OptionalHeader::Pe32 {
                size_of_headers, ..
            }
            | OptionalHeader::Pe32Plus {
                size_of_headers, ..
            } => size_of_headers,
        }
    }

    pub fn entry_point_rva(&self) -> u32 {
        match self.optional {
            OptionalHeader::Pe32 {
                entry_point_rva, ..
            }
            | OptionalHeader::Pe32Plus {
                entry_point_rva, ..
            } => entry_point_rva,
        }
    }

    pub fn subsystem(&self) -> u16 {
        match self.optional {
            OptionalHeader::Pe32 { subsystem, .. } | OptionalHeader::Pe32Plus { subsystem, .. } => {
                subsystem
            }
        }
    }

    pub fn directory(&self, index: usize) -> Option<DataDirectory> {
        self.data_directories.get(index).copied()
    }

    /// The raw file bytes backing this image.
    pub fn raw_data(&self) -> &'a [u8] {
        self.data
    }

    pub fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        if rva < self.size_of_headers() {
            return Some(rva as usize);
        }
        for s in &self.sections {
            let span = s.virtual_size.max(s.size_of_raw_data);
            let Some(section_end) = s.virtual_address.checked_add(span) else {
                continue;
            };
            if rva >= s.virtual_address && rva < section_end {
                let off = u64::from(s.pointer_to_raw_data) + u64::from(rva - s.virtual_address);
                return usize::try_from(off).ok().filter(|&o| o <= self.data.len());
            }
        }
        None
    }

    fn cstr_at(&self, off: usize) -> Result<String, PeError> {
        let bytes = self.data.get(off..).ok_or(PeError::BadTable)?;
        let end = bytes
            .iter()
            .position(|&b| b == 0)
            .ok_or(PeError::BadTable)?;
        Ok(String::from_utf8_lossy(&bytes[..end]).into_owned())
    }

    fn cstr_at_rva(&self, rva: u32) -> Result<String, PeError> {
        let off = self.rva_to_offset(rva).ok_or(PeError::BadTable)?;
        self.cstr_at(off)
    }

    pub fn exports(&self) -> Result<Vec<ExportEntry>, PeError> {
        let mut out = Vec::new();
        let Some(dir) = self.directory(DIR_EXPORT) else {
            return Ok(out);
        };
        if dir.virtual_address == 0 {
            return Ok(out);
        }
        let base = self
            .rva_to_offset(dir.virtual_address)
            .ok_or(PeError::BadTable)?;
        let ordinal_base = u32_at(self.data, base + 16)?;
        let num_functions = u32_at(self.data, base + 20)? as usize;
        let num_names = u32_at(self.data, base + 24)? as usize;
        let funcs_rva = u32_at(self.data, base + 28)?;
        let names_rva = u32_at(self.data, base + 32)?;
        let ords_rva = u32_at(self.data, base + 36)?;
        let funcs = self.rva_to_offset(funcs_rva).ok_or(PeError::BadTable)?;
        let names = self.rva_to_offset(names_rva).ok_or(PeError::BadTable)?;
        let ords = self.rva_to_offset(ords_rva).ok_or(PeError::BadTable)?;
        for i in 0..num_names.min(num_functions) {
            let name_ptr = u32_at(self.data, names + i * 4)?;
            let ord_idx = u16_at(self.data, ords + i * 2)? as usize;
            if ord_idx >= num_functions {
                return Err(PeError::BadTable);
            }
            let func_off = funcs + ord_idx * 4;
            let func_rva = u32_at(self.data, func_off)?;
            out.push(ExportEntry {
                name: self.cstr_at_rva(name_ptr)?,
                ordinal: (ordinal_base as u16).wrapping_add(ord_idx as u16),
                rva: func_rva,
            });
        }
        Ok(out)
    }

    pub fn imports(&self) -> Result<Vec<ImportEntry>, PeError> {
        let mut out = Vec::new();
        let Some(dir) = self.directory(DIR_IMPORT) else {
            return Ok(out);
        };
        if dir.virtual_address == 0 {
            return Ok(out);
        }
        let desc = self
            .rva_to_offset(dir.virtual_address)
            .ok_or(PeError::BadTable)?;
        let plus = self.is_64bit();
        let thunk_size = if plus { 8 } else { 4 };
        let ord_flag: u64 = if plus { 1 << 63 } else { 1 << 31 };
        let mut i = 0usize;
        loop {
            let o = desc + i * 20;
            let oft = u32_at(self.data, o)?;
            let name_rva = u32_at(self.data, o + 12)?;
            let first_thunk = u32_at(self.data, o + 16)?;
            if oft == 0 && name_rva == 0 && first_thunk == 0 {
                break;
            }
            let dll = self.cstr_at_rva(name_rva)?;
            let thunk_rva = if oft != 0 { oft } else { first_thunk };
            let thunk = self.rva_to_offset(thunk_rva).ok_or(PeError::BadTable)?;
            let mut j = 0usize;
            loop {
                let t = if plus {
                    u64_at(self.data, thunk + j * thunk_size)?
                } else {
                    u32_at(self.data, thunk + j * thunk_size)? as u64
                };
                if t == 0 {
                    break;
                }
                let iat_slot = first_thunk.wrapping_add(j as u32 * thunk_size as u32);
                if t & ord_flag != 0 {
                    out.push(ImportEntry {
                        dll: dll.clone(),
                        function: String::new(),
                        ordinal: Some((t & 0xFFFF) as u16),
                        thunk_rva: iat_slot,
                    });
                } else {
                    let hn = self.rva_to_offset(t as u32).ok_or(PeError::BadTable)?;
                    let function = self.cstr_at(hn + 2)?;
                    out.push(ImportEntry {
                        dll: dll.clone(),
                        function,
                        ordinal: None,
                        thunk_rva: iat_slot,
                    });
                }
                j += 1;
            }
            i += 1;
        }
        Ok(out)
    }

    /// Base relocation blocks (data directory 5). Type 0 (ABSOLUTE) entries
    /// are padding and skipped.
    pub fn relocations(&self) -> Result<Vec<RelocBlock>, PeError> {
        let mut out = Vec::new();
        let Some(dir) = self.directory(DIR_BASERELOC) else {
            return Ok(out);
        };
        if dir.virtual_address == 0 {
            return Ok(out);
        }
        let mut off = self
            .rva_to_offset(dir.virtual_address)
            .ok_or(PeError::BadTable)?;
        let end = off.saturating_add(dir.size as usize).min(self.data.len());
        while off + 8 <= end {
            let page_rva = u32_at(self.data, off)?;
            let block_size = u32_at(self.data, off + 4)? as usize;
            if block_size < 8 || off + block_size > self.data.len() {
                break;
            }
            let mut entries = Vec::new();
            for i in 0..(block_size - 8) / 2 {
                let e = u16_at(self.data, off + 8 + i * 2)?;
                let reloc_type = (e >> 12) as u8;
                if reloc_type != 0 {
                    entries.push(RelocEntry {
                        reloc_type,
                        offset: e & 0xFFF,
                    });
                }
            }
            out.push(RelocBlock { page_rva, entries });
            off += block_size;
        }
        Ok(out)
    }

    /// TLS directory (data directory 9), if present.
    pub fn tls_directory(&self) -> Result<Option<TlsDirectory>, PeError> {
        let Some(dir) = self.directory(DIR_TLS) else {
            return Ok(None);
        };
        if dir.virtual_address == 0 {
            return Ok(None);
        }
        let off = self
            .rva_to_offset(dir.virtual_address)
            .ok_or(PeError::BadTable)?;
        let d = match self.optional {
            OptionalHeader::Pe32Plus { .. } => TlsDirectory {
                start_address_of_raw_data: u64_at(self.data, off)?,
                end_address_of_raw_data: u64_at(self.data, off + 8)?,
                address_of_index: u64_at(self.data, off + 16)?,
                address_of_call_backs: u64_at(self.data, off + 24)?,
                size_of_zero_fill: u32_at(self.data, off + 32)?,
                characteristics: u32_at(self.data, off + 36)?,
            },
            OptionalHeader::Pe32 { .. } => TlsDirectory {
                start_address_of_raw_data: u64::from(u32_at(self.data, off)?),
                end_address_of_raw_data: u64::from(u32_at(self.data, off + 4)?),
                address_of_index: u64::from(u32_at(self.data, off + 8)?),
                address_of_call_backs: u64::from(u32_at(self.data, off + 12)?),
                size_of_zero_fill: u32_at(self.data, off + 16)?,
                characteristics: u32_at(self.data, off + 20)?,
            },
        };
        Ok(Some(d))
    }

    /// Delay-load import descriptors (data directory 13). Each entry's
    /// `thunk_rva` points at the delay-IAT slot to be patched on first use.
    pub fn delay_imports(&self) -> Result<Vec<DelayImport>, PeError> {
        let mut out = Vec::new();
        let Some(dir) = self.directory(DIR_DELAY_IMPORT) else {
            return Ok(out);
        };
        if dir.virtual_address == 0 {
            return Ok(out);
        }
        let desc = self
            .rva_to_offset(dir.virtual_address)
            .ok_or(PeError::BadTable)?;
        let plus = self.is_64bit();
        let thunk_size = if plus { 8 } else { 4 };
        let ord_flag: u64 = if plus { 1 << 63 } else { 1 << 31 };
        let mut i = 0usize;
        loop {
            let o = desc + i * 32;
            let gr_attrs = u32_at(self.data, o)?;
            let name_rva = u32_at(self.data, o + 4)?;
            let phmod = u32_at(self.data, o + 8)?;
            let p_iat = u32_at(self.data, o + 12)?;
            let p_int = u32_at(self.data, o + 16)?;
            if gr_attrs == 0 && name_rva == 0 && p_iat == 0 && p_int == 0 {
                break;
            }
            let dll = self.cstr_at_rva(name_rva)?;
            let lookup_rva = if p_int != 0 { p_int } else { p_iat };
            let lookup = self.rva_to_offset(lookup_rva).ok_or(PeError::BadTable)?;
            let mut entries = Vec::new();
            let mut j = 0usize;
            loop {
                let t = if plus {
                    u64_at(self.data, lookup + j * thunk_size)?
                } else {
                    u32_at(self.data, lookup + j * thunk_size)? as u64
                };
                if t == 0 {
                    break;
                }
                let iat_slot = p_iat.wrapping_add(j as u32 * thunk_size as u32);
                if t & ord_flag != 0 {
                    entries.push(ImportEntry {
                        dll: dll.clone(),
                        function: String::new(),
                        ordinal: Some((t & 0xFFFF) as u16),
                        thunk_rva: iat_slot,
                    });
                } else {
                    let hn = self.rva_to_offset(t as u32).ok_or(PeError::BadTable)?;
                    let function = self.cstr_at(hn + 2)?;
                    entries.push(ImportEntry {
                        dll: dll.clone(),
                        function,
                        ordinal: None,
                        thunk_rva: iat_slot,
                    });
                }
                j += 1;
            }
            out.push(DelayImport {
                dll,
                module_handle_rva: phmod,
                iat_rva: p_iat,
                int_rva: p_int,
                entries,
            });
            i += 1;
        }
        Ok(out)
    }
}

/// Synthetic PE images used by MPF's own tests and by downstream crates' tests.
pub mod testing {
    pub const TEST_IMAGE_BASE: u64 = 0x1_4000_0000;
    pub const TEST_SIZE_OF_IMAGE: u32 = 0x2000;

    fn p16(b: &mut [u8], off: usize, v: u16) {
        b[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn p32(b: &mut [u8], off: usize, v: u32) {
        b[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn p64(b: &mut [u8], off: usize, v: u64) {
        b[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }
    fn ps(b: &mut [u8], off: usize, s: &str) {
        b[off..off + s.len()].copy_from_slice(s.as_bytes());
    }

    /// A minimal but fully-formed PE32+ DLL with one `.rdata` section,
    /// one export (`TestFunc`, ordinal 1), one import (`KERNEL32.dll!BaseFunc`),
    /// one base-relocation block, a TLS directory with one callback, and one
    /// delay import (`DELAY.dll!DelayFunc`).
    pub fn build_test_pe64() -> Vec<u8> {
        let mut b = vec![0u8; 0x500];
        // DOS header
        b[0] = b'M';
        b[1] = b'Z';
        p32(&mut b, 0x3C, 0x80);
        // PE signature + COFF
        p32(&mut b, 0x80, 0x4550);
        p16(&mut b, 0x84, 0x8664); // AMD64
        p16(&mut b, 0x86, 1); // sections
        p16(&mut b, 0x94, 0xF0); // optional header size
        p16(&mut b, 0x96, 0x2022); // DLL | executable | large address aware
                                   // Optional header PE32+ at 0x98
        p16(&mut b, 0x98, 0x20B);
        b[0x9A] = 1; // linker major
        p32(&mut b, 0x9C, 0x200); // size of code
        p32(&mut b, 0xA0, 0x400); // size of initialized data
        p32(&mut b, 0xA8, 0x1000); // entry point RVA
        p32(&mut b, 0xAC, 0x1000); // base of code
        p64(&mut b, 0xB0, TEST_IMAGE_BASE);
        p32(&mut b, 0xB8, 0x1000); // section alignment
        p32(&mut b, 0xBC, 0x200); // file alignment
        p16(&mut b, 0xC0, 6); // OS version 6.0
        p16(&mut b, 0xC8, 6); // subsystem version 6.0
        p32(&mut b, 0xD0, TEST_SIZE_OF_IMAGE);
        p32(&mut b, 0xD4, 0x200); // size of headers
        p16(&mut b, 0xDC, 3); // subsystem: console
        p16(&mut b, 0xDE, 0x160);
        p64(&mut b, 0xE0, 0x100000); // stack reserve
        p64(&mut b, 0xE8, 0x1000); // stack commit
        p64(&mut b, 0xF0, 0x100000); // heap reserve
        p64(&mut b, 0xF8, 0x1000); // heap commit
        p32(&mut b, 0x104, 16); // number of RVA and sizes
                                // data directories at 0x108: [0] export, [1] import
        p32(&mut b, 0x108, 0x1000);
        p32(&mut b, 0x10C, 0x28);
        p32(&mut b, 0x110, 0x1100);
        p32(&mut b, 0x114, 0x28);
        p32(&mut b, 0x130, 0x11A0); // [5] base relocations
        p32(&mut b, 0x134, 12);
        p32(&mut b, 0x150, 0x11B0); // [9] TLS directory
        p32(&mut b, 0x154, 40);
        p32(&mut b, 0x170, 0x11E0); // [13] delay imports
        p32(&mut b, 0x174, 64);
        // section header at 0x188: .rdata, RVA 0x1000, raw at 0x200
        ps(&mut b, 0x188, ".rdata");
        p32(&mut b, 0x190, 0x300);
        p32(&mut b, 0x194, 0x1000);
        p32(&mut b, 0x198, 0x300);
        p32(&mut b, 0x19C, 0x200);
        p32(&mut b, 0x1A8, 0x4000_0040); // initialized read-only data
                                         // raw data (file offset 0x200 == RVA 0x1000)
                                         // export directory at RVA 0x1000 (file 0x200)
        p32(&mut b, 0x20C, 0x1048); // name RVA
        p32(&mut b, 0x210, 1); // ordinal base
        p32(&mut b, 0x214, 1); // number of functions
        p32(&mut b, 0x218, 1); // number of names
        p32(&mut b, 0x21C, 0x1030); // address of functions
        p32(&mut b, 0x220, 0x1038); // address of names
        p32(&mut b, 0x224, 0x103C); // address of name ordinals
        p32(&mut b, 0x230, 0x1080); // functions[0] RVA (file 0x230 == RVA 0x1030)
        p32(&mut b, 0x238, 0x1060); // names[0] RVA (file 0x238 == RVA 0x1038)
        p16(&mut b, 0x23C, 0); // name ordinals[0] (file 0x23C == RVA 0x103C)
        ps(&mut b, 0x248, "mpf_test.dll"); // file 0x248 == RVA 0x1048
        ps(&mut b, 0x260, "TestFunc"); // file 0x260 == RVA 0x1060
        b[0x280] = 0xC3; // file 0x280 == RVA 0x1080: ret
                         // import descriptors at RVA 0x1100 (file 0x300)
        p32(&mut b, 0x300, 0x1160); // original first thunk
        p32(&mut b, 0x30C, 0x1180); // name RVA
        p32(&mut b, 0x310, 0x1170); // first thunk
                                    // ILT at RVA 0x1160 (file 0x360), IAT at RVA 0x1170 (file 0x370)
        p64(&mut b, 0x360, 0x1190);
        p64(&mut b, 0x368, 0);
        p64(&mut b, 0x370, 0x1190);
        p64(&mut b, 0x378, 0);
        ps(&mut b, 0x380, "KERNEL32.dll"); // file 0x380 == RVA 0x1180
        p16(&mut b, 0x390, 0); // hint (file 0x390 == RVA 0x1190)
        ps(&mut b, 0x392, "BaseFunc");
        // TLS callback array at RVA 0x1098 (file 0x298): one callback VA, then NULL
        p64(&mut b, 0x298, TEST_IMAGE_BASE + 0x1080);
        p64(&mut b, 0x2A0, 0);
        // base relocation block at RVA 0x11A0 (file 0x3A0):
        // one DIR64 entry at page offset 0x080
        p32(&mut b, 0x3A0, 0x1000); // page RVA (VirtualAddress)
        p32(&mut b, 0x3A4, 12); // block size
        p16(&mut b, 0x3A8, 0xA080); // type 10 (DIR64) | offset 0x080
                                    // TLS directory at RVA 0x11B0 (file 0x3B0), PE32+ layout (40 bytes)
        p64(&mut b, 0x3B0, TEST_IMAGE_BASE + 0x1000); // start of raw data
        p64(&mut b, 0x3B8, TEST_IMAGE_BASE + 0x1080); // end of raw data
        p64(&mut b, 0x3C0, TEST_IMAGE_BASE + 0x1090); // address of index
        p64(&mut b, 0x3C8, TEST_IMAGE_BASE + 0x1098); // address of callbacks
        p32(&mut b, 0x3D0, 0); // size of zero fill
        p32(&mut b, 0x3D4, 0); // characteristics
                               // delay import descriptor at RVA 0x11E0 (file 0x3E0), 32 bytes,
                               // followed by a zero terminator at 0x400..0x420
        p32(&mut b, 0x3E4, 0x1240); // szName RVA (file 0x440)
        p32(&mut b, 0x3E8, 0x1228); // phmod RVA (file 0x428)
        p32(&mut b, 0x3EC, 0x1230); // pIAT (file 0x430)
        p32(&mut b, 0x3F0, 0x1260); // pINT (file 0x460)
        p32(&mut b, 0x428, 0); // module handle slot (file 0x428 == RVA 0x1228)
        p64(&mut b, 0x430, 0x1250); // delay IAT[0] -> by-name (file 0x430 == RVA 0x1230)
        p64(&mut b, 0x438, 0);
        ps(&mut b, 0x440, "DELAY.dll"); // file 0x440 == RVA 0x1240
        p16(&mut b, 0x450, 0); // hint (file 0x450 == RVA 0x1250)
        ps(&mut b, 0x452, "DelayFunc");
        p64(&mut b, 0x460, 0x1250); // delay INT[0] (file 0x460 == RVA 0x1260)
        p64(&mut b, 0x468, 0);
        b
    }
}

#[cfg(test)]
mod tests {
    use super::testing::build_test_pe64;
    use super::*;

    fn p16(b: &mut [u8], off: usize, value: u16) {
        b[off..off + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn p32(b: &mut [u8], off: usize, value: u32) {
        b[off..off + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn p64(b: &mut [u8], off: usize, value: u64) {
        b[off..off + 8].copy_from_slice(&value.to_le_bytes());
    }

    #[test]
    fn parses_headers() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        assert_eq!(pe.machine(), MACHINE_AMD64);
        assert!(pe.is_64bit());
        assert_eq!(pe.image_base(), testing::TEST_IMAGE_BASE);
        assert_eq!(pe.size_of_image(), testing::TEST_SIZE_OF_IMAGE);
        assert_eq!(pe.entry_point_rva(), 0x1000);
        assert_eq!(pe.subsystem(), 3);
        assert_eq!(pe.sections.len(), 1);
        assert_eq!(pe.sections[0].name(), ".rdata");
    }

    #[test]
    fn maps_rvas() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        assert_eq!(pe.rva_to_offset(0x100), Some(0x100)); // headers
        assert_eq!(pe.rva_to_offset(0x1048), Some(0x248));
        assert_eq!(pe.rva_to_offset(0x2000), None);
    }

    #[test]
    fn reads_exports() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        let exports = pe.exports().unwrap();
        assert_eq!(exports.len(), 1);
        assert_eq!(exports[0].name, "TestFunc");
        assert_eq!(exports[0].ordinal, 1);
        assert_eq!(exports[0].rva, 0x1080);
    }

    #[test]
    fn reads_imports() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        let imports = pe.imports().unwrap();
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].dll, "KERNEL32.dll");
        assert_eq!(imports[0].function, "BaseFunc");
        assert_eq!(imports[0].ordinal, None);
        assert_eq!(imports[0].thunk_rva, 0x1170);
    }

    #[test]
    fn reads_relocations() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        let blocks = pe.relocations().unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].page_rva, 0x1000);
        assert_eq!(
            blocks[0].entries,
            vec![RelocEntry {
                reloc_type: 10,
                offset: 0x080
            }]
        );
    }

    #[test]
    fn reads_tls_directory() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        let tls = pe.tls_directory().unwrap().unwrap();
        assert_eq!(
            tls.start_address_of_raw_data,
            testing::TEST_IMAGE_BASE + 0x1000
        );
        assert_eq!(tls.address_of_call_backs, testing::TEST_IMAGE_BASE + 0x1098);
    }

    #[test]
    fn reads_delay_imports() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        let delay = pe.delay_imports().unwrap();
        assert_eq!(delay.len(), 1);
        assert_eq!(delay[0].dll, "DELAY.dll");
        assert_eq!(delay[0].iat_rva, 0x1230);
        assert_eq!(delay[0].entries.len(), 1);
        assert_eq!(delay[0].entries[0].function, "DelayFunc");
        assert_eq!(delay[0].entries[0].thunk_rva, 0x1230);
    }

    #[test]
    fn empty_directories_parse_as_empty() {
        let mut data = build_test_pe64();
        // Zero out the reloc/TLS/delay directory entries.
        for off in [0x130usize, 0x150, 0x170] {
            data[off..off + 8].fill(0);
        }
        let pe = PeImage::parse(&data).unwrap();
        assert!(pe.relocations().unwrap().is_empty());
        assert!(pe.tls_directory().unwrap().is_none());
        assert!(pe.delay_imports().unwrap().is_empty());
    }

    #[test]
    fn rejects_garbage() {
        assert!(matches!(PeImage::parse(&[]), Err(PeError::Truncated)));
        assert!(matches!(PeImage::parse(b"N"), Err(PeError::Truncated)));
        assert!(matches!(PeImage::parse(b"NO"), Err(PeError::BadDosMagic)));
        let mut bad = build_test_pe64();
        bad[0] = b'X';
        assert!(matches!(PeImage::parse(&bad), Err(PeError::BadDosMagic)));
        bad = build_test_pe64();
        bad[0x80] = 0;
        assert!(matches!(PeImage::parse(&bad), Err(PeError::BadPeSignature)));
        bad = build_test_pe64();
        p16(&mut bad, 0x98, 0x777);
        assert!(matches!(
            PeImage::parse(&bad),
            Err(PeError::UnsupportedMagic(0x777))
        ));
        assert_eq!(PeError::Truncated.to_string(), "file is truncated");
        assert_eq!(
            PeError::BadDosMagic.to_string(),
            "bad DOS magic (expected MZ)"
        );
        assert_eq!(PeError::BadPeSignature.to_string(), "bad PE signature");
        assert_eq!(
            PeError::UnsupportedMagic(0x777).to_string(),
            "unsupported optional-header magic 0x0777"
        );
        assert_eq!(PeError::BadTable.to_string(), "bad or out-of-range table");
    }

    #[test]
    fn parses_pe32_and_machine_names() {
        let mut data = build_test_pe64();
        // PE32 has a 0xe0 optional header and a 32-bit image base.
        p16(&mut data, 0x94, 0xe0);
        p16(&mut data, 0x98, OPT_MAGIC_PE32);
        p32(&mut data, 0xb4, 0x400000);
        p32(&mut data, 0xf4, 16);
        p32(&mut data, 0xf8, 0x1000);
        p32(&mut data, 0xfc, 0x28);
        p32(&mut data, 0x100, 0x1100);
        p32(&mut data, 0x104, 0x28);
        data.copy_within(0x188..0x1b0, 0x178);
        p16(&mut data, 0x84, MACHINE_I386);
        // Convert thunk entries to PE32 DWORD entries.
        p32(&mut data, 0x360, 0x1190);
        p32(&mut data, 0x364, 0);
        p32(&mut data, 0x370, 0x1190);
        p32(&mut data, 0x374, 0);
        let pe = PeImage::parse(&data).unwrap();
        assert!(!pe.is_64bit());
        assert_eq!(pe.image_base(), 0x400000);
        assert_eq!(pe.size_of_headers(), 0x200);
        assert_eq!(pe.subsystem(), 3);
        assert_eq!(machine_name(MACHINE_I386), "i386");
        assert_eq!(machine_name(MACHINE_AMD64), "x86_64");
        assert_eq!(machine_name(MACHINE_ARM64), "aarch64");
        assert_eq!(machine_name(MACHINE_UNKNOWN), "unknown");
        assert_eq!(pe.imports().unwrap()[0].function, "BaseFunc");
    }

    #[test]
    fn rejects_malformed_tables_without_panicking() {
        let mut data = build_test_pe64();
        p32(&mut data, 0x3c, u32::MAX);
        assert!(matches!(PeImage::parse(&data), Err(PeError::Truncated)));

        let mut data = build_test_pe64();
        p16(&mut data, 0x86, u16::MAX);
        assert!(matches!(PeImage::parse(&data), Err(PeError::Truncated)));

        let mut data = build_test_pe64();
        p32(&mut data, 0x238, 0x1fff);
        let pe = PeImage::parse(&data).unwrap();
        assert!(matches!(pe.exports(), Err(PeError::BadTable)));

        let mut data = build_test_pe64();
        p16(&mut data, 0x23c, 2);
        let pe = PeImage::parse(&data).unwrap();
        assert!(matches!(pe.exports(), Err(PeError::BadTable)));

        let mut data = build_test_pe64();
        // Name at RVA 0x11f0 (file 0x3f0) runs to EOF with no NUL terminator.
        p32(&mut data, 0x238, 0x11f0);
        data[0x3f0..].copy_from_slice(&[b'A'; 0x110]);
        let pe = PeImage::parse(&data).unwrap();
        assert!(matches!(pe.exports(), Err(PeError::BadTable)));

        let mut data = build_test_pe64();
        p32(&mut data, 0x110, 0x1fff);
        let pe = PeImage::parse(&data).unwrap();
        assert!(matches!(pe.imports(), Err(PeError::BadTable)));
    }

    #[test]
    fn rva_to_offset_rejects_overflowing_and_unbacked_sections() {
        let mut data = build_test_pe64();
        // Section end overflows u32, so the section must not match.
        p32(&mut data, 0x190, 0x2000);
        p32(&mut data, 0x194, 0xFFFF_F000);
        let pe = PeImage::parse(&data).unwrap();
        assert_eq!(pe.rva_to_offset(0x1048), None);

        let mut data = build_test_pe64();
        // Raw pointer beyond the file end.
        p32(&mut data, 0x19C, 0x10000);
        let pe = PeImage::parse(&data).unwrap();
        assert_eq!(pe.rva_to_offset(0x1048), None);
    }

    #[test]
    fn rejects_every_truncation_point_without_panicking() {
        let full64 = build_test_pe64();
        for len in 0..full64.len() {
            let truncated = &full64[..len];
            if let Ok(pe) = PeImage::parse(truncated) {
                let _ = pe.exports();
                let _ = pe.imports();
            }
        }

        let mut full32 = build_test_pe64();
        p16(&mut full32, 0x94, 0xe0);
        p16(&mut full32, 0x98, OPT_MAGIC_PE32);
        p32(&mut full32, 0xb4, 0x400000);
        p32(&mut full32, 0xf4, 16);
        p32(&mut full32, 0xf8, 0x1000);
        p32(&mut full32, 0xfc, 0x28);
        p32(&mut full32, 0x100, 0x1100);
        p32(&mut full32, 0x104, 0x28);
        full32.copy_within(0x188..0x1b0, 0x178);
        p16(&mut full32, 0x84, MACHINE_I386);
        p32(&mut full32, 0x360, 0x1190);
        p32(&mut full32, 0x364, 0);
        p32(&mut full32, 0x370, 0x1190);
        p32(&mut full32, 0x374, 0);
        for len in 0..full32.len() {
            let truncated = &full32[..len];
            if let Ok(pe) = PeImage::parse(truncated) {
                let _ = pe.exports();
                let _ = pe.imports();
            }
        }
    }

    #[test]
    fn exercises_export_and_import_truncation_paths() {
        // The export name table begins at the last two bytes of the file.
        let mut data = build_test_pe64();
        p32(&mut data, 0x220, 0x12fe);
        assert!(matches!(
            PeImage::parse(&data).unwrap().exports(),
            Err(PeError::Truncated)
        ));

        // The export function table begins at the last two bytes of the file.
        let mut data = build_test_pe64();
        p32(&mut data, 0x21c, 0x12fe);
        assert!(matches!(
            PeImage::parse(&data).unwrap().exports(),
            Err(PeError::Truncated)
        ));

        // A zero OriginalFirstThunk must fall back to FirstThunk.
        let mut data = build_test_pe64();
        p32(&mut data, 0x300, 0);
        assert_eq!(
            PeImage::parse(&data).unwrap().imports().unwrap()[0].function,
            "BaseFunc"
        );

        // 64-bit thunk read straddles EOF (file offset 0x4fc).
        let mut data = build_test_pe64();
        p32(&mut data, 0x300, 0x12fc);
        assert!(matches!(
            PeImage::parse(&data).unwrap().imports(),
            Err(PeError::Truncated)
        ));

        // 32-bit thunk read straddles EOF (file offset 0x4fe).
        let mut data = build_test_pe64();
        p16(&mut data, 0x94, 0xe0);
        p16(&mut data, 0x98, OPT_MAGIC_PE32);
        p32(&mut data, 0xb4, 0x400000);
        p32(&mut data, 0xf4, 16);
        p32(&mut data, 0xf8, 0x1000);
        p32(&mut data, 0xfc, 0x28);
        p32(&mut data, 0x100, 0x1100);
        p32(&mut data, 0x104, 0x28);
        data.copy_within(0x188..0x1b0, 0x178);
        p16(&mut data, 0x84, MACHINE_I386);
        p32(&mut data, 0x300, 0x12fe);
        assert!(matches!(
            PeImage::parse(&data).unwrap().imports(),
            Err(PeError::Truncated)
        ));
    }

    #[test]
    fn rejects_bad_export_and_import_tables() {
        // Name pointer that does not map to any section.
        let mut data = build_test_pe64();
        p32(&mut data, 0x220, 0x1fff);
        assert!(matches!(
            PeImage::parse(&data).unwrap().exports(),
            Err(PeError::BadTable)
        ));

        // Export ordinal table pointing outside the image.
        let mut data = build_test_pe64();
        p32(&mut data, 0x224, 0x1fff);
        assert!(matches!(
            PeImage::parse(&data).unwrap().exports(),
            Err(PeError::BadTable)
        ));

        // Function table pointing outside the image.
        let mut data = build_test_pe64();
        p32(&mut data, 0x21c, 0x1fff);
        assert!(matches!(
            PeImage::parse(&data).unwrap().exports(),
            Err(PeError::BadTable)
        ));

        // Import thunk table pointing outside the image.
        let mut data = build_test_pe64();
        p32(&mut data, 0x300, 0x1fff);
        assert!(matches!(
            PeImage::parse(&data).unwrap().imports(),
            Err(PeError::BadTable)
        ));

        // Import hint/name pointing outside the image.
        let mut data = build_test_pe64();
        p64(&mut data, 0x360, 0x1fff);
        assert!(matches!(
            PeImage::parse(&data).unwrap().imports(),
            Err(PeError::BadTable)
        ));
    }

    #[test]
    fn integer_readers_reject_offsets_past_usize() {
        assert_eq!(u16_at(&[], usize::MAX), Err(PeError::Truncated));
        assert_eq!(u32_at(&[], usize::MAX), Err(PeError::Truncated));
        assert_eq!(u64_at(&[], usize::MAX), Err(PeError::Truncated));
    }

    #[test]
    fn machine_names_cover_all_known_architectures() {
        assert_eq!(machine_name(MACHINE_ARM64), "aarch64");
        assert_eq!(machine_name(0x1234), "unknown");
        assert_eq!(machine_name(MACHINE_UNKNOWN), "unknown");
    }

    #[test]
    fn rva_below_headers_maps_identically() {
        let data = build_test_pe64();
        let pe = PeImage::parse(&data).unwrap();
        assert_eq!(pe.rva_to_offset(0), Some(0));
    }

    #[test]
    fn empty_directories_and_import_ordinals() {
        let mut data = build_test_pe64();
        p32(&mut data, 0x104, 0); // no data directories
        let pe = PeImage::parse(&data).unwrap();
        assert!(pe.directory(0).is_none());
        assert!(pe.exports().unwrap().is_empty());
        assert!(pe.imports().unwrap().is_empty());

        let mut data = build_test_pe64();
        p32(&mut data, 0x108, 0); // empty export directory
        p32(&mut data, 0x110, 0); // empty import directory
        let pe = PeImage::parse(&data).unwrap();
        assert!(pe.exports().unwrap().is_empty());
        assert!(pe.imports().unwrap().is_empty());

        let mut data = build_test_pe64();
        p64(&mut data, 0x360, (1u64 << 63) | 7);
        p64(&mut data, 0x368, 0);
        let pe = PeImage::parse(&data).unwrap();
        let imports = pe.imports().unwrap();
        assert_eq!(imports[0].ordinal, Some(7));
        assert!(imports[0].function.is_empty());
    }

    #[test]
    fn section_name_handles_full_and_invalid_utf8_names() {
        let full = SectionHeader {
            name: *b"12345678",
            virtual_size: 0,
            virtual_address: 0,
            size_of_raw_data: 0,
            pointer_to_raw_data: 0,
            characteristics: 0,
        };
        assert_eq!(full.name(), "12345678");
        let invalid = SectionHeader {
            name: [0xff, 0, 0, 0, 0, 0, 0, 0],
            ..full
        };
        assert_eq!(invalid.name(), "");
    }
}
