use super::*;
use std::mem::size_of;

#[test]
fn struct_size_matches_c_sizeof() {
    assert_eq!(size_of::<RasDevInfoA>(), RAS_DEVINFO_A_SIZE as usize);
}

#[test]
fn two_call_query_then_fill() {
    let mut cb = 0;
    let mut devices = 0;
    assert_eq!(
        ras_enum_devices_a(None, Some(&mut cb), Some(&mut devices)),
        ERROR_BUFFER_TOO_SMALL
    );
    assert_eq!(cb, RAS_DEVINFO_A_SIZE);
    assert_eq!(devices, 1);

    let mut info = RasDevInfoA {
        dw_size: RAS_DEVINFO_A_SIZE,
        device_type: [0; RAS_MAX_DEVICE_TYPE + 1],
        device_name: [0; RAS_MAX_DEVICE_NAME + 1],
    };
    assert_eq!(
        ras_enum_devices_a(Some(&mut info), Some(&mut cb), Some(&mut devices)),
        0
    );
    assert_eq!(&info.device_type[..6], b"modem\0");
    assert_eq!(&info.device_name[..15], b"WINE virtmodem\0");
}
