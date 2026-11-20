use crate::value_name;

#[test]
fn names_match_the_wine_c_oracle_rows() {
    let rows: &[(&[u8], u8, &str)] = &[
        (&[0x00], 0x01, "X-01-00"),
        (&[0x01], 0x01, "X-01-01"),
        (&[0x02], 0x01, "X-01-02"),
        (&[0x03], 0x01, "X-01-03"),
        (&[0x04], 0x01, "X-01-04"),
        (&[0x05], 0x01, "X-01-05"),
        (&[0x06], 0x01, "X-01-06"),
        (&[0x07], 0x01, "X-01-07"),
        (&[0x08], 0x01, "X-01-08"),
        (&[0x09], 0x01, "X-01-09"),
        (&[0x0a], 0x01, "X-01-0A"),
        (&[0x0b], 0x01, "X-01-0B"),
        (&[0x0c], 0x01, "X-01-0C"),
        (&[0x0d], 0x01, "X-01-0D"),
        (&[0x0e], 0x01, "X-01-0E"),
        (&[0x0f], 0x01, "X-01-0F"),
        (&[], 0x00, "X-00-"),
        (&[0x7f, 0x80, 0xff], 0xff, "X-FF-7F80FF"),
        (b"Wine", 0x01, "X-01-57696E65"),
        (&[0x00, 0xff], 0x10, "X-10-00FF"),
    ];

    for &(resource, ntype, expected) in rows {
        assert_eq!(value_name(resource, ntype), expected);
    }
}
