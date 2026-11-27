use crate::stub::{
    apply_delta_a, apply_delta_b, apply_delta_provided_b, apply_delta_w, create_delta_a,
    create_delta_b, create_delta_w, delta_free, delta_normalize_provided_b, get_delta_info_a,
    get_delta_info_b, get_delta_info_w, get_delta_signature_a, get_delta_signature_b,
    get_delta_signature_w, StubException,
};

type StubHelper = fn() -> StubException;

/// The `dlls/msdelta/msdelta.spec` export table, in spec order. Every export is `@ stub`, and each
/// name is paired with the helper that raises its record.
pub const EXPORTS: [(&str, StubHelper); 15] = [
    ("ApplyDeltaA", apply_delta_a),
    ("ApplyDeltaB", apply_delta_b),
    ("ApplyDeltaProvidedB", apply_delta_provided_b),
    ("ApplyDeltaW", apply_delta_w),
    ("CreateDeltaA", create_delta_a),
    ("CreateDeltaB", create_delta_b),
    ("CreateDeltaW", create_delta_w),
    ("DeltaFree", delta_free),
    ("DeltaNormalizeProvidedB", delta_normalize_provided_b),
    ("GetDeltaInfoA", get_delta_info_a),
    ("GetDeltaInfoB", get_delta_info_b),
    ("GetDeltaInfoW", get_delta_info_w),
    ("GetDeltaSignatureA", get_delta_signature_a),
    ("GetDeltaSignatureB", get_delta_signature_b),
    ("GetDeltaSignatureW", get_delta_signature_w),
];

/// The stub record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, stub)| stub())
}
