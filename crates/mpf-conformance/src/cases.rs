//! Seeded conformance cases, one per ported behavior. Names follow wine's
//! `dlls/<dll>/tests` conventions where applicable.

use crate::Case;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NONCE: AtomicU64 = AtomicU64::new(0);

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let n = NONCE.fetch_add(1, Ordering::Relaxed);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("mpf-conf-{tag}-{t}-{n}"))
}

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn kernel32_lstrlen_semantics() -> Result<(), String> {
    // mirrors dlls/kernel32/tests/string.c lstrlenW expectations
    let s = w("hello");
    if mpf_kernel32::lstrlen_w(&s) != 5 {
        return Err("lstrlen_w miscounts".into());
    }
    Ok(())
}

fn kernel32_lstrcpyn_truncation() -> Result<(), String> {
    let src = w("abcdef");
    let mut dst = [0u16; 4];
    let n = mpf_kernel32::lstrcpyn_w(&mut dst, &src);
    if n != 6 || &dst != &[b'a' as u16, b'b' as u16, b'c' as u16, 0] {
        return Err("lstrcpyn_w wrong".into());
    }
    Ok(())
}

fn ntdll_rtl_compare() -> Result<(), String> {
    use mpf_ntdll::unicode::*;
    let a = WideString::from_utf8("abc");
    let b = WideString::from_utf8("abd");
    if rtl_compare_unicode_strings(&a, &b) >= 0 {
        return Err("compare wrong".into());
    }
    if rtl_compare_unicode_strings(&a, &WideString::from_utf8("ABD")) <= 0 {
        return Err("compare ordering wrong".into());
    }
    if !rtl_equal_unicode_string_ignore_case(&a, &WideString::from_utf8("ABC")) {
        return Err("ignore-case equal wrong".into());
    }
    if rtl_equal_unicode_string(&a, &WideString::from_utf8("ABC")) {
        return Err("exact equal wrong".into());
    }
    Ok(())
}

fn ntdll_ansi_roundtrip() -> Result<(), String> {
    use mpf_ntdll::unicode::*;
    let wide = WideString::from_utf8("abc");
    let ansi = rtl_unicode_string_to_ansi_string(&wide);
    if ansi != b"abc" || rtl_ansi_string_to_unicode_string(&ansi) != wide {
        return Err("ansi roundtrip wrong".into());
    }
    Ok(())
}

fn ntdll_heap() -> Result<(), String> {
    use mpf_ntdll::heap::*;
    let heap = rtl_process_heap();
    let p = rtl_allocate_heap(heap, 128, true);
    if p.is_null() {
        return Err("alloc failed".into());
    }
    let bytes = unsafe { std::slice::from_raw_parts(p, 128) };
    if !bytes.iter().all(|&b| b == 0) {
        return Err("not zeroed".into());
    }
    if rtl_free_heap(heap, p) != mpf_ntdll::ntstatus::STATUS_SUCCESS {
        return Err("free failed".into());
    }
    Ok(())
}

fn ntdll_critical_section() -> Result<(), String> {
    use mpf_ntdll::critical_section::CriticalSection;
    let cs = CriticalSection::new();
    let _g1 = cs.enter();
    let _g2 = cs.enter();
    if cs.try_enter().is_none() {
        // same thread: recursive try must succeed
        return Err("recursive try_enter failed".into());
    }
    Ok(())
}

fn ntdll_object_handles() -> Result<(), String> {
    use mpf_ntdll::ntstatus::{is_success, STATUS_INVALID_HANDLE};
    use mpf_ntdll::object::ObjectManager;
    let mut om = ObjectManager::new();
    let h = om.allocate_handle(3);
    if om.query_object(h) != Some(3) || !is_success(om.close_handle(h)) {
        return Err("handle lifecycle wrong".into());
    }
    if om.close_handle(h) != STATUS_INVALID_HANDLE {
        return Err("double close wrong".into());
    }
    Ok(())
}

fn ntdll_exception_guard() -> Result<(), String> {
    use mpf_ntdll::exception::{guard, EXCEPTION_RUST_PANIC};
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = guard(|| -> u32 { panic!("boom") });
    let _ = std::panic::set_hook(prev);
    match r {
        Err(e) if e.code == EXCEPTION_RUST_PANIC => Ok(()),
        _ => Err("panic not converted".into()),
    }
}

fn pe_parse_headers() -> Result<(), String> {
    let data = mpf_pe::testing::build_test_pe64();
    let pe = mpf_pe::PeImage::parse(&data).map_err(|e| e.to_string())?;
    if pe.machine() != mpf_pe::MACHINE_AMD64 || !pe.is_64bit() || pe.size_of_image() != 0x2000 {
        return Err("headers wrong".into());
    }
    Ok(())
}

fn pe_tables() -> Result<(), String> {
    let data = mpf_pe::testing::build_test_pe64();
    let pe = mpf_pe::PeImage::parse(&data).map_err(|e| e.to_string())?;
    let exports = pe.exports().map_err(|e| e.to_string())?;
    let imports = pe.imports().map_err(|e| e.to_string())?;
    let relocs = pe.relocations().map_err(|e| e.to_string())?;
    let delay = pe.delay_imports().map_err(|e| e.to_string())?;
    if exports.len() != 1 || exports[0].name != "TestFunc" {
        return Err("exports wrong".into());
    }
    if imports.len() != 1 || imports[0].function != "BaseFunc" {
        return Err("imports wrong".into());
    }
    if relocs.len() != 1 || relocs[0].entries.len() != 1 || relocs[0].entries[0].reloc_type != 10 {
        return Err("relocs wrong".into());
    }
    if pe.tls_directory().map_err(|e| e.to_string())?.is_none() {
        return Err("tls missing".into());
    }
    if delay.len() != 1 || delay[0].dll != "DELAY.dll" {
        return Err("delay wrong".into());
    }
    Ok(())
}

fn pe_mapped() -> Result<(), String> {
    let file = mpf_pe::testing::build_test_pe64();
    let mut mapped = vec![0u8; 0x2000];
    mapped[..0x200].copy_from_slice(&file[..0x200]);
    mapped[0x1000..0x1300].copy_from_slice(&file[0x200..0x500]);
    let pe = mpf_pe::PeImage::parse_mapped(&mapped).map_err(|e| e.to_string())?;
    if pe.exports().map_err(|e| e.to_string())?[0].rva != 0x1080 {
        return Err("mapped exports wrong".into());
    }
    Ok(())
}

fn loader_pipeline() -> Result<(), String> {
    let data = mpf_pe::testing::build_test_pe64();
    let mut m = mpf_loader::map_image(&data).map_err(|e| e.to_string())?;
    let applied = m.relocate(0x5000_0000);
    if applied != 1 {
        return Err("reloc count wrong".into());
    }
    let mut resolver = |dll: &str, func: &str, _ord: Option<u16>| {
        if dll == "KERNEL32.dll" && func == "BaseFunc" {
            Some(0xDEAD_BEEF)
        } else {
            None
        }
    };
    if m.resolve_imports(&mut resolver) != 1 {
        return Err("import resolve wrong".into());
    }
    let v = u64::from_le_bytes(m.image()[0x1170..0x1178].try_into().unwrap());
    if v != 0xDEAD_BEEF {
        return Err("IAT not patched".into());
    }
    Ok(())
}

fn winnt_layouts() -> Result<(), String> {
    use core::mem::offset_of;
    if offset_of!(mpf_winnt::Peb, ldr) != 0x18
        || offset_of!(mpf_winnt::Teb, process_environment_block) != 0x60
    {
        return Err("PEB/TEB offsets wrong".into());
    }
    Ok(())
}

fn wineboot_prefix() -> Result<(), String> {
    let root = temp_dir("prefix");
    let prefix = mpf_wineboot::init_prefix(&root).map_err(|e| e.to_string())?;
    let system = std::fs::read_to_string(prefix.system_reg()).map_err(|e| e.to_string())?;
    if !system.starts_with(mpf_advapi32::REG_HEADER) || !system.contains("11.22") {
        return Err("system.reg wrong".into());
    }
    mpf_advapi32::init_from_prefix(&root).map_err(|e| e.to_string())?;
    std::fs::remove_dir_all(&root).map_err(|e| e.to_string())?;
    Ok(())
}

fn advapi32_registry() -> Result<(), String> {
    use mpf_advapi32::{RegValue, Registry, RootKey};
    let mut reg = Registry::new();
    let key = reg
        .reg_create_key_ex_w(RootKey::LocalMachine, "Software\\MPF")
        .map_err(|e| format!("create: {e}"))?;
    if reg.reg_set_value_ex_w(key, "version", RegValue::String("11.22".into())) != 0 {
        return Err("set failed".into());
    }
    if reg.reg_query_value_ex_w(key, "version").map_err(|e| format!("query: {e}"))?
        != RegValue::String("11.22".into())
    {
        return Err("query wrong".into());
    }
    Ok(())
}

fn kernel32_file() -> Result<(), String> {
    use mpf_kernel32::file::*;
    let dir = temp_dir("file");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = w(&dir.join("data.txt").to_string_lossy());
    let h = create_file_w(&path, GENERIC_READ | GENERIC_WRITE, true);
    if h == INVALID_HANDLE_VALUE {
        return Err("create failed".into());
    }
    write_file(h, b"mpf").map_err(|e| format!("write: {e}"))?;
    close_handle(h);
    let h = create_file_w(&path, GENERIC_READ, false);
    let mut buf = [0u8; 8];
    let n = read_file(h, &mut buf).map_err(|e| format!("read: {e}"))?;
    close_handle(h);
    if &buf[..n] != b"mpf" {
        return Err("content wrong".into());
    }
    Ok(())
}

fn kernel32_module() -> Result<(), String> {
    use mpf_kernel32::module::*;
    let data = mpf_pe::testing::build_test_pe64();
    let module = mpf_loader::map_image(&data).map_err(|e| e.to_string())?;
    let base = module.base();
    let size = module.size_of_image;
    register_module("mpf_test.dll", base, size);
    set_current_exe(base);
    if get_module_handle_w(None) as usize != base {
        return Err("current exe wrong".into());
    }
    if get_proc_address(base as *const core::ffi::c_void, "TestFunc") != Some(base + 0x1080) {
        return Err("get_proc_address wrong".into());
    }
    std::mem::forget(module);
    Ok(())
}

fn kernel32_thread() -> Result<(), String> {
    use mpf_kernel32::thread::*;
    extern "system" fn noop(_p: *mut core::ffi::c_void) -> u32 {
        0
    }
    let h = create_thread(noop, core::ptr::null_mut());
    if wait_for_thread(h, 0) != WAIT_OBJECT_0 {
        return Err("wait wrong".into());
    }
    if get_current_thread_id() == 0 {
        return Err("thread id 0".into());
    }
    Ok(())
}

fn kernel32_process() -> Result<(), String> {
    use mpf_kernel32::process::*;
    if get_current_process_id() != std::process::id() {
        return Err("pid wrong".into());
    }
    if get_current_process() as isize != -1 {
        return Err("pseudo handle wrong".into());
    }
    Ok(())
}

/// All seeded cases.
pub fn all() -> Vec<Case> {
    vec![
        Case { name: "kernel32!lstrlenW semantics", run: kernel32_lstrlen_semantics },
        Case { name: "kernel32!lstrcpynW truncation", run: kernel32_lstrcpyn_truncation },
        Case { name: "ntdll!RtlCompareUnicodeString/Equal", run: ntdll_rtl_compare },
        Case { name: "ntdll!ANSI<->Unicode roundtrip", run: ntdll_ansi_roundtrip },
        Case { name: "ntdll!RtlAllocateHeap/RtlFreeHeap", run: ntdll_heap },
        Case { name: "ntdll!critical section recursion", run: ntdll_critical_section },
        Case { name: "ntdll!object handle table", run: ntdll_object_handles },
        Case { name: "ntdll!exception guard catches panic", run: ntdll_exception_guard },
        Case { name: "pe!parse headers", run: pe_parse_headers },
        Case { name: "pe!exports/imports/relocs/TLS/delay", run: pe_tables },
        Case { name: "pe!parse_mapped export lookup", run: pe_mapped },
        Case { name: "loader!map/relocate/resolve", run: loader_pipeline },
        Case { name: "winnt!PEB/TEB x64 offsets", run: winnt_layouts },
        Case { name: "wineboot!prefix + hives", run: wineboot_prefix },
        Case { name: "advapi32!Reg* roundtrip", run: advapi32_registry },
        Case { name: "kernel32!CreateFileW/ReadFile/WriteFile", run: kernel32_file },
        Case { name: "kernel32!GetModuleHandle/GetProcAddress", run: kernel32_module },
        Case { name: "kernel32!CreateThread/WaitForSingleObject", run: kernel32_thread },
        Case { name: "kernel32!GetCurrentProcess(Id)", run: kernel32_process },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_cases_pass() {
        let cases = all();
        let (passed, failed) = crate::run(&cases);
        assert_eq!(failed, 0, "conformance failures");
        assert_eq!(passed, cases.len());
    }
}
