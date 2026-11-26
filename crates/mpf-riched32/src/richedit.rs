/// `DLL_PROCESS_DETACH` from `include/winnt.h`.
pub const DLL_PROCESS_DETACH: u32 = 0;

/// `DLL_PROCESS_ATTACH` from `include/winnt.h`.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// `DLL_THREAD_ATTACH` from `include/winnt.h`.
pub const DLL_THREAD_ATTACH: u32 = 2;

/// `DLL_THREAD_DETACH` from `include/winnt.h`.
pub const DLL_THREAD_DETACH: u32 = 3;

/// `CS_VREDRAW` from `include/winuser.rh`.
pub const CS_VREDRAW: u32 = 0x0000_0001;

/// `CS_HREDRAW` from `include/winuser.rh`.
pub const CS_HREDRAW: u32 = 0x0000_0002;

/// `CS_DBLCLKS` from `include/winuser.rh`.
pub const CS_DBLCLKS: u32 = 0x0000_0008;

/// `CS_GLOBALCLASS` from `include/winuser.rh`.
pub const CS_GLOBALCLASS: u32 = 0x0000_4000;

/// `COLOR_WINDOW` from `include/winuser.h`.
pub const COLOR_WINDOW: usize = 5;

/// `IDC_ARROW`, which is `MAKEINTRESOURCE(32512)` in `include/winuser.h`.
pub const IDC_ARROW: usize = 32512;

/// `RICHEDIT_CLASS10A` from `include/richedit.h`.
pub const RICHEDIT_CLASS10A: &str = "RICHEDIT";

/// The `WNDCLASSA` that `RICHED32_Register` fills in. `lpfn_wnd_proc` is the window procedure's
/// address, as the caller supplies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WndClassA {
    pub style: u32,
    pub lpfn_wnd_proc: usize,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: usize,
    pub h_icon: usize,
    pub h_cursor: usize,
    pub hbr_background: usize,
    pub lpsz_menu_name: Option<&'static str>,
    pub lpsz_class_name: &'static str,
}

/// The OS calls that `DllMain` and `RICHED32_Register` make. Only `LoadCursorA` returns a value
/// the port uses; the C code stores it in the class without checking it.
pub trait Host {
    /// `DisableThreadLibraryCalls(module)`.
    fn disable_thread_library_calls(&mut self, module: usize);

    /// `LoadCursorA(instance, name)`, which returns the cursor handle.
    fn load_cursor_a(&mut self, instance: usize, name: usize) -> usize;

    /// `RegisterClassA(&class)`.
    fn register_class_a(&mut self, class: &WndClassA);

    /// `UnregisterClassA(class_name, instance)`.
    fn unregister_class_a(&mut self, class_name: &str, instance: usize);
}

/// `RICHED32_Register`: fills in the RichEdit 1.0 window class and registers it. Returns TRUE
/// whatever `RegisterClassA` does.
pub fn register_class(host: &mut impl Host, wnd_proc: usize) -> bool {
    let h_cursor = host.load_cursor_a(0, IDC_ARROW);
    let class = WndClassA {
        style: CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW | CS_GLOBALCLASS,
        lpfn_wnd_proc: wnd_proc,
        cb_cls_extra: 0,
        // `sizeof(void *)` on the target this is built for.
        cb_wnd_extra: std::mem::size_of::<usize>() as i32,
        h_instance: 0,
        h_icon: 0,
        h_cursor,
        hbr_background: COLOR_WINDOW + 1,
        lpsz_menu_name: None,
        lpsz_class_name: RICHEDIT_CLASS10A,
    };
    host.register_class_a(&class);
    true
}

/// `DllMain`. Attaching the process disables thread library calls and registers the class.
/// Detaching unregisters the class unless `lpvReserved` is set. Every other reason returns TRUE
/// without an OS call. `wnd_proc` is the address of `RichEdit10ANSIWndProc` in riched20.
pub fn dll_main(
    host: &mut impl Host,
    hinst_dll: usize,
    fdw_reason: u32,
    lpv_reserved: usize,
    wnd_proc: usize,
) -> bool {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            host.disable_thread_library_calls(hinst_dll);
            register_class(host, wnd_proc)
        }
        DLL_PROCESS_DETACH => {
            if lpv_reserved == 0 {
                host.unregister_class_a(RICHEDIT_CLASS10A, 0);
            }
            true
        }
        _ => true,
    }
}
