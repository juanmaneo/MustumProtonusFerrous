/// The failure cases of `copy_multiszA` / `copy_multiszW`.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// `SCARD_E_INVALID_PARAMETER`: a string of the list failed to convert in the size pass.
    InvalidParameter,
    /// `SCARD_E_NO_MEMORY`: a string failed to convert in the copy pass.
    NoMemory,
    /// `SCARD_E_INSUFFICIENT_BUFFER`: the caller's buffer holds fewer than `required` units.
    InsufficientBuffer { required: usize },
}

impl Error {
    /// The `SCARD_E_*` status code of this error.
    pub fn status(&self) -> i32 {
        match self {
            Error::InvalidParameter => 0x8010_0004_u32 as i32,
            Error::NoMemory => 0x8010_0006_u32 as i32,
            Error::InsufficientBuffer { .. } => 0x8010_0008_u32 as i32,
        }
    }
}
