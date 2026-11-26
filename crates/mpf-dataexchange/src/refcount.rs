//! The `LONG ref` that each object carries, updated by `InterlockedIncrement` and
//! `InterlockedDecrement`.

/// The wrapping `LONG`. `AddRef` and `Release` return it as a `ULONG`, so a count below
/// zero reads back as a large unsigned value.
#[derive(Debug)]
pub(crate) struct RefCount(i32);

impl RefCount {
    /// The reference held by the static object, or by a freshly allocated manager.
    pub(crate) fn initial() -> Self {
        Self(1)
    }

    pub(crate) fn get(&self) -> u32 {
        self.0 as u32
    }

    pub(crate) fn add_ref(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(1);
        self.get()
    }

    pub(crate) fn release(&mut self) -> u32 {
        self.0 = self.0.wrapping_sub(1);
        self.get()
    }
}
