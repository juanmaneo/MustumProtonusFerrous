/// `AvQuerySystemResponsiveness`: always FALSE, and never writes the value pointer.
pub fn av_query_system_responsiveness(_avrt_handle: usize) -> bool {
    false
}

/// `AvRevertMmThreadCharacteristics`: always TRUE.
pub fn av_revert_mm_thread_characteristics(_avrt_handle: usize) -> bool {
    true
}

/// `AvSetMmThreadPriority`: always TRUE, whatever the priority.
pub fn av_set_mm_thread_priority(_avrt_handle: usize, _priority: i32) -> bool {
    true
}
