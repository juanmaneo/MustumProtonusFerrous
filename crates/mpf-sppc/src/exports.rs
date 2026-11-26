/// How `dlls/sppc/sppc.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real C function, ported as one of the `sl_*` functions in this crate.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `sppc.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 67] = [
    ("SLCallServer", ExportKind::Stub),
    ("SLpAuthenticateGenuineTicketResponse", ExportKind::Stub),
    ("SLpBeginGenuineTicketTransaction", ExportKind::Stub),
    ("SLpClearActivationInProgress", ExportKind::Stub),
    ("SLpDepositDownlevelGenuineTicket", ExportKind::Stub),
    ("SLpDepositTokenActivationResponse", ExportKind::Stub),
    ("SLpGenerateTokenActivationChallenge", ExportKind::Stub),
    ("SLpGetGenuineBlob", ExportKind::Stub),
    ("SLpGetGenuineLocal", ExportKind::Stub),
    ("SLpGetLicenseAcquisitionInfo", ExportKind::Stub),
    ("SLpGetMSPidInformation", ExportKind::Stub),
    ("SLpGetMachineUGUID", ExportKind::Stub),
    ("SLpGetTokenActivationGrantInfo", ExportKind::Stub),
    ("SLpIAActivateProduct", ExportKind::Stub),
    (
        "SLpIsCurrentInstalledProductKeyDefaultKey",
        ExportKind::Stub,
    ),
    ("SLpProcessVMPipeMessage", ExportKind::Stub),
    ("SLpSetActivationInProgress", ExportKind::Stub),
    ("SLpTriggerServiceWorker", ExportKind::Stub),
    ("SLpVLActivateProduct", ExportKind::Stub),
    ("SLClose", ExportKind::Stdcall),
    ("SLConsumeRight", ExportKind::Stub),
    ("SLDepositMigrationBlob", ExportKind::Stub),
    ("SLDepositOfflineConfirmationId", ExportKind::Stub),
    ("SLDepositOfflineConfirmationIdEx", ExportKind::Stub),
    ("SLDepositStoreToken", ExportKind::Stub),
    ("SLFireEvent", ExportKind::Stub),
    ("SLGatherMigrationBlob", ExportKind::Stub),
    ("SLGatherMigrationBlobEx", ExportKind::Stub),
    ("SLGenerateOfflineInstallationId", ExportKind::Stub),
    ("SLGenerateOfflineInstallationIdEx", ExportKind::Stub),
    ("SLGetActiveLicenseInfo", ExportKind::Stub),
    ("SLGetApplicationInformation", ExportKind::Stub),
    ("SLGetApplicationPolicy", ExportKind::Stub),
    ("SLGetAuthenticationResult", ExportKind::Stub),
    ("SLGetEncryptedPIDEx", ExportKind::Stub),
    ("SLGetGenuineInformation", ExportKind::Stub),
    ("SLGetInstalledProductKeyIds", ExportKind::Stub),
    ("SLGetLicense", ExportKind::Stub),
    ("SLGetLicenseFileId", ExportKind::Stub),
    ("SLGetLicenseInformation", ExportKind::Stub),
    ("SLGetLicensingStatusInformation", ExportKind::Stdcall),
    ("SLGetPKeyId", ExportKind::Stub),
    ("SLGetPKeyInformation", ExportKind::Stub),
    ("SLGetPolicyInformation", ExportKind::Stub),
    ("SLGetPolicyInformationDWORD", ExportKind::Stub),
    ("SLGetProductSkuInformation", ExportKind::Stub),
    ("SLGetSLIDList", ExportKind::Stub),
    ("SLGetServiceInformation", ExportKind::Stub),
    ("SLInstallLicense", ExportKind::Stub),
    ("SLInstallProofOfPurchase", ExportKind::Stub),
    ("SLInstallProofOfPurchaseEx", ExportKind::Stub),
    ("SLIsGenuineLocalEx", ExportKind::Stub),
    ("SLLoadApplicationPolicies", ExportKind::Stub),
    ("SLOpen", ExportKind::Stdcall),
    ("SLPersistApplicationPolicies", ExportKind::Stdcall),
    ("SLPersistRTSPayloadOverride", ExportKind::Stub),
    ("SLReArm", ExportKind::Stub),
    ("SLRegisterEvent", ExportKind::Stub),
    ("SLRegisterPlugin", ExportKind::Stub),
    ("SLSetAuthenticationData", ExportKind::Stub),
    ("SLSetCurrentProductKey", ExportKind::Stub),
    ("SLSetGenuineInformation", ExportKind::Stub),
    ("SLUninstallLicense", ExportKind::Stub),
    ("SLUninstallProofOfPurchase", ExportKind::Stub),
    ("SLUnloadApplicationPolicies", ExportKind::Stub),
    ("SLUnregisterEvent", ExportKind::Stub),
    ("SLUnregisterPlugin", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
