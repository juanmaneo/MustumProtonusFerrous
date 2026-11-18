//! Oracle tests generated from `target/canon/jsproxy` by `gen_rows.py`. Do not edit.
//!
//! Each row is one line of the Wine oracle output for `dlls/jsproxy/main.c` at Wine
//! commit 9f8c4ae, built with `gcc -fsanitize=address,undefined` from the verbatim
//! function bodies. The rows are `target/canon/jsproxy/vectors.txt`, in order; 25 rows
//! and none were excluded, because the sanitizers reported nothing.

use super::*;

const S_OK: u32 = 0;
const S_FALSE: u32 = 1;
/// Wine's `AF_INET6`, the family of the `v6only` and `mixed` oracle hosts.
const AF_INET6: i32 = 23;

/// The name `dispex_GetDispID` got, the `HRESULT`, and the id it returned.
const DISPID_ROWS: &[(&str, u32, Option<i32>)] = &[
    ("dns_resolve", 0x00000000, Some(4096)),
    ("dns_resolve2", 0x80020006, None),
    ("", 0x80020006, None),
    ("Dns_resolve", 0x80020006, None),
];

/// One `InvokeEx` call: the dispatch id, the arguments (`Some` is a BSTR, `None` another VARIANT), the `HRESULT` and the BSTR result.
struct InvokeRow {
    id: i32,
    args: &'static [Option<&'static str>],
    hr: u32,
    text: Option<&'static str>,
}

const INVOKE_ROWS: &[InvokeRow] = &[
    InvokeRow {
        id: 4096,
        args: &[Some("single")],
        hr: 0x00000000,
        text: Some("192.168.0.1"),
    },
    InvokeRow {
        id: 4096,
        args: &[],
        hr: 0x8002000E,
        text: None,
    },
    InvokeRow {
        id: 4096,
        args: &[None],
        hr: 0x80020008,
        text: None,
    },
    InvokeRow {
        id: 4097,
        args: &[Some("single")],
        hr: 0x80020003,
        text: None,
    },
    InvokeRow {
        id: 4096,
        args: &[Some("lookup_fail")],
        hr: 0x00000001,
        text: None,
    },
];

/// The `IN` argument of `printf_addr` and its dotted-decimal text.
const FORMAT_ROWS: &[(u32, &str)] = &[(0x0100007F, "127.0.0.1"), (0xFFFFFFFF, "255.255.255.255")];

/// The host, the `HRESULT` and the BSTR result of `dns_resolve`.
const DNS_ROWS: &[(&str, u32, Option<&str>)] = &[
    ("mixed", 0x00000000, Some("10.1.2.3")),
    ("v6only", 0x00000001, None),
    ("none", 0x00000001, None),
    ("lookup_fail", 0x00000001, None),
];

/// One `InternetInitializeAutoProxyDll` call: the buffer, the previous script, the file, and the result.
struct InitRow {
    buffer: Option<(u32, Option<&'static str>, u32)>,
    prev: Option<&'static str>,
    file: Option<&'static str>,
    ret: bool,
    last_error: u32,
    state: Option<&'static str>,
}

const INIT_ROWS: &[InitRow] = &[
    InitRow {
        buffer: None,
        prev: None,
        file: Some("hello"),
        ret: true,
        last_error: 0,
        state: Some("hello"),
    },
    InitRow {
        buffer: None,
        prev: Some("old"),
        file: None,
        ret: false,
        last_error: 0,
        state: None,
    },
    InitRow {
        buffer: Some((AUTO_PROXY_SCRIPT_BUFFER_SIZE, Some("abcd"), 4)),
        prev: Some("old"),
        file: None,
        ret: true,
        last_error: 0,
        state: Some("abcd"),
    },
    InitRow {
        buffer: Some((AUTO_PROXY_SCRIPT_BUFFER_SIZE, Some(""), 0)),
        prev: Some("old"),
        file: Some("file"),
        ret: false,
        last_error: 87,
        state: Some("old"),
    },
    InitRow {
        buffer: Some((16, Some("abcd"), 4)),
        prev: Some("old"),
        file: Some("file"),
        ret: true,
        last_error: 0,
        state: Some("file"),
    },
    InitRow {
        buffer: Some((AUTO_PROXY_SCRIPT_BUFFER_SIZE, None, 4)),
        prev: None,
        file: None,
        ret: false,
        last_error: 0,
        state: None,
    },
];

/// The previous script and the script after `InternetDeInitializeAutoProxyDll`.
const DEINIT_ROWS: &[(Option<&str>, Option<&str>)] = &[(Some("old"), None)];

/// One `InternetGetProxyInfo` call. `ran` is what the run stub received: script, URL and hostname, or `NULL`.
struct ProxyRow {
    prev: Option<&'static str>,
    url: &'static str,
    host: Option<&'static str>,
    ret: bool,
    last_error: u32,
    ran: Option<&'static str>,
}

const PROXY_ROWS: &[ProxyRow] = &[
    ProxyRow {
        prev: None,
        url: "u.test/a",
        host: Some("h.test"),
        ret: false,
        last_error: 1003,
        ran: None,
    },
    ProxyRow {
        prev: Some("script"),
        url: "u.test/a",
        host: None,
        ret: true,
        last_error: 0,
        ran: Some("script;u.test/a;NULL"),
    },
    ProxyRow {
        prev: Some("script"),
        url: "u.test/a",
        host: Some("h.test"),
        ret: false,
        last_error: 0,
        ran: Some("script;u.test/a;h.test"),
    },
];

/// The `getaddrinfo` results of the C shim's host table: family and host-order IPv4 address per entry.
/// One host of the shim: its name and its `getaddrinfo` entries, or `None` for a failed lookup.
struct Host {
    name: &'static str,
    entries: Option<&'static [(i32, u32)]>,
}

const HOSTS: &[Host] = &[
    Host {
        name: "lookup_fail",
        entries: None,
    },
    Host {
        name: "none",
        entries: Some(&[]),
    },
    Host {
        name: "v6only",
        entries: Some(&[(AF_INET6, 0)]),
    },
    Host {
        name: "mixed",
        entries: Some(&[
            (AF_INET6, 0),
            (AF_INET, 0x0A01_0203),
            (AF_INET, 0x0405_0607),
        ]),
    },
    Host {
        name: "single",
        entries: Some(&[(AF_INET, 0xC0A8_0001)]),
    },
];

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn fixture(hostname: &[u16]) -> Option<Vec<AddrEntry>> {
    let name = String::from_utf16(hostname).unwrap();
    let host = HOSTS.iter().find(|host| host.name == name).unwrap();
    host.entries.map(|list| {
        list.iter()
            .map(|&(family, host)| AddrEntry {
                family,
                s_addr: host.to_be(),
            })
            .collect()
    })
}

/// `dns_resolve` on a non-empty name: `S_OK` with the BSTR, or `S_FALSE`.
fn dns_result(hostname: &[u16]) -> (u32, Option<String>) {
    match first_ipv4_text(fixture(hostname).as_deref()) {
        Some(text) => (S_OK, Some(text)),
        None => (S_FALSE, None),
    }
}

#[test]
fn dispid_rows_match_wine() {
    for &(name, hr, id) in DISPID_ROWS {
        let got = match dispid_for_name(&utf16(name)) {
            Ok(id) => (S_OK, Some(id)),
            Err(code) => (code as u32, None),
        };
        assert_eq!(got, (hr, id));
    }
}

#[test]
fn invoke_rows_match_wine() {
    for row in INVOKE_ROWS {
        let owned: Vec<Option<Vec<u16>>> = row.args.iter().map(|arg| arg.map(utf16)).collect();
        let call: Vec<InvokeArg> = owned
            .iter()
            .map(|arg| arg.as_deref().map_or(InvokeArg::Other, InvokeArg::Bstr))
            .collect();
        let got = match invoke_ex(row.id, &call) {
            Ok(name) => dns_result(name),
            Err(code) => (code as u32, None),
        };
        assert_eq!(got, (row.hr, row.text.map(str::to_string)));
    }
}

#[test]
fn format_rows_match_wine() {
    for &(s_addr, text) in FORMAT_ROWS {
        assert_eq!(ipv4_text(s_addr), text);
    }
}

#[test]
fn dns_rows_match_wine() {
    for &(host, hr, text) in DNS_ROWS {
        assert_eq!(dns_result(&utf16(host)), (hr, text.map(str::to_string)));
    }
}

#[test]
fn init_rows_match_wine() {
    for row in INIT_ROWS {
        let mut state = row.prev.map(utf16);
        let text = row.buffer.and_then(|(_, script, _)| script).map(utf16);
        let buffer = row
            .buffer
            .map(|(struct_size, _, script_size)| ScriptBuffer {
                struct_size,
                script: text.as_deref(),
                script_size,
            });
        let file = row.file.map(utf16);
        let (ret, last_error) = match initialize_script(&mut state, buffer, || file) {
            InitOutcome::Loaded => (true, 0),
            InitOutcome::LoadFailed => (false, 0),
            InitOutcome::InvalidParameter => (false, ERROR_INVALID_PARAMETER),
        };
        assert_eq!((ret, last_error), (row.ret, row.last_error));
        assert_eq!(state, row.state.map(utf16));
    }
}

#[test]
fn deinit_rows_match_wine() {
    for &(prev, state) in DEINIT_ROWS {
        let mut script = prev.map(utf16);
        deinitialize_script(&mut script);
        assert_eq!(script, state.map(utf16));
    }
}

#[test]
fn proxy_rows_match_wine() {
    for row in PROXY_ROWS {
        let state = row.prev.map(utf16);
        match proxy_script(state.as_deref()) {
            Err(code) => assert_eq!((row.ret, row.last_error, row.ran), (false, code, None)),
            Ok(script) => {
                let run = format!(
                    "{};{};{}",
                    String::from_utf16(script).unwrap(),
                    row.url,
                    row.host.unwrap_or("NULL")
                );
                assert_eq!((row.last_error, row.ran), (0, Some(run.as_str())));
            }
        }
    }
}
