use crate::identity::Identity;

/// One entry of the DSM device list (`struct all_devices` in `dsm_ctrl.c`).
pub struct Device {
    pub modname: String,
    pub identity: Identity,
}

/// Index of the device `OpenDS` loads: the device whose `ProductName` matches exactly, or the
/// first device when the name is empty or unknown. The caller checks that the list is not empty.
pub fn select_device(devices: &[Device], product_name: &str) -> usize {
    if product_name.is_empty() {
        return 0;
    }
    devices
        .iter()
        .position(|device| device.identity.product_name == product_name)
        .unwrap_or(0)
}

/// Devices whose `SupportedGroups` share a bit with the application's groups, in list order.
/// These are the entries `TWAIN_UserSelect` offers.
pub fn selectable_identities(devices: &[Device], origin_groups: u32) -> Vec<Identity> {
    devices
        .iter()
        .filter(|device| (device.identity.supported_groups & origin_groups) != 0)
        .map(|device| device.identity.clone())
        .collect()
}
