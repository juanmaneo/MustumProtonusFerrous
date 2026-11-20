//! Section names and app and class names: `MSSTYLES_ParseIniSectionName` and
//! `parse_app_class_name`.

use crate::string::copy_units;

/// `MAX_THEME_APP_NAME` and `MAX_THEME_CLASS_NAME`, and the size of the part and
/// state buffers. Each name keeps at most 59 units.
const NAME_BUF: usize = 60;

/// The size of the buffer that holds a whole section name.
const SECTION_BUF: usize = 255;

/// The parts of a section name `[app:]Class[.Part][(State)]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionName {
    pub app: Vec<u16>,
    pub class: Vec<u16>,
    /// `None` when the part is absent or empty.
    pub part: Option<Vec<u16>>,
    /// `None` when the state is absent or empty.
    pub state: Option<Vec<u16>>,
}

/// Splits a section name, given without its brackets, into its parts.
///
/// A name with no class, or with a `(` that has no matching `)`, gives `None`,
/// as does a colon that is the last unit. The unit after the colon is skipped,
/// so `app:Class` gives the class `lass`, as Wine does. The part and state
/// lookup that Wine does next (`MSSTYLES_LookupPartState`) is left to the caller.
pub fn split_section_name(section: &[u16]) -> Option<SectionName> {
    let sec = copy_units(section, 0, (section.len() + 1).min(SECTION_BUF));
    let (app, comp) = match find(&sec, ':') {
        Some(colon) => (name_copy(&sec[..colon]), tail(&sec, colon + 2)),
        None => (Vec::new(), &sec[..]),
    };
    let (class, part, state) = match find(comp, '.') {
        Some(dot) => {
            let rest = &comp[dot + 1..];
            match find(rest, '(') {
                Some(open) => (
                    name_copy(&comp[..dot]),
                    Some(name_copy(&rest[..open])),
                    Some(state_in(&rest[open + 1..])?),
                ),
                None => (name_copy(&comp[..dot]), Some(name_copy(rest)), None),
            }
        }
        None => match find(comp, '(') {
            Some(open) => (
                name_copy(&comp[..open]),
                None,
                Some(state_in(&comp[open + 1..])?),
            ),
            None => (name_copy(comp), None, None),
        },
    };
    if class.is_empty() {
        return None;
    }
    Some(SectionName {
        app,
        class,
        part: part.filter(|part| !part.is_empty()),
        state: state.filter(|state| !state.is_empty()),
    })
}

/// Splits an `app::class` name at its first `::`.
///
/// Without the separator the whole name is the class and the app is empty. The
/// name ends at its first NUL.
pub fn split_app_class_name(name: &[u16]) -> (Vec<u16>, Vec<u16>) {
    let name = &name[..find(name, '\0').unwrap_or(name.len())];
    match name.windows(2).position(|pair| pair == [':' as u16; 2]) {
        Some(sep) => (name_copy(&name[..sep]), name_copy(&name[sep + 2..])),
        None => (Vec::new(), name_copy(name)),
    }
}

fn name_copy(text: &[u16]) -> Vec<u16> {
    copy_units(text, 0, NAME_BUF)
}

/// The state up to the closing parenthesis, which must be present.
fn state_in(text: &[u16]) -> Option<Vec<u16>> {
    let close = find(text, ')')?;
    Some(name_copy(&text[..close]))
}

fn find(text: &[u16], c: char) -> Option<usize> {
    text.iter().position(|&unit| unit == c as u16)
}

/// `text[index..]`, which is empty when `index` is past the end.
fn tail(text: &[u16], index: usize) -> &[u16] {
    text.get(index..).unwrap_or_default()
}
