//! Oracle-derived test vectors for the ported traffic logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `TcRegisterClient` and
//! `TcDeregisterClient` (`dlls/traffic/traffic_main.c`) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/traffic/`, and running the harness. The 23 printed rows cover 4 explicit calls
//! and the 19 `@ stub` exports. The harness ran on 64-bit Linux, and ASan and UBSan reported
//! nothing for any row, so no vector is excluded.

use super::*;

/// The stub record every `@ stub` export raises. Only the function name differs per export.
fn stub_record(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "traffic.dll",
        function,
    }
}

#[test]
fn oracle_explicit_rows() {
    // row 01 TcRegisterClient(version=1 ctx=NULL list=NULL buffer=NULL) -> ret=120
    assert_eq!(tc_register_client(None), 120);

    // row 02 TcRegisterClient(version=2 ctx=0x1234 list=NULL buffer=&sentinel) -> ret=120 buffer=0xffffffffffffffff
    let mut buffer: Handle = 0x55;
    assert_eq!(tc_register_client(Some(&mut buffer)), 120);
    assert_eq!(buffer, 0xffff_ffff_ffff_ffff);

    // row 03 TcDeregisterClient(NULL) -> ret=120
    assert_eq!(tc_deregister_client(), 120);

    // row 04 TcDeregisterClient(0x1234) -> ret=120
    assert_eq!(tc_deregister_client(), 120);
}

#[test]
fn oracle_stub_rows() {
    // row 05 TcAddClassMap -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcAddClassMap
    assert_eq!(tc_add_class_map(), stub_record("TcAddClassMap"));
    // row 06 TcAddFilter -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcAddFilter
    assert_eq!(tc_add_filter(), stub_record("TcAddFilter"));
    // row 07 TcAddFlow -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcAddFlow
    assert_eq!(tc_add_flow(), stub_record("TcAddFlow"));
    // row 08 TcCloseInterface -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcCloseInterface
    assert_eq!(tc_close_interface(), stub_record("TcCloseInterface"));
    // row 09 TcDeleteFilter -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcDeleteFilter
    assert_eq!(tc_delete_filter(), stub_record("TcDeleteFilter"));
    // row 10 TcDeleteFlow -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcDeleteFlow
    assert_eq!(tc_delete_flow(), stub_record("TcDeleteFlow"));
    // row 11 TcEnumerateFlows -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcEnumerateFlows
    assert_eq!(tc_enumerate_flows(), stub_record("TcEnumerateFlows"));
    // row 12 TcEnumerateInterfaces -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcEnumerateInterfaces
    assert_eq!(
        tc_enumerate_interfaces(),
        stub_record("TcEnumerateInterfaces")
    );
    // row 13 TcGetFlowNameA -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcGetFlowNameA
    assert_eq!(tc_get_flow_name_a(), stub_record("TcGetFlowNameA"));
    // row 14 TcGetFlowNameW -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcGetFlowNameW
    assert_eq!(tc_get_flow_name_w(), stub_record("TcGetFlowNameW"));
    // row 15 TcModifyFlow -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcModifyFlow
    assert_eq!(tc_modify_flow(), stub_record("TcModifyFlow"));
    // row 16 TcOpenInterfaceA -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcOpenInterfaceA
    assert_eq!(tc_open_interface_a(), stub_record("TcOpenInterfaceA"));
    // row 17 TcOpenInterfaceW -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcOpenInterfaceW
    assert_eq!(tc_open_interface_w(), stub_record("TcOpenInterfaceW"));
    // row 18 TcQueryFlowA -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcQueryFlowA
    assert_eq!(tc_query_flow_a(), stub_record("TcQueryFlowA"));
    // row 19 TcQueryFlowW -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcQueryFlowW
    assert_eq!(tc_query_flow_w(), stub_record("TcQueryFlowW"));
    // row 20 TcQueryInterface -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcQueryInterface
    assert_eq!(tc_query_interface(), stub_record("TcQueryInterface"));
    // row 21 TcSetFlowA -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcSetFlowA
    assert_eq!(tc_set_flow_a(), stub_record("TcSetFlowA"));
    // row 22 TcSetFlowW -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcSetFlowW
    assert_eq!(tc_set_flow_w(), stub_record("TcSetFlowW"));
    // row 23 TcSetInterface -> code=0x80000100 flags=0x1 params=2 module=traffic.dll function=TcSetInterface
    assert_eq!(tc_set_interface(), stub_record("TcSetInterface"));
}
