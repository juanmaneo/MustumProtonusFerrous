/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = pidgen.dll` in `dlls/pidgen/Makefile.in`.
const PIDGEN_DLL_NAME: &str = "pidgen.dll";

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

/// `PIDGenA`, an `@ stub` export in `dlls/pidgen/pidgen.spec`.
pub fn pid_gen_a() -> StubException {
    wine_spec_unimplemented_stub(PIDGEN_DLL_NAME, "PIDGenA")
}

/// `PIDGenW`, an `@ stub` export in `dlls/pidgen/pidgen.spec`.
pub fn pid_gen_w() -> StubException {
    wine_spec_unimplemented_stub(PIDGEN_DLL_NAME, "PIDGenW")
}

/// `SetupPIDGenA`, an `@ stub` export in `dlls/pidgen/pidgen.spec`.
pub fn setup_pid_gen_a() -> StubException {
    wine_spec_unimplemented_stub(PIDGEN_DLL_NAME, "SetupPIDGenA")
}

/// `SetupPIDGenW`, an `@ stub` export in `dlls/pidgen/pidgen.spec`.
pub fn setup_pid_gen_w() -> StubException {
    wine_spec_unimplemented_stub(PIDGEN_DLL_NAME, "SetupPIDGenW")
}

/// `VerifyPIDSequenceW`, an `@ stub` export in `dlls/pidgen/pidgen.spec`.
pub fn verify_pid_sequence_w() -> StubException {
    wine_spec_unimplemented_stub(PIDGEN_DLL_NAME, "VerifyPIDSequenceW")
}
