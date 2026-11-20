use super::*;

#[test]
fn verbs_match_wine_oracle() {
    let rows: [(&[u8], u8); 4] = [(b"OPTIONS", 0), (b"G", 1), (b"GET", 1), (b"get", 17)];
    for (input, expected) in rows {
        assert_eq!(parse_verb(input), expected, "{input:?}");
    }
}

#[test]
fn tokens_match_wine_oracle() {
    let rows: [(&[u8], usize); 3] = [(b"Host: x", 4), (b"(x)", 0), (b"abc-def", 7)];
    for (input, expected) in rows {
        assert_eq!(parse_token(input), expected, "{input:?}");
    }
}

#[test]
fn header_ids_match_wine_oracle() {
    let rows: [(&[u8], usize); 4] = [
        (b"Host", 28),
        (b"hOsT", 28),
        (b"Content-L", 11),
        (b"X-Foo", 41),
    ];
    for (input, expected) in rows {
        assert_eq!(parse_header_name(input), expected, "{input:?}");
    }
}

#[test]
fn header_lines_match_wine_oracle() {
    let rows: [(&[u8], &[u8], &[u8]); 2] = [
        (b"Host:  example.com \t\r\n", b"Host", b"example.com"),
        (b"X:\t\tv\t \r\n", b"X", b"v"),
    ];
    for (input, expected_name, expected_value) in rows {
        let parsed = parse_header(input);
        assert_eq!(parsed.name, expected_name, "{input:?}");
        assert_eq!(parsed.value, expected_value, "{input:?}");
    }
}

#[test]
fn exact_comparisons_match_wine_oracle() {
    let rows: [(&[u8], &[u8], i32); 3] = [
        (b"HT", b"HTTP/", 0),
        (b"HTTP/1.1", b"HTTP/", 1),
        (b"HTX", b"HTTP/", -1),
    ];
    for (input, expected, result) in rows {
        assert_eq!(compare_exact(input, expected), result, "{input:?}");
    }
}

#[test]
fn numbers_match_wine_oracle() {
    let rows: [(&[u8], (i32, usize)); 2] = [(b"123xyz", (123, 3)), (b"x", (0, 0))];
    for (input, expected) in rows {
        assert_eq!(parse_number(input), expected, "{input:?}");
    }
}

#[test]
fn paths_match_wine_oracle() {
    let rows: [(&[u8], &[u8], u32); 6] = [
        (b"/api", b"/api/v1", 2),
        (b"/api/", b"/api/", 2),
        (b"/api", b"/apix", 0),
        (b"/a", b"/a?x", 2),
        (b"/a/b", b"/a/b/c", 3),
        (b"/a/b", b"/a/bc", 0),
    ];
    for (queue_path, connection_path, expected) in rows {
        assert_eq!(
            compare_paths(queue_path, connection_path),
            expected,
            "{queue_path:?} / {connection_path:?}"
        );
    }
}
