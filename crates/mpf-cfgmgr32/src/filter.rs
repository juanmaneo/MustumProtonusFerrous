//! The filter engine of `devobject.c`: `devprop_filters_validate` and
//! `devprop_filter_matches_properties`.

use crate::devprop::{type_validate, DEVPROP_TYPE_EMPTY, DEVPROP_TYPE_GUID, DEVPROP_TYPE_STRING};
use crate::{comp_key_eq, Filter, Property};
use std::cmp::Ordering;

pub const DEVPROP_OPERATOR_NONE: u32 = 0x0000_0000;
pub const DEVPROP_OPERATOR_EXISTS: u32 = 0x0000_0001;
pub const DEVPROP_OPERATOR_NOT_EXISTS: u32 =
    DEVPROP_OPERATOR_MODIFIER_NOT | DEVPROP_OPERATOR_EXISTS;
pub const DEVPROP_OPERATOR_EQUALS: u32 = 0x0000_0002;
pub const DEVPROP_OPERATOR_GREATER_THAN: u32 = 0x0000_0003;
pub const DEVPROP_OPERATOR_LESS_THAN: u32 = 0x0000_0004;
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: u32 = 0x0000_0005;
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: u32 = 0x0000_0006;
pub const DEVPROP_OPERATOR_BITWISE_AND: u32 = 0x0000_0007;
pub const DEVPROP_OPERATOR_BITWISE_OR: u32 = 0x0000_0008;
pub const DEVPROP_OPERATOR_BEGINS_WITH: u32 = 0x0000_0009;
pub const DEVPROP_OPERATOR_ENDS_WITH: u32 = 0x0000_000a;
pub const DEVPROP_OPERATOR_CONTAINS: u32 = 0x0000_000b;
pub const DEVPROP_OPERATOR_MODIFIER_NOT: u32 = 0x0001_0000;
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: u32 = 0x0002_0000;
pub const DEVPROP_OPERATOR_NOT_EQUALS: u32 =
    DEVPROP_OPERATOR_MODIFIER_NOT | DEVPROP_OPERATOR_EQUALS;
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: u32 =
    DEVPROP_OPERATOR_EQUALS | DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE;
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: u32 =
    DEVPROP_OPERATOR_MODIFIER_NOT | DEVPROP_OPERATOR_EQUALS | DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE;
pub const DEVPROP_OPERATOR_LIST_CONTAINS: u32 = 0x0000_1000;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: u32 = 0x0000_2000;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: u32 = 0x0000_3000;
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: u32 = 0x0000_4000;
pub const DEVPROP_OPERATOR_AND_OPEN: u32 = 0x0010_0000;
pub const DEVPROP_OPERATOR_AND_CLOSE: u32 = 0x0020_0000;
pub const DEVPROP_OPERATOR_OR_OPEN: u32 = 0x0030_0000;
pub const DEVPROP_OPERATOR_OR_CLOSE: u32 = 0x0040_0000;
pub const DEVPROP_OPERATOR_NOT_OPEN: u32 = 0x0050_0000;
pub const DEVPROP_OPERATOR_NOT_CLOSE: u32 = 0x0060_0000;
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: u32 = 0x1000_0000;
pub const DEVPROP_OPERATOR_MASK_EVAL: u32 = 0x0000_0fff;
pub const DEVPROP_OPERATOR_MASK_LIST: u32 = 0x0000_f000;
pub const DEVPROP_OPERATOR_MASK_MODIFIER: u32 = 0x000f_0000;
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: u32 = 0xf00f_ffff;
pub const DEVPROP_OPERATOR_MASK_LOGICAL: u32 = 0x0ff0_0000;
pub const DEVPROP_OPERATOR_MASK_ARRAY: u32 = 0xf000_0000;

/// The logical group that a `*_OPEN` operator starts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Group {
    Not,
    And,
    Or,
}

impl Group {
    fn of(operator: u32) -> Self {
        match operator & DEVPROP_OPERATOR_MASK_LOGICAL {
            DEVPROP_OPERATOR_NOT_OPEN => Group::Not,
            DEVPROP_OPERATOR_OR_OPEN => Group::Or,
            _ => Group::And,
        }
    }
}

/// `devprop_filters_validate`: whether `filters` is a well-formed filter list. An empty
/// list is invalid. Each group must close, and the groups nest by their own kind.
pub fn filters_validate(filters: &[Filter]) -> bool {
    if filters.is_empty() {
        return false;
    }
    let mut i = 0;
    while i < filters.len() {
        let filter = &filters[i];
        let operator = filter.operator;
        let property = &filter.property;
        let compare = operator & DEVPROP_OPERATOR_MASK_EVAL;
        let list = operator & DEVPROP_OPERATOR_MASK_LIST;
        let modifier = operator & DEVPROP_OPERATOR_MASK_MODIFIER;
        let logical = operator & DEVPROP_OPERATOR_MASK_LOGICAL;
        let array = operator & DEVPROP_OPERATOR_MASK_ARRAY;

        if compare > DEVPROP_OPERATOR_CONTAINS
            || (logical != 0 && operator & DEVPROP_OPERATOR_MASK_NOT_LOGICAL != 0)
            || (array != 0 && operator != DEVPROP_OPERATOR_ARRAY_CONTAINS)
            || property.buffer.is_some() != (property.size() != 0)
        {
            return false;
        }
        if compare != 0
            && compare != DEVPROP_OPERATOR_EXISTS
            && !type_validate(property.ty, property.size())
        {
            return false;
        }
        if !matches!(
            modifier,
            0 | DEVPROP_OPERATOR_MODIFIER_NOT | DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE
        ) {
            return false;
        }
        if !matches!(
            list,
            0 | DEVPROP_OPERATOR_LIST_CONTAINS
                | DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH
                | DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH
                | DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS
        ) {
            return false;
        }
        let mut step = 1;
        match logical {
            0 => {}
            DEVPROP_OPERATOR_AND_OPEN | DEVPROP_OPERATOR_OR_OPEN | DEVPROP_OPERATOR_NOT_OPEN => {
                let inner = &filters[i + 1..];
                let Some(close) = find_closing(logical, inner) else {
                    return false;
                };
                if !filters_validate(&inner[..close]) {
                    return false;
                }
                step = close + 2;
            }
            _ => return false,
        }
        i += step;
    }
    true
}

/// `devprop_filter_matches_properties` with `DEVPROP_OPERATOR_AND_OPEN` as the outer
/// operator: whether the properties satisfy all of `filters`.
///
/// `filters` must pass `filters_validate`, or be empty (no filter, which matches).
/// Other input gives an unspecified result, but the function does not panic.
pub fn filters_match(props: &[Property], filters: &[Filter]) -> bool {
    match_group(props, Group::And, filters)
}

fn match_group(props: &[Property], group: Group, filters: &[Filter]) -> bool {
    let mut ret = true;
    let mut i = 0;
    while i < filters.len() {
        let operator = filters[i].operator;
        let mut step = 1;
        if operator == DEVPROP_OPERATOR_NONE
            || operator & (DEVPROP_OPERATOR_MASK_LIST | DEVPROP_OPERATOR_MASK_ARRAY) != 0
        {
            ret = false;
        } else if operator & DEVPROP_OPERATOR_MASK_LOGICAL != 0 {
            let inner = &filters[i + 1..];
            let close = find_closing(operator & DEVPROP_OPERATOR_MASK_LOGICAL, inner)
                .unwrap_or(inner.len());
            ret = match_group(props, Group::of(operator), &inner[..close]);
            step = close + 2;
        } else if operator & DEVPROP_OPERATOR_MASK_EVAL != 0 {
            ret = eval_compare(props, &filters[i]);
        }
        let stop = match group {
            Group::Not | Group::And => !ret,
            Group::Or => ret,
        };
        if stop {
            break;
        }
        i += step;
    }
    if group == Group::Not {
        !ret
    } else {
        ret
    }
}

/// `find_closing_filter`: the index in `after` of the close for a group opened by
/// `open`, where `after` starts just past the opener. Nested groups of the same kind
/// are skipped. `None` when the group never closes.
fn find_closing(open: u32, after: &[Filter]) -> Option<usize> {
    let close = open + (DEVPROP_OPERATOR_AND_CLOSE - DEVPROP_OPERATOR_AND_OPEN);
    let mut depth = 0usize;
    for (i, filter) in after.iter().enumerate() {
        let logical = filter.operator & DEVPROP_OPERATOR_MASK_LOGICAL;
        if logical == close {
            if depth == 0 {
                return Some(i);
            }
            depth -= 1;
        } else if logical == open {
            depth += 1;
        }
    }
    None
}

/// `devprop_filter_eval_compare`: the result of one comparison filter against the
/// properties. The lookup is by key, and the comparison needs equal type and size.
fn eval_compare(props: &[Property], filter: &Filter) -> bool {
    let operator = filter.operator;
    let cmp_prop = &filter.property;
    let found = props
        .iter()
        .find(|prop| comp_key_eq(&prop.comp_key, &cmp_prop.comp_key));
    let result = match operator & DEVPROP_OPERATOR_MASK_EVAL {
        DEVPROP_OPERATOR_EXISTS => found.is_some_and(|prop| prop.ty != DEVPROP_TYPE_EMPTY),
        DEVPROP_OPERATOR_EQUALS
        | DEVPROP_OPERATOR_LESS_THAN
        | DEVPROP_OPERATOR_GREATER_THAN
        | DEVPROP_OPERATOR_LESS_THAN_EQUALS
        | DEVPROP_OPERATOR_GREATER_THAN_EQUALS => match found {
            Some(prop) if prop.ty == cmp_prop.ty && prop.size() == cmp_prop.size() => {
                ordering_holds(operator, prop, cmp_prop)
            }
            _ => false,
        },
        _ => return false,
    };
    if operator & DEVPROP_OPERATOR_MODIFIER_NOT != 0 {
        !result
    } else {
        result
    }
}

/// Applies the operator's bit tests to the ordering of `prop` against `cmp_prop`. The
/// tests are the C ones, so `GREATER_THAN` tests the `EQUALS` bit and
/// `GREATER_THAN_EQUALS` tests the `LESS_THAN` bit.
fn ordering_holds(operator: u32, prop: &Property, cmp_prop: &Property) -> bool {
    let (a, b) = (
        prop.buffer.unwrap_or_default(),
        cmp_prop.buffer.unwrap_or_default(),
    );
    let order = match prop.ty {
        DEVPROP_TYPE_STRING => {
            wide_cmp(a, b, operator & DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE != 0)
        }
        DEVPROP_TYPE_GUID if operator & DEVPROP_OPERATOR_EQUALS == 0 => Ordering::Equal,
        _ => a.cmp(b),
    };
    if operator & DEVPROP_OPERATOR_EQUALS != 0 {
        order == Ordering::Equal
    } else {
        // Only LESS_THAN and GREATER_THAN_EQUALS reach here without the EQUALS bit, and both
        // have the LESS_THAN bit set.
        order == Ordering::Less
    }
}

/// `wcscmp`, or `wcsicmp` when `ignore_case` is set, over UTF-16LE units. Reading stops at
/// the first NUL unit, or at the end of the buffer when there is none.
fn wide_cmp(a: &[u8], b: &[u8], ignore_case: bool) -> Ordering {
    let mut a = wide_units(a);
    let mut b = wide_units(b);
    loop {
        let x = fold(a.next().unwrap_or(0), ignore_case);
        let y = fold(b.next().unwrap_or(0), ignore_case);
        if x != y {
            return x.cmp(&y);
        }
        if x == 0 {
            return Ordering::Equal;
        }
    }
}

fn wide_units(bytes: &[u8]) -> impl Iterator<Item = u16> + '_ {
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|&[lo, hi]| u16::from_le_bytes([lo, hi]))
        .take_while(|&unit| unit != 0)
}

/// Folds ASCII upper case to lower case, as the oracle's `wcsicmp` does.
fn fold(unit: u16, ignore_case: bool) -> u16 {
    if ignore_case && (u16::from(b'A')..=u16::from(b'Z')).contains(&unit) {
        unit + 0x20
    } else {
        unit
    }
}
