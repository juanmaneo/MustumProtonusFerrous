use crate::{Hresult, E_NOTIMPL};

/// A method of `IRealTimeStylus` whose Wine body is a `FIXME` followed by `return E_NOTIMPL`.
/// The variant names are the C names without the `rtstylus_` prefix.
pub enum StubMethod {
    GetEnabled,
    PutEnabled,
    GetHwnd,
    PutHwnd,
    GetWindowInputRectangle,
    PutWindowInputRectangle,
    AddStylusSyncPlugin,
    RemoveStylusSyncPlugin,
    RemoveAllStylusSyncPlugins,
    GetStylusSyncPlugin,
    GetStylusSyncPluginCount,
    AddStylusAsyncPlugin,
    RemoveStylusAsyncPlugin,
    RemoveAllStylusAsyncPlugins,
    GetStylusAsyncPlugin,
    GetStylusAsyncPluginCount,
    GetChildRealTimeStylusPlugin,
    PutrefChildRealTimeStylusPlugin,
    AddCustomStylusDataToQueue,
    ClearStylusQueues,
    SetAllTabletsMode,
    SetSingleTabletMode,
    GetTablet,
    GetTabletContextIdFromTablet,
    GetTabletFromTabletContextId,
    GetAllTabletContextIds,
    GetStyluses,
    GetStylusForId,
    SetDesiredPacketDescription,
    GetDesiredPacketDescription,
    GetPacketDescriptionData,
}

/// Every [`StubMethod`], in the order of the C bodies in `rtscom_main.c`.
pub const STUB_METHODS: [StubMethod; 31] = [
    StubMethod::GetEnabled,
    StubMethod::PutEnabled,
    StubMethod::GetHwnd,
    StubMethod::PutHwnd,
    StubMethod::GetWindowInputRectangle,
    StubMethod::PutWindowInputRectangle,
    StubMethod::AddStylusSyncPlugin,
    StubMethod::RemoveStylusSyncPlugin,
    StubMethod::RemoveAllStylusSyncPlugins,
    StubMethod::GetStylusSyncPlugin,
    StubMethod::GetStylusSyncPluginCount,
    StubMethod::AddStylusAsyncPlugin,
    StubMethod::RemoveStylusAsyncPlugin,
    StubMethod::RemoveAllStylusAsyncPlugins,
    StubMethod::GetStylusAsyncPlugin,
    StubMethod::GetStylusAsyncPluginCount,
    StubMethod::GetChildRealTimeStylusPlugin,
    StubMethod::PutrefChildRealTimeStylusPlugin,
    StubMethod::AddCustomStylusDataToQueue,
    StubMethod::ClearStylusQueues,
    StubMethod::SetAllTabletsMode,
    StubMethod::SetSingleTabletMode,
    StubMethod::GetTablet,
    StubMethod::GetTabletContextIdFromTablet,
    StubMethod::GetTabletFromTabletContextId,
    StubMethod::GetAllTabletContextIds,
    StubMethod::GetStyluses,
    StubMethod::GetStylusForId,
    StubMethod::SetDesiredPacketDescription,
    StubMethod::GetDesiredPacketDescription,
    StubMethod::GetPacketDescriptionData,
];

/// Every stub method returns `E_NOTIMPL` and has no other effect.
pub fn stub_method(_method: StubMethod) -> Hresult {
    E_NOTIMPL
}
