use crate::stub::*;
use crate::StubException;

/// The `comm.drv16.spec` export table as `(ordinal, name)`, in spec order. Every entry is `@ stub`.
/// Ordinal 16 (`WEP`) is commented out in the spec, so it is not listed.
pub const EXPORTS: [(u16, &str); 21] = [
    (1, "INICOM"),
    (2, "SETCOM"),
    (3, "SETQUE"),
    (4, "RECCOM"),
    (5, "SNDCOM"),
    (6, "CTX"),
    (7, "TRMCOM"),
    (8, "STACOM"),
    (9, "CEXTFCN"),
    (10, "CFLUSH"),
    (11, "CEVT"),
    (12, "CEVTGET"),
    (13, "CSETBRK"),
    (14, "CCLRBRK"),
    (15, "GETDCB"),
    (17, "SUSPENDOPENCOMMPORTS"),
    (18, "REACTIVATEOPENCOMMPORTS"),
    (19, "COMMWRITESTRING"),
    (20, "READCOMMSTRING"),
    (100, "ENABLENOTIFICATION"),
    (101, "ordinal_only__win31"),
];

/// The stub record for the export `name`, or `None` when the spec does not export it. Names are
/// case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    let stub = match name {
        "INICOM" => inicom,
        "SETCOM" => setcom,
        "SETQUE" => setque,
        "RECCOM" => reccom,
        "SNDCOM" => sndcom,
        "CTX" => ctx,
        "TRMCOM" => trmcom,
        "STACOM" => stacom,
        "CEXTFCN" => cextfcn,
        "CFLUSH" => cflush,
        "CEVT" => cevt,
        "CEVTGET" => cevtget,
        "CSETBRK" => csetbrk,
        "CCLRBRK" => cclrbrk,
        "GETDCB" => getdcb,
        "SUSPENDOPENCOMMPORTS" => suspend_open_comm_ports,
        "REACTIVATEOPENCOMMPORTS" => reactivate_open_comm_ports,
        "COMMWRITESTRING" => comm_write_string,
        "READCOMMSTRING" => read_comm_string,
        "ENABLENOTIFICATION" => enable_notification,
        "ordinal_only__win31" => ordinal_only_win31,
        _ => return None,
    };
    Some(stub())
}
