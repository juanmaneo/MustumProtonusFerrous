//! PE mapper + loader stage: mirrors `loader/` + `dlls/ntdll/loader.c` in
//! reference Wine — parse the image, lay headers and sections out in a flat
//! buffer, then apply base relocations and resolve imports into the IAT.

use mpf_pe::{DelayImport, ExportEntry, ImportEntry, PeError, PeImage, RelocBlock, TlsDirectory};
use std::fmt;

#[derive(Debug)]
pub enum LoadError {
    Parse(PeError),
    EmptyImage,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Parse(e) => write!(f, "PE parse failed: {e}"),
            LoadError::EmptyImage => write!(f, "image has size 0"),
        }
    }
}

impl std::error::Error for LoadError {}

/// Resolves an import to a native address, mirroring the role of
/// `find_builtin_dll`/`get_builtin_entry` in ntdll's loader.
pub type ImportResolver<'a> = dyn FnMut(&str, &str, Option<u16>) -> Option<u64> + 'a;

pub struct MemoryModule {
    image: Vec<u8>,
    pub size_of_image: u32,
    pub entry_point_rva: u32,
    /// Preferred base from the PE header; `image_base` tracks the actual base
    /// after relocation.
    pub preferred_base: u64,
    pub image_base: u64,
    pub is_64bit: bool,
    pub relocations: Vec<RelocBlock>,
    pub tls: Option<TlsDirectory>,
    pub tls_callbacks: Vec<u32>,
    pub delay_imports: Vec<DelayImport>,
    pub imports: Vec<ImportEntry>,
    pub exports: Vec<ExportEntry>,
}

impl MemoryModule {
    /// Address of the mapped image. Stable for the life of the module:
    /// the buffer is heap-allocated by the `Vec`, so moving the struct does
    /// not move the image.
    pub fn base(&self) -> usize {
        self.image.as_ptr() as usize
    }

    pub fn image(&self) -> &[u8] {
        &self.image
    }

    pub fn image_mut(&mut self) -> &mut [u8] {
        &mut self.image
    }

    fn rva_index(&self, rva: u32) -> Option<usize> {
        let off = rva as usize;
        (off < self.image.len()).then_some(off)
    }

    /// Applies base relocations for `new_base` (HIGHLOW and DIR64 entries),
    /// mirroring `relocate_image` in ntdll. Returns the number of entries
    /// patched. Updates `image_base`.
    pub fn relocate(&mut self, new_base: u64) -> usize {
        let delta = new_base.wrapping_sub(self.image_base);
        let mut count = 0;
        for block in &self.relocations {
            for e in &block.entries {
                let rva = block.page_rva.wrapping_add(u32::from(e.offset));
                let Some(off) = self.rva_index(rva) else {
                    continue;
                };
                match e.reloc_type {
                    3 if off + 4 <= self.image.len() => {
                        let v = u32::from_le_bytes(self.image[off..off + 4].try_into().unwrap());
                        self.image[off..off + 4]
                            .copy_from_slice(&v.wrapping_add(delta as u32).to_le_bytes());
                        count += 1;
                    }
                    10 if off + 8 <= self.image.len() => {
                        let v = u64::from_le_bytes(self.image[off..off + 8].try_into().unwrap());
                        self.image[off..off + 8]
                            .copy_from_slice(&v.wrapping_add(delta).to_le_bytes());
                        count += 1;
                    }
                    _ => {}
                }
            }
        }
        self.image_base = new_base;
        count
    }

    fn write_thunk(&mut self, thunk_rva: u32, addr: u64) -> bool {
        let Some(off) = self.rva_index(thunk_rva) else {
            return false;
        };
        if self.is_64bit {
            if off + 8 <= self.image.len() {
                self.image[off..off + 8].copy_from_slice(&addr.to_le_bytes());
                true
            } else {
                false
            }
        } else if off + 4 <= self.image.len() {
            self.image[off..off + 4].copy_from_slice(&(addr as u32).to_le_bytes());
            true
        } else {
            false
        }
    }

    /// Resolves the import table into the IAT, mirroring `resolve_imports` in
    /// ntdll's loader. Returns the number of thunks patched.
    pub fn resolve_imports(&mut self, resolver: &mut ImportResolver) -> usize {
        let mut patches = Vec::new();
        for imp in &self.imports {
            if let Some(addr) = resolver(&imp.dll, &imp.function, imp.ordinal) {
                patches.push((imp.thunk_rva, addr));
            }
        }
        let mut count = 0;
        for (rva, addr) in patches {
            if self.write_thunk(rva, addr) {
                count += 1;
            }
        }
        count
    }

    /// Resolves delay-load imports (normally done lazily on first call, as
    /// `delay_load` in ntdll does; here eagerly). Returns patched count.
    pub fn resolve_delay_imports(&mut self, resolver: &mut ImportResolver) -> usize {
        let mut patches = Vec::new();
        for delay in &self.delay_imports {
            for imp in &delay.entries {
                if let Some(addr) = resolver(&delay.dll, &imp.function, imp.ordinal) {
                    patches.push((imp.thunk_rva, addr));
                }
            }
        }
        let mut count = 0;
        for (rva, addr) in patches {
            if self.write_thunk(rva, addr) {
                count += 1;
            }
        }
        count
    }
}

/// Maps a PE file into a flat buffer of `size_of_image` bytes with headers
/// at RVA 0 and each section at its `virtual_address`, and parses the
/// loader-relevant tables (relocations, TLS, imports, delay imports).
pub fn map_image(data: &[u8]) -> Result<MemoryModule, LoadError> {
    let pe = PeImage::parse(data).map_err(LoadError::Parse)?;
    let size = pe.size_of_image();
    if size == 0 {
        return Err(LoadError::EmptyImage);
    }
    let mut image = vec![0u8; size as usize];
    let headers = (pe.size_of_headers() as usize)
        .min(data.len())
        .min(size as usize);
    image[..headers].copy_from_slice(&data[..headers]);
    for s in &pe.sections {
        let raw = s.pointer_to_raw_data as usize;
        let raw_size = s.size_of_raw_data as usize;
        if raw_size == 0 {
            continue;
        }
        let available = data.len().saturating_sub(raw);
        let src = &data[raw..raw + raw_size.min(available)];
        let dst_off = s.virtual_address as usize;
        if dst_off >= image.len() {
            continue;
        }
        let n = src.len().min(image.len() - dst_off);
        image[dst_off..dst_off + n].copy_from_slice(&src[..n]);
    }
    let relocations = pe.relocations().map_err(LoadError::Parse)?;
    let tls = pe.tls_directory().map_err(LoadError::Parse)?;
    let delay_imports = pe.delay_imports().map_err(LoadError::Parse)?;
    let imports = pe.imports().map_err(LoadError::Parse)?;
    let exports = pe.exports().map_err(LoadError::Parse)?;
    // TLS callback array: VAs converted to RVAs against the preferred base.
    let mut tls_callbacks = Vec::new();
    if let Some(t) = &tls {
        let mut rva = t.address_of_call_backs.wrapping_sub(pe.image_base()) as u32;
        loop {
            let off = pe
                .rva_to_offset(rva)
                .ok_or(PeError::BadTable)
                .map_err(LoadError::Parse)?;
            let next = if pe.is_64bit() {
                u64_at_pub(pe.raw_data(), off).map_err(LoadError::Parse)?
            } else {
                u32_at_pub(pe.raw_data(), off).map_err(LoadError::Parse)? as u64
            };
            if next == 0 {
                break;
            }
            tls_callbacks.push(next.wrapping_sub(pe.image_base()) as u32);
            rva = rva.wrapping_add(if pe.is_64bit() { 8 } else { 4 });
        }
    }
    Ok(MemoryModule {
        image,
        size_of_image: size,
        entry_point_rva: pe.entry_point_rva(),
        preferred_base: pe.image_base(),
        image_base: pe.image_base(),
        is_64bit: pe.is_64bit(),
        relocations,
        tls,
        tls_callbacks,
        delay_imports,
        imports,
        exports,
    })
}

fn u64_at_pub(d: &[u8], o: usize) -> Result<u64, PeError> {
    let b = d
        .get(o..)
        .and_then(|s| s.get(..8))
        .ok_or(PeError::Truncated)?;
    Ok(u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

fn u32_at_pub(d: &[u8], o: usize) -> Result<u32, PeError> {
    let b = d
        .get(o..)
        .and_then(|s| s.get(..4))
        .ok_or(PeError::Truncated)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpf_pe::testing::{build_test_pe32, build_test_pe64, TEST_IMAGE_BASE};
    use mpf_pe::RelocEntry;

    fn put32(data: &mut [u8], off: usize, v: u32) {
        data[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn put64(data: &mut [u8], off: usize, v: u64) {
        data[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn import_at(thunk_rva: u32) -> ImportEntry {
        ImportEntry {
            dll: "X.dll".into(),
            function: "F".into(),
            ordinal: None,
            thunk_rva,
        }
    }

    fn reloc(page_rva: u32, entries: &[(u8, u16)]) -> RelocBlock {
        RelocBlock {
            page_rva,
            entries: entries
                .iter()
                .map(|&(reloc_type, offset)| RelocEntry { reloc_type, offset })
                .collect(),
        }
    }

    #[test]
    fn maps_synthetic_image() {
        let data = build_test_pe64();
        let m = map_image(&data).unwrap();
        assert_eq!(m.size_of_image, 0x2000);
        assert_eq!(m.entry_point_rva, 0x1000);
        assert_eq!(m.image_base, mpf_pe::testing::TEST_IMAGE_BASE);
        assert_eq!(m.image().len(), 0x2000);
        // headers copied: MZ at RVA 0
        assert_eq!(&m.image()[..2], b"MZ");
        // section content landed at its RVA: export dir at 0x1000,
        // code byte 0xC3 at RVA 0x1080, dll name at RVA 0x1048
        assert_eq!(m.image()[0x1080], 0xC3);
        assert_eq!(&m.image()[0x1048..0x1048 + 12], b"mpf_test.dll");
    }

    #[test]
    fn rejects_non_pe() {
        assert!(matches!(map_image(b"not a pe"), Err(LoadError::Parse(_))));
    }

    #[test]
    fn reports_errors_and_keeps_image_pointer_stable() {
        let err = LoadError::EmptyImage;
        assert_eq!(err.to_string(), "image has size 0");
        let err = LoadError::Parse(PeError::BadDosMagic);
        assert_eq!(
            err.to_string(),
            "PE parse failed: bad DOS magic (expected MZ)"
        );

        let mut data = build_test_pe64();
        data[0xD0..0xD4].copy_from_slice(&0u32.to_le_bytes());
        assert!(matches!(map_image(&data), Err(LoadError::EmptyImage)));

        let m = map_image(&build_test_pe64()).unwrap();
        let base = m.base();
        let moved = m;
        assert_eq!(moved.base(), base);
        assert_eq!(moved.image().as_ptr() as usize, base);
    }

    #[test]
    fn ignores_empty_and_out_of_range_sections() {
        let mut data = build_test_pe64();
        // Two extra sections: one whose VA lies past the image end, and one
        // with a zero raw size.
        data[0x86..0x88].copy_from_slice(&3u16.to_le_bytes());
        let section2 = 0x188 + 40;
        data[section2..section2 + 8].copy_from_slice(b".outside");
        data[section2 + 12..section2 + 16].copy_from_slice(&0x2000u32.to_le_bytes());
        data[section2 + 16..section2 + 20].copy_from_slice(&0x200u32.to_le_bytes());
        data[section2 + 20..section2 + 24].copy_from_slice(&0x200u32.to_le_bytes());
        let section3 = section2 + 40;
        data[section3..section3 + 8].copy_from_slice(b".zero\0\0\0");
        data[section3 + 12..section3 + 16].copy_from_slice(&0x1800u32.to_le_bytes());
        let module = map_image(&data).unwrap();
        assert_eq!(module.image().len(), 0x2000);
    }

    #[test]
    fn parses_loader_tables() {
        let m = map_image(&build_test_pe64()).unwrap();
        assert_eq!(m.relocations.len(), 1);
        assert!(m.tls.is_some());
        assert_eq!(m.tls_callbacks, vec![0x1080]);
        assert_eq!(m.delay_imports.len(), 1);
        assert_eq!(m.imports.len(), 1);
        assert_eq!(m.exports.len(), 1);
        assert!(m.is_64bit);
    }

    #[test]
    fn relocates_dir64_entries() {
        let data = build_test_pe64();
        let mut m = map_image(&data).unwrap();
        let new_base = 0x5000_0000u64;
        let delta = new_base.wrapping_sub(m.image_base);
        let applied = m.relocate(new_base);
        assert_eq!(applied, 1);
        assert_eq!(m.image_base, new_base);
        let v = u64::from_le_bytes(m.image()[0x1080..0x1088].try_into().unwrap());
        assert_eq!(v, 0xC3u64.wrapping_add(delta));
    }

    #[test]
    fn resolves_imports_into_iat() {
        let data = build_test_pe64();
        let mut m = map_image(&data).unwrap();
        let mut resolver = |dll: &str, func: &str, ord: Option<u16>| {
            assert_eq!(dll, "KERNEL32.dll");
            assert_eq!(func, "BaseFunc");
            assert_eq!(ord, None);
            Some(0xDEAD_BEEF)
        };
        let n = m.resolve_imports(&mut resolver);
        assert_eq!(n, 1);
        let v = u64::from_le_bytes(m.image()[0x1170..0x1178].try_into().unwrap());
        assert_eq!(v, 0xDEAD_BEEF);
    }

    #[test]
    fn resolves_delay_imports_into_delay_iat() {
        let data = build_test_pe64();
        let mut m = map_image(&data).unwrap();
        let mut resolver = |dll: &str, func: &str, _ord: Option<u16>| {
            assert_eq!(dll, "DELAY.dll");
            assert_eq!(func, "DelayFunc");
            Some(0x1234)
        };
        let n = m.resolve_delay_imports(&mut resolver);
        assert_eq!(n, 1);
        let v = u64::from_le_bytes(m.image()[0x1230..0x1238].try_into().unwrap());
        assert_eq!(v, 0x1234);
    }

    #[test]
    fn image_mut_patches_the_mapped_image() {
        let mut m = map_image(&build_test_pe64()).unwrap();
        m.image_mut()[0x1080] = 0x90;
        assert_eq!(m.image()[0x1080], 0x90);
    }

    #[test]
    fn relocates_highlow_entries_of_a_pe32_image() {
        let mut m = map_image(&build_test_pe32()).unwrap();
        assert!(!m.is_64bit);
        assert_eq!(m.preferred_base, 0x40_0000);
        let applied = m.relocate(0x50_0000);
        assert_eq!(applied, 1);
        assert_eq!(m.image_base, 0x50_0000);
        let v = u32::from_le_bytes(m.image()[0x1080..0x1084].try_into().unwrap());
        assert_eq!(v, 0xC3 + 0x10_0000);
    }

    #[test]
    fn relocation_skips_entries_it_cannot_apply() {
        let mut m = map_image(&build_test_pe64()).unwrap();
        let end = m.image().len() as u32;
        m.relocations = vec![
            // Page beyond the image.
            reloc(end, &[(10, 0), (3, 0)]),
            // HIGHLOW with fewer than 4 bytes left, DIR64 with fewer than 8.
            reloc(end - 0x1000, &[(3, 0xFFE), (10, 0xFFC)]),
            // HIGH (type 1) is not applied; ABSOLUTE is padding.
            reloc(0x1000, &[(1, 0x100), (0, 0x100)]),
            // One entry of each kind that does apply.
            reloc(0x1000, &[(3, 0x200), (10, 0x300)]),
        ];
        assert_eq!(m.relocate(TEST_IMAGE_BASE + 0x1_0000), 2);
        let hl = u32::from_le_bytes(m.image()[0x1200..0x1204].try_into().unwrap());
        assert_eq!(hl, 0x1_0000);
        let d64 = u64::from_le_bytes(m.image()[0x1300..0x1308].try_into().unwrap());
        assert_eq!(d64, 0x1_0000);
    }

    #[test]
    fn resolves_32_bit_thunks() {
        let mut m = map_image(&build_test_pe32()).unwrap();
        let mut resolver = |_: &str, _: &str, _: Option<u16>| Some(0x1_2345_6789);
        assert_eq!(m.resolve_imports(&mut resolver), 1);
        // Only the low 32 bits fit in a PE32 thunk; the next slot is untouched.
        let v = u32::from_le_bytes(m.image()[0x1170..0x1174].try_into().unwrap());
        assert_eq!(v, 0x2345_6789);
        let next = u32::from_le_bytes(m.image()[0x1174..0x1178].try_into().unwrap());
        assert_eq!(next, 0);
    }

    #[test]
    fn thunks_outside_the_image_are_not_counted() {
        let mut resolver = |_: &str, _: &str, _: Option<u16>| Some(0xABCD);

        // 64-bit: past the end, and too close to the end for 8 bytes.
        let mut m = map_image(&build_test_pe64()).unwrap();
        let end = m.image().len() as u32;
        m.imports = vec![import_at(end), import_at(end - 4), import_at(0x1170)];
        assert_eq!(m.resolve_imports(&mut resolver), 1);

        // 32-bit: past the end, and too close to the end for 4 bytes.
        let mut m = map_image(&build_test_pe32()).unwrap();
        let end = m.image().len() as u32;
        m.imports = vec![import_at(end), import_at(end - 2), import_at(0x1170)];
        assert_eq!(m.resolve_imports(&mut resolver), 1);
    }

    #[test]
    fn delay_thunks_outside_the_image_are_not_counted() {
        let mut m = map_image(&build_test_pe64()).unwrap();
        let end = m.image().len() as u32;
        m.delay_imports = vec![mpf_pe::DelayImport {
            dll: "DELAY.dll".into(),
            module_handle_rva: 0,
            iat_rva: 0,
            int_rva: 0,
            entries: vec![import_at(end), import_at(0x1230)],
        }];
        let mut resolver = |_: &str, _: &str, _: Option<u16>| Some(0xABCD);
        assert_eq!(m.resolve_delay_imports(&mut resolver), 1);
    }

    #[test]
    fn maps_images_without_a_tls_directory() {
        let mut data = build_test_pe64();
        put32(&mut data, 0x150, 0); // TLS directory RVA
        let m = map_image(&data).unwrap();
        assert!(m.tls.is_none());
        assert!(m.tls_callbacks.is_empty());
    }

    #[test]
    fn parses_pe32_tls_callbacks() {
        let m = map_image(&build_test_pe32()).unwrap();
        assert_eq!(m.tls_callbacks, vec![0x1080]);
        let tls = m.tls.unwrap();
        assert_eq!(tls.address_of_call_backs, 0x40_1098);
    }

    #[test]
    fn propagates_errors_from_every_loader_table() {
        // Directory table offsets: export, import, base reloc, TLS, delay
        // import. Each is pointed outside the image in turn.
        for dir_offset in [0x108usize, 0x110, 0x130, 0x150, 0x170] {
            let mut data = build_test_pe64();
            put32(&mut data, dir_offset, 0x1fff);
            assert!(
                matches!(map_image(&data), Err(LoadError::Parse(PeError::BadTable))),
                "directory at {dir_offset:#x}"
            );
        }
    }

    #[test]
    fn rejects_tls_callback_arrays_that_leave_the_file() {
        // 64-bit: the array starts 4 bytes before the end of the file.
        let mut data = build_test_pe64();
        put64(&mut data, 0x3c8, TEST_IMAGE_BASE + 0x12fc);
        assert!(matches!(
            map_image(&data),
            Err(LoadError::Parse(PeError::Truncated))
        ));

        // 32-bit: the array starts 2 bytes before the end of the file.
        let mut data = build_test_pe32();
        put32(&mut data, 0x3bc, 0x40_0000 + 0x12fe);
        assert!(matches!(
            map_image(&data),
            Err(LoadError::Parse(PeError::Truncated))
        ));

        // The array address maps to no section.
        let mut data = build_test_pe64();
        put64(&mut data, 0x3c8, TEST_IMAGE_BASE + 0x1fff);
        assert!(matches!(
            map_image(&data),
            Err(LoadError::Parse(PeError::BadTable))
        ));
    }

    #[test]
    fn unresolved_imports_leave_thunk_untouched() {
        let data = build_test_pe64();
        let mut m = map_image(&data).unwrap();
        let mut resolver = |_: &str, _: &str, _: Option<u16>| None;
        assert_eq!(m.resolve_imports(&mut resolver), 0);
        let v = u64::from_le_bytes(m.image()[0x1170..0x1178].try_into().unwrap());
        assert_eq!(v, 0x1190); // original ILT value still there

        assert_eq!(m.resolve_delay_imports(&mut resolver), 0);
        let v = u64::from_le_bytes(m.image()[0x1230..0x1238].try_into().unwrap());
        assert_ne!(v, 0x1234); // delay IAT slot is not patched
    }
}
