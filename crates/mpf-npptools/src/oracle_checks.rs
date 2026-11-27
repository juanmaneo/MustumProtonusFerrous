//! Oracle-derived test vectors for the npptools export stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504 to 516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/npptools/`, and calling it once for each of the 60 `@ stub` names in
//! `dlls/npptools/npptools.spec`, in spec order. The harness replaces `RtlRaiseException` with a
//! recorder that jumps back with `longjmp`, because the C loop would otherwise never end. ASan and
//! UBSan reported nothing for any row, so no vector is excluded and all 60 rows remain.

use super::*;

/// One row per `@ stub` export, in spec order: (export name, code, flags, number of parameters,
/// module, function).
#[rustfmt::skip]
const ORACLE_ROWS: [(&str, u32, u32, u32, &str, &str); 60] = [
    ("ClearEventData", 0x8000_0100, 0x01, 2, "npptools.dll", "ClearEventData"),
    ("CreateBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "CreateBlob"),
    ("CreateNPPInterface", 0x8000_0100, 0x01, 2, "npptools.dll", "CreateNPPInterface"),
    ("DestroyBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "DestroyBlob"),
    ("DestroyNPPBlobTable", 0x8000_0100, 0x01, 2, "npptools.dll", "DestroyNPPBlobTable"),
    ("DuplicateBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "DuplicateBlob"),
    ("FilterNPPBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "FilterNPPBlob"),
    ("FindOneOf", 0x8000_0100, 0x01, 2, "npptools.dll", "FindOneOf"),
    ("FindUnknownBlobCategories", 0x8000_0100, 0x01, 2, "npptools.dll", "FindUnknownBlobCategories"),
    ("FindUnknownBlobTags", 0x8000_0100, 0x01, 2, "npptools.dll", "FindUnknownBlobTags"),
    ("GetBoolFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetBoolFromBlob"),
    ("GetClassIDFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetClassIDFromBlob"),
    ("GetDwordFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetDwordFromBlob"),
    ("GetMacAddressFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetMacAddressFromBlob"),
    ("GetNPPAddressFilterFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPAddressFilterFromBlob"),
    ("GetNPPBlobFromUI", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPBlobFromUI"),
    ("GetNPPBlobTable", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPBlobTable"),
    ("GetNPPEtypeSapFilter", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPEtypeSapFilter"),
    ("GetNPPMacTypeAsNumber", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPMacTypeAsNumber"),
    ("GetNPPPatternFilterFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPPatternFilterFromBlob"),
    ("GetNPPTriggerFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNPPTriggerFromBlob"),
    ("GetNetworkInfoFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetNetworkInfoFromBlob"),
    ("GetStringFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetStringFromBlob"),
    ("GetStringsFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "GetStringsFromBlob"),
    ("IsRemoteNPP", 0x8000_0100, 0x01, 2, "npptools.dll", "IsRemoteNPP"),
    ("LockBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "LockBlob"),
    ("MarshalBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "MarshalBlob"),
    ("MergeBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "MergeBlob"),
    ("NmAddUsedEntry", 0x8000_0100, 0x01, 2, "npptools.dll", "NmAddUsedEntry"),
    ("NmHeapAllocate", 0x8000_0100, 0x01, 2, "npptools.dll", "NmHeapAllocate"),
    ("NmHeapFree", 0x8000_0100, 0x01, 2, "npptools.dll", "NmHeapFree"),
    ("NmHeapReallocate", 0x8000_0100, 0x01, 2, "npptools.dll", "NmHeapReallocate"),
    ("NmHeapSetMaxSize", 0x8000_0100, 0x01, 2, "npptools.dll", "NmHeapSetMaxSize"),
    ("NmHeapSize", 0x8000_0100, 0x01, 2, "npptools.dll", "NmHeapSize"),
    ("NmRemoveUsedEntry", 0x8000_0100, 0x01, 2, "npptools.dll", "NmRemoveUsedEntry"),
    ("RaiseNMEvent", 0x8000_0100, 0x01, 2, "npptools.dll", "RaiseNMEvent"),
    ("ReadBlobFromFile", 0x8000_0100, 0x01, 2, "npptools.dll", "ReadBlobFromFile"),
    ("RegCreateBlobKey", 0x8000_0100, 0x01, 2, "npptools.dll", "RegCreateBlobKey"),
    ("RegOpenBlobKey", 0x8000_0100, 0x01, 2, "npptools.dll", "RegOpenBlobKey"),
    ("ReleaseEventSystem", 0x8000_0100, 0x01, 2, "npptools.dll", "ReleaseEventSystem"),
    ("RemoveFromBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "RemoveFromBlob"),
    ("SelectNPPBlobFromTable", 0x8000_0100, 0x01, 2, "npptools.dll", "SelectNPPBlobFromTable"),
    ("SendEvent", 0x8000_0100, 0x01, 2, "npptools.dll", "SendEvent"),
    ("SetBoolInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetBoolInBlob"),
    ("SetClassIDInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetClassIDInBlob"),
    ("SetDwordInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetDwordInBlob"),
    ("SetMacAddressInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetMacAddressInBlob"),
    ("SetNPPAddressFilterInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetNPPAddressFilterInBlob"),
    ("SetNPPEtypeSapFilter", 0x8000_0100, 0x01, 2, "npptools.dll", "SetNPPEtypeSapFilter"),
    ("SetNPPPatternFilterInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetNPPPatternFilterInBlob"),
    ("SetNPPTriggerInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetNPPTriggerInBlob"),
    ("SetNetworkInfoInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetNetworkInfoInBlob"),
    ("SetStringInBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "SetStringInBlob"),
    ("SubkeyExists", 0x8000_0100, 0x01, 2, "npptools.dll", "SubkeyExists"),
    ("UnMarshalBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "UnMarshalBlob"),
    ("UnlockBlob", 0x8000_0100, 0x01, 2, "npptools.dll", "UnlockBlob"),
    ("WriteBlobToFile", 0x8000_0100, 0x01, 2, "npptools.dll", "WriteBlobToFile"),
    ("WriteCrackedBlobToFile", 0x8000_0100, 0x01, 2, "npptools.dll", "WriteCrackedBlobToFile"),
    ("recursiveDeleteKey", 0x8000_0100, 0x01, 2, "npptools.dll", "recursiveDeleteKey"),
    ("setKeyAndValue", 0x8000_0100, 0x01, 2, "npptools.dll", "setKeyAndValue"),
];

#[test]
fn oracle_stub_rows() {
    for (export, code, flags, number_parameters, module, function) in ORACLE_ROWS {
        assert_eq!(
            call_export(export),
            Some(StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            })
        );
    }
}
