//! Oracle-derived test vectors for the ported mswsock logic.
//!
//! The vectors are the rows of `target/canon/mswsock/oracle_rows.txt`, printed by `oracle.c` in
//! the same directory. The harness compiles the verbatim Wine C bodies of `mswsock.c` (lines
//! 32-168) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`, lines 504-516) with
//! `gcc -fsanitize=address,undefined -g`. It prints 44 rows: 3 GUID rows (K01-K03), 14 extension
//! rows (F01-F14), 1 `WSARecvEx` row (W01) and 26 stub rows (S01-S26, one per `@ stub` export).
//! No row was excluded, because the sanitizers printed nothing.
//!
//! The harness differs from the port in three ways:
//! - `RtlRaiseException` longjmps out of the stub, because the verbatim stub body loops around
//!   it. The port returns the record instead.
//! - The pointers that `WSAIoctl` returns are fake functions that record the call and return the
//!   row's `ret`. The port uses `&'static str` names for them.
//! - The socket is `0x100` for `AcceptEx` and `0x200` for `TransmitFile`. The port passes it to
//!   the `ioctl` closure, and the closure's log shows it as the `@` suffix of each entry.

use super::*;

/// The `WSAID_*` rows of `oracle.c`: `(id, name, guid, printed row)`.
const GUID_ROWS: [(&str, &str, Guid, &str); 3] = [
    (
        "K01",
        "WSAID_ACCEPTEX",
        WSAID_ACCEPTEX,
        "K01 WSAID_ACCEPTEX guid=b5367df1-cbac-11cf-95ca-00805f48a192",
    ),
    (
        "K02",
        "WSAID_GETACCEPTEXSOCKADDRS",
        WSAID_GETACCEPTEXSOCKADDRS,
        "K02 WSAID_GETACCEPTEXSOCKADDRS guid=b5367df2-cbac-11cf-95ca-00805f48a192",
    ),
    (
        "K03",
        "WSAID_TRANSMITFILE",
        WSAID_TRANSMITFILE,
        "K03 WSAID_TRANSMITFILE guid=b5367df0-cbac-11cf-95ca-00805f48a192",
    ),
];

/// The extension rows of `oracle.c`: `(id, ops, fail, ret, printed row)`. `ops` is a sequence of
/// `A` (AcceptEx), `T` (TransmitFile) and `G` (GetAcceptExSockaddrs). In `fail`, bit 1 fails the
/// AcceptEx lookup, bit 2 the GetAcceptExSockaddrs lookup and bit 4 the TransmitFile lookup.
const EXTENSION_ROWS: [(&str, &str, u32, bool, &str); 14] = [
    (
        "F01",
        "A",
        0,
        true,
        "F01 ops=A fail=0 ret=1 :: A:1:1:a@100,g@100,t@100 init=1",
    ),
    (
        "F02",
        "A",
        1,
        true,
        "F02 ops=A fail=1 ret=1 :: A:0:0:a@100,g@100,t@100 init=1",
    ),
    (
        "F03",
        "A",
        7,
        true,
        "F03 ops=A fail=7 ret=1 :: A:0:0:a@100,g@100,t@100 init=1",
    ),
    (
        "F04",
        "A",
        0,
        false,
        "F04 ops=A fail=0 ret=0 :: A:0:1:a@100,g@100,t@100 init=1",
    ),
    (
        "F05",
        "AA",
        0,
        true,
        "F05 ops=AA fail=0 ret=1 :: A:1:1:a@100,g@100,t@100 A:1:1:- init=1",
    ),
    (
        "F06",
        "AA",
        1,
        true,
        "F06 ops=AA fail=1 ret=1 :: A:0:0:a@100,g@100,t@100 A:0:0:- init=1",
    ),
    (
        "F07",
        "T",
        0,
        true,
        "F07 ops=T fail=0 ret=1 :: T:1:1:a@200,g@200,t@200 init=1",
    ),
    (
        "F08",
        "T",
        4,
        true,
        "F08 ops=T fail=4 ret=1 :: T:0:0:a@200,g@200,t@200 init=1",
    ),
    (
        "F09",
        "G",
        0,
        true,
        "F09 ops=G fail=0 ret=1 :: G:-:0:- init=0",
    ),
    (
        "F10",
        "GA",
        0,
        true,
        "F10 ops=GA fail=0 ret=1 :: G:-:0:- A:1:1:a@100,g@100,t@100 init=1",
    ),
    (
        "F11",
        "AG",
        2,
        true,
        "F11 ops=AG fail=2 ret=1 :: A:1:1:a@100,g@100,t@100 G:-:0:- init=1",
    ),
    (
        "F12",
        "AG",
        0,
        true,
        "F12 ops=AG fail=0 ret=1 :: A:1:1:a@100,g@100,t@100 G:-:1:- init=1",
    ),
    (
        "F13",
        "AT",
        1,
        true,
        "F13 ops=AT fail=1 ret=1 :: A:0:0:a@100,g@100,t@100 T:1:1:- init=1",
    ),
    (
        "F14",
        "AT",
        4,
        true,
        "F14 ops=AT fail=4 ret=1 :: A:1:1:a@100,g@100,t@100 T:0:0:- init=1",
    ),
];

/// The stub rows S01-S26: `(helper, function, code, flags, number_parameters, module)`, in the
/// order of the `@ stub` lines in `dlls/mswsock/mswsock.spec`.
type StubRow = (
    fn() -> StubException,
    &'static str,
    u32,
    u32,
    u32,
    &'static str,
);

const STUB_ROWS: [StubRow; 26] = [
    (
        get_address_by_name_a,
        "GetAddressByNameA",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_address_by_name_w,
        "GetAddressByNameW",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_name_by_type_a,
        "GetNameByTypeA",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_name_by_type_w,
        "GetNameByTypeW",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_service_a,
        "GetServiceA",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_service_w,
        "GetServiceW",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_type_by_name_a,
        "GetTypeByNameA",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        get_type_by_name_w,
        "GetTypeByNameW",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        migrate_winsock_configuration,
        "MigrateWinsockConfiguration",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        np_load_name_spaces,
        "NPLoadNameSpaces",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        nsp_startup,
        "NSPStartup",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        service_main,
        "ServiceMain",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        set_service_a,
        "SetServiceA",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        set_service_w,
        "SetServiceW",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        start_wsdp_service,
        "StartWsdpService",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        stop_wsdp_service,
        "StopWsdpService",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        svchost_push_service_globals,
        "SvchostPushServiceGlobals",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        wsp_startup,
        "WSPStartup",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (dn_expand, "dn_expand", 0x8000_0100, 0x1, 2, "mswsock.dll"),
    (
        getnetbyname,
        "getnetbyname",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (
        inet_network,
        "inet_network",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
    (rcmd, "rcmd", 0x8000_0100, 0x1, 2, "mswsock.dll"),
    (rexec, "rexec", 0x8000_0100, 0x1, 2, "mswsock.dll"),
    (rresvport, "rresvport", 0x8000_0100, 0x1, 2, "mswsock.dll"),
    (s_perror, "s_perror", 0x8000_0100, 0x1, 2, "mswsock.dll"),
    (
        sethostname,
        "sethostname",
        0x8000_0100,
        0x1,
        2,
        "mswsock.dll",
    ),
];

/// The `WSAID_*` value as `oracle.c` prints it: `data1-data2-data3-data4[0..2]-data4[2..8]`.
fn format_guid(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// The `WSAIoctl` fake's tag, the `fail` bit and the pointer name for each lookup key.
fn lookup_key(guid: &Guid) -> (char, u32, &'static str) {
    if *guid == WSAID_ACCEPTEX {
        ('a', 1, "AcceptEx")
    } else if *guid == WSAID_GETACCEPTEXSOCKADDRS {
        ('g', 2, "GetAcceptExSockaddrs")
    } else {
        ('t', 4, "TransmitFile")
    }
}

/// The `ioctl` log of one operation: `-` when the operation looked nothing up.
fn render_log(log: &[String]) -> String {
    if log.is_empty() {
        "-".to_string()
    } else {
        log.join(",")
    }
}

/// Runs one row's operations on fresh state and renders the printed row from `oracle.c`.
fn render_row(id: &str, ops: &str, fail: u32, ret: bool) -> String {
    let mut ext: Extensions<&'static str> = Extensions::default();
    let mut line = format!("{id} ops={ops} fail={fail} ret={} ::", u8::from(ret));
    for op in ops.chars() {
        let mut log: Vec<String> = Vec::new();
        let mut called = false;
        let ioctl = |socket: u32, guid: &Guid| {
            let (tag, bit, name) = lookup_key(guid);
            log.push(format!("{tag}@{socket:x}"));
            (fail & bit == 0).then_some(name)
        };
        match op {
            'A' => {
                let r = ext.accept_ex(0x100, ioctl, |_| {
                    called = true;
                    ret
                });
                line.push_str(&format!(
                    " A:{}:{}:{}",
                    u8::from(r),
                    u8::from(called),
                    render_log(&log)
                ));
            }
            'T' => {
                let r = ext.transmit_file(0x200, ioctl, |_| {
                    called = true;
                    ret
                });
                line.push_str(&format!(
                    " T:{}:{}:{}",
                    u8::from(r),
                    u8::from(called),
                    render_log(&log)
                ));
            }
            _ => {
                ext.get_accept_ex_sockaddrs(|_| called = true);
                line.push_str(&format!(" G:-:{}:{}", u8::from(called), render_log(&log)));
            }
        }
    }
    line.push_str(&format!(" init={}", u8::from(ext.initialised())));
    line
}

#[test]
fn guid_rows_match_oracle() {
    for (id, name, guid, printed) in GUID_ROWS {
        assert_eq!(format!("{id} {name} guid={}", format_guid(&guid)), printed);
    }
}

#[test]
fn extension_rows_match_oracle() {
    for (id, ops, fail, ret, printed) in EXTENSION_ROWS {
        assert_eq!(render_row(id, ops, fail, ret), printed);
    }
}

#[test]
fn wsarecvex_row_matches_oracle() {
    // W01: `WSARecvEx ret=-1`.
    assert_eq!(wsa_recv_ex(), -1);
}

#[test]
fn stub_rows_match_oracle() {
    for (helper, function, code, flags, number_parameters, module) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}
