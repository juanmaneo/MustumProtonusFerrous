//! Oracle-derived test vectors for the ported ndfapi logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the six explicit exports
//! (`dlls/ndfapi/ndfapi_main.c`, lines 30-70) and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/ndfapi/`, and running the harness. The 25 printed rows cover the six explicit
//! exports and the 19 `@ stub` exports. ASan and UBSan flagged no row, so none is excluded.

use super::*;

/// The record the harness printed for a `@ stub` export: `code=0x80000100 flags=0x1 nargs=2
/// module=ndfapi.dll function=<name>`.
fn raised(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module: "ndfapi.dll",
        function,
    }
}

#[test]
fn oracle_explicit_export_rows() {
    // row 01 NdfCloseIncident handle=NULL -> hr=0x80004001
    assert_eq!(ndf_close_incident(), 0x8000_4001u32 as i32);
    // row 02 NdfCreateConnectivityIncident handle_in=0x1234 -> hr=0x80004001 handle_out=0x1234
    assert_eq!(ndf_create_connectivity_incident(), 0x8000_4001u32 as i32);
    // row 03 NdfCreateWebIncident url=http://example.com handle_in=0x1234 -> hr=0x80004001 handle_out=0x1234
    assert_eq!(ndf_create_web_incident(), 0x8000_4001u32 as i32);
    // row 04 NdfCreateWinSockIncident sock=3 host=example port=80 app=app userId=NULL handle_in=0x1234 -> hr=0x80004001 handle_out=0x1234
    assert_eq!(ndf_create_winsock_incident(), 0x8000_4001u32 as i32);
    // row 05 NdfDiagnoseIncident handle=0x1234 wait=INFINITE flags=0 count_in=7 -> hr=0x80004001 count_out=7 causes_out=0x5678
    assert_eq!(ndf_diagnose_incident(), 0x8000_4001u32 as i32);
    // row 06 NdfExecuteDiagnosis handle=0x1234 hwnd=0x5678 -> hr=0x80004001
    assert_eq!(ndf_execute_diagnosis(), 0x8000_4001u32 as i32);
}

#[test]
fn oracle_stub_rows() {
    // row 07 function=DllGetClassObject
    assert_eq!(dll_get_class_object(), raised("DllGetClassObject"));
    // row 08 function=NdfRunDllDiagnoseIncident
    assert_eq!(
        ndf_run_dll_diagnose_incident(),
        raised("NdfRunDllDiagnoseIncident")
    );
    // row 09 function=NdfRunDllDiagnoseNetConnectionIncident
    assert_eq!(
        ndf_run_dll_diagnose_net_connection_incident(),
        raised("NdfRunDllDiagnoseNetConnectionIncident")
    );
    // row 10 function=NdfRunDllDiagnoseWithAnswerFile
    assert_eq!(
        ndf_run_dll_diagnose_with_answer_file(),
        raised("NdfRunDllDiagnoseWithAnswerFile")
    );
    // row 11 function=NdfRunDllDuplicateIPDefendingSystem
    assert_eq!(
        ndf_run_dll_duplicate_ip_defending_system(),
        raised("NdfRunDllDuplicateIPDefendingSystem")
    );
    // row 12 function=NdfRunDllDuplicateIPOffendingSystem
    assert_eq!(
        ndf_run_dll_duplicate_ip_offending_system(),
        raised("NdfRunDllDuplicateIPOffendingSystem")
    );
    // row 13 function=NdfRunDllHelpTopic
    assert_eq!(ndf_run_dll_help_topic(), raised("NdfRunDllHelpTopic"));
    // row 14 function=NdfCancelIncident
    assert_eq!(ndf_cancel_incident(), raised("NdfCancelIncident"));
    // row 15 function=NdfCreateDNSIncident
    assert_eq!(ndf_create_dns_incident(), raised("NdfCreateDNSIncident"));
    // row 16 function=NdfCreateGroupingIncident
    assert_eq!(
        ndf_create_grouping_incident(),
        raised("NdfCreateGroupingIncident")
    );
    // row 17 function=NdfCreateInboundIncident
    assert_eq!(
        ndf_create_inbound_incident(),
        raised("NdfCreateInboundIncident")
    );
    // row 18 function=NdfCreateIncident
    assert_eq!(ndf_create_incident(), raised("NdfCreateIncident"));
    // row 19 function=NdfCreateNetConnectionIncident
    assert_eq!(
        ndf_create_net_connection_incident(),
        raised("NdfCreateNetConnectionIncident")
    );
    // row 20 function=NdfCreatePnrpIncident
    assert_eq!(ndf_create_pnrp_incident(), raised("NdfCreatePnrpIncident"));
    // row 21 function=NdfCreateSharingIncident
    assert_eq!(
        ndf_create_sharing_incident(),
        raised("NdfCreateSharingIncident")
    );
    // row 22 function=NdfCreateWebIncidentEx
    assert_eq!(
        ndf_create_web_incident_ex(),
        raised("NdfCreateWebIncidentEx")
    );
    // row 23 function=NdfGetTraceFile
    assert_eq!(ndf_get_trace_file(), raised("NdfGetTraceFile"));
    // row 24 function=NdfRepairIncident
    assert_eq!(ndf_repair_incident(), raised("NdfRepairIncident"));
    // row 25 function=NdfRepairIncidentEx
    assert_eq!(ndf_repair_incident_ex(), raised("NdfRepairIncidentEx"));
}
