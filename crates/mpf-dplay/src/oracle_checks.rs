//! Oracle-derived test vectors for the ported dplay logic.
//!
//! The reference has no C bodies, because `dlls/dplay` is a data-only DLL made of forwards. The
//! vectors come from running `winebuild` (`tools/winebuild`, built from `target/repos/wine` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/dplay/`) on `dlls/dplay/dplay.spec`
//! and on mutant specs in `target/canon/dplay/specs/`. The 18 rows below are numbered as printed.
//! ASan and UBSan flagged none of them, so none is excluded. LeakSanitizer reports leaks inside
//! `winebuild` at exit. They are not row errors, and the runs used `ASAN_OPTIONS=detect_leaks=0`.

use super::*;

/// The forwards of `dplay.spec`, in spec order.
fn spec_forwards() -> Vec<&'static str> {
    EXPORTS.iter().map(|e| e.forward).collect()
}

#[test]
fn oracle_dplay_spec_rows() {
    // row 01 i386 `--def`: DirectPlayCreate@12=dplayx.DirectPlayCreate @1
    assert_eq!(
        export("DirectPlayCreate"),
        Some(Export {
            name: "DirectPlayCreate",
            ordinal: 1,
            nb_args: 3,
            forward: "dplayx.DirectPlayCreate",
        })
    );
    assert_eq!(
        export("DirectPlayCreate").map(|e| e.stack_bytes()),
        Some(12)
    );

    // row 02 i386 `--def`: DirectPlayEnumerate@8=dplayx.DirectPlayEnumerate @2
    assert_eq!(
        export("DirectPlayEnumerate"),
        Some(Export {
            name: "DirectPlayEnumerate",
            ordinal: 2,
            nb_args: 2,
            forward: "dplayx.DirectPlayEnumerate",
        })
    );
    assert_eq!(
        export("DirectPlayEnumerate").map(|e| e.stack_bytes()),
        Some(8)
    );

    // row 03 i386 `--def`: LIBRARY dplay.dll (also the export directory name string)
    assert_eq!(MODULE_NAME, "dplay.dll");

    // row 04 i386 `--dll` export directory: Base 1, NumberOfFunctions 2, NumberOfNames 2
    assert_eq!(EXPORTS.len(), 2);
    assert_eq!(EXPORTS.iter().map(|e| e.ordinal).min(), Some(1));

    // rows 05 and 06 x86_64 `--def`: the same names, ordinals and forwards, with no @N suffix.
    // The crate models i386 stack sizes only.
    assert_eq!(EXPORTS[0].name, "DirectPlayCreate");
    assert_eq!(EXPORTS[0].ordinal, 1);
    assert_eq!(EXPORTS[0].forward, "dplayx.DirectPlayCreate");
    assert_eq!(EXPORTS[1].name, "DirectPlayEnumerate");
    assert_eq!(EXPORTS[1].ordinal, 2);
    assert_eq!(EXPORTS[1].forward, "dplayx.DirectPlayEnumerate");
}

#[test]
fn oracle_stack_bytes_rows() {
    // rows 07-11 i386 `--def` on arity.spec: `Fn@4n=dplayx.Fn @ordinal`, for n = 0..=4
    const ARITY_ROWS: [(&str, u16, u32, &str, u32); 5] = [
        ("F0", 1, 0, "dplayx.F0", 0),
        ("F1", 2, 1, "dplayx.F1", 4),
        ("F2", 3, 2, "dplayx.F2", 8),
        ("F3", 4, 3, "dplayx.F3", 12),
        ("F4", 5, 4, "dplayx.F4", 16),
    ];
    for (name, ordinal, nb_args, forward, bytes) in ARITY_ROWS {
        let e = Export {
            name,
            ordinal,
            nb_args,
            forward,
        };
        assert_eq!(e.stack_bytes(), bytes, "{name}");
    }

    // rows 12 and 13 i386 `--def` on order.spec: `Z@4=dplayx.Z @1` and `Y@4=dplayx.Y @2`.
    // Ordinals follow spec order, not name order. The crate records ordinals as values, so the
    // rows only check the 4-byte stack size here.
    let z = Export {
        name: "Z",
        ordinal: 1,
        nb_args: 1,
        forward: "dplayx.Z",
    };
    let y = Export {
        name: "Y",
        ordinal: 2,
        nb_args: 1,
        forward: "dplayx.Y",
    };
    assert_eq!((z.stack_bytes(), y.stack_bytes()), (4, 4));
}

#[test]
fn oracle_data_only_rows() {
    // row 14 i386 `--def --data-only` on dplay.spec: accepted, exit 0
    assert!(data_only_accepts(&spec_forwards()));

    // row 15 `--data-only` on `@ stdcall N(ptr) N_impl`: rejected, exit 1, "Only forwarded entry
    // points are allowed in data-only mode"
    assert!(!data_only_accepts(&["N_impl"]));

    // row 16 `--data-only` on `@ stdcall N(ptr)` (link name is N): rejected, exit 1, same message
    assert!(!data_only_accepts(&["N"]));

    // row 17 `--data-only` on arity.spec: accepted, exit 0
    assert!(data_only_accepts(&[
        "dplayx.F0",
        "dplayx.F1",
        "dplayx.F2",
        "dplayx.F3",
        "dplayx.F4",
    ]));

    // row 18 `--data-only` on an empty spec: accepted, exit 0
    assert!(data_only_accepts(&[]));
}
