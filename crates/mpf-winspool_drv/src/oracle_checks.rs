use super::is_valid_devmode_w;

struct Row {
    id: &'static str,
    /// `None` passes a NULL pointer. Otherwise: `(dmFields, dmSize, dmDriverExtra)`.
    header: Option<(u32, u16, u16)>,
    size: usize,
    expected: bool,
}

fn devmode(fields: u32, dm_size: u16, driver_extra: u16) -> Vec<u8> {
    let mut bytes = vec![0; 220];
    bytes[68..70].copy_from_slice(&dm_size.to_le_bytes());
    bytes[70..72].copy_from_slice(&driver_extra.to_le_bytes());
    bytes[72..76].copy_from_slice(&fields.to_le_bytes());
    bytes
}

#[test]
fn matches_25_wine_c_oracle_rows() {
    // Rows and results come from target/canon/winspool.drv/oracle.c, which
    // compiles Wine's IsValidDevmodeW verbatim with ASan and UBSan.
    let rows = [
        Row {
            id: "O01",
            header: None,
            size: 0,
            expected: false,
        },
        Row {
            id: "O02",
            header: Some((0, 72, 0)),
            size: 72,
            expected: false,
        },
        Row {
            id: "O03",
            header: Some((0, 73, 0)),
            size: 73,
            expected: false,
        },
        Row {
            id: "O04",
            header: Some((0, 74, 0)),
            size: 74,
            expected: false,
        },
        Row {
            id: "O05",
            header: Some((0, 75, 0)),
            size: 75,
            expected: false,
        },
        Row {
            id: "O06",
            header: Some((0, 76, 0)),
            size: 76,
            expected: true,
        },
        Row {
            id: "O07",
            header: Some((1, 76, 0)),
            size: 76,
            expected: false,
        },
        Row {
            id: "O08",
            header: Some((1, 77, 0)),
            size: 77,
            expected: false,
        },
        Row {
            id: "O09",
            header: Some((1, 78, 0)),
            size: 78,
            expected: true,
        },
        Row {
            id: "O10",
            header: Some((0x40, 180, 0)),
            size: 180,
            expected: false,
        },
        Row {
            id: "O11",
            header: Some((0x40, 181, 0)),
            size: 181,
            expected: false,
        },
        Row {
            id: "O12",
            header: Some((0x40, 182, 0)),
            size: 182,
            expected: false,
        },
        Row {
            id: "O13",
            header: Some((0x40, 183, 0)),
            size: 183,
            expected: false,
        },
        Row {
            id: "O14",
            header: Some((0x40, 184, 0)),
            size: 184,
            expected: true,
        },
        Row {
            id: "O15",
            header: Some((0, 75, 1)),
            size: 76,
            expected: false,
        },
        Row {
            id: "O16",
            header: Some((0, 76, 1)),
            size: 80,
            expected: true,
        },
        Row {
            id: "O17",
            header: Some((0, 76, 2)),
            size: 80,
            expected: true,
        },
        Row {
            id: "O18",
            header: Some((0, 76, 3)),
            size: 80,
            expected: true,
        },
        Row {
            id: "O19",
            header: Some((0, 76, 4)),
            size: 80,
            expected: true,
        },
        Row {
            id: "O20",
            header: Some((0, 76, 5)),
            size: 80,
            expected: false,
        },
        Row {
            id: "O21",
            header: Some((0, 88, 1)),
            size: 92,
            expected: true,
        },
        Row {
            id: "O22",
            header: Some((0, 88, 2)),
            size: 92,
            expected: true,
        },
        Row {
            id: "O23",
            header: Some((0, 88, 3)),
            size: 92,
            expected: true,
        },
        Row {
            id: "O24",
            header: Some((0, 88, 4)),
            size: 92,
            expected: true,
        },
        Row {
            id: "O25",
            header: Some((0, 88, 5)),
            size: 92,
            expected: false,
        },
    ];

    for row in rows {
        let got = match row.header {
            None => is_valid_devmode_w(None, row.size),
            Some((fields, dm_size, extra)) => {
                let bytes = devmode(fields, dm_size, extra);
                is_valid_devmode_w(Some(&bytes), row.size)
            }
        };
        assert_eq!(got, row.expected, "{}", row.id);
    }
}
