//! Headless subset of Wine's `dlls/wintab32` (the Wintab tablet API).
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`info`]: the category predicates `is_logcontext_category` and `is_string_field`.
//! - [`scale`]: `MulDiv` and `ScaleForContext`, and the Y flip that `AddPacketToContextQueue` applies.
//! - [`packet`]: the `WTPACKET` byte layout of `TABLET_CopyPacketData` and `TABLET_BlankPacketData`
//!   (LP64 sizes).
//! - [`queue`]: the packet queue behind `WTPacket`, `WTPacketsGet`, `WTPacketsPeek`, `WTDataGet`,
//!   `WTDataPeek`, `WTQueuePacketsEx` and `WTQueueSizeSet`, including overflow flagging.
//! - [`context`]: an open context's state (`WTOpenW`, `WTEnable`, `WTOverlap`) and the delivery of a
//!   packet to it (`AddPacketToContextQueue`).
//!
//! Skipped: the handle table (`gOpenContexts`, `gTopContext`) and the critical section; window
//! messages (the caller posts the `_WT_*` messages for [`Accepted::Queued`]); the driver bridge and
//! `LoadTablet`; `WTInfo` payloads and system metrics; the LOGCONTEXT ANSI/Unicode conversion; the
//! stubs (`WTConfig`, `WTExtGet`, `WTExtSet`, `WTSave`, `WTRestore`); the reallocation in
//! `WTQueueSizeSet`; and the `cMaxPkts` argument of `WTDataGet`/`WTDataPeek`, which Wine ignores too.
//!
//! Deviations, all where Wine reads stale memory or has undefined behaviour:
//! - `WTDataGet` with a begin serial queued but no end serial queued after it copies stale slots.
//!   The port returns no packets, as `WTDataPeek` already does.
//! - A queue shrunk below its length keeps its packets. `push` then refuses new packets, flagging
//!   the last one, until the queue drains below the new capacity. Wine writes past the array.
//! - A queue with capacity 0 refuses packets without flagging. Wine writes `PacketQueue[-1]`.
//! - Signed overflow and `abs(INT_MIN)` wrap instead of being undefined.
//! - The 32-bit `pkContext` is encoded zero-extended to the 8-byte `HCTX` of LP64 builds.

pub mod context;
pub mod info;
pub mod packet;
pub mod queue;
pub mod scale;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// What became of a packet delivered to an open context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accepted {
    /// The packet was queued. `csr_message` is set when Wine posts a cursor-change message for it.
    Queued { csr_message: bool },
    /// The queue was full: its last packet is flagged `TPS_QUEUE_ERR` and this packet is dropped.
    Overflow,
}

impl Accepted {
    /// Whether the caller must post the cursor-change message for the packet.
    pub fn csr_message(self) -> bool {
        matches!(self, Accepted::Queued { csr_message: true })
    }
}
