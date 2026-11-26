use crate::transaction::NtCreateTransactionArgs;

/// `NTSTATUS`, which Wine declares as `LONG` in `include/winnt.h`.
pub type NtStatus = i32;

/// `HANDLE` as an opaque value.
pub type Handle = usize;

/// `INVALID_HANDLE_VALUE` from `include/winbase.h`, `(HANDLE)~(ULONG_PTR)0`.
pub const INVALID_HANDLE_VALUE: Handle = usize::MAX;

/// The OS calls that `ktmw32_main.c` makes. The caller supplies them, so the port holds no OS state.
pub trait Nt {
    /// `RtlNtStatusToDosError`.
    fn nt_status_to_dos_error(&mut self, status: NtStatus) -> u32;

    /// `RtlSetLastWin32Error`.
    fn set_last_error(&mut self, error: u32);

    /// `NtCommitTransaction`, called with `Wait` = `TRUE`.
    fn nt_commit_transaction(&mut self, transaction: Handle, wait: bool) -> NtStatus;

    /// `NtRollbackTransaction`, called with `Wait` = `TRUE`.
    fn nt_rollback_transaction(&mut self, transaction: Handle, wait: bool) -> NtStatus;

    /// `NtCreateTransaction`. Returns the status and the handle that the call writes.
    fn nt_create_transaction(&mut self, args: &NtCreateTransactionArgs<'_>) -> (NtStatus, Handle);
}

/// `set_ntstatus` from `dlls/ktmw32/ktmw32_main.c`. A nonzero status sets the last error to its
/// DOS error and returns `FALSE`. Zero returns `TRUE`.
pub fn set_ntstatus(nt: &mut impl Nt, status: NtStatus) -> bool {
    if status != 0 {
        let error = nt.nt_status_to_dos_error(status);
        nt.set_last_error(error);
    }
    status == 0
}
