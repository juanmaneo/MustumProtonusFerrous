//! Headless port of the deterministic helpers in Wine's `dlls/msvfw32`, the Video for Windows
//! library.
//!
//! Ported (names in `dlls/msvfw32/msvideo_main.c` and `dlls/msvfw32/drawdib.c`):
//! - `get_size_image`: [get_size_image].
//! - `num_colours`: [num_colours].
//! - `compare_fourcc`: [fourcc_eq_ignore_case]. Wine's callers test only whether the result is
//!   zero, so the port returns that test as a `bool`.
//!
//! Skipped: `VideoForWindowsVersion` and `DrawDibProfileDisplay` (constant replies, the latter a
//! FIXME stub); `wine_dbgstr_icerr` (debug text only); `ICInfo`, `ICOpen`, `ICSendMessage` and the
//! rest of the driver table, the registry and the `DrawDib` device contexts, the `MCIWnd` window
//! class and the file-name preview dialogs, which need OS, registry, window, file or COM state.
//!
//! Deviations from Wine, where the C is undefined behaviour:
//! - [get_size_image] wraps `width * depth` and `abs(height)`. In C, `int` overflow is undefined.
//!   Wine's gcc build wraps, and the sanitizer run of the reference flags these cases.
//! - `biClrUsed` values from 2^31 up are `int` conversions, which the C leaves
//!   implementation-defined. [num_colours] gives the same result as gcc.

mod dib;
mod fourcc;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use dib::{get_size_image, num_colours};
pub use fourcc::fourcc_eq_ignore_case;
