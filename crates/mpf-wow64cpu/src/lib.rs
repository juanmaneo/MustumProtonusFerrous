//! Port of the portable logic in Wine's `dlls/wow64cpu/cpu.c`.
//!
//! Ported:
//! - `fpux_to_fpu`: FXSAVE x87/SSE state to the 32-bit `I386_FLOATING_SAVE_AREA`,
//!   with the full tag word rebuilt from the abridged tag byte: [`fpux_to_fpu`]
//! - `copy_context_64to32`: flag-gated copy of a 64-bit context into the 32-bit
//!   WOW64 context, including the FXSAVE image in `ExtendedRegisters`: [`copy_context_64to32`]
//!
//! Skipped:
//! - `syscall_32to64`, `unix_call_32to64`, `BTCpuSimulate`, `BTCpuGetBopCode`,
//!   `__wine_get_unix_opcode`: hand-written machine-code thunks and their buffer.
//! - `BTCpuProcessInit`, `BTCpuGetContext`, `BTCpuSetContext`, `BTCpuResetToConsistentState`:
//!   process, thread and exception state, which needs NT calls and the live thread context.
//! - `BTCpuIsProcessorFeaturePresent`: forwards to `RtlIsProcessorFeaturePresent`.
//! - `BTCpuTurboThunkControl`: returns a constant status (stub).
//! - `DllMain`, and the `FIXME: xstate` path (`CONTEXT_I386_XSTATE` is not copied).

mod fpu;

pub use fpu::{fpux_to_fpu, I386FloatingSaveArea, XsaveFormat, M128A};

pub const CONTEXT_I386: u32 = 0x0001_0000;
pub const CONTEXT_I386_CONTROL: u32 = CONTEXT_I386 | 0x0001;
pub const CONTEXT_I386_INTEGER: u32 = CONTEXT_I386 | 0x0002;
pub const CONTEXT_I386_SEGMENTS: u32 = CONTEXT_I386 | 0x0004;
pub const CONTEXT_I386_FLOATING_POINT: u32 = CONTEXT_I386 | 0x0008;
pub const CONTEXT_I386_DEBUG_REGISTERS: u32 = CONTEXT_I386 | 0x0010;
pub const CONTEXT_I386_EXTENDED_REGISTERS: u32 = CONTEXT_I386 | 0x0020;

/// `I386_CONTEXT`, limited to the fields `copy_context_64to32` writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I386Context {
    pub context_flags: u32,
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
    pub dr6: u32,
    pub dr7: u32,
    pub float_save: I386FloatingSaveArea,
    pub seg_gs: u32,
    pub seg_fs: u32,
    pub seg_es: u32,
    pub seg_ds: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub ebp: u32,
    pub eip: u32,
    pub seg_cs: u32,
    pub eflags: u32,
    pub esp: u32,
    pub seg_ss: u32,
    pub extended_registers: [u8; 512],
}

/// `AMD64_CONTEXT`, limited to the fields `copy_context_64to32` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Amd64Context {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rip: u64,
    pub eflags: u32,
    pub seg_cs: u16,
    pub seg_ss: u16,
    pub seg_ds: u16,
    pub seg_es: u16,
    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr6: u64,
    pub dr7: u64,
    pub flt_save: XsaveFormat,
}

/// `copy_context_64to32`: copies the parts of a 64-bit context selected by `flags`
/// into the 32-bit context. Fields outside the selected groups are left untouched.
pub fn copy_context_64to32(
    ctx32: &mut I386Context,
    flags: u32,
    ctx64: &Amd64Context,
    fs32_sel: u16,
    ds64_sel: u16,
) {
    ctx32.context_flags = flags;
    let flags = flags & !CONTEXT_I386;
    if (flags & CONTEXT_I386_INTEGER) != 0 {
        ctx32.eax = ctx64.rax as u32;
        ctx32.ebx = ctx64.rbx as u32;
        ctx32.ecx = ctx64.rcx as u32;
        ctx32.edx = ctx64.rdx as u32;
        ctx32.esi = ctx64.rsi as u32;
        ctx32.edi = ctx64.rdi as u32;
    }
    if (flags & CONTEXT_I386_CONTROL) != 0 {
        ctx32.esp = ctx64.rsp as u32;
        ctx32.ebp = ctx64.rbp as u32;
        ctx32.eip = ctx64.rip as u32;
        ctx32.eflags = ctx64.eflags;
        ctx32.seg_cs = u32::from(ctx64.seg_cs);
        ctx32.seg_ss = u32::from(ctx64.seg_ss);
    }
    if (flags & CONTEXT_I386_SEGMENTS) != 0 {
        ctx32.seg_ds = u32::from(ctx64.seg_ds);
        ctx32.seg_es = u32::from(ctx64.seg_es);
        ctx32.seg_fs = u32::from(fs32_sel);
        ctx32.seg_gs = u32::from(ds64_sel);
    }
    if (flags & CONTEXT_I386_DEBUG_REGISTERS) != 0 {
        ctx32.dr0 = ctx64.dr0 as u32;
        ctx32.dr1 = ctx64.dr1 as u32;
        ctx32.dr2 = ctx64.dr2 as u32;
        ctx32.dr3 = ctx64.dr3 as u32;
        ctx32.dr6 = ctx64.dr6 as u32;
        ctx32.dr7 = ctx64.dr7 as u32;
    }
    if (flags & CONTEXT_I386_FLOATING_POINT) != 0 {
        ctx32.float_save = fpux_to_fpu(&ctx64.flt_save);
    }
    if (flags & CONTEXT_I386_EXTENDED_REGISTERS) != 0 {
        ctx32.extended_registers = ctx64.flt_save.to_bytes();
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
