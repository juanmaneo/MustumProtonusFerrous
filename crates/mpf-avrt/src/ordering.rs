/// `EXCEPTION_WINE_STUB` from `include/winternl.h`, raised when a stub entry point is called.
const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The exception record that `__wine_spec_unimplemented_stub` raises for one export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubRaise {
    pub code: u32,
    pub flags: u32,
    /// `ExceptionInformation[1]`: the name of the export that was called.
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub`. The C function never returns; this returns the record it raises.
pub fn unimplemented_stub(function: &'static str) -> StubRaise {
    StubRaise {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        function,
    }
}

/// `AvRtCreateThreadOrderingGroup` (`@ stub`).
pub fn av_rt_create_thread_ordering_group() -> StubRaise {
    unimplemented_stub("AvRtCreateThreadOrderingGroup")
}

/// `AvRtCreateThreadOrderingGroupExA` (`@ stub`).
pub fn av_rt_create_thread_ordering_group_ex_a() -> StubRaise {
    unimplemented_stub("AvRtCreateThreadOrderingGroupExA")
}

/// `AvRtCreateThreadOrderingGroupExW` (`@ stub`).
pub fn av_rt_create_thread_ordering_group_ex_w() -> StubRaise {
    unimplemented_stub("AvRtCreateThreadOrderingGroupExW")
}

/// `AvRtDeleteThreadOrderingGroup` (`@ stub`).
pub fn av_rt_delete_thread_ordering_group() -> StubRaise {
    unimplemented_stub("AvRtDeleteThreadOrderingGroup")
}

/// `AvRtJoinThreadOrderingGroup` (`@ stub`).
pub fn av_rt_join_thread_ordering_group() -> StubRaise {
    unimplemented_stub("AvRtJoinThreadOrderingGroup")
}

/// `AvRtLeaveThreadOrderingGroup` (`@ stub`).
pub fn av_rt_leave_thread_ordering_group() -> StubRaise {
    unimplemented_stub("AvRtLeaveThreadOrderingGroup")
}

/// `AvRtWaitOnThreadOrderingGroup` (`@ stub`).
pub fn av_rt_wait_on_thread_ordering_group() -> StubRaise {
    unimplemented_stub("AvRtWaitOnThreadOrderingGroup")
}
