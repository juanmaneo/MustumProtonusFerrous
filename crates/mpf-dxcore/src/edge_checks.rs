//! Safe-slice and unimplemented-property edge behavior.

use crate::property::*;
use crate::{
    get_property, get_property_size, AdapterProperties, DXGI_ERROR_INVALID_CALL, E_POINTER,
};

fn adapter(description: &[u8]) -> AdapterProperties<'_> {
    AdapterProperties {
        instance_luid: [0; 8],
        driver_version: [0; 8],
        hardware_id: [0; 4],
        dedicated_adapter_memory: 0,
        shared_system_memory: 0,
        driver_description: description,
    }
}

#[test]
fn description_ends_at_first_nul_or_slice_end() {
    assert_eq!(
        get_property_size(DRIVER_DESCRIPTION, b"abc\0ignored"),
        Ok(4)
    );
    assert_eq!(get_property_size(DRIVER_DESCRIPTION, b"abc"), Ok(4));
    assert_eq!(get_property_size(DRIVER_DESCRIPTION, b""), Ok(1));

    let mut embedded = [0xaa; 8];
    assert_eq!(
        get_property(
            &adapter(b"abc\0ignored"),
            DRIVER_DESCRIPTION,
            Some(&mut embedded)
        ),
        0
    );
    assert_eq!(&embedded[..4], b"abc\0");
    assert_eq!(&embedded[4..], &[0xaa; 4]);

    let mut unterminated = [0xaa; 4];
    assert_eq!(
        get_property(
            &adapter(b"abc"),
            DRIVER_DESCRIPTION,
            Some(&mut unterminated)
        ),
        0
    );
    assert_eq!(&unterminated, b"abc\0");
}

#[test]
fn pointer_check_precedes_invalid_property_check() {
    assert_eq!(get_property(&adapter(b""), u32::MAX, None), E_POINTER);
    assert_eq!(get_property_size(4, b""), Err(DXGI_ERROR_INVALID_CALL));
    assert_eq!(get_property_size(14, b""), Err(DXGI_ERROR_INVALID_CALL));
}
