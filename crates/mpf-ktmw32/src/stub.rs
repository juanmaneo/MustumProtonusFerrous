/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = ktmw32.dll` in `dlls/ktmw32/Makefile.in`.
const KTMW32_DLL_NAME: &str = "ktmw32.dll";

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

/// `CommitComplete`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn commit_complete() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "CommitComplete")
}

/// `CommitEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn commit_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "CommitEnlistment")
}

/// `CommitTransactionAsync`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn commit_transaction_async() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "CommitTransactionAsync")
}

/// `CreateEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn create_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "CreateEnlistment")
}

/// `CreateResourceManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn create_resource_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "CreateResourceManager")
}

/// `CreateTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn create_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "CreateTransactionManager")
}

/// `GetCurrentClockTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_current_clock_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetCurrentClockTransactionManager")
}

/// `GetEnlistmentId`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_enlistment_id() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetEnlistmentId")
}

/// `GetEnlistmentRecoveryInformation`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_enlistment_recovery_information() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetEnlistmentRecoveryInformation")
}

/// `GetNotificationResourceManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_notification_resource_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetNotificationResourceManager")
}

/// `GetNotificationResourceManagerAsync`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_notification_resource_manager_async() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetNotificationResourceManagerAsync")
}

/// `GetTransactionId`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_transaction_id() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetTransactionId")
}

/// `GetTransactionInformation`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_transaction_information() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetTransactionInformation")
}

/// `GetTransactionManagerId`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn get_transaction_manager_id() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "GetTransactionManagerId")
}

/// `OpenEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn open_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "OpenEnlistment")
}

/// `OpenResourceManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn open_resource_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "OpenResourceManager")
}

/// `OpenTransaction`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn open_transaction() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "OpenTransaction")
}

/// `OpenTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn open_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "OpenTransactionManager")
}

/// `OpenTransactionManagerById`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn open_transaction_manager_by_id() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "OpenTransactionManagerById")
}

/// `PrePrepareComplete`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn pre_prepare_complete() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrePrepareComplete")
}

/// `PrePrepareEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn pre_prepare_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrePrepareEnlistment")
}

/// `PrepareComplete`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn prepare_complete() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrepareComplete")
}

/// `PrepareEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn prepare_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrepareEnlistment")
}

/// `PrivCreateTransaction`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn priv_create_transaction() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrivCreateTransaction")
}

/// `PrivIsLogWritableTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn priv_is_log_writable_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrivIsLogWritableTransactionManager")
}

/// `PrivPropagationComplete`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn priv_propagation_complete() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrivPropagationComplete")
}

/// `PrivPropagationFailed`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn priv_propagation_failed() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrivPropagationFailed")
}

/// `PrivRegisterProtocolAddressInformation`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn priv_register_protocol_address_information() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "PrivRegisterProtocolAddressInformation")
}

/// `ReadOnlyEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn read_only_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "ReadOnlyEnlistment")
}

/// `RecoverEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn recover_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RecoverEnlistment")
}

/// `RecoverResourceManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn recover_resource_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RecoverResourceManager")
}

/// `RecoverTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn recover_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RecoverTransactionManager")
}

/// `RenameTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn rename_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RenameTransactionManager")
}

/// `RollbackComplete`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn rollback_complete() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RollbackComplete")
}

/// `RollbackEnlistment`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn rollback_enlistment() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RollbackEnlistment")
}

/// `RollbackTransactionAsync`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn rollback_transaction_async() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RollbackTransactionAsync")
}

/// `RollforwardTransactionManager`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn rollforward_transaction_manager() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "RollforwardTransactionManager")
}

/// `SetEnlistmentRecoveryInformation`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn set_enlistment_recovery_information() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "SetEnlistmentRecoveryInformation")
}

/// `SetResourceManagerCompletionPort`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn set_resource_manager_completion_port() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "SetResourceManagerCompletionPort")
}

/// `SetTransactionInformation`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn set_transaction_information() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "SetTransactionInformation")
}

/// `SinglePhaseReject`, exported as `@ stub` in `dlls/ktmw32/ktmw32.spec`.
pub fn single_phase_reject() -> StubException {
    wine_spec_unimplemented_stub(KTMW32_DLL_NAME, "SinglePhaseReject")
}
