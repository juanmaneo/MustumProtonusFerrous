//! Oracle-derived test vectors for the ported hrtfapo logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `CreateHrtfApo`
//! (`dlls/hrtfapo/main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -std=gnu11 -fsanitize=address,undefined -g` under `target/canon/hrtfapo/`, and running the
//! harness. The 17 printed rows all ran clean under ASan and UBSan, so none is excluded.

use super::*;

/// `(id, init, slot, ret, slot_after)`. `init` is the first byte of the C stand-in `HrtfApoInit`
/// (`None` is a NULL pointer). `slot` is `*xapo` before the call (`None` is a NULL `xapo`), and
/// `slot_after` is `*xapo` after the call.
type CreateRow = (&'static str, Option<u8>, Option<u32>, u32, Option<u32>);

/// `(id, code, flags, nargs, module, function)`.
type StubRow = (&'static str, u32, u32, u32, &'static str, &'static str);

/// The `CreateHrtfApo` rows R01 to R09.
const CREATE_HRTF_APO_ROWS: [CreateRow; 9] = [
    ("R01", None, None, 0x8000_4001, None),
    ("R02", None, Some(0), 0x8000_4001, Some(0)),
    ("R03", None, Some(0xdead), 0x8000_4001, Some(0xdead)),
    ("R04", Some(0x01), None, 0x8000_4001, None),
    ("R05", Some(0x01), Some(0), 0x8000_4001, Some(0)),
    ("R06", Some(0x01), Some(0xdead), 0x8000_4001, Some(0xdead)),
    ("R07", Some(0xff), None, 0x8000_4001, None),
    ("R08", Some(0xff), Some(0), 0x8000_4001, Some(0)),
    ("R09", Some(0xff), Some(0xdead), 0x8000_4001, Some(0xdead)),
];

/// The `__wine_spec_unimplemented_stub` rows R10 to R17. Each call raised once (`raises=1` in the
/// harness output) before the harness left the loop.
const STUB_ROWS: [StubRow; 8] = [
    (
        "R10",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "CreateHrtfApoWithDatasetType",
    ),
    (
        "R11",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "CreateHrtfEngineFactory",
    ),
    (
        "R12",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "IsHrtfApoAvailable",
    ),
    (
        "R13",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "GetHrtfEngineMinFrameCount",
    ),
    (
        "R14",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "GetHrtfEngineMinFrameCount",
    ),
    (
        "R15",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "IsHrtfApoAvailable",
    ),
    (
        "R16",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "CreateHrtfEngineFactory",
    ),
    (
        "R17",
        0x8000_0100,
        0x01,
        2,
        "hrtfapo.dll",
        "CreateHrtfApoWithDatasetType",
    ),
];

#[test]
fn oracle_create_hrtf_apo_rows() {
    for &(id, init, slot, ret, after) in CREATE_HRTF_APO_ROWS.iter() {
        let mut value = slot.unwrap_or(0);
        let got = create_hrtf_apo::<u8, u32>(init.as_ref(), slot.map(|_| &mut value));
        assert_eq!(got, ret, "{id}");
        assert_eq!(slot.map(|_| value), after, "{id}");
    }
}

#[test]
fn oracle_stub_rows() {
    // R14 to R17 call the same four exports again, in reverse order. No state carries over
    // between raises.
    let got = [
        create_hrtf_apo_with_dataset_type(),
        create_hrtf_engine_factory(),
        is_hrtf_apo_available(),
        get_hrtf_engine_min_frame_count(),
        get_hrtf_engine_min_frame_count(),
        is_hrtf_apo_available(),
        create_hrtf_engine_factory(),
        create_hrtf_apo_with_dataset_type(),
    ];
    for ((id, code, flags, nargs, module, function), got) in STUB_ROWS.into_iter().zip(got) {
        let want = StubException {
            code,
            flags,
            number_parameters: nargs,
            module,
            function,
        };
        assert_eq!(got, want, "{id}");
    }
}
