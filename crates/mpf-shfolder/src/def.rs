use crate::exports::{ArgType, Export, EXPORTS};

/// The DLL name, from `MODULE = shfolder.dll` in `dlls/shfolder/Makefile.in`. winebuild writes it as `LIBRARY`.
pub const MODULE_NAME: &str = "shfolder.dll";

const SPEC_FILE_NAME: &str = "shfolder.spec";

/// The targets `winebuild` builds `shfolder.spec` for, named by their `-b` CPU.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu {
    I386,
    X86_64,
    Arm,
    Arm64,
}

impl Cpu {
    /// `get_target_ptr_size` in `tools/tools.h`.
    pub fn ptr_size(self) -> u32 {
        match self {
            Cpu::I386 | Cpu::Arm => 4,
            Cpu::X86_64 | Cpu::Arm64 => 8,
        }
    }

    /// `output_def_file` in `tools/winebuild/spec32.c` appends `@<bytes>` only for i386, with `kill_at` off.
    fn decorates_stdcall(self) -> bool {
        self == Cpu::I386
    }
}

/// `get_args_size` in `tools/winebuild/utils.c`. Every `long` and `ptr` argument takes one pointer-sized slot.
pub fn get_args_size(args: &[ArgType], cpu: Cpu) -> u32 {
    args.len() as u32 * cpu.ptr_size()
}

/// The `EXPORTS` line that `winebuild --def` writes for `export` on `cpu`.
pub fn def_line(export: &Export, cpu: Cpu) -> String {
    let decoration = if cpu.decorates_stdcall() {
        format!("@{}", get_args_size(export.args, cpu))
    } else {
        String::new()
    };
    format!(
        "  {}{}={} @{}",
        export.name, decoration, export.forward, export.ordinal
    )
}

/// The whole `.def` file that `winebuild --def -E shfolder.spec` writes for `cpu`.
pub fn def_file(cpu: Cpu) -> String {
    let mut out = format!(
        "; File generated automatically from {SPEC_FILE_NAME}; do not edit!\n\nLIBRARY {MODULE_NAME}\n\nEXPORTS\n"
    );
    for export in EXPORTS.iter() {
        out.push_str(&def_line(export, cpu));
        out.push('\n');
    }
    out
}
