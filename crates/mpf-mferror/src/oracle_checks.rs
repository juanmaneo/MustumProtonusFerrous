//! Oracle-derived test vectors for the ported mferror message table.
//!
//! Vectors were captured by compiling the verbatim Wine `complete_msg` and `sort_lanmsg` from
//! `tools/wmc/mcy.y` with `gcc -fsanitize=address,undefined -g` under `target/canon/mferror/`, and
//! running the harness once per `MessageId` entry of `dlls/mferror/mferror.mc` (101 rows). UBSan
//! flags 98 of those rows, all the `Severity=Error` entries: `last_sev << 30` shifts a signed `int`
//! into its sign bit (`left shift of 3 by 30 places cannot be represented in type 'int'`). Those 98
//! rows are excluded here. The 3 unflagged rows are the `Severity=Success` entries, which shift 0.
//! The excluded values are checked against Wine's `include/mferror.h` in `edge_checks.rs`.

use super::*;

/// `(SymbolicName, MessageId, severity, facility, realid)` for the harness rows that UBSan does not flag.
const ORACLE_ROWS: [(&str, u32, u32, u32, u32); 3] = [
    ("MF_S_MULTIPLE_BEGIN", 14040, 0, 0xd, 0x000d_36d8),
    ("MF_S_SINK_NOT_FINALIZED", 19010, 0, 0xd, 0x000d_4a42),
    ("MF_S_CLOCK_STOPPED", 40004, 0, 0xd, 0x000d_9c44),
];

#[test]
fn oracle_message_id_rows() {
    for (symbol, id, severity, facility, realid) in ORACLE_ROWS {
        assert_eq!(make_message_id(severity, facility, id), realid);
        let message = find_by_symbol(symbol).unwrap();
        assert_eq!(message.id, id);
        assert_eq!(message.severity, severity);
        assert_eq!(message.hresult(), realid);
    }
}
