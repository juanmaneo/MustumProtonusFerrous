//! Oracle-derived test vectors for the ported olethk32 stub records.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g`, using
//! `target/canon/olethk32/oracle.c`, and running the harness. The harness has one wrapper per
//! `@ stub` in `dlls/olethk32/olethk32.spec`, plus one call with an empty name, which the spec
//! cannot produce. The 15 printed rows are all kept: ASan and UBSan flagged none of them, so no
//! vector is excluded.

use super::*;

#[test]
fn oracle_stub_rows() {
    // row 01 call=InvokeOn32 -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=InvokeOn32
    assert_eq!(
        invoke_on_32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "InvokeOn32",
        }
    );

    // row 02 call=IntOpInitialize -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=IntOpInitialize
    assert_eq!(
        int_op_initialize(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "IntOpInitialize",
        }
    );

    // row 03 call=CallbackProcessing_3216 -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=CallbackProcessing_3216
    assert_eq!(
        callback_processing_3216(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "CallbackProcessing_3216",
        }
    );

    // row 04 call=IUnknownObj32 -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=IUnknownObj32
    assert_eq!(
        i_unknown_obj32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "IUnknownObj32",
        }
    );

    // row 05 call=CSm16ReleaseHandler_Release32 -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=CSm16ReleaseHandler_Release32
    assert_eq!(
        c_sm16_release_handler_release32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "CSm16ReleaseHandler_Release32",
        }
    );

    // row 06 call=ThkMgrInitialize -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=ThkMgrInitialize
    assert_eq!(
        thk_mgr_initialize(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "ThkMgrInitialize",
        }
    );

    // row 07 call=ThkMgrUninitialize -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=ThkMgrUninitialize
    assert_eq!(
        thk_mgr_uninitialize(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "ThkMgrUninitialize",
        }
    );

    // row 08 call=TransformHRESULT_1632 -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=TransformHRESULT_1632
    assert_eq!(
        transform_hresult_1632(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "TransformHRESULT_1632",
        }
    );

    // row 09 call=TransformHRESULT_3216 -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=TransformHRESULT_3216
    assert_eq!(
        transform_hresult_3216(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "TransformHRESULT_3216",
        }
    );

    // row 10 call=ConvertObjDescriptor -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=ConvertObjDescriptor
    assert_eq!(
        convert_obj_descriptor(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "ConvertObjDescriptor",
        }
    );

    // row 11 call=ConvertHr1632Thunk -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=ConvertHr1632Thunk
    assert_eq!(
        convert_hr1632_thunk(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "ConvertHr1632Thunk",
        }
    );

    // row 12 call=ConvertHr3216Thunk -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=ConvertHr3216Thunk
    assert_eq!(
        convert_hr3216_thunk(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "ConvertHr3216Thunk",
        }
    );

    // row 13 call=IntOpUninitialize -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=IntOpUninitialize
    assert_eq!(
        int_op_uninitialize(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "IntOpUninitialize",
        }
    );

    // row 14 call=ThkAddAppCompatFlag -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=ThkAddAppCompatFlag
    assert_eq!(
        thk_add_app_compat_flag(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "ThkAddAppCompatFlag",
        }
    );

    // row 15 call=empty_name -> code=0x80000100 flags=0x1 nparams=2 p0=olethk32.dll p1=
    assert_eq!(
        wine_spec_unimplemented_stub("olethk32.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "olethk32.dll",
            function: "",
        }
    );
}
