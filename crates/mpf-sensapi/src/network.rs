/// `NETWORK_ALIVE_LAN` from `include/sensapi.h`.
pub const NETWORK_ALIVE_LAN: u32 = 1;
/// `NETWORK_ALIVE_WAN` from `include/sensapi.h`.
pub const NETWORK_ALIVE_WAN: u32 = 2;
/// `NETWORK_ALIVE_AOL` from `include/sensapi.h`.
pub const NETWORK_ALIVE_AOL: u32 = 4;

/// `IsNetworkAlive` from `dlls/sensapi/sensapi.c`. When the pointer is not NULL, the C body
/// assigns `NETWORK_ALIVE_LAN` to `*lpdwFlags`, which replaces the old value rather than
/// OR-ing into it. It returns `TRUE` in both cases.
pub fn is_network_alive(flags: Option<&mut u32>) -> bool {
    if let Some(flags) = flags {
        *flags = NETWORK_ALIVE_LAN;
    }
    true
}
