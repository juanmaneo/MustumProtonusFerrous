//! Oracle-derived test vectors for the ported `psapi.spec` export table.
//!
//! Vectors were captured by running the verbatim Wine `tools/winebuild` (built with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/psapi/`) in `--def -m32` mode on
//! `dlls/psapi/psapi.spec`, through `target/canon/psapi/oracle.sh`. The 29 printed rows are the
//! `LIBRARY` line, the 27 exports, and one probe spec, `@ stdcall -import Same(long) Same`,
//! whose export and link names match. ASan and UBSan reported nothing for any row, so no vector
//! was excluded.

use super::*;

/// The `.def` line that `winebuild` printed for each export, in spec order. Rows 02 to 28.
const EXPORT_ROWS: [(&str, &str); 27] = [
    (
        "EmptyWorkingSet",
        "  EmptyWorkingSet@4=K32EmptyWorkingSet @1",
    ),
    (
        "EnumDeviceDrivers",
        "  EnumDeviceDrivers@12=K32EnumDeviceDrivers @2",
    ),
    ("EnumPageFilesA", "  EnumPageFilesA@8=K32EnumPageFilesA @3"),
    ("EnumPageFilesW", "  EnumPageFilesW@8=K32EnumPageFilesW @4"),
    (
        "EnumProcessModules",
        "  EnumProcessModules@16=K32EnumProcessModules @5",
    ),
    (
        "EnumProcessModulesEx",
        "  EnumProcessModulesEx@20=K32EnumProcessModulesEx @6",
    ),
    ("EnumProcesses", "  EnumProcesses@12=K32EnumProcesses @7"),
    (
        "GetDeviceDriverBaseNameA",
        "  GetDeviceDriverBaseNameA@12=K32GetDeviceDriverBaseNameA @8",
    ),
    (
        "GetDeviceDriverBaseNameW",
        "  GetDeviceDriverBaseNameW@12=K32GetDeviceDriverBaseNameW @9",
    ),
    (
        "GetDeviceDriverFileNameA",
        "  GetDeviceDriverFileNameA@12=K32GetDeviceDriverFileNameA @10",
    ),
    (
        "GetDeviceDriverFileNameW",
        "  GetDeviceDriverFileNameW@12=K32GetDeviceDriverFileNameW @11",
    ),
    (
        "GetMappedFileNameA",
        "  GetMappedFileNameA@16=K32GetMappedFileNameA @12",
    ),
    (
        "GetMappedFileNameW",
        "  GetMappedFileNameW@16=K32GetMappedFileNameW @13",
    ),
    (
        "GetModuleBaseNameA",
        "  GetModuleBaseNameA@16=K32GetModuleBaseNameA @14",
    ),
    (
        "GetModuleBaseNameW",
        "  GetModuleBaseNameW@16=K32GetModuleBaseNameW @15",
    ),
    (
        "GetModuleFileNameExA",
        "  GetModuleFileNameExA@16=K32GetModuleFileNameExA @16",
    ),
    (
        "GetModuleFileNameExW",
        "  GetModuleFileNameExW@16=K32GetModuleFileNameExW @17",
    ),
    (
        "GetModuleInformation",
        "  GetModuleInformation@16=K32GetModuleInformation @18",
    ),
    (
        "GetPerformanceInfo",
        "  GetPerformanceInfo@8=K32GetPerformanceInfo @19",
    ),
    (
        "GetProcessImageFileNameA",
        "  GetProcessImageFileNameA@12=K32GetProcessImageFileNameA @20",
    ),
    (
        "GetProcessImageFileNameW",
        "  GetProcessImageFileNameW@12=K32GetProcessImageFileNameW @21",
    ),
    (
        "GetProcessMemoryInfo",
        "  GetProcessMemoryInfo@12=K32GetProcessMemoryInfo @22",
    ),
    ("GetWsChanges", "  GetWsChanges@12=K32GetWsChanges @23"),
    (
        "GetWsChangesEx",
        "  GetWsChangesEx@12=K32GetWsChangesEx @24",
    ),
    (
        "InitializeProcessForWsWatch",
        "  InitializeProcessForWsWatch@4=K32InitializeProcessForWsWatch @25",
    ),
    (
        "QueryWorkingSet",
        "  QueryWorkingSet@12=K32QueryWorkingSet @26",
    ),
    (
        "QueryWorkingSetEx",
        "  QueryWorkingSetEx@12=K32QueryWorkingSetEx @27",
    ),
];

#[test]
fn oracle_library_row() {
    // row 01 | LIBRARY psapi.dll
    assert_eq!(def_body().lines().next(), Some("LIBRARY psapi.dll"));
}

#[test]
fn oracle_export_rows() {
    // rows 02 to 28: the export lines, in spec order, after the three header lines.
    let body = def_body();
    let lines: Vec<&str> = body.lines().skip(3).collect();
    assert_eq!(lines.len(), EXPORT_ROWS.len());
    for (i, (name, line)) in EXPORT_ROWS.iter().enumerate() {
        assert_eq!(EXPORTS[i].name, *name);
        assert_eq!(lines[i], *line);
        assert_eq!(EXPORTS[i].def_line(), *line);
    }
}

#[test]
fn oracle_probe_row() {
    // row 29 | Same@4 @1: an export whose name equals its link name has no `=` part.
    let probe = Export {
        name: "Same",
        ordinal: 1,
        args: &[ArgKind::Long],
        link_name: "Same",
    };
    assert_eq!(probe.def_line(), "  Same@4 @1");
}
