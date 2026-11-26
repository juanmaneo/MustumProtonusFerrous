//! Edge-case tests for the ported tdi.sys logic.
//!
//! These cover the export table, the lookup for names that are not exported, and the stub record
//! shape. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and include/ntstatus.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(STATUS_SUCCESS, 0);
}

#[test]
fn stub_record_carries_the_names_it_is_given() {
    // Not oracle-derived: the record layout of __wine_spec_unimplemented_stub, which stores the
    // module and function pointers as ExceptionInformation[0] and [1] and sets NumberParameters to 2.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Func"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Func",
        }
    );
}

#[test]
fn export_table_has_56_unique_names_and_each_resolves_to_its_own_stub() {
    // Not oracle-derived: EXPORTS must match the 56 `@ stub` lines in tdi.sys.spec, in spec order.
    let unique: HashSet<&str> = EXPORTS.iter().copied().collect();
    assert_eq!(EXPORTS.len(), 56);
    assert_eq!(unique.len(), EXPORTS.len());
    assert_eq!(EXPORTS[0], "CTEAllocateString");
    assert_eq!(EXPORTS[55], "TdiReturnChainedReceives");
    for name in EXPORTS {
        let stub = export_stub(name).expect(name);
        assert_eq!(stub.module, "tdi.sys");
        assert_eq!(stub.function, name);
    }
}

#[test]
fn names_outside_the_export_table_have_no_stub() {
    // Not oracle-derived: DriverEntry is compiled into the DLL but tdi.sys.spec does not export it.
    // The lookup is case-sensitive, so lowercase or uppercase variants of a real export miss too.
    assert_eq!(export_stub(""), None);
    assert_eq!(export_stub("DriverEntry"), None);
    assert_eq!(export_stub("cteblock"), None);
    assert_eq!(export_stub("CTEBLOCK"), None);
}

#[test]
fn snake_case_helpers_keep_acronyms_and_digits() {
    // Not oracle-derived: checks the snake_case naming rule on the awkward C names. The C names
    // come from tdi.sys.spec; the helpers must still call with the exact spelling.
    assert_eq!(tdi_get9f_triage_block().function, "TdiGet9FTriageBlock");
    assert_eq!(tdi_pnp_power_complete().function, "TdiPnPPowerComplete");
    assert_eq!(
        tdi_register_pnp_handlers().function,
        "TdiRegisterPnPHandlers"
    );
    assert_eq!(cte_system_up_time().function, "CTESystemUpTime");
    assert_eq!(dll_initialize().function, "DllInitialize");
}
