//! A minimal JSON reader, just enough for cargo-llvm-cov's `--json` export.
//! It exists so `xtask` stays free of external dependencies, like the rest
//! of the workspace.

/// A parsed JSON value. Objects keep their members in document order.
#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

impl Value {
    /// The value of `key` if `self` is an object that has it.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(members) => members.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(elements) => Some(elements),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(text) => Some(text),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(number) => Some(*number),
            _ => None,
        }
    }
}

/// Parses a complete JSON document. Errors carry the character offset.
pub fn parse(input: &str) -> Result<Value, String> {
    let mut parser = Parser {
        src: input.chars().collect(),
        pos: 0,
    };
    let value = parser.value()?;
    parser.skip_whitespace();
    if parser.pos < parser.src.len() {
        return Err(parser.error("trailing characters"));
    }
    Ok(value)
}

struct Parser {
    src: Vec<char>,
    pos: usize,
}

impl Parser {
    fn error(&self, message: &str) -> String {
        format!("invalid JSON at offset {}: {message}", self.pos)
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    /// Consumes and returns the next character; at the end of input it
    /// returns `None` and stays put.
    fn next(&mut self) -> Option<char> {
        let next = self.peek();
        self.pos += usize::from(next.is_some());
        next
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek()
            .is_some_and(|c| matches!(c, ' ' | '\t' | '\n' | '\r'))
        {
            self.pos += 1;
        }
    }

    fn expect(&mut self, want: char) -> Result<(), String> {
        if self.peek() == Some(want) {
            self.pos += 1;
            Ok(())
        } else {
            Err(self.error(&format!("expected '{want}'")))
        }
    }

    fn literal(&mut self, word: &str, value: Value) -> Result<Value, String> {
        for want in word.chars() {
            self.expect(want)?;
        }
        Ok(value)
    }

    fn value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        match self.peek() {
            Some('{') => self.object(),
            Some('[') => self.array(),
            Some('"') => self.string().map(Value::String),
            Some('t') => self.literal("true", Value::Bool(true)),
            Some('f') => self.literal("false", Value::Bool(false)),
            Some('n') => self.literal("null", Value::Null),
            Some('-' | '0'..='9') => self.number(),
            _ => Err(self.error("expected a value")),
        }
    }

    /// Reads a bracketed, comma-separated sequence: skips the opening
    /// bracket, then calls `item` once per element until `close`.
    fn items(
        &mut self,
        close: char,
        mut item: impl FnMut(&mut Self) -> Result<(), String>,
    ) -> Result<(), String> {
        self.pos += 1;
        self.skip_whitespace();
        if self.peek() == Some(close) {
            self.pos += 1;
            return Ok(());
        }
        loop {
            item(self)?;
            self.skip_whitespace();
            match self.next() {
                Some(',') => {}
                Some(c) if c == close => return Ok(()),
                _ => return Err(self.error(&format!("expected ',' or '{close}'"))),
            }
        }
    }

    fn object(&mut self) -> Result<Value, String> {
        let mut members = Vec::new();
        self.items('}', |p| {
            p.skip_whitespace();
            let key = p.string()?;
            p.skip_whitespace();
            p.expect(':')?;
            members.push((key, p.value()?));
            Ok(())
        })?;
        Ok(Value::Object(members))
    }

    fn array(&mut self) -> Result<Value, String> {
        let mut elements = Vec::new();
        self.items(']', |p| {
            elements.push(p.value()?);
            Ok(())
        })?;
        Ok(Value::Array(elements))
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect('"')?;
        let mut out = String::new();
        loop {
            match self.next() {
                None => return Err(self.error("unterminated string")),
                Some('"') => return Ok(out),
                Some('\\') => out.push(self.escape()?),
                Some(c) => out.push(c),
            }
        }
    }

    fn escape(&mut self) -> Result<char, String> {
        match self.next() {
            Some(c @ ('"' | '\\' | '/')) => Ok(c),
            Some('b') => Ok('\u{8}'),
            Some('f') => Ok('\u{c}'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('u') => self.unicode(),
            _ => Err(self.error("invalid escape")),
        }
    }

    /// The four hex digits of a `\u` escape. A lone surrogate, which is not a
    /// `char`, becomes U+FFFD.
    fn unicode(&mut self) -> Result<char, String> {
        let mut code = 0;
        for _ in 0..4 {
            let Some(digit) = self.next().and_then(|c| c.to_digit(16)) else {
                return Err(self.error("invalid \\u escape"));
            };
            code = code * 16 + digit;
        }
        Ok(char::from_u32(code).unwrap_or(char::REPLACEMENT_CHARACTER))
    }

    fn number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        while self
            .peek()
            .is_some_and(|c| matches!(c, '0'..='9' | '-' | '+' | '.' | 'e' | 'E'))
        {
            self.pos += 1;
        }
        let text: String = self.src[start..self.pos].iter().collect();
        text.parse()
            .map(Value::Number)
            .map_err(|_| self.error(&format!("invalid number '{text}'")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn error(input: &str) -> String {
        parse(input).unwrap_err()
    }

    #[test]
    fn parses_scalars_and_surrounding_whitespace() {
        assert_eq!(parse("null"), Ok(Value::Null));
        assert_eq!(parse("true"), Ok(Value::Bool(true)));
        assert_eq!(parse("false"), Ok(Value::Bool(false)));
        assert_eq!(parse("0"), Ok(Value::Number(0.0)));
        assert_eq!(parse("-12"), Ok(Value::Number(-12.0)));
        assert_eq!(parse("3.5"), Ok(Value::Number(3.5)));
        assert_eq!(parse("1e3"), Ok(Value::Number(1000.0)));
        assert_eq!(parse("2E-2"), Ok(Value::Number(0.02)));
        assert_eq!(parse(" \t\r\n 7 \n"), Ok(Value::Number(7.0)));
    }

    #[test]
    fn parses_strings_and_every_escape() {
        assert_eq!(parse(r#""plain""#), Ok(Value::String("plain".into())));
        assert_eq!(parse(r#""""#), Ok(Value::String(String::new())));
        assert_eq!(parse(r#""é 日本""#), Ok(Value::String("é 日本".into())));
        assert_eq!(
            parse(r#""\"\\\/\b\f\n\r\t""#),
            Ok(Value::String("\"\\/\u{8}\u{c}\n\r\t".into()))
        );
        assert_eq!(parse(r#""\u0041\u00e9""#), Ok(Value::String("Aé".into())));
        // A lone surrogate is not a char.
        assert_eq!(parse(r#""\ud800""#), Ok(Value::String("\u{fffd}".into())));
    }

    #[test]
    fn parses_nested_containers() {
        assert_eq!(parse("[]"), Ok(Value::Array(vec![])));
        assert_eq!(parse("{}"), Ok(Value::Object(vec![])));
        assert_eq!(parse("[ ]"), Ok(Value::Array(vec![])));
        assert_eq!(parse("{ }"), Ok(Value::Object(vec![])));
        assert_eq!(
            parse(r#" { "a" : [ 1 , { "b" : null } ] , "c" : { } } "#),
            Ok(Value::Object(vec![
                (
                    "a".into(),
                    Value::Array(vec![
                        Value::Number(1.0),
                        Value::Object(vec![("b".into(), Value::Null)])
                    ])
                ),
                ("c".into(), Value::Object(vec![])),
            ]))
        );
    }

    #[test]
    fn accessors_only_match_their_own_type() {
        let doc = parse(r#"{"n":1,"s":"x","a":[true]}"#).unwrap();
        assert_eq!(doc.get("n").and_then(Value::as_f64), Some(1.0));
        assert_eq!(doc.get("s").and_then(Value::as_str), Some("x"));
        assert_eq!(
            doc.get("a").and_then(Value::as_array),
            Some(&[Value::Bool(true)][..])
        );
        assert!(doc.get("missing").is_none());

        assert!(doc.get("n").and_then(Value::as_str).is_none());
        assert!(doc.get("s").and_then(Value::as_array).is_none());
        assert!(doc.get("a").and_then(Value::as_f64).is_none());
        // `get` on a non-object.
        assert!(Value::Null.get("n").is_none());
    }

    #[test]
    fn get_returns_the_first_of_duplicate_keys() {
        let doc = parse(r#"{"k":1,"k":2}"#).unwrap();
        assert_eq!(doc.get("k"), Some(&Value::Number(1.0)));
    }

    #[test]
    fn rejects_malformed_input_with_an_offset() {
        assert_eq!(error("@"), "invalid JSON at offset 0: expected a value");
        for (input, expected) in [
            ("", "expected a value"),
            ("   ", "expected a value"),
            ("tru", "expected 'e'"),
            ("nul", "expected 'l'"),
            ("fax", "expected 'l'"),
            ("-", "invalid number '-'"),
            ("1.2.3", "invalid number '1.2.3'"),
            (r#""abc"#, "unterminated string"),
            (r#""\"#, "invalid escape"),
            (r#""\x""#, "invalid escape"),
            (r#""\u12""#, "invalid \\u escape"),
            (r#""\u"#, "invalid \\u escape"),
            ("[1 2]", "expected ',' or ']'"),
            ("[1", "expected ',' or ']'"),
            ("[1,]", "expected a value"),
            ("{1:2}", "expected '\"'"),
            ("{", "expected '\"'"),
            (r#"{"a" 1}"#, "expected ':'"),
            (r#"{"a":}"#, "expected a value"),
            (r#"{"a":1 "b":2}"#, "expected ',' or '}'"),
            (r#"{"a":1,}"#, "expected '\"'"),
            ("1 2", "trailing characters"),
            ("[] x", "trailing characters"),
        ] {
            let message = error(input);
            assert!(
                message.starts_with("invalid JSON at offset ") && message.ends_with(expected),
                "{input:?} -> {message}"
            );
        }
    }
}
