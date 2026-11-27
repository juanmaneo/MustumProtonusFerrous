//! Oracle-derived test vectors for the ported srvcli export table.
//!
//! Vectors come from `target/canon/srvcli`, run by `run_oracle.sh`. The 52 stub rows are the
//! `__wine_spec_unimplemented_stub` records that the harness prints. The harness compiles the
//! verbatim Wine body from `dlls/ntdll/exception.c` with `gcc -fsanitize=address,undefined -g`
//! and replaces `RtlRaiseException` with a recorder. The 9 forward rows are the targets that the
//! reference `winebuild` writes, because the harness runs only the stubs. The 61 rows cover every
//! export in `dlls/srvcli/srvcli.spec`. ASan and UBSan flagged no row, so none is excluded.

use super::*;

/// `(ordinal, name, code, flags, number_parameters, module)` for each `@ stub` export. `name` is
/// also the function parameter that `winebuild` passes to the stub.
const STUB_ROWS: [(u16, &str, u32, u32, u32, &str); 52] = [
    (1, "I_NetDfsGetVersion", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        2,
        "I_NetServerSetServiceBits",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        3,
        "I_NetServerSetServiceBitsEx",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (4, "LocalAliasGet", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (5, "LocalFileClose", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (6, "LocalFileEnum", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (7, "LocalFileEnumEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (8, "LocalFileGetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (9, "LocalFileGetInfoEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        10,
        "LocalServerCertificateMappingAdd",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        11,
        "LocalServerCertificateMappingEnum",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        12,
        "LocalServerCertificateMappingGet",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        13,
        "LocalServerCertificateMappingRemove",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (14, "LocalSessionDel", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (15, "LocalSessionEnum", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (16, "LocalSessionEnumEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (17, "LocalSessionGetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        18,
        "LocalSessionGetInfoEx",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (19, "LocalShareAdd", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (20, "LocalShareDelEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (21, "LocalShareEnum", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (22, "LocalShareEnumEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (23, "LocalShareGetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (24, "LocalShareGetInfoEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (25, "LocalShareSetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (26, "NetConnectionEnum", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (27, "NetFileClose", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (29, "NetFileGetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (31, "NetServerAliasAdd", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (32, "NetServerAliasDel", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (33, "NetServerAliasEnum", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        34,
        "NetServerComputerNameAdd",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        35,
        "NetServerComputerNameDel",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (38, "NetServerSetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        39,
        "NetServerStatisticsGet",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        40,
        "NetServerTransportAdd",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        41,
        "NetServerTransportAddEx",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        42,
        "NetServerTransportDel",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (
        43,
        "NetServerTransportEnum",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (44, "NetSessionDel", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (46, "NetSessionGetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (48, "NetShareCheck", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (50, "NetShareDelEx", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (51, "NetShareDelSticky", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (53, "NetShareEnumSticky", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (55, "NetShareSetInfo", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        56,
        "NetpsNameCanonicalize",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (57, "NetpsNameCompare", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (58, "NetpsNameValidate", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (
        59,
        "NetpsPathCanonicalize",
        0x8000_0100,
        0x1,
        2,
        "srvcli.dll",
    ),
    (60, "NetpsPathCompare", 0x8000_0100, 0x1, 2, "srvcli.dll"),
    (61, "NetpsPathType", 0x8000_0100, 0x1, 2, "srvcli.dll"),
];

/// `(ordinal, name, module, function)` for each forward, from the `winebuild` output.
const FORWARD_ROWS: [(u16, &str, &str, &str); 9] = [
    (28, "NetFileEnum", "netapi32", "NetFileEnum"),
    (30, "NetRemoteTOD", "netapi32", "NetRemoteTOD"),
    (36, "NetServerDiskEnum", "netapi32", "NetServerDiskEnum"),
    (37, "NetServerGetInfo", "netapi32", "NetServerGetInfo"),
    (45, "NetSessionEnum", "netapi32", "NetSessionEnum"),
    (47, "NetShareAdd", "netapi32", "NetShareAdd"),
    (49, "NetShareDel", "netapi32", "NetShareDel"),
    (52, "NetShareEnum", "netapi32", "NetShareEnum"),
    (54, "NetShareGetInfo", "netapi32", "NetShareGetInfo"),
];

#[test]
fn oracle_stub_rows() {
    let helpers: [fn() -> StubException; 52] = [
        i_net_dfs_get_version,
        i_net_server_set_service_bits,
        i_net_server_set_service_bits_ex,
        local_alias_get,
        local_file_close,
        local_file_enum,
        local_file_enum_ex,
        local_file_get_info,
        local_file_get_info_ex,
        local_server_certificate_mapping_add,
        local_server_certificate_mapping_enum,
        local_server_certificate_mapping_get,
        local_server_certificate_mapping_remove,
        local_session_del,
        local_session_enum,
        local_session_enum_ex,
        local_session_get_info,
        local_session_get_info_ex,
        local_share_add,
        local_share_del_ex,
        local_share_enum,
        local_share_enum_ex,
        local_share_get_info,
        local_share_get_info_ex,
        local_share_set_info,
        net_connection_enum,
        net_file_close,
        net_file_get_info,
        net_server_alias_add,
        net_server_alias_del,
        net_server_alias_enum,
        net_server_computer_name_add,
        net_server_computer_name_del,
        net_server_set_info,
        net_server_statistics_get,
        net_server_transport_add,
        net_server_transport_add_ex,
        net_server_transport_del,
        net_server_transport_enum,
        net_session_del,
        net_session_get_info,
        net_share_check,
        net_share_del_ex,
        net_share_del_sticky,
        net_share_enum_sticky,
        net_share_set_info,
        netps_name_canonicalize,
        netps_name_compare,
        netps_name_validate,
        netps_path_canonicalize,
        netps_path_compare,
        netps_path_type,
    ];
    for ((ordinal, name, code, flags, number_parameters, module), helper) in
        STUB_ROWS.into_iter().zip(helpers)
    {
        let export = export_by_name(name).expect("stub row names a spec export");
        assert_eq!(export.ordinal, ordinal);
        assert_eq!(export.kind, ExportKind::Stub);
        assert_eq!(
            helper(),
            StubException {
                code,
                flags,
                number_parameters,
                module,
                function: name,
            }
        );
    }
}

#[test]
fn oracle_forward_rows() {
    for (ordinal, name, module, function) in FORWARD_ROWS {
        let export = export_by_name(name).expect("forward row names a spec export");
        assert_eq!(export.ordinal, ordinal);
        assert_eq!(export.kind, ExportKind::Forward { module, function });
        assert_eq!(forward_target(name), Some((module, function)));
    }
}

#[test]
fn oracle_export_counts() {
    assert_eq!(EXPORTS.len(), 61);
    assert_eq!(STUB_ROWS.len() + FORWARD_ROWS.len(), EXPORTS.len());
    let stubs = EXPORTS
        .iter()
        .filter(|export| export.kind == ExportKind::Stub)
        .count();
    let forwards = EXPORTS
        .iter()
        .filter(|export| matches!(export.kind, ExportKind::Forward { .. }))
        .count();
    assert_eq!(stubs, STUB_ROWS.len());
    assert_eq!(forwards, FORWARD_ROWS.len());
}
