//! Oracle-derived test vectors for the ported wofutil logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `WofIsExternalFile` and
//! `WofShouldCompressBinaries` (`dlls/wofutil/main.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/wofutil/`, and running the harness. The 21 printed rows (01 to 21) are all
//! clean under ASan and UBSan, so no vector is excluded.

use super::*;

/// Rows 01 to 09, one per `@ stub` export in spec order:
/// `(export, code, flags, number of parameters, module, function)`.
const STUB_ROWS: [(&str, u32, u32, u32, &str, &str); 9] = [
    (
        "WofEnumEntries",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofEnumEntries",
    ),
    (
        "WofFileEnumFiles",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofFileEnumFiles",
    ),
    (
        "WofGetDriverVersion",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofGetDriverVersion",
    ),
    (
        "WofSetFileDataLocation",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofSetFileDataLocation",
    ),
    (
        "WofWimAddEntry",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofWimAddEntry",
    ),
    (
        "WofWimEnumFiles",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofWimEnumFiles",
    ),
    (
        "WofWimRemoveEntry",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofWimRemoveEntry",
    ),
    (
        "WofWimSuspendEntry",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofWimSuspendEntry",
    ),
    (
        "WofWimUpdateEntry",
        0x8000_0100,
        0x1,
        2,
        "wofutil.dll",
        "WofWimUpdateEntry",
    ),
];

/// Which arguments of `WofIsExternalFile` are passed as non-NULL: `(path, result, provider, ptr, length)`.
type ExternalInputs = (bool, bool, bool, bool, bool);

/// The printed `(hr, result, provider, length)`, where `None` is the "-" of an unpassed argument.
type ExternalOutputs = (i32, Option<i32>, Option<u32>, Option<u32>);

/// Which arguments of `WofShouldCompressBinaries` are passed as non-NULL: `(volume, alg)`.
type CompressInputs = (bool, bool);

/// The printed `(ret, alg)`, where `None` is the "-" of an unpassed `alg`.
type CompressOutputs = (i32, Option<u32>);

/// Calls `wof_is_external_file` the way the harness does. The sentinels are `result` -1,
/// `provider` 0xdead and `length` 0xbeef, and each `true` flag passes that argument as non-NULL.
fn is_external_file(inputs: ExternalInputs) -> ExternalOutputs {
    let (path, result, provider, ptr, length) = inputs;
    let path_text = [u16::from(b'a'), 0];
    let mut result_value = -1;
    let mut provider_value = 0xdead;
    let mut length_value = 0xbeef;
    let hr = wof_is_external_file(
        path.then_some(&path_text[..]),
        result.then_some(&mut result_value),
        provider.then_some(&mut provider_value),
        usize::from(ptr),
        length.then_some(&mut length_value),
    );
    (
        hr,
        result.then_some(result_value),
        provider.then_some(provider_value),
        length.then_some(length_value),
    )
}

#[test]
fn oracle_stub_rows() {
    for &(name, code, flags, number_parameters, module, function) in &STUB_ROWS {
        let (_, helper) = STUB_EXPORTS.iter().find(|(n, _)| *n == name).unwrap();
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}

#[test]
fn oracle_wof_is_external_file_rows() {
    let rows: [(ExternalInputs, ExternalOutputs); 8] = [
        // row 10
        ((false, false, false, false, false), (0, None, None, None)),
        // row 11
        (
            (true, true, true, true, true),
            (0, Some(0), Some(0), Some(0)),
        ),
        // row 12
        ((false, true, false, false, false), (0, Some(0), None, None)),
        // row 13
        ((false, false, true, false, false), (0, None, Some(0), None)),
        // row 14
        ((false, false, false, false, true), (0, None, None, Some(0))),
        // row 15
        ((false, false, false, true, false), (0, None, None, None)),
        // row 16
        (
            (true, true, true, false, false),
            (0, Some(0), Some(0), None),
        ),
        // row 17
        (
            (false, true, false, true, true),
            (0, Some(0), None, Some(0)),
        ),
    ];
    for (inputs, expected) in rows {
        assert_eq!(is_external_file(inputs), expected);
    }
}

#[test]
fn oracle_wof_should_compress_binaries_rows() {
    // alg starts at 0xfeed, which is 65261, and is unchanged whenever it is passed.
    let volume = [u16::from(b'C'), u16::from(b':'), u16::from(b'\\'), 0];
    let rows: [(CompressInputs, CompressOutputs); 4] = [
        // row 18
        ((false, false), (0, None)),
        // row 19
        ((true, false), (0, None)),
        // row 20
        ((false, true), (0, Some(65261))),
        // row 21
        ((true, true), (0, Some(65261))),
    ];
    for ((use_volume, use_alg), (ret, alg_after)) in rows {
        let mut alg = 0xfeed;
        let got = wof_should_compress_binaries(
            use_volume.then_some(&volume[..]),
            use_alg.then_some(&mut alg),
        );
        assert_eq!(got, ret);
        assert_eq!(use_alg.then_some(alg), alg_after);
    }
}
