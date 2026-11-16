//! File basics, mirroring `dlls/kernel32/file.c` (CreateFileW/ReadFile/
//! WriteFile/CloseHandle) over std::fs with a handle table, like ntdll's
//! object manager does for real handles.

use mpf_ntdll::object::ObjectManager;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::Mutex;

/// INVALID_HANDLE_VALUE
pub const INVALID_HANDLE_VALUE: u64 = u64::MAX;
pub const GENERIC_READ: u32 = 0x8000_0000;
pub const GENERIC_WRITE: u32 = 0x4000_0000;

/// Win32 error codes used here.
pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_INVALID_HANDLE: u32 = 6;

struct FileTable {
    om: ObjectManager,
    files: HashMap<u64, File>,
}

static FILES: Mutex<Option<FileTable>> = Mutex::new(None);

fn with_table<R>(f: impl FnOnce(&mut FileTable) -> R) -> R {
    let mut guard = FILES.lock().unwrap();
    f(guard.get_or_insert(FileTable {
        om: ObjectManager::new(),
        files: HashMap::new(),
    }))
}

/// Converts a NUL-terminated UTF-16 path to a Rust String.
pub fn wide_to_string(path: &[u16]) -> String {
    let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
    path[..end]
        .iter()
        .filter_map(|&c| char::from_u32(u32::from(c)))
        .collect()
}

/// CreateFileW: opens `path` (read always; write if GENERIC_WRITE; created
/// if `create`). Returns a handle, or INVALID_HANDLE_VALUE on failure.
pub fn create_file_w(path: &[u16], desired_access: u32, create: bool) -> u64 {
    let path = wide_to_string(path);
    let mut options = OpenOptions::new();
    options.read(true);
    if desired_access & GENERIC_WRITE != 0 {
        options.write(true);
    }
    if create {
        options.create(true);
    }
    let file = match options.open(&path) {
        Ok(f) => f,
        Err(_) => return INVALID_HANDLE_VALUE,
    };
    with_table(|t| {
        let handle = t.om.allocate_handle(1);
        t.files.insert(handle, file);
        handle
    })
}

/// ReadFile: reads up to `buf.len()` bytes; returns the byte count.
pub fn read_file(handle: u64, buf: &mut [u8]) -> Result<usize, u32> {
    with_table(|t| match t.files.get_mut(&handle) {
        Some(f) => f.read(buf).map_err(|_| ERROR_INVALID_HANDLE),
        None => Err(ERROR_INVALID_HANDLE),
    })
}

/// WriteFile: writes `buf`; returns the byte count.
pub fn write_file(handle: u64, buf: &[u8]) -> Result<usize, u32> {
    with_table(|t| match t.files.get_mut(&handle) {
        Some(f) => f.write(buf).map_err(|_| ERROR_INVALID_HANDLE),
        None => Err(ERROR_INVALID_HANDLE),
    })
}

/// CloseHandle on a file handle.
pub fn close_handle(handle: u64) -> u32 {
    with_table(|t| match t.files.remove(&handle) {
        Some(_) => {
            t.om.close_handle(handle);
            ERROR_SUCCESS
        }
        None => ERROR_INVALID_HANDLE,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> Vec<u16> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("mpf-k32-{name}-{nonce}.txt"));
        p.to_string_lossy()
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect()
    }

    #[test]
    fn wide_to_string_stops_at_nul() {
        let w: Vec<u16> = "C:\\temp\\x.txt\0garbage".encode_utf16().collect();
        assert_eq!(wide_to_string(&w), "C:\\temp\\x.txt");
        assert_eq!(wide_to_string(&[]), "");
    }

    #[test]
    fn create_write_read_close_roundtrip() {
        let path = temp_path("rw");
        let h = create_file_w(&path, GENERIC_READ | GENERIC_WRITE, true);
        assert_ne!(h, INVALID_HANDLE_VALUE);
        assert_eq!(write_file(h, b"hello mpf").unwrap(), 9);
        assert_eq!(close_handle(h), ERROR_SUCCESS);

        let h = create_file_w(&path, GENERIC_READ, false);
        assert_ne!(h, INVALID_HANDLE_VALUE);
        let mut buf = [0u8; 16];
        let n = read_file(h, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"hello mpf");
        assert_eq!(close_handle(h), ERROR_SUCCESS);
        std::fs::remove_file(wide_to_string(&path)).unwrap();
    }

    #[test]
    fn missing_file_without_create_fails() {
        let path = temp_path("missing");
        assert_eq!(
            create_file_w(&path, GENERIC_READ, false),
            INVALID_HANDLE_VALUE
        );
    }

    #[test]
    fn bad_handles_report_errors() {
        let mut buf = [0u8; 4];
        assert_eq!(read_file(9999, &mut buf), Err(ERROR_INVALID_HANDLE));
        assert_eq!(write_file(9999, b"x"), Err(ERROR_INVALID_HANDLE));
        assert_eq!(close_handle(9999), ERROR_INVALID_HANDLE);
    }
}
