/// `DCIRVAL` / `DCI_*` codes from `include/dciddi.h`.
pub const DCI_OK: i32 = 0;
pub const DCI_FAIL_GENERIC: i32 = -1;
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2;
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3;
pub const DCI_FAIL_UNSUPPORTED: i32 = -4;

/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// Device context handle. `HDC` is a pointer in C; the stubs only ever return `NULL` or pass it through.
pub type Hdc = usize;

/// `NULL` HDC.
pub const NULL_HDC: Hdc = 0;

/// Last-error slot that the DCI stubs write to.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Dciman32 {
    pub(crate) last_error: u32,
}

impl Dciman32 {
    /// Value of the last error set by the most recent call.
    pub fn last_error(&self) -> u32 {
        self.last_error
    }

    /// `DCIOpenProvider`: always fails with `NULL`.
    pub fn open_provider(&mut self) -> Hdc {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        NULL_HDC
    }

    /// `DCICloseProvider`: no effect beyond setting the last error.
    pub fn close_provider(&mut self, _hdc: Hdc) {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
    }

    /// `DCICreatePrimary`: the surface info pointer is only logged by the C
    /// stub, so it is accepted and ignored here.
    pub fn create_primary<T>(&mut self, _hdc: Hdc, _surface_info: Option<&mut T>) -> i32 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        DCI_FAIL_UNSUPPORTED
    }
}
