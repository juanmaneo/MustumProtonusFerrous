use crate::args::stack_bytes_i386;
use crate::spec::Export;

/// The target CPU. It decides whether winebuild decorates a stdcall name with its stack size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arch {
    I386,
    X86_64,
}

/// The export line that `winebuild --def` writes for `export` on `arch`. On i386 the name carries
/// its stdcall stack size, as in `IcmpCloseHandle@4`. The forward target follows an equals sign,
/// and the ordinal comes last.
pub fn def_line(export: &Export<'_>, arch: Arch) -> String {
    let decoration = match arch {
        Arch::I386 => format!("@{}", stack_bytes_i386(&export.args)),
        Arch::X86_64 => String::new(),
    };
    format!(
        "  {}{}={} @{}",
        export.name, decoration, export.forward, export.ordinal
    )
}
