//! Oracle rows for the icmp port.
//!
//! Each row is an input and the output of the reference `winebuild --def` (`tools/winebuild`,
//! built unmodified with `gcc -fsanitize=address,undefined -g` under `target/canon/icmp`). Rows
//! 01-08 are the entries of `dlls/icmp/icmp.spec` with `-m32`, rows 09-16 the same entries with
//! `-m64`, and rows 17-24 a synthetic spec with one entry per argument type, with `-m32`. No row
//! was excluded: ASan and UBSan reported nothing for any run.

use super::*;

/// (architecture, ordinal, spec line, winebuild --def export line)
const ORACLE_ROWS: [(Arch, u16, &str, &str); 24] = [
    (Arch::I386, 1, "@ stdcall IcmpCloseHandle(ptr) iphlpapi.IcmpCloseHandle", "  IcmpCloseHandle@4=iphlpapi.IcmpCloseHandle @1"),
    (Arch::I386, 2, "@ stdcall IcmpCreateFile() iphlpapi.IcmpCreateFile", "  IcmpCreateFile@0=iphlpapi.IcmpCreateFile @2"),
    (Arch::I386, 3, "@ stdcall IcmpParseReplies(ptr long) iphlpapi.IcmpParseReplies", "  IcmpParseReplies@8=iphlpapi.IcmpParseReplies @3"),
    (Arch::I386, 4, "@ stdcall IcmpSendEcho2(ptr ptr ptr ptr long ptr long ptr ptr long long) iphlpapi.IcmpSendEcho2", "  IcmpSendEcho2@44=iphlpapi.IcmpSendEcho2 @4"),
    (Arch::I386, 5, "@ stdcall IcmpSendEcho(ptr long ptr long ptr ptr long long) iphlpapi.IcmpSendEcho", "  IcmpSendEcho@32=iphlpapi.IcmpSendEcho @5"),
    (Arch::I386, 6, "@ stdcall do_echo_rep() iphlpapi.do_echo_rep", "  do_echo_rep@0=iphlpapi.do_echo_rep @6"),
    (Arch::I386, 7, "@ stdcall do_echo_req() iphlpapi.do_echo_req", "  do_echo_req@0=iphlpapi.do_echo_req @7"),
    (Arch::I386, 8, "@ stdcall register_icmp() iphlpapi.register_icmp", "  register_icmp@0=iphlpapi.register_icmp @8"),
    (Arch::X86_64, 1, "@ stdcall IcmpCloseHandle(ptr) iphlpapi.IcmpCloseHandle", "  IcmpCloseHandle=iphlpapi.IcmpCloseHandle @1"),
    (Arch::X86_64, 2, "@ stdcall IcmpCreateFile() iphlpapi.IcmpCreateFile", "  IcmpCreateFile=iphlpapi.IcmpCreateFile @2"),
    (Arch::X86_64, 3, "@ stdcall IcmpParseReplies(ptr long) iphlpapi.IcmpParseReplies", "  IcmpParseReplies=iphlpapi.IcmpParseReplies @3"),
    (Arch::X86_64, 4, "@ stdcall IcmpSendEcho2(ptr ptr ptr ptr long ptr long ptr ptr long long) iphlpapi.IcmpSendEcho2", "  IcmpSendEcho2=iphlpapi.IcmpSendEcho2 @4"),
    (Arch::X86_64, 5, "@ stdcall IcmpSendEcho(ptr long ptr long ptr ptr long long) iphlpapi.IcmpSendEcho", "  IcmpSendEcho=iphlpapi.IcmpSendEcho @5"),
    (Arch::X86_64, 6, "@ stdcall do_echo_rep() iphlpapi.do_echo_rep", "  do_echo_rep=iphlpapi.do_echo_rep @6"),
    (Arch::X86_64, 7, "@ stdcall do_echo_req() iphlpapi.do_echo_req", "  do_echo_req=iphlpapi.do_echo_req @7"),
    (Arch::X86_64, 8, "@ stdcall register_icmp() iphlpapi.register_icmp", "  register_icmp=iphlpapi.register_icmp @8"),
    (Arch::I386, 1, "@ stdcall A_long(long) x.y", "  A_long@4=x.y @1"),
    (Arch::I386, 2, "@ stdcall A_ptr(ptr) x.y", "  A_ptr@4=x.y @2"),
    (Arch::I386, 3, "@ stdcall A_str(str) x.y", "  A_str@4=x.y @3"),
    (Arch::I386, 4, "@ stdcall A_wstr(wstr) x.y", "  A_wstr@4=x.y @4"),
    (Arch::I386, 5, "@ stdcall A_int64(int64) x.y", "  A_int64@8=x.y @5"),
    (Arch::I386, 6, "@ stdcall A_int128(int128) x.y", "  A_int128@16=x.y @6"),
    (Arch::I386, 7, "@ stdcall A_float(float) x.y", "  A_float@4=x.y @7"),
    (Arch::I386, 8, "@ stdcall A_double(double) x.y", "  A_double@8=x.y @8"),
];

#[test]
fn oracle_def_lines() {
    for (arch, ordinal, spec, expected) in ORACLE_ROWS {
        let export = parse_stdcall_forward(spec, ordinal).expect("oracle spec line parses");
        assert_eq!(def_line(&export, arch), expected, "spec line: {spec}");
    }
}
