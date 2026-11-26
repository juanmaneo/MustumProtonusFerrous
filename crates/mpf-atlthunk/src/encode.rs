/// `mov offset(%rip), %rcx; jmp *offset(%rip)`, packed as in `struct thunk_code`
/// on x86-64 (13 bytes). Both RIP-relative displacements are measured from the
/// end of their own field and truncated to 32 bits, as the C pointer
/// subtraction is.
pub fn encode_x86_64(thunk: u64, arg: u64, proc_: u64) -> [u8; 13] {
    let mov_offset = arg.wrapping_sub(thunk.wrapping_add(7)) as u32;
    let jmp_offset = proc_.wrapping_sub(thunk.wrapping_add(13)) as u32;
    let mut code = [0u8; 13];
    code[0..3].copy_from_slice(&[0x48, 0x8b, 0x0d]);
    code[3..7].copy_from_slice(&mov_offset.to_le_bytes());
    code[7..9].copy_from_slice(&0x25ffu16.to_le_bytes());
    code[9..13].copy_from_slice(&jmp_offset.to_le_bytes());
    code
}

/// `movl data, %eax; movl %eax, 4(%esp); jmp *proc` as in `struct thunk_code`
/// on i386 (15 bytes). Absolute addresses are truncated to 32 bits.
pub fn encode_i386(arg: u64, proc_: u64) -> [u8; 15] {
    let mut code = [0u8; 15];
    code[0] = 0xa1;
    code[1..5].copy_from_slice(&(arg as u32).to_le_bytes());
    code[5..9].copy_from_slice(&0x0424_4489u32.to_le_bytes());
    code[9..11].copy_from_slice(&0x25ffu16.to_le_bytes());
    code[11..15].copy_from_slice(&(proc_ as u32).to_le_bytes());
    code
}

/// Three AArch64 instruction words (`ldr x0`, `ldr x16`, `br x16`) followed by
/// the unwritten pad word, which the zeroed pool leaves as 0. Literal offsets are
/// counted in 32-bit words from each instruction and shifted into imm19.
pub fn encode_aarch64(thunk: u64, arg: u64, proc_: u64) -> [u8; 16] {
    let ldr_x0 = 0x5800_0000u32 | (((arg as i64 - thunk as i64) / 4) << 5) as u32;
    let ldr_x16 = 0x5800_0010u32 | (((proc_ as i64 - thunk as i64 - 4) / 4) << 5) as u32;
    let mut code = [0u8; 16];
    code[0..4].copy_from_slice(&ldr_x0.to_le_bytes());
    code[4..8].copy_from_slice(&ldr_x16.to_le_bytes());
    code[8..12].copy_from_slice(&0xd61f_0200u32.to_le_bytes());
    code
}
