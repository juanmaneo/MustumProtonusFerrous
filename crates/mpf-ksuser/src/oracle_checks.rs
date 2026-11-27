//! Oracle-derived test vectors for the ported ksuser stub logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`, lines 504-516) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/ksuser/`, and running the harness. The harness prints 18 rows, and every row
//! shows `raised=1`. ASan and UBSan flagged no row, so none are excluded and all 18 remain.

use super::*;

/// One harness row, in print order: the `module` and `function` passed in, then the record the
/// harness printed back: code, flags, number of parameters, module and function.
type Row = (
    &'static str,
    &'static str,
    u32,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// The 18 printed rows. Rows 01-04 are the four `ksuser.spec` exports called with `ksuser.dll`.
const ORACLE_ROWS: [Row; 18] = [
    // row 01
    (
        "ksuser.dll",
        "KsCreateAllocator",
        0x8000_0100,
        0x01,
        2,
        "ksuser.dll",
        "KsCreateAllocator",
    ),
    // row 02
    (
        "ksuser.dll",
        "KsCreateClock",
        0x8000_0100,
        0x01,
        2,
        "ksuser.dll",
        "KsCreateClock",
    ),
    // row 03
    (
        "ksuser.dll",
        "KsCreatePin",
        0x8000_0100,
        0x01,
        2,
        "ksuser.dll",
        "KsCreatePin",
    ),
    // row 04
    (
        "ksuser.dll",
        "KsCreateTopologyNode",
        0x8000_0100,
        0x01,
        2,
        "ksuser.dll",
        "KsCreateTopologyNode",
    ),
    // row 05
    ("ksuser.dll", "", 0x8000_0100, 0x01, 2, "ksuser.dll", ""),
    // row 06
    (
        "ksuser.dll",
        "KsUnknown",
        0x8000_0100,
        0x01,
        2,
        "ksuser.dll",
        "KsUnknown",
    ),
    // row 07
    (
        "",
        "KsCreateAllocator",
        0x8000_0100,
        0x01,
        2,
        "",
        "KsCreateAllocator",
    ),
    // row 08
    (
        "",
        "KsCreateClock",
        0x8000_0100,
        0x01,
        2,
        "",
        "KsCreateClock",
    ),
    // row 09
    ("", "KsCreatePin", 0x8000_0100, 0x01, 2, "", "KsCreatePin"),
    // row 10
    (
        "",
        "KsCreateTopologyNode",
        0x8000_0100,
        0x01,
        2,
        "",
        "KsCreateTopologyNode",
    ),
    // row 11
    ("", "", 0x8000_0100, 0x01, 2, "", ""),
    // row 12
    ("", "KsUnknown", 0x8000_0100, 0x01, 2, "", "KsUnknown"),
    // row 13
    (
        "fontsub.dll",
        "KsCreateAllocator",
        0x8000_0100,
        0x01,
        2,
        "fontsub.dll",
        "KsCreateAllocator",
    ),
    // row 14
    (
        "fontsub.dll",
        "KsCreateClock",
        0x8000_0100,
        0x01,
        2,
        "fontsub.dll",
        "KsCreateClock",
    ),
    // row 15
    (
        "fontsub.dll",
        "KsCreatePin",
        0x8000_0100,
        0x01,
        2,
        "fontsub.dll",
        "KsCreatePin",
    ),
    // row 16
    (
        "fontsub.dll",
        "KsCreateTopologyNode",
        0x8000_0100,
        0x01,
        2,
        "fontsub.dll",
        "KsCreateTopologyNode",
    ),
    // row 17
    ("fontsub.dll", "", 0x8000_0100, 0x01, 2, "fontsub.dll", ""),
    // row 18
    (
        "fontsub.dll",
        "KsUnknown",
        0x8000_0100,
        0x01,
        2,
        "fontsub.dll",
        "KsUnknown",
    ),
];

#[test]
fn oracle_wine_spec_unimplemented_stub_rows() {
    for (module, function, code, flags, number_parameters, p0, p1) in ORACLE_ROWS {
        assert_eq!(
            wine_spec_unimplemented_stub(module, function),
            StubException {
                code,
                flags,
                number_parameters,
                module: p0,
                function: p1,
            },
            "input module={module:?} function={function:?}"
        );
    }
}

#[test]
fn oracle_export_helper_rows() {
    // Rows 01-04: each helper is the `@ stub` export of that name, called with `ksuser.dll`.
    let helpers = [
        ks_create_allocator,
        ks_create_clock,
        ks_create_pin,
        ks_create_topology_node,
    ];
    for (row, helper) in ORACLE_ROWS.iter().zip(helpers) {
        let (_, _, code, flags, number_parameters, module, function) = *row;
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function,
            }
        );
    }
}
