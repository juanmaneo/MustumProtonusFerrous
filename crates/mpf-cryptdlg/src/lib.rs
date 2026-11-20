//! Deterministic logic from Wine's `dlls/cryptdlg/main.c`.
//!
//! This port covers the certificate trust-status confidence mapping,
//! `CertTrustInit`, `CRYPT_FormatHexString`, and the formatting stage of
//! `CRYPT_FormatCPS`. Certificate-chain, registry, ASN.1 decoding, resource,
//! UI, and forwarding operations are skipped because they depend on Windows
//! services or external runtime state. `format_cps` accepts a decoded string;
//! Wine's formatter copies through the first embedded NUL while this port
//! copies the complete Rust string, so that case is a documented deviation.

pub mod confidence;
pub mod cps;
pub mod fill;
pub mod hex;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `S_OK`.
pub const S_OK: i32 = 0;
/// `S_FALSE`.
pub const S_FALSE: i32 = 1;
/// `TRUSTERROR_MAX_STEPS`: entries in the provider trust-step error array.
pub const TRUSTERROR_MAX_STEPS: usize = 38;
/// `TRUSTERROR_STEP_FINAL_WVTINIT`: step checked by `CertTrustInit`.
pub const TRUSTERROR_STEP_FINAL_WVTINIT: usize = 30;

/// Mirrors Wine's `CertTrustInit` result for the provider step-error array.
pub fn cert_trust_init(step_errors: Option<&[u32; TRUSTERROR_MAX_STEPS]>) -> i32 {
    match step_errors {
        Some(steps) if steps[TRUSTERROR_STEP_FINAL_WVTINIT] == 0 => S_OK,
        _ => S_FALSE,
    }
}
