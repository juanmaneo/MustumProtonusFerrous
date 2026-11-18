//! The property-sheet handle table: `alloc_handle`, `free_handle` and `get_handle_data`.

/// `HANDLE_FIRST`: the value of the first handle.
pub const HANDLE_FIRST: u64 = 0x4344_0001;
/// Number of slots in `handles[]`.
pub const HANDLE_COUNT: usize = 0x1000;

/// The object kinds that `alloc_handle` accepts, with their `HANDLE_*` codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Kind {
    /// `HANDLE_PROPSHEET`.
    PropSheet = 1,
    /// `HANDLE_PROPSHEETPAGE`.
    PropSheetPage = 2,
    /// `HANDLE_PROPSHEETFUNC`.
    PropSheetFunc = 3,
}

impl Kind {
    /// Maps a `HANDLE_*` code to a kind. `HANDLE_FREE` (0) and unknown codes give `None`.
    pub fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            1 => Some(Self::PropSheet),
            2 => Some(Self::PropSheetPage),
            3 => Some(Self::PropSheetFunc),
            _ => None,
        }
    }
}

/// A fixed table of `HANDLE_COUNT` slots, handed out as `HANDLE_FIRST + index`.
///
/// Freed slots are reused most recently freed first. A slot that was never used is taken
/// only when no freed slot is left, which matches the `first_free_handle` list in the C code.
pub struct HandleTable<T> {
    slots: Vec<Option<(Kind, T)>>,
    released: Vec<usize>,
    next: usize,
}

impl<T> Default for HandleTable<T> {
    fn default() -> Self {
        Self {
            slots: std::iter::repeat_with(|| None).take(HANDLE_COUNT).collect(),
            released: Vec::new(),
            next: 0,
        }
    }
}

impl<T> HandleTable<T> {
    /// `alloc_handle`: stores `data` under a new handle, or returns `None` when the table is full.
    pub fn alloc(&mut self, kind: Kind, data: T) -> Option<u64> {
        let index = match self.released.pop() {
            Some(index) => index,
            None if self.next < HANDLE_COUNT => {
                self.next += 1;
                self.next - 1
            }
            None => return None,
        };
        self.slots[index] = Some((kind, data));
        Some(HANDLE_FIRST + index as u64)
    }

    /// `free_handle`: releases the slot and returns its data, or `None` when the handle is not in use.
    pub fn free(&mut self, handle: u64) -> Option<T> {
        let index = Self::index(handle)?;
        let (_, data) = self.slots[index].take()?;
        self.released.push(index);
        Some(data)
    }

    /// `get_handle_data`: the kind and data of an allocated handle.
    pub fn get(&self, handle: u64) -> Option<(Kind, &T)> {
        let index = Self::index(handle)?;
        self.slots[index].as_ref().map(|(kind, data)| (*kind, data))
    }

    fn index(handle: u64) -> Option<usize> {
        let index = handle.checked_sub(HANDLE_FIRST)?;
        (index < HANDLE_COUNT as u64).then_some(index as usize)
    }
}
