/// The parameter types that winebuild accepts in a spec line, from `enum arg_type` in
/// `tools/winebuild/build.h`.
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

/// The target CPU. It sets the pointer size that `get_args_size` uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu {
    I386,
    X86_64,
}

impl Cpu {
    /// `get_target_ptr_size` in `tools/tools.h`, for the two CPUs this crate covers.
    pub fn ptr_size(self) -> u32 {
        match self {
            Cpu::I386 => 4,
            Cpu::X86_64 => 8,
        }
    }
}

/// `get_args_size` in `tools/winebuild/utils.c`: the argument bytes that a `stdcall` function
/// pops on `cpu`.
pub fn stdcall_arg_bytes(args: &[ArgType], cpu: Cpu) -> u32 {
    args.iter()
        .map(|arg| match arg {
            ArgType::Int64 | ArgType::Double => 8,
            // int128 is passed as a pointer on x86_64, and takes 16 bytes elsewhere.
            ArgType::Int128 => match cpu {
                Cpu::I386 => 16,
                Cpu::X86_64 => cpu.ptr_size(),
            },
            _ => cpu.ptr_size(),
        })
        .sum()
}
