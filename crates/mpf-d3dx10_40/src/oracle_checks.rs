//! Oracle vectors captured by `target/canon/d3dx10_40/harness.c`, which compiles the unmodified
//! `dlls/d3dx10_40/d3dx10_40_main.c` with Win32 header shims using
//! `gcc -fsanitize=address,undefined -g`. The run produced the version rows below; sanitizers
//! reported no findings.

use super::d3dx10_check_version;

#[test]
fn version_matches_oracle_rows() {
    // version 29 40 1
    assert!(d3dx10_check_version(29, 40));
    // version 29 39 0
    assert!(!d3dx10_check_version(29, 39));
    // version 28 40 0
    assert!(!d3dx10_check_version(28, 40));
}
