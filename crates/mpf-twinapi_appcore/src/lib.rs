//! Headless port of Wine's `dlls/twinapi.appcore`, the WinRT activation and app-state shim.
//!
//! Ported:
//! - `analytics_version_info_get_DeviceFamilyVersion` version packing ([`analytics_info::device_family_version`]).
//! - The class-name dispatch of `DllGetActivationFactory` ([`activation_factory_for_class`]).
//!
//! Skipped (each is a FIXME stub, or needs COM objects, HSTRINGs, the registry, the PEB or a window):
//! - The `QueryInterface`, `AddRef`, `Release`, `GetIids` and `GetRuntimeClassName` plumbing of every factory and statics object.
//! - `DeviceFamily` and the `WindowsCreateString` results, which need HSTRINGs.
//! - The `UBR` registry read in `DeviceFamilyVersion`, and the PEB reads.
//! - `RegisterAppStateChangeNotification`, `RegisterAppConstrainedChangeNotification` and the matching unregister calls: FIXME stubs.
//! - `DllGetClassObject`: FIXME stub.
//! - `GetForCurrentView`, `TryUnsnap`, `ShowShareUI` and the other ApplicationView and DataTransfer methods: FIXME stubs.
//! - `EasClientDeviceInformation` getters, `AnalyticsInfo::DeviceForm`, and the `AdvertisingId` getter: FIXME stubs or handle-bound state.
//!
//! Divergences from Wine:
//! - [`activation_factory_for_class`] returns the factory kind, not an `IActivationFactory` pointer. Wine's `QueryInterface` on those factories always succeeds for `IActivationFactory`, so the kind is the whole decision.

pub mod analytics_info;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The activation factories that `DllGetActivationFactory` hands out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationFactory {
    EasClientDeviceInformation,
    AnalyticsInfo,
    AdvertisingManager,
    ApplicationView,
    CoreApplication,
    DataTransferManager,
}

const CLASSES: [(&str, ActivationFactory); 6] = [
    (
        "Windows.Security.ExchangeActiveSyncProvisioning.EasClientDeviceInformation",
        ActivationFactory::EasClientDeviceInformation,
    ),
    (
        "Windows.System.Profile.AnalyticsInfo",
        ActivationFactory::AnalyticsInfo,
    ),
    (
        "Windows.System.UserProfile.AdvertisingManager",
        ActivationFactory::AdvertisingManager,
    ),
    (
        "Windows.UI.ViewManagement.ApplicationView",
        ActivationFactory::ApplicationView,
    ),
    (
        "Windows.ApplicationModel.Core.CoreApplication",
        ActivationFactory::CoreApplication,
    ),
    (
        "Windows.ApplicationModel.DataTransfer.DataTransferManager",
        ActivationFactory::DataTransferManager,
    ),
];

/// Returns the factory for a runtime class name, as `DllGetActivationFactory` does.
///
/// The name is compared as UTF-16 code units up to the first NUL, the way `wcscmp` does, so text after an embedded NUL is ignored. Any other name has no factory, which Wine reports as `CLASS_E_CLASSNOTAVAILABLE`.
pub fn activation_factory_for_class(class_name: &[u16]) -> Option<ActivationFactory> {
    let end = class_name
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(class_name.len());
    let name = &class_name[..end];
    CLASSES
        .iter()
        .find(|(class, _)| class.encode_utf16().eq(name.iter().copied()))
        .map(|&(_, factory)| factory)
}
