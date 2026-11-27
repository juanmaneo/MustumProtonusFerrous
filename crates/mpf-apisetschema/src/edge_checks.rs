//! Boundary and branch tests for the ported apiset logic. Expectations marked "Not oracle-derived" come from
//! reading the Wine source, not from the captured vectors in `oracle_checks.rs`.

use super::*;

fn split(line: &[u8]) -> Vec<Vec<u8>> {
    let mut tokens = tokens::Tokenizer::new(line);
    let mut out = Vec::new();
    while let Some(token) = tokens.next_token() {
        out.push(token);
    }
    out
}

fn strs(words: &[&str]) -> Vec<Vec<u8>> {
    words.iter().map(|word| word.as_bytes().to_vec()).collect()
}

/// The little-endian dword at byte offset `at`.
fn word_at(section: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(section[at..at + 4].try_into().unwrap())
}

#[test]
fn tokenizer_splits_on_c_isspace_and_separators() {
    // Not oracle-derived: `isspace` in the C locale also covers vertical tab and form feed, bytes from 0x80 are
    // not spaces, and `(` and `)` are tokens of their own.
    assert_eq!(
        split(b"a \t\x0bb\x0cc\rd\ne"),
        strs(&["a", "b", "c", "d", "e"])
    );
    assert_eq!(split("é x".as_bytes()), strs(&["é", "x"]));
    assert_eq!(split(b"(x)"), strs(&["(", "x", ")"]));
    assert_eq!(split(b"  "), strs(&[]));
}

#[test]
fn tokenizer_comments_only_start_a_token() {
    // Not oracle-derived: `#` and `;` end the line only where a token would start.
    assert_eq!(split(b"# x"), strs(&[]));
    assert_eq!(split(b"; x"), strs(&[]));
    assert_eq!(split(b"x#y ;z"), strs(&["x#y"]));
}

#[test]
fn tokenizer_backslash_keeps_the_next_byte() {
    // Not oracle-derived: a backslash keeps the next byte in the token, even a space or a separator.
    assert_eq!(split(b"a\\ b"), strs(&["a b"]));
    assert_eq!(split(b"\\(x\\)"), strs(&["(x)"]));
    assert_eq!(split(b"a\\\\b"), strs(&["a\\b"]));
}

#[test]
fn tokenizer_trailing_backslash_and_nul_end_the_line() {
    // Not oracle-derived: Wine keeps the newline after a trailing backslash, and this port has no newline.
    // A NUL byte ends the line, as in a C string.
    assert_eq!(split(b"a\\"), strs(&["a"]));
    assert_eq!(split(b"a\0b"), strs(&["a"]));
}

#[test]
fn apiset_hash_needs_a_hyphen() {
    // Not oracle-derived: Wine reads past a name with no hyphen (undefined behaviour), so the port returns None.
    assert_eq!(apiset_hash(b"nohyphen"), None);
    // Not oracle-derived: the hashed part is empty, so the hash is 0.
    assert_eq!(apiset_hash(b"-"), Some((0, 0)));
}

#[test]
fn apiset_hash_collides_for_equal_prefix_sums() {
    // Not oracle-derived: 31 * 97 + 98 == 31 * 98 + 67 == 3105, so "ab-1" and "bC-1" share a hash.
    assert_eq!(apiset_hash(b"ab-1"), Some((3105, 2)));
    assert_eq!(apiset_hash(b"bC-1"), Some((3105, 2)));
}

#[test]
fn pool_finds_a_string_inside_another() {
    // Not oracle-derived: strstr finds "bc" inside "abcd", which was added as two strings.
    let mut pool = pool::StringPool::default();
    assert_eq!(pool.add(b"ab", 2), 0);
    assert_eq!(pool.add(b"cd", 2), 2);
    assert_eq!(pool.add(b"bc", 2), 1);
    assert_eq!(pool.bytes(), b"abcd");
    // Not oracle-derived: an empty needle matches at offset 0.
    assert_eq!(pool.add(b"", 0), 0);
}

#[test]
fn pool_appends_only_the_name_part_of_a_value() {
    // Not oracle-derived: the search uses the whole token "x:y", but only the first `len` bytes are appended.
    let mut pool = pool::StringPool::default();
    assert_eq!(pool.add(b"x:y", 1), 0);
    assert_eq!(pool.bytes(), b"x");
    assert_eq!(pool.add(b"x", 1), 0);
}

#[test]
fn parse_line_rejects_other_directives() {
    // Not oracle-derived: Wine also parses `@` and ordinal lines as exports, which this port does not model.
    let mut schema = Schema::default();
    assert_eq!(
        schema.parse_line(b"@ stdcall foo()"),
        Err(SpecError::UnknownDirective)
    );
    assert_eq!(
        schema.parse_line(b"apisetx a-1 = x"),
        Err(SpecError::UnknownDirective)
    );
}

#[test]
fn parse_line_needs_a_name_and_an_equals_sign() {
    // Not oracle-derived: a line with only `apiset`, or a name with nothing after it, is a syntax error.
    let mut schema = Schema::default();
    assert_eq!(schema.parse_line(b"apiset"), Err(SpecError::Syntax));
    assert_eq!(schema.parse_line(b"apiset api-a-1"), Err(SpecError::Syntax));
}

#[test]
fn parse_line_rejects_a_name_without_hyphen() {
    // Not oracle-derived: the oracle's no-hyphen vector is excluded (see oracle_checks.rs).
    let mut schema = Schema::default();
    assert_eq!(
        schema.parse_line(b"apiset nohyphen = x.dll"),
        Err(SpecError::NoHyphen)
    );
}

#[test]
fn parse_line_checks_duplicates_before_equals() {
    // Not oracle-derived: Wine checks for a duplicate name before it reads `=`.
    let mut schema = Schema::parse_spec(b"apiset api-a-1 = x\n").expect("one record");
    assert_eq!(
        schema.parse_line(b"apiset api-a-1 x"),
        Err(SpecError::DuplicateName)
    );
    assert_eq!(
        schema.parse_line(b"apiset api-a-1 = y"),
        Err(SpecError::DuplicateName)
    );
}

#[test]
fn parse_line_rejects_a_duplicate_hash() {
    // Not oracle-derived: "ab" and "bC" hash to the same value (see apiset_hash_collides_for_equal_prefix_sums).
    let mut schema = Schema::default();
    schema.parse_line(b"apiset ab-1 = x").expect("first record");
    assert_eq!(
        schema.parse_line(b"apiset bC-1 = y"),
        Err(SpecError::DuplicateHash)
    );
}

#[test]
fn parse_line_rejects_a_fourth_value_and_keeps_the_schema() {
    // Not oracle-derived: Wine skips the extra value and counts an error. This port returns the error and
    // leaves the schema unchanged.
    let mut schema = Schema::default();
    assert_eq!(
        schema.parse_line(b"apiset api-a-1 = a b c d"),
        Err(SpecError::TooManyValues)
    );
    assert!(schema.entries.is_empty());
    assert!(schema.pool.bytes().is_empty());
}

#[test]
fn parse_spec_splits_lines_and_stops_at_the_first_error() {
    // Not oracle-derived: parse_spec splits on newlines, accepts a last line with no newline, and returns the
    // first error, where Wine would keep parsing and fail at the end.
    let schema = Schema::parse_spec(b"# head\n\napiset api-a-1 = x\napiset api-b-1 = y")
        .expect("two records");
    assert_eq!(schema.entries.len(), 2);
    let err = Schema::parse_spec(b"apiset api-a-1 = x\napiset api-a-1 = y\napiset\n").err();
    assert_eq!(err, Some(SpecError::DuplicateName));
}

#[test]
fn section_writes_zero_name_for_a_value_named_at_pool_offset_zero() {
    // Not oracle-derived: the value name "x:y" is found at pool offset 0 inside the record name "x:y-1", so
    // spec32.c writes NameOffset and NameLength as 0. The layout is 28 (header) + 24 (name entry), so the value
    // entry starts at byte 52; the strings start at 72; "y" is at pool offset 2.
    let schema = Schema::parse_spec(b"apiset x:y-1 = x:y\n").expect("one record");
    let section = schema.section();
    assert_eq!(word_at(&section, 32), 72);
    assert_eq!(word_at(&section, 56), 0);
    assert_eq!(word_at(&section, 60), 0);
    assert_eq!(word_at(&section, 64), 76);
    assert_eq!(word_at(&section, 68), 2);
}
