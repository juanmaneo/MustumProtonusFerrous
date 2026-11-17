//! UCRT on-exit table behavior from Wine's `dlls/msvcrt/onexit.c` and
//! `dlls/ucrtbase/atexit.c`.
//!
//! The table preserves callback registration order and executes non-null
//! callbacks in reverse order, then resets so it can be reused. Rust ownership
//! replaces the C heap allocation and lock. `atexit` is represented by an
//! explicit registry rather than a process-global static.

/// A safe on-exit callback table.
#[derive(Default)]
pub struct OnExitTable {
    callbacks: Vec<Option<fn()>>,
}

impl OnExitTable {
    /// Creates an initialized empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of registered entries, including null callbacks.
    pub fn len(&self) -> usize {
        self.callbacks.len()
    }

    /// Whether no entries are registered.
    pub fn is_empty(&self) -> bool {
        self.callbacks.is_empty()
    }

    /// Registers a callback; `None` is a valid no-op entry like Wine's C API.
    pub fn register(&mut self, callback: Option<fn()>) {
        self.callbacks.push(callback);
    }

    /// Executes callbacks in LIFO order and resets the table before execution.
    pub fn execute(&mut self) {
        let callbacks = std::mem::take(&mut self.callbacks);
        for callback in callbacks.into_iter().rev().flatten() {
            callback();
        }
    }
}

/// Mirrors `_initialize_onexit_table`: null table returns `-1`, otherwise `0`.
pub fn initialize_onexit_table(table: Option<&mut OnExitTable>) -> i32 {
    match table {
        Some(_table) => 0,
        None => -1,
    }
}

/// Mirrors `_register_onexit_function`'s status code.
pub fn register_onexit_function(table: Option<&mut OnExitTable>, callback: Option<fn()>) -> i32 {
    match table {
        Some(table) => {
            table.register(callback);
            0
        }
        None => -1,
    }
}

/// Mirrors `_execute_onexit_table`'s status code.
pub fn execute_onexit_table(table: Option<&mut OnExitTable>) -> i32 {
    match table {
        Some(table) => {
            table.execute();
            0
        }
        None => -1,
    }
}

/// A local equivalent of UCRT's `atexit` registration table.
#[derive(Default)]
pub struct AtexitRegistry {
    table: OnExitTable,
}

impl AtexitRegistry {
    /// Registers a process-exit callback in this registry.
    pub fn atexit(&mut self, callback: fn()) -> i32 {
        register_onexit_function(Some(&mut self.table), Some(callback))
    }

    /// Executes all registered callbacks once, in reverse registration order.
    pub fn execute(&mut self) {
        self.table.execute();
    }
}

/// Mirrors `__isascii`: true for values in `0..0x80` (negative values are not).
pub fn is_ascii(c: i32) -> bool {
    (c as u32) < 0x80
}

/// Mirrors `__toascii`: keeps the low seven bits.
pub fn to_ascii(c: i32) -> i32 {
    (c as u32 & 0x7f) as i32
}

/// Mirrors `__iscsym`: ASCII letters, digits and `_`. EOF and other values
/// outside the byte range are not symbol characters.
pub fn is_csym(c: i32) -> bool {
    u8::try_from(c).is_ok_and(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Mirrors `__iscsymf`: ASCII letters and `_`.
pub fn is_csymf(c: i32) -> bool {
    u8::try_from(c).is_ok_and(|b| b.is_ascii_alphabetic() || b == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ORDER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn first() {
        ORDER.lock().unwrap().push(1);
    }

    fn second() {
        ORDER.lock().unwrap().push(2);
    }

    #[test]
    fn null_table_status_matches_ucrt() {
        assert_eq!(initialize_onexit_table(None), -1);
        assert_eq!(register_onexit_function(None, None), -1);
        assert_eq!(register_onexit_function(None, Some(first)), -1);
        assert_eq!(execute_onexit_table(None), -1);
    }

    #[test]
    fn initialized_empty_table_is_noop_on_execute() {
        let mut table = OnExitTable::default();
        assert_eq!(initialize_onexit_table(Some(&mut table)), 0);
        assert_eq!(execute_onexit_table(Some(&mut table)), 0);
        assert!(table.is_empty());
    }

    #[test]
    fn executes_non_null_callbacks_in_reverse_order() {
        let _guard = TEST_LOCK.lock().unwrap();
        ORDER.lock().unwrap().clear();
        let mut table = OnExitTable::new();
        assert_eq!(register_onexit_function(Some(&mut table), Some(first)), 0);
        assert_eq!(register_onexit_function(Some(&mut table), None), 0);
        assert_eq!(register_onexit_function(Some(&mut table), Some(second)), 0);
        assert_eq!(table.len(), 3);
        assert_eq!(execute_onexit_table(Some(&mut table)), 0);
        assert_eq!(*ORDER.lock().unwrap(), vec![2, 1]);
        assert!(table.is_empty());
    }

    #[test]
    fn table_can_be_reused_and_repeated_execution_is_empty() {
        let _guard = TEST_LOCK.lock().unwrap();
        ORDER.lock().unwrap().clear();
        let mut table = OnExitTable::new();
        table.register(Some(first));
        table.execute();
        table.execute();
        table.register(Some(second));
        table.execute();
        assert_eq!(*ORDER.lock().unwrap(), vec![1, 2]);
    }

    #[test]
    fn grows_beyond_initial_wine_capacity() {
        let mut table = OnExitTable::new();
        for _ in 0..65 {
            table.register(None);
        }
        assert_eq!(table.len(), 65);
        table.execute();
        assert!(table.is_empty());
    }

    #[test]
    fn atexit_registry_runs_lifo_once() {
        let _guard = TEST_LOCK.lock().unwrap();
        ORDER.lock().unwrap().clear();
        let mut registry = AtexitRegistry::default();
        assert_eq!(registry.atexit(first), 0);
        assert_eq!(registry.atexit(second), 0);
        registry.execute();
        registry.execute();
        assert_eq!(*ORDER.lock().unwrap(), vec![2, 1]);
    }

    #[test]
    fn ascii_predicates_follow_msvcrt_semantics() {
        assert!(is_ascii(0) && is_ascii(0x7f));
        assert!(!is_ascii(0x80) && !is_ascii(-1) && !is_ascii(0x100));
        assert_eq!(to_ascii(0xc1), 0x41);
        assert_eq!(to_ascii(0x7f), 0x7f);
        assert_eq!(to_ascii(-1), 0x7f);
    }

    #[test]
    fn csym_predicates_accept_ascii_identifier_characters() {
        for c in *b"aZ09_" {
            assert!(is_csym(i32::from(c)));
        }
        for c in [b' ', b'-', 127, 0x80, 0xe9] {
            assert!(!is_csym(i32::from(c)));
        }
        assert!(!is_csym(-1));
        assert!(!is_csym(256));
        assert!(is_csymf(i32::from(b'a')) && is_csymf(i32::from(b'_')));
        assert!(!is_csymf(i32::from(b'0')) && !is_csymf(-1) && !is_csymf(0x80));
    }
}
