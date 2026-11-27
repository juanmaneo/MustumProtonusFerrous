//! Oracle-derived test vectors for d3dx10_37.
//!
//! Version and stub rows were captured from `target/canon/d3dx10_37/harness.c`, which compiles
//! the unmodified `dlls/d3dx10_37/d3dx10_37_main.c` and the verbatim
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` with Windows header shims,
//! using `gcc -fsanitize=address,undefined -g`. The run printed 22 version rows and both stub
//! records with no sanitizer reports, so no vector is excluded. Export-table counts were parsed
//! from `dlls/d3dx10_37/d3dx10_37.spec`.

use super::*;

/// `(D3D10 SDK version, D3DX10 SDK version, raw BOOL result)` rows from the C harness.
const CHECK_VERSION_VECTORS: [(u32, u32, bool); 22] = [
    (29, 37, true),
    (29, 36, false),
    (29, 38, false),
    (29, 42, false),
    (29, 43, false),
    (29, 0, false),
    (29, 1, false),
    (29, 0xffff_ffff, false),
    (28, 37, false),
    (30, 37, false),
    (0, 37, false),
    (0xffff_ffff, 37, false),
    (37, 29, false),
    (0, 0, false),
    (0xffff_ffff, 0xffff_ffff, false),
    (29, 37, true),
    (29, 293, false),
    (285, 37, false),
    (37, 37, false),
    (1, 37, false),
    (29, 2_147_483_685, false),
    (2_147_483_677, 37, false),
];

#[test]
fn check_version_matches_harness() {
    for (d3d, d3dx, expected) in CHECK_VERSION_VECTORS {
        assert_eq!(d3dx10_check_version(d3d, d3dx), expected, "{d3d} {d3dx}");
    }
}

#[test]
fn stub_records_match_harness() {
    let expected = StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module: "d3dx10_37.dll",
        function: "D3DX10CreateReduction",
    };
    assert_eq!(d3dx10_create_reduction(), expected);
    assert_eq!(
        d3dx10_get_driver_level(),
        StubException {
            function: "D3DX10GetDriverLevel",
            ..expected
        }
    );
}

#[test]
fn spec_table_summary_matches_reference() {
    assert_eq!(EXPORTS.len(), 181);

    let forwards = EXPORTS
        .iter()
        .filter(|(_, kind)| matches!(kind, ExportKind::Forward(_)))
        .count();
    let forwards_to = |dll| {
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Forward(dll))
            .count()
    };
    assert_eq!(forwards, 171);
    assert_eq!(forwards_to("d3dx10_43"), 168);
    assert_eq!(forwards_to("d3dx10_39"), 3);

    let local: Vec<&str> = EXPORTS
        .iter()
        .filter_map(|(name, kind)| (*kind == ExportKind::Stdcall).then_some(*name))
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

    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter_map(|(name, kind)| (*kind == ExportKind::Stub).then_some(*name))
        .collect();
    assert_eq!(stubs, ["D3DX10CreateReduction", "D3DX10GetDriverLevel"]);
}

#[test]
fn spec_table_names_are_unique() {
    let mut names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}
