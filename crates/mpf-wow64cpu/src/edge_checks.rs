//! Edge-case tests for the ported wow64cpu logic.
//!
//! Expectations here are hand-derived from the C source or the `winnt.h` layout,
//! not from the oracle, except where a comment says otherwise. The UB rows that
//! the oracle excludes are covered here as well.

use super::*;
use crate::oracle_checks::sentinel_ctx;

/// FXSAVE image with zeroed control fields and the given status, tag byte and float registers.
fn fxsave(status_word: u16, tag_word: u8, regs: [(u64, i64); 8]) -> XsaveFormat {
    XsaveFormat {
        control_word: 0x037f,
        status_word,
        tag_word,
        reserved1: 0,
        error_opcode: 0,
        error_offset: 0,
        error_selector: 0,
        reserved2: 0,
        data_offset: 0,
        data_selector: 0,
        reserved3: 0,
        mxcsr: 0,
        mxcsr_mask: 0,
        float_registers: regs.map(|(low, high)| M128A { low, high }),
        xmm_registers: [M128A { low: 0, high: 0 }; 16],
        reserved4: [0; 96],
    }
}

/// Tag of physical register 0 when it is the only occupied register and TOP is 0.
fn tag_of(low: u64, high: i64) -> u32 {
    let mut regs = [(0u64, 0i64); 8];
    regs[0] = (low, high);
    fpux_to_fpu(&fxsave(0, 0x01, regs)).tag_word & 3
}

fn fixture64() -> Amd64Context {
    let mut regs = [(0u64, 0i64); 8];
    regs[0] = (1 << 63, 0x4000);
    Amd64Context {
        rax: 0x1111_1111_2222_2222,
        rbx: 0x3333_3333_4444_4444,
        rcx: 5,
        rdx: 6,
        rsi: 7,
        rdi: 8,
        rsp: 0x9999_0000_aaaa_bbbb,
        rbp: 0xcccc_dddd_eeee_ffff,
        rip: 0x0000_0001_4000_1000,
        eflags: 0x0000_0246,
        seg_cs: 0x33,
        seg_ss: 0x2b,
        seg_ds: 0x2b,
        seg_es: 0x2b,
        dr0: 0x0000_0001_0000_1000,
        dr1: 0x2000,
        dr2: 0x3000,
        dr3: 0x4000,
        dr6: 0x5000,
        dr7: 0x6000,
        flt_save: fxsave(0x3800, 0x01, regs),
    }
}

/// Copies `flags` from `fixture64()` with fs32 = 0x2b and ds64 = 0x53 into a sentinel context.
fn copy(flags: u32) -> I386Context {
    let mut ctx32 = sentinel_ctx();
    copy_context_64to32(&mut ctx32, flags, &fixture64(), 0x2b, 0x53);
    ctx32
}

#[test]
fn tag_valid_boundaries() {
    // Hand-derived from the C branches: integer bit set and exponent not 0 or 0x7fff.
    assert_eq!(tag_of(1 << 63, 0x0001), 0);
    assert_eq!(tag_of(u64::MAX, 0x7ffe), 0);
    assert_eq!(tag_of(1 << 63, 0x8001), 0);
}

#[test]
fn tag_special_boundaries() {
    // Exponent all ones: infinity and NaN, with and without the sign bit.
    assert_eq!(tag_of(0, 0x7fff), 2);
    assert_eq!(tag_of(1 << 63, 0xffff), 2);
    // Unnormal: exponent not 0 or 0x7fff, integer bit clear.
    assert_eq!(tag_of((1 << 63) - 1, 0x0001), 2);
    assert_eq!(tag_of(0, 0x7ffe), 2);
    // Denormal (exponent 0, mantissa non-zero) and pseudo-denormal with the integer bit set.
    assert_eq!(tag_of(1, 0), 2);
    assert_eq!(tag_of(1 << 63, 0), 2);
}

#[test]
fn tag_zero_and_negative_zero() {
    assert_eq!(tag_of(0, 0), 1);
    assert_eq!(tag_of(0, 0x8000), 1);
}

#[test]
fn empty_tag_bits_give_three() {
    let regs = [(1u64 << 63, 1i64); 8];
    assert_eq!(fpux_to_fpu(&fxsave(0, 0x00, regs)).tag_word, 0xffff_ffff);
}

#[test]
fn stack_top_rotates_the_register_index() {
    // Hand-derived: with TOP = 3, physical register 3 reads float register 0 (valid),
    // and every other physical register reads a zero float register (tag 1).
    let mut regs = [(0u64, 0i64); 8];
    regs[0] = (1 << 63, 1);
    let fpu = fpux_to_fpu(&fxsave(3 << 11, 0xff, regs));
    assert_eq!(fpu.tag_word, 0xffff_5515);
}

#[test]
fn cr0_state_is_status_word_with_high_bits_set() {
    let fpu = fpux_to_fpu(&fxsave(0x1234, 0, [(0, 0); 8]));
    assert_eq!(fpu.cr0_npx_state, 0xffff_1234);
    assert_eq!(fpu.control_word, 0x037f);
}

#[test]
fn error_selector_takes_opcode_bits_above_bit_15() {
    // Not oracle-derived: rows with ErrorOpcode bit 15 set are UB in C and are excluded from
    // the oracle. The expectation is the 32-bit pattern the C code produces in practice.
    let mut fx = fxsave(0, 0, [(0, 0); 8]);
    fx.error_opcode = 0x8001;
    fx.error_selector = 0x1234;
    assert_eq!(fpux_to_fpu(&fx).error_selector, 0x8001_1234);
    fx.error_opcode = 0xffff;
    fx.error_selector = 0xffff;
    assert_eq!(fpux_to_fpu(&fx).error_selector, 0xffff_ffff);
}

#[test]
fn to_bytes_matches_fxsave_offsets() {
    // Offsets from include/winnt.h (XSAVE_FORMAT). Not oracle-derived.
    let mut fx = fxsave(0xabcd, 0x56, [(0, 0); 8]);
    fx.control_word = 0x1234;
    fx.reserved1 = 0x78;
    fx.error_opcode = 0x9abc;
    fx.error_offset = 0xdead_beef;
    fx.error_selector = 0x1357;
    fx.reserved2 = 0x2468;
    fx.data_offset = 0x0102_0304;
    fx.data_selector = 0x0506;
    fx.reserved3 = 0x0708;
    fx.mxcsr = 0x0a0b_0c0d;
    fx.mxcsr_mask = 0x0e0f_1011;
    fx.float_registers[7] = M128A {
        low: 0x1122_3344_5566_7788,
        high: 0x99aa,
    };
    fx.xmm_registers[15] = M128A { low: 1, high: -1 };
    fx.reserved4[95] = 0x42;
    let b = fx.to_bytes();
    assert_eq!(&b[0x00..0x02], &[0x34, 0x12]);
    assert_eq!(&b[0x02..0x04], &[0xcd, 0xab]);
    assert_eq!(b[0x04], 0x56);
    assert_eq!(b[0x05], 0x78);
    assert_eq!(&b[0x06..0x08], &[0xbc, 0x9a]);
    assert_eq!(&b[0x08..0x0c], &[0xef, 0xbe, 0xad, 0xde]);
    assert_eq!(&b[0x0c..0x0e], &[0x57, 0x13]);
    assert_eq!(&b[0x0e..0x10], &[0x68, 0x24]);
    assert_eq!(&b[0x10..0x14], &[0x04, 0x03, 0x02, 0x01]);
    assert_eq!(&b[0x14..0x16], &[0x06, 0x05]);
    assert_eq!(&b[0x16..0x18], &[0x08, 0x07]);
    assert_eq!(&b[0x18..0x1c], &[0x0d, 0x0c, 0x0b, 0x0a]);
    assert_eq!(&b[0x1c..0x20], &[0x11, 0x10, 0x0f, 0x0e]);
    // Float register 7 starts at 0x20 + 7 * 16 = 0x90.
    assert_eq!(
        &b[0x90..0x98],
        &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]
    );
    assert_eq!(&b[0x98..0xa0], &[0xaa, 0x99, 0, 0, 0, 0, 0, 0]);
    // XMM register 15 starts at 0xa0 + 15 * 16 = 0x190.
    assert_eq!(b[0x190], 1);
    assert_eq!(&b[0x198..0x1a0], &[0xff; 8]);
    assert_eq!(b[0x1a0 + 95], 0x42);
    assert_eq!(b.len(), 512);
}

#[test]
fn copy_with_no_group_only_sets_context_flags() {
    let mut want = sentinel_ctx();
    want.context_flags = CONTEXT_I386;
    assert_eq!(copy(CONTEXT_I386), want);
}

#[test]
fn copy_without_i386_bit_still_selects_groups() {
    // The i386 bit is masked off before the group tests, so bare 0x2 selects INTEGER.
    let c = copy(0x0000_0002);
    assert_eq!(c.context_flags, 0x2);
    assert_eq!(c.eax, 0x2222_2222);
}

#[test]
fn integer_group_truncates_to_32_bits() {
    let c = copy(CONTEXT_I386_INTEGER);
    assert_eq!(c.eax, 0x2222_2222);
    assert_eq!(c.ebx, 0x4444_4444);
    assert_eq!(c.ecx, 5);
    assert_eq!(c.edx, 6);
    assert_eq!(c.esi, 7);
    assert_eq!(c.edi, 8);
    assert_eq!(c.esp, 0xa5a5_a5a5);
    assert_eq!(c.seg_gs, 0xa5a5_a5a5);
}

#[test]
fn control_group_copies_stack_flags_and_segments() {
    let c = copy(CONTEXT_I386_CONTROL);
    assert_eq!(c.esp, 0xaaaa_bbbb);
    assert_eq!(c.ebp, 0xeeee_ffff);
    assert_eq!(c.eip, 0x4000_1000);
    assert_eq!(c.eflags, 0x246);
    assert_eq!(c.seg_cs, 0x33);
    assert_eq!(c.seg_ss, 0x2b);
    assert_eq!(c.eax, 0xa5a5_a5a5);
    assert_eq!(c.seg_ds, 0xa5a5_a5a5);
}

#[test]
fn segments_group_uses_selector_arguments_for_fs_and_gs() {
    let c = copy(CONTEXT_I386_SEGMENTS);
    assert_eq!(c.seg_ds, 0x2b);
    assert_eq!(c.seg_es, 0x2b);
    assert_eq!(c.seg_fs, 0x2b);
    assert_eq!(c.seg_gs, 0x53);
    assert_eq!(c.seg_ss, 0xa5a5_a5a5);
}

#[test]
fn debug_group_truncates_registers_to_32_bits() {
    let c = copy(CONTEXT_I386_DEBUG_REGISTERS);
    assert_eq!(c.dr0, 0x1000);
    assert_eq!(c.dr1, 0x2000);
    assert_eq!(c.dr2, 0x3000);
    assert_eq!(c.dr3, 0x4000);
    assert_eq!(c.dr6, 0x5000);
    assert_eq!(c.dr7, 0x6000);
    assert_eq!(c.eax, 0xa5a5_a5a5);
}

#[test]
fn floating_point_group_converts_the_fxsave_state() {
    let c = copy(CONTEXT_I386_FLOATING_POINT);
    assert_eq!(c.float_save, fpux_to_fpu(&fixture64().flt_save));
    assert_eq!(c.float_save.status_word, 0x3800);
    assert_eq!(c.extended_registers, [0xa5; 512]);
}

#[test]
fn extended_group_copies_the_fxsave_image() {
    let c = copy(CONTEXT_I386_EXTENDED_REGISTERS);
    assert_eq!(c.extended_registers, fixture64().flt_save.to_bytes());
    assert_eq!(&c.extended_registers[0..2], &[0x7f, 0x03]);
    assert_eq!(c.float_save, sentinel_ctx().float_save);
}
