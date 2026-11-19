//! The tablet packet (`WTPACKET`) and the bytes `TABLET_CopyPacketData` and
//! `TABLET_BlankPacketData` write for a packet data mask. Sizes are the LP64 ones: `HCTX` is
//! 8 bytes, and `LONG`, `DWORD`, `UINT` and each `ORIENTATION` or `ROTATION` component 4 bytes.
//! Values are little-endian.

pub const PK_CONTEXT: u32 = 0x0001;
pub const PK_STATUS: u32 = 0x0002;
pub const PK_TIME: u32 = 0x0004;
pub const PK_CHANGED: u32 = 0x0008;
pub const PK_SERIAL_NUMBER: u32 = 0x0010;
pub const PK_CURSOR: u32 = 0x0020;
pub const PK_BUTTONS: u32 = 0x0040;
pub const PK_X: u32 = 0x0080;
pub const PK_Y: u32 = 0x0100;
pub const PK_Z: u32 = 0x0200;
pub const PK_NORMAL_PRESSURE: u32 = 0x0400;
pub const PK_TANGENT_PRESSURE: u32 = 0x0800;
pub const PK_ORIENTATION: u32 = 0x1000;
pub const PK_ROTATION: u32 = 0x2000;

/// Set on the last queued packet when the queue overflows.
pub const TPS_QUEUE_ERR: u32 = 0x0002;

/// One tablet packet, as `WTPACKET` holds it, before the packet data mask selects its fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Packet {
    /// `pkContext`: the handle of the context the packet was delivered to.
    pub context: u32,
    pub status: u32,
    pub time: i32,
    pub changed: u32,
    pub serial: u32,
    pub cursor: u32,
    pub buttons: u32,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub normal_pressure: u32,
    pub tangent_pressure: u32,
    pub orientation: [i32; 3],
    pub rotation: [i32; 3],
}

/// The bytes `TABLET_CopyPacketData` writes for `packet` under `mask`, in Wine's field order.
pub fn encode(mask: u32, packet: &Packet) -> Vec<u8> {
    let mut out = Vec::new();
    let mut put = |bit: u32, bytes: &[u8]| {
        if mask & bit != 0 {
            out.extend_from_slice(bytes);
        }
    };
    put(PK_CONTEXT, &u64::from(packet.context).to_le_bytes());
    put(PK_STATUS, &packet.status.to_le_bytes());
    put(PK_TIME, &packet.time.to_le_bytes());
    put(PK_CHANGED, &packet.changed.to_le_bytes());
    put(PK_SERIAL_NUMBER, &packet.serial.to_le_bytes());
    put(PK_CURSOR, &packet.cursor.to_le_bytes());
    put(PK_BUTTONS, &packet.buttons.to_le_bytes());
    put(PK_X, &packet.x.to_le_bytes());
    put(PK_Y, &packet.y.to_le_bytes());
    put(PK_Z, &packet.z.to_le_bytes());
    put(PK_NORMAL_PRESSURE, &packet.normal_pressure.to_le_bytes());
    put(PK_TANGENT_PRESSURE, &packet.tangent_pressure.to_le_bytes());
    put(PK_ORIENTATION, &i32s_le(&packet.orientation));
    put(PK_ROTATION, &i32s_le(&packet.rotation));
    out
}

/// The bytes `TABLET_CopyPacketData` writes for each of `packets` in turn.
pub fn encode_all(mask: u32, packets: &[Packet]) -> Vec<u8> {
    packets
        .iter()
        .flat_map(|packet| encode(mask, packet))
        .collect()
}

/// The size of one packet under `mask`, as `TABLET_BlankPacketData` counts it.
pub fn encoded_len(mask: u32) -> usize {
    encode(mask, &Packet::default()).len()
}

/// The buffer `WTPacketsGet` returns for `slots` packets: `packets` followed by zeroed slots.
/// `packets` must not hold more than `slots` packets.
pub fn fill(mask: u32, packets: &[Packet], slots: usize) -> Vec<u8> {
    let mut out = encode_all(mask, packets);
    out.resize(slots * encoded_len(mask), 0);
    out
}

fn i32s_le(values: &[i32; 3]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}
