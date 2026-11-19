//! Bytecode writer token mappings to D3D9 values, from Wine's
//! `dlls/d3dcompiler_43/bytecodewriter.c`.

// BEGIN GENERATED: consts-token
pub const D3DSTT_UNKNOWN: u32 = 0x0;
pub const D3DSTT_1D: u32 = 0x8000000;
pub const D3DSTT_2D: u32 = 0x10000000;
pub const D3DSTT_CUBE: u32 = 0x18000000;
pub const D3DSTT_VOLUME: u32 = 0x20000000;
pub const D3DSP_REGNUM_MASK: u32 = 0x7ff;
pub const D3DSP_WRITEMASK_0: u32 = 0x10000;
pub const D3DSP_WRITEMASK_1: u32 = 0x20000;
pub const D3DSP_WRITEMASK_2: u32 = 0x40000;
pub const D3DSP_WRITEMASK_3: u32 = 0x80000;
pub const D3DSPDM_SATURATE: u32 = 0x100000;
pub const D3DSPDM_PARTIALPRECISION: u32 = 0x200000;
pub const D3DSPDM_MSAMPCENTROID: u32 = 0x400000;
pub const D3DSP_REGTYPE_SHIFT: u32 = 0x1c;
pub const D3DSP_REGTYPE_SHIFT2: u32 = 0x8;
pub const D3DSP_REGTYPE_MASK: u32 = 0x70000000;
pub const D3DSP_REGTYPE_MASK2: u32 = 0x1800;
pub const D3DSPR_TEMP: u32 = 0x0;
pub const D3DSPR_INPUT: u32 = 0x1;
pub const D3DSPR_CONST: u32 = 0x2;
pub const D3DSPR_ADDR: u32 = 0x3;
pub const D3DSPR_TEXTURE: u32 = 0x3;
pub const D3DSPR_RASTOUT: u32 = 0x4;
pub const D3DSPR_ATTROUT: u32 = 0x5;
pub const D3DSPR_TEXCRDOUT: u32 = 0x6;
pub const D3DSPR_OUTPUT: u32 = 0x6;
pub const D3DSPR_CONSTINT: u32 = 0x7;
pub const D3DSPR_COLOROUT: u32 = 0x8;
pub const D3DSPR_DEPTHOUT: u32 = 0x9;
pub const D3DSPR_SAMPLER: u32 = 0xa;
pub const D3DSPR_CONSTBOOL: u32 = 0xe;
pub const D3DSPR_LOOP: u32 = 0xf;
pub const D3DSPR_MISCTYPE: u32 = 0x11;
pub const D3DSPR_LABEL: u32 = 0x12;
pub const D3DSPR_PREDICATE: u32 = 0x13;
pub const D3DVS_X_X: u32 = 0x0;
pub const D3DVS_X_Y: u32 = 0x10000;
pub const D3DVS_X_Z: u32 = 0x20000;
pub const D3DVS_X_W: u32 = 0x30000;
pub const D3DVS_Y_X: u32 = 0x0;
pub const D3DVS_Y_Y: u32 = 0x40000;
pub const D3DVS_Y_Z: u32 = 0x80000;
pub const D3DVS_Y_W: u32 = 0xc0000;
pub const D3DVS_Z_X: u32 = 0x0;
pub const D3DVS_Z_Y: u32 = 0x100000;
pub const D3DVS_Z_Z: u32 = 0x200000;
pub const D3DVS_Z_W: u32 = 0x300000;
pub const D3DVS_W_X: u32 = 0x0;
pub const D3DVS_W_Y: u32 = 0x400000;
pub const D3DVS_W_Z: u32 = 0x800000;
pub const D3DVS_W_W: u32 = 0xc00000;
pub const D3DSPSM_NONE: u32 = 0x0;
pub const D3DSPSM_NEG: u32 = 0x1000000;
pub const D3DSPSM_BIAS: u32 = 0x2000000;
pub const D3DSPSM_BIASNEG: u32 = 0x3000000;
pub const D3DSPSM_SIGN: u32 = 0x4000000;
pub const D3DSPSM_SIGNNEG: u32 = 0x5000000;
pub const D3DSPSM_COMP: u32 = 0x6000000;
pub const D3DSPSM_X2: u32 = 0x7000000;
pub const D3DSPSM_X2NEG: u32 = 0x8000000;
pub const D3DSPSM_DZ: u32 = 0x9000000;
pub const D3DSPSM_DW: u32 = 0xa000000;
pub const D3DSPSM_ABS: u32 = 0xb000000;
pub const D3DSPSM_ABSNEG: u32 = 0xc000000;
pub const D3DSPSM_NOT: u32 = 0xd000000;
pub const D3DSPC_GT: u32 = 0x1;
pub const D3DSPC_EQ: u32 = 0x2;
pub const D3DSPC_GE: u32 = 0x3;
pub const D3DSPC_LT: u32 = 0x4;
pub const D3DSPC_NE: u32 = 0x5;
pub const D3DSPC_LE: u32 = 0x6;
pub const BWRITER_COMPARISON_GT: u32 = 0x1;
pub const BWRITER_COMPARISON_EQ: u32 = 0x2;
pub const BWRITER_COMPARISON_GE: u32 = 0x3;
pub const BWRITER_COMPARISON_LT: u32 = 0x4;
pub const BWRITER_COMPARISON_NE: u32 = 0x5;
pub const BWRITER_COMPARISON_LE: u32 = 0x6;
pub const BWRITERSPR_TEMP: u32 = 0x0;
pub const BWRITERSPR_INPUT: u32 = 0x1;
pub const BWRITERSPR_CONST: u32 = 0x2;
pub const BWRITERSPR_ADDR: u32 = 0x3;
pub const BWRITERSPR_TEXTURE: u32 = 0x4;
pub const BWRITERSPR_RASTOUT: u32 = 0x5;
pub const BWRITERSPR_ATTROUT: u32 = 0x6;
pub const BWRITERSPR_TEXCRDOUT: u32 = 0x7;
pub const BWRITERSPR_OUTPUT: u32 = 0x8;
pub const BWRITERSPR_CONSTINT: u32 = 0x9;
pub const BWRITERSPR_COLOROUT: u32 = 0xa;
pub const BWRITERSPR_DEPTHOUT: u32 = 0xb;
pub const BWRITERSPR_SAMPLER: u32 = 0xc;
pub const BWRITERSPR_CONSTBOOL: u32 = 0xd;
pub const BWRITERSPR_LOOP: u32 = 0xe;
pub const BWRITERSPR_MISCTYPE: u32 = 0xf;
pub const BWRITERSPR_LABEL: u32 = 0x10;
pub const BWRITERSPR_PREDICATE: u32 = 0x11;
pub const BWRITERSPDM_SATURATE: u32 = 0x1;
pub const BWRITERSPDM_PARTIALPRECISION: u32 = 0x2;
pub const BWRITERSPDM_MSAMPCENTROID: u32 = 0x4;
pub const BWRITERSTT_UNKNOWN: u32 = 0x0;
pub const BWRITERSTT_1D: u32 = 0x1;
pub const BWRITERSTT_2D: u32 = 0x2;
pub const BWRITERSTT_CUBE: u32 = 0x3;
pub const BWRITERSTT_VOLUME: u32 = 0x4;
pub const BWRITERSPSM_NONE: u32 = 0x0;
pub const BWRITERSPSM_NEG: u32 = 0x1;
pub const BWRITERSPSM_BIAS: u32 = 0x2;
pub const BWRITERSPSM_BIASNEG: u32 = 0x3;
pub const BWRITERSPSM_SIGN: u32 = 0x4;
pub const BWRITERSPSM_SIGNNEG: u32 = 0x5;
pub const BWRITERSPSM_COMP: u32 = 0x6;
pub const BWRITERSPSM_X2: u32 = 0x7;
pub const BWRITERSPSM_X2NEG: u32 = 0x8;
pub const BWRITERSPSM_DZ: u32 = 0x9;
pub const BWRITERSPSM_DW: u32 = 0xa;
pub const BWRITERSPSM_ABS: u32 = 0xb;
pub const BWRITERSPSM_ABSNEG: u32 = 0xc;
pub const BWRITERSPSM_NOT: u32 = 0xd;
pub const BWRITERSP_WRITEMASK_0: u32 = 0x1;
pub const BWRITERSP_WRITEMASK_1: u32 = 0x2;
pub const BWRITERSP_WRITEMASK_2: u32 = 0x4;
pub const BWRITERSP_WRITEMASK_3: u32 = 0x8;
pub const BWRITERVS_X_X: u32 = 0x0;
pub const BWRITERVS_X_Y: u32 = 0x1;
pub const BWRITERVS_X_Z: u32 = 0x2;
pub const BWRITERVS_X_W: u32 = 0x3;
pub const BWRITERVS_Y_X: u32 = 0x0;
pub const BWRITERVS_Y_Y: u32 = 0x4;
pub const BWRITERVS_Y_Z: u32 = 0x8;
pub const BWRITERVS_Y_W: u32 = 0xc;
pub const BWRITERVS_Z_X: u32 = 0x0;
pub const BWRITERVS_Z_Y: u32 = 0x10;
pub const BWRITERVS_Z_Z: u32 = 0x20;
pub const BWRITERVS_Z_W: u32 = 0x30;
pub const BWRITERVS_W_X: u32 = 0x0;
pub const BWRITERVS_W_Y: u32 = 0x40;
pub const BWRITERVS_W_Z: u32 = 0x80;
pub const BWRITERVS_W_W: u32 = 0xc0;
// END GENERATED: consts-token

/// Each bytecode writer swizzle selector, with its D3D9 bits.
const SWIZZLE_PAIRS: [(u32, u32); 16] = [
    (BWRITERVS_X_X, D3DVS_X_X),
    (BWRITERVS_X_Y, D3DVS_X_Y),
    (BWRITERVS_X_Z, D3DVS_X_Z),
    (BWRITERVS_X_W, D3DVS_X_W),
    (BWRITERVS_Y_X, D3DVS_Y_X),
    (BWRITERVS_Y_Y, D3DVS_Y_Y),
    (BWRITERVS_Y_Z, D3DVS_Y_Z),
    (BWRITERVS_Y_W, D3DVS_Y_W),
    (BWRITERVS_Z_X, D3DVS_Z_X),
    (BWRITERVS_Z_Y, D3DVS_Z_Y),
    (BWRITERVS_Z_Z, D3DVS_Z_Z),
    (BWRITERVS_Z_W, D3DVS_Z_W),
    (BWRITERVS_W_X, D3DVS_W_X),
    (BWRITERVS_W_Y, D3DVS_W_Y),
    (BWRITERVS_W_Z, D3DVS_W_Z),
    (BWRITERVS_W_W, D3DVS_W_W),
];

/// The D3D9 swizzle bits for every selector whose bits are all set in `bwriter_swizzle`.
pub fn d3d9_swizzle(bwriter_swizzle: u32) -> u32 {
    let mut ret = 0;
    for (bwriter, d3d9) in SWIZZLE_PAIRS {
        if (bwriter_swizzle & bwriter) == bwriter {
            ret |= d3d9;
        }
    }
    ret
}

/// The D3D9 write mask for the channels set in `bwriter_writemask`.
pub fn d3d9_writemask(bwriter_writemask: u32) -> u32 {
    let mut ret = 0;
    if (bwriter_writemask & BWRITERSP_WRITEMASK_0) != 0 {
        ret |= D3DSP_WRITEMASK_0;
    }
    if (bwriter_writemask & BWRITERSP_WRITEMASK_1) != 0 {
        ret |= D3DSP_WRITEMASK_1;
    }
    if (bwriter_writemask & BWRITERSP_WRITEMASK_2) != 0 {
        ret |= D3DSP_WRITEMASK_2;
    }
    if (bwriter_writemask & BWRITERSP_WRITEMASK_3) != 0 {
        ret |= D3DSP_WRITEMASK_3;
    }
    ret
}

/// The D3D9 source modifier for a bytecode writer source modifier. Unknown values give 0.
pub fn d3d9_srcmod(bwriter_srcmod: u32) -> u32 {
    match bwriter_srcmod {
        // BEGIN GENERATED: d3d9_srcmod
        BWRITERSPSM_NONE => D3DSPSM_NONE,
        BWRITERSPSM_NEG => D3DSPSM_NEG,
        BWRITERSPSM_BIAS => D3DSPSM_BIAS,
        BWRITERSPSM_BIASNEG => D3DSPSM_BIASNEG,
        BWRITERSPSM_SIGN => D3DSPSM_SIGN,
        BWRITERSPSM_SIGNNEG => D3DSPSM_SIGNNEG,
        BWRITERSPSM_COMP => D3DSPSM_COMP,
        BWRITERSPSM_X2 => D3DSPSM_X2,
        BWRITERSPSM_X2NEG => D3DSPSM_X2NEG,
        BWRITERSPSM_DZ => D3DSPSM_DZ,
        BWRITERSPSM_DW => D3DSPSM_DW,
        BWRITERSPSM_ABS => D3DSPSM_ABS,
        BWRITERSPSM_ABSNEG => D3DSPSM_ABSNEG,
        BWRITERSPSM_NOT => D3DSPSM_NOT,
        // END GENERATED: d3d9_srcmod
        _ => 0,
    }
}

/// The D3D9 destination modifier bits for the modifiers set in `bwriter_mod`.
pub fn d3d9_dstmod(bwriter_mod: u32) -> u32 {
    let mut ret = 0;
    if (bwriter_mod & BWRITERSPDM_SATURATE) != 0 {
        ret |= D3DSPDM_SATURATE;
    }
    if (bwriter_mod & BWRITERSPDM_PARTIALPRECISION) != 0 {
        ret |= D3DSPDM_PARTIALPRECISION;
    }
    if (bwriter_mod & BWRITERSPDM_MSAMPCENTROID) != 0 {
        ret |= D3DSPDM_MSAMPCENTROID;
    }
    ret
}

/// The D3D9 comparison for a bytecode writer comparison type. Unknown values give 0.
pub fn d3d9_comparetype(comparison: u32) -> u32 {
    match comparison {
        // BEGIN GENERATED: d3d9_comparetype
        BWRITER_COMPARISON_GT => D3DSPC_GT,
        BWRITER_COMPARISON_EQ => D3DSPC_EQ,
        BWRITER_COMPARISON_GE => D3DSPC_GE,
        BWRITER_COMPARISON_LT => D3DSPC_LT,
        BWRITER_COMPARISON_NE => D3DSPC_NE,
        BWRITER_COMPARISON_LE => D3DSPC_LE,
        // END GENERATED: d3d9_comparetype
        _ => 0,
    }
}

/// The D3D9 texture type for a bytecode writer sampler texture type. Unknown values give 0.
pub fn d3d9_sampler(bwriter_sampler: u32) -> u32 {
    match bwriter_sampler {
        // BEGIN GENERATED: d3d9_sampler
        BWRITERSTT_UNKNOWN => D3DSTT_UNKNOWN,
        BWRITERSTT_1D => D3DSTT_1D,
        BWRITERSTT_2D => D3DSTT_2D,
        BWRITERSTT_CUBE => D3DSTT_CUBE,
        BWRITERSTT_VOLUME => D3DSTT_VOLUME,
        // END GENERATED: d3d9_sampler
        _ => 0,
    }
}

/// The D3D9 register type for a bytecode writer register type. Unknown values give `u32::MAX`.
pub fn d3d9_register(bwriter_register: u32) -> u32 {
    match bwriter_register {
        // BEGIN GENERATED: d3d9_register
        BWRITERSPR_TEMP => D3DSPR_TEMP,
        BWRITERSPR_INPUT => D3DSPR_INPUT,
        BWRITERSPR_CONST => D3DSPR_CONST,
        BWRITERSPR_ADDR => D3DSPR_ADDR,
        BWRITERSPR_TEXTURE => D3DSPR_TEXTURE,
        BWRITERSPR_RASTOUT => D3DSPR_RASTOUT,
        BWRITERSPR_ATTROUT => D3DSPR_ATTROUT,
        BWRITERSPR_TEXCRDOUT => D3DSPR_TEXCRDOUT,
        BWRITERSPR_OUTPUT => D3DSPR_OUTPUT,
        BWRITERSPR_CONSTINT => D3DSPR_CONSTINT,
        BWRITERSPR_COLOROUT => D3DSPR_COLOROUT,
        BWRITERSPR_DEPTHOUT => D3DSPR_DEPTHOUT,
        BWRITERSPR_SAMPLER => D3DSPR_SAMPLER,
        BWRITERSPR_CONSTBOOL => D3DSPR_CONSTBOOL,
        BWRITERSPR_LOOP => D3DSPR_LOOP,
        BWRITERSPR_MISCTYPE => D3DSPR_MISCTYPE,
        BWRITERSPR_LABEL => D3DSPR_LABEL,
        BWRITERSPR_PREDICATE => D3DSPR_PREDICATE,
        // END GENERATED: d3d9_register
        _ => u32::MAX,
    }
}

/// The register token for `reg_type` and `num`. The type is split over two fields.
pub fn d3dsp_register(reg_type: u32, num: u32) -> u32 {
    ((reg_type << D3DSP_REGTYPE_SHIFT) & D3DSP_REGTYPE_MASK)
        | ((reg_type << D3DSP_REGTYPE_SHIFT2) & D3DSP_REGTYPE_MASK2)
        | (num & D3DSP_REGNUM_MASK)
}
