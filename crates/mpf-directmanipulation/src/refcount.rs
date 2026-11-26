//! Reference counting with the `InterlockedIncrement` / `InterlockedDecrement` results.

/// Reference count of one COM object. Every Wine `create_*` helper starts at 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefCount(u32);

impl Default for RefCount {
    fn default() -> Self {
        Self(1)
    }
}

impl RefCount {
    pub fn add_ref(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }

    pub fn release(&mut self) -> u32 {
        self.0 -= 1;
        self.0
    }

    pub fn count(&self) -> u32 {
        self.0
    }
}
