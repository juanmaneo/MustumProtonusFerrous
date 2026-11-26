/// How `dlls/tbs/tbs.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function, ported as [`crate::tbsi_context_create`],
    /// [`crate::tbsi_get_device_info`] or [`crate::get_device_id_string`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `tbs.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 21] = [
    ("Tbsi_Create_Attestation_From_Log", ExportKind::Stub),
    ("Tbsi_Get_TCG_Logs", ExportKind::Stub),
    ("GetDeviceID", ExportKind::Stub),
    ("GetDeviceIDString", ExportKind::Stdcall),
    ("GetDeviceIDWithTimeout", ExportKind::Stub),
    ("Tbsi_Context_Create", ExportKind::Stdcall),
    ("Tbsi_Create_Windows_Key", ExportKind::Stub),
    ("Tbsi_FilterLog", ExportKind::Stub),
    ("Tbsi_GetDeviceInfo", ExportKind::Stdcall),
    ("Tbsi_Get_OwnerAuth", ExportKind::Stub),
    ("Tbsi_Get_TCG_Log", ExportKind::Stub),
    ("Tbsi_Get_TCG_Log_Ex", ExportKind::Stub),
    ("Tbsi_Physical_Presence_Command", ExportKind::Stub),
    ("Tbsi_Revoke_Attestation", ExportKind::Stub),
    ("Tbsi_ShaHash", ExportKind::Stub),
    ("Tbsip_Cancel_Commands", ExportKind::Stub),
    ("Tbsip_Context_Close", ExportKind::Stub),
    ("Tbsip_Submit_Command", ExportKind::Stub),
    ("Tbsip_Submit_Command_NonBlocking", ExportKind::Stub),
    ("Tbsip_TestInterruptInformation", ExportKind::Stub),
    ("Tbsip_TestMorBit", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
