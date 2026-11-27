/// A declared argument type of a spec entry. The names are the `ArgNames` table in
/// `tools/winebuild/parser.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgType {
    Word,
    SWord,
    SegPtr,
    SegStr,
    Long,
    Ptr,
    Str,
    WStr,
    Int64,
    Int128,
    Float,
    Double,
}

const ARG_NAMES: [(&str, ArgType); 12] = [
    ("word", ArgType::Word),
    ("s_word", ArgType::SWord),
    ("segptr", ArgType::SegPtr),
    ("segstr", ArgType::SegStr),
    ("long", ArgType::Long),
    ("ptr", ArgType::Ptr),
    ("str", ArgType::Str),
    ("wstr", ArgType::WStr),
    ("int64", ArgType::Int64),
    ("int128", ArgType::Int128),
    ("float", ArgType::Float),
    ("double", ArgType::Double),
];

/// The type for an argument name in any spec, or `None` for a name winebuild does not know.
/// Names are compared case-sensitively, as `strcmp` does.
pub fn from_name(name: &str) -> Option<ArgType> {
    ARG_NAMES
        .iter()
        .find(|(known, _)| *known == name)
        .map(|&(_, arg)| arg)
}

impl ArgType {
    /// The types that winebuild only accepts in 16-bit specs.
    fn is_win16_only(self) -> bool {
        matches!(self, Self::Word | Self::SWord | Self::SegPtr | Self::SegStr)
    }
}

/// The type for an argument name in a 32-bit spec. Returns `None` for an unknown name and for a
/// 16-bit-only type, which winebuild rejects in a 32-bit spec.
pub fn parse_win32_arg(name: &str) -> Option<ArgType> {
    from_name(name).filter(|arg| !arg.is_win16_only())
}

/// The bytes the arguments take on the i386 stack, counted as `get_args_size` in
/// `tools/winebuild/utils.c` counts them.
pub fn stack_bytes_i386(args: &[ArgType]) -> u32 {
    args.iter()
        .map(|arg| match arg {
            ArgType::Int64 | ArgType::Double => 8,
            ArgType::Int128 => 16,
            // The pointer size on i386.
            _ => 4,
        })
        .sum()
}
