use crate::args::{parse_win32_arg, ArgType};

/// The `@ stdcall` entries of `dlls/icmp/icmp.spec`, in spec order.
pub const ICMP_SPEC: [&str; 8] = [
    "@ stdcall IcmpCloseHandle(ptr) iphlpapi.IcmpCloseHandle",
    "@ stdcall IcmpCreateFile() iphlpapi.IcmpCreateFile",
    "@ stdcall IcmpParseReplies(ptr long) iphlpapi.IcmpParseReplies",
    "@ stdcall IcmpSendEcho2(ptr ptr ptr ptr long ptr long ptr ptr long long) iphlpapi.IcmpSendEcho2",
    "@ stdcall IcmpSendEcho(ptr long ptr long ptr ptr long long) iphlpapi.IcmpSendEcho",
    "@ stdcall do_echo_rep() iphlpapi.do_echo_rep",
    "@ stdcall do_echo_req() iphlpapi.do_echo_req",
    "@ stdcall register_icmp() iphlpapi.register_icmp",
];

/// One export-table entry: its name, ordinal, declared argument types and forward target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Export<'a> {
    pub name: &'a str,
    pub ordinal: u16,
    pub args: Vec<ArgType>,
    pub forward: &'a str,
}

/// Parses an `@ stdcall Name(args) module.Function` line as winebuild does for a forward, and
/// gives it `ordinal`. Returns `None` for a line that winebuild would not export as a forward:
/// another calling convention, a code entry point, an unknown or 16-bit-only argument type, or
/// a syntax error.
pub fn parse_stdcall_forward(line: &str, ordinal: u16) -> Option<Export<'_>> {
    let rest = line
        .trim()
        .strip_prefix('@')?
        .strip_prefix(char::is_whitespace)?;
    let rest = rest
        .trim_start()
        .strip_prefix("stdcall")?
        .strip_prefix(char::is_whitespace)?;
    let (name, rest) = rest.split_once('(')?;
    let (args, target) = rest.split_once(')')?;
    let (name, target) = (name.trim(), target.trim());
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    if !target.contains('.') || target.contains(char::is_whitespace) {
        return None;
    }
    let args = args
        .split_whitespace()
        .map(parse_win32_arg)
        .collect::<Option<Vec<_>>>()?;
    Some(Export {
        name,
        ordinal,
        args,
        forward: target,
    })
}

/// The export table of icmp.spec. Winebuild numbers entries that name no ordinal from 1 in
/// spec order.
pub fn exports() -> Vec<Export<'static>> {
    ICMP_SPEC
        .into_iter()
        .enumerate()
        .filter_map(|(index, line)| parse_stdcall_forward(line, index as u16 + 1))
        .collect()
}
