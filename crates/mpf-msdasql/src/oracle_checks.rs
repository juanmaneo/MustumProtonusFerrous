//! Replays of the oracle rows. The oracle compiles the verbatim Wine bodies from `dlls/msdasql/session.c` and
//! `msdasql_main.c` with `-fsanitize=address,undefined`, under `target/canon/msdasql` (gitignored). Each row below is a
//! line of that oracle's `oracle.txt`: the driver input and the output the C driver prints. The oracle has 211 rows,
//! and `edge_checks.rs` covers the rest of the port's branches. No row was excluded, because ASan and UBSan reported
//! nothing for any of them.

use super::*;

/// `(driver input, driver output)` pairs copied from `oracle.txt`. `\uXXXX` is a UTF-16 escape in the input.
const ROWS: &[(&str, &str)] = &[
    (r"bind|1|0", "129"),
    (r"bind|5|0", "18"),
    (r"bind|5|1", "2"),
    (r"bind|-16|1", "3"),
    (r"bind|-18|0", "19"),
    (r"bind|10|0", "7"),
    (r"bind|11|1", "135"),
    (r"bind|-25|1", "20"),
    (r"bind|-10|1", "130"),
    (r"bind|0|0", "3"),
    (r"var|-4", "1"),
    (r"var|-9", "0"),
    (r"fixed|-9", "0"),
    (r"fixed|12", "1"),
    (r"sqlname|-16", "SQL_C_SLONG"),
    (r"dbname|135", "DBTYPE_DBTIMESTAMP"),
    (r"dbname|136", "Unknown"),
    (r"mode|Share Exclusive", "S_OK:3:12"),
    (r"mode|Share Deny None x", "E_FAIL"),
    (r"mode|\u0052EAD", "S_OK:3:1"),
    (r"mode|Read\u0000Write", "S_OK:3:1"),
    (r"count|64|64,1,64", "2"),
    (r"count|3|1,2,3,7,8,9,11,15", "7"),
    (r"bogus|x", "error"),
];

/// Decodes the ASCII text of a driver field, with `\uXXXX` escapes, into UTF-16 units.
fn units(text: &str) -> Vec<u16> {
    let mut out = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        match rest.strip_prefix(r"\u") {
            Some(tail) => {
                out.push(u16::from_str_radix(&tail[..4], 16).unwrap());
                rest = &tail[4..];
            }
            None => {
                out.push(u16::from(rest.as_bytes()[0]));
                rest = &rest[1..];
            }
        }
    }
    out
}

/// Runs one driver line through the port and formats the result as the C driver prints it. `S_OK` is followed by
/// `VT_I4` (3) and the value, as the C driver prints the `VARIANT`.
fn run(line: &str) -> String {
    let fields: Vec<&str> = line.split('|').collect();
    let field = |i: usize| fields.get(i).copied().unwrap_or("");
    let num = |i: usize| field(i).parse::<i16>().unwrap();
    match field(0) {
        "bind" => sqltype_to_bindtype(num(1), field(2) != "0").to_string(),
        "var" => i32::from(is_variable_length(num(1))).to_string(),
        "fixed" => i32::from(is_fixed_length(num(1))).to_string(),
        "sqlname" => debugstr_sqltype(num(1)).to_owned(),
        "dbname" => debugstr_dbtype(field(1).parse().unwrap()).to_owned(),
        "mode" => match convert_dbproperty_mode(&units(field(1))) {
            Some(value) => format!("S_OK:3:{value}"),
            None => "E_FAIL".to_owned(),
        },
        "count" => {
            let flags: Vec<u32> = field(2).split(',').map(|t| t.parse().unwrap()).collect();
            get_property_count(field(1).parse().unwrap(), &flags).to_string()
        }
        _ => "error".to_owned(),
    }
}

#[test]
fn oracle_rows_match_the_port() {
    for &(line, expected) in ROWS {
        assert_eq!(run(line), expected, "{line}");
    }
}
