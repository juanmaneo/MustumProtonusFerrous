/// An argument type from a `.spec` line, limited to the types `iprop.spec` uses. `long long` in
/// the spec is two `Long` entries, because `winebuild` reads one type per token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    /// `long`
    Long,
    /// `ptr`
    Ptr,
    /// `wstr`
    Wstr,
}

/// `get_ptr_size()` on i386, the only architecture `dlls/iprop/Makefile.in` enables.
const I386_PTR_SIZE: u32 = 4;

/// The stack bytes `winebuild` adds to the stdcall decoration of an export with `args`, from
/// `get_args_size` in `tools/winebuild/utils.c`. Each `long`, `ptr` and `wstr` is pointer-sized
/// on i386.
pub fn stack_bytes(args: &[ArgKind]) -> u32 {
    I386_PTR_SIZE * args.len() as u32
}
