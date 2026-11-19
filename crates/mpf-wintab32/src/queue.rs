//! The packet queue of an open context: `PacketQueue`, `PacketsQueued` and `QueueSize` of
//! `context.c`, and the operations the `WTPacket*`, `WTData*` and `WTQueue*` calls perform on it.

use std::ops::Range;

use crate::packet::{Packet, TPS_QUEUE_ERR};

/// The queue size of a newly opened context (`WTOpenW`).
pub const DEFAULT_QUEUE_SIZE: usize = 50;

pub struct Queue {
    capacity: usize,
    packets: Vec<Packet>,
}

impl Queue {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            packets: Vec::new(),
        }
    }

    /// The queue size set by `WTQueueSizeSet` (`WTQueueSizeGet`).
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Sets the queue size. Packets already queued stay even when they exceed the new size.
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    pub fn packets(&self) -> &[Packet] {
        &self.packets
    }

    /// Flushes every queued packet, as `WTEnable` does when it disables a context.
    pub fn clear(&mut self) {
        self.packets.clear();
    }

    /// Queues `packet`, or refuses it when the queue is full. A refused packet flags the last
    /// queued packet with `TPS_QUEUE_ERR`. The result is `false` when the packet was refused.
    pub fn push(&mut self, packet: Packet) -> bool {
        if self.packets.len() < self.capacity {
            self.packets.push(packet);
            return true;
        }
        if let Some(last) = self.packets.last_mut() {
            last.status |= TPS_QUEUE_ERR;
        }
        false
    }

    /// The first `max` packets, removed from the queue (`WTPacketsGet`).
    pub fn take_front(&mut self, max: usize) -> Vec<Packet> {
        let count = max.min(self.packets.len());
        self.packets.drain(..count).collect()
    }

    /// The first `max` packets, left in the queue (`WTPacketsPeek`).
    pub fn peek_front(&self, max: usize) -> &[Packet] {
        &self.packets[..max.min(self.packets.len())]
    }

    /// The packet with `serial` and every packet queued before it are removed (`WTPacket`). Returns
    /// how many were removed, which is Wine's return value, and the packet.
    pub fn take_through(&mut self, serial: u32) -> Option<(usize, Packet)> {
        let index = self.position(serial)?;
        let packet = self.packets[index];
        self.packets.drain(..=index);
        Some((index + 1, packet))
    }

    /// The index range from the packet with serial `begin` through the packet with serial `end`
    /// (`WTDataGet` and `WTDataPeek`). `None` when `begin` is not queued or `end` is not queued
    /// after it. Wine's `WTDataGet` copies stale slots then, which the port does not reproduce.
    pub fn data_range(&self, begin: u32, end: u32) -> Option<Range<usize>> {
        let first = self.position(begin)?;
        let last = first + self.packets[first..].iter().position(|p| p.serial == end)?;
        Some(first..last + 1)
    }

    /// Removes the packets in `range` and returns them.
    pub fn take_range(&mut self, range: Range<usize>) -> Vec<Packet> {
        self.packets.drain(range).collect()
    }

    /// The serials of the first and last queued packets (`WTQueuePacketsEx`), or `None` when the
    /// queue is empty.
    pub fn ends(&self) -> Option<(u32, u32)> {
        self.packets
            .first()
            .zip(self.packets.last())
            .map(|(first, last)| (first.serial, last.serial))
    }

    fn position(&self, serial: u32) -> Option<usize> {
        self.packets.iter().position(|p| p.serial == serial)
    }
}
