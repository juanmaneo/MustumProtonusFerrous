//! The init-string parser `parse_init_string` in `dlls/oledb32/datainit.c`.

const DQUOTE: u16 = b'"' as u16;
const SQUOTE: u16 = b'\'' as u16;
const EQUALS: u16 = b'=' as u16;
const SEMICOLON: u16 = b';' as u16;
const PROVIDER: &str = "provider";

/// One `name=value` pair. The C `struct dbprop_pair` holds two BSTRs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbProp {
    pub name: Vec<u16>,
    pub value: Vec<u16>,
}

/// `parse_init_string`: splits `init` into `name=value` pairs, in order.
///
/// The C string ends at its first NUL. A name keeps its trailing blanks and a value keeps its
/// leading and trailing blanks. A quoted value ends at the matching quote, or at the end of the
/// string if there is none. An unquoted value ends at `;`. One `;` after a value is consumed.
/// A pair whose name is `Provider` (ASCII case-insensitive, exact length) is dropped. The
/// allocation-failure path is not modelled.
pub fn parse_init_string(init: &[u16]) -> Vec<DbProp> {
    let s = c_string(init);
    let mut props = Vec::new();
    let mut start = Some(0);
    while let Some(begin) = start {
        let Some(eq) = find_char(s, begin, EQUALS) else {
            break;
        };
        let mut name_start = begin;
        while is_space(at(s, name_start)) {
            name_start += 1;
        }
        let name = &s[name_start..eq];

        let quote = Some(at(s, eq + 1)).filter(|&c| c == DQUOTE || c == SQUOTE);
        let (value_start, delim) = match quote {
            Some(q) => (eq + 2, find_char(s, eq + 2, q)),
            None => (eq + 1, find_char(s, eq + 1, SEMICOLON)),
        };
        let value = &s[value_start..delim.unwrap_or(s.len())];
        start = delim.map(|d| {
            let d = if Some(at(s, d)) == quote { d + 1 } else { d };
            if at(s, d) == SEMICOLON {
                d + 1
            } else {
                d
            }
        });

        if !is_provider(name) {
            props.push(DbProp {
                name: name.to_vec(),
                value: value.to_vec(),
            });
        }
    }
    props
}

fn c_string(init: &[u16]) -> &[u16] {
    let end = init.iter().position(|&c| c == 0).unwrap_or(init.len());
    &init[..end]
}

fn at(s: &[u16], i: usize) -> u16 {
    s.get(i).copied().unwrap_or(0)
}

fn find_char(s: &[u16], from: usize, c: u16) -> Option<usize> {
    s[from..].iter().position(|&x| x == c).map(|p| from + p)
}

/// `iswspace` for ASCII: space, and `\t` through `\r`.
fn is_space(c: u16) -> bool {
    c == 0x20 || (0x09..=0x0d).contains(&c)
}

fn is_provider(name: &[u16]) -> bool {
    name.len() == PROVIDER.len()
        && name
            .iter()
            .zip(PROVIDER.bytes())
            .all(|(&c, p)| fold(c) == u16::from(p))
}

/// ASCII lowercase for `wcsicmp`.
fn fold(c: u16) -> u16 {
    match c {
        0x41..=0x5a => c + 0x20,
        _ => c,
    }
}
