//! Port of the portable logic in Wine's `dlls/riched32/richedit.c` and `dlls/riched32/riched32.spec`.
//!
//! Ported:
//! - `DllMain` attach and detach handling: [`dll_main`], with the `DLL_*` values from
//!   `include/winnt.h`: [`DLL_PROCESS_ATTACH`], [`DLL_PROCESS_DETACH`], [`DLL_THREAD_ATTACH`],
//!   [`DLL_THREAD_DETACH`]
//! - `RICHED32_Register`, which fills in and registers the RichEdit 1.0 window class:
//!   [`register_class`], [`WndClassA`], with the `CS_*`, `COLOR_WINDOW`, `IDC_ARROW` and
//!   `RICHEDIT_CLASS10A` values: [`CS_DBLCLKS`], [`CS_HREDRAW`], [`CS_VREDRAW`], [`CS_GLOBALCLASS`],
//!   [`COLOR_WINDOW`], [`IDC_ARROW`], [`RICHEDIT_CLASS10A`]
//! - The OS calls that both functions make, behind the [`Host`] trait
//!
//! Skipped:
//! - `DllGetVersion`, exported by `riched32.spec` as a private `stdcall` function. Neither
//!   `richedit.c` nor the rest of `dlls/riched32` defines it, and it is not a `@ stub` export, so
//!   there is no body to port and no stub record to model.
//! - `RichEdit10ANSIWndProc`, the window procedure in riched20. The port takes its address as the
//!   `wnd_proc` argument.
//! - The `TRACE` calls and the debug channel.
//! - The results of `DisableThreadLibraryCalls`, `RegisterClassA` and `UnregisterClassA`, which the
//!   C code ignores. `RICHED32_Register` returns TRUE whatever `RegisterClassA` returns.
//! - The Windows handle types, `WINAPI` and the `__stdcall` convention: handles are plain integers.

mod richedit;

pub use richedit::{
    dll_main, register_class, Host, WndClassA, COLOR_WINDOW, CS_DBLCLKS, CS_GLOBALCLASS,
    CS_HREDRAW, CS_VREDRAW, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH,
    DLL_THREAD_DETACH, IDC_ARROW, RICHEDIT_CLASS10A,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
