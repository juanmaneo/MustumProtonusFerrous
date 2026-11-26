/// `VDMEnumProcessWOW` from `dlls/vdmdbg/vdmdbg.c`. The C body only traces its arguments and
/// returns 0 without calling the callback, so the port returns 0.
pub fn vdm_enum_process_wow() -> i32 {
    0
}

/// `VDMEnumTaskWOW` from `dlls/vdmdbg/vdmdbg.c`. The C body only traces its arguments and
/// returns 0 without calling the callback, so the port returns 0.
pub fn vdm_enum_task_wow() -> i32 {
    0
}
