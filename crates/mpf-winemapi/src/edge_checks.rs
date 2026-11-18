//! Edge cases the oracle rows do not reach: the whole C white-space set, the
//! `smtp:` prefix boundaries, the recipient count rule, and bytes above ASCII.

use super::*;

fn with_count<'a>(recips: &'a [Recipient<'a>], count: u32) -> Message<'a> {
    Message {
        recip_count: count,
        recips: Some(recips),
        files: &[],
        subject: None,
        note_text: None,
    }
}

#[test]
fn c_space_set_is_the_c_locale_isspace_set() {
    let spaces: Vec<u8> = (0..=u8::MAX).filter(|&byte| is_c_space(byte)).collect();
    assert_eq!(spaces, [9, 10, 11, 12, 13, 32]);
}

#[test]
fn smtp_prefix_is_stripped_only_when_complete() {
    assert_eq!(strip_smtp_prefix(b"SMTP:"), b"");
    assert_eq!(strip_smtp_prefix(b"sMtP:x@y"), b"x@y");
    assert_eq!(strip_smtp_prefix(b"smtp"), b"smtp");
    assert_eq!(strip_smtp_prefix(b"smtq:x"), b"smtq:x");
    assert_eq!(strip_smtp_prefix(b""), b"");
}

#[test]
fn bytes_above_ascii_are_not_white_space_and_are_not_stripped() {
    assert!(is_non_empty(Some(b"\xa0\xe9")));
    assert!(!is_c_space(0xa0));
    assert_eq!(strip_smtp_prefix(b"\xffsmtp:x"), b"\xffsmtp:x");
    let mut argv = Vec::new();
    add_target(&mut argv, MAPI_TO, b"\xa0\xff@\xe9");
    assert_eq!(argv, [b"\xa0\xff@\xe9"]);
}

#[test]
fn xdg_argv_skips_missing_files_and_ignores_unknown_classes() {
    let recips = [
        Recipient {
            class: 9,
            address: Some(b"x@y"),
        },
        Recipient {
            class: 0,
            address: Some(b"o@x"),
        },
        Recipient {
            class: 2,
            address: Some(b"c@d"),
        },
    ];
    let message = Message {
        files: &[None, Some(b"a.txt")],
        ..with_count(&recips, 3)
    };
    let expected: [&[u8]; 5] = [b"xdg-email", b"--cc", b"c@d", b"--attach", b"a.txt"];
    assert_eq!(xdg_argv(&message), expected);
}

#[test]
fn recipients_follow_the_count_not_the_list() {
    let recips = [
        Recipient {
            class: 1,
            address: Some(b"a@b"),
        },
        Recipient {
            class: 1,
            address: Some(b"c@d"),
        },
    ];
    assert_eq!(with_count(&recips, 0).recipients().len(), 0);
    assert_eq!(with_count(&recips, 1).recipients().len(), 1);
    assert_eq!(with_count(&recips, 9).recipients().len(), 2);
    assert_eq!(
        send_mail_argv(Some(&with_count(&recips, 0))),
        Ok(vec![&b"xdg-email"[..]])
    );
}

#[test]
fn mailto_skips_missing_addresses_and_strips_smtp_from_cc() {
    let recips = [
        Recipient {
            class: 1,
            address: None,
        },
        Recipient {
            class: 2,
            address: Some(b"SMTP:c@d"),
        },
        Recipient {
            class: 1,
            address: Some(b"e@f"),
        },
    ];
    assert_eq!(
        mailto_url(&with_count(&recips, 3), b"", b""),
        b"mailto:\"e@f\"?subject=\"\"&cc=\"c@d\"&bcc=\"\"&body=\"\""
    );
}
