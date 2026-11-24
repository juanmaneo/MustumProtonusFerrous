//! Metadata helpers from `dlls/fusion/assembly.c`: the compressed blob length prefix (`GetData`)
//! and the RVA to file offset mapping (`rva_to_offset`).

/// One `IMAGE_SECTION_HEADER`, with the fields the RVA mapping reads.
pub struct Section {
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub virtual_size: u32,
}

/// `GetData`: decodes the compressed length at the start of a blob and returns it with the bytes
/// after the prefix. `None` is the C's NULL, for a lead byte of 0xE0 or more or for a prefix cut
/// short by the end of `data`. The C reads past the end in the cut-short case, so the port has no
/// C result to match there.
pub fn decode_blob_length(data: &[u8]) -> Option<(u32, &[u8])> {
    let lead = *data.first()?;
    let (width, length) = if lead & 0x80 == 0 {
        (1, u32::from(lead & 0x7F))
    } else if lead & 0xC0 == 0x80 {
        let b = data.get(..2)?;
        (2, (u32::from(b[0] & 0x3F) << 8) | u32::from(b[1]))
    } else if lead & 0xE0 == 0xC0 {
        let b = data.get(..4)?;
        (
            4,
            (u32::from(b[0] & 0x1F) << 24)
                | (u32::from(b[1]) << 16)
                | (u32::from(b[2]) << 8)
                | u32::from(b[3]),
        )
    } else {
        return None;
    };
    Some((length, &data[width..]))
}

/// `rva_to_offset`: maps an RVA to a file offset through the section table. The early return
/// tests the first section's `PointerToRawData`, as the C does. Section ends are computed in
/// 32 bits, so a section that runs past 0xFFFFFFFF matches no RVA.
pub fn rva_to_offset(sections: &[Section], rva: u32) -> u32 {
    if sections
        .first()
        .is_some_and(|first| rva < first.pointer_to_raw_data)
    {
        return rva;
    }
    for section in sections {
        let limit = if section.size_of_raw_data != 0 {
            section.size_of_raw_data
        } else {
            section.virtual_size
        };
        if rva >= section.virtual_address && rva < section.virtual_address.wrapping_add(limit) {
            if section.pointer_to_raw_data == 0 {
                return rva;
            }
            return rva
                .wrapping_sub(section.virtual_address)
                .wrapping_add(section.pointer_to_raw_data);
        }
    }
    0
}
