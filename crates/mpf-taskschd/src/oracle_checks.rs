use super::{validate_daily_days_interval, E_INVALIDARG, S_OK};

struct Vector {
    days: i16,
    expected_status: i32,
}

#[test]
fn wine_oracle_rows() {
    let vectors = [
        Vector {
            days: -32768,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: -32767,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: -10000,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: -100,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: -2,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: -1,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: 0,
            expected_status: E_INVALIDARG,
        },
        Vector {
            days: 1,
            expected_status: S_OK,
        },
        Vector {
            days: 2,
            expected_status: S_OK,
        },
        Vector {
            days: 3,
            expected_status: S_OK,
        },
        Vector {
            days: 7,
            expected_status: S_OK,
        },
        Vector {
            days: 28,
            expected_status: S_OK,
        },
        Vector {
            days: 29,
            expected_status: S_OK,
        },
        Vector {
            days: 30,
            expected_status: S_OK,
        },
        Vector {
            days: 31,
            expected_status: S_OK,
        },
        Vector {
            days: 365,
            expected_status: S_OK,
        },
        Vector {
            days: 366,
            expected_status: S_OK,
        },
        Vector {
            days: 1000,
            expected_status: S_OK,
        },
        Vector {
            days: 32766,
            expected_status: S_OK,
        },
        Vector {
            days: 32767,
            expected_status: S_OK,
        },
    ];

    for vector in vectors {
        assert_eq!(
            validate_daily_days_interval(vector.days),
            vector.expected_status,
            "days interval {}",
            vector.days
        );
    }
}
