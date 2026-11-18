//! The capability block that `DPWSCB_GetCaps` reports.

use crate::header::HEADER_SIZE;
use crate::tcp::GUARANTEED_MAX_BUFFER_SIZE;

/// `DPGETCAPS_GUARANTEED`: the `dwFlags` bit that asks for the guaranteed-message limits.
pub const DPGETCAPS_GUARANTEED: u32 = 0x0000_0001;

const DPCAPS_ASYNCSUPPORTED: u32 = 0x0001_0000;
const DPCAPS_GUARANTEEDOPTIMIZED: u32 = 0x0000_0020;
const DPCAPS_GUARANTEEDSUPPORTED: u32 = 0x0000_0040;

const DPWS_MAXQUEUESIZE: u32 = 0;
const DPWS_HUNDREDBAUD: u32 = 0;
const DPWS_LATENCY: u32 = 500;
const DPWS_MAXLOCALPLAYERS: u32 = 65536;
const DPWS_TIMEOUT: u32 = 5000;
const DPWS_MAXBUFFERSIZE: u32 = 65479;
const DPWS_MAXPLAYERS: u32 = 65536;
const DPWS_GUARANTEED_MAXPLAYERS: u32 = 64;

/// The `DPCAPS` fields that `DPWSCB_GetCaps` fills in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caps {
    pub flags: u32,
    pub max_queue_size: u32,
    pub hundred_baud: u32,
    pub latency: u32,
    pub max_local_players: u32,
    pub header_length: u32,
    pub timeout: u32,
    pub max_buffer_size: u32,
    pub max_players: u32,
}

/// `DPWSCB_GetCaps` for a request whose `dwFlags` is `flags`.
pub fn get_caps(flags: u32) -> Caps {
    let (max_buffer_size, max_players) = if flags & DPGETCAPS_GUARANTEED != 0 {
        (GUARANTEED_MAX_BUFFER_SIZE, DPWS_GUARANTEED_MAXPLAYERS)
    } else {
        (DPWS_MAXBUFFERSIZE, DPWS_MAXPLAYERS)
    };
    Caps {
        flags: DPCAPS_ASYNCSUPPORTED | DPCAPS_GUARANTEEDOPTIMIZED | DPCAPS_GUARANTEEDSUPPORTED,
        max_queue_size: DPWS_MAXQUEUESIZE,
        hundred_baud: DPWS_HUNDREDBAUD,
        latency: DPWS_LATENCY,
        max_local_players: DPWS_MAXLOCALPLAYERS,
        header_length: HEADER_SIZE as u32,
        timeout: DPWS_TIMEOUT,
        max_buffer_size,
        max_players,
    }
}
