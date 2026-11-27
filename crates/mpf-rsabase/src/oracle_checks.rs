//! Oracle-derived test vectors for the ported rsabase logic.
//!
//! Two oracles, both built with `gcc -fsanitize=address,undefined -g` under `target/canon/rsabase/`.
//! The `get_args_size` rows are printed by `args_harness.c`, which holds the verbatim Wine bodies of
//! `get_args_size`, `get_target_ptr_size` and `get_ptr_size`. Its 25 rows ran clean under ASan and
//! UBSan, so none are excluded. The `.def` rows are the 27 `EXPORTS` lines that `winebuild --def`
//! writes for `rsabase.spec` with `-b i386-windows` and `-b x86_64-windows`, from the reference
//! `tools/winebuild` sources. Those rows came out clean too. winebuild leaks memory at exit, so the
//! run used `ASAN_OPTIONS=detect_leaks=0`; the leaks are in the tool, not in the rows.

use super::*;

#[test]
fn oracle_args_size_rows() {
    // row 01 cpu=i386 args=[] -> 0
    assert_eq!(args_size(&[], Cpu::I386), 0);
    // row 02 cpu=i386 args=[ptr] -> 4
    assert_eq!(args_size(&[ArgType::Ptr], Cpu::I386), 4);
    // row 03 cpu=i386 args=[ptr str long ptr] -> 16
    assert_eq!(
        args_size(
            &[ArgType::Ptr, ArgType::Str, ArgType::Long, ArgType::Ptr],
            Cpu::I386
        ),
        16
    );
    // row 04 cpu=i386 args=[int64] -> 8
    assert_eq!(args_size(&[ArgType::Int64], Cpu::I386), 8);
    // row 05 cpu=i386 args=[double] -> 8
    assert_eq!(args_size(&[ArgType::Double], Cpu::I386), 8);
    // row 06 cpu=i386 args=[int128] -> 16
    assert_eq!(args_size(&[ArgType::Int128], Cpu::I386), 16);
    // row 07 cpu=i386 args=[word sword segptr segstr long ptr str wstr float] -> 36
    assert_eq!(
        args_size(
            &[
                ArgType::Word,
                ArgType::SWord,
                ArgType::SegPtr,
                ArgType::SegStr,
                ArgType::Long,
                ArgType::Ptr,
                ArgType::Str,
                ArgType::WStr,
                ArgType::Float,
            ],
            Cpu::I386
        ),
        36
    );
    // row 08 cpu=i386 args=[word long int64] -> 16
    assert_eq!(
        args_size(&[ArgType::Word, ArgType::Long, ArgType::Int64], Cpu::I386),
        16
    );
    // row 09 cpu=x86_64 args=[ptr str long ptr] -> 32
    assert_eq!(
        args_size(
            &[ArgType::Ptr, ArgType::Str, ArgType::Long, ArgType::Ptr],
            Cpu::X86_64
        ),
        32
    );
    // row 10 cpu=x86_64 args=[int64] -> 8
    assert_eq!(args_size(&[ArgType::Int64], Cpu::X86_64), 8);
    // row 11 cpu=x86_64 args=[int128] -> 8
    assert_eq!(args_size(&[ArgType::Int128], Cpu::X86_64), 8);
    // row 12 cpu=x86_64 args=[long double word] -> 24
    assert_eq!(
        args_size(
            &[ArgType::Long, ArgType::Double, ArgType::Word],
            Cpu::X86_64
        ),
        24
    );
    // row 13 cpu=arm args=[long int64] -> 16
    assert_eq!(args_size(&[ArgType::Long, ArgType::Int64], Cpu::Arm), 16);
    // row 14 cpu=arm args=[int64 long] -> 12
    assert_eq!(args_size(&[ArgType::Int64, ArgType::Long], Cpu::Arm), 12);
    // row 15 cpu=arm args=[long int64 long double] -> 32
    assert_eq!(
        args_size(
            &[
                ArgType::Long,
                ArgType::Int64,
                ArgType::Long,
                ArgType::Double
            ],
            Cpu::Arm
        ),
        32
    );
    // row 16 cpu=arm args=[int128] -> 16
    assert_eq!(args_size(&[ArgType::Int128], Cpu::Arm), 16);
    // row 17 cpu=arm args=[word double] -> 16
    assert_eq!(args_size(&[ArgType::Word, ArgType::Double], Cpu::Arm), 16);
    // row 18 cpu=arm64 args=[long int64] -> 16
    assert_eq!(args_size(&[ArgType::Long, ArgType::Int64], Cpu::Arm64), 16);
    // row 19 cpu=arm64 args=[int128] -> 16
    assert_eq!(args_size(&[ArgType::Int128], Cpu::Arm64), 16);
    // row 20 cpu=arm64ec args=[ptr word] -> 16
    assert_eq!(args_size(&[ArgType::Ptr, ArgType::Word], Cpu::Arm64ec), 16);
    // row 21 cpu=arm args=[] -> 0
    assert_eq!(args_size(&[], Cpu::Arm), 0);
    // row 22 cpu=x86_64 args=[] -> 0
    assert_eq!(args_size(&[], Cpu::X86_64), 0);
    // row 23 cpu=i386 args=[word double] -> 12
    assert_eq!(args_size(&[ArgType::Word, ArgType::Double], Cpu::I386), 12);
    // row 24 cpu=x86_64 args=[ptr word] -> 16
    assert_eq!(args_size(&[ArgType::Ptr, ArgType::Word], Cpu::X86_64), 16);
    // row 25 cpu=arm64 args=[word double] -> 16
    assert_eq!(args_size(&[ArgType::Word, ArgType::Double], Cpu::Arm64), 16);
}

/// `winebuild --def -b i386-windows` output for `rsabase.spec`, the 27 `EXPORTS` lines.
const DEF_I386: [&str; 27] = [
    "  CPAcquireContext@16=rsaenh.CPAcquireContext @1",
    "  CPCreateHash@20=rsaenh.CPCreateHash @2",
    "  CPDecrypt@28=rsaenh.CPDecrypt @3",
    "  CPDeriveKey@20=rsaenh.CPDeriveKey @4",
    "  CPDestroyHash@8=rsaenh.CPDestroyHash @5",
    "  CPDestroyKey@8=rsaenh.CPDestroyKey @6",
    "  CPDuplicateHash@20=rsaenh.CPDuplicateHash @7",
    "  CPDuplicateKey@20=rsaenh.CPDuplicateKey @8",
    "  CPEncrypt@32=rsaenh.CPEncrypt @9",
    "  CPExportKey@28=rsaenh.CPExportKey @10",
    "  CPGenKey@16=rsaenh.CPGenKey @11",
    "  CPGenRandom@12=rsaenh.CPGenRandom @12",
    "  CPGetHashParam@24=rsaenh.CPGetHashParam @13",
    "  CPGetKeyParam@24=rsaenh.CPGetKeyParam @14",
    "  CPGetProvParam@20=rsaenh.CPGetProvParam @15",
    "  CPGetUserKey@12=rsaenh.CPGetUserKey @16",
    "  CPHashData@20=rsaenh.CPHashData @17",
    "  CPHashSessionKey@16=rsaenh.CPHashSessionKey @18",
    "  CPImportKey@24=rsaenh.CPImportKey @19",
    "  CPReleaseContext@8=rsaenh.CPReleaseContext @20",
    "  CPSetHashParam@20=rsaenh.CPSetHashParam @21",
    "  CPSetKeyParam@20=rsaenh.CPSetKeyParam @22",
    "  CPSetProvParam@16=rsaenh.CPSetProvParam @23",
    "  CPSignHash@28=rsaenh.CPSignHash @24",
    "  CPVerifySignature@28=rsaenh.CPVerifySignature @25",
    "  DllRegisterServer@0=rsaenh.DllRegisterServer @26 PRIVATE",
    "  DllUnregisterServer@0=rsaenh.DllUnregisterServer @27 PRIVATE",
];

/// `winebuild --def -b x86_64-windows` output for `rsabase.spec`, the 27 `EXPORTS` lines.
const DEF_X86_64: [&str; 27] = [
    "  CPAcquireContext=rsaenh.CPAcquireContext @1",
    "  CPCreateHash=rsaenh.CPCreateHash @2",
    "  CPDecrypt=rsaenh.CPDecrypt @3",
    "  CPDeriveKey=rsaenh.CPDeriveKey @4",
    "  CPDestroyHash=rsaenh.CPDestroyHash @5",
    "  CPDestroyKey=rsaenh.CPDestroyKey @6",
    "  CPDuplicateHash=rsaenh.CPDuplicateHash @7",
    "  CPDuplicateKey=rsaenh.CPDuplicateKey @8",
    "  CPEncrypt=rsaenh.CPEncrypt @9",
    "  CPExportKey=rsaenh.CPExportKey @10",
    "  CPGenKey=rsaenh.CPGenKey @11",
    "  CPGenRandom=rsaenh.CPGenRandom @12",
    "  CPGetHashParam=rsaenh.CPGetHashParam @13",
    "  CPGetKeyParam=rsaenh.CPGetKeyParam @14",
    "  CPGetProvParam=rsaenh.CPGetProvParam @15",
    "  CPGetUserKey=rsaenh.CPGetUserKey @16",
    "  CPHashData=rsaenh.CPHashData @17",
    "  CPHashSessionKey=rsaenh.CPHashSessionKey @18",
    "  CPImportKey=rsaenh.CPImportKey @19",
    "  CPReleaseContext=rsaenh.CPReleaseContext @20",
    "  CPSetHashParam=rsaenh.CPSetHashParam @21",
    "  CPSetKeyParam=rsaenh.CPSetKeyParam @22",
    "  CPSetProvParam=rsaenh.CPSetProvParam @23",
    "  CPSignHash=rsaenh.CPSignHash @24",
    "  CPVerifySignature=rsaenh.CPVerifySignature @25",
    "  DllRegisterServer=rsaenh.DllRegisterServer @26 PRIVATE",
    "  DllUnregisterServer=rsaenh.DllUnregisterServer @27 PRIVATE",
];

#[test]
fn oracle_def_lines_i386() {
    for (export, expected) in EXPORTS.iter().zip(DEF_I386) {
        assert_eq!(def_line(export, Cpu::I386), expected);
    }
}

#[test]
fn oracle_def_lines_x86_64() {
    for (export, expected) in EXPORTS.iter().zip(DEF_X86_64) {
        assert_eq!(def_line(export, Cpu::X86_64), expected);
    }
}
