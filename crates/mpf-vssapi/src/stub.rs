//! Stub exception records and helpers for every `@ stub` in `vssapi.spec`.

pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;
const VSSAPI_DLL_NAME: &str = "vssapi.dll";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        number_parameters: 2,
        module,
        function,
    }
}

pub const STUB_EXPORTS: &[&str] = &[
    "IsVolumeSnapshotted",
    "VssFreeSnapshotProperties",
    "ShouldBlockRevert",
    "??0CVssJetWriter@@QAA@XZ",
    "??0CVssJetWriter@@QAE@XZ",
    "??0CVssJetWriter@@QEAA@XZ",
    "??1CVssJetWriter@@UAA@XZ",
    "??1CVssJetWriter@@UAE@XZ",
    "??1CVssJetWriter@@UEAA@XZ",
    "?AreComponentsSelected@CVssJetWriter@@IBA_NXZ",
    "?AreComponentsSelected@CVssJetWriter@@IBG_NXZ",
    "?AreComponentsSelected@CVssJetWriter@@IEBA_NXZ",
    "?AreComponentsSelected@CVssWriter@@IBA_NXZ",
    "?AreComponentsSelected@CVssWriter@@IBG_NXZ",
    "?AreComponentsSelected@CVssWriter@@IEBA_NXZ",
    "?CreateVssExamineWriterMetadata@@YGJPAGPAPAVIVssExamineWriterMetadata@@@Z",
    "?CreateVssExamineWriterMetadata@@YAJPEAGPEAPEAVIVssExamineWriterMetadata@@@Z",
    "?CreateVssSnapshotSetDescription@@YGJU_GUID@@JPAPAVIVssSnapshotSetDescription@@@Z",
    "?GetBackupType@CVssJetWriter@@IBA?AW4_VSS_BACKUP_TYPE@@XZ",
    "?GetBackupType@CVssJetWriter@@IBG?AW4_VSS_BACKUP_TYPE@@XZ",
    "?GetBackupType@CVssJetWriter@@IEBA?AW4_VSS_BACKUP_TYPE@@XZ",
    "?GetBackupType@CVssWriter@@IBA?AW4_VSS_BACKUP_TYPE@@XZ",
    "?GetBackupType@CVssWriter@@IBG?AW4_VSS_BACKUP_TYPE@@XZ",
    "?GetBackupType@CVssWriter@@IEBA?AW4_VSS_BACKUP_TYPE@@XZ",
    "?GetContext@CVssJetWriter@@IBAJXZ",
    "?GetContext@CVssJetWriter@@IBGJXZ",
    "?GetContext@CVssJetWriter@@IEBAJXZ",
    "?GetContext@CVssWriter@@IBAJXZ",
    "?GetContext@CVssWriter@@IBGJXZ",
    "?GetContext@CVssWriter@@IEBAJXZ",
    "?GetCurrentLevel@CVssJetWriter@@IBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    "?GetCurrentLevel@CVssJetWriter@@IBG?AW4_VSS_APPLICATION_LEVEL@@XZ",
    "?GetCurrentLevel@CVssJetWriter@@IEBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    "?GetCurrentLevel@CVssWriter@@IBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    "?GetCurrentLevel@CVssWriter@@IBG?AW4_VSS_APPLICATION_LEVEL@@XZ",
    "?GetCurrentLevel@CVssWriter@@IEBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    "?GetCurrentSnapshotSetId@CVssJetWriter@@IBA?AU_GUID@@XZ",
    "?GetCurrentSnapshotSetId@CVssJetWriter@@IBG?AU_GUID@@XZ",
    "?GetCurrentSnapshotSetId@CVssJetWriter@@IEBA?AU_GUID@@XZ",
    "?GetCurrentSnapshotSetId@CVssWriter@@IBA?AU_GUID@@XZ",
    "?GetCurrentSnapshotSetId@CVssWriter@@IBG?AU_GUID@@XZ",
    "?GetCurrentSnapshotSetId@CVssWriter@@IEBA?AU_GUID@@XZ",
    "?GetCurrentVolumeArray@CVssJetWriter@@IBAPAPBGXZ",
    "?GetCurrentVolumeArray@CVssJetWriter@@IBGPAPBGXZ",
    "?GetCurrentVolumeArray@CVssJetWriter@@IEBAPEAPEBGXZ",
    "?GetCurrentVolumeArray@CVssWriter@@IBAPAPBGXZ",
    "?GetCurrentVolumeArray@CVssWriter@@IBGPAPBGXZ",
    "?GetCurrentVolumeArray@CVssWriter@@IEBAPEAPEBGXZ",
    "?GetCurrentVolumeCount@CVssJetWriter@@IBAIXZ",
    "?GetCurrentVolumeCount@CVssJetWriter@@IBGIXZ",
    "?GetCurrentVolumeCount@CVssJetWriter@@IEBAIXZ",
    "?GetCurrentVolumeCount@CVssWriter@@IBAIXZ",
    "?GetCurrentVolumeCount@CVssWriter@@IBGIXZ",
    "?GetCurrentVolumeCount@CVssWriter@@IEBAIXZ",
    "?GetRestoreType@CVssJetWriter@@IBA?AW4_VSS_RESTORE_TYPE@@XZ",
    "?GetRestoreType@CVssJetWriter@@IBG?AW4_VSS_RESTORE_TYPE@@XZ",
    "?GetRestoreType@CVssJetWriter@@IEBA?AW4_VSS_RESTORE_TYPE@@XZ",
    "?GetRestoreType@CVssWriter@@IBA?AW4_VSS_RESTORE_TYPE@@XZ",
    "?GetRestoreType@CVssWriter@@IBG?AW4_VSS_RESTORE_TYPE@@XZ",
    "?GetRestoreType@CVssWriter@@IEBA?AW4_VSS_RESTORE_TYPE@@XZ",
    "?GetSnapshotDeviceName@CVssJetWriter@@IBAJPBGPAPBG@Z",
    "?GetSnapshotDeviceName@CVssJetWriter@@IBGJPBGPAPBG@Z",
    "?GetSnapshotDeviceName@CVssJetWriter@@IEBAJPEBGPEAPEBG@Z",
    "?GetSnapshotDeviceName@CVssWriter@@IBAJPBGPAPBG@Z",
    "?GetSnapshotDeviceName@CVssWriter@@IBGJPBGPAPBG@Z",
    "?GetSnapshotDeviceName@CVssWriter@@IEBAJPEBGPEAPEBG@Z",
    "?Initialize@CVssJetWriter@@QAAJU_GUID@@PBG_N211K@Z",
    "?Initialize@CVssJetWriter@@QAGJU_GUID@@PBG_N211K@Z",
    "?Initialize@CVssJetWriter@@QEAAJU_GUID@@PEBG_N211K@Z",
    "?InstallAlternateWriter@CVssWriter@@QAAJU_GUID@@0@Z",
    "?InstallAlternateWriter@CVssWriter@@QAGJU_GUID@@0@Z",
    "?InstallAlternateWriter@CVssWriter@@QEAAJU_GUID@@0@Z",
    "?IsBootableSystemStateBackedUp@CVssJetWriter@@IBA_NXZ",
    "?IsBootableSystemStateBackedUp@CVssJetWriter@@IBG_NXZ",
    "?IsBootableSystemStateBackedUp@CVssJetWriter@@IEBA_NXZ",
    "?IsBootableSystemStateBackedUp@CVssWriter@@IBA_NXZ",
    "?IsBootableSystemStateBackedUp@CVssWriter@@IBG_NXZ",
    "?IsBootableSystemStateBackedUp@CVssWriter@@IEBA_NXZ",
    "?IsPartialFileSupportEnabled@CVssJetWriter@@IBA_NXZ",
    "?IsPartialFileSupportEnabled@CVssJetWriter@@IBG_NXZ",
    "?IsPartialFileSupportEnabled@CVssJetWriter@@IEBA_NXZ",
    "?IsPartialFileSupportEnabled@CVssWriter@@IBA_NXZ",
    "?IsPartialFileSupportEnabled@CVssWriter@@IBG_NXZ",
    "?IsPartialFileSupportEnabled@CVssWriter@@IEBA_NXZ",
    "?IsPathAffected@CVssJetWriter@@IBA_NPBG@Z",
    "?IsPathAffected@CVssJetWriter@@IBG_NPBG@Z",
    "?IsPathAffected@CVssJetWriter@@IEBA_NPEBG@Z",
    "?IsPathAffected@CVssWriter@@IBA_NPBG@Z",
    "?IsPathAffected@CVssWriter@@IBG_NPBG@Z",
    "?IsPathAffected@CVssWriter@@IEBA_NPEBG@Z",
    "?LoadVssSnapshotSetDescription@@YGJPBGPAPAVIVssSnapshotSetDescription@@U_GUID@@@Z",
    "?OnAbortBegin@CVssJetWriter@@UAAXXZ",
    "?OnAbortBegin@CVssJetWriter@@UAGXXZ",
    "?OnAbortBegin@CVssJetWriter@@UEAAXXZ",
    "?OnAbortEnd@CVssJetWriter@@UAAXXZ",
    "?OnAbortEnd@CVssJetWriter@@UAGXXZ",
    "?OnAbortEnd@CVssJetWriter@@UEAAXXZ",
    "?OnBackOffIOOnVolume@CVssWriter@@UAA_NPAGU_GUID@@1@Z",
    "?OnBackOffIOOnVolume@CVssWriter@@UAG_NPAGU_GUID@@1@Z",
    "?OnBackOffIOOnVolume@CVssWriter@@UEAA_NPEAGU_GUID@@1@Z",
    "?OnBackupComplete@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnBackupComplete@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnBackupComplete@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnBackupCompleteBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnBackupCompleteBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnBackupCompleteBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnBackupCompleteEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    "?OnBackupCompleteEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    "?OnBackupCompleteEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    "?OnBackupShutdown@CVssWriter@@UAA_NU_GUID@@@Z",
    "?OnBackupShutdown@CVssWriter@@UAG_NU_GUID@@@Z",
    "?OnBackupShutdown@CVssWriter@@UEAA_NU_GUID@@@Z",
    "?OnContinueIOOnVolume@CVssWriter@@UAA_NPAGU_GUID@@1@Z",
    "?OnContinueIOOnVolume@CVssWriter@@UAG_NPAGU_GUID@@1@Z",
    "?OnContinueIOOnVolume@CVssWriter@@UEAA_NPEAGU_GUID@@1@Z",
    "?OnFreezeBegin@CVssJetWriter@@UAA_NXZ",
    "?OnFreezeBegin@CVssJetWriter@@UAG_NXZ",
    "?OnFreezeBegin@CVssJetWriter@@UEAA_NXZ",
    "?OnFreezeEnd@CVssJetWriter@@UAA_N_N@Z",
    "?OnFreezeEnd@CVssJetWriter@@UAG_N_N@Z",
    "?OnFreezeEnd@CVssJetWriter@@UEAA_N_N@Z",
    "?OnIdentify@CVssJetWriter@@UAA_NPAVIVssCreateWriterMetadata@@@Z",
    "?OnIdentify@CVssJetWriter@@UAG_NPAVIVssCreateWriterMetadata@@@Z",
    "?OnIdentify@CVssJetWriter@@UEAA_NPEAVIVssCreateWriterMetadata@@@Z",
    "?OnIdentify@CVssWriter@@UAA_NPAVIVssCreateWriterMetadata@@@Z",
    "?OnIdentify@CVssWriter@@UAG_NPAVIVssCreateWriterMetadata@@@Z",
    "?OnIdentify@CVssWriter@@UEAA_NPEAVIVssCreateWriterMetadata@@@Z",
    "?OnPostRestore@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPostRestore@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPostRestore@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPostRestoreBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPostRestoreBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPostRestoreBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPostRestoreEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    "?OnPostRestoreEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    "?OnPostRestoreEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    "?OnPostSnapshot@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPostSnapshot@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPostSnapshot@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPostSnapshot@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPostSnapshot@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPostSnapshot@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPreRestore@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPreRestore@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPreRestore@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPreRestoreBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPreRestoreBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPreRestoreBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPreRestoreEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    "?OnPreRestoreEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    "?OnPreRestoreEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    "?OnPrepareBackup@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPrepareBackup@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPrepareBackup@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPrepareBackupBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    "?OnPrepareBackupBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    "?OnPrepareBackupBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    "?OnPrepareBackupEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    "?OnPrepareBackupEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    "?OnPrepareBackupEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    "?OnPrepareSnapshotBegin@CVssJetWriter@@UAA_NXZ",
    "?OnPrepareSnapshotBegin@CVssJetWriter@@UAG_NXZ",
    "?OnPrepareSnapshotBegin@CVssJetWriter@@UEAA_NXZ",
    "?OnPrepareSnapshotEnd@CVssJetWriter@@UAA_N_N@Z",
    "?OnPrepareSnapshotEnd@CVssJetWriter@@UAG_N_N@Z",
    "?OnPrepareSnapshotEnd@CVssJetWriter@@UEAA_N_N@Z",
    "?OnThawBegin@CVssJetWriter@@UAA_NXZ",
    "?OnThawBegin@CVssJetWriter@@UAG_NXZ",
    "?OnThawBegin@CVssJetWriter@@UEAA_NXZ",
    "?OnThawEnd@CVssJetWriter@@UAA_N_N@Z",
    "?OnThawEnd@CVssJetWriter@@UAG_N_N@Z",
    "?OnThawEnd@CVssJetWriter@@UEAA_N_N@Z",
    "?OnVSSApplicationStartup@CVssWriter@@UAA_NXZ",
    "?OnVSSApplicationStartup@CVssWriter@@UAG_NXZ",
    "?OnVSSApplicationStartup@CVssWriter@@UEAA_NXZ",
    "?OnVSSShutdown@CVssWriter@@UAA_NXZ",
    "?OnVSSShutdown@CVssWriter@@UAG_NXZ",
    "?OnVSSShutdown@CVssWriter@@UEAA_NXZ",
    "?SetWriterFailure@CVssJetWriter@@IAAJJ@Z",
    "?SetWriterFailure@CVssJetWriter@@IAGJJ@Z",
    "?SetWriterFailure@CVssJetWriter@@IEAAJJ@Z",
    "?SetWriterFailure@CVssWriter@@IAAJJ@Z",
    "?SetWriterFailure@CVssWriter@@IAGJJ@Z",
    "?SetWriterFailure@CVssWriter@@IEAAJJ@Z",
    "?Uninitialize@CVssJetWriter@@QAAXXZ",
    "?Uninitialize@CVssJetWriter@@QAGXXZ",
    "?Uninitialize@CVssJetWriter@@QEAAXXZ",
    "CreateVssExamineWriterMetadataInternal",
    "CreateVssExpressWriterInternal",
    "CreateWriter",
    "CreateWriterEx",
    "DllGetClassObject",
    "GetProviderMgmtInterface",
    "GetProviderMgmtInterfaceInternal",
    "IsVolumeSnapshottedInternal",
    "ShouldBlockRevertInternal",
    "VssFreeSnapshotPropertiesInternal",
];

pub fn stub_000() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "IsVolumeSnapshotted")
}
pub fn stub_001() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "VssFreeSnapshotProperties")
}
pub fn stub_002() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "ShouldBlockRevert")
}
pub fn stub_003() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "??0CVssJetWriter@@QAA@XZ")
}
pub fn stub_004() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "??0CVssJetWriter@@QAE@XZ")
}
pub fn stub_005() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "??0CVssJetWriter@@QEAA@XZ")
}
pub fn stub_006() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "??1CVssJetWriter@@UAA@XZ")
}
pub fn stub_007() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "??1CVssJetWriter@@UAE@XZ")
}
pub fn stub_008() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "??1CVssJetWriter@@UEAA@XZ")
}
pub fn stub_009() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?AreComponentsSelected@CVssJetWriter@@IBA_NXZ",
    )
}
pub fn stub_010() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?AreComponentsSelected@CVssJetWriter@@IBG_NXZ",
    )
}
pub fn stub_011() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?AreComponentsSelected@CVssJetWriter@@IEBA_NXZ",
    )
}
pub fn stub_012() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?AreComponentsSelected@CVssWriter@@IBA_NXZ",
    )
}
pub fn stub_013() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?AreComponentsSelected@CVssWriter@@IBG_NXZ",
    )
}
pub fn stub_014() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?AreComponentsSelected@CVssWriter@@IEBA_NXZ",
    )
}
pub fn stub_015() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?CreateVssExamineWriterMetadata@@YGJPAGPAPAVIVssExamineWriterMetadata@@@Z",
    )
}
pub fn stub_016() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?CreateVssExamineWriterMetadata@@YAJPEAGPEAPEAVIVssExamineWriterMetadata@@@Z",
    )
}
pub fn stub_017() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?CreateVssSnapshotSetDescription@@YGJU_GUID@@JPAPAVIVssSnapshotSetDescription@@@Z",
    )
}
pub fn stub_018() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetBackupType@CVssJetWriter@@IBA?AW4_VSS_BACKUP_TYPE@@XZ",
    )
}
pub fn stub_019() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetBackupType@CVssJetWriter@@IBG?AW4_VSS_BACKUP_TYPE@@XZ",
    )
}
pub fn stub_020() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetBackupType@CVssJetWriter@@IEBA?AW4_VSS_BACKUP_TYPE@@XZ",
    )
}
pub fn stub_021() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetBackupType@CVssWriter@@IBA?AW4_VSS_BACKUP_TYPE@@XZ",
    )
}
pub fn stub_022() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetBackupType@CVssWriter@@IBG?AW4_VSS_BACKUP_TYPE@@XZ",
    )
}
pub fn stub_023() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetBackupType@CVssWriter@@IEBA?AW4_VSS_BACKUP_TYPE@@XZ",
    )
}
pub fn stub_024() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetContext@CVssJetWriter@@IBAJXZ")
}
pub fn stub_025() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetContext@CVssJetWriter@@IBGJXZ")
}
pub fn stub_026() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetContext@CVssJetWriter@@IEBAJXZ")
}
pub fn stub_027() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetContext@CVssWriter@@IBAJXZ")
}
pub fn stub_028() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetContext@CVssWriter@@IBGJXZ")
}
pub fn stub_029() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetContext@CVssWriter@@IEBAJXZ")
}
pub fn stub_030() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentLevel@CVssJetWriter@@IBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    )
}
pub fn stub_031() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentLevel@CVssJetWriter@@IBG?AW4_VSS_APPLICATION_LEVEL@@XZ",
    )
}
pub fn stub_032() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentLevel@CVssJetWriter@@IEBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    )
}
pub fn stub_033() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentLevel@CVssWriter@@IBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    )
}
pub fn stub_034() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentLevel@CVssWriter@@IBG?AW4_VSS_APPLICATION_LEVEL@@XZ",
    )
}
pub fn stub_035() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentLevel@CVssWriter@@IEBA?AW4_VSS_APPLICATION_LEVEL@@XZ",
    )
}
pub fn stub_036() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentSnapshotSetId@CVssJetWriter@@IBA?AU_GUID@@XZ",
    )
}
pub fn stub_037() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentSnapshotSetId@CVssJetWriter@@IBG?AU_GUID@@XZ",
    )
}
pub fn stub_038() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentSnapshotSetId@CVssJetWriter@@IEBA?AU_GUID@@XZ",
    )
}
pub fn stub_039() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentSnapshotSetId@CVssWriter@@IBA?AU_GUID@@XZ",
    )
}
pub fn stub_040() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentSnapshotSetId@CVssWriter@@IBG?AU_GUID@@XZ",
    )
}
pub fn stub_041() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentSnapshotSetId@CVssWriter@@IEBA?AU_GUID@@XZ",
    )
}
pub fn stub_042() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeArray@CVssJetWriter@@IBAPAPBGXZ",
    )
}
pub fn stub_043() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeArray@CVssJetWriter@@IBGPAPBGXZ",
    )
}
pub fn stub_044() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeArray@CVssJetWriter@@IEBAPEAPEBGXZ",
    )
}
pub fn stub_045() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeArray@CVssWriter@@IBAPAPBGXZ",
    )
}
pub fn stub_046() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeArray@CVssWriter@@IBGPAPBGXZ",
    )
}
pub fn stub_047() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeArray@CVssWriter@@IEBAPEAPEBGXZ",
    )
}
pub fn stub_048() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeCount@CVssJetWriter@@IBAIXZ",
    )
}
pub fn stub_049() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeCount@CVssJetWriter@@IBGIXZ",
    )
}
pub fn stub_050() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeCount@CVssJetWriter@@IEBAIXZ",
    )
}
pub fn stub_051() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetCurrentVolumeCount@CVssWriter@@IBAIXZ")
}
pub fn stub_052() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?GetCurrentVolumeCount@CVssWriter@@IBGIXZ")
}
pub fn stub_053() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetCurrentVolumeCount@CVssWriter@@IEBAIXZ",
    )
}
pub fn stub_054() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetRestoreType@CVssJetWriter@@IBA?AW4_VSS_RESTORE_TYPE@@XZ",
    )
}
pub fn stub_055() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetRestoreType@CVssJetWriter@@IBG?AW4_VSS_RESTORE_TYPE@@XZ",
    )
}
pub fn stub_056() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetRestoreType@CVssJetWriter@@IEBA?AW4_VSS_RESTORE_TYPE@@XZ",
    )
}
pub fn stub_057() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetRestoreType@CVssWriter@@IBA?AW4_VSS_RESTORE_TYPE@@XZ",
    )
}
pub fn stub_058() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetRestoreType@CVssWriter@@IBG?AW4_VSS_RESTORE_TYPE@@XZ",
    )
}
pub fn stub_059() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetRestoreType@CVssWriter@@IEBA?AW4_VSS_RESTORE_TYPE@@XZ",
    )
}
pub fn stub_060() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetSnapshotDeviceName@CVssJetWriter@@IBAJPBGPAPBG@Z",
    )
}
pub fn stub_061() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetSnapshotDeviceName@CVssJetWriter@@IBGJPBGPAPBG@Z",
    )
}
pub fn stub_062() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetSnapshotDeviceName@CVssJetWriter@@IEBAJPEBGPEAPEBG@Z",
    )
}
pub fn stub_063() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetSnapshotDeviceName@CVssWriter@@IBAJPBGPAPBG@Z",
    )
}
pub fn stub_064() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetSnapshotDeviceName@CVssWriter@@IBGJPBGPAPBG@Z",
    )
}
pub fn stub_065() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?GetSnapshotDeviceName@CVssWriter@@IEBAJPEBGPEAPEBG@Z",
    )
}
pub fn stub_066() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?Initialize@CVssJetWriter@@QAAJU_GUID@@PBG_N211K@Z",
    )
}
pub fn stub_067() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?Initialize@CVssJetWriter@@QAGJU_GUID@@PBG_N211K@Z",
    )
}
pub fn stub_068() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?Initialize@CVssJetWriter@@QEAAJU_GUID@@PEBG_N211K@Z",
    )
}
pub fn stub_069() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?InstallAlternateWriter@CVssWriter@@QAAJU_GUID@@0@Z",
    )
}
pub fn stub_070() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?InstallAlternateWriter@CVssWriter@@QAGJU_GUID@@0@Z",
    )
}
pub fn stub_071() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?InstallAlternateWriter@CVssWriter@@QEAAJU_GUID@@0@Z",
    )
}
pub fn stub_072() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsBootableSystemStateBackedUp@CVssJetWriter@@IBA_NXZ",
    )
}
pub fn stub_073() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsBootableSystemStateBackedUp@CVssJetWriter@@IBG_NXZ",
    )
}
pub fn stub_074() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsBootableSystemStateBackedUp@CVssJetWriter@@IEBA_NXZ",
    )
}
pub fn stub_075() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsBootableSystemStateBackedUp@CVssWriter@@IBA_NXZ",
    )
}
pub fn stub_076() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsBootableSystemStateBackedUp@CVssWriter@@IBG_NXZ",
    )
}
pub fn stub_077() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsBootableSystemStateBackedUp@CVssWriter@@IEBA_NXZ",
    )
}
pub fn stub_078() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPartialFileSupportEnabled@CVssJetWriter@@IBA_NXZ",
    )
}
pub fn stub_079() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPartialFileSupportEnabled@CVssJetWriter@@IBG_NXZ",
    )
}
pub fn stub_080() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPartialFileSupportEnabled@CVssJetWriter@@IEBA_NXZ",
    )
}
pub fn stub_081() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPartialFileSupportEnabled@CVssWriter@@IBA_NXZ",
    )
}
pub fn stub_082() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPartialFileSupportEnabled@CVssWriter@@IBG_NXZ",
    )
}
pub fn stub_083() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPartialFileSupportEnabled@CVssWriter@@IEBA_NXZ",
    )
}
pub fn stub_084() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?IsPathAffected@CVssJetWriter@@IBA_NPBG@Z")
}
pub fn stub_085() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?IsPathAffected@CVssJetWriter@@IBG_NPBG@Z")
}
pub fn stub_086() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?IsPathAffected@CVssJetWriter@@IEBA_NPEBG@Z",
    )
}
pub fn stub_087() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?IsPathAffected@CVssWriter@@IBA_NPBG@Z")
}
pub fn stub_088() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?IsPathAffected@CVssWriter@@IBG_NPBG@Z")
}
pub fn stub_089() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?IsPathAffected@CVssWriter@@IEBA_NPEBG@Z")
}
pub fn stub_090() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?LoadVssSnapshotSetDescription@@YGJPBGPAPAVIVssSnapshotSetDescription@@U_GUID@@@Z",
    )
}
pub fn stub_091() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnAbortBegin@CVssJetWriter@@UAAXXZ")
}
pub fn stub_092() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnAbortBegin@CVssJetWriter@@UAGXXZ")
}
pub fn stub_093() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnAbortBegin@CVssJetWriter@@UEAAXXZ")
}
pub fn stub_094() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnAbortEnd@CVssJetWriter@@UAAXXZ")
}
pub fn stub_095() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnAbortEnd@CVssJetWriter@@UAGXXZ")
}
pub fn stub_096() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnAbortEnd@CVssJetWriter@@UEAAXXZ")
}
pub fn stub_097() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackOffIOOnVolume@CVssWriter@@UAA_NPAGU_GUID@@1@Z",
    )
}
pub fn stub_098() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackOffIOOnVolume@CVssWriter@@UAG_NPAGU_GUID@@1@Z",
    )
}
pub fn stub_099() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackOffIOOnVolume@CVssWriter@@UEAA_NPEAGU_GUID@@1@Z",
    )
}
pub fn stub_100() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupComplete@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_101() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupComplete@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_102() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupComplete@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_103() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupCompleteBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_104() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupCompleteBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_105() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupCompleteBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_106() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupCompleteEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_107() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupCompleteEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_108() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupCompleteEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_109() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupShutdown@CVssWriter@@UAA_NU_GUID@@@Z",
    )
}
pub fn stub_110() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupShutdown@CVssWriter@@UAG_NU_GUID@@@Z",
    )
}
pub fn stub_111() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnBackupShutdown@CVssWriter@@UEAA_NU_GUID@@@Z",
    )
}
pub fn stub_112() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnContinueIOOnVolume@CVssWriter@@UAA_NPAGU_GUID@@1@Z",
    )
}
pub fn stub_113() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnContinueIOOnVolume@CVssWriter@@UAG_NPAGU_GUID@@1@Z",
    )
}
pub fn stub_114() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnContinueIOOnVolume@CVssWriter@@UEAA_NPEAGU_GUID@@1@Z",
    )
}
pub fn stub_115() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnFreezeBegin@CVssJetWriter@@UAA_NXZ")
}
pub fn stub_116() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnFreezeBegin@CVssJetWriter@@UAG_NXZ")
}
pub fn stub_117() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnFreezeBegin@CVssJetWriter@@UEAA_NXZ")
}
pub fn stub_118() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnFreezeEnd@CVssJetWriter@@UAA_N_N@Z")
}
pub fn stub_119() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnFreezeEnd@CVssJetWriter@@UAG_N_N@Z")
}
pub fn stub_120() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnFreezeEnd@CVssJetWriter@@UEAA_N_N@Z")
}
pub fn stub_121() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnIdentify@CVssJetWriter@@UAA_NPAVIVssCreateWriterMetadata@@@Z",
    )
}
pub fn stub_122() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnIdentify@CVssJetWriter@@UAG_NPAVIVssCreateWriterMetadata@@@Z",
    )
}
pub fn stub_123() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnIdentify@CVssJetWriter@@UEAA_NPEAVIVssCreateWriterMetadata@@@Z",
    )
}
pub fn stub_124() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnIdentify@CVssWriter@@UAA_NPAVIVssCreateWriterMetadata@@@Z",
    )
}
pub fn stub_125() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnIdentify@CVssWriter@@UAG_NPAVIVssCreateWriterMetadata@@@Z",
    )
}
pub fn stub_126() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnIdentify@CVssWriter@@UEAA_NPEAVIVssCreateWriterMetadata@@@Z",
    )
}
pub fn stub_127() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestore@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_128() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestore@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_129() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestore@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_130() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestoreBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_131() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestoreBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_132() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestoreBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_133() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestoreEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_134() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestoreEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_135() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostRestoreEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_136() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostSnapshot@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_137() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostSnapshot@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_138() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostSnapshot@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_139() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostSnapshot@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_140() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostSnapshot@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_141() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPostSnapshot@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_142() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestore@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_143() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestore@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_144() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestore@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_145() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestoreBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_146() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestoreBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_147() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestoreBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_148() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestoreEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_149() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestoreEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_150() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPreRestoreEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_151() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackup@CVssWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_152() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackup@CVssWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_153() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackup@CVssWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_154() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackupBegin@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_155() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackupBegin@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_156() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackupBegin@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@@Z",
    )
}
pub fn stub_157() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackupEnd@CVssJetWriter@@UAA_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_158() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackupEnd@CVssJetWriter@@UAG_NPAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_159() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareBackupEnd@CVssJetWriter@@UEAA_NPEAVIVssWriterComponents@@_N@Z",
    )
}
pub fn stub_160() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareSnapshotBegin@CVssJetWriter@@UAA_NXZ",
    )
}
pub fn stub_161() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareSnapshotBegin@CVssJetWriter@@UAG_NXZ",
    )
}
pub fn stub_162() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareSnapshotBegin@CVssJetWriter@@UEAA_NXZ",
    )
}
pub fn stub_163() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareSnapshotEnd@CVssJetWriter@@UAA_N_N@Z",
    )
}
pub fn stub_164() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareSnapshotEnd@CVssJetWriter@@UAG_N_N@Z",
    )
}
pub fn stub_165() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnPrepareSnapshotEnd@CVssJetWriter@@UEAA_N_N@Z",
    )
}
pub fn stub_166() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnThawBegin@CVssJetWriter@@UAA_NXZ")
}
pub fn stub_167() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnThawBegin@CVssJetWriter@@UAG_NXZ")
}
pub fn stub_168() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnThawBegin@CVssJetWriter@@UEAA_NXZ")
}
pub fn stub_169() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnThawEnd@CVssJetWriter@@UAA_N_N@Z")
}
pub fn stub_170() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnThawEnd@CVssJetWriter@@UAG_N_N@Z")
}
pub fn stub_171() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnThawEnd@CVssJetWriter@@UEAA_N_N@Z")
}
pub fn stub_172() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnVSSApplicationStartup@CVssWriter@@UAA_NXZ",
    )
}
pub fn stub_173() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnVSSApplicationStartup@CVssWriter@@UAG_NXZ",
    )
}
pub fn stub_174() -> StubException {
    wine_spec_unimplemented_stub(
        VSSAPI_DLL_NAME,
        "?OnVSSApplicationStartup@CVssWriter@@UEAA_NXZ",
    )
}
pub fn stub_175() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnVSSShutdown@CVssWriter@@UAA_NXZ")
}
pub fn stub_176() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnVSSShutdown@CVssWriter@@UAG_NXZ")
}
pub fn stub_177() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?OnVSSShutdown@CVssWriter@@UEAA_NXZ")
}
pub fn stub_178() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?SetWriterFailure@CVssJetWriter@@IAAJJ@Z")
}
pub fn stub_179() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?SetWriterFailure@CVssJetWriter@@IAGJJ@Z")
}
pub fn stub_180() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?SetWriterFailure@CVssJetWriter@@IEAAJJ@Z")
}
pub fn stub_181() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?SetWriterFailure@CVssWriter@@IAAJJ@Z")
}
pub fn stub_182() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?SetWriterFailure@CVssWriter@@IAGJJ@Z")
}
pub fn stub_183() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?SetWriterFailure@CVssWriter@@IEAAJJ@Z")
}
pub fn stub_184() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?Uninitialize@CVssJetWriter@@QAAXXZ")
}
pub fn stub_185() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?Uninitialize@CVssJetWriter@@QAGXXZ")
}
pub fn stub_186() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "?Uninitialize@CVssJetWriter@@QEAAXXZ")
}
pub fn stub_187() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "CreateVssExamineWriterMetadataInternal")
}
pub fn stub_188() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "CreateVssExpressWriterInternal")
}
pub fn stub_189() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "CreateWriter")
}
pub fn stub_190() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "CreateWriterEx")
}
pub fn stub_191() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "DllGetClassObject")
}
pub fn stub_192() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "GetProviderMgmtInterface")
}
pub fn stub_193() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "GetProviderMgmtInterfaceInternal")
}
pub fn stub_194() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "IsVolumeSnapshottedInternal")
}
pub fn stub_195() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "ShouldBlockRevertInternal")
}
pub fn stub_196() -> StubException {
    wine_spec_unimplemented_stub(VSSAPI_DLL_NAME, "VssFreeSnapshotPropertiesInternal")
}

pub const STUB_HELPERS: &[fn() -> StubException] = &[
    stub_000, stub_001, stub_002, stub_003, stub_004, stub_005, stub_006, stub_007, stub_008,
    stub_009, stub_010, stub_011, stub_012, stub_013, stub_014, stub_015, stub_016, stub_017,
    stub_018, stub_019, stub_020, stub_021, stub_022, stub_023, stub_024, stub_025, stub_026,
    stub_027, stub_028, stub_029, stub_030, stub_031, stub_032, stub_033, stub_034, stub_035,
    stub_036, stub_037, stub_038, stub_039, stub_040, stub_041, stub_042, stub_043, stub_044,
    stub_045, stub_046, stub_047, stub_048, stub_049, stub_050, stub_051, stub_052, stub_053,
    stub_054, stub_055, stub_056, stub_057, stub_058, stub_059, stub_060, stub_061, stub_062,
    stub_063, stub_064, stub_065, stub_066, stub_067, stub_068, stub_069, stub_070, stub_071,
    stub_072, stub_073, stub_074, stub_075, stub_076, stub_077, stub_078, stub_079, stub_080,
    stub_081, stub_082, stub_083, stub_084, stub_085, stub_086, stub_087, stub_088, stub_089,
    stub_090, stub_091, stub_092, stub_093, stub_094, stub_095, stub_096, stub_097, stub_098,
    stub_099, stub_100, stub_101, stub_102, stub_103, stub_104, stub_105, stub_106, stub_107,
    stub_108, stub_109, stub_110, stub_111, stub_112, stub_113, stub_114, stub_115, stub_116,
    stub_117, stub_118, stub_119, stub_120, stub_121, stub_122, stub_123, stub_124, stub_125,
    stub_126, stub_127, stub_128, stub_129, stub_130, stub_131, stub_132, stub_133, stub_134,
    stub_135, stub_136, stub_137, stub_138, stub_139, stub_140, stub_141, stub_142, stub_143,
    stub_144, stub_145, stub_146, stub_147, stub_148, stub_149, stub_150, stub_151, stub_152,
    stub_153, stub_154, stub_155, stub_156, stub_157, stub_158, stub_159, stub_160, stub_161,
    stub_162, stub_163, stub_164, stub_165, stub_166, stub_167, stub_168, stub_169, stub_170,
    stub_171, stub_172, stub_173, stub_174, stub_175, stub_176, stub_177, stub_178, stub_179,
    stub_180, stub_181, stub_182, stub_183, stub_184, stub_185, stub_186, stub_187, stub_188,
    stub_189, stub_190, stub_191, stub_192, stub_193, stub_194, stub_195, stub_196,
];
