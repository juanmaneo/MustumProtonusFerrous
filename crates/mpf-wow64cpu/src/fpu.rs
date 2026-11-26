//! The FXSAVE (`XSAVE_FORMAT`) layout and `fpux_to_fpu` from `dlls/wow64cpu/cpu.c`.
//! Field offsets follow `include/winnt.h`.

const TAG_VALID: u32 = 0;
const TAG_ZERO: u32 = 1;
const TAG_SPECIAL: u32 = 2;
const TAG_EMPTY: u32 = 3;

/// `M128A`: one 80-bit x87 register is stored in the low 10 bytes of this.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct M128A {
    pub low: u64,
    pub high: i64,
}

impl M128A {
    fn to_bytes(self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[..8].copy_from_slice(&self.low.to_le_bytes());
        out[8..].copy_from_slice(&self.high.to_le_bytes());
        out
    }
}

/// `XSAVE_FORMAT` (`XMM_SAVE_AREA32`): the 512-byte FXSAVE image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XsaveFormat {
    pub control_word: u16,
    pub status_word: u16,
    /// Abridged tag byte: one bit per physical register, set when occupied.
    pub tag_word: u8,
    pub reserved1: u8,
    pub error_opcode: u16,
    pub error_offset: u32,
    pub error_selector: u16,
    pub reserved2: u16,
    pub data_offset: u32,
    pub data_selector: u16,
    pub reserved3: u16,
    pub mxcsr: u32,
    pub mxcsr_mask: u32,
    pub float_registers: [M128A; 8],
    pub xmm_registers: [M128A; 16],
    pub reserved4: [u8; 96],
}

impl XsaveFormat {
    /// The 512 bytes the struct occupies in memory (little-endian), as `*(XSAVE_FORMAT *)` copies them.
    pub fn to_bytes(&self) -> [u8; 512] {
        let mut out = [0u8; 512];
        put(&mut out, 0x00, &self.control_word.to_le_bytes());
        put(&mut out, 0x02, &self.status_word.to_le_bytes());
        put(&mut out, 0x04, &[self.tag_word, self.reserved1]);
        put(&mut out, 0x06, &self.error_opcode.to_le_bytes());
        put(&mut out, 0x08, &self.error_offset.to_le_bytes());
        put(&mut out, 0x0c, &self.error_selector.to_le_bytes());
        put(&mut out, 0x0e, &self.reserved2.to_le_bytes());
        put(&mut out, 0x10, &self.data_offset.to_le_bytes());
        put(&mut out, 0x14, &self.data_selector.to_le_bytes());
        put(&mut out, 0x16, &self.reserved3.to_le_bytes());
        put(&mut out, 0x18, &self.mxcsr.to_le_bytes());
        put(&mut out, 0x1c, &self.mxcsr_mask.to_le_bytes());
        for (i, reg) in self.float_registers.iter().enumerate() {
            put(&mut out, 0x20 + 16 * i, &reg.to_bytes());
        }
        for (i, reg) in self.xmm_registers.iter().enumerate() {
            put(&mut out, 0xa0 + 16 * i, &reg.to_bytes());
        }
        put(&mut out, 0x1a0, &self.reserved4);
        out
    }
}

fn put(out: &mut [u8; 512], offset: usize, bytes: &[u8]) {
    out[offset..offset + bytes.len()].copy_from_slice(bytes);
}

/// `I386_FLOATING_SAVE_AREA`, the x87 state of a 32-bit context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I386FloatingSaveArea {
    pub control_word: u32,
    pub status_word: u32,
    pub tag_word: u32,
    pub error_offset: u32,
    pub error_selector: u32,
    pub data_offset: u32,
    pub data_selector: u32,
    pub register_area: [u8; 80],
    pub cr0_npx_state: u32,
}

/// `fpux_to_fpu`: converts an FXSAVE image to the 32-bit x87 save area.
///
/// The FXSAVE tag byte only says whether each register is occupied. The full
/// 16-bit tag word is rebuilt from the register contents: 0 valid, 1 zero,
/// 2 special (NaN, infinity, denormal or unnormal), 3 empty.
pub fn fpux_to_fpu(fpux: &XsaveFormat) -> I386FloatingSaveArea {
    let status = u32::from(fpux.status_word);
    let stack_top = ((status >> 11) & 7) as usize;
    let mut fpu = I386FloatingSaveArea {
        control_word: u32::from(fpux.control_word),
        status_word: status,
        tag_word: 0xffff_0000,
        error_offset: fpux.error_offset,
        // `ErrorOpcode << 16` is taken as the 32-bit pattern, which is what the C code produces.
        error_selector: u32::from(fpux.error_selector) | (u32::from(fpux.error_opcode) << 16),
        data_offset: fpux.data_offset,
        data_selector: u32::from(fpux.data_selector),
        register_area: [0; 80],
        cr0_npx_state: status | 0xffff_0000,
    };
    for (i, reg) in fpux.float_registers.iter().enumerate() {
        fpu.register_area[10 * i..10 * i + 10].copy_from_slice(&reg.to_bytes()[..10]);
        let tag = if (fpux.tag_word & (1 << i)) == 0 {
            TAG_EMPTY
        } else {
            classify_tag(&fpux.float_registers[i.wrapping_sub(stack_top) & 7])
        };
        fpu.tag_word |= tag << (2 * i);
    }
    fpu
}

fn classify_tag(reg: &M128A) -> u32 {
    let exponent = reg.high & 0x7fff;
    if exponent == 0x7fff {
        TAG_SPECIAL
    } else if exponent == 0 {
        if reg.low != 0 {
            TAG_SPECIAL
        } else {
            TAG_ZERO
        }
    } else if (reg.low >> 63) != 0 {
        TAG_VALID
    } else {
        TAG_SPECIAL
    }
}
