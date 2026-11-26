//! Boundary and branch tests for the port. An expectation that is not derived from an oracle row
//! is marked "Not oracle-derived".

use super::*;

use crate::oracle_checks::{driver_reply, u32_at, w, FakeOs};

#[test]
fn request_size_stops_at_32_bits() {
    // Not oracle-derived: the largest query size that still fits a DWORD.
    assert_eq!(request_size(0, 0), Some(24));
    assert_eq!(request_size(2, 100), Some(148));
    assert_eq!(request_size(357_913_939, 3), Some(u32::MAX));
    assert_eq!(request_size(357_913_939, 4), None);
    assert_eq!(request_size(1, u32::MAX), None);
}

#[test]
fn more_data_buflen_wraps_like_the_c_arithmetic() {
    // Not oracle-derived: `size - (24 + 12 * count)` in 32-bit arithmetic.
    assert_eq!(more_data_buflen(48, 2), 0);
    assert_eq!(more_data_buflen(47, 2), u32::MAX);
}

#[test]
fn fill_request_places_count_name_and_ids() {
    // Not oracle-derived: field offsets of the query header and entries.
    let mut query = vec![0u8; 60];
    fill_request(&mut query, b"eth", &[7, 8]);
    assert_eq!(u32_at(&query, 4), 2);
    assert_eq!(&query[8..12], b"eth\0");
    assert_eq!(u32_at(&query, 24), 7);
    assert_eq!(u32_at(&query, 36), 8);
}

#[test]
fn collect_reply_rejects_a_short_or_overcounted_reply() {
    // Not oracle-derived: the malformed-reply deviation in lib.rs.
    assert_eq!(collect_reply(&[0u8; 47], 2, None), Err(ERROR_INVALID_DATA));
    let overcounted = driver_reply(148, 3, &[], &[]);
    assert_eq!(
        collect_reply(&overcounted, 2, None),
        Err(ERROR_INVALID_DATA)
    );
}

#[test]
fn collect_reply_rejects_data_outside_the_reply() {
    // Not oracle-derived: the malformed-reply deviation in lib.rs.
    let reply = driver_reply(148, 1, &[(5, 140, 20)], &[]);
    let mut buf = [0u8; 32];
    assert_eq!(
        collect_reply(&reply, 1, Some(&mut buf[..])),
        Err(ERROR_INVALID_DATA)
    );
}

#[test]
fn collect_reply_gives_an_empty_entry_no_data() {
    // Not oracle-derived: an empty entry has no data, whatever its offset.
    let reply = driver_reply(148, 1, &[(5, 9999, 0)], &[]);
    let mut buf = [0u8; 32];
    let expected = Reply {
        params: vec![ReceivedParam {
            option_id: 5,
            is_vendor: false,
            data: None,
        }],
        buflen: 0,
    };
    assert_eq!(collect_reply(&reply, 1, Some(&mut buf[..])), Ok(expected));
}

#[test]
fn collect_reply_without_a_buffer_reports_only_the_total() {
    // Not oracle-derived: a NULL buffer skips the data, so an offset past the reply is not read.
    let reply = driver_reply(148, 1, &[(5, 9999, 3)], &[]);
    assert_eq!(
        collect_reply(&reply, 1, None),
        Ok(Reply {
            params: Vec::new(),
            buflen: 3,
        })
    );
}

#[test]
fn collect_reply_rejects_data_larger_than_the_buffer() {
    // Not oracle-derived: the insufficient-buffer deviation in lib.rs.
    let reply = driver_reply(148, 1, &[(5, 48, 3)], &[(48, &b"ABC"[..])]);
    let mut small = [0u8; 2];
    assert_eq!(
        collect_reply(&reply, 1, Some(&mut small[..])),
        Err(ERROR_INSUFFICIENT_BUFFER)
    );
}

#[test]
fn collect_reply_rejects_a_total_above_32_bits() {
    // Not oracle-derived: the total must fit the DWORD that `*buflen` receives.
    let recv_count = 65_537;
    let mut reply = vec![0u8; params_offset(recv_count) as usize];
    reply[4..8].copy_from_slice(&65_536u32.to_le_bytes());
    for i in 0..65_536 {
        let at = params_offset(i) as usize;
        reply[at + 8..at + 12].copy_from_slice(&65_536u32.to_le_bytes());
    }
    assert_eq!(
        collect_reply(&reply, recv_count, None),
        Err(ERROR_INVALID_DATA)
    );
}

#[test]
fn request_params_without_adapter_or_buflen_is_invalid() {
    // Not oracle-derived: the NULL checks come before any host call.
    let eth = w("eth");
    let mut os = FakeOs::default();
    let mut buflen = 0;
    assert_eq!(
        request_params(&mut os, None, &[], None, Some(&mut buflen)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(
        request_params(&mut os, Some(eth.as_slice()), &[], None, None),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!(os.opens, 0);
}

#[test]
fn request_params_rejects_a_query_size_above_32_bits() {
    // Not oracle-derived: the query-size deviation, checked after the handle is open.
    let eth = w("eth");
    let mut os = FakeOs::default();
    let mut buflen = u32::MAX;
    assert_eq!(
        request_params(&mut os, Some(eth.as_slice()), &[], None, Some(&mut buflen)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!((os.opens, os.closes), (1, 1));
    assert!(os.inputs.is_empty());
    assert_eq!(buflen, u32::MAX);
}

#[test]
fn request_params_keeps_buflen_when_the_reply_is_malformed() {
    // Not oracle-derived: `*buflen` changes only on success or ERROR_MORE_DATA.
    let eth = w("eth");
    let mut os = FakeOs {
        image: driver_reply(148, 3, &[], &[]),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    let mut buf = [0u8; 100];
    assert_eq!(
        request_params(
            &mut os,
            Some(eth.as_slice()),
            &[5],
            Some(&mut buf[..]),
            Some(&mut buflen)
        ),
        Err(ERROR_INVALID_DATA)
    );
    assert_eq!((buflen, os.closes), (100, 1));
}

#[test]
fn request_params_keeps_buflen_when_the_buffer_is_too_small() {
    // Not oracle-derived: the insufficient-buffer deviation leaves `*buflen` unchanged.
    let eth = w("eth");
    let image = driver_reply(148, 1, &[(5, 48, 3)], &[(48, &b"ABC"[..])]);
    let mut os = FakeOs {
        image,
        ..FakeOs::default()
    };
    let mut buflen = 100;
    let mut small = [0u8; 2];
    assert_eq!(
        request_params(
            &mut os,
            Some(eth.as_slice()),
            &[5],
            Some(&mut small[..]),
            Some(&mut buflen)
        ),
        Err(ERROR_INSUFFICIENT_BUFFER)
    );
    assert_eq!(buflen, 100);
}

#[test]
fn get_adapter_luid_reads_the_name_up_to_its_nul() {
    // Not oracle-derived: the name ends at its first NUL.
    let mut os = FakeOs::default();
    assert_eq!(get_adapter_luid(&mut os, &w("eth\0x")), Ok(0));
    assert_eq!(os.names, vec![w("eth")]);
}

#[test]
fn get_adapter_luid_takes_a_braced_name_as_a_guid_when_it_parses() {
    // Not oracle-derived: the `{` branch, with and without a parsed GUID.
    let braced = w("{x}");
    let mut parsed = FakeOs {
        guid_ok: true,
        guid_luid: Ok(7),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut parsed, &braced), Ok(7));
    assert!(parsed.names.is_empty());
    let mut unparsed = FakeOs::default();
    assert_eq!(get_adapter_luid(&mut unparsed, &braced), Ok(0));
    assert_eq!(unparsed.names, vec![braced]);
}

#[test]
fn get_adapter_name_reads_at_most_15_alias_units() {
    // Not oracle-derived: the alias is read through IF_NAMESIZE units, one of them the NUL.
    let mut os = FakeOs {
        alias: Ok(w(&"a".repeat(20))),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_name(&mut os, &w("eth")), Ok(vec![b'a'; 15]));
    let mut os = FakeOs {
        alias: Ok(w("ab\0cd")),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_name(&mut os, &w("eth")), Ok(b"ab".to_vec()));
}

#[test]
fn every_stub_export_raises_the_stub_fault() {
    // Not oracle-derived: the oracle row covers one export; this checks all 42 names.
    assert_eq!(STUB_EXPORTS.len(), 42);
    for &name in STUB_EXPORTS {
        assert_eq!(
            call_stub_export(name),
            Some(unimplemented_stub(MODULE_NAME, name))
        );
    }
}

#[test]
fn implemented_exports_and_unknown_names_are_not_stubs() {
    // Not oracle-derived: name lookup for the implemented exports and for an unknown name.
    for name in [
        "DhcpCApiInitialize",
        "DhcpCApiCleanup",
        "DhcpRequestParams",
        "NoSuchExport",
    ] {
        assert_eq!(call_stub_export(name), None);
    }
}
