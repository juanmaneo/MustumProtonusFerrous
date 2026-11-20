//! Deterministic transaction-option value semantics from Wine's
//! `dlls/xolehlp/xolehlp.c`.
//!
//! The COM interfaces, allocation, distributed transaction manager, and FIXME
//! stubs are intentionally not modeled. `XACTOPT` is a 32-bit timeout and its
//! fixed-size description; Wine's methods copy the complete value unchanged.

/// The HRESULT returned when an options pointer is null.
pub const E_INVALIDARG: i32 = 0x8007_0057_u32 as i32;
/// The successful HRESULT.
pub const S_OK: i32 = 0;
/// The byte capacity of `XACTOPT.szDescription` in Wine's `transact.idl`.
pub const DESCRIPTION_CAPACITY: usize = 40;

/// Portable value representation of Wine's `XACTOPT`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XactOptions {
    pub timeout: u32,
    pub description: [u8; DESCRIPTION_CAPACITY],
}

/// Stored `ITransactionOptions` state without COM identity or allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransactionOptions {
    options: XactOptions,
}

impl TransactionOptions {
    /// Creates initialized state. Wine's COM constructor leaves this value
    /// uninitialized until `SetOptions`, so callers must provide a value here.
    pub const fn new(options: XactOptions) -> Self {
        Self { options }
    }

    /// Mirrors `TransactionOptions_SetOptions`: null is invalid; otherwise the
    /// complete options value replaces the stored value.
    pub fn set_options(&mut self, options: Option<&XactOptions>) -> i32 {
        let Some(options) = options else {
            return E_INVALIDARG;
        };
        self.options = *options;
        S_OK
    }

    /// Mirrors `TransactionOptions_GetOptions`: null is invalid; otherwise a
    /// copy of the complete stored value is written to the destination.
    pub fn get_options(&self, output: Option<&mut XactOptions>) -> i32 {
        let Some(output) = output else {
            return E_INVALIDARG;
        };
        *output = self.options;
        S_OK
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
