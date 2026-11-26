//! Oracle-derived test vectors for the ported opcservices logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `dlls/opcservices/uri.c` (`opc_part_uri_get_rels_uri`), `opc_private.h`
//! (`opc_array_reserve`) and `package.c` (`opc_package_add_content_type`) with
//! `gcc -fsanitize=address,undefined -g -fshort-wchar` under
//! `target/canon/opcservices/`, and running the harness. It printed 25 rows.
//! One further input, a path with no `/`, is excluded: ASan reports
//! `negative-size-param` in the C `memcpy`, so the C has no defined result to
//! compare with. Excluded vectors: 1.

use super::*;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// One part of a sequence: its extension (`None` when the name has none) and content type.
type Part = (Option<&'static str>, &'static str);

/// Applies `classify_content_type` to each part in order, the way the C harness
/// does, and returns one letter per part: `D` for AddDefault, `O` for AddOverride
/// and `N` for Nothing.
fn run_sequence(parts: &[Part]) -> String {
    let mut defaults: Vec<(Vec<u16>, Vec<u16>)> = Vec::new();
    let mut letters = String::new();
    for &(ext, content_type) in parts {
        let ext = ext.map(wide);
        let content_type = wide(content_type);
        let action = {
            let view: Vec<DefaultType<'_>> = defaults
                .iter()
                .map(|(ext, content_type)| DefaultType {
                    ext: ext.as_slice(),
                    content_type: content_type.as_slice(),
                })
                .collect();
            classify_content_type(&view, ext.as_deref(), &content_type)
        };
        letters.push(match action {
            ContentTypeAction::Nothing => 'N',
            ContentTypeAction::AddOverride => 'O',
            ContentTypeAction::AddDefault => {
                defaults.push((ext.expect("AddDefault needs an extension"), content_type));
                'D'
            }
        });
    }
    letters
}

#[test]
fn oracle_rels_part_path_rows() {
    let rows: &[(&str, Option<&str>)] = &[
        ("/", Some("/_rels/.rels")),
        ("/doc.xml", Some("/_rels/doc.xml.rels")),
        ("/a/b/c.xml", Some("/a/b/_rels/c.xml.rels")),
        ("/path/uri", Some("/path/_rels/uri.rels")),
        ("/words/x", Some("/_rels/x.rels")),
        ("/abcdef/x", Some("//_rels/x.rels")),
        ("/ab/cd/ef/gh.xml", Some("/ab/_rels/gh.xml.rels")),
        ("/_rels/.rels", None),
        ("/_rels.rels", None),
        ("/a/_rels/b.rels", None),
        ("/abc/x.rels", Some("/abc/_rels/x.rels.rels")),
    ];
    for &(path, expected) in rows {
        assert_eq!(
            rels_part_path(&wide(path)),
            expected.map(wide),
            "path {path:?}"
        );
    }
}

#[test]
fn oracle_classify_content_type_sequences() {
    let rows: &[(&[Part], &str)] = &[
        (&[(Some("png"), "image/png")], "D"),
        (
            &[(Some("png"), "image/png"), (Some("png"), "image/png")],
            "DN",
        ),
        (
            &[(Some("png"), "image/png"), (Some("png"), "image/jpeg")],
            "DO",
        ),
        (
            &[(Some("PNG"), "image/png"), (Some("png"), "image/png")],
            "DN",
        ),
        (&[(None, "text/xml")], "O"),
        (
            &[
                (Some("png"), "image/png"),
                (None, "text/xml"),
                (Some("png"), "image/jpeg"),
            ],
            "DOO",
        ),
        (
            &[(Some("png"), "image/png"), (Some("PNG"), "IMAGE/PNG")],
            "DO",
        ),
        (
            &[
                (Some("xml"), "a"),
                (Some("XML"), "a"),
                (Some("Xml"), "b"),
                (Some("xml"), "b"),
            ],
            "DNOO",
        ),
    ];
    for &(parts, expected) in rows {
        assert_eq!(run_sequence(parts), expected, "parts {parts:?}");
    }
}

#[test]
fn oracle_array_reserve_capacity_rows() {
    let rows: &[(usize, usize, usize, Option<usize>)] = &[
        (0, 1, 8, Some(4)),
        (4, 4, 8, Some(4)),
        (4, 5, 8, Some(8)),
        (3, 9, 8, Some(16)),
        (0, 4, 1 << 62, None),
        (0, 2, 1 << 63, None),
    ];
    for &(capacity, count, size, expected) in rows {
        assert_eq!(
            array_reserve_capacity(capacity, count, size),
            expected,
            "capacity {capacity} count {count} size {size}"
        );
    }
}
