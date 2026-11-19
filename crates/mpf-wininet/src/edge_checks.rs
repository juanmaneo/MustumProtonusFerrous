//! Edge cases for each layout and check. The table is copied from oracle lines, and each row's
//! comment gives its line in `oracle.txt`. The direct tests cover the intended behavior for full
//! RFC 850 weekday names, which the oracle cannot give reliably, and the error values.

use crate::cookie::{is_domain_legal, CookieDomainError};
use crate::date::parse_http_date;
use crate::oracle_checks::{decode, run};

/// `(input line, expected output)` copied from `oracle.txt`.
const ORACLE_EDGE: &[(&str, &str)] = &[
    (
        r"date|Mon, 06-Nov-1994 08:49:37 GMT",
        "ok 124285853770000000",
    ), // oracle 4
    (r"date|Sun Nov  6 08:49:37 1994", "ok 124285853770000000"), // oracle 5
    (r"date|Sun Nov 6 08:49:37 1994 GMT", "ok 124285853770000000"), // oracle 6
    (r"date|Sunday Nov 6 08:49:37 1994", "fail"),                // oracle 9
    (r"date|Jan 01 00:00:00 2020", "fail"),                      // oracle 10
    (r"date|Mon, 31 Feb 2020 00:00:00 GMT", "fail"),             // oracle 12
    (
        r"date|Sun, 29 Feb 2020 00:00:00 GMT",
        "ok 132274080000000000",
    ), // oracle 14
    (r"date|Sun Nov 31 08:49:37 1994", "fail"),                  // oracle 15
    (r"date|Thu, 01 Jan 1600 00:00:00 GMT", "fail"),             // oracle 17
    (r"date|Thu, 01 Jan 30828 00:00:00 GMT", "fail"),            // oracle 19
    (r"date|Thu, 01 Jan 2020 23:59:60 GMT", "fail"),             // oracle 21
    (r"date|Thu, 01 Jan 2020 10:00:00 PST", "fail"),             // oracle 22
    (r"date|Thu, 01 Jan 2020 10:00:00", "fail"),                 // oracle 23
    (r"date|Thu, 01 Jan 2020 10:00:00 GMT ", "fail"),            // oracle 24
    (r"date|Thu Jan 1 10:00:00 2020 GMT", "ok 132223464000000000"), // oracle 25
    (r"date| Sun,  6  Nov 1994 8:49:37 GMT", "fail"),            // oracle 27
    (
        r"date|Thu, 01 Jan 2020 -0:00:00 GMT",
        "ok 132223104000000000",
    ), // oracle 28
    (r"date|Sun, 06 Nov 1994 -8:49:37 GMT", "fail"),             // oracle 29
    (r"date|Funday, 06-Nov-1994 08:49:37 GMT", "fail"),          // oracle 31
    (r"date|Sun", "fail"),                                       // oracle 32
    (r"match|server.example|server.example", "1"),               // oracle 33
    (r"match|WWW.Example.COM|*.example.com", "1"),               // oracle 36
    (r"match|www.example.com|www.example.com", "1"),             // oracle 37
    (r"match|www.example.com|example.com", "0"),                 // oracle 38
    (r"match|a.b|<local>", "0"),                                 // oracle 40
    (r"match|www.b.c|*.b.c", "1"),                               // oracle 42
    (r"match|www.foo.a.b|*.a.b", "1"),                           // oracle 43
    (r"match|a.b|*.a.b", "0"),                                   // oracle 44
    (r"match|foo|*.bar", "0"),                                   // oracle 45
    (r"match|foo.|*.", "1"),                                     // oracle 46
    (r"match|foo.bar|*.", "0"),                                  // oracle 47
    (r"match|a.example.com|", "0"),                              // oracle 48
    (r"match|example.com|*", "0"),                               // oracle 49
    (r"match|x.y|*x", "0"),                                      // oracle 50
    (r"legal||www.example.com", "invalid_name"),                 // oracle 53
    (r"legal|example.com|", "invalid_name"),                     // oracle 54
    (r"legal|example.com|com", "rejected"),                      // oracle 57
    (r"legal|ample.com|example.com", "invalid_parameter"),       // oracle 58
    (r"legal|EXAMPLE.COM|www.example.com", "legal"),             // oracle 59
    (r"legal|example.com|example.com", "legal"),                 // oracle 60
    (r"keep|HTTP/1.1|close", "0"),                               // oracle 62
    (r"keep|HTTP/1.1|TE, close, Upgrade", "0"),                  // oracle 64
    (r"keep|HTTP/1.1|Closed, x", "1"),                           // oracle 65
    (r"keep|HTTP/1.0|Keep-Alive", "1"),                          // oracle 66
    (r"keep|HTTP/1.0|", "0"),                                    // oracle 67
    (r"ulong|-0", "0 2"),                                        // oracle 71
    (r"ulong|+7", "7 2"),                                        // oracle 72
    (r"ulong|-4294967295", "1 11"),                              // oracle 73
    (r"ulong|-4294967296", "1 11"),                              // oracle 75
    (r"ulong|\u0663\u0664", "34 2"),                             // oracle 76
    (r"ulong|x", "0 0"),                                         // oracle 77
    (r"ulong|-", "0 0"),                                         // oracle 78
    (
        r"date|Sun, 06 Nov 1994 23:59:59 GMT",
        "ok 124286399990000000",
    ), // oracle 79
    (
        r"date|Sun, 06 Nov 1994 00:00:00 GMT",
        "ok 124285536000000000",
    ), // oracle 80
    (
        r"date|Sun, 06 Nov 1994 08:49:37 UTC",
        "ok 124285853770000000",
    ), // oracle 81
    (r"date|Sun, 06 Nov 1994 08:49:37 EST", "fail"),             // oracle 82
    (
        r"date|Sun, 06 Nov 1994 08:49:37 GMT\u0000junk",
        "ok 124285853770000000",
    ), // oracle 83
    (
        r"date|Sun, 06 Nov 1994 08:49:37GMT",
        "ok 124285853770000000",
    ), // oracle 84
    (r"date|Sun, 06 Nov 1994 08-49-37 GMT", "fail"),             // oracle 85
    (r"date|Sun, 06 Nov 1994 8:49 GMT", "fail"),                 // oracle 86
    (r"date|Sun, 06 Nov 1600 08:49:37 GMT", "fail"),             // oracle 87
    (r"date|Sun, 06 Nov x 08:49:37 GMT", "fail"),                // oracle 88
    (r"date|Sun, 00 Nov 1994 08:49:37 GMT", "fail"),             // oracle 89
    (r"date|Sun, 32 Nov 1994 08:49:37 GMT", "fail"),             // oracle 90
    (r"date|Sun, 06 Xyz 1994 08:49:37 GMT", "fail"),             // oracle 91
    (r"date|Sux, 06 Nov 1994 08:49:37 GMT", "fail"),             // oracle 92
    (r"date|Sunday, 06 Nov 1994 08:49:37 GMT", "fail"),          // oracle 93
    (r"date|Funny, 06 Nov 1994 08:49:37 GMT", "fail"),           // oracle 94
    (r"date|Sun, 06 Nov 1994 08:49:37 PST", "fail"),             // oracle 95
    (r"date|Sun Nov 6 23:59:59 1994", "ok 124286399990000000"),  // oracle 96
    (r"date|Sun Nov 6 08:49:37 1994 UTC", "ok 124285853770000000"), // oracle 97
    (r"date|Sun Nov 6 08:49:37 1994 ", "ok 124285853770000000"), // oracle 98
    (r"date|Sun Nov 6 08:49:37 1994 PST", "fail"),               // oracle 99
    (r"date|Sun Nov 0 08:49:37 1994", "fail"),                   // oracle 100
    (r"date|Sun Nov 6 08:49:37 1600", "fail"),                   // oracle 101
    (r"date|Foo Nov 6 08:49:37 1994", "fail"),                   // oracle 102
    (r"date|Sun Foo 6 08:49:37 1994", "fail"),                   // oracle 103
    (r"date|Mondaysxxxx, 06-Nov-1994 08:49:37 GMT", "fail"),     // oracle 104
    (r"date|Sunday, 06-Xyz-1994 08:49:37 GMT", "fail"),          // oracle 106
    (r"date|Sunday, 06Nov-1994 08:49:37 GMT", "fail"),           // oracle 107
    (r"date|Sunday, 06-Nov1994 08:49:37 GMT", "fail"),           // oracle 108
    (r"date|Sunday, 06-Nov-1600 08:49:37 GMT", "fail"),          // oracle 109
    (r"date|Sunday, 06-Nov-1994 08:49:37 EST", "fail"),          // oracle 110
    (r"date|Sunday, 00-Nov-1994 08:49:37 GMT", "fail"),          // oracle 111
    (r"match|localhost|<local>", "1"),                           // oracle 112
    (r"match|<LOCAL>|<local>", "1"),                             // oracle 113
    (r"match|www.example|<local>", "0"),                         // oracle 114
    (r"match|foo|*.", "0"),                                      // oracle 115
    (r"match|www.example.com|*.example.org", "0"),               // oracle 116
    (r"match|www.example.com|*.ample.com", "0"),                 // oracle 117
    (r"match|xample.com|*.example.com", "0"),                    // oracle 118
    (r"match|foo|bar", "0"),                                     // oracle 119
    (r"match|foo|fo", "0"),                                      // oracle 120
    (r"match|www.example.com|www.example.com.", "0"),            // oracle 121
    (r"legal|www.example.com|example.com", "rejected"),          // oracle 122
    (r"legal|examplecom|example.com", "rejected"),               // oracle 123
    (r"legal|example.com|examplecom", "rejected"),               // oracle 124
    (r"legal|.|www.example.com", "invalid_name"),                // oracle 125
    (r"legal|example.com|.www.example.com", "invalid_name"),     // oracle 126
    (r"legal|a.b|xa.b", "invalid_parameter"),                    // oracle 127
    (r"legal|b.c|a.b.c", "legal"),                               // oracle 128
    (r"keep|HTTP/1.1|Close", "0"),                               // oracle 129
    (r"keep|HTTP/1.0|keep-alive", "1"),                          // oracle 130
    (r"keep|HTTP/1.0|Keep-Alive-Foo", "0"),                      // oracle 131
    (r"keep|HTTP/1.1|,,close,,", "0"),                           // oracle 132
    (r"keep|HTTP/1.1|xclose", "1"),                              // oracle 133
    (r"keep|HTTP/2.0|Keep-Alive", "1"),                          // oracle 134
    (r"keep|HTTP/1.1|keep-alive\u0000Close", "1"),               // oracle 135
    (r"keep|HTTP/1.1\u0000x|close", "0"),                        // oracle 136
    (r"ulong|4294967295", "4294967295 10"),                      // oracle 137
    (r"ulong|4294967296", "4294967295 10"),                      // oracle 138
    (r"ulong|99999999999999999999999", "4294967295 23"),         // oracle 139
    (r"ulong|-99999999999999999999999", "1 24"),                 // oracle 140
    (r"ulong|\u0968", "2 1"),                                    // oracle 141
    (r"ulong|\u0660x", "0 1"),                                   // oracle 142
    (r"ulong|\u0669", "9 1"),                                    // oracle 143
    (r"ulong|\u066a", "0 0"),                                    // oracle 144
    (r"ulong|\u065f", "0 0"),                                    // oracle 145
    (r"ulong|\uff1a", "0 0"),                                    // oracle 146
    (r"ulong|\uff19", "9 1"),                                    // oracle 147
    (r"ulong|\u0009\u000b\u000c\u000d 7", "7 6"),                // oracle 148
    (r"ulong|\u00a07", "0 0"),                                   // oracle 149
    (r"date|00", "fail"),                                        // oracle 150
    (r"date|Su", "fail"),                                        // oracle 151
    (r"date|Monday, 06 Nov 1994 08:49:37 GMT", "fail"),          // oracle 152
    (r"date|Sun, x Nov 1994 08:49:37 GMT", "fail"),              // oracle 153
    (r"date|Sun, 06 N", "fail"),                                 // oracle 154
    (r"date|Sun, 06 Nov", "fail"),                               // oracle 155
    (r"date|Sun, 06 Nov 1994 08:60:37 GMT", "fail"),             // oracle 156
    (r"date|Sun, 31 Apr 1994 08:49:37 GMT", "fail"),             // oracle 157
    (
        r"date|Sun, 30 Apr 1994 08:49:37 GMT",
        "ok 124121693770000000",
    ), // oracle 158
    (r"date|Sun, 30 Feb 1994 08:49:37 GMT", "fail"),             // oracle 159
    (r"date|Sun Nov 6 24:49:37 1994", "fail"),                   // oracle 160
    (r"date|Sun Nov 6 08:60:37 1994", "fail"),                   // oracle 161
    (r"date|Sun Nov 6 08:49:60 1994", "fail"),                   // oracle 162
    (r"date|Sun Nov 6 08-49-37 1994", "fail"),                   // oracle 163
    (r"date|Sun Nov x 08:49:37 1994", "fail"),                   // oracle 164
    (r"date|Sun Nov 6 08:49:37 x", "fail"),                      // oracle 165
    (r"date|Sun Nov 6 08:49:37 30828", "fail"),                  // oracle 166
    (r"date|Sunxyz Nov 6 08:49:37 1994", "fail"),                // oracle 167
    (r"date|Sux, 06-Nov-1994 08:49:37 GMT", "fail"),             // oracle 168
    (r"date|, 06-Nov-1994 08:49:37 GMT", "fail"),                // oracle 169
    (r"date|Sunday, x-Nov-1994 08:49:37 GMT", "fail"),           // oracle 170
    (r"date|Sunday, 06-N", "fail"),                              // oracle 171
    (r"date|Sunday, 06-Nov 1994 08:49:37 GMT", "fail"),          // oracle 172
    (r"date|Sunday, 06-Nov-x 08:49:37 GMT", "fail"),             // oracle 173
    (r"date|Sunday, 30-Feb-1994 08:49:37 GMT", "fail"),          // oracle 174
    (r"date|Sunday, 06-Nov-1994 08:60:37 GMT", "fail"),          // oracle 175
    (r"legal|a.b|abc", "rejected"),                              // oracle 177
    (r"keep|HTTP/1.0|Keep-Alive\u0009x", "0"),                   // oracle 178
    (r"date|0\u0000x", "ok 0"),                                  // oracle 179
    (r"date|\u0000Sun, 06 Nov 1994 08:49:37 GMT", "fail"),       // oracle 180
    (
        r"date|Sun Nov 6 08:49:37 1994\u0000, x",
        "ok 124285853770000000",
    ), // oracle 181
    (
        r"date|Sun, 06 Nov 1994 08: 49: 37 GMT",
        "ok 124285853770000000",
    ), // oracle 182
    (
        r"date|Sun, 06 Nov 1994 08:49:37  GMT",
        "ok 124285853770000000",
    ), // oracle 183
    (r"date|Sun, 06 Nov 1994 xx:49:37 GMT", "fail"),             // oracle 184
    (r"date|Sun, 06 Nov 1994 08:xx:37 GMT", "fail"),             // oracle 185
    (r"date|Sun, 06 Nov 1994 08:49:xx GMT", "fail"),             // oracle 186
    (r"date|Sun, 06 Nov 1994 08:49:-1 GMT", "fail"),             // oracle 187
    (
        r"date|Sun, 29 Feb 2100 00:00:00 GMT",
        "ok 157520160000000000",
    ), // oracle 188
    (
        r"date|Sun, 29 Feb 1994 00:00:00 GMT",
        "ok 124069536000000000",
    ), // oracle 189
    (
        r"date|Sun, 31 Dec 30827 23:59:59 GMT",
        "ok 9223149887990000000",
    ), // oracle 190
    (
        r"date|Sun, 31 Jan 1994 08:49:37 GMT",
        "ok 124044797770000000",
    ), // oracle 191
    (
        r"date|Sun, 6 Nov 1994 08:49:37 GMT",
        "ok 124285853770000000",
    ), // oracle 192
    (
        r"date|Sun, 06Nov 1994 08:49:37 GMT",
        "ok 124285853770000000",
    ), // oracle 193
    (r"date|Sun, 06 Nov 1994 08:49:37 Z", "fail"),               // oracle 194
    (r"date|Sun Nov 6 08:49:37 1994 GMT x", "fail"),             // oracle 195
    (r"date|Sun, 06 -Nov-1994 08:49:37 GMT", "fail"),            // oracle 196
    (r"date|Sun, 06-Nov-1994 08:49:37", "fail"),                 // oracle 197
    (r"date|Sun, 06-Nov-1994 08:49:37 utc", "fail"),             // oracle 198
    (r"date|Sun, 06-Nov-1994 08:49:37 GMT x", "fail"),           // oracle 199
    (r"date|Sun, 06-Nov-1994 08:49:-1 GMT", "fail"),             // oracle 200
    (r"match|foo.bar|<local>", "0"),                             // oracle 201
    (r"match|a.b.c.d|*.c.d", "1"),                               // oracle 202
    (r"match|x.c.d|*.c.d", "1"),                                 // oracle 203
    (r"match||", "0"),                                           // oracle 204
    (r"legalx|a\u0000.b|x.a\u0000.b", "legal"),                  // oracle 205
    (r"legalx|a\u0000.b|x.a\u0000.c", "legal"),                  // oracle 206
    (r"legal|a\u0000.b|x.a\u0000.b", "rejected"),                // oracle 207
    (r"keep|HTTP/1.1|Close\u0000x", "0"),                        // oracle 208
    (r"keep|HTTP/1.1|  , close", "0"),                           // oracle 209
    (r"ulong|9223372036854775807", "4294967295 19"),             // oracle 210
    (r"ulong|9223372036854775808", "4294967295 19"),             // oracle 211
    (r"ulong|-9223372036854775808", "1 20"),                     // oracle 212
    (r"ulong|-9223372036854775809", "1 20"),                     // oracle 213
    (r"ulong|+", "0 0"),                                         // oracle 214
    (r"ulong|--5", "0 0"),                                       // oracle 215
    (r"ulong|4294967294", "4294967294 10"),                      // oracle 216
];

#[test]
fn oracle_edge_rows_match_the_port() {
    for &(line, expected) in ORACLE_EDGE {
        assert_eq!(run(line), expected, "{line}");
    }
}

#[test]
fn unknown_mode_is_an_error() {
    assert_eq!(run("bogus|x|y"), "error");
}

#[test]
fn zero_and_empty_dates() {
    assert_eq!(parse_http_date(&decode("0")), Some(0));
    assert_eq!(parse_http_date(&decode("0\\u0000x")), Some(0));
    assert_eq!(parse_http_date(&decode("00")), None);
    assert_eq!(parse_http_date(&[]), None);
}

#[test]
fn full_rfc850_weekday_names_are_accepted() {
    // Not oracle rows. In C the full names read an uninitialized unit after the weekday, and
    // `Wednesday` writes past its buffer, so the intended result is checked here instead.
    let expected = Some(124_285_853_770_000_000);
    assert_eq!(
        parse_http_date(&decode("Sunday, 06-Nov-1994 08:49:37 GMT")),
        expected
    );
    assert_eq!(
        parse_http_date(&decode("Wednesday, 06-Nov-1994 08:49:37 GMT")),
        expected
    );
}

#[test]
fn cookie_domain_results_and_names() {
    let check = |domain: &str, full: &str| is_domain_legal(&decode(domain), &decode(full));
    assert_eq!(check("example.com", "www.example.com"), Ok(()));
    assert_eq!(
        check("", "www.example.com"),
        Err(CookieDomainError::InvalidName)
    );
    assert_eq!(
        check("example.org", "www.example.com"),
        Err(CookieDomainError::InvalidParameter)
    );
    assert_eq!(
        check("com", "example.com"),
        Err(CookieDomainError::Rejected)
    );
    assert_eq!(
        format!("{:?}", CookieDomainError::InvalidName),
        "InvalidName"
    );
    assert_eq!(
        format!("{:?}", CookieDomainError::InvalidParameter),
        "InvalidParameter"
    );
    assert_eq!(format!("{:?}", CookieDomainError::Rejected), "Rejected");
}
