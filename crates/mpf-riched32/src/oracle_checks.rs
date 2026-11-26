//! Oracle-derived test vectors for the ported riched32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `DllMain` and `RICHED32_Register`
//! from `dlls/riched32/richedit.c` under `target/canon/riched32/` with
//! `gcc -fsanitize=address,undefined -g`. Shims stand in for the Windows types, constants and OS
//! calls, and the shimmed OS calls record their arguments instead of doing anything. The harness
//! printed 18 rows, and ASan and UBSan reported nothing for any of them, so no vector is excluded.
//!
//! The capture ran on x86_64, where `sizeof (void *)` is 8, so the rows print `cbwnd=8`. The tests
//! compute that value with `size_of::<usize>()`, which gives the same result on that target.
//!
//! The rows print `proc=ok` when `lpfnWndProc` is `RichEdit10ANSIWndProc`. The tests pass
//! [`WND_PROC`] in that position.

use super::*;

/// The window procedure address the tests pass in, standing in for `RichEdit10ANSIWndProc`.
pub(crate) const WND_PROC: usize = 0x7000;

/// The cursor handle the recorder's `LoadCursorA` returns, which is `0x1234` in the harness.
pub(crate) const CURSOR: usize = 0x1234;

/// A module handle, `0x400000`, the usual image base.
pub(crate) const HINST: usize = 0x400000;

/// One OS call, as the recorder saw it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Call {
    DisableThreadLibraryCalls(usize),
    LoadCursorA(usize, usize),
    RegisterClassA(WndClassA),
    UnregisterClassA(String, usize),
}

/// A [`Host`] that records each call in order. `load_cursor_a` returns `cursor`.
pub(crate) struct Recorder {
    pub(crate) cursor: usize,
    pub(crate) calls: Vec<Call>,
}

impl Default for Recorder {
    fn default() -> Self {
        Recorder {
            cursor: CURSOR,
            calls: Vec::new(),
        }
    }
}

impl Host for Recorder {
    fn disable_thread_library_calls(&mut self, module: usize) {
        self.calls.push(Call::DisableThreadLibraryCalls(module));
    }

    fn load_cursor_a(&mut self, instance: usize, name: usize) -> usize {
        self.calls.push(Call::LoadCursorA(instance, name));
        self.cursor
    }

    fn register_class_a(&mut self, class: &WndClassA) {
        self.calls.push(Call::RegisterClassA(*class));
    }

    fn unregister_class_a(&mut self, class_name: &str, instance: usize) {
        self.calls
            .push(Call::UnregisterClassA(class_name.to_string(), instance));
    }
}

/// The `WNDCLASSA` the harness prints for `RICHED32_Register`, with `WND_PROC` as its window procedure.
pub(crate) fn printed_class() -> WndClassA {
    WndClassA {
        style: 0x400b,
        lpfn_wnd_proc: WND_PROC,
        cb_cls_extra: 0,
        cb_wnd_extra: std::mem::size_of::<usize>() as i32,
        h_instance: 0,
        h_icon: 0,
        h_cursor: CURSOR,
        hbr_background: 6,
        lpsz_menu_name: None,
        lpsz_class_name: "RICHEDIT",
    }
}

/// Calls `dll_main` the way the harness's `row` does, and returns the result and the recorded calls.
fn dll_main_row(hinst: usize, reason: u32, reserved: usize) -> (bool, Vec<Call>) {
    let mut rec = Recorder::default();
    let ret = dll_main(&mut rec, hinst, reason, reserved, WND_PROC);
    (ret, rec.calls)
}

#[test]
fn oracle_dll_main_rows() {
    // row 01 reason=1 reserved=0 hinst=0x400000 -> ret=1 calls=DisableThreadLibraryCalls(0x400000) | LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let (ret, calls) = dll_main_row(HINST, DLL_PROCESS_ATTACH, 0);
    assert!(ret);
    assert_eq!(
        calls,
        vec![
            Call::DisableThreadLibraryCalls(0x400000),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );

    // row 02 reason=1 reserved=1 hinst=0x400000 -> ret=1 calls=DisableThreadLibraryCalls(0x400000) | LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let (ret, calls) = dll_main_row(HINST, DLL_PROCESS_ATTACH, 1);
    assert!(ret);
    assert_eq!(
        calls,
        vec![
            Call::DisableThreadLibraryCalls(0x400000),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );

    // row 03 reason=1 reserved=0 hinst=0 -> ret=1 calls=DisableThreadLibraryCalls(0) | LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let (ret, calls) = dll_main_row(0, DLL_PROCESS_ATTACH, 0);
    assert!(ret);
    assert_eq!(
        calls,
        vec![
            Call::DisableThreadLibraryCalls(0),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );

    // row 04 reason=1 reserved=1 hinst=0 -> ret=1 calls=DisableThreadLibraryCalls(0) | LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let (ret, calls) = dll_main_row(0, DLL_PROCESS_ATTACH, 1);
    assert!(ret);
    assert_eq!(
        calls,
        vec![
            Call::DisableThreadLibraryCalls(0),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );

    // row 05 reason=0 reserved=0 hinst=0x400000 -> ret=1 calls=UnregisterClassA(RICHEDIT,0)
    let (ret, calls) = dll_main_row(HINST, DLL_PROCESS_DETACH, 0);
    assert!(ret);
    assert_eq!(
        calls,
        vec![Call::UnregisterClassA("RICHEDIT".to_string(), 0)]
    );

    // row 06 reason=0 reserved=1 hinst=0x400000 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(HINST, DLL_PROCESS_DETACH, 1);
    assert!(ret);
    assert!(calls.is_empty());

    // row 07 reason=0 reserved=0 hinst=0 -> ret=1 calls=UnregisterClassA(RICHEDIT,0)
    let (ret, calls) = dll_main_row(0, DLL_PROCESS_DETACH, 0);
    assert!(ret);
    assert_eq!(
        calls,
        vec![Call::UnregisterClassA("RICHEDIT".to_string(), 0)]
    );

    // row 08 reason=2 reserved=0 hinst=0x400000 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(HINST, DLL_THREAD_ATTACH, 0);
    assert!(ret);
    assert!(calls.is_empty());

    // row 09 reason=3 reserved=0 hinst=0x400000 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(HINST, DLL_THREAD_DETACH, 0);
    assert!(ret);
    assert!(calls.is_empty());

    // row 10 reason=3 reserved=1 hinst=0x400000 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(HINST, DLL_THREAD_DETACH, 1);
    assert!(ret);
    assert!(calls.is_empty());

    // row 11 reason=4 reserved=0 hinst=0x400000 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(HINST, 4, 0);
    assert!(ret);
    assert!(calls.is_empty());

    // row 12 reason=4294967295 reserved=0 hinst=0x400000 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(HINST, u32::MAX, 0);
    assert!(ret);
    assert!(calls.is_empty());

    // row 13 reason=2 reserved=1 hinst=0 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(0, DLL_THREAD_ATTACH, 1);
    assert!(ret);
    assert!(calls.is_empty());

    // row 14 reason=1 reserved=0 hinst=0x7ff00000 -> ret=1 calls=DisableThreadLibraryCalls(0x7ff00000) | LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let (ret, calls) = dll_main_row(0x7ff0_0000, DLL_PROCESS_ATTACH, 0);
    assert!(ret);
    assert_eq!(
        calls,
        vec![
            Call::DisableThreadLibraryCalls(0x7ff0_0000),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );

    // row 15 reason=0 reserved=0 hinst=0x7ff00000 -> ret=1 calls=UnregisterClassA(RICHEDIT,0)
    let (ret, calls) = dll_main_row(0x7ff0_0000, DLL_PROCESS_DETACH, 0);
    assert!(ret);
    assert_eq!(
        calls,
        vec![Call::UnregisterClassA("RICHEDIT".to_string(), 0)]
    );

    // row 16 reason=0 reserved=1 hinst=0 -> ret=1 calls=-
    let (ret, calls) = dll_main_row(0, DLL_PROCESS_DETACH, 1);
    assert!(ret);
    assert!(calls.is_empty());

    // row 17 reason=1 reserved=1 hinst=0x400000 -> ret=1 calls=DisableThreadLibraryCalls(0x400000) | LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let (ret, calls) = dll_main_row(HINST, DLL_PROCESS_ATTACH, 1);
    assert!(ret);
    assert_eq!(
        calls,
        vec![
            Call::DisableThreadLibraryCalls(0x400000),
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );
}

#[test]
fn oracle_register_row() {
    // row 18 RICHED32_Register -> ret=1 calls=LoadCursorA(0,0x7f00)=0x1234 | RegisterClassA{style=0x400b cbcls=0 cbwnd=8 inst=0 icon=0 cur=0x1234 bg=0x6 menu=NULL name=RICHEDIT proc=ok}
    let mut rec = Recorder::default();
    assert!(register_class(&mut rec, WND_PROC));
    assert_eq!(
        rec.calls,
        vec![
            Call::LoadCursorA(0, IDC_ARROW),
            Call::RegisterClassA(printed_class()),
        ]
    );
}
