//! Oracle-derived test vectors for the modelled comm.drv16 stubs.
//!
//! Vectors were captured by compiling the verbatim `__wine_spec_unimplemented_stub` body
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/comm_drv16/`, and running the harness. The harness calls the stub once per export
//! of `dlls/comm.drv16/comm.drv16.spec`, with the module name from `dlls/comm.drv16/Makefile.in`.
//! The recorder replaces `RtlRaiseException` and leaves through `longjmp`, because the C stub
//! loops. The 21 printed rows are all kept: ASan and UBSan flagged none of them, so none is
//! excluded.

use crate::stub::*;

#[test]
fn oracle_stub_rows() {
    // row 01 ordinal 1 INICOM | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=INICOM
    assert_eq!(
        inicom(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "INICOM",
        }
    );

    // row 02 ordinal 2 SETCOM | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=SETCOM
    assert_eq!(
        setcom(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "SETCOM",
        }
    );

    // row 03 ordinal 3 SETQUE | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=SETQUE
    assert_eq!(
        setque(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "SETQUE",
        }
    );

    // row 04 ordinal 4 RECCOM | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=RECCOM
    assert_eq!(
        reccom(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "RECCOM",
        }
    );

    // row 05 ordinal 5 SNDCOM | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=SNDCOM
    assert_eq!(
        sndcom(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "SNDCOM",
        }
    );

    // row 06 ordinal 6 CTX | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CTX
    assert_eq!(
        ctx(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CTX",
        }
    );

    // row 07 ordinal 7 TRMCOM | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=TRMCOM
    assert_eq!(
        trmcom(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "TRMCOM",
        }
    );

    // row 08 ordinal 8 STACOM | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=STACOM
    assert_eq!(
        stacom(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "STACOM",
        }
    );

    // row 09 ordinal 9 CEXTFCN | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CEXTFCN
    assert_eq!(
        cextfcn(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CEXTFCN",
        }
    );

    // row 10 ordinal 10 CFLUSH | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CFLUSH
    assert_eq!(
        cflush(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CFLUSH",
        }
    );

    // row 11 ordinal 11 CEVT | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CEVT
    assert_eq!(
        cevt(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CEVT",
        }
    );

    // row 12 ordinal 12 CEVTGET | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CEVTGET
    assert_eq!(
        cevtget(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CEVTGET",
        }
    );

    // row 13 ordinal 13 CSETBRK | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CSETBRK
    assert_eq!(
        csetbrk(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CSETBRK",
        }
    );

    // row 14 ordinal 14 CCLRBRK | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=CCLRBRK
    assert_eq!(
        cclrbrk(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "CCLRBRK",
        }
    );

    // row 15 ordinal 15 GETDCB | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=GETDCB
    assert_eq!(
        getdcb(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "GETDCB",
        }
    );

    // row 16 ordinal 17 SUSPENDOPENCOMMPORTS | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=SUSPENDOPENCOMMPORTS
    assert_eq!(
        suspend_open_comm_ports(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "SUSPENDOPENCOMMPORTS",
        }
    );

    // row 17 ordinal 18 REACTIVATEOPENCOMMPORTS | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=REACTIVATEOPENCOMMPORTS
    assert_eq!(
        reactivate_open_comm_ports(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "REACTIVATEOPENCOMMPORTS",
        }
    );

    // row 18 ordinal 19 COMMWRITESTRING | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=COMMWRITESTRING
    assert_eq!(
        comm_write_string(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "COMMWRITESTRING",
        }
    );

    // row 19 ordinal 20 READCOMMSTRING | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=READCOMMSTRING
    assert_eq!(
        read_comm_string(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "READCOMMSTRING",
        }
    );

    // row 20 ordinal 100 ENABLENOTIFICATION | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=ENABLENOTIFICATION
    assert_eq!(
        enable_notification(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "ENABLENOTIFICATION",
        }
    );

    // row 21 ordinal 101 ordinal_only__win31 | code=0x80000100 flags=0x1 nparams=2 module=comm.drv16 function=ordinal_only__win31
    assert_eq!(
        ordinal_only_win31(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "comm.drv16",
            function: "ordinal_only__win31",
        }
    );
}
