//! The per-match expansion in `RegExp2_Replace`, `dlls/vbscript/vbregexp.c`. The C loop copies the
//! text between matches and calls this expansion for each match.

use crate::unit_at;
use std::ops::Range;

const DOLLAR: u16 = b'$' as u16;
const AMP: u16 = b'&' as u16;
const BACKTICK: u16 = b'`' as u16;
const APOSTROPHE: u16 = b'\'' as u16;

/// One match: the matched range of the source, and the range of each capture group. A group that
/// did not take part in the match is `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub whole: Range<usize>,
    pub groups: Vec<Option<Range<usize>>>,
}

fn digit(c: u16) -> Option<usize> {
    match c {
        0x30..=0x39 => Some(usize::from(c - 0x30)),
        _ => None,
    }
}

/// The group named by the `$` at `i`, as `(group number, units in the pattern)`. A two-digit
/// number wins when it names a group. Otherwise one digit names a group when it is in range.
fn group_ref(template: &[u16], i: usize, count: usize) -> Option<(usize, usize)> {
    let first = digit(unit_at(template, i + 1))?;
    if let Some(second) = digit(unit_at(template, i + 2)) {
        let n = first * 10 + second;
        if (1..=count).contains(&n) {
            return Some((n, 3));
        }
    }
    (1..=count).contains(&first).then_some((first, 2))
}

/// Appends the expansion of the `$` at `i` to `out`, and returns how many units it consumed.
fn expand_pattern(
    template: &[u16],
    i: usize,
    source: &[u16],
    m: &Match,
    out: &mut Vec<u16>,
) -> usize {
    match unit_at(template, i + 1) {
        DOLLAR => {
            out.push(DOLLAR);
            2
        }
        AMP => {
            out.extend_from_slice(&source[m.whole.clone()]);
            2
        }
        BACKTICK => {
            out.extend_from_slice(&source[..m.whole.start]);
            2
        }
        APOSTROPHE => {
            out.extend_from_slice(&source[m.whole.end..]);
            2
        }
        _ => match group_ref(template, i, m.groups.len()) {
            Some((n, len)) => {
                if let Some(range) = &m.groups[n - 1] {
                    out.extend_from_slice(&source[range.clone()]);
                }
                len
            }
            None => {
                out.push(DOLLAR);
                1
            }
        },
    }
}

/// Expands the `$` patterns of `template` for one match of `source`. The patterns are `$$`, `$&`,
/// `` $` ``, `$'`, and `$n` or `$nn` for a group. A pattern that names no group is copied as
/// text. The ranges of `m` must lie within `source`.
pub fn expand(template: &[u16], source: &[u16], m: &Match) -> Vec<u16> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < template.len() {
        if template[i] == DOLLAR {
            i += expand_pattern(template, i, source, m, &mut out);
        } else {
            out.push(template[i]);
            i += 1;
        }
    }
    out
}
