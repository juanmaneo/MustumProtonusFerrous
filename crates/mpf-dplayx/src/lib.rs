//! Deterministic packet logic from Wine's `dlls/dplayx/dplayx_messages.c`.
//!
//! Ported (pure logic, checked against the verbatim C by oracle rows and edge tests):
//! - `DP_MSG_ComputeMessageSize`: the wrapping `DWORD` sum of the send buffer lengths.
//! - `DP_MSG_ReadPackedPlayer`: the fixed-layout packed player record.
//! - `DP_MSG_ReadSuperPackedPlayer` and its helpers `DP_MSG_ReadString`,
//!   `DP_MSG_ReadInteger`, and the `DPLAYI_SUPERPACKEDPLAYER_*` mask macros.
//!
//! Each reader takes a byte slice, a start offset and a maximum size, as the C does. It returns
//! owned values rather than pointers into the packet, and leaves the offset unchanged on error.
//!
//! Deviations from the C:
//! - The C reads through raw pointers, so a short packet or a misaligned `DPID` is undefined
//!   behaviour there. Here every read is bounds-checked and integers are decoded little-endian.
//! - `max_size` larger than the slice, or an offset past `max_size`, is rejected. The C would read
//!   past the buffer.
//! - A sized string of one byte is rejected. The C indexes word `-1` for it.
//! - The C returns a pointer into the packet, and callers read it as a NUL-terminated string.
//!   Here a name holds the code units before its first NUL.
//!
//! Skipped: the send and reply paths (`DP_MSG_SendRequestPlayerId` and the other `DP_MSG_Send*`
//! functions), `DP_MSG_ExpectReply`, the reply-thread functions, and everything that needs
//! `IDirectPlayImpl`, critical sections, events, handles or `malloc`.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `DPLAYI_SUPERPACKEDPLAYER_SHORT_NAME_PRESENT`.
const SHORT_NAME_PRESENT: u32 = 0x001;
/// `DPLAYI_SUPERPACKEDPLAYER_LONG_NAME_PRESENT`.
const LONG_NAME_PRESENT: u32 = 0x002;
/// `DPLAYI_SUPERPACKEDPLAYER_PARENT_ID_PRESENT`.
const PARENT_ID_PRESENT: u32 = 0x100;

/// A decoded player record, as `DPPLAYERINFO` fills it in. Names omit their terminating NUL.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlayerInfo {
    pub flags: u32,
    pub id: u32,
    pub version_or_system_player_id: u32,
    pub short_name: Option<Vec<u16>>,
    pub long_name: Option<Vec<u16>>,
    pub player_data: Vec<u8>,
    pub sp_data: Vec<u8>,
    pub player_ids: Vec<u32>,
    pub parent_id: u32,
    pub shortcut_ids: Vec<u32>,
}

/// The packet is truncated, or a field is malformed. `DPERR_GENERIC` in the C.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidPacket;

/// `DP_MSG_ComputeMessageSize`: sums the segment lengths with `DWORD` wrapping.
pub fn compute_message_size(lengths: &[u32]) -> u32 {
    lengths
        .iter()
        .fold(0u32, |size, &length| size.wrapping_add(length))
}

/// `DP_MSG_ReadPackedPlayer`. On success `offset` moves past the record.
pub fn read_packed_player(
    data: &[u8],
    offset: &mut usize,
    max_size: usize,
) -> Result<PlayerInfo, InvalidPacket> {
    let mut cursor = Cursor::new(data, *offset, max_size)?;
    let fields: [u32; 12] = cursor.read_u32s()?;
    let [_size, flags, id, short_name_length, long_name_length, sp_data_length, player_data_length, player_count, _system_player_id, _fixed_size, version, parent_id] =
        fields;

    let mut info = PlayerInfo {
        flags,
        id,
        version_or_system_player_id: version,
        parent_id,
        ..PlayerInfo::default()
    };
    if short_name_length != 0 {
        info.short_name = Some(cursor.read_sized_string(short_name_length as usize)?);
    }
    if long_name_length != 0 {
        info.long_name = Some(cursor.read_sized_string(long_name_length as usize)?);
    }
    info.sp_data = cursor.read_bytes(sp_data_length as usize)?.to_vec();
    info.player_data = cursor.read_bytes(player_data_length as usize)?.to_vec();
    info.player_ids = cursor.read_ids(player_count)?;

    *offset = cursor.position;
    Ok(info)
}

/// `DP_MSG_ReadSuperPackedPlayer`. On success `offset` moves past the record.
pub fn read_super_packed_player(
    data: &[u8],
    offset: &mut usize,
    max_size: usize,
) -> Result<PlayerInfo, InvalidPacket> {
    let mut cursor = Cursor::new(data, *offset, max_size)?;
    let [_size, flags, id, info_mask, version]: [u32; 5] = cursor.read_u32s()?;

    let mut info = PlayerInfo {
        flags,
        id,
        version_or_system_player_id: version,
        ..PlayerInfo::default()
    };
    if info_mask & SHORT_NAME_PRESENT != 0 {
        info.short_name = Some(cursor.read_nul_terminated_string()?);
    }
    if info_mask & LONG_NAME_PRESENT != 0 {
        info.long_name = Some(cursor.read_nul_terminated_string()?);
    }
    info.player_data = cursor.read_length_prefixed_bytes(width_field(info_mask, 4))?;
    info.sp_data = cursor.read_length_prefixed_bytes(width_field(info_mask, 2))?;
    info.player_ids = cursor.read_length_prefixed_ids(width_field(info_mask, 6))?;
    if info_mask & PARENT_ID_PRESENT != 0 {
        info.parent_id = cursor.read_u32()?;
    }
    info.shortcut_ids = cursor.read_length_prefixed_ids(width_field(info_mask, 9))?;

    *offset = cursor.position;
    Ok(info)
}

/// The 2-bit length width at `shift` in a super-packed info mask. Width 0 means no field,
/// 1 a byte, 2 a `WORD` and 3 a `DWORD`.
fn width_field(info_mask: u32, shift: u32) -> u32 {
    (info_mask >> shift) & 3
}

/// The little-endian `WCHAR` code units in `bytes`. A trailing odd byte is ignored.
fn words_le(bytes: &[u8]) -> Vec<u16> {
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect()
}

/// The little-endian `DWORD` values in `bytes`. A trailing partial word is ignored.
fn dwords_le(bytes: &[u8]) -> Vec<u32> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|quad| u32::from_le_bytes(*quad))
        .collect()
}

struct Cursor<'a> {
    data: &'a [u8],
    position: usize,
    end: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8], position: usize, max_size: usize) -> Result<Self, InvalidPacket> {
        if position > max_size || max_size > data.len() {
            return Err(InvalidPacket);
        }
        Ok(Self {
            data,
            position,
            end: max_size,
        })
    }

    fn read_bytes(&mut self, length: usize) -> Result<&'a [u8], InvalidPacket> {
        if length > self.end - self.position {
            return Err(InvalidPacket);
        }
        let bytes = &self.data[self.position..self.position + length];
        self.position += length;
        Ok(bytes)
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], InvalidPacket> {
        let mut array = [0u8; N];
        array.copy_from_slice(self.read_bytes(N)?);
        Ok(array)
    }

    fn read_u32(&mut self) -> Result<u32, InvalidPacket> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    fn read_u32s<const N: usize>(&mut self) -> Result<[u32; N], InvalidPacket> {
        let mut values = [0u32; N];
        for value in &mut values {
            *value = self.read_u32()?;
        }
        Ok(values)
    }

    /// `DP_MSG_ReadSizedString`. The last code unit of the field must be NUL, and the field is
    /// consumed whole. An odd length leaves its trailing byte unread as the C does.
    fn read_sized_string(&mut self, byte_length: usize) -> Result<Vec<u16>, InvalidPacket> {
        if byte_length < 2 {
            return Err(InvalidPacket);
        }
        let bytes = self.read_bytes(byte_length)?;
        let words = words_le(bytes);
        let terminator = words.len() - 1;
        if words[terminator] != 0 {
            return Err(InvalidPacket);
        }
        Ok(words[..terminator]
            .iter()
            .copied()
            .take_while(|&word| word != 0)
            .collect())
    }

    /// `DP_MSG_ReadString`. The NUL must fall within the bytes that remain, and the string is
    /// consumed up to and including it.
    fn read_nul_terminated_string(&mut self) -> Result<Vec<u16>, InvalidPacket> {
        let remaining = &self.data[self.position..self.end];
        let words = words_le(remaining);
        let length = words
            .iter()
            .position(|&word| word == 0)
            .ok_or(InvalidPacket)?;
        self.position += (length + 1) * 2;
        Ok(words[..length].to_vec())
    }

    /// `DP_MSG_ReadInteger`: reads a width-coded length, where 0 means no field.
    fn read_width_integer(&mut self, width: u32) -> Result<u32, InvalidPacket> {
        match width {
            0 => Ok(0),
            1 => Ok(u32::from(self.read_bytes(1)?[0])),
            2 => Ok(u32::from(u16::from_le_bytes(self.read_array()?))),
            _ => self.read_u32(),
        }
    }

    fn read_length_prefixed_bytes(&mut self, width: u32) -> Result<Vec<u8>, InvalidPacket> {
        let length = self.read_width_integer(width)?;
        Ok(self.read_bytes(length as usize)?.to_vec())
    }

    fn read_length_prefixed_ids(&mut self, width: u32) -> Result<Vec<u32>, InvalidPacket> {
        let count = self.read_width_integer(width)?;
        self.read_ids(count)
    }

    fn read_ids(&mut self, count: u32) -> Result<Vec<u32>, InvalidPacket> {
        let bytes = self.read_bytes((count as usize).saturating_mul(4))?;
        Ok(dwords_le(bytes))
    }
}
