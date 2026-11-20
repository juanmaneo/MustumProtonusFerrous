//! Maps a relative virtual address to a file offset through the PE section table (C
//! `pe_rva_to_offset` in `assembly.c`). Only the section fields the mapping reads are modelled;
//! reading the headers from a file is not ported.

/// The fields of an `IMAGE_SECTION_HEADER` that the RVA mapping uses.
pub struct Section {
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub raw_pointer: u32,
}

/// Returns the file offset for `rva` in the first section that contains it (C
/// `pe_rva_to_offset`). The section end and the offset are computed in 32 bits, so they wrap as
/// in C.
pub fn rva_to_offset(sections: &[Section], rva: u32) -> Option<u32> {
    for section in sections {
        let end = section.virtual_address.wrapping_add(section.virtual_size);
        if rva >= section.virtual_address && rva < end {
            return Some(
                rva.wrapping_sub(section.virtual_address)
                    .wrapping_add(section.raw_pointer),
            );
        }
    }
    None
}
