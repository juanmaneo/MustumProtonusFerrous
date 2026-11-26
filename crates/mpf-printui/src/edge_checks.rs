//! Boundary and branch tests for the ported printui logic.
//!
//! The cases marked "ad-hoc C run" were run through the verbatim C in `target/canon/printui/edge.c`
//! with `gcc -fsanitize=address,undefined`, outside the printed oracle rows. The cases marked
//! "not oracle-derived" come from reading the C, because the harness cannot pass them, or because
//! the C is undefined there.

use super::oracle_checks::{splitter, wide};
use super::*;
use std::cell::Cell;

/// Parses `args` as the argument vector of `parse_rundll`.
fn parse(args: &[&str]) -> RunDll {
    let argv: Vec<Vec<u16>> = args.iter().copied().map(wide).collect();
    parse_rundll(&argv)
}

#[test]
fn edge_subcommand_letters_are_taken_after_their_command() {
    // Ad-hoc C runs: `/gA`, `/iB` and `/d/`.
    let cases: [(&[&str], u8, u8); 3] = [
        (&["/gA"][..], b'g', b'A'),
        (&["/iB"][..], b'i', b'B'),
        (&["/d/"][..], b'd', b'/'),
    ];
    for (args, command, subcommand) in cases {
        let cx = parse(args);
        assert!(cx.ok, "{args:?}");
        assert_eq!(
            (cx.command, cx.subcommand),
            (u16::from(command), u16::from(subcommand)),
            "{args:?}"
        );
    }

    // Ad-hoc C run `/X` `arg`: the extra parameter starts at `arg`.
    let cx = parse(&["/X", "arg"]);
    assert_eq!((cx.command, cx.subcommand), (u16::from(b'X'), 0));
    assert_eq!((cx.next_arg, cx.pending), (2, Some(wide("arg"))));
}

#[test]
fn edge_subcommand_takes_the_next_character_even_after_a_flag() {
    // Ad-hoc C run `/Sqx`: `q` becomes the subcommand of `S`, so no flag is set, and `x` replaces
    // `S` as the command.
    let cx = parse(&["/Sqx"]);
    assert!(cx.ok);
    assert_eq!((cx.command, cx.subcommand), (u16::from(b'x'), 0));
    assert_eq!(cx.flags, [false; FLAG_MAX]);
    assert_eq!((cx.next_arg, cx.pending), (1, None));
}

#[test]
fn edge_subcommand_does_not_read_the_next_argument() {
    // Ad-hoc C run `/d` `/?`: `get_next_wchar(FALSE)` stops at the end of `/d`, so `d` gets no
    // subcommand and `/?` is parsed as the next argument.
    let cx = parse(&["/d", "/?"]);
    assert!(cx.ok);
    assert_eq!(
        (cx.command, cx.subcommand, cx.next_arg),
        (u16::from(b'?'), 0, 2)
    );
    assert_eq!(cx.pending, Some(wide("")));
}

#[test]
fn edge_slash_inside_an_argument_is_a_command_letter() {
    // Ad-hoc C runs `/q/?` and `/q/x`: after the flag `q`, the second `/` is read as a command
    // letter, and the letter after it replaces it.
    let cx = parse(&["/q/?"]);
    assert!(cx.ok);
    assert_eq!(cx.command, u16::from(b'?'));
    assert_eq!(cx.flags, [true, false, false, false, false]);
    assert_eq!(cx.pending, Some(wide("")));

    let cx = parse(&["/q/x"]);
    assert!(cx.ok);
    assert_eq!(cx.command, u16::from(b'x'));
    assert_eq!(cx.flags, [true, false, false, false, false]);
    assert_eq!((cx.next_arg, cx.pending), (1, None));
}

#[test]
fn edge_option_without_a_value_clears_an_earlier_value() {
    // Ad-hoc C run `/nA` `/n`: the second `/n` has no value, and its `None` overwrites `A`.
    let cx = parse(&["/nA", "/n"]);
    assert!(cx.ok);
    assert!(cx.options.iter().all(Option::is_none));
    assert_eq!((cx.next_arg, cx.pending), (2, None));
}

#[test]
fn edge_at_redirect_consumes_its_argument_and_fails() {
    // Ad-hoc C run `@x`: the file name is consumed, not read, and the parse fails.
    let cx = parse(&["@x"]);
    assert!(!cx.ok);
    assert_eq!((cx.next_arg, cx.pending), (1, None));
}

#[test]
fn edge_whitespace_only_argument_is_an_unknown_character() {
    // Ad-hoc C run `" "`: the whitespace loop reaches the NUL, which is neither `@` nor `/`, so the
    // parse reports an unknown character.
    let cx = parse(&[" "]);
    assert!(!cx.ok);
    assert_eq!((cx.command, cx.next_arg, cx.pending), (0, 1, None));
}

#[test]
fn edge_argument_text_stops_at_its_first_nul() {
    // Not oracle-derived: the harness cannot pass an embedded NUL in an argument. The value of
    // `/c` ends at the NUL.
    let cx = parse(&["/cab\0cd"]);
    assert!(cx.ok);
    assert_eq!(cx.options[2], Some(wide("ab")));
    assert_eq!((cx.next_arg, cx.pending), (1, None));
}

#[test]
fn edge_extra_parameter_after_whitespace_to_the_end() {
    // Not oracle-derived: `/S` then a whitespace-only argument makes the C decrement a NULL
    // `pNextCharW`, which is undefined behaviour (UBSan reports it at printui.c:201). The port keeps
    // the cursor NULL and still returns TRUE.
    let cx = parse(&["/S", " "]);
    assert!(cx.ok);
    assert_eq!(
        (cx.command, cx.subcommand, cx.next_arg, cx.pending),
        (u16::from(b'S'), 0, 2, None)
    );
}

#[test]
fn edge_print_ui_entry_w_does_not_split_an_empty_command_line() {
    // Not oracle-derived: the harness always passes a fixed `pCommand`. A NUL first character
    // fails the `pCommand[0]` check the same way an empty string does (row 23).
    let calls = Cell::new(0);
    let line = [0, u16::from(b'?')];
    assert_eq!(
        print_ui_entry_w(Some(&line[..]), splitter(&[], &calls)),
        Dispatch::Failed
    );
    assert_eq!(calls.get(), 0);
}

#[test]
fn edge_print_ui_entry_w_cuts_the_command_line_at_its_first_nul() {
    // Not oracle-derived: `CommandLineToArgvW` reads the command line up to its NUL.
    let mut seen = Vec::new();
    let line = [u16::from(b'/'), u16::from(b'?'), 0, u16::from(b'q')];
    let dispatch = print_ui_entry_w(Some(&line[..]), |text| {
        seen = text.to_vec();
        vec![wide("/?")]
    });
    assert_eq!(seen, wide("/?"));
    assert_eq!(
        dispatch,
        Dispatch::NotImplemented {
            command: u16::from(b'?'),
            subcommand: 0,
        }
    );
}

#[test]
fn edge_export_table_kinds() {
    assert_eq!(EXPORTS.len(), 23);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        21
    );
    assert_eq!(export_kind("PrintUIEntryW"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Private));
    assert_eq!(export_kind("bPrinterSetup"), Some(ExportKind::Stub));
    assert_eq!(export_kind("printuientryw"), None);
    assert_eq!(export_kind(""), None);
}
