//! Port of `dlls/mscms/icc.c`: ICC header and tag table access over the profile bytes.
//!
//! The header and tag table are big-endian on disk. Wine byte-swaps every 32-bit word on the
//! way in and out, including the 44 reserved bytes, so the port does the same.

use crate::Profile;

/// `sizeof(PROFILEHEADER)`.
pub const PROFILE_HEADER_SIZE: usize = 128;
/// Offset of `phReserved` inside the header.
pub const RESERVED_OFFSET: usize = 84;
/// `sizeof(struct tag_entry)`.
pub const TAG_ENTRY_SIZE: usize = 12;
/// Offset of the tag count, which directly follows the header. The first tag entry follows it.
pub const TAG_TABLE_OFFSET: usize = PROFILE_HEADER_SIZE + 4;

/// `PROFILEHEADER` in host order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProfileHeader {
    pub size: u32,
    pub cmm_type: u32,
    pub version: u32,
    pub class: u32,
    pub data_color_space: u32,
    pub connection_space: u32,
    pub date_time: [u32; 3],
    pub signature: u32,
    pub platform: u32,
    pub profile_flags: u32,
    pub manufacturer: u32,
    pub model: u32,
    pub attributes: [u32; 2],
    pub rendering_intent: u32,
    pub illuminant: [u32; 3],
    pub creator: u32,
    pub reserved: [u8; 44],
}

/// One `struct tag_entry` in host order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagEntry {
    pub sig: u32,
    pub offset: u32,
    pub size: u32,
}

/// The data of a tag, as returned by [`Profile::tag_data`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TagRead<'a> {
    pub data: &'a [u8],
    /// Whether another tag entry shares this tag's data extent.
    pub linked: bool,
}

/// Why a tag access failed. These are the `FALSE` returns of `get_tag_data` and `set_tag_data`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagError {
    /// No tag has the signature, or its data extent runs past the profile.
    Missing,
    /// The offset lies past the end of the tag.
    OutOfRange,
    /// The caller's buffer holds fewer than `needed` bytes.
    TooSmall { needed: u32 },
}

impl ProfileHeader {
    /// Decodes a header stored in host (little-endian) word order, with the reserved bytes raw.
    pub fn from_native(bytes: &[u8; PROFILE_HEADER_SIZE]) -> Self {
        let word = |i: usize| {
            u32::from_le_bytes([
                bytes[4 * i],
                bytes[4 * i + 1],
                bytes[4 * i + 2],
                bytes[4 * i + 3],
            ])
        };
        let mut reserved = [0; 44];
        reserved.copy_from_slice(&bytes[RESERVED_OFFSET..]);
        Self {
            size: word(0),
            cmm_type: word(1),
            version: word(2),
            class: word(3),
            data_color_space: word(4),
            connection_space: word(5),
            date_time: [word(6), word(7), word(8)],
            signature: word(9),
            platform: word(10),
            profile_flags: word(11),
            manufacturer: word(12),
            model: word(13),
            attributes: [word(14), word(15)],
            rendering_intent: word(16),
            illuminant: [word(17), word(18), word(19)],
            creator: word(20),
            reserved,
        }
    }

    /// Encodes the header in host (little-endian) word order, with the reserved bytes raw.
    pub fn to_native(&self) -> [u8; PROFILE_HEADER_SIZE] {
        let words = [
            self.size,
            self.cmm_type,
            self.version,
            self.class,
            self.data_color_space,
            self.connection_space,
            self.date_time[0],
            self.date_time[1],
            self.date_time[2],
            self.signature,
            self.platform,
            self.profile_flags,
            self.manufacturer,
            self.model,
            self.attributes[0],
            self.attributes[1],
            self.rendering_intent,
            self.illuminant[0],
            self.illuminant[1],
            self.illuminant[2],
            self.creator,
        ];
        let mut bytes = [0; PROFILE_HEADER_SIZE];
        for (at, word) in (0..PROFILE_HEADER_SIZE).step_by(4).zip(words) {
            bytes[at..at + 4].copy_from_slice(&word.to_le_bytes());
        }
        bytes[RESERVED_OFFSET..].copy_from_slice(&self.reserved);
        bytes
    }
}

impl Profile {
    /// Decodes the header (`get_profile_header`).
    pub fn header(&self) -> ProfileHeader {
        let mut bytes = [0; PROFILE_HEADER_SIZE];
        bytes.copy_from_slice(&self.data[..PROFILE_HEADER_SIZE]);
        swap_words(&mut bytes);
        ProfileHeader::from_native(&bytes)
    }

    /// Encodes and stores the header (`set_profile_header`).
    pub fn set_header(&mut self, header: &ProfileHeader) {
        let mut bytes = header.to_native();
        swap_words(&mut bytes);
        self.data[..PROFILE_HEADER_SIZE].copy_from_slice(&bytes);
    }

    /// The tag count, or 0 when the tag table runs past the end of the profile (`get_tag_count`).
    pub fn tag_count(&self) -> u32 {
        let count = self.be_u32(PROFILE_HEADER_SIZE);
        let table_end = TAG_TABLE_OFFSET as u64 + u64::from(count) * TAG_ENTRY_SIZE as u64;
        if table_end > self.data.len() as u64 {
            0
        } else {
            count
        }
    }

    /// The tag entry at the 1-based `index`, if the entry is valid and its data lies inside the
    /// profile (`get_tag_entry`).
    pub fn tag_entry(&self, index: u32) -> Option<TagEntry> {
        if index == 0 || index > self.tag_count() {
            return None;
        }
        let entry = self.raw_entry(index as usize - 1);
        self.in_image(&entry).then_some(entry)
    }

    /// The first tag entry with `sig`, if its data lies inside the profile (`get_adjusted_tag`).
    /// Only the first match is considered, even when it is out of range.
    pub fn adjusted_tag(&self, sig: u32) -> Option<TagEntry> {
        let entry = self.entries().find(|entry| entry.sig == sig)?;
        self.in_image(&entry).then_some(entry)
    }

    /// Reads the tag's data from `offset` on (`get_tag_data`). `capacity` is the caller's buffer
    /// in bytes; `None` models a NULL buffer, which forces the offset to 0 and always reports
    /// the full size as needed.
    pub fn tag_data(
        &self,
        sig: u32,
        offset: u32,
        capacity: Option<u32>,
    ) -> Result<TagRead<'_>, TagError> {
        let tag = self.adjusted_tag(sig).ok_or(TagError::Missing)?;
        let offset = if capacity.is_some() { offset } else { 0 };
        if offset > tag.size {
            return Err(TagError::OutOfRange);
        }
        let needed = tag.size - offset;
        match capacity {
            Some(capacity) if capacity >= needed => {}
            _ => return Err(TagError::TooSmall { needed }),
        }
        let start = tag.offset as usize + offset as usize;
        let end = tag.offset as usize + tag.size as usize;
        Ok(TagRead {
            data: &self.data[start..end],
            linked: self.is_linked(&tag),
        })
    }

    /// Writes `src` into the tag at `offset`, clipped to the tag's size. Returns the number of
    /// bytes written (`set_tag_data`).
    pub fn set_tag_data(&mut self, sig: u32, offset: u32, src: &[u8]) -> Result<u32, TagError> {
        let tag = self.adjusted_tag(sig).ok_or(TagError::Missing)?;
        if offset > tag.size {
            return Err(TagError::OutOfRange);
        }
        let len = ((tag.size - offset) as usize).min(src.len());
        let start = tag.offset as usize + offset as usize;
        self.data[start..start + len].copy_from_slice(&src[..len]);
        Ok(len as u32)
    }

    fn be_u32(&self, at: usize) -> u32 {
        u32::from_be_bytes([
            self.data[at],
            self.data[at + 1],
            self.data[at + 2],
            self.data[at + 3],
        ])
    }

    fn raw_entry(&self, index: usize) -> TagEntry {
        let at = TAG_TABLE_OFFSET + index * TAG_ENTRY_SIZE;
        TagEntry {
            sig: self.be_u32(at),
            offset: self.be_u32(at + 4),
            size: self.be_u32(at + 8),
        }
    }

    fn entries(&self) -> impl Iterator<Item = TagEntry> + '_ {
        (0..self.tag_count() as usize).map(move |index| self.raw_entry(index))
    }

    fn in_image(&self, entry: &TagEntry) -> bool {
        let len = self.data.len() as u64;
        u64::from(entry.offset) <= len && u64::from(entry.size) <= len - u64::from(entry.offset)
    }

    fn is_linked(&self, tag: &TagEntry) -> bool {
        self.entries().any(|entry| {
            entry.sig != tag.sig && entry.offset == tag.offset && entry.size == tag.size
        })
    }
}

fn swap_words(bytes: &mut [u8; PROFILE_HEADER_SIZE]) {
    for at in (0..PROFILE_HEADER_SIZE).step_by(4) {
        bytes[at..at + 4].reverse();
    }
}
