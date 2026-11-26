/// `KeAcquireSpinLock` from `dlls/hal/hal.c`. ntoskrnl takes the lock at DPC level, which is not
/// modelled here. The caller's IRQL is always set to 0.
pub fn ke_acquire_spin_lock(irql: &mut u8) {
    *irql = 0;
}

/// `KfAcquireSpinLock` from `dlls/hal/hal.c`: returns the IRQL that [`ke_acquire_spin_lock`] stores.
pub fn kf_acquire_spin_lock() -> u8 {
    let mut irql = 0;
    ke_acquire_spin_lock(&mut irql);
    irql
}
