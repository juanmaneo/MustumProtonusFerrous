/// `KfLowerIrql` from `dlls/hal/hal.c`. The C body only traces the call.
pub fn kf_lower_irql(_new_irql: u8) {}

/// `KfRaiseIrql` from `dlls/hal/hal.c`: reports IRQL 0 whatever level was requested.
pub fn kf_raise_irql(_new_irql: u8) -> u8 {
    0
}

/// `KeGetCurrentIrql` from `dlls/hal/hal.c`: reports IRQL 0.
pub fn ke_get_current_irql() -> u8 {
    0
}
