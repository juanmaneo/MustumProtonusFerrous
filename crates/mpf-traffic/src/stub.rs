/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = traffic.dll` in `dlls/traffic/Makefile.in`.
const TRAFFIC_DLL_NAME: &str = "traffic.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// `TcAddClassMap`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_add_class_map() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcAddClassMap")
}

/// `TcAddFilter`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_add_filter() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcAddFilter")
}

/// `TcAddFlow`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_add_flow() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcAddFlow")
}

/// `TcCloseInterface`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_close_interface() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcCloseInterface")
}

/// `TcDeleteFilter`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_delete_filter() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcDeleteFilter")
}

/// `TcDeleteFlow`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_delete_flow() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcDeleteFlow")
}

/// `TcEnumerateFlows`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_enumerate_flows() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcEnumerateFlows")
}

/// `TcEnumerateInterfaces`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_enumerate_interfaces() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcEnumerateInterfaces")
}

/// `TcGetFlowNameA`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_get_flow_name_a() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcGetFlowNameA")
}

/// `TcGetFlowNameW`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_get_flow_name_w() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcGetFlowNameW")
}

/// `TcModifyFlow`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_modify_flow() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcModifyFlow")
}

/// `TcOpenInterfaceA`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_open_interface_a() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcOpenInterfaceA")
}

/// `TcOpenInterfaceW`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_open_interface_w() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcOpenInterfaceW")
}

/// `TcQueryFlowA`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_query_flow_a() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcQueryFlowA")
}

/// `TcQueryFlowW`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_query_flow_w() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcQueryFlowW")
}

/// `TcQueryInterface`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_query_interface() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcQueryInterface")
}

/// `TcSetFlowA`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_set_flow_a() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcSetFlowA")
}

/// `TcSetFlowW`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_set_flow_w() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcSetFlowW")
}

/// `TcSetInterface`, exported as `@ stub` in `dlls/traffic/traffic.spec`.
pub fn tc_set_interface() -> StubException {
    wine_spec_unimplemented_stub(TRAFFIC_DLL_NAME, "TcSetInterface")
}
