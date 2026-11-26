//! Oracle-derived test vectors for the ported dhcpcsvc logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/dhcpcsvc/dhcpcsvc.c`
//! (lines 38-151) and the stub helper from `dlls/ntdll/exception.c` (lines 504-516), with minimal
//! Windows type shims and fake host calls, using `gcc -fsanitize=address,undefined -g` under
//! `target/canon/dhcpcsvc/`. The harness printed 25 rows. ASan and UBSan reported nothing, so no
//! row is excluded.
//!
//! Each test quotes the rows it checks. Where a row prints a value that the Rust API does not
//! return (`luid=0x0` on an error, or the option array that a NULL buffer leaves untouched), the
//! assertion checks the closest result the API does return.

use super::*;

/// `ERROR_NOT_FOUND`, the status the harness's fake name lookups return.
pub(crate) const ERROR_NOT_FOUND: u32 = 1168;

/// Seven `é` (two UTF-8 bytes each) and `a`: the alias of row 11, 15 bytes once converted.
const FIT: &str = "\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}a";

/// The host calls, with the defaults of the harness's `reset()`: alias `eth0`, and every call succeeds.
pub(crate) struct FakeOs {
    pub guid_ok: bool,
    pub guid_luid: Result<Luid, u32>,
    pub name_luid: Result<Luid, u32>,
    pub alias: Result<Vec<u16>, u32>,
    pub wide_ok: bool,
    pub open_result: Result<(), u32>,
    pub calloc_ok: bool,
    pub ioctl_result: Result<(), u32>,
    /// Bytes the driver writes into the query buffer, up to the buffer's length.
    pub image: Vec<u8>,
    pub opens: u32,
    pub closes: u32,
    /// The `(code, buffer)` of each `DeviceIoControl`, with the buffer as the driver received it.
    pub inputs: Vec<(u32, Vec<u8>)>,
    /// The adapter names passed to `ConvertInterfaceNameToLuidW`.
    pub names: Vec<Vec<u16>>,
}

impl Default for FakeOs {
    fn default() -> Self {
        FakeOs {
            guid_ok: false,
            guid_luid: Ok(0),
            name_luid: Ok(0),
            alias: Ok(w("eth0")),
            wide_ok: true,
            open_result: Ok(()),
            calloc_ok: true,
            ioctl_result: Ok(()),
            image: Vec::new(),
            opens: 0,
            closes: 0,
            inputs: Vec::new(),
            names: Vec::new(),
        }
    }
}

impl Os for FakeOs {
    fn guid_from_string(&mut self, _s: &[u16]) -> Option<Guid> {
        self.guid_ok.then_some([0; 16])
    }

    fn guid_to_luid(&mut self, _guid: &Guid) -> Result<Luid, u32> {
        self.guid_luid
    }

    fn name_to_luid(&mut self, name: &[u16]) -> Result<Luid, u32> {
        self.names.push(name.to_vec());
        self.name_luid
    }

    fn luid_to_alias(&mut self, _luid: Luid) -> Result<Vec<u16>, u32> {
        self.alias.clone()
    }

    fn wide_to_unix(&mut self, s: &[u16]) -> Option<Vec<u8>> {
        if self.wide_ok {
            String::from_utf16(s).ok().map(String::into_bytes)
        } else {
            None
        }
    }
}

impl MountMgr for FakeOs {
    fn open(&mut self) -> Result<(), u32> {
        self.opens += 1;
        self.open_result
    }

    fn close(&mut self) {
        self.closes += 1;
    }

    fn alloc_zeroed(&mut self, len: usize) -> Option<Vec<u8>> {
        self.calloc_ok.then(|| vec![0; len])
    }

    fn ioctl(&mut self, code: u32, buf: &mut [u8]) -> Result<(), u32> {
        self.inputs.push((code, buf.to_vec()));
        let n = buf.len().min(self.image.len());
        buf[..n].copy_from_slice(&self.image[..n]);
        self.ioctl_result
    }
}

/// The UTF-16 units of `s`.
pub(crate) fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// A little-endian `u32` at byte offset `at`.
pub(crate) fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
}

/// The harness's 148-byte driver reply: `size` and `count`, then `(id, offset, size)` entries at
/// `24 + 12 * i`, then the data blobs at their offsets.
pub(crate) fn driver_reply(
    size: u32,
    count: u32,
    entries: &[(u32, u32, u32)],
    data: &[(usize, &[u8])],
) -> Vec<u8> {
    let mut image = vec![0; 148];
    image[..4].copy_from_slice(&size.to_le_bytes());
    image[4..8].copy_from_slice(&count.to_le_bytes());
    for (i, &(id, offset, len)) in entries.iter().enumerate() {
        let at = 24 + 12 * i;
        image[at..at + 4].copy_from_slice(&id.to_le_bytes());
        image[at + 4..at + 8].copy_from_slice(&offset.to_le_bytes());
        image[at + 8..at + 12].copy_from_slice(&len.to_le_bytes());
    }
    for &(at, bytes) in data {
        image[at..at + bytes.len()].copy_from_slice(bytes);
    }
    image
}

#[test]
fn oracle_api_rows() {
    // Row 1: init_version status=0 version=2
    let mut version = 0xdead_beef;
    assert_eq!(dhcp_c_api_initialize(&mut version), ERROR_SUCCESS);
    assert_eq!(version, 2);
    // Row 2: cleanup returned
    dhcp_c_api_cleanup();
}

#[test]
fn oracle_luid_rows() {
    // Row 3: luid_guid status=0 luid=0x1234
    let mut os = FakeOs {
        guid_ok: true,
        guid_luid: Ok(0x1234),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut os, &w("{0000}")), Ok(0x1234));
    assert!(os.names.is_empty());

    // Row 4: luid_guid_err status=5 luid=0x0
    let mut os = FakeOs {
        guid_ok: true,
        guid_luid: Err(5),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut os, &w("{0000}")), Err(5));

    // Row 5: luid_guid_fallthrough status=0 luid=0x77
    let mut os = FakeOs {
        name_luid: Ok(0x77),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut os, &w("{bad")), Ok(0x77));
    assert_eq!(os.names, vec![w("{bad")]);

    // Row 6: luid_name status=0 luid=0x99
    let mut os = FakeOs {
        name_luid: Ok(0x99),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut os, &w("eth0")), Ok(0x99));

    // Row 7: luid_name_err status=1168 luid=0x0
    let mut os = FakeOs {
        name_luid: Err(ERROR_NOT_FOUND),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut os, &w("eth0")), Err(ERROR_NOT_FOUND));

    // Row 8: luid_empty status=1168 luid=0x0
    let mut os = FakeOs {
        guid_ok: true,
        guid_luid: Ok(0x11),
        name_luid: Err(ERROR_NOT_FOUND),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_luid(&mut os, &w("")), Err(ERROR_NOT_FOUND));
    assert_eq!(os.names, vec![Vec::<u16>::new()]);
}

#[test]
fn oracle_name_rows() {
    // Row 9: name_ok status=0 name=eth0
    assert_eq!(
        get_adapter_name(&mut FakeOs::default(), &w("eth0")),
        Ok(b"eth0".to_vec())
    );

    // Row 10: name_alias_err status=1168 name=
    let mut os = FakeOs {
        alias: Err(ERROR_NOT_FOUND),
        ..FakeOs::default()
    };
    assert_eq!(get_adapter_name(&mut os, &w("eth0")), Err(ERROR_NOT_FOUND));

    // Row 11: name_utf8_fit status=0 bytes=c3a9c3a9c3a9c3a9c3a9c3a9c3a96100
    // The row's bytes are seven c3a9 pairs, then 61, then the NUL. The port returns them without the NUL.
    let mut os = FakeOs {
        alias: Ok(w(FIT)),
        ..FakeOs::default()
    };
    assert_eq!(
        get_adapter_name(&mut os, &w("eth0")),
        Ok(FIT.as_bytes().to_vec())
    );

    // Row 12: name_utf8_too_long status=87
    let mut os = FakeOs {
        alias: Ok(w(&"\u{e9}".repeat(8))),
        ..FakeOs::default()
    };
    assert_eq!(
        get_adapter_name(&mut os, &w("eth0")),
        Err(ERROR_INVALID_PARAMETER)
    );

    // Row 13: name_wc_fail status=87
    let mut os = FakeOs {
        wide_ok: false,
        ..FakeOs::default()
    };
    assert_eq!(
        get_adapter_name(&mut os, &w("eth0")),
        Err(ERROR_INVALID_PARAMETER)
    );
}

#[test]
fn oracle_request_error_rows() {
    let eth0 = w("eth0");

    // Row 14: args_adapter_null status=87 opens=0 buflen=100
    let mut os = FakeOs::default();
    let mut buflen = 100;
    assert_eq!(
        request_params(&mut os, None, &[5, 9], None, Some(&mut buflen)),
        Err(ERROR_INVALID_PARAMETER)
    );
    assert_eq!((os.opens, buflen), (0, 100));

    // Row 15: req_name_fail status=1168 opens=0 buflen=100
    let mut os = FakeOs {
        name_luid: Err(ERROR_NOT_FOUND),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Err(ERROR_NOT_FOUND)
    );
    assert_eq!((os.opens, buflen), (0, 100));

    // Row 16: req_open_fail status=5 opens=1 closes=0 ioctls=0 buflen=100
    let mut os = FakeOs {
        open_result: Err(5),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Err(5)
    );
    assert_eq!(
        (os.opens, os.closes, os.inputs.len(), buflen),
        (1, 0, 0, 100)
    );

    // Row 17: req_oom status=14 opens=1 closes=1 ioctls=0 buflen=100
    let mut os = FakeOs {
        calloc_ok: false,
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Err(ERROR_OUTOFMEMORY)
    );
    assert_eq!(
        (os.opens, os.closes, os.inputs.len(), buflen),
        (1, 1, 0, 100)
    );
}

#[test]
fn oracle_request_layout_row() {
    let eth0 = w("eth0");

    // Row 18: req_request_layout status=0 buflen=0 in_size=148 code=0x006dc100 size=0 count=2
    //   name=eth0 id0=5 id1=9 opens=1 closes=1
    // The harness's reply is empty, so buflen comes back as 0 and no option is reported.
    let mut os = FakeOs::default();
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Ok(Vec::new())
    );
    assert_eq!(buflen, 0);
    assert_eq!(os.inputs.len(), 1);
    let (code, query) = &os.inputs[0];
    assert_eq!(*code, 0x006d_c100);
    assert_eq!(query.len(), 148);
    assert_eq!(query_size(query), 0);
    assert_eq!(u32_at(query, 4), 2);
    assert_eq!(&query[8..12], b"eth0");
    assert_eq!(u32_at(query, 24), 5);
    assert_eq!(u32_at(query, 36), 9);
    assert_eq!((os.opens, os.closes), (1, 1));
}

#[test]
fn oracle_request_ioctl_error_rows() {
    let eth0 = w("eth0");

    // Row 19: req_ioctl_fail status=1 buflen=100 opens=1 closes=1 ioctls=1
    let mut os = FakeOs {
        ioctl_result: Err(1),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Err(1)
    );
    assert_eq!(
        (os.opens, os.closes, os.inputs.len(), buflen),
        (1, 1, 1, 100)
    );

    // Row 20: req_more_data status=234 buflen=152 closes=1
    let mut os = FakeOs {
        ioctl_result: Err(ERROR_MORE_DATA),
        image: driver_reply(200, 2, &[], &[]),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Err(ERROR_MORE_DATA)
    );
    assert_eq!((buflen, os.closes), (152, 1));

    // Row 21: req_more_data_wrap status=234 buflen=4294967258
    let mut os = FakeOs {
        ioctl_result: Err(ERROR_MORE_DATA),
        image: driver_reply(10, 2, &[], &[]),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Err(ERROR_MORE_DATA)
    );
    assert_eq!(buflen, 4_294_967_258);
}

#[test]
fn oracle_request_success_rows() {
    let eth0 = w("eth0");
    let reply = driver_reply(148, 2, &[(5, 48, 3), (9, 0, 0)], &[(48, &b"ABC"[..])]);

    // Row 22: req_success_nobuf status=0 buflen=3 p0=(id=5 vendor=0 nbytes=0 data=null)
    //   p1=(id=9 vendor=0 nbytes=0 data=null)
    // With a NULL buffer the C code leaves the caller's option array alone, so none is returned.
    let mut os = FakeOs {
        image: reply.clone(),
        ..FakeOs::default()
    };
    let mut buflen = 100;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            None,
            Some(&mut buflen)
        ),
        Ok(Vec::new())
    );
    assert_eq!(buflen, 3);

    // Row 23: req_success_buf status=0 buflen=3 buf=ABC p0=(id=5 vendor=0 nbytes=3 data=ABC)
    //   p1=(id=9 vendor=0 nbytes=0 data=null)
    let mut os = FakeOs {
        image: reply,
        ..FakeOs::default()
    };
    let mut buf = [0u8; 16];
    let mut buflen = 16;
    assert_eq!(
        request_params(
            &mut os,
            Some(eth0.as_slice()),
            &[5, 9],
            Some(&mut buf[..]),
            Some(&mut buflen)
        ),
        Ok(vec![
            ReceivedParam {
                option_id: 5,
                is_vendor: false,
                data: Some(0..3),
            },
            ReceivedParam {
                option_id: 9,
                is_vendor: false,
                data: None,
            },
        ])
    );
    assert_eq!(buflen, 3);
    assert_eq!(&buf[..3], b"ABC");
}

#[test]
fn oracle_stub_row() {
    // Row 24: stub_dhcp_free_mem code=0x80000100 flags=1 nparams=2 module=dhcpcsvc.dll
    //   function=DhcpFreeMem addr_is_stub=1 nested_null=1
    // The harness reads `ExceptionAddress` and `ExceptionRecord`, which the port does not model.
    assert_eq!(
        call_stub_export("DhcpFreeMem"),
        Some(StubFault {
            code: 0x8000_0100,
            flags: 1,
            number_parameters: 2,
            module: "dhcpcsvc.dll",
            function: "DhcpFreeMem",
        })
    );
}

#[test]
fn oracle_ioctl_code_row() {
    // Row 25: ioctl_code value=0x006dc100
    assert_eq!(query_dhcp_request_params_ioctl(), 0x006d_c100);
}
