//! Oracle-derived test vectors for the ported mouhid.sys logic.
//!
//! Vectors were printed by compiling the verbatim Wine C bodies of `sign_extend`,
//! `scale_value`, `MulDiv`, `add_contact`, `release_contacts` and `process_hid_report`
//! (with the `struct contact` and `struct device` definitions) using
//! `gcc -fsanitize=address,undefined -g`. The harness is under `target/canon/mouhid.sys/`.
//! It printed 23 rows: 11 scale rows, 5 `MulDiv` rows and 7 contact scenarios.
//! One scale row (`bitsize0_ub`, `sign_extend` with `BitSize` 0) is excluded because UBSan
//! flags the shift `1 << (BitSize - 1)` as having a negative exponent, so 22 rows remain.
//!
//! Each scenario feeds the fake HID reader step by step through the verbatim
//! `process_hid_report` logic. `removal` is the verbatim `release_contacts` on what is left.

use super::*;

/// One `HidP_GetUsageValue` source: `(page, collection, usage) -> value`.
struct Field {
    page: u16,
    collection: u16,
    usage: u16,
    value: u32,
}

/// One `process_hid_report` call: the fields it reads, and the events and contacts it produced.
struct Step {
    fields: &'static [Field],
    events: &'static [PointerEvent],
    contacts: &'static [Contact],
}

struct Scenario {
    name: &'static str,
    caps: &'static [ContactCaps],
    steps: &'static [Step],
    removal: &'static [PointerEvent],
}

/// Fake `HidP` reads backed by the harness's `fields[]`. `HidP_GetUsages` reports one usage
/// when the field value is non-zero.
struct FakeReport(&'static [Field]);

impl FakeReport {
    fn field(&self, page: u16, collection: u16, usage: u16) -> Option<&Field> {
        self.0.iter().find(|field| {
            field.page == page && field.collection == collection && field.usage == usage
        })
    }
}

impl ReportReader for FakeReport {
    fn usage_value(&self, page: u16, collection: u16, usage: u16) -> Option<u32> {
        self.field(page, collection, usage).map(|field| field.value)
    }

    fn usages(&self, page: u16, collection: u16, usage: u16) -> Option<u32> {
        self.field(page, collection, usage)
            .map(|field| u32::from(field.value != 0))
    }
}
/// `(value, bit_size, logical_min, logical_max, min, max, sign_extend, scale_value)`
type ScaleRow = (u32, u16, i32, i32, i32, i32, i32, i32);

const SCALE_ROWS: &[ScaleRow] = &[
    (255, 8, -128, 127, 0, 65535, -1, 32639),   // signed_ff_8bit
    (127, 8, -128, 127, 0, 65535, 127, 65535),  // signed_7f_8bit_max
    (128, 8, -128, 127, 0, 65535, -128, 0),     // signed_80_8bit_min
    (128, 8, 0, 255, 0, 65535, 128, 32896),     // unsigned_range_80
    (2048, 16, 0, 4095, 0, 65535, 2048, 32776), // mid_16bit
    (5000, 16, 0, 4095, 0, 65535, 5000, 0),     // out_of_range_16bit
    (5, 8, 5, 5, 0, 65535, 5, -1),              // zero_width_range
    (9, 8, 10, 5, 0, 65535, 9, 0),              // inverted_range
    (65535, 16, -32768, 32767, 0, 65535, -1, 32767), // signed_ffff_16bit
    (1, 1, -1, 1, 0, 100, 1, 100),              // one_bit_sign
];

const MUL_DIV_ROWS: &[(i32, i32, i32, i32)] = &[
    (5, 3, 2, 8),            // round_half_up_positive
    (-5, 3, 2, -8),          // negative_product_rounds_away
    (5, 3, -2, -8),          // negative_divisor
    (7, 1, 0, -1),           // zero_divisor
    (100000, 100000, 1, -1), // overflow_to_minus_one
];

const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "single_contact_lifecycle",
        caps: &[ContactCaps {
            link_collection: 1,
            x: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
            y: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
        }],
        steps: &[
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 3,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 2048,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 1024,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Down,
                    id: 3,
                    flags: 16391,
                    x: 32776,
                    y: 16388,
                }],
                contacts: &[Contact {
                    id: 3,
                    x: 32776,
                    y: 16388,
                }],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 3,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 100,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 200,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Update,
                    id: 3,
                    flags: 16390,
                    x: 1600,
                    y: 3201,
                }],
                contacts: &[Contact {
                    id: 3,
                    x: 1600,
                    y: 3201,
                }],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 0,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Up,
                    id: 3,
                    flags: 16384,
                    x: 1600,
                    y: 3201,
                }],
                contacts: &[],
            },
        ],
        removal: &[],
    },
    Scenario {
        name: "missing_contact_count_keeps_state",
        caps: &[ContactCaps {
            link_collection: 1,
            x: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
            y: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
        }],
        steps: &[
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 9,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 0,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 0,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Down,
                    id: 9,
                    flags: 16391,
                    x: 0,
                    y: 0,
                }],
                contacts: &[Contact { id: 9, x: 0, y: 0 }],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 9,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 4000,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 4000,
                    },
                ],
                events: &[],
                contacts: &[Contact { id: 9, x: 0, y: 0 }],
            },
        ],
        removal: &[PointerEvent {
            kind: PointerKind::Up,
            id: 9,
            flags: 16384,
            x: 0,
            y: 0,
        }],
    },
    Scenario {
        name: "two_collections_signed_scale",
        caps: &[
            ContactCaps {
                link_collection: 1,
                x: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
                y: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 1023,
                },
            },
            ContactCaps {
                link_collection: 2,
                x: ValueCaps {
                    bit_size: 8,
                    logical_min: -128,
                    logical_max: 127,
                },
                y: ValueCaps {
                    bit_size: 8,
                    logical_min: 0,
                    logical_max: 255,
                },
            },
        ],
        steps: &[
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 2,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 1,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 4095,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 1023,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 81,
                        value: 2,
                    },
                    Field {
                        page: 1,
                        collection: 2,
                        usage: 48,
                        value: 255,
                    },
                    Field {
                        page: 1,
                        collection: 2,
                        usage: 49,
                        value: 128,
                    },
                ],
                events: &[
                    PointerEvent {
                        kind: PointerKind::Down,
                        id: 1,
                        flags: 16391,
                        x: 65535,
                        y: 65535,
                    },
                    PointerEvent {
                        kind: PointerKind::Down,
                        id: 2,
                        flags: 16391,
                        x: 32639,
                        y: 32896,
                    },
                ],
                contacts: &[
                    Contact {
                        id: 1,
                        x: 65535,
                        y: 65535,
                    },
                    Contact {
                        id: 2,
                        x: 32639,
                        y: 32896,
                    },
                ],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 0,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 81,
                        value: 2,
                    },
                    Field {
                        page: 1,
                        collection: 2,
                        usage: 48,
                        value: 127,
                    },
                    Field {
                        page: 1,
                        collection: 2,
                        usage: 49,
                        value: 0,
                    },
                ],
                events: &[
                    PointerEvent {
                        kind: PointerKind::Update,
                        id: 2,
                        flags: 16390,
                        x: 65535,
                        y: 0,
                    },
                    PointerEvent {
                        kind: PointerKind::Up,
                        id: 1,
                        flags: 16384,
                        x: 65535,
                        y: 65535,
                    },
                ],
                contacts: &[Contact {
                    id: 2,
                    x: 65535,
                    y: 0,
                }],
            },
        ],
        removal: &[PointerEvent {
            kind: PointerKind::Up,
            id: 2,
            flags: 16384,
            x: 65535,
            y: 0,
        }],
    },
    Scenario {
        name: "duplicate_contact_id",
        caps: &[
            ContactCaps {
                link_collection: 1,
                x: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
                y: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
            },
            ContactCaps {
                link_collection: 2,
                x: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
                y: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
            },
        ],
        steps: &[
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 2,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 7,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 1,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 81,
                        value: 7,
                    },
                    Field {
                        page: 1,
                        collection: 2,
                        usage: 48,
                        value: 2,
                    },
                    Field {
                        page: 1,
                        collection: 2,
                        usage: 49,
                        value: 2,
                    },
                ],
                events: &[
                    PointerEvent {
                        kind: PointerKind::Down,
                        id: 7,
                        flags: 16391,
                        x: 16,
                        y: 16,
                    },
                    PointerEvent {
                        kind: PointerKind::Down,
                        id: 7,
                        flags: 16391,
                        x: 32,
                        y: 32,
                    },
                ],
                contacts: &[
                    Contact {
                        id: 7,
                        x: 16,
                        y: 16,
                    },
                    Contact {
                        id: 7,
                        x: 32,
                        y: 32,
                    },
                ],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 7,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 5,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 5,
                    },
                    Field {
                        page: 13,
                        collection: 2,
                        usage: 66,
                        value: 0,
                    },
                ],
                events: &[
                    PointerEvent {
                        kind: PointerKind::Update,
                        id: 7,
                        flags: 16390,
                        x: 80,
                        y: 80,
                    },
                    PointerEvent {
                        kind: PointerKind::Up,
                        id: 7,
                        flags: 16384,
                        x: 32,
                        y: 32,
                    },
                ],
                contacts: &[Contact {
                    id: 7,
                    x: 80,
                    y: 80,
                }],
            },
        ],
        removal: &[PointerEvent {
            kind: PointerKind::Up,
            id: 7,
            flags: 16384,
            x: 80,
            y: 80,
        }],
    },
    Scenario {
        name: "id_truncated_to_wparaml",
        caps: &[ContactCaps {
            link_collection: 1,
            x: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
            y: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
        }],
        steps: &[
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 65541,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 0,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 0,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Down,
                    id: 5,
                    flags: 16391,
                    x: 0,
                    y: 0,
                }],
                contacts: &[Contact {
                    id: 65541,
                    x: 0,
                    y: 0,
                }],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 65541,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 0,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 0,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Update,
                    id: 5,
                    flags: 16390,
                    x: 0,
                    y: 0,
                }],
                contacts: &[Contact {
                    id: 65541,
                    x: 0,
                    y: 0,
                }],
            },
        ],
        removal: &[PointerEvent {
            kind: PointerKind::Up,
            id: 5,
            flags: 16384,
            x: 0,
            y: 0,
        }],
    },
    Scenario {
        name: "missing_field_releases_contact",
        caps: &[ContactCaps {
            link_collection: 1,
            x: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
            y: ValueCaps {
                bit_size: 16,
                logical_min: 0,
                logical_max: 4095,
            },
        }],
        steps: &[
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 4,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 1000,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 1000,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Down,
                    id: 4,
                    flags: 16391,
                    x: 16004,
                    y: 16004,
                }],
                contacts: &[Contact {
                    id: 4,
                    x: 16004,
                    y: 16004,
                }],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 81,
                        value: 4,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 1000,
                    },
                ],
                events: &[PointerEvent {
                    kind: PointerKind::Up,
                    id: 4,
                    flags: 16384,
                    x: 16004,
                    y: 16004,
                }],
                contacts: &[],
            },
            Step {
                fields: &[
                    Field {
                        page: 13,
                        collection: 0,
                        usage: 84,
                        value: 1,
                    },
                    Field {
                        page: 13,
                        collection: 1,
                        usage: 66,
                        value: 1,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 48,
                        value: 1,
                    },
                    Field {
                        page: 1,
                        collection: 1,
                        usage: 49,
                        value: 1,
                    },
                ],
                events: &[],
                contacts: &[],
            },
        ],
        removal: &[],
    },
    Scenario {
        name: "release_order_on_removal",
        caps: &[
            ContactCaps {
                link_collection: 1,
                x: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
                y: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
            },
            ContactCaps {
                link_collection: 2,
                x: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
                y: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
            },
            ContactCaps {
                link_collection: 3,
                x: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
                y: ValueCaps {
                    bit_size: 16,
                    logical_min: 0,
                    logical_max: 4095,
                },
            },
        ],
        steps: &[Step {
            fields: &[
                Field {
                    page: 13,
                    collection: 0,
                    usage: 84,
                    value: 3,
                },
                Field {
                    page: 13,
                    collection: 1,
                    usage: 66,
                    value: 1,
                },
                Field {
                    page: 13,
                    collection: 1,
                    usage: 81,
                    value: 10,
                },
                Field {
                    page: 1,
                    collection: 1,
                    usage: 48,
                    value: 0,
                },
                Field {
                    page: 1,
                    collection: 1,
                    usage: 49,
                    value: 0,
                },
                Field {
                    page: 13,
                    collection: 2,
                    usage: 66,
                    value: 1,
                },
                Field {
                    page: 13,
                    collection: 2,
                    usage: 81,
                    value: 11,
                },
                Field {
                    page: 1,
                    collection: 2,
                    usage: 48,
                    value: 4095,
                },
                Field {
                    page: 1,
                    collection: 2,
                    usage: 49,
                    value: 0,
                },
                Field {
                    page: 13,
                    collection: 3,
                    usage: 66,
                    value: 1,
                },
                Field {
                    page: 13,
                    collection: 3,
                    usage: 81,
                    value: 12,
                },
                Field {
                    page: 1,
                    collection: 3,
                    usage: 48,
                    value: 0,
                },
                Field {
                    page: 1,
                    collection: 3,
                    usage: 49,
                    value: 4095,
                },
            ],
            events: &[
                PointerEvent {
                    kind: PointerKind::Down,
                    id: 10,
                    flags: 16391,
                    x: 0,
                    y: 0,
                },
                PointerEvent {
                    kind: PointerKind::Down,
                    id: 11,
                    flags: 16391,
                    x: 65535,
                    y: 0,
                },
                PointerEvent {
                    kind: PointerKind::Down,
                    id: 12,
                    flags: 16391,
                    x: 0,
                    y: 65535,
                },
            ],
            contacts: &[
                Contact { id: 10, x: 0, y: 0 },
                Contact {
                    id: 11,
                    x: 65535,
                    y: 0,
                },
                Contact {
                    id: 12,
                    x: 0,
                    y: 65535,
                },
            ],
        }],
        removal: &[
            PointerEvent {
                kind: PointerKind::Up,
                id: 10,
                flags: 16384,
                x: 0,
                y: 0,
            },
            PointerEvent {
                kind: PointerKind::Up,
                id: 11,
                flags: 16384,
                x: 65535,
                y: 0,
            },
            PointerEvent {
                kind: PointerKind::Up,
                id: 12,
                flags: 16384,
                x: 0,
                y: 65535,
            },
        ],
    },
];
#[test]
fn oracle_sign_extend_and_scale_rows() {
    for &(value, bit_size, logical_min, logical_max, min, max, sign, scaled) in SCALE_ROWS {
        let caps = ValueCaps {
            bit_size,
            logical_min,
            logical_max,
        };
        assert_eq!(
            sign_extend(value, &caps),
            sign,
            "sign_extend({value}, bit_size {bit_size}, {logical_min}..={logical_max})"
        );
        assert_eq!(
            scale_value(value, &caps, min, max),
            scaled,
            "scale_value({value}, bit_size {bit_size}, {logical_min}..={logical_max}, {min}..={max})"
        );
    }
}

#[test]
fn oracle_mul_div_rows() {
    for &(a, b, c, expected) in MUL_DIV_ROWS {
        assert_eq!(mul_div(a, b, c), expected, "mul_div({a}, {b}, {c})");
    }
}

#[test]
fn oracle_contact_scenarios() {
    for scenario in SCENARIOS {
        let mut tracker = ContactTracker::default();
        for (index, step) in scenario.steps.iter().enumerate() {
            let mut events = Vec::new();
            tracker.process_report(&FakeReport(step.fields), scenario.caps, &mut events);
            assert_eq!(
                events, step.events,
                "{} step {index}: events",
                scenario.name
            );
            assert_eq!(
                tracker.contacts(),
                step.contacts,
                "{} step {index}: contacts",
                scenario.name
            );
        }
        let mut events = Vec::new();
        tracker.release_all(&mut events);
        assert_eq!(events, scenario.removal, "{}: removal", scenario.name);
    }
}
