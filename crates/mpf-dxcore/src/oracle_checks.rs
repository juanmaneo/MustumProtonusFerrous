//! Results from the sanitizer-clean C oracle built from the Wine property code.

use crate::property::*;
use crate::{
    get_property, get_property_size, AdapterProperties, DXGI_ERROR_INVALID_CALL, E_INVALIDARG,
    E_POINTER, S_OK,
};

const ADAPTER: AdapterProperties<'static> = AdapterProperties {
    instance_luid: [0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55],
    driver_version: [8, 7, 6, 5, 4, 3, 2, 1],
    hardware_id: [0x1234, 0x5678, 0x9abc, 0xdef0],
    dedicated_adapter_memory: 0x0102_0304_0506_0708,
    shared_system_memory: 0x8877_6655_4433_2211,
    driver_description: b"GPU X\0",
};

struct Vector {
    name: &'static str,
    property: u32,
    capacity: Option<usize>,
    expected: &'static str,
}

const VECTORS: [Vector; 20] = [
    Vector {
        name: "luid",
        property: INSTANCE_LUID,
        capacity: Some(8),
        expected: "luid|hr=00000000|size=8|data=4433221188776655",
    },
    Vector {
        name: "version",
        property: DRIVER_VERSION,
        capacity: Some(8),
        expected: "version|hr=00000000|size=8|data=0807060504030201",
    },
    Vector {
        name: "description",
        property: DRIVER_DESCRIPTION,
        capacity: Some(16),
        expected: "description|hr=00000000|size=6|data=475055205800aaaaaaaaaaaaaaaaaaaa",
    },
    Vector {
        name: "hardware_id",
        property: HARDWARE_ID,
        capacity: Some(16),
        expected: "hardware_id|hr=00000000|size=16|data=3412000078560000bc9a0000f0de0000",
    },
    Vector {
        name: "dedicated_memory",
        property: DEDICATED_ADAPTER_MEMORY,
        capacity: Some(8),
        expected: "dedicated_memory|hr=00000000|size=8|data=0807060504030201",
    },
    Vector {
        name: "dedicated_system_memory",
        property: DEDICATED_SYSTEM_MEMORY,
        capacity: Some(8),
        expected: "dedicated_system_memory|hr=00000000|size=8|data=0000000000000000",
    },
    Vector {
        name: "shared_memory",
        property: SHARED_SYSTEM_MEMORY,
        capacity: Some(8),
        expected: "shared_memory|hr=00000000|size=8|data=1122334455667788",
    },
    Vector {
        name: "is_hardware",
        property: IS_HARDWARE,
        capacity: Some(4),
        expected: "is_hardware|hr=00000000|size=1|data=01aaaaaa",
    },
    Vector {
        name: "luid_oversized",
        property: INSTANCE_LUID,
        capacity: Some(10),
        expected: "luid_oversized|hr=00000000|size=8|data=4433221188776655aaaa",
    },
    Vector {
        name: "luid_short",
        property: INSTANCE_LUID,
        capacity: Some(7),
        expected: "luid_short|hr=80070057|size=8|data=aaaaaaaaaaaaaa",
    },
    Vector {
        name: "version_short",
        property: DRIVER_VERSION,
        capacity: Some(7),
        expected: "version_short|hr=80070057|size=8|data=aaaaaaaaaaaaaa",
    },
    Vector {
        name: "description_short",
        property: DRIVER_DESCRIPTION,
        capacity: Some(5),
        expected: "description_short|hr=80070057|size=6|data=00aaaaaaaa",
    },
    Vector {
        name: "description_zero",
        property: DRIVER_DESCRIPTION,
        capacity: Some(0),
        expected: "description_zero|hr=80070057|size=6|data=",
    },
    Vector {
        name: "hardware_id_short",
        property: HARDWARE_ID,
        capacity: Some(15),
        expected: "hardware_id_short|hr=80070057|size=16|data=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    },
    Vector {
        name: "dedicated_memory_short",
        property: DEDICATED_ADAPTER_MEMORY,
        capacity: Some(7),
        expected: "dedicated_memory_short|hr=80070057|size=8|data=aaaaaaaaaaaaaa",
    },
    Vector {
        name: "dedicated_system_memory_short",
        property: DEDICATED_SYSTEM_MEMORY,
        capacity: Some(7),
        expected: "dedicated_system_memory_short|hr=80070057|size=8|data=aaaaaaaaaaaaaa",
    },
    Vector {
        name: "shared_memory_short",
        property: SHARED_SYSTEM_MEMORY,
        capacity: Some(7),
        expected: "shared_memory_short|hr=80070057|size=8|data=aaaaaaaaaaaaaa",
    },
    Vector {
        name: "is_hardware_zero",
        property: IS_HARDWARE,
        capacity: Some(0),
        expected: "is_hardware_zero|hr=80070057|size=1|data=",
    },
    Vector {
        name: "unknown",
        property: u32::MAX,
        capacity: Some(8),
        expected: "unknown|hr=887a0001|size=0|data=aaaaaaaaaaaaaaaa",
    },
    Vector {
        name: "unknown_null",
        property: u32::MAX,
        capacity: None,
        expected: "unknown_null|hr=80004003|size=0|data=-",
    },
];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn run(vector: &Vector) -> String {
    let size = get_property_size(vector.property, ADAPTER.driver_description).unwrap_or(0);
    let mut buffer = [0xaa; 16];
    let capacity = vector.capacity.unwrap_or(0);
    let result = match vector.capacity {
        Some(_) => get_property(&ADAPTER, vector.property, Some(&mut buffer[..capacity])),
        None => get_property(&ADAPTER, vector.property, None),
    };
    let data = match vector.capacity {
        Some(_) => hex(&buffer[..capacity]),
        None => "-".to_owned(),
    };
    format!(
        "{}|hr={:08x}|size={size}|data={data}",
        vector.name, result as u32
    )
}

#[test]
fn wine_c_oracle_rows_match() {
    for vector in &VECTORS {
        assert_eq!(run(vector), vector.expected, "{}", vector.name);
    }
}

#[test]
fn oracle_rows_include_all_success_and_error_classes() {
    assert_eq!(VECTORS.len(), 20);
    assert_eq!(S_OK, 0);
    assert_eq!(E_INVALIDARG as u32, 0x8007_0057);
    assert_eq!(E_POINTER as u32, 0x8000_4003);
    assert_eq!(DXGI_ERROR_INVALID_CALL as u32, 0x887a_0001);
}
