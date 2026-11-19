//! The packing side of `IDirectMusicBufferImpl` from `dlls/dmusic/buffer.c`: the event layout, the
//! buffer size rounding of `create_format_and_size`, and the position and time rules of the methods.

use crate::{
    Guid, Hresult, DMUS_E_BUFFER_EMPTY, DMUS_E_BUFFER_FULL, DMUS_E_INVALID_EVENT, GUID_NULL,
    KSDATAFORMAT_SUBTYPE_MIDI,
};

/// `DMUS_EVENT_STRUCTURED`: the header flag of a channel message.
const DMUS_EVENT_STRUCTURED: u32 = 0x1;

/// `sizeof(DMUS_EVENTHEADER)`: the header is packed to 4 bytes, so its 20 bytes have no padding.
const EVENT_HEADER_SIZE: u64 = 20;

/// `QWORD_ALIGN(x)`.
const fn qword_align(x: u64) -> u64 {
    (x + 7) & !7
}

/// `DMUS_EVENT_SIZE(cb)`: the header and `cb` bytes of data, rounded up to 8 bytes.
const fn dmus_event_size(cb: u64) -> u64 {
    qword_align(EVENT_HEADER_SIZE + cb)
}

/// The buffer size that `create_format_and_size` stores for `cb_buffer`: rounded up to a multiple of
/// 4. The addition wraps at 32 bits, as the C `DWORD` arithmetic does, so `0xFFFFFFFD` gives 0.
pub const fn rounded_buffer_size(cb_buffer: u32) -> u32 {
    cb_buffer.wrapping_add(3) & !3
}

/// The state of `IDirectMusicBufferImpl` that its packing methods read and write.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventBuffer {
    format: Guid,
    size: u32,
    write_pos: u32,
    start_time: i64,
    data: Vec<u8>,
}

impl EventBuffer {
    /// `create_format_and_size` for a buffer of `cb_buffer` bytes. A `GUID_NULL` format means MIDI.
    pub fn new(cb_buffer: u32, format: Guid) -> Self {
        let size = rounded_buffer_size(cb_buffer);
        Self {
            format: if format == GUID_NULL {
                KSDATAFORMAT_SUBTYPE_MIDI
            } else {
                format
            },
            size,
            write_pos: 0,
            start_time: 0,
            data: vec![0; size as usize],
        }
    }

    /// `Flush`: empties the buffer. The start time stays until the next event sets it.
    pub fn flush(&mut self) {
        self.write_pos = 0;
    }

    /// `PackStructured`: appends a channel message as a 3-byte structured event. The status byte
    /// must have its top bit set.
    pub fn pack_structured(
        &mut self,
        ref_time: i64,
        channel_group: u32,
        channel_message: u32,
    ) -> Result<(), Hresult> {
        let end = self.check_room(4)?;
        if channel_message & 0x80 == 0 {
            return Err(DMUS_E_INVALID_EVENT);
        }
        self.write_event(
            end,
            ref_time,
            channel_group,
            3,
            DMUS_EVENT_STRUCTURED,
            &channel_message.to_le_bytes(),
        );
        Ok(())
    }

    /// `PackUnstructured`: appends `data` as an unstructured event.
    pub fn pack_unstructured(
        &mut self,
        ref_time: i64,
        channel_group: u32,
        data: &[u8],
    ) -> Result<(), Hresult> {
        let end = self.check_room(data.len() as u64)?;
        // `check_room` has bounded the length by the 32-bit buffer size.
        let cb = data.len() as u32;
        self.write_event(end, ref_time, channel_group, cb, 0, data);
        Ok(())
    }

    /// `SetUsedBytes`: moves the write position. A position past the buffer end is rejected.
    pub fn set_used_bytes(&mut self, used: u32) -> Result<(), Hresult> {
        if used > self.size {
            return Err(DMUS_E_BUFFER_FULL);
        }
        self.write_pos = used;
        Ok(())
    }

    /// `GetUsedBytes`.
    pub fn used_bytes(&self) -> u32 {
        self.write_pos
    }

    /// `GetMaxBytes`.
    pub fn max_bytes(&self) -> u32 {
        self.size
    }

    /// `GetBufferFormat`.
    pub fn format(&self) -> Guid {
        self.format
    }

    /// `GetStartTime`: the time of the first event, or `DMUS_E_BUFFER_EMPTY` while nothing is used.
    pub fn start_time(&self) -> Result<i64, Hresult> {
        if self.write_pos == 0 {
            return Err(DMUS_E_BUFFER_EMPTY);
        }
        Ok(self.start_time)
    }

    /// `SetStartTime`: the time that the next event sets when it is the first one in the buffer.
    pub fn set_start_time(&mut self, ref_time: i64) {
        self.start_time = ref_time;
    }

    /// The whole buffer, including the bytes past the used count.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the write position after an event of `cb` data bytes, or `DMUS_E_BUFFER_FULL` when
    /// that event does not fit. The sum is 64-bit, so it cannot wrap.
    fn check_room(&self, cb: u64) -> Result<u32, Hresult> {
        let end = u64::from(self.write_pos) + dmus_event_size(cb);
        if end > u64::from(self.size) {
            return Err(DMUS_E_BUFFER_FULL);
        }
        Ok(end as u32)
    }

    /// Writes the header and payload at the write position, then moves it to `end`. The header is
    /// little-endian: `cbEvent`, `dwChannelGroup`, `rtDelta` as 64 bits, and `dwFlags`.
    fn write_event(
        &mut self,
        end: u32,
        ref_time: i64,
        channel_group: u32,
        cb: u32,
        flags: u32,
        payload: &[u8],
    ) {
        if self.write_pos == 0 {
            self.start_time = ref_time;
        }
        let delta = ref_time.wrapping_sub(self.start_time);
        let event = &mut self.data[self.write_pos as usize..];
        event[0..4].copy_from_slice(&cb.to_le_bytes());
        event[4..8].copy_from_slice(&channel_group.to_le_bytes());
        event[8..16].copy_from_slice(&delta.to_le_bytes());
        event[16..20].copy_from_slice(&flags.to_le_bytes());
        event[20..20 + payload.len()].copy_from_slice(payload);
        self.write_pos = end;
    }
}
