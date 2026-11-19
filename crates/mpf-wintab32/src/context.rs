//! An open tablet context: the state `WTOpenW`, `WTEnable` and `WTOverlap` set, and the delivery of
//! a packet to it (`AddPacketToContextQueue`).

use crate::packet::Packet;
use crate::queue::{Queue, DEFAULT_QUEUE_SIZE};
use crate::scale::{map_point, Axis};
use crate::Accepted;

pub const CXO_MESSAGES: u32 = 0x0004;
pub const CXO_CSRMESSAGES: u32 = 0x0008;
pub const CXS_DISABLED: u32 = 0x0001;
pub const CXS_OBSCURED: u32 = 0x0002;
pub const CXS_ONTOP: u32 = 0x0004;

pub struct Context {
    handle: u32,
    enabled: bool,
    status: u32,
    options: u32,
    pk_data: u32,
    x_axis: Axis,
    y_axis: Axis,
    active_cursor: u32,
    queue: Queue,
}

impl Context {
    /// Opens a context as `WTOpenW` does. An enabled context starts on top, a disabled one with
    /// `CXS_DISABLED`. No cursor is active yet.
    pub fn open(handle: u32, enable: bool, options: u32, pk_data: u32) -> Self {
        Self {
            handle,
            enabled: enable,
            status: if enable { CXS_ONTOP } else { CXS_DISABLED },
            options,
            pk_data,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            active_cursor: u32::MAX,
            queue: Queue::new(DEFAULT_QUEUE_SIZE),
        }
    }

    pub fn status(&self) -> u32 {
        self.status
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// The packet data mask (`lcPktData`) that selects the fields of each packet.
    pub fn pk_data(&self) -> u32 {
        self.pk_data
    }

    /// The cursor of the last queued packet, or `u32::MAX` before any packet was queued.
    pub fn active_cursor(&self) -> u32 {
        self.active_cursor
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn queue_mut(&mut self) -> &mut Queue {
        &mut self.queue
    }

    pub fn set_axes(&mut self, x_axis: Axis, y_axis: Axis) {
        self.x_axis = x_axis;
        self.y_axis = y_axis;
    }

    /// `WTEnable`. Enabling a disabled context puts it on top. Disabling an enabled one flushes its
    /// queue. Other calls change nothing.
    pub fn set_enabled(&mut self, enable: bool) {
        if enable && !self.enabled {
            self.enabled = true;
            self.status = CXS_ONTOP;
        } else if !enable && self.enabled {
            self.enabled = false;
            self.status = CXS_DISABLED;
            self.queue.clear();
        }
    }

    /// `WTOverlap`: moves the context to the top or to the bottom of the overlap order.
    pub fn set_overlap(&mut self, to_top: bool) {
        self.status = if to_top { CXS_ONTOP } else { CXS_OBSCURED };
    }

    /// Delivers `packet` to this context. A disabled context takes nothing and returns `None`, as
    /// Wine skips it. Otherwise the packet is stamped with the handle, masked with the data mask,
    /// mapped through the axes and queued. A change of cursor is reported as a cursor message when
    /// the options ask for one and the context takes messages.
    pub fn accept(&mut self, mut packet: Packet) -> Option<Accepted> {
        if !self.enabled {
            return None;
        }
        packet.context = self.handle;
        packet.changed &= self.pk_data;
        (packet.x, packet.y) = map_point(self.x_axis, self.y_axis, packet.x, packet.y);
        if !self.queue.push(packet) {
            return Some(Accepted::Overflow);
        }
        let cursor_changed = self.active_cursor != packet.cursor;
        self.active_cursor = packet.cursor;
        let csr_message = cursor_changed
            && (self.options & CXO_CSRMESSAGES) != 0
            && (self.options & CXO_MESSAGES) != 0;
        Some(Accepted::Queued { csr_message })
    }
}
