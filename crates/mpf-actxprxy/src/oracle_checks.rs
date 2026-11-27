//! Oracle-derived test vectors for the ported actxprxy logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the four `usrmarshal.c` Next
//! functions and of `NdrDllCanUnloadNow` (`dlls/rpcrt4/cpsf.c`), with the `DllCanUnloadNow`
//! expansion from `include/rpcproxy.h`, using `gcc -fsanitize=address,undefined -g` under
//! `target/canon/actxprxy/oracle.c`. The harness printed 20 rows: 16 Next rows and 4 CanUnloadNow
//! rows. ASan and UBSan flagged none of them, so none is excluded.

use super::*;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

type Row = (
    Kind,
    u32,
    Option<u32>,
    Hresult,
    Option<u32>,
    Hresult,
    Option<u32>,
);

#[derive(Clone, Copy)]
enum Kind {
    ShellProxy,
    ShellStub,
    ObjectsProxy,
    ObjectsStub,
}

/// The scripted callee's return value, its optional `*pceltFetched` write, and the `celt` it saw.
static CB_RET: AtomicI32 = AtomicI32::new(0);
static CB_WRITE_ON: AtomicBool = AtomicBool::new(false);
static CB_WRITE: AtomicU32 = AtomicU32::new(0);
static CB_CELT: AtomicU32 = AtomicU32::new(0);

fn scripted(celt: u32, celt_fetched: &mut u32) -> Hresult {
    CB_CELT.store(celt, Ordering::SeqCst);
    if CB_WRITE_ON.load(Ordering::SeqCst) {
        *celt_fetched = CB_WRITE.load(Ordering::SeqCst);
    }
    CB_RET.load(Ordering::SeqCst)
}

/// Runs one row. `pin` is the caller's `pceltFetched` (`None` for NULL, only valid for the proxies),
/// and the return value is the function's result and the caller's final `*pceltFetched`.
fn run(
    kind: Kind,
    celt: u32,
    pin: Option<u32>,
    cb_ret: Hresult,
    cb_write: Option<u32>,
) -> (Hresult, Option<u32>) {
    CB_RET.store(cb_ret, Ordering::SeqCst);
    CB_WRITE_ON.store(cb_write.is_some(), Ordering::SeqCst);
    CB_WRITE.store(cb_write.unwrap_or(0), Ordering::SeqCst);
    let mut slot = pin.unwrap_or(0);
    let ret = match kind {
        Kind::ShellProxy | Kind::ObjectsProxy => enum_next_proxy(
            celt,
            if pin.is_some() { Some(&mut slot) } else { None },
            scripted,
        ),
        Kind::ShellStub => enum_shell_items_next_stub(celt, &mut slot, scripted),
        Kind::ObjectsStub => enum_objects_next_stub(celt, &mut slot, scripted),
    };
    assert_eq!(CB_CELT.load(Ordering::SeqCst), celt);
    (ret, if pin.is_some() { Some(slot) } else { None })
}

/// Rows in the form `(kind, celt, pceltFetched in, callee return, callee write, expected return,
/// expected *pceltFetched)`. `None` for `pceltFetched` means the C caller passed NULL.
const NEXT_ROWS: [Row; 16] = [
    (Kind::ShellProxy, 3, None, 0, Some(5), 0, None),
    (Kind::ShellProxy, 3, Some(9), 0, Some(5), 0, Some(5)),
    (Kind::ShellProxy, 0, Some(9), 1, None, 1, Some(9)),
    (
        Kind::ShellProxy,
        2,
        Some(9),
        -2147467259,
        Some(4),
        -2147467259,
        Some(4),
    ),
    (
        Kind::ShellProxy,
        2,
        None,
        -2147467259,
        None,
        -2147467259,
        None,
    ),
    (Kind::ShellStub, 3, Some(9), 0, None, 0, Some(3)),
    (Kind::ShellStub, 3, Some(9), 1, Some(2), 1, Some(2)),
    (Kind::ShellStub, 3, Some(9), 1, None, 1, Some(0)),
    (
        Kind::ShellStub,
        7,
        Some(9),
        -2147467259,
        Some(4),
        -2147467259,
        Some(4),
    ),
    (Kind::ShellStub, 0, Some(9), 0, None, 0, Some(0)),
    (Kind::ObjectsProxy, 2, None, 0, Some(1), 0, None),
    (Kind::ObjectsProxy, 2, Some(9), 1, None, 1, Some(9)),
    (Kind::ObjectsProxy, 2, Some(9), 0, Some(6), 0, Some(6)),
    (Kind::ObjectsStub, 2, Some(9), 0, None, 0, Some(0)),
    (Kind::ObjectsStub, 2, Some(9), 0, Some(7), 0, Some(7)),
    (
        Kind::ObjectsStub,
        2,
        Some(9),
        -2147467259,
        None,
        -2147467259,
        Some(0),
    ),
];

/// Rows in the form `(RefCount, expected return)`.
const UNLOAD_ROWS: [(i32, Hresult); 4] = [(0, 0), (1, 1), (-1, 1), (2147483647, 1)];

#[test]
fn oracle_next_rows() {
    for (kind, celt, pin, cb_ret, cb_write, ret, out) in NEXT_ROWS {
        assert_eq!(run(kind, celt, pin, cb_ret, cb_write), (ret, out));
    }
}

#[test]
fn oracle_dll_can_unload_now_rows() {
    for (ref_count, ret) in UNLOAD_ROWS {
        assert_eq!(dll_can_unload_now(ref_count), ret);
    }
}
