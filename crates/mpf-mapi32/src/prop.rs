//! Property-tag helpers from `dlls/mapi32/prop.c`, with the constants of `mapidefs.h`.
//!
//! The C search functions return a pointer into a property array. The match reads only the
//! `ulPropTag` of each element, so these functions take the tags and return an index.

/// `MV_FLAG`: the multi-valued bit of a property tag.
pub const MV_FLAG: u32 = 0x1000;
/// `PROP_TYPE_MASK`: the low 16 bits of a property tag, which hold the type.
pub const PROP_TYPE_MASK: u32 = 0xFFFF;

pub const PT_UNSPECIFIED: u32 = 0;
pub const PT_NULL: u32 = 1;
pub const PT_I2: u32 = 2;
pub const PT_LONG: u32 = 3;
pub const PT_R4: u32 = 4;
pub const PT_DOUBLE: u32 = 5;
pub const PT_CURRENCY: u32 = 6;
pub const PT_APPTIME: u32 = 7;
pub const PT_ERROR: u32 = 10;
pub const PT_BOOLEAN: u32 = 11;
pub const PT_OBJECT: u32 = 13;
pub const PT_I8: u32 = 20;
pub const PT_STRING8: u32 = 30;
pub const PT_UNICODE: u32 = 31;
pub const PT_SYSTIME: u32 = 64;
pub const PT_CLSID: u32 = 72;
pub const PT_BINARY: u32 = 258;

/// `PROP_TYPE`: the type of a property tag.
pub const fn prop_type(tag: u32) -> u32 {
    tag & PROP_TYPE_MASK
}

/// `PROP_ID`: the property id of a property tag.
pub const fn prop_id(tag: u32) -> u32 {
    tag >> 16
}

/// `FBadPropTag`. Returns `true` when the type, with the multi-valued bit removed, is not one of
/// the known types. The C code tests `ulPropTag & (~MV_FLAG & PROP_TYPE_MASK)`, so bits 13 to 15
/// also make a tag invalid.
pub fn fbad_prop_tag(tag: u32) -> bool {
    !matches!(
        tag & !MV_FLAG & PROP_TYPE_MASK,
        PT_UNSPECIFIED
            | PT_NULL
            | PT_I2
            | PT_LONG
            | PT_R4
            | PT_DOUBLE
            | PT_CURRENCY
            | PT_APPTIME
            | PT_ERROR
            | PT_BOOLEAN
            | PT_OBJECT
            | PT_I8
            | PT_STRING8
            | PT_UNICODE
            | PT_SYSTIME
            | PT_CLSID
            | PT_BINARY
    )
}

/// `FBadColumnSet`. Returns `true` when any tag is invalid, or when a tag has the exact type
/// `PT_ERROR`. A multi-valued `PT_ERROR` tag is not rejected by the type check.
pub fn fbad_column_set(tags: &[u32]) -> bool {
    tags.iter()
        .any(|&tag| prop_type(tag) == PT_ERROR || fbad_prop_tag(tag))
}

/// `PpropFindProp`. The index of the first valid tag that matches `wanted`. A tag matches when it
/// is equal, or when `wanted` has type `PT_UNSPECIFIED` and the ids are equal.
pub fn find_prop_index(tags: &[u32], wanted: u32) -> Option<usize> {
    tags.iter().position(|&tag| {
        !fbad_prop_tag(tag)
            && (tag == wanted
                || (prop_type(wanted) == PT_UNSPECIFIED && prop_id(tag) == prop_id(wanted)))
    })
}

/// `LpValFindProp`. The index of the first tag with the same property id as `wanted`. The type
/// is not checked.
pub fn lval_find_prop_index(wanted: u32, tags: &[u32]) -> Option<usize> {
    tags.iter().position(|&tag| prop_id(wanted) == prop_id(tag))
}
