//! Module (GetModuleHandle/GetProcAddress) support, mirroring
//! `dlls/kernel32/module.c` + ntdll's LDR lookup.

use core::ffi::c_void;
use mpf_pe::PeImage;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub base: usize,
    pub size: u32,
}

struct ModuleTable {
    modules: Vec<ModuleInfo>,
    current_exe: usize,
}

static MODULES: Mutex<Option<ModuleTable>> = Mutex::new(None);

fn with_table<R>(f: impl FnOnce(&mut ModuleTable) -> R) -> R {
    let mut guard = MODULES.lock().unwrap();
    f(guard.get_or_insert(ModuleTable {
        modules: Vec::new(),
        current_exe: 0,
    }))
}

fn normalize(name: &str) -> String {
    let n = name.trim_end_matches(['\0', '.']).to_ascii_lowercase();
    if n.ends_with("dll") || n.ends_with("exe") {
        n
    } else {
        format!("{n}.dll")
    }
}

/// Registers a loaded module (as the loader does when adding an LDR entry).
pub fn register_module(name: &str, base: usize, size: u32) {
    with_table(|t| {
        t.modules.push(ModuleInfo {
            name: name.to_string(),
            base,
            size,
        })
    });
}

/// Marks the current executable's base (for GetModuleHandleW(NULL)).
pub fn set_current_exe(base: usize) {
    with_table(|t| t.current_exe = base);
}

/// GetModuleHandleW: `None` → the current exe; otherwise case-insensitive
/// name match with or without the `.dll` extension.
pub fn get_module_handle_w(name: Option<&str>) -> *mut c_void {
    with_table(|t| match name {
        None => t.current_exe as *mut c_void,
        Some(n) => {
            let want = normalize(n);
            t.modules
                .iter()
                .find(|m| normalize(&m.name) == want)
                .map(|m| m.base as *mut c_void)
                .unwrap_or(core::ptr::null_mut())
        }
    })
}

/// GetModuleFileNameW: writes the module's name into `buf` (NUL-terminated),
/// returns the length written.
pub fn get_module_file_name_w(base: *const c_void, buf: &mut [u16]) -> u32 {
    with_table(|t| {
        let name = t
            .modules
            .iter()
            .find(|m| m.base == base as usize)
            .map(|m| m.name.clone())
            .unwrap_or_default();
        let units: Vec<u16> = name.encode_utf16().collect();
        let n = units.len().min(buf.len().saturating_sub(1));
        buf[..n].copy_from_slice(&units[..n]);
        if !buf.is_empty() {
            buf[n] = 0;
        }
        n as u32
    })
}

/// GetProcAddress: resolves an export by name from a mapped module image.
pub fn get_proc_address(base: *const c_void, name: &str) -> Option<usize> {
    let (base, size) = with_table(|t| {
        t.modules
            .iter()
            .find(|m| m.base == base as usize)
            .map(|m| (m.base, m.size))
    })?;
    if base == 0 || size == 0 {
        return None;
    }
    let image = unsafe { std::slice::from_raw_parts(base as *const u8, size as usize) };
    let pe = PeImage::parse_mapped(image).ok()?;
    let exports = pe.exports().ok()?;
    exports
        .iter()
        .find(|e| e.name == name)
        .map(|e| base + e.rva as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mpf_pe::testing::build_test_pe64;

    fn mapped_module() -> (usize, u32) {
        let module = mpf_loader::map_image(&build_test_pe64()).unwrap();
        let base = module.base();
        let size = module.size_of_image;
        std::mem::forget(module); // keep the mapping alive for the test process
        (base, size)
    }

    #[test]
    fn module_handle_lookup_is_case_insensitive() {
        let (base, size) = mapped_module();
        register_module("mpf_test.dll", base, size);
        set_current_exe(base);
        assert_eq!(get_module_handle_w(None) as usize, base);
        assert_eq!(get_module_handle_w(Some("MPF_TEST.DLL")) as usize, base);
        assert_eq!(get_module_handle_w(Some("mpf_test")) as usize, base);
        assert!(get_module_handle_w(Some("other.dll")).is_null());
    }

    #[test]
    fn module_file_name_is_written_nul_terminated() {
        let (base, size) = mapped_module();
        register_module("Example.dll", base, size);
        let mut buf = [0u16; 64];
        let n = get_module_file_name_w(base as *const c_void, &mut buf);
        let expect: Vec<u16> = "Example.dll\0".encode_utf16().collect();
        assert_eq!(n, 11);
        assert_eq!(&buf[..12], &expect[..]);
        let mut tiny = [0u16; 4];
        let n2 = get_module_file_name_w(base as *const c_void, &mut tiny);
        let expect2: Vec<u16> = "Exa\0".encode_utf16().collect();
        assert_eq!(n2, 3);
        assert_eq!(&tiny[..4], &expect2[..]);
    }

    #[test]
    fn proc_address_resolves_exports_from_mapped_image() {
        let (base, size) = mapped_module();
        register_module("mpf_test.dll", base, size);
        assert_eq!(
            get_proc_address(base as *const c_void, "TestFunc"),
            Some(base + 0x1080)
        );
        assert_eq!(get_proc_address(base as *const c_void, "Missing"), None);
        assert_eq!(get_proc_address(core::ptr::null(), "TestFunc"), None);
    }
}
