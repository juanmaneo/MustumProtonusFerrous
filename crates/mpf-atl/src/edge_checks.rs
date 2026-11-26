//! Edge-case tests for the registrar script engine.
//!
//! Expectations without a comment were run through the verbatim C bodies by the probe
//! harnesses in `target/canon/atl` (`probe` and `probe2`); they are not among the oracle
//! rows. Expectations marked "Rust only" have no C result, because the C loop never ends
//! on them.

use super::*;
use crate::oracle_checks::{oracle_registrar, run_script, wide, Mock};

const OK: u32 = 0;
const FAIL: u32 = 0x8000_4005;
const EXCEPTION: u32 = 0x8002_0009;
const ACCESS_DENIED: u32 = 0x8007_0005;

const CREATE_FOO: &str = r"create HKEY_CLASSES_ROOT\Foo ok";
const CLOSE_FOO: &str = r"close HKEY_CLASSES_ROOT\Foo";
const OPEN_FOO: &str = r"open HKEY_CLASSES_ROOT\Foo ok";
const DELETE_FOO: &str = r"delete_key HKEY_CLASSES_ROOT\Foo";
const SET_N: &str = r"set HKEY_CLASSES_ROOT\Foo N sz=4:x ok";

fn check(script: &str, register: bool, hresult: u32, log: &[&str]) {
    check_in(&oracle_registrar(), script, register, hresult, log);
}

fn check_in(registrar: &Registrar, script: &str, register: bool, hresult: u32, log: &[&str]) {
    let (actual, lines) = run_script(registrar, script, register);
    assert_eq!(actual, hresult, "hresult of {script:?}");
    assert_eq!(lines, log, "log of {script:?}");
}

#[test]
fn empty_and_root_only_scripts_are_ok() {
    for register in [true, false] {
        check("", register, OK, &[]);
        check("\u{20}\u{9}\u{d}\u{a}\u{20}", register, OK, &[]);
        check("HKCR", register, OK, &[]);
    }
    check("HKXX", true, OK, &[]);
}

#[test]
fn malformed_scripts_fail_before_any_registry_call() {
    for script in [
        "HKXX { }",
        "HKCR Foo",
        "HKCR {}",
        "'' { }",
        "HKCR{ }",
        "HKCR\u{a0} { }",
        "HKCR { } x y",
        "HKCR { } 'x",
        "HKCR 'x",
        "'abc",
        "HKCR { val",
        "HKCR { 'Foo }",
        "HKCR { NoRemove 'x",
    ] {
        check(script, true, EXCEPTION, &[]);
    }
}

#[test]
fn words_after_a_root_block() {
    // A word that ends the script is ignored; one followed by more input is an error.
    check("HKCR { } x", true, OK, &[]);
    check("HKCR { } ''", true, OK, &[]);
    check("HKCR { } }", true, OK, &[]);
    check("HKCR { } HKCU", true, OK, &[]);
    check("HKCR { Foo { } } }", true, OK, &[CREATE_FOO, CLOSE_FOO]);
    check(
        "HKCR { } HKCU { Bar { } }",
        true,
        OK,
        &[
            r"create HKEY_CURRENT_USER\Bar ok",
            r"close HKEY_CURRENT_USER\Bar",
        ],
    );
}

#[test]
fn every_root_name_is_accepted() {
    for name in [
        "HKEY_CLASSES_ROOT",
        "HKEY_CURRENT_USER",
        "HKEY_LOCAL_MACHINE",
        "HKEY_USERS",
        "HKEY_PERFORMANCE_DATA",
        "HKEY_DYN_DATA",
        "HKEY_CURRENT_CONFIG",
        "HKCR",
        "HKCU",
        "HKLM",
        "HKU",
        "HKPD",
        "HKDD",
        "HKCC",
    ] {
        check(&format!("{name} {{ }}"), true, OK, &[]);
    }
}

#[test]
fn names_fold_case_and_accept_quotes_and_separators() {
    check(
        "hkcr { noremove foo { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\foo ok",
            r"close HKEY_CLASSES_ROOT\foo",
        ],
    );
    check(
        "HKCR { '' { } }",
        true,
        OK,
        &[r"create HKEY_CLASSES_ROOT ok", r"close HKEY_CLASSES_ROOT"],
    );
    check(
        "HKCR { '' { } }",
        false,
        OK,
        &[
            r"open HKEY_CLASSES_ROOT ok",
            r"delete_key HKEY_CLASSES_ROOT",
            r"close HKEY_CLASSES_ROOT",
        ],
    );
    check(
        "HKCR { Caf\u{e9} { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\Caf? ok",
            r"close HKEY_CLASSES_ROOT\Caf?",
        ],
    );
    check(
        "HKCR { \u{661} { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\? ok",
            r"close HKEY_CLASSES_ROOT\?",
        ],
    );
    check(
        "HKCR { Foo\u{b}{\u{c}} }",
        true,
        OK,
        &[CREATE_FOO, CLOSE_FOO],
    );
    check(
        "HKCR\u{9}{\u{d}\u{a}Foo\u{b}{\u{c}}\u{d}\u{a}}",
        true,
        OK,
        &[CREATE_FOO, CLOSE_FOO],
    );
}

#[test]
fn unregister_keeps_noremove_and_deletes_the_rest() {
    check(
        "HKCR { NoRemove Foo { } }",
        false,
        OK,
        &[OPEN_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo = s - 'x' }",
        false,
        OK,
        &[OPEN_FOO, DELETE_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo = s 'x' }",
        false,
        OK,
        &[OPEN_FOO, DELETE_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { ForceRemove Denied { } }",
        false,
        OK,
        &[
            r"open HKEY_CLASSES_ROOT\Denied ok",
            r"delete_key HKEY_CLASSES_ROOT\Denied",
            r"close HKEY_CLASSES_ROOT\Denied",
        ],
    );
}

#[test]
fn denied_create_runs_the_unregister_retry() {
    check(
        "HKCR { ForceRemove Denied { } }",
        true,
        ACCESS_DENIED,
        &[
            r"delete_tree HKEY_CLASSES_ROOT\Denied",
            r"create HKEY_CLASSES_ROOT\Denied err 5",
            r"open HKEY_CLASSES_ROOT\Denied ok",
            r"delete_key HKEY_CLASSES_ROOT\Denied",
            r"close HKEY_CLASSES_ROOT\Denied",
        ],
    );
    check(
        "HKCR { Denied { } }",
        true,
        ACCESS_DENIED,
        &[
            r"create HKEY_CLASSES_ROOT\Denied err 5",
            r"open HKEY_CLASSES_ROOT\Denied ok",
            r"delete_key HKEY_CLASSES_ROOT\Denied",
            r"close HKEY_CLASSES_ROOT\Denied",
        ],
    );
}

#[test]
fn unterminated_quotes_fail_and_keep_the_partial_log() {
    check(
        "HKCR { Foo 'x",
        true,
        EXCEPTION,
        &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, DELETE_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo 'x",
        false,
        EXCEPTION,
        &[OPEN_FOO, DELETE_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo = 'x",
        true,
        EXCEPTION,
        &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, CLOSE_FOO],
    );
    check("HKCR { Foo = 'x", false, EXCEPTION, &[OPEN_FOO, CLOSE_FOO]);
    check(
        "HKCR { Foo = s 'x }",
        true,
        EXCEPTION,
        &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo = s 'x }",
        false,
        EXCEPTION,
        &[OPEN_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo { val N = s 'x } }",
        true,
        EXCEPTION,
        &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo { val",
        true,
        EXCEPTION,
        &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, CLOSE_FOO],
    );
    for word in ["d 'x", "b 'zz"] {
        let script = format!("HKCR {{ Foo {{ val N = {word} }} }}");
        check(
            &script,
            true,
            EXCEPTION,
            &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, CLOSE_FOO],
        );
        check(&script, false, EXCEPTION, &[OPEN_FOO, CLOSE_FOO]);
    }
}

#[test]
fn value_keywords_types_and_names() {
    check(
        "HKCR { Foo { val N = sz x } }",
        true,
        EXCEPTION,
        &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, CLOSE_FOO],
    );
    check(
        "HKCR { Foo { val N = sz x } }",
        false,
        EXCEPTION,
        &[OPEN_FOO, CLOSE_FOO],
    );
    // An unknown type letter fails in the register pass only, since the unregister
    // pass skips the value word.
    for word in ["S x", "q 1"] {
        check(
            &format!("HKCR {{ Foo {{ val N = {word} }} }}"),
            true,
            EXCEPTION,
            &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, DELETE_FOO, CLOSE_FOO],
        );
        check(
            &format!("HKCR {{ Foo {{ val N = {word} }} }}"),
            false,
            OK,
            &[OPEN_FOO, DELETE_FOO, CLOSE_FOO],
        );
    }
    check(
        "HKCR { Foo { val '' = s 'x' } }",
        true,
        OK,
        &[
            CREATE_FOO,
            r"set HKEY_CLASSES_ROOT\Foo @ sz=4:x ok",
            CLOSE_FOO,
        ],
    );
    check(
        "HKCR { Foo { 'val' N = s 'x' } }",
        true,
        OK,
        &[CREATE_FOO, SET_N, CLOSE_FOO],
    );
    check(
        "HKCR { Foo { VAL N = s 'x' } }",
        true,
        OK,
        &[CREATE_FOO, SET_N, CLOSE_FOO],
    );
    check(
        "HKCR { Foo { val N = b '' } }",
        true,
        OK,
        &[
            CREATE_FOO,
            r"set HKEY_CLASSES_ROOT\Foo N bin=0: ok",
            CLOSE_FOO,
        ],
    );
    check(
        "HKCR { Foo { val N = b AbCd } }",
        true,
        OK,
        &[
            CREATE_FOO,
            r"set HKEY_CLASSES_ROOT\Foo N bin=2:abcd ok",
            CLOSE_FOO,
        ],
    );
    check(
        "HKCR { Foo { val N = b zz } }",
        false,
        OK,
        &[OPEN_FOO, DELETE_FOO, CLOSE_FOO],
    );
}

#[test]
fn binary_values_reject_odd_length_and_non_hex_digits() {
    for word in ["zz", "0", "0G", "1\u{661}"] {
        check(
            &format!("HKCR {{ Foo {{ val N = b {word} }} }}"),
            true,
            FAIL,
            &[CREATE_FOO, CLOSE_FOO, OPEN_FOO, DELETE_FOO, CLOSE_FOO],
        );
    }
}

#[test]
fn dword_values_follow_wcstoul() {
    let cases: [(&str, u32); 16] = [
        ("+5", 5),
        ("' 12'", 12),
        ("12x", 12),
        ("abc", 0),
        ("\u{661}\u{662}", 12),
        ("\u{ff12}", 2),
        ("-4294967296", 1),
        ("-4294967295", 1),
        ("-2147483648", 0x8000_0000),
        ("4294967295", u32::MAX),
        ("99999999999999999999999", u32::MAX),
        ("-99999999999999999999999", 1),
        ("-", 0),
        ("+", 0),
        ("--5", 0),
        ("''", 0),
    ];
    for (word, value) in cases {
        let set = format!(r"set HKEY_CLASSES_ROOT\Foo N dword=0x{value:08x} ok");
        check(
            &format!("HKCR {{ Foo {{ val N = d {word} }} }}"),
            true,
            OK,
            &[CREATE_FOO, set.as_str(), CLOSE_FOO],
        );
    }
}

#[test]
fn percent_macros_and_escapes() {
    check(
        "HKCR { %module% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\app.dll ok",
            r"close HKEY_CLASSES_ROOT\app.dll",
        ],
    );
    check(
        "HKCR { %% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\% ok",
            r"close HKEY_CLASSES_ROOT\%",
        ],
    );
    check(
        "HKCR { %%%% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\%% ok",
            r"close HKEY_CLASSES_ROOT\%%",
        ],
    );
    check(
        "HKCR { %NAME% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\Widget ok",
            r"close HKEY_CLASSES_ROOT\Widget",
        ],
    );
    check(
        "HKCR { %%NAME%% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\%NAME% ok",
            r"close HKEY_CLASSES_ROOT\%NAME%",
        ],
    );
    check("HKCR { %MODULEX% { } }", true, EXCEPTION, &[]);
    check("HKCR { %MODULE { } }", true, EXCEPTION, &[]);
    check("HKCR { %Nope% { } }", true, EXCEPTION, &[]);
    check("HKCR %", true, EXCEPTION, &[]);
}

#[test]
fn clear_replacements_removes_the_macros() {
    let mut registrar = oracle_registrar();
    registrar.clear_replacements();
    check_in(&registrar, "HKCR { %MODULE% { } }", true, EXCEPTION, &[]);
}

#[test]
fn replacement_keys_and_items_stop_at_nul() {
    let mut registrar = Registrar::default();
    registrar.add_replacement(&wide("MOD\0X"), &wide("a\0b"));
    check_in(
        &registrar,
        "HKCR { %MOD% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\a ok",
            r"close HKEY_CLASSES_ROOT\a",
        ],
    );
    check_in(&registrar, "HKCR { %MOD\0X% { } }", true, EXCEPTION, &[]);
}

#[test]
fn replacement_items_are_not_expanded_again() {
    let mut registrar = Registrar::default();
    registrar.add_replacement(&wide("A"), &wide("%B%"));
    registrar.add_replacement(&wide("B"), &wide("x"));
    check_in(
        &registrar,
        "HKCR { %A% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\%B% ok",
            r"close HKEY_CLASSES_ROOT\%B%",
        ],
    );
    check_in(
        &registrar,
        "HKCR { %B% { } }",
        true,
        OK,
        &[
            r"create HKEY_CLASSES_ROOT\x ok",
            r"close HKEY_CLASSES_ROOT\x",
        ],
    );
}

#[test]
fn script_is_cut_at_the_first_nul() {
    check("HKCR { }\0 HKXX", true, OK, &[]);
    check("HKCR { }\0 %MODULE%", true, OK, &[]);
    check("HKCR { Foo }\0garbage", true, OK, &[CREATE_FOO, CLOSE_FOO]);
}

#[test]
fn unclosed_blocks_are_exceptions() {
    // Rust only: at the end of input the C loop keeps creating keys from empty words.
    // In the oracle that overflows its log buffer, so there is no C result to compare.
    for script in ["HKCR {", "HKCR { Foo", "HKCR { Foo { }"] {
        let (hresult, _) = run_script(&oracle_registrar(), script, true);
        assert_eq!(hresult, EXCEPTION, "hresult of {script:?}");
    }
}

#[test]
fn error_codes_map_to_hresults() {
    assert_eq!(Error::Exception.hresult(), EXCEPTION);
    assert_eq!(Error::Fail.hresult(), FAIL);
    assert_eq!(Error::Win32(5).hresult(), ACCESS_DENIED);
    // Not oracle-derived: HRESULT_FROM_WIN32 passes a non-positive code through unchanged.
    assert_eq!(Error::Win32(0x8000_0000).hresult(), 0x8000_0000);
}

#[test]
fn mock_fails_a_null_parent_and_a_null_value_key() {
    // Not registrar expectations: the register pass never reaches these calls with a NULL
    // handle. They keep the mock's error branches covered.
    let mut reg = Mock::new();
    assert_eq!(reg.create_key(None, &wide("Foo")).err(), Some(6));
    assert_eq!(reg.set_value(None, None, &Value::Dword(1)).err(), Some(6));
    assert_eq!(
        reg.log,
        [
            r"create <null>\Foo err 6",
            r"set <null> @ dword=0x00000001 err 6",
        ]
    );
}
