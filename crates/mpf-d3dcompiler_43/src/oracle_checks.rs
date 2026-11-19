//! Replays hand-picked rows of the Wine oracle in `target/canon/d3dcompiler_43`. Its C is
//! verbatim from `blob.c` and `bytecodewriter.c`, built with `gcc -fsanitize=address,undefined`.
//! No vector was excluded for a sanitizer report. Each check follows a comment that names the
//! `oracle.out` row it replays.

use crate::{blob::*, opcode::*, sm1_version, token::*, ShaderType};

#[test]
fn sm1_version_rows() {
    // oracle.out:425 sm1_version 0x0 0x2 0x2 -> 0x0
    assert_eq!(sm1_version(ShaderType::Unknown, 0x2, 0x2), 0x0);
    // oracle.out:443 sm1_version 0x1 0x2 0x0 -> 0xfffe0200
    assert_eq!(sm1_version(ShaderType::Vertex, 0x2, 0x0), 0xfffe0200);
    // oracle.out:467 sm1_version 0x2 0x3 0x0 -> 0xffff0300
    assert_eq!(sm1_version(ShaderType::Pixel, 0x3, 0x0), 0xffff0300);
}

#[test]
fn token_rows() {
    // oracle.out:478 d3d9_writemask 0x3 -> 0x30000
    assert_eq!(d3d9_writemask(0x3), 0x30000);
    // oracle.out:520 d3d9_srcmod 0xd -> 0xd000000
    assert_eq!(d3d9_srcmod(0xd), 0xd000000);
    // oracle.out:521 d3d9_srcmod 0xe -> 0x0
    assert_eq!(d3d9_srcmod(0xe), 0x0);
    // oracle.out:544 d3d9_dstmod 0x5 -> 0x500000
    assert_eq!(d3d9_dstmod(0x5), 0x500000);
    // oracle.out:572 d3d9_comparetype 0x1 -> 0x1
    assert_eq!(d3d9_comparetype(0x1), 0x1);
    // oracle.out:606 d3d9_sampler 0x3 -> 0x18000000
    assert_eq!(d3d9_sampler(0x3), 0x18000000);
    // oracle.out:607 d3d9_sampler 0x4 -> 0x20000000
    assert_eq!(d3d9_sampler(0x4), 0x20000000);
    // oracle.out:652 d3d9_register 0x11 -> 0x13
    assert_eq!(d3d9_register(0x11), 0x13);
    // oracle.out:653 d3d9_register 0x12 -> 0xffffffff
    assert_eq!(d3d9_register(0x12), 0xffffffff);
    // oracle.out:824 d3dsp_register 0x7 0x0 -> 0x70000000
    assert_eq!(d3dsp_register(0x7, 0x0), 0x70000000);
    // oracle.out:825 d3dsp_register 0x7 0x7ff -> 0x700007ff
    assert_eq!(d3dsp_register(0x7, 0x7ff), 0x700007ff);
}

#[test]
fn opcode_rows() {
    // oracle.out:748 d3d9_opcode 0x51 -> 0x10042
    assert_eq!(d3d9_opcode(0x51), 0x10042);
    // oracle.out:749 d3d9_opcode 0x52 -> 0x20042
    assert_eq!(d3d9_opcode(0x52), 0x20042);
    // oracle.out:753 d3d9_opcode 0x56 -> 0xffffffff
    assert_eq!(d3d9_opcode(0x56), 0xffffffff);
}

#[test]
fn blob_rows() {
    // oracle.out:924 check_blob_part 0x4e475349 0x0 -> 0x1
    assert!(check_blob_part(TAG_ISGN, 0x0));
    // oracle.out:949 check_blob_part 0x4e47534f 0x2 -> 0x1
    assert!(check_blob_part(TAG_OSGN, 0x2));
    // oracle.out:1169 check_blob_strip 0x46454452 0x1 -> 0x0
    assert!(!check_blob_strip(TAG_RDEF, 0x1));
    // oracle.out:1322 check_blob_strip 0x4e475349 0x1f -> 0x1
    assert!(check_blob_strip(TAG_ISGN, 0x1f));
    // oracle.out:1347 section_count_after_switch 0x4 0x3 -> 0x3
    assert_eq!(blob_part_section_count(0x4, 0x3), 0x3);
    // oracle.out:1393 part_out_of_range 0xd -> 0x1
    assert!(blob_part_out_of_range(0xd));
    // oracle.out:1403 part_out_of_range 0x8000 -> 0x0
    assert!(!blob_part_out_of_range(0x8000));
}
