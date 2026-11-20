//! Oracle rows from Wine's `inet_addr` in `dlls/ws2_32/protocol.c`.
//!
//! The verbatim `inet_addr` body was compiled with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/ws2_32` (a shim supplies only `SetLastError` and the trace macro). The rows below
//! were clean, and their expected values were captured from that executable. Inputs with bytes
//! above 0x7f are not used: passing a negative plain `char` to `isspace`/`isdigit` is undefined
//! in C, so Wine's `1.2.3 \xfe\xff` row is not replayed.

use super::inet_addr::{inet_addr, INADDR_NONE};

#[test]
fn matches_the_sanitized_wine_oracle_rows() {
    let rows: &[(&[u8], u32)] = &[
        (b"1.2.3.4", 0x0403_0201),
        (b"1 2 3 4", 0x0100_0000),
        (b"1.2.3. 4", INADDR_NONE),
        (b"1.2.3 .4", 0x0300_0201),
        (b"3.4.5.6.7", INADDR_NONE),
        (b" 3.4.5.6", INADDR_NONE),
        (b"3.4.5.6 ", 0x0605_0403),
        (b"3. 4.5.6", INADDR_NONE),
        (b"3 .4.5.6", 0x0300_0000),
        (b"1.2.3", 0x0300_0201),
        (b".1.2.3", INADDR_NONE),
        (b"0.0.0.0", 0),
        (b"", INADDR_NONE),
        (b" ", 0),
        (b"  ", INADDR_NONE),
        (b"127.127.127.255", 0xff7f_7f7f),
        (b"127.0.0.1:123", INADDR_NONE),
        (b"127.127.127.256", INADDR_NONE),
        (b"0xa1a2b3b4 ", 0xb4b3_a2a1),
        (b"1.2.3.0xaA", 0xaa03_0201),
        (b"1.2.3.010", 0x0803_0201),
        (b"1.2", 0x0200_0001),
        (b"203569230", 0x4e38_220c),
        (b"0x00010203", 0x0302_0100),
        (b"017700000001", 0x0100_007f),
    ];

    for (input, expected) in rows {
        assert_eq!(inet_addr(Some(input)), Ok(*expected), "input: {input:?}");
    }
}
