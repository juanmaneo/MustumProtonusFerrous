/// `HCOMDB`, from `DECLARE_HANDLE(HCOMDB)` in `dlls/msports/msports.c`. The C type is a pointer
/// to an opaque struct. It is modelled as an integer, as the `fltlib` port models `HANDLE`.
pub type HCOMDB = isize;

/// `INVALID_HANDLE_VALUE` from `include/winbase.h`, `((HANDLE)~(ULONG_PTR)0)`.
pub const INVALID_HANDLE_VALUE: HCOMDB = -1;

/// `ERROR_ACCESS_DENIED` from `include/winerror.h`.
pub const ERROR_ACCESS_DENIED: i32 = 5;

/// `ERROR_INVALID_PARAMETER` from `include/winerror.h`.
pub const ERROR_INVALID_PARAMETER: i32 = 87;

/// `ComDBOpen`: stores `INVALID_HANDLE_VALUE` in `*phComDB` and returns `ERROR_ACCESS_DENIED`.
/// The prior value of `*phComDB` is overwritten without being read.
pub fn com_db_open(phcomdb: &mut HCOMDB) -> i32 {
    *phcomdb = INVALID_HANDLE_VALUE;
    ERROR_ACCESS_DENIED
}

/// `ComDBClose`: returns `ERROR_INVALID_PARAMETER` for every handle, because the C body never
/// reads its argument.
pub fn com_db_close() -> i32 {
    ERROR_INVALID_PARAMETER
}
