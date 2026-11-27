/// One enumerator of a typelib enum: its IDL name and its value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Member {
    pub name: &'static str,
    pub value: i32,
}

/// Declares the constants of one `typedef enum` from `dlls/msxml2/msxml2_tlb.idl`, then the
/// member table $table that lists them in IDL order.
macro_rules! typelib_enum {
    ($table:ident { $($member:ident = $value:expr,)+ }) => {
        $(pub const $member: i32 = $value;)+

        pub const $table: &[Member] = &[$(Member { name: stringify!($member), value: $member }),+];
    };
}

typelib_enum!(DOM_NODE_TYPE {
    NODE_INVALID = 0,
    NODE_ELEMENT = 1,
    NODE_ATTRIBUTE = 2,
    NODE_TEXT = 3,
    NODE_CDATA_SECTION = 4,
    NODE_ENTITY_REFERENCE = 5,
    NODE_ENTITY = 6,
    NODE_PROCESSING_INSTRUCTION = 7,
    NODE_COMMENT = 8,
    NODE_DOCUMENT = 9,
    NODE_DOCUMENT_TYPE = 10,
    NODE_DOCUMENT_FRAGMENT = 11,
    NODE_NOTATION = 12,
});

/// Returns the value of the member called `name` in `members`, or `None` if there is none.
pub fn member_value(members: &[Member], name: &str) -> Option<i32> {
    members
        .iter()
        .find(|member| member.name == name)
        .map(|member| member.value)
}

/// Returns the name of the first member of `members` with the value `value`, or `None` if no
/// member has that value.
pub fn member_name(members: &[Member], value: i32) -> Option<&'static str> {
    members
        .iter()
        .find(|member| member.value == value)
        .map(|member| member.name)
}
