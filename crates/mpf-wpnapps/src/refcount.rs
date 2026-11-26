//! The `LONG ref` of the static factories and of `struct toast_notifier`.

/// `InterlockedIncrement` and `InterlockedDecrement` wrap on overflow. `AddRef` and `Release`
/// return the new value as a `ULONG`, so a count below zero reads back as a large unsigned value.
#[derive(Debug)]
pub(crate) struct RefCount(i32);

impl RefCount {
    /// The reference held by a static factory or by a freshly created notifier.
    pub(crate) fn initial() -> Self {
        Self(1)
    }

    pub(crate) fn get(&self) -> i32 {
        self.0
    }

    pub(crate) fn add_ref(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(1);
        self.0 as u32
    }

    pub(crate) fn release(&mut self) -> u32 {
        self.0 = self.0.wrapping_sub(1);
        self.0 as u32
    }
}
