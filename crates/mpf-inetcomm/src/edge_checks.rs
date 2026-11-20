//! Edge cases the oracle rows do not reach. The `EDGE_ROWS` lines were run through the same Wine
//! driver as the rows in `oracle_checks`, with no sanitizer report, and each expected output is the
//! driver's output. The rest are Rust-only: long values, NUL bytes in `mhtml:`, SMTP codes past
//! 32 bits, and inputs without a CRLF, which are undefined behavior in the C code.

use super::oracle_checks::run;
use crate::header;
use crate::mhtml::parse_mhtml_url;
use crate::smtp::{parse_status, Status, RESPONSE_ERROR};

const EDGE_ROWS: &[(&str, &str)] = &[
    (r"noline", "error"),
    (r"nope|x", "error"),
    (r"unquote|abc\", "abc"),
    (r#"unquote|a"b"#, r#"a"b"#),
    (r"unquote|\q", "q"),
    (r#"unquote|"abc"#, "abc"),
    (r#"unquote|  "a b""#, "a b"),
    (r#"unquote|\t"q""#, "q"),
    (r"params|\q", r"\\q"),
    (r"params|\xZZ", r"\\xZZ"),
    (r"params|a\x", r"a\\x"),
    (r"params|a\x1", r"a\\x1"),
    (r"params|\x1Z", r"\\x1Z"),
    (r"params|\xE9\x7f", r"\xe9\x7f"),
    (r"params|\x4B\x6b", "Kk"),
    (r"params|a\nb\x0A\x09", r"a\nb\n\t"),
    (r"params|a\r\\b", r"a\r\\b"),
    (r#"params|a;b="q""#, "a|b=q"),
    (r#"params|a;b="unterminated"#, "a|b=unterminated"),
    (r#"params|"a;b""#, r#""a;b""#),
    (r"params|no params", "no params"),
    (r"params|a;novalue", "a"),
    (r"params|a; name =v", "a|name =v"),
    (r"params|a;b=x;c=y;", "a|b=x|c=y"),
    (r"params|a;;x=1", "a|x=1"),
    (r"unfold|x\r\n", "x"),
    (r"unfold|\r\n", ""),
    (r"ctype|a/b", "a|b"),
    (r"ctype|image/JPEG", "image|JPEG"),
    (r"cenc|base64", "base64"),
    (r"cenc|7bit", "7bit"),
    (r"cenc|8bit", "8bit"),
    (r"cenc|7BIT", "7bit"),
    (r"block|", "0 0"),
    (r"block|\r", "1 1"),
    (r"block|\r\nbody", "2 2"),
    (r"block|a\r\nb\r\n", "6 6"),
    (r"block|x\r\n\r\n\r\n", "5 5"),
    (r"block|a\nb\r\n\r\n", "7 7"),
    (r"block|Subject: x", "10 10"),
    (r"b64|0000", "d34d34"),
    (r"b64|QUJD\r\nREVG", "414243444546"),
    (r"b64|QU!JD", "414243"),
    (r"b64|QUJDRA", "41424344"),
    (r"b64|Q", ""),
    (r"qp|=41", "41"),
    (r"qp|=ab", "ab"),
    (r"qp|=4G", "3d"),
    (r"qp|=", ""),
    (r"qp|a=", "61"),
    (r"qp|a=3D", "613d"),
    (r"qp|==41", "3d31"),
    (r"qp|a=\r\nb", "6162"),
    (r"qp|a=\r\n", "61"),
    (r"qp|\xff", "ff"),
    (r"mhtml|mhtm", "fail"),
    (r"mhtml|http:x", "fail"),
    (r"mhtml|mhtml:", "ok 0 -1"),
    (r"mhtml|mhtml:file.mht", "ok 8 -1"),
    (r"mhtml|mhtml:a!", "ok 1 8"),
    (r"mhtml|mhtml:a!x-usc:", "ok 1 14"),
    (r"mhtml|mhtml:a!x-us", "ok 1 8"),
    (r"mhtml|mhtml:\xe9", "ok 1 -1"),
    (r"mhtml|MHTML:A!b", "ok 1 8"),
    (r"smtp|", "error 800ccc60"),
    (r"smtp| 250", "error 800ccc60"),
    (r"smtp|x250", "error 800ccc60"),
    (r"smtp|250-", "ok 250 0 000ccc74"),
    (r"smtp|2500-x", "ok 2500 0 800ccc60"),
    (r"smtp|25a0 x", "ok 25 1 800ccc60"),
    (r"smtp|999 unknown", "ok 999 1 800ccc60"),
    (r"smtp|211 status", "ok 211 1 000ccc70"),
    (r"field|X: a\r\n", "X|a|@6"),
    (r"field|A:B:C\r\n\r\n", "A|B:C|@7"),
    (r"field|: empty name\r\n\r\n", "|empty name|@14"),
    (r"field|Subject:  padded  \r\n\r\n", "Subject|padded  |@20"),
    (r"field|X-Foo: a\r\nb\r\n", "X-Foo|a|@10"),
    (
        r"field|X-Foo: one\r\n\ttwo\r\n three\r\n\r\n",
        "X-Foo|one two three|@26",
    ),
    (
        r#"field|Content-Disposition: attachment; filename="a b.txt"\r\n\r\n"#,
        r"Content-Disposition|attachment|filename=a b.txt|@53",
    ),
    (
        r"field|Content-Type: text/plain; name=a=b\r\n\r\n",
        "Content-Type|text/plain|name=a=b|@36",
    ),
    (
        r"field|subject: hello world\r\n\r\n",
        "subject|hello world|@22",
    ),
    (r"field|Subject: a; b=c\r\n\r\n", "Subject|a; b=c|@17"),
    (r"field|\r\n", "none"),
    (r"field|X: \r\n", "X||@5"),
];

#[test]
fn edge_rows_match_wine() {
    for &(line, output) in EDGE_ROWS {
        assert_eq!(run(line), output, "driver line: {line}");
    }
}

#[test]
fn long_values_are_read_whole() {
    let block = format!("block|{}\\r\\n\\r\\nzzz", "a".repeat(1023));
    assert_eq!(run(&block), "1027 1027");
    let body = format!("block|{}", "a".repeat(3000));
    assert_eq!(run(&body), "3000 3000");
    let field = format!("field|X: {}\\r\\n", "a".repeat(1500));
    assert_eq!(run(&field), format!("X|{}|@1505", "a".repeat(1500)));
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn mhtml_url_ends_at_the_first_nul() {
    let url = wide("mhtml:a\0!b");
    let parsed = parse_mhtml_url(&url).unwrap();
    assert_eq!(parsed.mhtml, wide("a").as_slice());
    assert_eq!(parsed.location, None);
    assert!(parse_mhtml_url(&wide("\0mhtml:a")).is_none());
}

#[test]
fn smtp_code_clamps_at_the_windows_long_limit() {
    assert_eq!(
        parse_status(b"2147483647 x"),
        Some(Status {
            code: 0x7fff_ffff,
            done: true,
            hr: RESPONSE_ERROR
        })
    );
    assert_eq!(
        parse_status(b"2147483648 x"),
        Some(Status {
            code: 0x7fff_ffff,
            done: true,
            hr: RESPONSE_ERROR
        })
    );
    assert_eq!(
        parse_status(b"99999999999999999999-x"),
        Some(Status {
            code: 0x7fff_ffff,
            done: false,
            hr: RESPONSE_ERROR
        })
    );
}

#[test]
fn every_smtp_status_code_has_its_result() {
    const CODES: [(u32, u32); 25] = [
        (211, 0x000c_cc70),
        (214, 0x000c_cc71),
        (220, 0x000c_cc72),
        (221, 0x000c_cc73),
        (245, 0x000c_cc79),
        (250, 0x000c_cc74),
        (251, 0x000c_cc75),
        (334, 0x000c_cc78),
        (354, 0x000c_cc76),
        (421, 0x800c_cc67),
        (450, 0x800c_cc68),
        (451, 0x800c_cc6a),
        (452, 0x800c_cc6c),
        (454, 0x800c_cc7f),
        (500, 0x800c_cc62),
        (501, 0x800c_cc63),
        (502, 0x800c_cc64),
        (503, 0x800c_cc65),
        (504, 0x800c_cc66),
        (530, 0x800c_cc7c),
        (550, 0x800c_cc69),
        (551, 0x800c_cc6b),
        (552, 0x800c_cc6d),
        (553, 0x800c_cc6e),
        (554, 0x800c_cc6f),
    ];
    for (code, hr) in CODES {
        let line = format!("{code} x");
        let status = parse_status(line.as_bytes()).unwrap();
        assert_eq!((status.code, status.hr), (code, hr));
    }
}

#[test]
fn field_without_crlf_takes_the_rest_of_the_block() {
    let field = header::read_field(b"X: abc").unwrap();
    assert_eq!(field.name, b"X");
    assert_eq!(field.value, b"abc".to_vec());
    assert!(field.params.is_empty());
    assert!(field.rest.is_empty());
}

#[test]
fn unfold_without_crlf_keeps_the_text() {
    assert_eq!(header::unfold(b"abc"), b"abc".to_vec());
}
