//! Oracle vectors for `recommend_dib_format`.
//!
//! The rows come from running Wine's verbatim `WinGRecommendDIBFormat` under
//! `gcc -fsanitize=address,undefined`. Each row is the header before the call,
//! the return value, and the header after the call; `None` is a null pointer.
//! The header values are the eleven `BITMAPINFOHEADER` fields in Win32 order,
//! so the casts in `header` only narrow values that already fit their field.

use super::*;

struct Row {
    before: Option<[i64; 11]>,
    returned: bool,
    after: Option<[i64; 11]>,
}

fn header(f: [i64; 11]) -> BitmapInfoHeader {
    BitmapInfoHeader {
        size: f[0] as u32,
        width: f[1] as i32,
        height: f[2] as i32,
        planes: f[3] as u16,
        bit_count: f[4] as u16,
        compression: f[5] as u32,
        size_image: f[6] as u32,
        x_pels_per_meter: f[7] as i32,
        y_pels_per_meter: f[8] as i32,
        clr_used: f[9] as u32,
        clr_important: f[10] as u32,
    }
}

const ROWS: [Row; 21] = [
    Row {
        before: None,
        returned: false,
        after: None,
    },
    Row {
        before: Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            4294967295, 2147483647, 2147483647, 65535, 65535, 4294967295, 4294967295, 2147483647,
            2147483647, 4294967295, 4294967295,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            0,
            -2147483648,
            -2147483648,
            0,
            0,
            0,
            0,
            -2147483648,
            -2147483648,
            0,
            0,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            2558280143,
            1127546737,
            -1602179383,
            1,
            65535,
            2658218913,
            2835104923,
            -2147483648,
            100575395,
            1,
            87204562,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            2122958142,
            -2047647177,
            -1388103592,
            49987,
            1,
            40,
            3436612826,
            -2147483648,
            -2147483648,
            3969198891,
            286434425,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            1, -194500387, 1, 37803, 65535, 698793523, 2791793815, -824302476, 0, 1, 2389762656,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            2131851036,
            -2147483648,
            1,
            65535,
            48181,
            1987346465,
            40,
            -621490157,
            1749528417,
            4294967295,
            2472105249,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            347335206,
            -1,
            2147483647,
            4216,
            35538,
            2645808298,
            0,
            -1359723808,
            2062968835,
            605284978,
            4294967295,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            40,
            -1220923368,
            278128275,
            52898,
            8,
            1,
            767318026,
            1972789939,
            -2147483648,
            40,
            40,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            3689630449,
            1940773351,
            -1818046246,
            8,
            8,
            1,
            40,
            -2147483648,
            -1,
            2162071968,
            236475998,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            3556564052, 1, 0, 0, 62810, 515888484, 40, 655814606, 0, 3906133038, 1,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            978063795,
            -2147483648,
            -1310935851,
            1,
            65535,
            3784651316,
            2232006035,
            2147483647,
            2147483647,
            0,
            3109021645,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            907850581,
            -910662689,
            -545521629,
            1,
            65278,
            1724882287,
            589029924,
            5757686,
            -2147483648,
            2078643526,
            0,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            2942748585,
            1303265921,
            115631507,
            8,
            0,
            335918138,
            40,
            -1871335744,
            1762391465,
            1864948109,
            3007942339,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            3470544168,
            -1033905698,
            1240239688,
            24544,
            32428,
            1,
            1,
            -165819707,
            -1712283869,
            398827203,
            0,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            4294967295, -577964450, 0, 52350, 18566, 1, 346102181, -741505679, 1, 1459459481,
            4294967295,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            1021756232,
            -1237271556,
            2147483647,
            4333,
            47500,
            115883380,
            2743648443,
            1771368531,
            0,
            2567439241,
            442642175,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            358867314,
            -736566859,
            -1594510177,
            59892,
            41904,
            2961355624,
            3766991517,
            1196265518,
            -496542121,
            4294967295,
            3456646640,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            3765680622, 1560095599, 937004091, 52558, 44419, 1, 4294967295, 1595472760, -1,
            994730266, 0,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
    Row {
        before: Some([
            40,
            -1246920569,
            1,
            34133,
            8,
            4294967295,
            1861139750,
            -1083076012,
            2147483647,
            4294967295,
            3585311764,
        ]),
        returned: true,
        after: Some([40, 320, 1, 1, 8, 0, 0, 0, 0, 0, 0]),
    },
];

#[test]
fn recommend_dib_format_matches_wine_oracle() {
    for row in ROWS {
        let mut before = row.before.map(header);
        assert_eq!(recommend_dib_format(before.as_mut()), row.returned);
        assert_eq!(before, row.after.map(header));
    }
}
