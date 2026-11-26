//! Port of the portable logic in Wine's `dlls/ktmw32/ktmw32_main.c` and `dlls/ktmw32/ktmw32.spec`.
//!
//! Ported:
//! - `CommitTransaction`, `RollbackTransaction` and `CreateTransaction`, with their `set_ntstatus`
//!   and `get_nt_timeout` helpers, and the `RtlInitUnicodeString` logic that `CreateTransaction`
//!   relies on: [`commit_transaction`], [`rollback_transaction`], [`create_transaction`],
//!   [`set_ntstatus`], [`get_nt_timeout`], [`init_unicode_string`]
//! - The 41 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`commit_complete`]
//!
//! Skipped:
//! - The NT calls (`NtCommitTransaction`, `NtRollbackTransaction`, `NtCreateTransaction`), the
//!   `RtlNtStatusToDosError` table and `RtlSetLastWin32Error`. The caller supplies them through the
//!   [`Nt`] trait.
//! - The `WINAPI` and `__cdecl` calling conventions, and the `OBJECT_ATTRIBUTES` `Length` and
//!   `SecurityQualityOfService` fields, which `InitializeObjectAttributes` fixes.
//! - The `DesiredAccess` (always 0, `FIXME` in the C source) and the NULL `TmHandle` of
//!   `NtCreateTransaction`, which are fixed in [`NtCreateTransactionArgs`].

mod nt;
mod stub;
mod timeout;
mod transaction;
mod unicode;

pub use nt::{set_ntstatus, Handle, Nt, NtStatus, INVALID_HANDLE_VALUE};
pub use stub::{
    commit_complete, commit_enlistment, commit_transaction_async, create_enlistment,
    create_resource_manager, create_transaction_manager, get_current_clock_transaction_manager,
    get_enlistment_id, get_enlistment_recovery_information, get_notification_resource_manager,
    get_notification_resource_manager_async, get_transaction_id, get_transaction_information,
    get_transaction_manager_id, open_enlistment, open_resource_manager, open_transaction,
    open_transaction_manager, open_transaction_manager_by_id, pre_prepare_complete,
    pre_prepare_enlistment, prepare_complete, prepare_enlistment, priv_create_transaction,
    priv_is_log_writable_transaction_manager, priv_propagation_complete, priv_propagation_failed,
    priv_register_protocol_address_information, read_only_enlistment, recover_enlistment,
    recover_resource_manager, recover_transaction_manager, rename_transaction_manager,
    rollback_complete, rollback_enlistment, rollback_transaction_async,
    rollforward_transaction_manager, set_enlistment_recovery_information,
    set_resource_manager_completion_port, set_transaction_information, single_phase_reject,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use timeout::{get_nt_timeout, INFINITE};
pub use transaction::{
    commit_transaction, create_transaction, rollback_transaction, Guid, NtCreateTransactionArgs,
    ObjectAttributes, SecurityAttributes, OBJ_CASE_INSENSITIVE, OBJ_INHERIT,
};
pub use unicode::{init_unicode_string, UnicodeString};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
