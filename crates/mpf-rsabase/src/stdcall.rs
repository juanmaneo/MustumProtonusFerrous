/// The argument types of `enum arg_type` in `tools/winebuild/build.h`.
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

/// The CPUs of `enum target_cpu` in `tools/tools.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu {
    I386,
    X86_64,
    Arm,
    Arm64,
    Arm64ec,
}

/// `get_target_ptr_size` in `tools/tools.h`, which `get_ptr_size` in `tools/winebuild/build.h` returns.
fn ptr_size(cpu: Cpu) -> u32 {
    match cpu {
        Cpu::I386 | Cpu::Arm => 4,
        Cpu::X86_64 | Cpu::Arm64 | Cpu::Arm64ec => 8,
    }
}

/// `get_args_size` in `tools/winebuild/utils.c`: the stdcall argument bytes for `args` on `cpu`.
/// `int64` and `double` take 8 bytes, and only on ARM does each start on an 8-byte boundary.
/// `int128` takes 16 bytes, except on x86_64 where it is passed by pointer. Every other type
/// takes one pointer.
pub fn args_size(args: &[ArgType], cpu: Cpu) -> u32 {
    let mut size = 0;
    for &arg in args {
        match arg {
            ArgType::Int64 | ArgType::Double => {
                if cpu == Cpu::Arm {
                    size = (size + 7) & !7;
                }
                size += 8;
            }
            ArgType::Int128 if cpu != Cpu::X86_64 => size += 16,
            _ => size += ptr_size(cpu),
        }
    }
    size
}
