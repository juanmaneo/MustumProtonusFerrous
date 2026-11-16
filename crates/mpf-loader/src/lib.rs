//! PE mapper: mirrors the first stage of `loader/` + `dlls/ntdll/loader.c`
//! in reference Wine — parse the image, then lay headers and sections out in
//! a flat, section-aligned buffer (before relocations/imports are applied).

use mpf_pe::{PeError, PeImage};
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

pub struct MemoryModule {
    image: Vec<u8>,
    pub size_of_image: u32,
    pub entry_point_rva: u32,
    pub image_base: u64,
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
}

/// Maps a PE file into a flat buffer of `size_of_image` bytes with headers
/// at RVA 0 and each section at its `virtual_address`.
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
    Ok(MemoryModule {
        image,
        size_of_image: size,
        entry_point_rva: pe.entry_point_rva(),
        image_base: pe.image_base(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpf_pe::testing::build_test_pe64;

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
}
