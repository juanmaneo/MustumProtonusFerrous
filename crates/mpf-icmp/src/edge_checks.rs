//! Edge tests for the icmp port: the argument table, the 16-bit-only rule, the i386 stack sizes,
//! the forward-line grammar and the export table.
//!
//! Expectations that are not oracle-derived say so in a comment. They come from reading the
//! reference source, or from running the reference winebuild by hand on one-line specs. They
//! are not among the rows in oracle_checks.

use super::*;
use crate::args::from_name;

#[test]
fn arg_names_match_the_winebuild_table() {
    // Not oracle-derived: the ArgNames table in tools/winebuild/parser.c.
    assert_eq!(from_name("word"), Some(ArgType::Word));
    assert_eq!(from_name("s_word"), Some(ArgType::SWord));
    assert_eq!(from_name("segptr"), Some(ArgType::SegPtr));
    assert_eq!(from_name("segstr"), Some(ArgType::SegStr));
    assert_eq!(from_name("long"), Some(ArgType::Long));
    assert_eq!(from_name("ptr"), Some(ArgType::Ptr));
    assert_eq!(from_name("str"), Some(ArgType::Str));
    assert_eq!(from_name("wstr"), Some(ArgType::WStr));
    assert_eq!(from_name("int64"), Some(ArgType::Int64));
    assert_eq!(from_name("int128"), Some(ArgType::Int128));
    assert_eq!(from_name("float"), Some(ArgType::Float));
    assert_eq!(from_name("double"), Some(ArgType::Double));
}

#[test]
fn unknown_arg_names_are_rejected_case_sensitively() {
    // Not oracle-derived: winebuild compares names with strcmp. "bogus" and "PTR" fail with
    // "Unknown argument type" when run on a one-line spec.
    assert_eq!(from_name("bogus"), None);
    assert_eq!(from_name("PTR"), None);
    assert_eq!(from_name(""), None);
}

#[test]
fn win16_only_types_are_rejected_in_win32_specs() {
    // Not oracle-derived: word, s_word, segptr and segstr each fail with "only allowed for
    // Win16" when the reference runs on a one-line 32-bit spec.
    for name in ["word", "s_word", "segptr", "segstr"] {
        assert_eq!(parse_win32_arg(name), None, "{name}");
    }
    assert_eq!(parse_win32_arg("long"), Some(ArgType::Long));
    assert_eq!(parse_win32_arg("bogus"), None);
}

#[test]
fn i386_stack_bytes_by_type() {
    // Not oracle-derived for the 16-bit-only types, which a 32-bit spec cannot declare. The
    // 4, 8 and 16 byte sizes of the other types are also checked by oracle rows 17-24.
    use ArgType::*;
    for arg in [Word, SWord, SegPtr, SegStr, Long, Ptr, Str, WStr, Float] {
        assert_eq!(stack_bytes_i386(&[arg]), 4, "{arg:?}");
    }
    assert_eq!(stack_bytes_i386(&[Int64]), 8);
    assert_eq!(stack_bytes_i386(&[Double]), 8);
    assert_eq!(stack_bytes_i386(&[Int128]), 16);
    assert_eq!(stack_bytes_i386(&[]), 0);
    assert_eq!(stack_bytes_i386(&[Ptr, Int64, Int128]), 28);
}

#[test]
fn forward_lines_with_spacing_parse_as_winebuild_does() {
    // Checked by hand: the reference --def output for each of these one-line specs gave the
    // same name, argument count and forward target.
    let parsed = |line| parse_stdcall_forward(line, 3).expect("accepted by winebuild");
    assert_eq!(
        parsed("   @ stdcall W (ptr) x.y"),
        Export {
            name: "W",
            ordinal: 3,
            args: vec![ArgType::Ptr],
            forward: "x.y"
        }
    );
    assert_eq!(
        parsed("@\tstdcall\tW(ptr)\tx.y"),
        Export {
            name: "W",
            ordinal: 3,
            args: vec![ArgType::Ptr],
            forward: "x.y"
        }
    );
    assert_eq!(
        parsed("@ stdcall  W( ptr ) x.y"),
        Export {
            name: "W",
            ordinal: 3,
            args: vec![ArgType::Ptr],
            forward: "x.y"
        }
    );
    assert_eq!(
        parsed("@ stdcall W(ptr)x.y"),
        Export {
            name: "W",
            ordinal: 3,
            args: vec![ArgType::Ptr],
            forward: "x.y"
        }
    );
    assert_eq!(
        parsed("@ stdcall W(ptr ptr) x.y"),
        Export {
            name: "W",
            ordinal: 3,
            args: vec![ArgType::Ptr, ArgType::Ptr],
            forward: "x.y"
        }
    );
    assert_eq!(
        parsed("@ stdcall W() x.y"),
        Export {
            name: "W",
            ordinal: 3,
            args: vec![],
            forward: "x.y"
        }
    );
    assert_eq!(parsed("@ stdcall W(ptr) x.y.z").forward, "x.y.z");
    assert_eq!(parsed("@ stdcall W(ptr) .y").forward, ".y");
    assert_eq!(parsed("@ stdcall W(ptr) x.").forward, "x.");
}

#[test]
fn malformed_lines_are_rejected() {
    // Checked by hand: the reference winebuild --def exits with status 1 on each of these
    // one-line specs.
    let rejected = [
        "stdcall W(ptr) x.y",
        "@stdcall W(ptr) x.y",
        "@",
        "@ stdcalls W(ptr) x.y",
        "@ stdcall (ptr) x.y",
        "@ stdcall W X(ptr) x.y",
        "@ stdcall Wptr x.y",
        "@ stdcall W(ptr x.y",
        "@ stdcall W(ptr) x.y extra",
        "@ stdcall W(ptr) x.y z",
        "@ stdcall W(bogus) x.y",
        "@ stdcall W(word) x.y",
        "@ stdcall W(ptr, long) x.y",
    ];
    for line in rejected {
        assert_eq!(parse_stdcall_forward(line, 1), None, "{line}");
    }
}

#[test]
fn lines_outside_the_forward_port_are_skipped() {
    // Not oracle-derived: the reference accepts these as code entries or cdecl entries, not as
    // forwards. The port returns None, so it invents no export for them.
    assert_eq!(
        parse_stdcall_forward("@ stdcall W(ptr) plaintarget", 1),
        None
    );
    assert_eq!(parse_stdcall_forward("@ stdcall W(ptr)", 1), None);
    assert_eq!(parse_stdcall_forward("@ cdecl W(ptr) x.y", 1), None);
}

#[test]
fn export_table_matches_icmp_spec() {
    // Not oracle-derived: the names, ordinals, argument counts and forwards read from
    // dlls/icmp/icmp.spec. The oracle rows check the same values through def_line.
    let table: Vec<(&str, u16, usize)> = exports()
        .iter()
        .map(|e| (e.name, e.ordinal, e.args.len()))
        .collect();
    assert_eq!(
        table,
        [
            ("IcmpCloseHandle", 1, 1),
            ("IcmpCreateFile", 2, 0),
            ("IcmpParseReplies", 3, 2),
            ("IcmpSendEcho2", 4, 11),
            ("IcmpSendEcho", 5, 8),
            ("do_echo_rep", 6, 0),
            ("do_echo_req", 7, 0),
            ("register_icmp", 8, 0),
        ]
    );
    assert!(exports()
        .iter()
        .all(|e| e.forward == format!("iphlpapi.{}", e.name)));
}
