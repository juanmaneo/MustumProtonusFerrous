/// The HTML entities that help.c decodes: (name, ANSI code page byte), sorted by name because
/// the lookup is a binary search.
pub(crate) const ENTITIES: &[(&str, u8)] = &[
    ("AElig", 0xC6),
    ("Aacute", 0xC1),
    ("Acirc", 0xC2),
    ("Agrave", 0xC0),
    ("Aring", 0xC5),
    ("Atilde", 0xC3),
    ("Auml", 0xC4),
    ("Ccedil", 0xC7),
    ("ETH", 0xD0),
    ("Eacute", 0xC9),
    ("Ecirc", 0xCA),
    ("Egrave", 0xC8),
    ("Euml", 0xCB),
    ("Iacute", 0xCD),
    ("Icirc", 0xCE),
    ("Igrave", 0xCC),
    ("Iuml", 0xCF),
    ("Ntilde", 0xD1),
    ("Oacute", 0xD3),
    ("Ocirc", 0xD4),
    ("Ograve", 0xD2),
    ("Oslash", 0xD8),
    ("Otilde", 0xD5),
    ("Ouml", 0xD6),
    ("THORN", 0xDE),
    ("Uacute", 0xDA),
    ("Ucirc", 0xDB),
    ("Ugrave", 0xD9),
    ("Uuml", 0xDC),
    ("Yacute", 0xDD),
    ("aacute", 0xE1),
    ("acirc", 0xE2),
    ("acute", 0xB4),
    ("aelig", 0xE6),
    ("agrave", 0xE0),
    ("amp", 0x26),
    ("aring", 0xE5),
    ("atilde", 0xE3),
    ("auml", 0xE4),
    ("brvbar", 0xA6),
    ("ccedil", 0xE7),
    ("cedil", 0xB8),
    ("cent", 0xA2),
    ("copy", 0xA9),
    ("curren", 0xA4),
    ("deg", 0xB0),
    ("divide", 0xF7),
    ("eacute", 0xE9),
    ("ecirc", 0xEA),
    ("egrave", 0xE8),
    ("eth", 0xF0),
    ("euml", 0xEB),
    ("frac12", 0xBD),
    ("frac14", 0xBC),
    ("frac34", 0xBE),
    ("gt", 0x3E),
    ("iacute", 0xED),
    ("icirc", 0xEE),
    ("iexcl", 0xA1),
    ("igrave", 0xEC),
    ("iquest", 0xBF),
    ("iuml", 0xEF),
    ("laquo", 0xAB),
    ("lt", 0x3C),
    ("macr", 0xAF),
    ("micro", 0xB5),
    ("middot", 0xB7),
    ("nbsp", 0x20),
    ("not", 0xAC),
    ("ntilde", 0xF1),
    ("oacute", 0xF3),
    ("ocirc", 0xF4),
    ("ograve", 0xF2),
    ("ordf", 0xAA),
    ("ordm", 0xBA),
    ("oslash", 0xF8),
    ("otilde", 0xF5),
    ("ouml", 0xF6),
    ("para", 0xB6),
    ("plusmn", 0xB1),
    ("pound", 0xA3),
    ("quot", 0x22),
    ("raquo", 0xBB),
    ("reg", 0xAE),
    ("sect", 0xA7),
    ("shy", 0xAD),
    ("sup1", 0xB9),
    ("sup2", 0xB2),
    ("sup3", 0xB3),
    ("szlig", 0xDF),
    ("thorn", 0xFE),
    ("times", 0xD7),
    ("uacute", 0xFA),
    ("ucirc", 0xFB),
    ("ugrave", 0xF9),
    ("uml", 0xA8),
    ("uuml", 0xFC),
    ("yacute", 0xFD),
    ("yen", 0xA5),
    ("yuml", 0xFF),
];

/// Decodes the HTML character references in `fragment` into bytes, as decode_html in help.c
/// does before its code page conversion. A reference is `&name;`, and it is replaced when the
/// name is in [`ENTITIES`]. Anything else is copied through unchanged.
pub fn decode_entities(fragment: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(fragment.len());
    let mut pos = 0;
    while let Some(amp) = find_from(fragment, pos, b'&') {
        out.extend_from_slice(&fragment[pos..amp]);
        let start = amp + 1;
        pos = start;
        match find_from(fragment, start, b';') {
            None => out.push(b'&'),
            Some(semi) => match entity_symbol(&fragment[start..semi]) {
                Some(symbol) => {
                    out.push(symbol);
                    pos = semi + 1;
                }
                None => out.push(b'&'),
            },
        }
    }
    out.extend_from_slice(&fragment[pos..]);
    out
}

fn find_from(bytes: &[u8], from: usize, needle: u8) -> Option<usize> {
    bytes[from..]
        .iter()
        .position(|&c| c == needle)
        .map(|i| from + i)
}

/// Looks up the name between `&` and `;`, as find_html_symbol does. A numeric reference never
/// matches: help.c passes the `#` to strtol, which converts nothing, so the reference is copied
/// through as text.
fn entity_symbol(name: &[u8]) -> Option<u8> {
    if name.first() == Some(&b'#') {
        return None;
    }
    ENTITIES
        .binary_search_by(|(code, _)| code.as_bytes().cmp(name))
        .ok()
        .map(|i| ENTITIES[i].1)
}
