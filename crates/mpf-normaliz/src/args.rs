/// An argument type from a `.spec` line, limited to the types `normaliz.spec` uses. `winebuild`
/// reads one type per whitespace token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    /// `long`
    Long,
    /// `ptr`
    Ptr,
    /// `wstr`
    Wstr,
}

/// `get_ptr_size()` on i386, the only architecture whose stdcall names carry a stack suffix.
const I386_PTR_SIZE: u32 = 4;

/// The stack bytes `winebuild` adds to the stdcall decoration of an export with `args`, from
/// `get_args_size` in `tools/winebuild/utils.c`. Each `long`, `ptr` and `wstr` takes one
/// pointer-sized slot on i386.
pub fn stack_bytes(args: &[ArgKind]) -> u32 {
    I386_PTR_SIZE * args.len() as u32
}
