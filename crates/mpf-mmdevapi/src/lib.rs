//! Pure audio-format decisions from Wine's `dlls/mmdevapi`.
//!
//! This crate ports the bytewise format comparison and static spatial-object channel mapping from
//! `spatialaudio.c`. It intentionally excludes device, COM, handle, and allocation behavior.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The byte size of a Windows `WAVEFORMATEX` header.
pub const WAVEFORMATEX_HEADER_SIZE: usize = 18;

/// Number of static spatial object types mapped by Wine.
pub const STATIC_OBJECT_TYPE_COUNT: usize = 13;

const OBJECT_TO_SPEAKER: [(u32, u32); STATIC_OBJECT_TYPE_COUNT] = [
    (0x0000_0002, 0x0000_0001), // Front left
    (0x0000_0004, 0x0000_0002), // Front right
    (0x0000_0008, 0x0000_0004), // Front center
    (0x0000_0010, 0x0000_0008), // Low frequency
    (0x0000_0020, 0x0000_0200), // Side left
    (0x0000_0040, 0x0000_0400), // Side right
    (0x0000_0080, 0x0000_0010), // Back left
    (0x0000_0100, 0x0000_0020), // Back right
    (0x0000_0200, 0x0000_1000), // Top front left
    (0x0000_0400, 0x0000_4000), // Top front right
    (0x0000_0800, 0x0000_8000), // Top back left
    (0x0000_1000, 0x0002_0000), // Top back right
    (0x0002_0000, 0x0000_0100), // Back center
];

/// The channel count, speaker mask, and output-channel index for each static object type.
///
/// A channel index of `u32::MAX` means that the corresponding object type was absent. The array
/// order follows Wine's `static_mask_to_channels` order, not numeric speaker-mask order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticChannelMap {
    pub channel_count: u16,
    pub speaker_mask: u32,
    pub channel_indices: [u32; STATIC_OBJECT_TYPE_COUNT],
}

/// Converts a spatial static-object mask to a speaker mask and channel indices.
///
/// Object bits not handled by Wine's mapping (including dynamic and bottom speakers) are ignored.
pub fn static_mask_to_channels(static_mask: u32) -> StaticChannelMap {
    let mut result = StaticChannelMap {
        channel_count: 0,
        speaker_mask: 0,
        channel_indices: [u32::MAX; STATIC_OBJECT_TYPE_COUNT],
    };

    for (index, (object_type, speaker)) in OBJECT_TO_SPEAKER.iter().enumerate() {
        if static_mask & object_type != 0 {
            result.channel_indices[index] = u32::from(result.channel_count);
            result.channel_count += 1;
            result.speaker_mask |= speaker;
        }
    }

    result
}

/// Compares two serialized `WAVEFORMATEX` values using Wine's bytewise semantics.
///
/// The first 18 bytes (including `cbSize`) must match, followed by exactly the extension bytes
/// declared by `cbSize`; trailing bytes are ignored. Unlike Wine's raw-pointer helper, truncated
/// headers or extensions safely return `false`.
pub fn formats_equal(first: &[u8], second: &[u8]) -> bool {
    if first.len() < WAVEFORMATEX_HEADER_SIZE || second.len() < WAVEFORMATEX_HEADER_SIZE {
        return false;
    }
    if first[..WAVEFORMATEX_HEADER_SIZE] != second[..WAVEFORMATEX_HEADER_SIZE] {
        return false;
    }

    let extension_size = u16::from_le_bytes([first[16], first[17]]) as usize;
    let end = WAVEFORMATEX_HEADER_SIZE + extension_size;
    if first.len() < end || second.len() < end {
        return false;
    }

    first[WAVEFORMATEX_HEADER_SIZE..end] == second[WAVEFORMATEX_HEADER_SIZE..end]
}
