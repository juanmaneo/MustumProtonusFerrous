//! Edge-case tests for the ported ctapi32 logic.
//!
//! These cover the registry, soname and attach boundaries, the DllMain reasons the
//! oracle rows do not reach, and the Rust-only shape of the exports. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use crate::oracle_checks::{open_lib, reg_bytes, run_dll_main, FakeLib};

const DLL_THREAD_DETACH: u32 = 3;

/// Library whose every entry point returns the same status.
struct Fixed(i8);

impl CtLibrary for Fixed {
    type Ptr = ();

    fn ct_init(&self, _: CtInitParams) -> i8 {
        self.0
    }

    fn ct_data(&self, _: CtDataParams<()>) -> i8 {
        self.0
    }

    fn ct_close(&self, _: CtCloseParams) -> i8 {
        self.0
    }
}

#[test]
fn registry_value_limit_is_518_bytes() {
    // Not oracle-derived: 518 bytes is MAX_PATH wide chars less the terminator.
    let data = [b'A', 0].repeat(259);
    assert_eq!(
        registry_library(Some((REG_SZ, data.as_slice()))),
        Some(vec![u16::from(b'A'); 259])
    );
    let too_long = vec![0u8; 519];
    assert_eq!(registry_library(Some((REG_SZ, too_long.as_slice()))), None);
}

#[test]
fn registry_drops_odd_trailing_byte() {
    // Not oracle-derived: direct check of the odd-byte rule; oracle row 22 covers it end to end.
    let data = [0x41, 0x00, 0x42];
    assert_eq!(
        registry_library(Some((REG_SZ, data.as_slice()))),
        Some(vec![0x41])
    );
}

#[test]
fn registry_empty_value_is_empty_name() {
    // Not oracle-derived: a zero-length value gives buffer_w[0] = 0, so the name is "", not the fallback.
    let data: [u8; 0] = [];
    assert_eq!(soname(Some((REG_SZ, data.as_slice()))), Vec::<u8>::new());
}

#[test]
fn narrow_name_stops_at_first_nul() {
    // Not oracle-derived: WideCharToMultiByte(-1) converts up to the first NUL unit.
    assert_eq!(narrow_name(&[0x41, 0, 0x42]), Some(b"A".to_vec()));
}

#[test]
fn narrow_name_limit_is_259_bytes() {
    // Not oracle-derived: soname[MAX_PATH] must hold the name plus its NUL.
    let fits = vec![u16::from(b'a'); 259];
    assert_eq!(
        narrow_name(fits.as_slice()).map(|name| name.len()),
        Some(259)
    );
    let overflows = vec![u16::from(b'a'); 260];
    assert_eq!(narrow_name(overflows.as_slice()), None);
}

#[test]
fn narrow_name_counts_utf8_bytes() {
    // Not oracle-derived: the limit applies to UTF-8 bytes; 129 x 'é' is 258 bytes and fits.
    let value = reg_bytes(&"é".repeat(129), true);
    assert_eq!(soname(Some((REG_SZ, value.as_slice()))).len(), 258);
}

#[test]
fn soname_multibyte_overflow_falls_back() {
    // Not oracle-derived: 130 x 'é' is 260 UTF-8 bytes, which Wine's soname[MAX_PATH] cannot hold.
    let value = reg_bytes(&"é".repeat(130), true);
    assert_eq!(
        soname(Some((REG_SZ, value.as_slice()))),
        FALLBACK_LIBCTAPI.to_vec()
    );
}

#[test]
fn narrow_name_lone_surrogate_is_replacement_char() {
    // Not oracle-derived: invalid UTF-16 maps to U+FFFD (lossy conversion assumed for CP_UNIXCP).
    assert_eq!(narrow_name(&[0xD800]), Some("\u{FFFD}".as_bytes().to_vec()));
}

#[test]
fn soname_uses_registry_value_and_falls_back_on_wrong_type() {
    // Not oracle-derived: a registry name is used as it is; the wrong-type fallback is also oracle row 20.
    let value = reg_bytes("./libctapi.so", true);
    assert_eq!(
        soname(Some((REG_SZ, value.as_slice()))),
        b"./libctapi.so".to_vec()
    );
    assert_eq!(
        soname(Some((4, value.as_slice()))),
        FALLBACK_LIBCTAPI.to_vec()
    );
    assert_eq!(soname(None), FALLBACK_LIBCTAPI.to_vec());
}

#[test]
fn load_functions_passes_soname_to_dlopen() {
    // Not oracle-derived: the soname selected from the registry is the name handed to dlopen.
    let value = reg_bytes("./libctapi.so", true);
    let mut requested = Vec::new();
    let loaded = load_functions(Some((REG_SZ, value.as_slice())), |name: &[u8]| {
        requested.push(name.to_vec());
        open_lib(name)
    });
    assert!(loaded);
    assert_eq!(requested, [b"./libctapi.so".to_vec()]);
}

#[test]
fn attach_looks_up_ct_symbols_in_order() {
    // Not oracle-derived: lookup order follows the C LOAD_FUNCPTR sequence; all present gives success.
    let mut asked = Vec::new();
    let status = attach(Some(|name: &str| {
        asked.push(name.to_string());
        true
    }));
    assert_eq!(status, STATUS_SUCCESS);
    assert_eq!(asked, ["CT_init", "CT_data", "CT_close"]);
}

#[test]
fn attach_stops_at_first_missing_symbol() {
    // Not oracle-derived: after a missing symbol, Wine returns without looking up the rest.
    let mut asked = Vec::new();
    let status = attach(Some(|name: &str| {
        asked.push(name.to_string());
        name != "CT_data"
    }));
    assert_eq!(status, STATUS_ENTRYPOINT_NOT_FOUND);
    assert_eq!(asked, ["CT_init", "CT_data"]);
}

#[test]
fn dll_main_other_reasons_do_nothing() {
    // Not oracle-derived: the switch has no DLL_THREAD_DETACH branch, so it returns TRUE untouched.
    assert_eq!(
        run_dll_main(DLL_THREAD_DETACH, false, STATUS_SUCCESS, None),
        (true, 0, 0, 0)
    );
}

#[test]
fn dll_main_nonzero_init_status_fails_attach() {
    // Not oracle-derived: Wine tests `if (__wine_init_unix_call())`, so any non-zero status fails.
    assert_eq!(
        run_dll_main(DLL_PROCESS_ATTACH, false, 1, None),
        (false, 1, 0, 0)
    );
}

#[test]
fn exports_return_every_i8_status_unchanged() {
    // Not oracle-derived: the NTSTATUS widening and IS8 narrowing are the identity over i8.
    for value in i8::MIN..=i8::MAX {
        let lib = Fixed(value);
        assert_eq!(ct_init(&lib, 0, 0), value);
        assert_eq!(ct_close(&lib, 0), value);
        let params = CtDataParams {
            ctn: 0,
            dad: (),
            sad: (),
            lenc: 0,
            command: (),
            lenr: (),
            response: (),
        };
        assert_eq!(ct_data(&lib, params), value);
    }
}

#[test]
fn unix_status_keeps_sign_of_library_result() {
    // Not oracle-derived: the oracle prints only IS8 results; the NTSTATUS is the same value sign-extended.
    assert_eq!(unix_ct_close(&Fixed(-56), CtCloseParams { ctn: 200 }), -56);
    assert_eq!(unix_ct_init(&Fixed(-2), CtInitParams { ctn: 0, pn: 0 }), -2);
}

#[test]
fn wow64_widens_pointers_without_sign_extension() {
    // Not oracle-derived beyond row 12: ULongToPtr zero-extends, so 0xffff_ffff stays positive.
    let lib = FakeLib::default();
    let params = Wow64CtDataParams {
        ctn: 0,
        dad: u32::MAX,
        sad: u32::MAX,
        lenc: 0,
        command: u32::MAX,
        lenr: u32::MAX,
        response: u32::MAX,
    };
    assert_eq!(wow64_ct_data(&lib, params), 0);
    assert_eq!(lib.seen.get(), [0xffff_ffff; 5]);
}

#[test]
fn unix_table_slots_follow_enum_order() {
    // Not oracle-derived beyond row 13: each slot index is the C enum value and selects its table entry.
    let slots = [
        UnixFunc::Attach,
        UnixFunc::Detach,
        UnixFunc::CtInit,
        UnixFunc::CtData,
        UnixFunc::CtClose,
    ];
    let native = [
        Entry::Attach,
        Entry::Detach,
        Entry::CtInit,
        Entry::CtData,
        Entry::CtClose,
    ];
    let wow64 = [
        Entry::Attach,
        Entry::Detach,
        Entry::CtInit,
        Entry::Wow64CtData,
        Entry::CtClose,
    ];
    assert_eq!(slots.len(), UNIX_FUNCS_COUNT);
    for (index, slot) in slots.into_iter().enumerate() {
        assert_eq!(slot as usize, index);
        assert_eq!(entry(false, slot), native[index]);
        assert_eq!(entry(true, slot), wow64[index]);
    }
}

#[test]
fn derived_impls_are_exercised() {
    // Not oracle-derived: calls the derived Clone, Debug and PartialEq impls of each public type.
    let init = CtInitParams { ctn: 1, pn: 2 };
    assert_eq!(Clone::clone(&init), init);
    assert_eq!(format!("{init:?}"), "CtInitParams { ctn: 1, pn: 2 }");

    let data = CtDataParams {
        ctn: 1,
        dad: 2_u64,
        sad: 3,
        lenc: 4,
        command: 5,
        lenr: 6,
        response: 7,
    };
    assert_eq!(Clone::clone(&data), data);
    assert!(format!("{data:?}").contains("response: 7"));

    let close = CtCloseParams { ctn: 9 };
    assert_eq!(Clone::clone(&close), close);
    assert!(format!("{close:?}").contains("ctn: 9"));

    let wow64 = Wow64CtDataParams {
        ctn: 1,
        dad: 2,
        sad: 3,
        lenc: 4,
        command: 5,
        lenr: 6,
        response: 7,
    };
    assert_eq!(Clone::clone(&wow64), wow64);
    assert!(format!("{wow64:?}").contains("response: 7"));

    assert_eq!(Clone::clone(&UnixFunc::CtData), UnixFunc::CtData);
    assert_eq!(format!("{:?}", UnixFunc::CtData), "CtData");
    assert_eq!(Clone::clone(&Entry::Wow64CtData), Entry::Wow64CtData);
    assert_eq!(format!("{:?}", Entry::Wow64CtData), "Wow64CtData");
    assert_ne!(Entry::CtData, Entry::Wow64CtData);
}
