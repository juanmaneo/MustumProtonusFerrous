/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = comm.drv16` in `dlls/comm.drv16/Makefile.in`.
const COMM_DRV16_DLL_NAME: &str = "comm.drv16";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

fn comm_drv16_stub(function: &'static str) -> StubException {
    wine_spec_unimplemented_stub(COMM_DRV16_DLL_NAME, function)
}

/// `INICOM`, exported as `@ stub` (ordinal 1).
pub fn inicom() -> StubException {
    comm_drv16_stub("INICOM")
}

/// `SETCOM`, exported as `@ stub` (ordinal 2).
pub fn setcom() -> StubException {
    comm_drv16_stub("SETCOM")
}

/// `SETQUE`, exported as `@ stub` (ordinal 3).
pub fn setque() -> StubException {
    comm_drv16_stub("SETQUE")
}

/// `RECCOM`, exported as `@ stub` (ordinal 4).
pub fn reccom() -> StubException {
    comm_drv16_stub("RECCOM")
}

/// `SNDCOM`, exported as `@ stub` (ordinal 5).
pub fn sndcom() -> StubException {
    comm_drv16_stub("SNDCOM")
}

/// `CTX`, exported as `@ stub` (ordinal 6).
pub fn ctx() -> StubException {
    comm_drv16_stub("CTX")
}

/// `TRMCOM`, exported as `@ stub` (ordinal 7).
pub fn trmcom() -> StubException {
    comm_drv16_stub("TRMCOM")
}

/// `STACOM`, exported as `@ stub` (ordinal 8).
pub fn stacom() -> StubException {
    comm_drv16_stub("STACOM")
}

/// `CEXTFCN`, exported as `@ stub` (ordinal 9).
pub fn cextfcn() -> StubException {
    comm_drv16_stub("CEXTFCN")
}

/// `CFLUSH`, exported as `@ stub` (ordinal 10).
pub fn cflush() -> StubException {
    comm_drv16_stub("CFLUSH")
}

/// `CEVT`, exported as `@ stub` (ordinal 11).
pub fn cevt() -> StubException {
    comm_drv16_stub("CEVT")
}

/// `CEVTGET`, exported as `@ stub` (ordinal 12).
pub fn cevtget() -> StubException {
    comm_drv16_stub("CEVTGET")
}

/// `CSETBRK`, exported as `@ stub` (ordinal 13).
pub fn csetbrk() -> StubException {
    comm_drv16_stub("CSETBRK")
}

/// `CCLRBRK`, exported as `@ stub` (ordinal 14).
pub fn cclrbrk() -> StubException {
    comm_drv16_stub("CCLRBRK")
}

/// `GETDCB`, exported as `@ stub` (ordinal 15).
pub fn getdcb() -> StubException {
    comm_drv16_stub("GETDCB")
}

/// `SUSPENDOPENCOMMPORTS`, exported as `@ stub` (ordinal 17).
pub fn suspend_open_comm_ports() -> StubException {
    comm_drv16_stub("SUSPENDOPENCOMMPORTS")
}

/// `REACTIVATEOPENCOMMPORTS`, exported as `@ stub` (ordinal 18).
pub fn reactivate_open_comm_ports() -> StubException {
    comm_drv16_stub("REACTIVATEOPENCOMMPORTS")
}

/// `COMMWRITESTRING`, exported as `@ stub` (ordinal 19).
pub fn comm_write_string() -> StubException {
    comm_drv16_stub("COMMWRITESTRING")
}

/// `READCOMMSTRING`, exported as `@ stub` (ordinal 20).
pub fn read_comm_string() -> StubException {
    comm_drv16_stub("READCOMMSTRING")
}

/// `ENABLENOTIFICATION`, exported as `@ stub` (ordinal 100).
pub fn enable_notification() -> StubException {
    comm_drv16_stub("ENABLENOTIFICATION")
}

/// `ordinal_only__win31`, exported as `@ stub` (ordinal 101). The spec gives it a name, so
/// winebuild passes the name rather than the ordinal.
pub fn ordinal_only_win31() -> StubException {
    comm_drv16_stub("ordinal_only__win31")
}
