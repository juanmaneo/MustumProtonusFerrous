//! The procedure hash table and the `FUNCDESC` fields read by `msscript.c`.

/// The bucket count of `ScriptProcedureCollection::hash_table`, `struct list hash_table[43]`.
pub const HASH_BUCKETS: u32 = 43;

/// `VT_VOID`, the return type of a procedure with no return value.
pub const VT_VOID: u16 = 24;

/// A procedure already in a bucket: its hash and its UTF-16 name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'a> {
    pub hash: u32,
    pub name: &'a [u16],
}

/// The bucket for `hash`, as `hash % ARRAY_SIZE(hash_table)`.
pub fn bucket_index(hash: u32) -> usize {
    (hash % HASH_BUCKETS) as usize
}

/// The first entry in `bucket` whose hash and name both match, in list order. A match needs the
/// same hash and the same name length and characters.
pub fn find_in_bucket(bucket: &[Entry<'_>], hash: u32, name: &[u16]) -> Option<usize> {
    bucket
        .iter()
        .position(|entry| entry.hash == hash && entry.name == name)
}

/// `proc->num_args = desc->cParams + desc->cParamsOpt`, stored as a `USHORT`.
pub fn num_args(c_params: i16, c_params_opt: i16) -> u16 {
    (i32::from(c_params) + i32::from(c_params_opt)) as u16
}

/// `ScriptProcedure_get_HasReturnValue`: a procedure has a return value unless its return type is
/// `VT_VOID`.
pub fn has_return_value(ret_type: u16) -> bool {
    ret_type != VT_VOID
}
