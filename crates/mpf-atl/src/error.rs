/// A failed script step. Each variant maps to the HRESULT the C code returns.
pub enum Error {
    /// `DISP_E_EXCEPTION`: malformed script.
    Exception,
    /// `E_FAIL`: a `b` value had an odd length or a non-hex digit.
    Fail,
    /// A registry call failed with this Win32 error code.
    Win32(u32),
}

impl Error {
    pub fn hresult(self) -> u32 {
        match self {
            Error::Exception => 0x8002_0009,
            Error::Fail => 0x8000_4005,
            Error::Win32(code) => hresult_from_win32(code),
        }
    }
}

/// `HRESULT_FROM_WIN32`. Non-positive codes pass through unchanged, as the C macro does.
fn hresult_from_win32(code: u32) -> u32 {
    if (code as i32) > 0 {
        0x8007_0000 | (code & 0xffff)
    } else {
        code
    }
}
