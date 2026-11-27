//! Oracle-derived test vectors for the ported d3dx10_36 logic.
//!
//! The `D3DX10CheckVersion` vectors were captured from `target/canon/d3dx10_36/harness.c`, which
//! compiles the unmodified `dlls/d3dx10_36/d3dx10_36_main.c` with the Windows header shims in
//! `target/canon/d3dx10_36/shim/`, using `gcc -fsanitize=address,undefined -g`. The run printed
//! 22 rows with no sanitizer reports, so no vector is excluded.
//!
//! The export table checks come from `target/canon/d3dx10_36/spec_table.py`, which parses
//! `dlls/d3dx10_36/d3dx10_36.spec` and reports 180 entries, 172 forwards and 8 names without a
//! forward. The `@ stub` count is 0.

use super::*;

/// `(d3dsdkvers, d3dxsdkvers, raw result)` rows from `harness.c`.
const CHECK_VERSION_VECTORS: [(u32, u32, bool); 22] = [
    (29, 36, true),
    (29, 35, false),
    (29, 37, false),
    (29, 42, false),
    (29, 43, false),
    (29, 0, false),
    (29, 1, false),
    (29, 0xffff_ffff, false),
    (28, 36, false),
    (30, 36, false),
    (0, 36, false),
    (0xffff_ffff, 36, false),
    (36, 29, false),
    (0, 0, false),
    (0xffff_ffff, 0xffff_ffff, false),
    (29, 36, true),
    (29, 292, false),
    (285, 36, false),
    (36, 36, false),
    (1, 36, false),
    (29, 2_147_483_684, false),
    (2_147_483_677, 36, false),
];

#[test]
fn check_version_matches_harness() {
    for (d3d, d3dx, expected) in CHECK_VERSION_VECTORS {
        assert_eq!(d3dx10_check_version(d3d, d3dx), expected, "{d3d} {d3dx}");
    }
}

#[test]
fn spec_table_summary_matches_generator() {
    assert_eq!(EXPORTS.len(), 180);

    let forwards: Vec<(&str, &str)> = EXPORTS
        .iter()
        .filter_map(|(name, kind)| match kind {
            ExportKind::Forward(dll) => Some((*name, *dll)),
            ExportKind::Stdcall => None,
        })
        .collect();
    assert_eq!(forwards.len(), 172);
    let to = |dll: &str| forwards.iter().filter(|(_, d)| *d == dll).count();
    assert_eq!(to("d3dx10_43"), 168);
    assert_eq!(to("d3dx10_39"), 3);
    assert_eq!(to("d3dx10_37"), 1);

    let local: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        local,
        [
            "D3DX10CheckVersion",
            "D3DX10CompileFromMemory",
            "D3DX10CreateEffectFromFileA",
            "D3DX10CreateEffectFromFileW",
            "D3DX10CreateEffectFromMemory",
            "D3DX10CreateEffectFromResourceA",
            "D3DX10CreateEffectFromResourceW",
            "D3DX10PreprocessShaderFromMemory",
        ]
    );
}

#[test]
fn spec_table_names_are_unique() {
    let mut names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}
