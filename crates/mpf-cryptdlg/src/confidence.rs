/// `CERT_TRUST_IS_NOT_TIME_VALID`.
pub const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 0x0000_0001;
/// `CERT_TRUST_IS_NOT_TIME_NESTED`.
pub const CERT_TRUST_IS_NOT_TIME_NESTED: u32 = 0x0000_0002;
/// `CERT_TRUST_IS_NOT_SIGNATURE_VALID`.
pub const CERT_TRUST_IS_NOT_SIGNATURE_VALID: u32 = 0x0000_0008;
/// `CERT_CONFIDENCE_SIG`.
pub const CERT_CONFIDENCE_SIG: u32 = 0x1000_0000;
/// `CERT_CONFIDENCE_TIME`.
pub const CERT_CONFIDENCE_TIME: u32 = 0x0100_0000;
/// `CERT_CONFIDENCE_TIMENEST`.
pub const CERT_CONFIDENCE_TIMENEST: u32 = 0x0010_0000;

/// Maps chain trust error bits to the confidence flags Wine assigns to a certificate.
pub fn trust_status_to_confidence(error_status: u32) -> u32 {
    let mut confidence = 0;
    if error_status & CERT_TRUST_IS_NOT_SIGNATURE_VALID == 0 {
        confidence |= CERT_CONFIDENCE_SIG;
    }
    if error_status & CERT_TRUST_IS_NOT_TIME_VALID == 0 {
        confidence |= CERT_CONFIDENCE_TIME;
    }
    if error_status & CERT_TRUST_IS_NOT_TIME_NESTED == 0 {
        confidence |= CERT_CONFIDENCE_TIMENEST;
    }
    confidence
}
