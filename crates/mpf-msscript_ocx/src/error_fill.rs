//! The source-position fill of `ScriptError`.

/// The `pos_filled` latch of `fill_error_pos`. Only the first call looks up the position, and later
/// calls keep the values from the first.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PosFill {
    filled: bool,
    line: u32,
    column: i32,
}

impl PosFill {
    /// `fetch` stands for `IActiveScriptError_GetSourcePosition`. It returns `None` when there is no
    /// error object or the call fails. It runs only on the first `fill`.
    pub fn fill(&mut self, fetch: impl FnOnce() -> Option<(u32, i32)>) {
        if self.filled {
            return;
        }
        self.filled = true;
        if let Some((line, column)) = fetch() {
            self.line = line;
            self.column = column;
        }
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> i32 {
        self.column
    }
}
