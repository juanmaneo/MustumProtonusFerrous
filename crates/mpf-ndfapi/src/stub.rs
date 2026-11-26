/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = ndfapi.dll` in `dlls/ndfapi/Makefile.in`.
const NDFAPI_DLL_NAME: &str = "ndfapi.dll";

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

/// `DllGetClassObject`, exported as `@ stub -private` in `dlls/ndfapi/ndfapi.spec`.
pub fn dll_get_class_object() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "DllGetClassObject")
}

/// `NdfRunDllDiagnoseIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_run_dll_diagnose_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRunDllDiagnoseIncident")
}

/// `NdfRunDllDiagnoseNetConnectionIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_run_dll_diagnose_net_connection_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRunDllDiagnoseNetConnectionIncident")
}

/// `NdfRunDllDiagnoseWithAnswerFile`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_run_dll_diagnose_with_answer_file() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRunDllDiagnoseWithAnswerFile")
}

/// `NdfRunDllDuplicateIPDefendingSystem`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_run_dll_duplicate_ip_defending_system() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRunDllDuplicateIPDefendingSystem")
}

/// `NdfRunDllDuplicateIPOffendingSystem`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_run_dll_duplicate_ip_offending_system() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRunDllDuplicateIPOffendingSystem")
}

/// `NdfRunDllHelpTopic`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_run_dll_help_topic() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRunDllHelpTopic")
}

/// `NdfCancelIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_cancel_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCancelIncident")
}

/// `NdfCreateDNSIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_dns_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateDNSIncident")
}

/// `NdfCreateGroupingIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_grouping_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateGroupingIncident")
}

/// `NdfCreateInboundIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_inbound_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateInboundIncident")
}

/// `NdfCreateIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateIncident")
}

/// `NdfCreateNetConnectionIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_net_connection_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateNetConnectionIncident")
}

/// `NdfCreatePnrpIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_pnrp_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreatePnrpIncident")
}

/// `NdfCreateSharingIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_sharing_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateSharingIncident")
}

/// `NdfCreateWebIncidentEx`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_create_web_incident_ex() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfCreateWebIncidentEx")
}

/// `NdfGetTraceFile`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_get_trace_file() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfGetTraceFile")
}

/// `NdfRepairIncident`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_repair_incident() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRepairIncident")
}

/// `NdfRepairIncidentEx`, exported as `@ stub` in `dlls/ndfapi/ndfapi.spec`.
pub fn ndf_repair_incident_ex() -> StubException {
    wine_spec_unimplemented_stub(NDFAPI_DLL_NAME, "NdfRepairIncidentEx")
}
