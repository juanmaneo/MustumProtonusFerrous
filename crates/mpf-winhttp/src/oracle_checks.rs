//! Replays the Wine oracle rows for `WinHttpCrackUrl` and `WinHttpCreateUrl` from Wine's
//! `dlls/winhttp/url.c` (commit e2aa6cd), compiled verbatim under ASan and UBSan. The rows are
//! the harness output for each clean vector; the expectations below were generated from it.
//!
//! Vectors excluded because the sanitizers flag the C code, so no expectation exists:
//! - X01 `crack http://h/a 0 z 8 - z z z`: verbatim_url.c:26:9: runtime error: null pointer passed as argument 2, which is declared to never be null
//! - X02 `create 0 0 - - - h 80 x - 10`: AddressSanitizer: heap-buffer-overflow
//! - X03 `create 0 3 - - - h 80 - - 64`: wine_mock.h:61:13: runtime error: load of null pointer of type 'const WCHAR'

use super::*;
use crate::crack::{crack_url, CrackedUrl, Piece, Slot, Slots};
use crate::create::{create_url, CreateError, UrlParts};

enum Cell {
    Absent,
    Text(&'static str),
    Overflow(usize),
}

struct CrackWant {
    status: u32,
    n_scheme: u32,
    port: u16,
    /// Scheme, user, password, host, path and extra, in that order.
    pieces: [Cell; 6],
}

struct CrackRow {
    id: &'static str,
    url: &'static str,
    flags: u32,
    slots: [Slot; 6],
    want: Result<CrackWant, u32>,
}

struct CreateRow {
    id: &'static str,
    flags: u32,
    n_scheme: u32,
    scheme: Option<&'static str>,
    user: Option<&'static str>,
    password: Option<&'static str>,
    host: Option<&'static str>,
    port: u16,
    path: Option<&'static str>,
    extra: Option<&'static str>,
    capacity: usize,
    want: Result<&'static str, CreateError>,
}

const CRACK_ROWS: &[CrackRow] = &[
    CrackRow {
        id: "C01",
        url: "http://example.com/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("http"),
                Cell::Absent,
                Cell::Absent,
                Cell::Text("example.com"),
                Cell::Text("/"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C02",
        url: "https://user:pw@host.example:8443/a/b?x=1#f",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 2,
            port: 8443,
            pieces: [
                Cell::Text("https"),
                Cell::Text("user"),
                Cell::Text("pw"),
                Cell::Text("host.example"),
                Cell::Text("/a/b"),
                Cell::Text("?x=1#f"),
            ],
        }),
    },
    CrackRow {
        id: "C03",
        url: "HTTP://EXAMPLE.org",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("HTTP"),
                Cell::Absent,
                Cell::Absent,
                Cell::Text("EXAMPLE.org"),
                Cell::Text(""),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C05",
        url: "ftp://host/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Err(ERROR_WINHTTP_UNRECOGNIZED_SCHEME),
    },
    CrackRow {
        id: "C06",
        url: "http:/host/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Err(ERROR_WINHTTP_INVALID_URL),
    },
    CrackRow {
        id: "C08",
        url: "http://host:8x9/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 8,
            pieces: [
                Cell::Text("http"),
                Cell::Absent,
                Cell::Absent,
                Cell::Text("host"),
                Cell::Text("/"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C09",
        url: "https://host:/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 2,
            port: 443,
            pieces: [
                Cell::Text("https"),
                Cell::Absent,
                Cell::Absent,
                Cell::Text("host"),
                Cell::Text("/"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C11",
        url: "http://u:@h/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("http"),
                Cell::Text("u"),
                Cell::Text(""),
                Cell::Text("h"),
                Cell::Text("/"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C12",
        url: "http://h/p@q",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("http"),
                Cell::Absent,
                Cell::Absent,
                Cell::Text("h"),
                Cell::Text("/p@q"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C16",
        url: "http://u:p@h/a b?x",
        flags: 0x80000000,
        slots: [
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(64),
            Slot::Buffer(16),
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("http"),
                Cell::Text("u"),
                Cell::Text("p"),
                Cell::Text("h"),
                Cell::Text("/a%20b"),
                Cell::Text("?x"),
            ],
        }),
    },
    CrackRow {
        id: "C18",
        url: "http://u%3ap:x@h/",
        flags: 0x10000000,
        slots: [
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(8),
        ],
        want: Ok(CrackWant {
            status: ERROR_SUCCESS,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("http"),
                Cell::Text("u"),
                Cell::Text("p:x"),
                Cell::Text("h"),
                Cell::Text("/"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C19",
        url: "http://u:p@host/",
        flags: 0x0,
        slots: [
            Slot::View,
            Slot::Buffer(8),
            Slot::Buffer(8),
            Slot::Buffer(2),
            Slot::View,
            Slot::View,
        ],
        want: Ok(CrackWant {
            status: ERROR_INSUFFICIENT_BUFFER,
            n_scheme: 1,
            port: 80,
            pieces: [
                Cell::Text("http"),
                Cell::Text("u"),
                Cell::Text("p"),
                Cell::Overflow(5),
                Cell::Text("/"),
                Cell::Text(""),
            ],
        }),
    },
    CrackRow {
        id: "C21",
        url: "http://h/\u{4e2d}",
        flags: 0x80000000,
        slots: [
            Slot::Buffer(8),
            Slot::Unrequested,
            Slot::Unrequested,
            Slot::Buffer(8),
            Slot::Buffer(64),
            Slot::Buffer(16),
        ],
        want: Err(ERROR_INVALID_PARAMETER),
    },
];

const CREATE_ROWS: &[CreateRow] = &[
    CreateRow {
        id: "K01",
        flags: 0x0,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: None,
        host: Some("example.com"),
        port: 80,
        path: Some("x"),
        extra: None,
        capacity: 64,
        want: Ok("http://example.com/x"),
    },
    CreateRow {
        id: "K02",
        flags: 0x0,
        n_scheme: 2,
        scheme: None,
        user: Some("u"),
        password: Some("p"),
        host: Some("h"),
        port: 443,
        path: Some("/"),
        extra: None,
        capacity: 64,
        want: Ok("https://u:p@h/"),
    },
    CreateRow {
        id: "K03",
        flags: 0x0,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: Some("p"),
        host: Some("h"),
        port: 80,
        path: None,
        extra: None,
        capacity: 64,
        want: Err(CreateError::Win32(ERROR_INVALID_PARAMETER)),
    },
    CreateRow {
        id: "K04",
        flags: 0x0,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: None,
        host: None,
        port: 8080,
        path: Some("/a"),
        extra: None,
        capacity: 64,
        want: Ok("http:///a"),
    },
    CreateRow {
        id: "K05",
        flags: 0x80000000,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: None,
        host: Some("h"),
        port: 80,
        path: Some("/a b"),
        extra: Some("?q=a b"),
        capacity: 64,
        want: Ok("http://h/a%20b?q=a%20b"),
    },
    CreateRow {
        id: "K06",
        flags: 0x0,
        n_scheme: 0,
        scheme: Some("HTTPS"),
        user: None,
        password: None,
        host: Some("h"),
        port: 443,
        path: None,
        extra: None,
        capacity: 64,
        want: Ok("HTTPS://h:443"),
    },
    CreateRow {
        id: "K07",
        flags: 0x0,
        n_scheme: 0,
        scheme: Some("ht"),
        user: None,
        password: None,
        host: Some("h"),
        port: 80,
        path: None,
        extra: None,
        capacity: 64,
        want: Ok("ht://h"),
    },
    CreateRow {
        id: "K08",
        flags: 0x0,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: None,
        host: Some("h"),
        port: 80,
        path: Some("x"),
        extra: None,
        capacity: 5,
        want: Err(CreateError::Insufficient { required: 11 }),
    },
    CreateRow {
        id: "K09",
        flags: 0x80000000,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: None,
        host: Some("h"),
        port: 80,
        path: Some("/%\u{fc}"),
        extra: None,
        capacity: 64,
        want: Ok("http://h/%25%FC"),
    },
    CreateRow {
        id: "K10",
        flags: 0x0,
        n_scheme: 0,
        scheme: None,
        user: None,
        password: None,
        host: Some("h"),
        port: 80,
        path: None,
        extra: Some("?x"),
        capacity: 64,
        want: Ok("http://h?x"),
    },
    CreateRow {
        id: "K11",
        flags: 0x0,
        n_scheme: 2,
        scheme: Some("http"),
        user: None,
        password: None,
        host: Some("h"),
        port: 80,
        path: None,
        extra: None,
        capacity: 64,
        want: Ok("http://h"),
    },
];

fn cell(cell: &Cell) -> Piece {
    match cell {
        Cell::Absent => Piece::Absent,
        Cell::Text(text) => Piece::Text(wide(text)),
        Cell::Overflow(len) => Piece::Overflow(*len),
    }
}

#[test]
fn oracle_crack_rows_match_wine() {
    for row in CRACK_ROWS {
        let url = wide(row.url);
        let [scheme, user, password, host, path, extra] = row.slots;
        let slots = Slots {
            scheme,
            user,
            password,
            host,
            path,
            extra,
        };
        match &row.want {
            Ok(want) => {
                let cracked = crack_url(&url, row.flags, &slots).expect(row.id);
                assert_eq!(cracked.status(), want.status, "{}", row.id);
                let [s, u, p, h, pa, e] = &want.pieces;
                let expected = CrackedUrl {
                    n_scheme: want.n_scheme,
                    port: want.port,
                    scheme: cell(s),
                    user: cell(u),
                    password: cell(p),
                    host: cell(h),
                    path: cell(pa),
                    extra: cell(e),
                };
                assert_eq!(cracked, expected, "{}", row.id);
            }
            Err(code) => {
                assert_eq!(crack_url(&url, row.flags, &slots), Err(*code), "{}", row.id);
            }
        }
    }
}

#[test]
fn oracle_create_rows_match_wine() {
    for row in CREATE_ROWS {
        let scheme = row.scheme.map(wide);
        let user = row.user.map(wide);
        let password = row.password.map(wide);
        let host = row.host.map(wide);
        let path = row.path.map(wide);
        let extra = row.extra.map(wide);
        let parts = UrlParts {
            scheme: scheme.as_deref(),
            n_scheme: row.n_scheme,
            user: user.as_deref(),
            password: password.as_deref(),
            host: host.as_deref(),
            port: row.port,
            path: path.as_deref(),
            extra: extra.as_deref(),
        };
        let got = create_url(&parts, row.flags, row.capacity);
        assert_eq!(got, row.want.map(wide), "{}", row.id);
    }
}
