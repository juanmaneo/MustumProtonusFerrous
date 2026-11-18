use crate::STATUS_INVALID_PARAMETER;

/// Length of a new queue (`hid_queue_create`).
pub const DEFAULT_LENGTH: usize = 32;
/// Smallest length `hid_queue_resize` accepts.
pub const MIN_LENGTH: usize = 2;
/// Largest length `hid_queue_resize` accepts.
pub const MAX_LENGTH: usize = 512;

/// The ring of input reports of one handle (`struct hid_queue`).
///
/// A slot holds `None` when it is empty. Pushing into a full ring overwrites
/// the oldest report, which is what `hid_queue_push_report` does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportQueue<T> {
    slots: Vec<Option<T>>,
    read_idx: usize,
    write_idx: usize,
}

impl<T> Default for ReportQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ReportQueue<T> {
    /// Creates an empty queue of `DEFAULT_LENGTH` slots.
    pub fn new() -> Self {
        Self {
            slots: empty_slots(DEFAULT_LENGTH),
            read_idx: 0,
            write_idx: 0,
        }
    }

    /// The number of slots, `queue->length`.
    pub fn length(&self) -> usize {
        self.slots.len()
    }

    /// The index of the next report to pop, `queue->read_idx`.
    pub fn read_index(&self) -> usize {
        self.read_idx
    }

    /// The index of the next slot to fill, `queue->write_idx`.
    pub fn write_index(&self) -> usize {
        self.write_idx
    }

    /// Stores `report` at the write index (`hid_queue_push_report`).
    ///
    /// When the write index reaches the read index, the read index moves past
    /// the slot that is about to be overwritten.
    pub fn push(&mut self, report: T) {
        let i = self.write_idx;
        let mut next = i + 1;
        if next >= self.length() {
            next = 0;
        }

        self.slots[i] = Some(report);
        if next == self.read_idx {
            self.read_idx = next + 1;
        }
        if self.read_idx >= self.length() {
            self.read_idx = 0;
        }
        self.write_idx = next;
    }

    /// Takes the report at the read index (`hid_queue_pop_report`).
    ///
    /// The read index only advances when the slot is not the write index, so
    /// popping an empty queue leaves the indices alone.
    pub fn pop(&mut self) -> Option<T> {
        let i = self.read_idx;
        let mut next = i + 1;
        if next >= self.length() {
            next = 0;
        }

        let report = self.slots[i].take();
        if i != self.write_idx {
            self.read_idx = next;
        }
        report
    }

    /// Changes the length (`hid_queue_resize`).
    ///
    /// Lengths outside `MIN_LENGTH..=MAX_LENGTH` fail with
    /// `STATUS_INVALID_PARAMETER`. Resizing to the current length keeps the
    /// queued reports. Any other valid length drops them and resets both indices.
    pub fn resize(&mut self, length: usize) -> Result<(), i32> {
        if !(MIN_LENGTH..=MAX_LENGTH).contains(&length) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        if length == self.length() {
            return Ok(());
        }

        self.slots = empty_slots(length);
        self.read_idx = 0;
        self.write_idx = 0;
        Ok(())
    }
}

fn empty_slots<T>(length: usize) -> Vec<Option<T>> {
    std::iter::repeat_with(|| None).take(length).collect()
}
