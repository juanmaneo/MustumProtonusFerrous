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

    /// Registers `image` as a module under `name`. The bytes are leaked so
    /// the address stays valid (and unique) for the rest of the process.
    fn register_image(name: &str, image: Vec<u8>) -> usize {
        let image: &'static [u8] = Box::leak(image.into_boxed_slice());
        let base = image.as_ptr() as usize;
        register_module(name, base, image.len() as u32);
        base
    }

    // The module table is process-wide and tests run in parallel, so each
    // test registers its modules under a name no other test uses.
    #[test]
    fn module_handle_lookup_is_case_insensitive() {
        let (base, size) = mapped_module();
        register_module("handle_lookup.dll", base, size);
        set_current_exe(base);
        assert_eq!(get_module_handle_w(None) as usize, base);
        assert_eq!(
            get_module_handle_w(Some("HANDLE_LOOKUP.DLL")) as usize,
            base
        );
        assert_eq!(get_module_handle_w(Some("handle_lookup")) as usize, base);
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
    fn module_file_name_handles_degenerate_buffers() {
        let (base, size) = mapped_module();
        register_module("Degenerate.dll", base, size);
        // No room for anything, not even the NUL: nothing is written.
        assert_eq!(get_module_file_name_w(base as *const c_void, &mut []), 0);
        // An unknown module has an empty name, still NUL-terminated.
        let mut buf = [0xFFFFu16; 4];
        assert_eq!(
            get_module_file_name_w(0xdead_2000 as *const c_void, &mut buf),
            0
        );
        assert_eq!(buf, [0, 0xFFFF, 0xFFFF, 0xFFFF]);
    }

    #[test]
    fn proc_address_resolves_exports_from_mapped_image() {
        let (base, size) = mapped_module();
        register_module("proc_address.dll", base, size);
        assert_eq!(
            get_proc_address(base as *const c_void, "TestFunc"),
            Some(base + 0x1080)
        );
        assert_eq!(get_proc_address(base as *const c_void, "Missing"), None);
        assert_eq!(get_proc_address(core::ptr::null(), "TestFunc"), None);
    }

    #[test]
    fn proc_address_is_none_for_modules_it_cannot_read() {
        let unknown = 0xdead_0000usize;
        assert_eq!(get_proc_address(unknown as *const c_void, "F"), None);

        register_module("zero_size.dll", 0xdead_1000, 0);
        assert_eq!(get_proc_address(0xdead_1000 as *const c_void, "F"), None);

        register_module("null_base.dll", 0, 16);
        assert_eq!(get_proc_address(core::ptr::null(), "F"), None);

        let not_pe = register_image("not_pe.dll", vec![0; 64]);
        assert_eq!(get_proc_address(not_pe as *const c_void, "F"), None);
    }

    #[test]
    fn proc_address_is_none_when_the_export_table_is_corrupt() {
        let (base, size) = mapped_module();
        // SAFETY: `mapped_module` leaks the mapping, so it outlives this read.
        let mut image =
            unsafe { std::slice::from_raw_parts(base as *const u8, size as usize) }.to_vec();
        // The export directory is at RVA 0x1000; point its function table
        // (offset 28) far outside the image.
        image[0x1000 + 28..][..4].copy_from_slice(&0xFFFF_0000u32.to_le_bytes());
        let bad = register_image("corrupt_exports.dll", image);
        assert_eq!(get_proc_address(bad as *const c_void, "TestFunc"), None);
    }
}
