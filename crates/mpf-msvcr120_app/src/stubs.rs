//! One helper per `@ stub` export of `dlls/msvcr120_app/msvcr120_app.spec`, 400 in all. Each returns the
//! record that `__wine_spec_unimplemented_stub` raises for that export, with the module and function
//! names winebuild passes (see [`crate::wine_spec_unimplemented_stub`]).
//!
//! A helper is named `stub_` followed by its export name in lower case. Each run of characters other
//! than `a-z` and `0-9` becomes one `_`, and leading or trailing `_` are dropped. That rule gives
//! `_NLG_Dispatch2` and `__NLG_Dispatch2` the same name, as it does `_NLG_Return2` and `__NLG_Return2`.
//! Those two pairs keep one `_` per character instead (`stub__nlg_dispatch2`, `stub___nlg_dispatch2`,
//! and likewise for `return2`), so every helper name is unique.

use crate::stub::{wine_spec_unimplemented_stub, StubException, MSVCR120_APP_DLL_NAME};

/// `??0_Context@details@Concurrency@@QAA@PAVContext@2@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_context_details_concurrency_qaa_pavcontext_2_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Context@details@Concurrency@@QAA@PAVContext@2@@Z",
    )
}

/// `??0_Context@details@Concurrency@@QAE@PAVContext@2@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_context_details_concurrency_qae_pavcontext_2_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Context@details@Concurrency@@QAE@PAVContext@2@@Z",
    )
}

/// `??0_Context@details@Concurrency@@QEAA@PEAVContext@2@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_context_details_concurrency_qeaa_peavcontext_2_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Context@details@Concurrency@@QEAA@PEAVContext@2@@Z",
    )
}

/// `??0_Interruption_exception@details@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_interruption_exception_details_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Interruption_exception@details@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0_Interruption_exception@details@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_interruption_exception_details_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Interruption_exception@details@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0_Interruption_exception@details@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_interruption_exception_details_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Interruption_exception@details@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0_Interruption_exception@details@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_interruption_exception_details_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Interruption_exception@details@Concurrency@@QAA@XZ",
    )
}

/// `??0_Interruption_exception@details@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_interruption_exception_details_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Interruption_exception@details@Concurrency@@QAE@XZ",
    )
}

/// `??0_Interruption_exception@details@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_interruption_exception_details_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_Interruption_exception@details@Concurrency@@QEAA@XZ",
    )
}

/// `??0_ReentrantLock@details@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_reentrantlock_details_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_ReentrantLock@details@Concurrency@@QAA@XZ",
    )
}

/// `??0_ReentrantLock@details@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_reentrantlock_details_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_ReentrantLock@details@Concurrency@@QAE@XZ",
    )
}

/// `??0_ReentrantLock@details@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_reentrantlock_details_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_ReentrantLock@details@Concurrency@@QEAA@XZ",
    )
}

/// `??0_SpinLock@details@Concurrency@@QAA@ACJ@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_spinlock_details_concurrency_qaa_acj_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_SpinLock@details@Concurrency@@QAA@ACJ@Z",
    )
}

/// `??0_SpinLock@details@Concurrency@@QAE@ACJ@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_spinlock_details_concurrency_qae_acj_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_SpinLock@details@Concurrency@@QAE@ACJ@Z",
    )
}

/// `??0_SpinLock@details@Concurrency@@QEAA@AECJ@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_spinlock_details_concurrency_qeaa_aecj_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_SpinLock@details@Concurrency@@QEAA@AECJ@Z",
    )
}

/// `??0_TaskCollection@details@Concurrency@@QAA@PAV_CancellationTokenState@12@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_taskcollection_details_concurrency_qaa_pav_cancellationtokenstate_12_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_TaskCollection@details@Concurrency@@QAA@PAV_CancellationTokenState@12@@Z",
    )
}

/// `??0_TaskCollection@details@Concurrency@@QAE@PAV_CancellationTokenState@12@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_taskcollection_details_concurrency_qae_pav_cancellationtokenstate_12_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_TaskCollection@details@Concurrency@@QAE@PAV_CancellationTokenState@12@@Z",
    )
}

/// `??0_TaskCollection@details@Concurrency@@QEAA@PEAV_CancellationTokenState@12@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_taskcollection_details_concurrency_qeaa_peav_cancellationtokenstate_12_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_TaskCollection@details@Concurrency@@QEAA@PEAV_CancellationTokenState@12@@Z",
    )
}

/// `??0_TaskCollection@details@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0_taskcollection_details_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_TaskCollection@details@Concurrency@@QAA@XZ",
    )
}

/// `??0_TaskCollection@details@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0_taskcollection_details_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_TaskCollection@details@Concurrency@@QAE@XZ",
    )
}

/// `??0_TaskCollection@details@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0_taskcollection_details_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0_TaskCollection@details@Concurrency@@QEAA@XZ",
    )
}

/// `??0bad_target@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0bad_target_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0bad_target@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0bad_target@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0bad_target_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0bad_target@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0bad_target@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0bad_target_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0bad_target@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0bad_target@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0bad_target_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??0bad_target@Concurrency@@QAA@XZ")
}

/// `??0bad_target@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0bad_target_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??0bad_target@Concurrency@@QAE@XZ")
}

/// `??0bad_target@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0bad_target_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??0bad_target@Concurrency@@QEAA@XZ")
}

/// `??0context_self_unblock@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0context_self_unblock_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_self_unblock@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0context_self_unblock@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0context_self_unblock_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_self_unblock@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0context_self_unblock@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0context_self_unblock_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_self_unblock@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0context_self_unblock@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0context_self_unblock_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_self_unblock@Concurrency@@QAA@XZ",
    )
}

/// `??0context_self_unblock@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0context_self_unblock_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_self_unblock@Concurrency@@QAE@XZ",
    )
}

/// `??0context_self_unblock@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0context_self_unblock_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_self_unblock@Concurrency@@QEAA@XZ",
    )
}

/// `??0context_unblock_unbalanced@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0context_unblock_unbalanced_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_unblock_unbalanced@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0context_unblock_unbalanced@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0context_unblock_unbalanced_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_unblock_unbalanced@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0context_unblock_unbalanced@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0context_unblock_unbalanced_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_unblock_unbalanced@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0context_unblock_unbalanced@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0context_unblock_unbalanced_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_unblock_unbalanced@Concurrency@@QAA@XZ",
    )
}

/// `??0context_unblock_unbalanced@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0context_unblock_unbalanced_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_unblock_unbalanced@Concurrency@@QAE@XZ",
    )
}

/// `??0context_unblock_unbalanced@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0context_unblock_unbalanced_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0context_unblock_unbalanced@Concurrency@@QEAA@XZ",
    )
}

/// `??0default_scheduler_exists@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0default_scheduler_exists_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0default_scheduler_exists@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0default_scheduler_exists@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0default_scheduler_exists_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0default_scheduler_exists@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0default_scheduler_exists@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0default_scheduler_exists_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0default_scheduler_exists@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0default_scheduler_exists@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0default_scheduler_exists_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0default_scheduler_exists@Concurrency@@QAA@XZ",
    )
}

/// `??0default_scheduler_exists@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0default_scheduler_exists_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0default_scheduler_exists@Concurrency@@QAE@XZ",
    )
}

/// `??0default_scheduler_exists@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0default_scheduler_exists_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0default_scheduler_exists@Concurrency@@QEAA@XZ",
    )
}

/// `??0improper_scheduler_reference@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0improper_scheduler_reference_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0improper_scheduler_reference@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0improper_scheduler_reference@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0improper_scheduler_reference_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0improper_scheduler_reference@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0improper_scheduler_reference@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0improper_scheduler_reference_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0improper_scheduler_reference@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0improper_scheduler_reference@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0improper_scheduler_reference_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0improper_scheduler_reference@Concurrency@@QAA@XZ",
    )
}

/// `??0improper_scheduler_reference@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0improper_scheduler_reference_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0improper_scheduler_reference@Concurrency@@QAE@XZ",
    )
}

/// `??0improper_scheduler_reference@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0improper_scheduler_reference_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0improper_scheduler_reference@Concurrency@@QEAA@XZ",
    )
}

/// `??0invalid_link_target@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0invalid_link_target_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_link_target@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0invalid_link_target@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0invalid_link_target_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_link_target@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0invalid_link_target@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0invalid_link_target_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_link_target@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0invalid_link_target@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0invalid_link_target_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_link_target@Concurrency@@QAA@XZ",
    )
}

/// `??0invalid_link_target@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0invalid_link_target_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_link_target@Concurrency@@QAE@XZ",
    )
}

/// `??0invalid_link_target@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0invalid_link_target_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_link_target@Concurrency@@QEAA@XZ",
    )
}

/// `??0invalid_operation@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0invalid_operation_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_operation@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0invalid_operation@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0invalid_operation_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_operation@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0invalid_operation@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0invalid_operation_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_operation@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0invalid_operation@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0invalid_operation_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_operation@Concurrency@@QAA@XZ",
    )
}

/// `??0invalid_operation@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0invalid_operation_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_operation@Concurrency@@QAE@XZ",
    )
}

/// `??0invalid_operation@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0invalid_operation_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_operation@Concurrency@@QEAA@XZ",
    )
}

/// `??0invalid_oversubscribe_operation@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0invalid_oversubscribe_operation_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_oversubscribe_operation@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0invalid_oversubscribe_operation@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0invalid_oversubscribe_operation_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_oversubscribe_operation@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0invalid_oversubscribe_operation@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0invalid_oversubscribe_operation_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_oversubscribe_operation@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0invalid_oversubscribe_operation@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0invalid_oversubscribe_operation_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_oversubscribe_operation@Concurrency@@QAA@XZ",
    )
}

/// `??0invalid_oversubscribe_operation@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0invalid_oversubscribe_operation_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_oversubscribe_operation@Concurrency@@QAE@XZ",
    )
}

/// `??0invalid_oversubscribe_operation@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0invalid_oversubscribe_operation_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0invalid_oversubscribe_operation@Concurrency@@QEAA@XZ",
    )
}

/// `??0message_not_found@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0message_not_found_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0message_not_found@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0message_not_found@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0message_not_found_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0message_not_found@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0message_not_found@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0message_not_found_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0message_not_found@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0message_not_found@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0message_not_found_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0message_not_found@Concurrency@@QAA@XZ",
    )
}

/// `??0message_not_found@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0message_not_found_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0message_not_found@Concurrency@@QAE@XZ",
    )
}

/// `??0message_not_found@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0message_not_found_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0message_not_found@Concurrency@@QEAA@XZ",
    )
}

/// `??0nested_scheduler_missing_detach@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0nested_scheduler_missing_detach_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0nested_scheduler_missing_detach@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0nested_scheduler_missing_detach@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0nested_scheduler_missing_detach_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0nested_scheduler_missing_detach@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0nested_scheduler_missing_detach@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0nested_scheduler_missing_detach_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0nested_scheduler_missing_detach@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0nested_scheduler_missing_detach@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0nested_scheduler_missing_detach_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0nested_scheduler_missing_detach@Concurrency@@QAA@XZ",
    )
}

/// `??0nested_scheduler_missing_detach@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0nested_scheduler_missing_detach_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0nested_scheduler_missing_detach@Concurrency@@QAE@XZ",
    )
}

/// `??0nested_scheduler_missing_detach@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0nested_scheduler_missing_detach_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0nested_scheduler_missing_detach@Concurrency@@QEAA@XZ",
    )
}

/// `??0operation_timed_out@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0operation_timed_out_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0operation_timed_out@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0operation_timed_out@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0operation_timed_out_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0operation_timed_out@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0operation_timed_out@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0operation_timed_out_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0operation_timed_out@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0operation_timed_out@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0operation_timed_out_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0operation_timed_out@Concurrency@@QAA@XZ",
    )
}

/// `??0operation_timed_out@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0operation_timed_out_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0operation_timed_out@Concurrency@@QAE@XZ",
    )
}

/// `??0operation_timed_out@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0operation_timed_out_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0operation_timed_out@Concurrency@@QEAA@XZ",
    )
}

/// `??0scheduler_not_attached@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0scheduler_not_attached_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_not_attached@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0scheduler_not_attached@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0scheduler_not_attached_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_not_attached@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0scheduler_not_attached@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0scheduler_not_attached_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_not_attached@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0scheduler_not_attached@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0scheduler_not_attached_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_not_attached@Concurrency@@QAA@XZ",
    )
}

/// `??0scheduler_not_attached@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0scheduler_not_attached_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_not_attached@Concurrency@@QAE@XZ",
    )
}

/// `??0scheduler_not_attached@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0scheduler_not_attached_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_not_attached@Concurrency@@QEAA@XZ",
    )
}

/// `??0scheduler_worker_creation_error@Concurrency@@QAA@J@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0scheduler_worker_creation_error_concurrency_qaa_j_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_worker_creation_error@Concurrency@@QAA@J@Z",
    )
}

/// `??0scheduler_worker_creation_error@Concurrency@@QAE@J@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0scheduler_worker_creation_error_concurrency_qae_j_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_worker_creation_error@Concurrency@@QAE@J@Z",
    )
}

/// `??0scheduler_worker_creation_error@Concurrency@@QEAA@J@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0scheduler_worker_creation_error_concurrency_qeaa_j_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_worker_creation_error@Concurrency@@QEAA@J@Z",
    )
}

/// `??0scheduler_worker_creation_error@Concurrency@@QAA@PBDJ@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0scheduler_worker_creation_error_concurrency_qaa_pbdj_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_worker_creation_error@Concurrency@@QAA@PBDJ@Z",
    )
}

/// `??0scheduler_worker_creation_error@Concurrency@@QAE@PBDJ@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0scheduler_worker_creation_error_concurrency_qae_pbdj_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_worker_creation_error@Concurrency@@QAE@PBDJ@Z",
    )
}

/// `??0scheduler_worker_creation_error@Concurrency@@QEAA@PEBDJ@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0scheduler_worker_creation_error_concurrency_qeaa_pebdj_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0scheduler_worker_creation_error@Concurrency@@QEAA@PEBDJ@Z",
    )
}

/// `??0task_canceled@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0task_canceled_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0task_canceled@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0task_canceled@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0task_canceled_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0task_canceled@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0task_canceled@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0task_canceled_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0task_canceled@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0task_canceled@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0task_canceled_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0task_canceled@Concurrency@@QAA@XZ",
    )
}

/// `??0task_canceled@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0task_canceled_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0task_canceled@Concurrency@@QAE@XZ",
    )
}

/// `??0task_canceled@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0task_canceled_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0task_canceled@Concurrency@@QEAA@XZ",
    )
}

/// `??0unsupported_os@Concurrency@@QAA@PBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_0unsupported_os_concurrency_qaa_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0unsupported_os@Concurrency@@QAA@PBD@Z",
    )
}

/// `??0unsupported_os@Concurrency@@QAE@PBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_0unsupported_os_concurrency_qae_pbd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0unsupported_os@Concurrency@@QAE@PBD@Z",
    )
}

/// `??0unsupported_os@Concurrency@@QEAA@PEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_0unsupported_os_concurrency_qeaa_pebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0unsupported_os@Concurrency@@QEAA@PEBD@Z",
    )
}

/// `??0unsupported_os@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_0unsupported_os_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0unsupported_os@Concurrency@@QAA@XZ",
    )
}

/// `??0unsupported_os@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_0unsupported_os_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0unsupported_os@Concurrency@@QAE@XZ",
    )
}

/// `??0unsupported_os@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_0unsupported_os_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??0unsupported_os@Concurrency@@QEAA@XZ",
    )
}

/// `??1_SpinLock@details@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_1_spinlock_details_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??1_SpinLock@details@Concurrency@@QAA@XZ",
    )
}

/// `??1_SpinLock@details@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_1_spinlock_details_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??1_SpinLock@details@Concurrency@@QAE@XZ",
    )
}

/// `??1_SpinLock@details@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_1_spinlock_details_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??1_SpinLock@details@Concurrency@@QEAA@XZ",
    )
}

/// `??1_TaskCollection@details@Concurrency@@QAA@XZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_1_taskcollection_details_concurrency_qaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??1_TaskCollection@details@Concurrency@@QAA@XZ",
    )
}

/// `??1_TaskCollection@details@Concurrency@@QAE@XZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_1_taskcollection_details_concurrency_qae_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??1_TaskCollection@details@Concurrency@@QAE@XZ",
    )
}

/// `??1_TaskCollection@details@Concurrency@@QEAA@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_1_taskcollection_details_concurrency_qeaa_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??1_TaskCollection@details@Concurrency@@QEAA@XZ",
    )
}

/// `??3@YAXPAXHPBDH@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_3_yaxpaxhpbdh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??3@YAXPAXHPBDH@Z")
}

/// `??3@YAXPEAXHPEBDH@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_3_yaxpeaxhpebdh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??3@YAXPEAXHPEBDH@Z")
}

/// `??4?$_SpinWait@$00@details@Concurrency@@QAAAAV012@ABV012@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_4_spinwait_00_details_concurrency_qaaaav012_abv012_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??4?$_SpinWait@$00@details@Concurrency@@QAAAAV012@ABV012@@Z",
    )
}

/// `??4?$_SpinWait@$00@details@Concurrency@@QAEAAV012@ABV012@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_4_spinwait_00_details_concurrency_qaeaav012_abv012_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??4?$_SpinWait@$00@details@Concurrency@@QAEAAV012@ABV012@@Z",
    )
}

/// `??4?$_SpinWait@$00@details@Concurrency@@QEAAAEAV012@AEBV012@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_4_spinwait_00_details_concurrency_qeaaaeav012_aebv012_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??4?$_SpinWait@$00@details@Concurrency@@QEAAAEAV012@AEBV012@@Z",
    )
}

/// `??4?$_SpinWait@$0A@@details@Concurrency@@QAAAAV012@ABV012@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_4_spinwait_0a_details_concurrency_qaaaav012_abv012_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??4?$_SpinWait@$0A@@details@Concurrency@@QAAAAV012@ABV012@@Z",
    )
}

/// `??4?$_SpinWait@$0A@@details@Concurrency@@QAEAAV012@ABV012@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_4_spinwait_0a_details_concurrency_qaeaav012_abv012_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??4?$_SpinWait@$0A@@details@Concurrency@@QAEAAV012@ABV012@@Z",
    )
}

/// `??4?$_SpinWait@$0A@@details@Concurrency@@QEAAAEAV012@AEBV012@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_4_spinwait_0a_details_concurrency_qeaaaeav012_aebv012_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??4?$_SpinWait@$0A@@details@Concurrency@@QEAAAEAV012@AEBV012@@Z",
    )
}

/// `??_F_Context@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_f_context_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??_F_Context@details@Concurrency@@QAAXXZ",
    )
}

/// `??_F_Context@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_f_context_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??_F_Context@details@Concurrency@@QAEXXZ",
    )
}

/// `??_F_Context@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_f_context_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "??_F_Context@details@Concurrency@@QEAAXXZ",
    )
}

/// `??_V@YAXPAXHPBDH@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_v_yaxpaxhpbdh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??_V@YAXPAXHPBDH@Z")
}

/// `??_V@YAXPEAXHPEBDH@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_v_yaxpeaxhpebdh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "??_V@YAXPEAXHPEBDH@Z")
}

/// `?CaptureCallstack@platform@details@Concurrency@@YAIPAPAXII@Z`, an `@ stub` export.
pub fn stub_capturecallstack_platform_details_concurrency_yaipapaxii_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?CaptureCallstack@platform@details@Concurrency@@YAIPAPAXII@Z",
    )
}

/// `?CreateResourceManager@Concurrency@@YAPAUIResourceManager@1@XZ`, an `@ stub` export for `-arch=win32`.
pub fn stub_createresourcemanager_concurrency_yapauiresourcemanager_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?CreateResourceManager@Concurrency@@YAPAUIResourceManager@1@XZ",
    )
}

/// `?CreateResourceManager@Concurrency@@YAPEAUIResourceManager@1@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_createresourcemanager_concurrency_yapeauiresourcemanager_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?CreateResourceManager@Concurrency@@YAPEAUIResourceManager@1@XZ",
    )
}

/// `?DisableTracing@Concurrency@@YAJXZ`, an `@ stub` export.
pub fn stub_disabletracing_concurrency_yajxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?DisableTracing@Concurrency@@YAJXZ")
}

/// `?EnableTracing@Concurrency@@YAJXZ`, an `@ stub` export.
pub fn stub_enabletracing_concurrency_yajxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?EnableTracing@Concurrency@@YAJXZ")
}

/// `?GetExecutionContextId@Concurrency@@YAIXZ`, an `@ stub` export.
pub fn stub_getexecutioncontextid_concurrency_yaixz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?GetExecutionContextId@Concurrency@@YAIXZ",
    )
}

/// `?GetOSVersion@Concurrency@@YA?AW4OSVersion@IResourceManager@1@XZ`, an `@ stub` export.
pub fn stub_getosversion_concurrency_ya_aw4osversion_iresourcemanager_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?GetOSVersion@Concurrency@@YA?AW4OSVersion@IResourceManager@1@XZ",
    )
}

/// `?GetProcessorCount@Concurrency@@YAIXZ`, an `@ stub` export.
pub fn stub_getprocessorcount_concurrency_yaixz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?GetProcessorCount@Concurrency@@YAIXZ",
    )
}

/// `?GetProcessorNodeCount@Concurrency@@YAIXZ`, an `@ stub` export.
pub fn stub_getprocessornodecount_concurrency_yaixz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?GetProcessorNodeCount@Concurrency@@YAIXZ",
    )
}

/// `?GetSchedulerId@Concurrency@@YAIXZ`, an `@ stub` export.
pub fn stub_getschedulerid_concurrency_yaixz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?GetSchedulerId@Concurrency@@YAIXZ")
}

/// `?GetSharedTimerQueue@details@Concurrency@@YAPAXXZ`, an `@ stub` export for `-arch=win32`.
pub fn stub_getsharedtimerqueue_details_concurrency_yapaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?GetSharedTimerQueue@details@Concurrency@@YAPAXXZ",
    )
}

/// `?GetSharedTimerQueue@details@Concurrency@@YAPEAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_getsharedtimerqueue_details_concurrency_yapeaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?GetSharedTimerQueue@details@Concurrency@@YAPEAXXZ",
    )
}

/// `?Log2@details@Concurrency@@YAKI@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_log2_details_concurrency_yaki_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?Log2@details@Concurrency@@YAKI@Z")
}

/// `?Log2@details@Concurrency@@YAK_K@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_log2_details_concurrency_yak_k_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?Log2@details@Concurrency@@YAK_K@Z")
}

/// `?_Abort@_StructuredTaskCollection@details@Concurrency@@AAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_abort_structuredtaskcollection_details_concurrency_aaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Abort@_StructuredTaskCollection@details@Concurrency@@AAAXXZ",
    )
}

/// `?_Abort@_StructuredTaskCollection@details@Concurrency@@AAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_abort_structuredtaskcollection_details_concurrency_aaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Abort@_StructuredTaskCollection@details@Concurrency@@AAEXXZ",
    )
}

/// `?_Abort@_StructuredTaskCollection@details@Concurrency@@AEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_abort_structuredtaskcollection_details_concurrency_aeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Abort@_StructuredTaskCollection@details@Concurrency@@AEAAXXZ",
    )
}

/// `?_Acquire@_ReentrantLock@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_acquire_reentrantlock_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Acquire@_ReentrantLock@details@Concurrency@@QAAXXZ",
    )
}

/// `?_Acquire@_ReentrantLock@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_acquire_reentrantlock_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Acquire@_ReentrantLock@details@Concurrency@@QAEXXZ",
    )
}

/// `?_Acquire@_ReentrantLock@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_acquire_reentrantlock_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Acquire@_ReentrantLock@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_acquireread_readerwriterlock_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QAAXXZ",
    )
}

/// `?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_acquireread_readerwriterlock_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QAEXXZ",
    )
}

/// `?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_acquireread_readerwriterlock_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_AcquireRead@_ReaderWriterLock@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_acquirewrite_readerwriterlock_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QAAXXZ",
    )
}

/// `?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_acquirewrite_readerwriterlock_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QAEXXZ",
    )
}

/// `?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_acquirewrite_readerwriterlock_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_AcquireWrite@_ReaderWriterLock@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_Cancel@_TaskCollection@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_cancel_taskcollection_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Cancel@_TaskCollection@details@Concurrency@@QAAXXZ",
    )
}

/// `?_Cancel@_TaskCollection@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_cancel_taskcollection_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Cancel@_TaskCollection@details@Concurrency@@QAEXXZ",
    )
}

/// `?_Cancel@_TaskCollection@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_cancel_taskcollection_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Cancel@_TaskCollection@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_CleanupToken@_StructuredTaskCollection@details@Concurrency@@AAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_cleanuptoken_structuredtaskcollection_details_concurrency_aaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_CleanupToken@_StructuredTaskCollection@details@Concurrency@@AAAXXZ",
    )
}

/// `?_CleanupToken@_StructuredTaskCollection@details@Concurrency@@AAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_cleanuptoken_structuredtaskcollection_details_concurrency_aaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_CleanupToken@_StructuredTaskCollection@details@Concurrency@@AAEXXZ",
    )
}

/// `?_CleanupToken@_StructuredTaskCollection@details@Concurrency@@AEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_cleanuptoken_structuredtaskcollection_details_concurrency_aeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_CleanupToken@_StructuredTaskCollection@details@Concurrency@@AEAAXXZ",
    )
}

/// `?_ConcRT_Assert@details@Concurrency@@YAXPBD0H@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_concrt_assert_details_concurrency_yaxpbd0h_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_Assert@details@Concurrency@@YAXPBD0H@Z",
    )
}

/// `?_ConcRT_Assert@details@Concurrency@@YAXPEBD0H@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_concrt_assert_details_concurrency_yaxpebd0h_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_Assert@details@Concurrency@@YAXPEBD0H@Z",
    )
}

/// `?_ConcRT_CoreAssert@details@Concurrency@@YAXPBD0H@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_concrt_coreassert_details_concurrency_yaxpbd0h_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_CoreAssert@details@Concurrency@@YAXPBD0H@Z",
    )
}

/// `?_ConcRT_CoreAssert@details@Concurrency@@YAXPEBD0H@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_concrt_coreassert_details_concurrency_yaxpebd0h_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_CoreAssert@details@Concurrency@@YAXPEBD0H@Z",
    )
}

/// `?_ConcRT_DumpMessage@details@Concurrency@@YAXPB_WZZ`, an `@ stub` export for `-arch=win32`.
pub fn stub_concrt_dumpmessage_details_concurrency_yaxpb_wzz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_DumpMessage@details@Concurrency@@YAXPB_WZZ",
    )
}

/// `?_ConcRT_DumpMessage@details@Concurrency@@YAXPEB_WZZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_concrt_dumpmessage_details_concurrency_yaxpeb_wzz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_DumpMessage@details@Concurrency@@YAXPEB_WZZ",
    )
}

/// `?_ConcRT_Trace@details@Concurrency@@YAXHPB_WZZ`, an `@ stub` export for `-arch=win32`.
pub fn stub_concrt_trace_details_concurrency_yaxhpb_wzz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_Trace@details@Concurrency@@YAXHPB_WZZ",
    )
}

/// `?_ConcRT_Trace@details@Concurrency@@YAXHPEB_WZZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_concrt_trace_details_concurrency_yaxhpeb_wzz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ConcRT_Trace@details@Concurrency@@YAXHPEB_WZZ",
    )
}

/// `?_Copy_str@exception@std@@AAAXPBD@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_copy_str_exception_std_aaaxpbd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Copy_str@exception@std@@AAAXPBD@Z")
}

/// `?_Copy_str@exception@std@@AAEXPBD@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_copy_str_exception_std_aaexpbd_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Copy_str@exception@std@@AAEXPBD@Z")
}

/// `?_Copy_str@exception@std@@AEAAXPEBD@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_copy_str_exception_std_aeaaxpebd_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Copy_str@exception@std@@AEAAXPEBD@Z",
    )
}

/// `?_Current_node@location@Concurrency@@SA?AV12@XZ`, an `@ stub` export.
pub fn stub_current_node_location_concurrency_sa_av12_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Current_node@location@Concurrency@@SA?AV12@XZ",
    )
}

/// `?_Destroy@_AsyncTaskCollection@details@Concurrency@@EAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_destroy_asynctaskcollection_details_concurrency_eaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Destroy@_AsyncTaskCollection@details@Concurrency@@EAAXXZ",
    )
}

/// `?_Destroy@_AsyncTaskCollection@details@Concurrency@@EAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_destroy_asynctaskcollection_details_concurrency_eaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Destroy@_AsyncTaskCollection@details@Concurrency@@EAEXXZ",
    )
}

/// `?_Destroy@_AsyncTaskCollection@details@Concurrency@@EEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_destroy_asynctaskcollection_details_concurrency_eeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Destroy@_AsyncTaskCollection@details@Concurrency@@EEAAXXZ",
    )
}

/// `?_Destroy@_CancellationTokenState@details@Concurrency@@EAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_destroy_cancellationtokenstate_details_concurrency_eaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Destroy@_CancellationTokenState@details@Concurrency@@EAAXXZ",
    )
}

/// `?_Destroy@_CancellationTokenState@details@Concurrency@@EAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_destroy_cancellationtokenstate_details_concurrency_eaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Destroy@_CancellationTokenState@details@Concurrency@@EAEXXZ",
    )
}

/// `?_Destroy@_CancellationTokenState@details@Concurrency@@EEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_destroy_cancellationtokenstate_details_concurrency_eeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Destroy@_CancellationTokenState@details@Concurrency@@EEAAXXZ",
    )
}

/// `?_GetConcRTTraceInfo@Concurrency@@YAPBU_CONCRT_TRACE_INFO@details@1@XZ`, an `@ stub` export for `-arch=win32`.
pub fn stub_getconcrttraceinfo_concurrency_yapbu_concrt_trace_info_details_1_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_GetConcRTTraceInfo@Concurrency@@YAPBU_CONCRT_TRACE_INFO@details@1@XZ",
    )
}

/// `?_GetConcRTTraceInfo@Concurrency@@YAPEBU_CONCRT_TRACE_INFO@details@1@XZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_getconcrttraceinfo_concurrency_yapebu_concrt_trace_info_details_1_xz() -> StubException
{
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_GetConcRTTraceInfo@Concurrency@@YAPEBU_CONCRT_TRACE_INFO@details@1@XZ",
    )
}

/// `?_GetCurrentInlineDepth@_StackGuard@details@Concurrency@@CAAAIXZ`, an `@ stub` export for `-arch=win32`.
pub fn stub_getcurrentinlinedepth_stackguard_details_concurrency_caaaixz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_GetCurrentInlineDepth@_StackGuard@details@Concurrency@@CAAAIXZ",
    )
}

/// `?_GetCurrentInlineDepth@_StackGuard@details@Concurrency@@CAAEA_KXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_getcurrentinlinedepth_stackguard_details_concurrency_caaea_kxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_GetCurrentInlineDepth@_StackGuard@details@Concurrency@@CAAEA_KXZ",
    )
}

/// `?_IsCanceling@_TaskCollection@details@Concurrency@@QAA_NXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_iscanceling_taskcollection_details_concurrency_qaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_IsCanceling@_TaskCollection@details@Concurrency@@QAA_NXZ",
    )
}

/// `?_IsCanceling@_TaskCollection@details@Concurrency@@QAE_NXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_iscanceling_taskcollection_details_concurrency_qae_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_IsCanceling@_TaskCollection@details@Concurrency@@QAE_NXZ",
    )
}

/// `?_IsCanceling@_TaskCollection@details@Concurrency@@QEAA_NXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_iscanceling_taskcollection_details_concurrency_qeaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_IsCanceling@_TaskCollection@details@Concurrency@@QEAA_NXZ",
    )
}

/// `?_Name_base@type_info@@CAPBDPBV1@PAU__type_info_node@@@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_name_base_type_info_capbdpbv1_pau_type_info_node_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Name_base@type_info@@CAPBDPBV1@PAU__type_info_node@@@Z",
    )
}

/// `?_Name_base@type_info@@CAPEBDPEBV1@PEAU__type_info_node@@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_name_base_type_info_capebdpebv1_peau_type_info_node_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Name_base@type_info@@CAPEBDPEBV1@PEAU__type_info_node@@@Z",
    )
}

/// `?_Name_base_internal@type_info@@CAPBDPBV1@PAU__type_info_node@@@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_name_base_internal_type_info_capbdpbv1_pau_type_info_node_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Name_base_internal@type_info@@CAPBDPBV1@PAU__type_info_node@@@Z",
    )
}

/// `?_Name_base_internal@type_info@@CAPEBDPEBV1@PEAU__type_info_node@@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_name_base_internal_type_info_capebdpebv1_peau_type_info_node_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Name_base_internal@type_info@@CAPEBDPEBV1@PEAU__type_info_node@@@Z",
    )
}

/// `?_NewCollection@_AsyncTaskCollection@details@Concurrency@@SAPAV123@PAV_CancellationTokenState@23@@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_newcollection_asynctaskcollection_details_concurrency_sapav123_pav_cancellationtokenstate_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_NewCollection@_AsyncTaskCollection@details@Concurrency@@SAPAV123@PAV_CancellationTokenState@23@@Z")
}

/// `?_NewCollection@_AsyncTaskCollection@details@Concurrency@@SAPEAV123@PEAV_CancellationTokenState@23@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_newcollection_asynctaskcollection_details_concurrency_sapeav123_peav_cancellationtokenstate_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_NewCollection@_AsyncTaskCollection@details@Concurrency@@SAPEAV123@PEAV_CancellationTokenState@23@@Z")
}

/// `?_Oversubscribe@_Context@details@Concurrency@@SAX_N@Z`, an `@ stub` export.
pub fn stub_oversubscribe_context_details_concurrency_sax_n_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Oversubscribe@_Context@details@Concurrency@@SAX_N@Z",
    )
}

/// `?_Release@_ReentrantLock@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_release_reentrantlock_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Release@_ReentrantLock@details@Concurrency@@QAAXXZ",
    )
}

/// `?_Release@_ReentrantLock@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_release_reentrantlock_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Release@_ReentrantLock@details@Concurrency@@QAEXXZ",
    )
}

/// `?_Release@_ReentrantLock@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_release_reentrantlock_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Release@_ReentrantLock@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_releaseread_readerwriterlock_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QAAXXZ",
    )
}

/// `?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_releaseread_readerwriterlock_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QAEXXZ",
    )
}

/// `?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_releaseread_readerwriterlock_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReleaseRead@_ReaderWriterLock@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_releasewrite_readerwriterlock_details_concurrency_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QAAXXZ",
    )
}

/// `?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_releasewrite_readerwriterlock_details_concurrency_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QAEXXZ",
    )
}

/// `?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_releasewrite_readerwriterlock_details_concurrency_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReleaseWrite@_ReaderWriterLock@details@Concurrency@@QEAAXXZ",
    )
}

/// `?_ReportUnobservedException@details@Concurrency@@YAXXZ`, an `@ stub` export.
pub fn stub_reportunobservedexception_details_concurrency_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_ReportUnobservedException@details@Concurrency@@YAXXZ",
    )
}

/// `?_RunAndWait@_TaskCollection@details@Concurrency@@QAA?AW4_TaskCollectionStatus@23@PAV_UnrealizedChore@23@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_runandwait_taskcollection_details_concurrency_qaa_aw4_taskcollectionstatus_23_pav_unrealizedchore_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_RunAndWait@_TaskCollection@details@Concurrency@@QAA?AW4_TaskCollectionStatus@23@PAV_UnrealizedChore@23@@Z")
}

/// `?_RunAndWait@_TaskCollection@details@Concurrency@@QAG?AW4_TaskCollectionStatus@23@PAV_UnrealizedChore@23@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_runandwait_taskcollection_details_concurrency_qag_aw4_taskcollectionstatus_23_pav_unrealizedchore_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_RunAndWait@_TaskCollection@details@Concurrency@@QAG?AW4_TaskCollectionStatus@23@PAV_UnrealizedChore@23@@Z")
}

/// `?_RunAndWait@_TaskCollection@details@Concurrency@@QEAA?AW4_TaskCollectionStatus@23@PEAV_UnrealizedChore@23@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_runandwait_taskcollection_details_concurrency_qeaa_aw4_taskcollectionstatus_23_peav_unrealizedchore_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_RunAndWait@_TaskCollection@details@Concurrency@@QEAA?AW4_TaskCollectionStatus@23@PEAV_UnrealizedChore@23@@Z")
}

/// `?_Schedule@_TaskCollection@details@Concurrency@@QAAXPAV_UnrealizedChore@23@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_schedule_taskcollection_details_concurrency_qaaxpav_unrealizedchore_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Schedule@_TaskCollection@details@Concurrency@@QAAXPAV_UnrealizedChore@23@@Z",
    )
}

/// `?_Schedule@_TaskCollection@details@Concurrency@@QAEXPAV_UnrealizedChore@23@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_schedule_taskcollection_details_concurrency_qaexpav_unrealizedchore_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Schedule@_TaskCollection@details@Concurrency@@QAEXPAV_UnrealizedChore@23@@Z",
    )
}

/// `?_Schedule@_TaskCollection@details@Concurrency@@QEAAXPEAV_UnrealizedChore@23@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_schedule_taskcollection_details_concurrency_qeaaxpeav_unrealizedchore_23_z(
) -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Schedule@_TaskCollection@details@Concurrency@@QEAAXPEAV_UnrealizedChore@23@@Z",
    )
}

/// `?_Schedule@_TaskCollection@details@Concurrency@@QAAXPAV_UnrealizedChore@23@PAVlocation@3@@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_schedule_taskcollection_details_concurrency_qaaxpav_unrealizedchore_23_pavlocation_3_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Schedule@_TaskCollection@details@Concurrency@@QAAXPAV_UnrealizedChore@23@PAVlocation@3@@Z")
}

/// `?_Schedule@_TaskCollection@details@Concurrency@@QAEXPAV_UnrealizedChore@23@PAVlocation@3@@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_schedule_taskcollection_details_concurrency_qaexpav_unrealizedchore_23_pavlocation_3_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Schedule@_TaskCollection@details@Concurrency@@QAEXPAV_UnrealizedChore@23@PAVlocation@3@@Z")
}

/// `?_Schedule@_TaskCollection@details@Concurrency@@QEAAXPEAV_UnrealizedChore@23@PEAVlocation@3@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_schedule_taskcollection_details_concurrency_qeaaxpeav_unrealizedchore_23_peavlocation_3_z(
) -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Schedule@_TaskCollection@details@Concurrency@@QEAAXPEAV_UnrealizedChore@23@PEAVlocation@3@@Z")
}

/// `?_SetUnobservedExceptionHandler@details@Concurrency@@YAXP6AXXZ@Z`, an `@ stub` export.
pub fn stub_setunobservedexceptionhandler_details_concurrency_yaxp6axxz_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_SetUnobservedExceptionHandler@details@Concurrency@@YAXP6AXXZ@Z",
    )
}

/// `?_Tidy@exception@std@@AAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_tidy_exception_std_aaaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Tidy@exception@std@@AAAXXZ")
}

/// `?_Tidy@exception@std@@AAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_tidy_exception_std_aaexxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Tidy@exception@std@@AAEXXZ")
}

/// `?_Tidy@exception@std@@AEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_tidy_exception_std_aeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_Tidy@exception@std@@AEAAXXZ")
}

/// `?_TryAcquire@_ReentrantLock@details@Concurrency@@QAA_NXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_tryacquire_reentrantlock_details_concurrency_qaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_TryAcquire@_ReentrantLock@details@Concurrency@@QAA_NXZ",
    )
}

/// `?_TryAcquire@_ReentrantLock@details@Concurrency@@QAE_NXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_tryacquire_reentrantlock_details_concurrency_qae_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_TryAcquire@_ReentrantLock@details@Concurrency@@QAE_NXZ",
    )
}

/// `?_TryAcquire@_ReentrantLock@details@Concurrency@@QEAA_NXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_tryacquire_reentrantlock_details_concurrency_qeaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_TryAcquire@_ReentrantLock@details@Concurrency@@QEAA_NXZ",
    )
}

/// `?_TryAcquireWrite@_ReaderWriterLock@details@Concurrency@@QAA_NXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_tryacquirewrite_readerwriterlock_details_concurrency_qaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_TryAcquireWrite@_ReaderWriterLock@details@Concurrency@@QAA_NXZ",
    )
}

/// `?_TryAcquireWrite@_ReaderWriterLock@details@Concurrency@@QAE_NXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_tryacquirewrite_readerwriterlock_details_concurrency_qae_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_TryAcquireWrite@_ReaderWriterLock@details@Concurrency@@QAE_NXZ",
    )
}

/// `?_TryAcquireWrite@_ReaderWriterLock@details@Concurrency@@QEAA_NXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_tryacquirewrite_readerwriterlock_details_concurrency_qeaa_nxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_TryAcquireWrite@_ReaderWriterLock@details@Concurrency@@QEAA_NXZ",
    )
}

/// `?_Type_info_dtor@type_info@@CAXPAV1@@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_type_info_dtor_type_info_caxpav1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Type_info_dtor@type_info@@CAXPAV1@@Z",
    )
}

/// `?_Type_info_dtor@type_info@@CAXPEAV1@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_type_info_dtor_type_info_caxpeav1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Type_info_dtor@type_info@@CAXPEAV1@@Z",
    )
}

/// `?_Type_info_dtor_internal@type_info@@CAXPAV1@@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_type_info_dtor_internal_type_info_caxpav1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Type_info_dtor_internal@type_info@@CAXPAV1@@Z",
    )
}

/// `?_Type_info_dtor_internal@type_info@@CAXPEAV1@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_type_info_dtor_internal_type_info_caxpeav1_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_Type_info_dtor_internal@type_info@@CAXPEAV1@@Z",
    )
}

/// `?_UnderlyingYield@details@Concurrency@@YAXXZ`, an `@ stub` export.
pub fn stub_underlyingyield_details_concurrency_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_UnderlyingYield@details@Concurrency@@YAXXZ",
    )
}

/// `?_ValidateExecute@@YAHP6AHXZ@Z`, an `@ stub` export for `-arch=arm`.
pub fn stub_validateexecute_yahp6ahxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateExecute@@YAHP6AHXZ@Z")
}

/// `?_ValidateExecute@@YAHP6GHXZ@Z`, an `@ stub` export for `-arch=i386`.
pub fn stub_validateexecute_yahp6ghxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateExecute@@YAHP6GHXZ@Z")
}

/// `?_ValidateExecute@@YAHP6A_JXZ@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_validateexecute_yahp6a_jxz_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateExecute@@YAHP6A_JXZ@Z")
}

/// `?_ValidateRead@@YAHPBXI@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_validateread_yahpbxi_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateRead@@YAHPBXI@Z")
}

/// `?_ValidateRead@@YAHPEBXI@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_validateread_yahpebxi_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateRead@@YAHPEBXI@Z")
}

/// `?_ValidateWrite@@YAHPAXI@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_validatewrite_yahpaxi_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateWrite@@YAHPAXI@Z")
}

/// `?_ValidateWrite@@YAHPEAXI@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_validatewrite_yahpeaxi_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_ValidateWrite@@YAHPEAXI@Z")
}

/// `?_inconsistency@@YAXXZ`, an `@ stub` export.
pub fn stub_inconsistency_yaxxz() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_inconsistency@@YAXXZ")
}

/// `?_set_new_handler@@YAP6AHI@ZH@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_set_new_handler_yap6ahi_zh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_set_new_handler@@YAP6AHI@ZH@Z")
}

/// `?_set_new_handler@@YAP6AH_K@ZH@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_set_new_handler_yap6ah_k_zh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?_set_new_handler@@YAP6AH_K@ZH@Z")
}

/// `?_set_se_translator@@YAP6AXIPAU_EXCEPTION_POINTERS@@@ZH@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_set_se_translator_yap6axipau_exception_pointers_zh_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_set_se_translator@@YAP6AXIPAU_EXCEPTION_POINTERS@@@ZH@Z",
    )
}

/// `?_set_se_translator@@YAP6AXIPEAU_EXCEPTION_POINTERS@@@ZH@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_set_se_translator_yap6axipeau_exception_pointers_zh_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_set_se_translator@@YAP6AXIPEAU_EXCEPTION_POINTERS@@@ZH@Z",
    )
}

/// `?_type_info_dtor_internal_method@type_info@@QAAXXZ`, an `@ stub` export for `-arch=arm`.
pub fn stub_type_info_dtor_internal_method_type_info_qaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_type_info_dtor_internal_method@type_info@@QAAXXZ",
    )
}

/// `?_type_info_dtor_internal_method@type_info@@QAEXXZ`, an `@ stub` export for `-arch=i386`.
pub fn stub_type_info_dtor_internal_method_type_info_qaexxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_type_info_dtor_internal_method@type_info@@QAEXXZ",
    )
}

/// `?_type_info_dtor_internal_method@type_info@@QEAAXXZ`, an `@ stub` export for `-arch=win64`.
pub fn stub_type_info_dtor_internal_method_type_info_qeaaxxz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?_type_info_dtor_internal_method@type_info@@QEAAXXZ",
    )
}

/// `?current@location@Concurrency@@SA?AV12@XZ`, an `@ stub` export.
pub fn stub_current_location_concurrency_sa_av12_xz() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?current@location@Concurrency@@SA?AV12@XZ",
    )
}

/// `?from_numa_node@location@Concurrency@@SA?AV12@G@Z`, an `@ stub` export.
pub fn stub_from_numa_node_location_concurrency_sa_av12_g_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?from_numa_node@location@Concurrency@@SA?AV12@G@Z",
    )
}

/// `?set_task_execution_resources@Concurrency@@YAXGPAU_GROUP_AFFINITY@@@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_set_task_execution_resources_concurrency_yaxgpau_group_affinity_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?set_task_execution_resources@Concurrency@@YAXGPAU_GROUP_AFFINITY@@@Z",
    )
}

/// `?set_task_execution_resources@Concurrency@@YAXGPEAU_GROUP_AFFINITY@@@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_set_task_execution_resources_concurrency_yaxgpeau_group_affinity_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?set_task_execution_resources@Concurrency@@YAXGPEAU_GROUP_AFFINITY@@@Z",
    )
}

/// `?set_task_execution_resources@Concurrency@@YAXK@Z`, an `@ stub` export for `-arch=win32`.
pub fn stub_set_task_execution_resources_concurrency_yaxk_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?set_task_execution_resources@Concurrency@@YAXK@Z",
    )
}

/// `?set_task_execution_resources@Concurrency@@YAX_K@Z`, an `@ stub` export for `-arch=win64`.
pub fn stub_set_task_execution_resources_concurrency_yax_k_z() -> StubException {
    wine_spec_unimplemented_stub(
        MSVCR120_APP_DLL_NAME,
        "?set_task_execution_resources@Concurrency@@YAX_K@Z",
    )
}

/// `?set_terminate@@YAP6AXXZH@Z`, an `@ stub` export.
pub fn stub_set_terminate_yap6axxzh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?set_terminate@@YAP6AXXZH@Z")
}

/// `?set_unexpected@@YAP6AXXZH@Z`, an `@ stub` export.
pub fn stub_set_unexpected_yap6axxzh_z() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "?set_unexpected@@YAP6AXXZH@Z")
}

/// `_FPE_Raise`, an `@ stub` export for `-arch=arm`.
pub fn stub_fpe_raise() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_FPE_Raise")
}

/// `_GetImageBase`, an `@ stub` export for `-arch=win64`.
pub fn stub_getimagebase() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_GetImageBase")
}

/// `_GetThrowImageBase`, an `@ stub` export for `-arch=win64`.
pub fn stub_getthrowimagebase() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_GetThrowImageBase")
}

/// `_LCbuild`, an `@ stub` export.
pub fn stub_lcbuild() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_LCbuild")
}

/// `_NLG_Dispatch2`, an `@ stub` export for `-arch=i386`.
#[allow(non_snake_case)]
pub fn stub__nlg_dispatch2() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_NLG_Dispatch2")
}

/// `__NLG_Dispatch2`, an `@ stub` export for `-arch=arm,win64`.
#[allow(non_snake_case)]
pub fn stub___nlg_dispatch2() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__NLG_Dispatch2")
}

/// `_NLG_Return`, an `@ stub` export for `-arch=i386`.
pub fn stub_nlg_return() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_NLG_Return")
}

/// `_NLG_Return2`, an `@ stub` export for `-arch=i386`.
#[allow(non_snake_case)]
pub fn stub__nlg_return2() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_NLG_Return2")
}

/// `__NLG_Return2`, an `@ stub` export for `-arch=arm,win64`.
#[allow(non_snake_case)]
pub fn stub___nlg_return2() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__NLG_Return2")
}

/// `_SetImageBase`, an `@ stub` export for `-arch=win64`.
pub fn stub_setimagebase() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_SetImageBase")
}

/// `_SetThrowImageBase`, an `@ stub` export for `-arch=win64`.
pub fn stub_setthrowimagebase() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_SetThrowImageBase")
}

/// `__BuildCatchObject`, an `@ stub` export.
pub fn stub_buildcatchobject() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__BuildCatchObject")
}

/// `__BuildCatchObjectHelper`, an `@ stub` export.
pub fn stub_buildcatchobjecthelper() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__BuildCatchObjectHelper")
}

/// `__FrameUnwindFilter`, an `@ stub` export.
pub fn stub_frameunwindfilter() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__FrameUnwindFilter")
}

/// `__GetPlatformExceptionInfo`, an `@ stub` export.
pub fn stub_getplatformexceptioninfo() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__GetPlatformExceptionInfo")
}

/// `__TypeMatch`, an `@ stub` export.
pub fn stub_typematch() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__TypeMatch")
}

/// `__crtCloseWinRTThreadHandle`, an `@ stub` export.
pub fn stub_crtclosewinrtthreadhandle() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtCloseWinRTThreadHandle")
}

/// `__crtCreateWinRTThread`, an `@ stub` export.
pub fn stub_crtcreatewinrtthread() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtCreateWinRTThread")
}

/// `__crtGetCurrentWinRTThread`, an `@ stub` export.
pub fn stub_crtgetcurrentwinrtthread() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtGetCurrentWinRTThread")
}

/// `__crtGetCurrentWinRTThreadId`, an `@ stub` export.
pub fn stub_crtgetcurrentwinrtthreadid() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtGetCurrentWinRTThreadId")
}

/// `__crtGetExitCodeWinRTThread`, an `@ stub` export.
pub fn stub_crtgetexitcodewinrtthread() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtGetExitCodeWinRTThread")
}

/// `__crtIsPackagedApp`, an `@ stub` export.
pub fn stub_crtispackagedapp() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtIsPackagedApp")
}

/// `__crtWaitForWinRTThreadExit`, an `@ stub` export.
pub fn stub_crtwaitforwinrtthreadexit() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__crtWaitForWinRTThreadExit")
}

/// `__get_flsindex`, an `@ stub` export.
pub fn stub_get_flsindex() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__get_flsindex")
}

/// `__get_tlsindex`, an `@ stub` export.
pub fn stub_get_tlsindex() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__get_tlsindex")
}

/// `__report_gsfailure`, an `@ stub` export.
pub fn stub_report_gsfailure() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__report_gsfailure")
}

/// `__unDNameHelper`, an `@ stub` export.
pub fn stub_undnamehelper() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__unDNameHelper")
}

/// `__wcsncnt`, an `@ stub` export.
pub fn stub_wcsncnt() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "__wcsncnt")
}

/// `_dosmaperr`, an `@ stub` export.
pub fn stub_dosmaperr() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_dosmaperr")
}

/// `_exit_app`, an `@ stub` export.
pub fn stub_exit_app() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_exit_app")
}

/// `_findfirst32i64`, an `@ stub` export.
pub fn stub_findfirst32i64() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_findfirst32i64")
}

/// `_findnext32i64`, an `@ stub` export.
pub fn stub_findnext32i64() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_findnext32i64")
}

/// `_freea`, an `@ stub` export.
pub fn stub_freea() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_freea")
}

/// `_freea_s`, an `@ stub` export.
pub fn stub_freea_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_freea_s")
}

/// `_freefls`, an `@ stub` export.
pub fn stub_freefls() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_freefls")
}

/// `_getws_s`, an `@ stub` export.
pub fn stub_getws_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_getws_s")
}

/// `_initptd`, an `@ stub` export.
pub fn stub_initptd() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_initptd")
}

/// `_invoke_watson`, an `@ stub` export.
pub fn stub_invoke_watson() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_invoke_watson")
}

/// `_ismbbalnum_l`, an `@ stub` export.
pub fn stub_ismbbalnum_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbalnum_l")
}

/// `_ismbbalpha_l`, an `@ stub` export.
pub fn stub_ismbbalpha_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbalpha_l")
}

/// `_ismbbblank`, an `@ stub` export.
pub fn stub_ismbbblank() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbblank")
}

/// `_ismbbblank_l`, an `@ stub` export.
pub fn stub_ismbbblank_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbblank_l")
}

/// `_ismbbgraph_l`, an `@ stub` export.
pub fn stub_ismbbgraph_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbgraph_l")
}

/// `_ismbbkalnum_l`, an `@ stub` export.
pub fn stub_ismbbkalnum_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbkalnum_l")
}

/// `_ismbbkprint_l`, an `@ stub` export.
pub fn stub_ismbbkprint_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbkprint_l")
}

/// `_ismbbkpunct_l`, an `@ stub` export.
pub fn stub_ismbbkpunct_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbkpunct_l")
}

/// `_ismbbprint_l`, an `@ stub` export.
pub fn stub_ismbbprint_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbprint_l")
}

/// `_ismbbpunct_l`, an `@ stub` export.
pub fn stub_ismbbpunct_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_ismbbpunct_l")
}

/// `_iswcsym_l`, an `@ stub` export.
pub fn stub_iswcsym_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_iswcsym_l")
}

/// `_iswcsymf_l`, an `@ stub` export.
pub fn stub_iswcsymf_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_iswcsymf_l")
}

/// `_lsearch_s`, an `@ stub` export.
pub fn stub_lsearch_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_lsearch_s")
}

/// `_mbstrnlen`, an `@ stub` export.
pub fn stub_mbstrnlen() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_mbstrnlen")
}

/// `_mbstrnlen_l`, an `@ stub` export.
pub fn stub_mbstrnlen_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_mbstrnlen_l")
}

/// `_printf_l`, an `@ stub` export.
pub fn stub_printf_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_printf_l")
}

/// `_printf_p`, an `@ stub` export.
pub fn stub_printf_p() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_printf_p")
}

/// `_printf_p_l`, an `@ stub` export.
pub fn stub_printf_p_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_printf_p_l")
}

/// `_printf_s_l`, an `@ stub` export.
pub fn stub_printf_s_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_printf_s_l")
}

/// `_recalloc_crt`, an `@ stub` export.
pub fn stub_recalloc_crt() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_recalloc_crt")
}

/// `_set_malloc_crt_max_wait`, an `@ stub` export.
pub fn stub_set_malloc_crt_max_wait() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_set_malloc_crt_max_wait")
}

/// `_strerror_s`, an `@ stub` export.
pub fn stub_strerror_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_strerror_s")
}

/// `_strset_s`, an `@ stub` export.
pub fn stub_strset_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_strset_s")
}

/// `_umask_s`, an `@ stub` export.
pub fn stub_umask_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_umask_s")
}

/// `_vprintf_l`, an `@ stub` export.
pub fn stub_vprintf_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vprintf_l")
}

/// `_vprintf_p`, an `@ stub` export.
pub fn stub_vprintf_p() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vprintf_p")
}

/// `_vprintf_p_l`, an `@ stub` export.
pub fn stub_vprintf_p_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vprintf_p_l")
}

/// `_vprintf_s_l`, an `@ stub` export.
pub fn stub_vprintf_s_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vprintf_s_l")
}

/// `_vwprintf_p`, an `@ stub` export.
pub fn stub_vwprintf_p() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vwprintf_p")
}

/// `_vwprintf_p_l`, an `@ stub` export.
pub fn stub_vwprintf_p_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vwprintf_p_l")
}

/// `_vwprintf_s_l`, an `@ stub` export.
pub fn stub_vwprintf_s_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_vwprintf_s_l")
}

/// `_wfindfirst32i64`, an `@ stub` export.
pub fn stub_wfindfirst32i64() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_wfindfirst32i64")
}

/// `_wfindnext32i64`, an `@ stub` export.
pub fn stub_wfindnext32i64() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_wfindnext32i64")
}

/// `_wprintf_p`, an `@ stub` export.
pub fn stub_wprintf_p() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_wprintf_p")
}

/// `_wprintf_p_l`, an `@ stub` export.
pub fn stub_wprintf_p_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_wprintf_p_l")
}

/// `_wprintf_s_l`, an `@ stub` export.
pub fn stub_wprintf_s_l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "_wprintf_s_l")
}

/// `cabs`, an `@ stub` export.
pub fn stub_cabs() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cabs")
}

/// `cabsf`, an `@ stub` export.
pub fn stub_cabsf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cabsf")
}

/// `cabsl`, an `@ stub` export.
pub fn stub_cabsl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cabsl")
}

/// `cacos`, an `@ stub` export.
pub fn stub_cacos() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cacos")
}

/// `cacosf`, an `@ stub` export.
pub fn stub_cacosf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cacosf")
}

/// `cacosh`, an `@ stub` export.
pub fn stub_cacosh() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cacosh")
}

/// `cacoshf`, an `@ stub` export.
pub fn stub_cacoshf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cacoshf")
}

/// `cacoshl`, an `@ stub` export.
pub fn stub_cacoshl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cacoshl")
}

/// `cacosl`, an `@ stub` export.
pub fn stub_cacosl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cacosl")
}

/// `cargl`, an `@ stub` export.
pub fn stub_cargl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cargl")
}

/// `casin`, an `@ stub` export.
pub fn stub_casin() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "casin")
}

/// `casinf`, an `@ stub` export.
pub fn stub_casinf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "casinf")
}

/// `casinh`, an `@ stub` export.
pub fn stub_casinh() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "casinh")
}

/// `casinhf`, an `@ stub` export.
pub fn stub_casinhf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "casinhf")
}

/// `casinhl`, an `@ stub` export.
pub fn stub_casinhl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "casinhl")
}

/// `casinl`, an `@ stub` export.
pub fn stub_casinl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "casinl")
}

/// `catan`, an `@ stub` export.
pub fn stub_catan() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "catan")
}

/// `catanf`, an `@ stub` export.
pub fn stub_catanf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "catanf")
}

/// `catanh`, an `@ stub` export.
pub fn stub_catanh() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "catanh")
}

/// `catanhf`, an `@ stub` export.
pub fn stub_catanhf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "catanhf")
}

/// `catanhl`, an `@ stub` export.
pub fn stub_catanhl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "catanhl")
}

/// `catanl`, an `@ stub` export.
pub fn stub_catanl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "catanl")
}

/// `ccos`, an `@ stub` export.
pub fn stub_ccos() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ccos")
}

/// `ccosf`, an `@ stub` export.
pub fn stub_ccosf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ccosf")
}

/// `ccosh`, an `@ stub` export.
pub fn stub_ccosh() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ccosh")
}

/// `ccoshf`, an `@ stub` export.
pub fn stub_ccoshf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ccoshf")
}

/// `ccoshl`, an `@ stub` export.
pub fn stub_ccoshl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ccoshl")
}

/// `ccosl`, an `@ stub` export.
pub fn stub_ccosl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ccosl")
}

/// `cexpf`, an `@ stub` export.
pub fn stub_cexpf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cexpf")
}

/// `cexpl`, an `@ stub` export.
pub fn stub_cexpl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cexpl")
}

/// `cimagl`, an `@ stub` export.
pub fn stub_cimagl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cimagl")
}

/// `clog`, an `@ stub` export.
pub fn stub_clog() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "clog")
}

/// `clog10`, an `@ stub` export.
pub fn stub_clog10() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "clog10")
}

/// `clog10f`, an `@ stub` export.
pub fn stub_clog10f() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "clog10f")
}

/// `clog10l`, an `@ stub` export.
pub fn stub_clog10l() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "clog10l")
}

/// `clogf`, an `@ stub` export.
pub fn stub_clogf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "clogf")
}

/// `clogl`, an `@ stub` export.
pub fn stub_clogl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "clogl")
}

/// `conj`, an `@ stub` export.
pub fn stub_conj() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "conj")
}

/// `conjf`, an `@ stub` export.
pub fn stub_conjf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "conjf")
}

/// `conjl`, an `@ stub` export.
pub fn stub_conjl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "conjl")
}

/// `cpow`, an `@ stub` export.
pub fn stub_cpow() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cpow")
}

/// `cpowf`, an `@ stub` export.
pub fn stub_cpowf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cpowf")
}

/// `cpowl`, an `@ stub` export.
pub fn stub_cpowl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cpowl")
}

/// `cproj`, an `@ stub` export.
pub fn stub_cproj() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cproj")
}

/// `cprojf`, an `@ stub` export.
pub fn stub_cprojf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cprojf")
}

/// `cprojl`, an `@ stub` export.
pub fn stub_cprojl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "cprojl")
}

/// `creall`, an `@ stub` export.
pub fn stub_creall() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "creall")
}

/// `csin`, an `@ stub` export.
pub fn stub_csin() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csin")
}

/// `csinf`, an `@ stub` export.
pub fn stub_csinf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csinf")
}

/// `csinh`, an `@ stub` export.
pub fn stub_csinh() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csinh")
}

/// `csinhf`, an `@ stub` export.
pub fn stub_csinhf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csinhf")
}

/// `csinhl`, an `@ stub` export.
pub fn stub_csinhl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csinhl")
}

/// `csinl`, an `@ stub` export.
pub fn stub_csinl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csinl")
}

/// `csqrt`, an `@ stub` export.
pub fn stub_csqrt() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csqrt")
}

/// `csqrtf`, an `@ stub` export.
pub fn stub_csqrtf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csqrtf")
}

/// `csqrtl`, an `@ stub` export.
pub fn stub_csqrtl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "csqrtl")
}

/// `ctan`, an `@ stub` export.
pub fn stub_ctan() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ctan")
}

/// `ctanf`, an `@ stub` export.
pub fn stub_ctanf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ctanf")
}

/// `ctanh`, an `@ stub` export.
pub fn stub_ctanh() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ctanh")
}

/// `ctanhf`, an `@ stub` export.
pub fn stub_ctanhf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ctanhf")
}

/// `ctanhl`, an `@ stub` export.
pub fn stub_ctanhl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ctanhl")
}

/// `ctanl`, an `@ stub` export.
pub fn stub_ctanl() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "ctanl")
}

/// `norm`, an `@ stub` export.
pub fn stub_norm() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "norm")
}

/// `normf`, an `@ stub` export.
pub fn stub_normf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "normf")
}

/// `norml`, an `@ stub` export.
pub fn stub_norml() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "norml")
}

/// `vfscanf`, an `@ stub` export.
pub fn stub_vfscanf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vfscanf")
}

/// `vfscanf_s`, an `@ stub` export.
pub fn stub_vfscanf_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vfscanf_s")
}

/// `vfwscanf`, an `@ stub` export.
pub fn stub_vfwscanf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vfwscanf")
}

/// `vfwscanf_s`, an `@ stub` export.
pub fn stub_vfwscanf_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vfwscanf_s")
}

/// `vscanf`, an `@ stub` export.
pub fn stub_vscanf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vscanf")
}

/// `vscanf_s`, an `@ stub` export.
pub fn stub_vscanf_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vscanf_s")
}

/// `vsscanf_s`, an `@ stub` export.
pub fn stub_vsscanf_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vsscanf_s")
}

/// `vswscanf_s`, an `@ stub` export.
pub fn stub_vswscanf_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vswscanf_s")
}

/// `vwscanf`, an `@ stub` export.
pub fn stub_vwscanf() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vwscanf")
}

/// `vwscanf_s`, an `@ stub` export.
pub fn stub_vwscanf_s() -> StubException {
    wine_spec_unimplemented_stub(MSVCR120_APP_DLL_NAME, "vwscanf_s")
}
