//! Oracle-derived test vectors for the ported fwpuclnt stubs.
//!
//! Vectors were captured by compiling the verbatim `dlls/fwpuclnt/main.c` with
//! minimal Windows type shims under `target/canon/fwpuclnt/` using
//! `gcc -fsanitize=address,undefined -g`, then running the harness. All 24
//! printed rows are kept. None was flagged by ASan or UBSan, so none is excluded.
//! Each row records the harness input, then the status and out-values printed.

use super::*;

#[test]
fn oracle_engine_open_rows() {
    // (harness input, status, *engineHandle)
    let rows: [(&str, u32, Handle); 4] = [
        ("server=NULL auth=0 out_pre=0x1234", 1764, 0),
        (
            "server=localhost auth=0xFFFFFFFF ids=set out_pre=0x1234",
            1764,
            0,
        ),
        ("server=empty auth=1 out_pre=0xdead", 1764, 0),
        ("server=x auth=0x10 out_pre=0x55", 1764, 0),
    ];
    for (input, status, handle) in rows {
        assert_eq!(fwpm_engine_open0(), (status, handle), "{input}");
    }
}

#[test]
fn oracle_engine_close_rows() {
    // (harness input, status)
    let rows: [(&str, u32); 4] = [
        ("h=NULL", 1764),
        ("h=0x1", 1764),
        ("h=-1", 1764),
        ("h=0x1234", 1764),
    ];
    for (input, status) in rows {
        assert_eq!(fwpm_engine_close0(), status, "{input}");
    }
}

#[test]
fn oracle_sub_layer_add_rows() {
    // (harness input, status)
    let rows: [(&str, u32); 3] = [
        ("engine=NULL sub=NULL sd=NULL", 1764),
        ("engine=0x1 sub=set sd=set", 1764),
        ("engine=0x1234 sub=NULL sd=set", 1764),
    ];
    for (input, status) in rows {
        assert_eq!(fwpm_sub_layer_add0(), status, "{input}");
    }
}

#[test]
fn oracle_sub_layer_get_by_key_rows() {
    // (harness input, status, *subLayer)
    let rows: [(&str, u32, Handle); 2] = [
        ("engine=NULL key=NULL out_pre=0xbeef", 1764, 0),
        ("engine=0x1 key=set out_pre=0xbeef", 1764, 0),
    ];
    for (input, status, sub_layer) in rows {
        assert_eq!(fwpm_sub_layer_get_by_key0(), (status, sub_layer), "{input}");
    }
}

#[test]
fn oracle_filter_create_enum_handle_rows() {
    // (harness input, status, *enumHandle)
    let rows: [(&str, u32, Handle); 2] = [
        ("engine=NULL tmpl=NULL out_pre=0x77", 1764, 0),
        ("engine=0x1 tmpl=set out_pre=0x77", 1764, 0),
    ];
    for (input, status, enum_handle) in rows {
        assert_eq!(
            fwpm_filter_create_enum_handle0(),
            (status, enum_handle),
            "{input}"
        );
    }
}

#[test]
fn oracle_filter_destroy_enum_handle_rows() {
    // (harness input, status)
    let rows: [(&str, u32); 2] = [
        ("engine=NULL enum=NULL", 1764),
        ("engine=0x1 enum=0x2", 1764),
    ];
    for (input, status) in rows {
        assert_eq!(fwpm_filter_destroy_enum_handle0(), status, "{input}");
    }
}

#[test]
fn oracle_filter_enum_rows() {
    // Every row prints the same out-values, whatever nEntriesRequested is:
    // req=0, req=1 and req=0xFFFFFFFF all come back as 1764, NULL and 0.
    let rows: [&str; 3] = [
        "req=0 entries_pre=0xcafe nret_pre=99",
        "req=1 entries_pre=0xcafe nret_pre=99",
        "req=0xFFFFFFFF entries_pre=0xcafe nret_pre=99",
    ];
    for input in rows {
        assert_eq!(
            fwpm_filter_enum0(),
            FilterEnum {
                status: 1764,
                entries: 0,
                n_entries_returned: 0,
            },
            "{input}"
        );
    }
}

#[test]
fn oracle_filter_add_rows() {
    // (harness input, status, *pFilterId after the call)
    let rows: [(&str, u32, u64); 2] = [
        (
            "engine=NULL filter=NULL sd=NULL id_pre=0xAAAA",
            1764,
            0xAAAA,
        ),
        ("engine=0x1 filter=set sd=set id_pre=0xAAAA", 1764, 0xAAAA),
    ];
    for (input, status, filter_id) in rows {
        assert_eq!(fwpm_filter_add0(0xAAAA), (status, filter_id), "{input}");
    }
}

#[test]
fn oracle_free_memory_rows() {
    // The stub has no return value and no observable effect. Both rows
    // (`mem=NULL` and `*mem=0x1111`) must complete without a panic.
    for _input in ["mem=NULL", "*mem=0x1111 -> mem unchanged"] {
        fwpm_free_memory0();
    }
}
