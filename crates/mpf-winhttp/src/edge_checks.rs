//! Edge cases the oracle rows do not reach: the boundaries of each rule, the inputs where Wine
//! is undefined (X01 to X03 in `oracle_checks.rs`), and every derived trait impl. Expectations
//! marked `oracle` were also observed from the ASan harness.

use super::*;
use crate::crack::{crack_url, CrackedUrl, Piece, Slot, Slots};
use crate::create::{create_url, CreateError, UrlParts};
use crate::escape::{decode_url, escape_string, escape_url};

const ESCAPE: u32 = ICU_ESCAPE;
const DECODE: u32 = ICU_DECODE;

const PUNCT: &str = " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~\u{7f}\u{1f}";

/// Scheme, host, path and extra are buffers, and user and password are not requested, so the
/// NULL pointers of a URL without userinfo are never copied.
const BUFFERS: [Slot; 6] = [
    Slot::Buffer(8),
    Slot::Unrequested,
    Slot::Unrequested,
    Slot::Buffer(8),
    Slot::Buffer(64),
    Slot::Buffer(16),
];

const VIEWS: [Slot; 6] = [Slot::View; 6];

fn crack_str(url: &str, flags: u32, slots: [Slot; 6]) -> Result<CrackedUrl, u32> {
    let [scheme, user, password, host, path, extra] = slots;
    let slots = Slots {
        scheme,
        user,
        password,
        host,
        path,
        extra,
    };
    crack_url(&wide(url), flags, &slots)
}

fn text(value: &str) -> Piece {
    Piece::Text(wide(value))
}

fn host_parts(host: Option<&[u16]>) -> UrlParts<'_> {
    UrlParts {
        scheme: None,
        n_scheme: 0,
        user: None,
        password: None,
        host,
        port: 80,
        path: None,
        extra: None,
    }
}

#[test]
fn need_escape_covers_controls_delete_and_unsafe_punctuation() {
    for (ch, escaped) in [
        ('\u{1f}', "%1F"),
        (' ', "%20"),
        ('"', "%22"),
        ('#', "%23"),
        ('%', "%25"),
        ('<', "%3C"),
        ('>', "%3E"),
        ('[', "%5B"),
        ('\\', "%5C"),
        (']', "%5D"),
        ('^', "%5E"),
        ('`', "%60"),
        ('{', "%7B"),
        ('|', "%7C"),
        ('}', "%7D"),
        ('~', "%7E"),
        ('\u{7f}', "%7F"),
    ] {
        assert_eq!(
            escape_string(&wide(&ch.to_string())),
            (wide(escaped), true),
            "{ch:?}"
        );
    }
}

#[test]
fn need_escape_leaves_other_printable_ascii_alone() {
    let plain = "!$&'()*+,-./09:;=?@AZ_az";
    assert_eq!(escape_string(&wide(plain)), (wide(plain), true));
}

#[test]
fn escape_string_flags_units_above_0xff_but_keeps_the_length() {
    assert_eq!(escape_string(&[0x100]), (wide("%00"), false));
    assert_eq!(escape_string(&[0x130]), (wide("%30"), false));
    assert_eq!(escape_string(&[0x41, 0x100]), (wide("A%00"), false));
    assert_eq!(escape_string(&[0xff]), (wide("%FF"), true));
    assert_eq!(escape_string(&[]), (Vec::new(), true));
}

#[test]
fn escape_url_encodes_from_the_last_slash() {
    assert_eq!(escape_url(&wide("a b")), Ok(wide("a b")));
    assert_eq!(escape_url(&wide("a b/c d")), Ok(wide("a b/c%20d")));
    assert_eq!(
        escape_url(&wide("http://h/a b?c d")),
        Ok(wide("http://h/a%20b?c%20d"))
    );
    assert_eq!(escape_url(&wide("a\u{100}")), Ok(wide("a\u{100}")));
    assert_eq!(escape_url(&wide("/\u{100}")), Err(ERROR_INVALID_PARAMETER));
    assert_eq!(
        escape_url(&wide("http://h\0/x y")),
        Ok(wide("http://h%00/x%20y"))
    );
    assert_eq!(escape_url(&wide("a\0/b c")), Ok(wide("a\0/b c")));
}

#[test]
fn decode_url_replaces_only_complete_escapes() {
    assert_eq!(decode_url(&wide("%41")), wide("A"));
    assert_eq!(decode_url(&wide("%ab")), wide("\u{ab}"));
    assert_eq!(decode_url(&wide("%7e")), wide("~"));
    assert_eq!(decode_url(&wide("%FC")), wide("\u{fc}"));
    assert_eq!(decode_url(&wide("%00")), vec![0]);
    assert_eq!(decode_url(&wide("%zz")), wide("%zz"));
    assert_eq!(decode_url(&wide("%4")), wide("%4"));
    assert_eq!(decode_url(&wide("%4z")), wide("%4z"));
    assert_eq!(decode_url(&wide("%G1")), wide("%G1"));
    assert_eq!(decode_url(&wide("%1G")), wide("%1G"));
    assert_eq!(decode_url(&wide("%%41")), wide("%A"));
    assert_eq!(decode_url(&wide("%")), wide("%"));
    assert_eq!(decode_url(&wide("\u{130}")), wide("\u{130}"));
    assert_eq!(decode_url(&[]), Vec::<u16>::new());
    assert_eq!(
        decode_url(&wide("%41%zz%4%4z%G1%1G%ab%00x\u{130}")),
        wide("A%zz%4%4z%G1%1G\u{ab}\0x\u{130}")
    );
}

#[test]
fn crack_rejects_a_missing_or_unknown_scheme() {
    for url in [
        "h",
        "",
        "htt:",
        "ftps://h/",
        "h\u{168}tp://h/",
        "http\0://h",
    ] {
        assert_eq!(
            crack_str(url, 0, VIEWS),
            Err(ERROR_WINHTTP_UNRECOGNIZED_SCHEME),
            "{url:?}"
        );
    }
}

#[test]
fn crack_rejects_a_bad_separator_or_empty_authority() {
    for url in ["http:", "http:/", "http://", "http:x y", "http:\u{4e2d}"] {
        assert_eq!(
            crack_str(url, 0, VIEWS),
            Err(ERROR_WINHTTP_INVALID_URL),
            "{url:?}"
        );
    }
    assert_eq!(
        crack_str("http:x y", ESCAPE, BUFFERS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
    assert_eq!(
        crack_str("http:\u{4e2d}", ESCAPE, BUFFERS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
}

#[test]
fn escape_flag_encodes_the_host_path_and_extra() {
    let host = crack_str("http://h x", ESCAPE, BUFFERS).unwrap();
    assert_eq!((host.host, host.path), (text("h%20x"), text("")));

    let cracked = crack_str("http://h/a b?x y", ESCAPE, BUFFERS).unwrap();
    assert_eq!(
        (cracked.path, cracked.extra),
        (text("/a%20b"), text("?x%20y"))
    );

    assert_eq!(
        crack_str("http://h?x", ESCAPE, BUFFERS).unwrap().host,
        text("h?x")
    );
    assert_eq!(
        crack_str("http://\u{4e2d}/x", ESCAPE, BUFFERS)
            .unwrap()
            .host,
        text("\u{4e2d}")
    );
    assert_eq!(
        crack_str("http://h/\u{4e2d}", ESCAPE, BUFFERS),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h/a b\u{100}", ESCAPE, BUFFERS),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h/%41", ESCAPE | DECODE, BUFFERS)
            .unwrap()
            .path,
        text("/%2541")
    );
    assert_eq!(
        crack_str("http://h/", ESCAPE, VIEWS),
        Err(ERROR_INVALID_PARAMETER)
    );
}

#[test]
fn escape_flag_gives_an_empty_view_host_when_the_host_is_empty() {
    let slots = [
        Slot::Buffer(8),
        Slot::Unrequested,
        Slot::Unrequested,
        Slot::View,
        Slot::Buffer(8),
        Slot::Unrequested,
    ];
    let cracked = crack_str("http:///x", ESCAPE, slots).unwrap();
    assert_eq!(
        (cracked.host, cracked.path, cracked.extra),
        (text(""), text("/x"), Piece::Absent)
    );
}

#[test]
fn decode_flag_decodes_before_cracking() {
    assert_eq!(
        crack_str("http://h/%41", DECODE, BUFFERS).unwrap().path,
        text("/A")
    );
    assert_eq!(
        crack_str("http://h/%4", DECODE, BUFFERS).unwrap().path,
        text("/%4")
    );
    assert_eq!(
        crack_str("http://h/%41%zz%4%4z%G1%1G%ab%00x\u{130}", DECODE, BUFFERS)
            .unwrap()
            .path,
        text("/A%zz%4%4z%G1%1G\u{ab}\0x\u{130}")
    );
    assert_eq!(
        crack_str("http://h/a", DECODE, VIEWS),
        Err(ERROR_INVALID_PARAMETER)
    );
}

#[test]
fn userinfo_splits_at_the_first_at_and_the_first_colon() {
    let c = crack_str("http://a@b@c/", 0, VIEWS).unwrap();
    assert_eq!(
        (c.user, c.password, c.host, c.path),
        (text("a"), Piece::Absent, text("b@c"), text("/"))
    );

    let c = crack_str("http://a/b@c", 0, VIEWS).unwrap();
    assert_eq!(
        (c.user, c.host, c.path),
        (Piece::Absent, text("a"), text("/b@c"))
    );

    let c = crack_str("http://:p@h/", 0, VIEWS).unwrap();
    assert_eq!((c.user, c.password), (text(""), text("p")));

    let c = crack_str("http://u:p:q@h/", 0, VIEWS).unwrap();
    assert_eq!((c.user, c.password), (text("u"), text("p:q")));

    let c = crack_str("http://u:p@q@h/", 0, VIEWS).unwrap();
    assert_eq!(
        (c.user, c.password, c.host, c.path),
        (text("u"), text("p"), text("q@h"), text("/"))
    );

    let c = crack_str("http://u@h", 0, VIEWS).unwrap();
    assert_eq!(
        (c.user, c.password, c.host, c.path, c.extra),
        (text("u"), Piece::Absent, text("h"), text(""), text(""))
    );

    let c = crack_str("http://u:p@h", 0, VIEWS).unwrap();
    assert_eq!(
        (c.user, c.password, c.host, c.path),
        (text("u"), text("p"), text("h"), text(""))
    );
}

#[test]
fn port_parsing_stops_at_a_non_digit_and_checks_the_range() {
    assert_eq!(crack_str("http://h:65535/", 0, VIEWS).unwrap().port, 65535);
    assert_eq!(crack_str("http://h:65535", 0, VIEWS).unwrap().port, 65535);
    assert_eq!(crack_str("http://h:6553x/", 0, VIEWS).unwrap().port, 6553);
    assert_eq!(crack_str("http://h:6553", 0, VIEWS).unwrap().port, 6553);
    assert_eq!(crack_str("http://h:x65536", 0, VIEWS).unwrap().port, 0);
    assert_eq!(crack_str("http://h:x/", 0, VIEWS).unwrap().port, 0);
    assert_eq!(crack_str("http://h:0", 0, VIEWS).unwrap().port, 0);
    assert_eq!(crack_str("http://h:00080", 0, VIEWS).unwrap().port, 80);
    assert_eq!(crack_str("http://h:80", 0, VIEWS).unwrap().port, 80);
    assert_eq!(crack_str("http://h:/", 0, VIEWS).unwrap().port, 80);
    assert_eq!(crack_str("http://h:", 0, VIEWS).unwrap().port, 80);

    assert_eq!(
        crack_str("http://h:65536/", 0, VIEWS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
    assert_eq!(
        crack_str("http://h:70000/", 0, VIEWS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
    assert_eq!(
        crack_str("http://h:70000", 0, VIEWS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
    assert_eq!(
        crack_str("http://h:65536x", 0, VIEWS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
    assert_eq!(
        crack_str("http://h:99999999/", 0, VIEWS),
        Err(ERROR_WINHTTP_INVALID_URL)
    );
}

#[test]
fn zero_length_buffer_is_an_error_for_every_component() {
    let zero = |index: usize| {
        let mut slots = VIEWS;
        slots[index] = Slot::Buffer(0);
        slots
    };
    assert_eq!(
        crack_str("http://h/", 0, zero(0)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://u@h/", 0, zero(1)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://u:p@h/", 0, zero(2)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h/", 0, zero(3)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h/", 0, zero(4)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h/a", 0, zero(5)),
        Err(ERROR_INVALID_PARAMETER)
    );

    assert_eq!(
        crack_str("http://h/", 0, zero(1)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h/", 0, zero(2)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://u@h", 0, zero(2)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h", 0, zero(3)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h", 0, zero(4)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h", 0, zero(5)),
        Err(ERROR_INVALID_PARAMETER)
    );

    let cracked = crack_str("http://h/a?x", 0, zero(5)).unwrap();
    assert_eq!((cracked.path, cracked.extra), (text("/a?x"), Piece::Absent));
}

#[test]
fn overflow_is_reported_per_component_and_loses_to_a_later_error() {
    let mut slots = VIEWS;
    slots[3] = Slot::Buffer(1);
    let cracked = crack_str("http://h/", 0, slots).unwrap();
    assert_eq!(cracked.host, Piece::Overflow(2));
    assert_eq!(cracked.path, text("/"));
    assert_eq!(cracked.status(), ERROR_INSUFFICIENT_BUFFER);

    assert_eq!(
        crack_str("http://h:70000/", 0, slots),
        Err(ERROR_WINHTTP_INVALID_URL)
    );

    slots[3] = Slot::Buffer(2);
    let cracked = crack_str("http://h/", 0, slots).unwrap();
    assert_eq!(cracked.host, text("h"));
    assert_eq!(cracked.status(), ERROR_SUCCESS);

    let mut slots = VIEWS;
    slots[0] = Slot::Buffer(4);
    slots[4] = Slot::Buffer(1);
    let cracked = crack_str("http://h/", 0, slots).unwrap();
    assert_eq!(cracked.scheme, Piece::Overflow(5));
    assert_eq!(cracked.path, Piece::Overflow(2));

    let mut slots = VIEWS;
    slots[1] = Slot::Buffer(1);
    let cracked = crack_str("http://u:p@h/", 0, slots).unwrap();
    assert_eq!(
        (cracked.user, cracked.password),
        (Piece::Overflow(2), text("p"))
    );
}

#[test]
fn buffer_user_and_password_without_userinfo_are_empty() {
    let mut slots = VIEWS;
    slots[1] = Slot::Buffer(8);
    slots[2] = Slot::Buffer(8);
    let cracked = crack_str("http://h/", 0, slots).unwrap();
    assert_eq!((cracked.user, cracked.password), (text(""), text("")));
}

#[test]
fn cracking_stops_at_the_component_whose_set_fails() {
    let mut slots = VIEWS;
    slots[1] = Slot::Buffer(0);
    assert_eq!(
        crack_str("http://u:p@h/", 0, slots),
        Err(ERROR_INVALID_PARAMETER)
    );

    let mut slots = VIEWS;
    slots[3] = Slot::Buffer(0);
    assert_eq!(
        crack_str("http://h:80/", 0, slots),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        crack_str("http://h:80", 0, slots),
        Err(ERROR_INVALID_PARAMETER)
    );

    let mut slots = VIEWS;
    slots[4] = Slot::Buffer(0);
    assert_eq!(
        crack_str("http://h/a?b", 0, slots),
        Err(ERROR_INVALID_PARAMETER)
    );
    slots[5] = Slot::Unrequested;
    assert_eq!(
        crack_str("http://h/a?b", 0, slots),
        Err(ERROR_INVALID_PARAMETER)
    );

    let escaped = [
        Slot::Buffer(8),
        Slot::Unrequested,
        Slot::Unrequested,
        Slot::Buffer(8),
        Slot::Unrequested,
        Slot::View,
    ];
    assert_eq!(
        crack_str("http://h/a?b", ESCAPE, escaped),
        Err(ERROR_INVALID_PARAMETER)
    );
}

#[test]
fn create_uses_http_unless_a_scheme_is_given() {
    let host = wide("h");
    assert_eq!(
        create_url(&host_parts(Some(&host)), 0, 64),
        Ok(wide("http://h"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                n_scheme: 2,
                port: 443,
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("https://h"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                n_scheme: 2,
                port: 80,
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("https://h:80"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                n_scheme: 3,
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Err(CreateError::Win32(ERROR_INVALID_PARAMETER))
    );
}

#[test]
fn create_takes_the_scheme_text_as_given_and_matches_it_by_prefix() {
    let host = wide("h");
    for (scheme, port, want) in [
        ("", 80, "://h"),
        ("htt", 80, "htt://h"),
        ("http", 443, "http://h:443"),
        ("HTTP", 443, "HTTP://h:443"),
        ("https", 443, "https://h"),
        ("https", 80, "https://h:80"),
        ("ftp", 80, "ftp://h:80"),
        ("x", 8080, "x://h:8080"),
        ("https\u{0}x", 443, "https\u{0}x://h"),
        ("https\u{0}x", 80, "https\u{0}x://h:80"),
    ] {
        let scheme = wide(scheme);
        let parts = UrlParts {
            scheme: Some(&scheme),
            port,
            ..host_parts(Some(&host))
        };
        assert_eq!(create_url(&parts, 0, 64), Ok(wide(want)), "{want:?}");
    }
}

#[test]
fn create_rejects_a_password_without_a_user_before_checking_capacity() {
    let host = wide("h");
    let password = wide("p");
    let parts = UrlParts {
        password: Some(&password),
        ..host_parts(Some(&host))
    };
    assert_eq!(
        create_url(&parts, 0, 1),
        Err(CreateError::Win32(ERROR_INVALID_PARAMETER))
    );
}

#[test]
fn create_reports_the_required_length_when_the_buffer_is_too_small() {
    let host = wide("h");
    let parts = host_parts(Some(&host));
    assert_eq!(
        create_url(&parts, 0, 7),
        Err(CreateError::Insufficient { required: 9 })
    );
    assert_eq!(
        create_url(&parts, 0, 8),
        Err(CreateError::Insufficient { required: 9 })
    );
    assert_eq!(create_url(&parts, 0, 9), Ok(wide("http://h")));
}

#[test]
fn create_escape_failure_is_reported_after_the_capacity_check() {
    let host = wide("h");
    let path = wide("\u{4e2d}");
    let parts = UrlParts {
        path: Some(&path),
        ..host_parts(Some(&host))
    };
    assert_eq!(
        create_url(&parts, ESCAPE, 64),
        Err(CreateError::Win32(ERROR_INVALID_PARAMETER))
    );
    assert_eq!(
        create_url(&parts, ESCAPE, 1),
        Err(CreateError::Insufficient { required: 13 })
    );
}

#[test]
fn create_adds_a_slash_only_between_a_host_and_a_relative_path() {
    let host = wide("h");
    let path = wide("a");
    let empty = wide("");
    let slash = wide("/");
    let extra = wide("x y");

    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&path),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://h/a"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&empty),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://h/"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&slash),
                extra: Some(&extra),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://h/x y"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&slash),
                extra: Some(&extra),
                ..host_parts(Some(&host))
            },
            ESCAPE,
            64
        ),
        Ok(wide("http://h/x%20y"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&path),
                ..host_parts(None)
            },
            0,
            64
        ),
        Ok(wide("http://a"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&slash),
                ..host_parts(None)
            },
            0,
            64
        ),
        Ok(wide("http:///"))
    );
    assert_eq!(create_url(&host_parts(None), 0, 64), Ok(wide("http://")));
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&empty),
                extra: Some(&empty),
                ..host_parts(None)
            },
            0,
            64
        ),
        Ok(wide("http://"))
    );
}

#[test]
fn create_escape_flag_encodes_path_and_extra_but_not_the_host() {
    let host = wide("h");
    let punct = wide(PUNCT);
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&punct),
                ..host_parts(Some(&host))
            },
            ESCAPE,
            300
        ),
        Ok(wide(
            "http://h/%20!%22%23$%25&'()*+,-./:;%3C=%3E?@%5B%5C%5D%5E_%60%7B%7C%7D%7E%7F%1F"
        ))
    );

    let extra = wide("x\u{ff}");
    assert_eq!(
        create_url(
            &UrlParts {
                extra: Some(&extra),
                ..host_parts(Some(&host))
            },
            ESCAPE,
            64
        ),
        Ok(wide("http://hx%FF"))
    );

    let high = wide("\u{100}");
    assert_eq!(
        create_url(
            &UrlParts {
                path: Some(&high),
                ..host_parts(Some(&host))
            },
            ESCAPE,
            64
        ),
        Err(CreateError::Win32(ERROR_INVALID_PARAMETER))
    );
}

#[test]
fn create_writes_userinfo_with_or_without_a_password() {
    let host = wide("h");
    let user = wide("u");
    let password = wide("p");
    let empty = wide("");

    assert_eq!(
        create_url(
            &UrlParts {
                user: Some(&user),
                password: Some(&password),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://u:p@h"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                user: Some(&user),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://u@h"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                user: Some(&user),
                password: Some(&empty),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://u:@h"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                user: Some(&empty),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://@h"))
    );
    assert_eq!(
        create_url(
            &UrlParts {
                user: Some(&empty),
                password: Some(&empty),
                ..host_parts(Some(&host))
            },
            0,
            64
        ),
        Ok(wide("http://:@h"))
    );
}

#[test]
fn derived_traits_cover_every_variant() {
    let slots = [Slot::Unrequested, Slot::View, Slot::Buffer(1)];
    for slot in slots {
        assert_eq!(Clone::clone(&slot), slot);
        assert!(!format!("{slot:?}").is_empty());
    }
    assert_ne!(slots[0], slots[1]);
    assert_ne!(slots[1], slots[2]);
    assert_ne!(slots[2], slots[0]);
    assert_eq!(Slot::Buffer(1), Slot::Buffer(1));
    assert_ne!(Slot::Buffer(1), Slot::Buffer(2));

    let pieces = [Piece::Absent, text("a"), Piece::Overflow(2)];
    for piece in &pieces {
        assert_eq!(piece.clone(), *piece);
        assert!(!format!("{piece:?}").is_empty());
    }
    assert_ne!(pieces[0], pieces[1]);
    assert_ne!(pieces[1], pieces[2]);
    assert_ne!(pieces[2], pieces[0]);
    assert_eq!(text("a"), text("a"));
    assert_ne!(text("a"), text("b"));
    assert_eq!(Piece::Overflow(2), Piece::Overflow(2));
    assert_ne!(Piece::Overflow(2), Piece::Overflow(3));

    let errors = [
        CreateError::Win32(ERROR_INVALID_PARAMETER),
        CreateError::Insufficient { required: 9 },
    ];
    for error in errors {
        assert_eq!(Clone::clone(&error), error);
        assert!(!format!("{error:?}").is_empty());
    }
    assert_ne!(errors[0], errors[1]);
    assert_ne!(
        CreateError::Win32(ERROR_INVALID_PARAMETER),
        CreateError::Win32(ERROR_INSUFFICIENT_BUFFER)
    );
    assert_eq!(
        CreateError::Insufficient { required: 9 },
        CreateError::Insufficient { required: 9 }
    );
    assert_ne!(
        CreateError::Insufficient { required: 9 },
        CreateError::Insufficient { required: 10 }
    );

    let cracked = crack_str("http://h/", 0, VIEWS).unwrap();
    assert_eq!(cracked.clone(), cracked);
    assert_ne!(
        CrackedUrl {
            port: 1,
            ..cracked.clone()
        },
        cracked
    );
    assert!(!format!("{cracked:?}").is_empty());
}
