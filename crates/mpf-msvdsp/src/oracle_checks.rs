//! Oracle-derived test vectors for the ported msvdsp logic.
//!
//! The vectors were captured under `target/canon/msvdsp/` by compiling the verbatim Wine C bodies
//! of `DllRegisterServer`, `DllUnregisterServer` and `DllGetClassObject` (`dlls/msvdsp/msvdsp.c`)
//! and `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`) with
//! `gcc -fsanitize=address,undefined -g`, and running the harness. The harness supplies the OS
//! entry points: each one appends its arguments to a byte record and returns the HRESULT injected
//! for that call index. The 21 printed rows (R01-R09, U01-U07, G01-G04 and C01) are all kept:
//! neither sanitizer flagged a row, so none is excluded.

use super::*;

/// The tag byte the harness writes first for each OS call.
const TAG_MFT_REGISTER: u8 = 1;
const TAG_DMO_REGISTER: u8 = 2;
const TAG_MFT_UNREGISTER: u8 = 3;
const TAG_DMO_UNREGISTER: u8 = 4;
const TAG_RESOURCES: u8 = 5;
const TAG_UNRESOURCES: u8 = 6;

/// `guid_zero` in the harness.
const ZERO_GUID: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// `guid_ones` in the harness.
const ONES_GUID: Guid = Guid {
    data1: 0xffff_ffff,
    data2: 0xffff,
    data3: 0xffff,
    data4: [0xff; 8],
};

/// `IID_IClassFactory`, a sample riid that the C body ignores.
const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// Records the OS calls the way the C harness does. `bytes` is the byte record and `trace` the
/// call names. The call numbered `fail_at` (1-based, 0 for never) returns `fail`; the others
/// return `ok`.
struct Recorder {
    bytes: Vec<u8>,
    trace: Vec<&'static str>,
    ok: HResult,
    fail_at: usize,
    fail: HResult,
}

impl Recorder {
    fn new(ok: HResult, fail_at: usize, fail: HResult) -> Self {
        Self {
            bytes: Vec::new(),
            trace: Vec::new(),
            ok,
            fail_at,
            fail,
        }
    }

    fn call(&mut self, tag: u8, name: &'static str) -> HResult {
        self.put_u8(tag);
        self.trace.push(name);
        if self.trace.len() == self.fail_at {
            self.fail
        } else {
            self.ok
        }
    }

    fn put_u8(&mut self, v: u8) {
        self.bytes.push(v);
    }

    fn put_u16(&mut self, v: u16) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }

    fn put_u32(&mut self, v: u32) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }

    fn put_guid(&mut self, g: &Guid) {
        self.put_u32(g.data1);
        self.put_u16(g.data2);
        self.put_u16(g.data3);
        self.bytes.extend_from_slice(&g.data4);
    }

    fn put_name(&mut self, name: &str) {
        let units: Vec<u16> = name.encode_utf16().collect();
        self.put_u32(units.len() as u32);
        for unit in units {
            self.put_u16(unit);
        }
    }

    fn put_mft_types(&mut self, types: &[MftRegisterTypeInfo]) {
        self.put_u32(types.len() as u32);
        for t in types {
            self.put_guid(&t.major_type);
            self.put_guid(&t.subtype);
        }
    }

    fn put_dmo_types(&mut self, types: &[DmoPartialMediaType]) {
        self.put_u32(types.len() as u32);
        for t in types {
            self.put_guid(&t.major_type);
            self.put_guid(&t.subtype);
        }
    }

    fn trace(&self) -> String {
        self.trace.join(",")
    }

    fn hash(&self) -> u64 {
        fnv1a(&self.bytes)
    }
}

impl Registrar for Recorder {
    fn register_resources(&mut self) -> HResult {
        self.call(TAG_RESOURCES, "res")
    }

    fn unregister_resources(&mut self) -> HResult {
        self.call(TAG_UNRESOURCES, "unres")
    }

    fn mft_register(
        &mut self,
        clsid: &Guid,
        category: &Guid,
        name: &str,
        flags: u32,
        inputs: &[MftRegisterTypeInfo],
        outputs: &[MftRegisterTypeInfo],
    ) -> HResult {
        let hr = self.call(TAG_MFT_REGISTER, "mft+");
        self.put_guid(clsid);
        self.put_guid(category);
        self.put_name(name);
        self.put_u32(flags);
        self.put_mft_types(inputs);
        self.put_mft_types(outputs);
        // The attributes argument is NULL in msvdsp.c; the harness records it as one byte.
        self.put_u8(0);
        hr
    }

    fn mft_unregister(&mut self, clsid: &Guid) -> HResult {
        let hr = self.call(TAG_MFT_UNREGISTER, "mft-");
        self.put_guid(clsid);
        hr
    }

    fn dmo_register(
        &mut self,
        name: &str,
        clsid: &Guid,
        category: &Guid,
        flags: u32,
        inputs: &[DmoPartialMediaType],
        outputs: &[DmoPartialMediaType],
    ) -> HResult {
        let hr = self.call(TAG_DMO_REGISTER, "dmo+");
        self.put_name(name);
        self.put_guid(clsid);
        self.put_guid(category);
        self.put_u32(flags);
        self.put_dmo_types(inputs);
        self.put_dmo_types(outputs);
        hr
    }

    fn dmo_unregister(&mut self, clsid: &Guid, category: &Guid) -> HResult {
        let hr = self.call(TAG_DMO_UNREGISTER, "dmo-");
        self.put_guid(clsid);
        self.put_guid(category);
        hr
    }
}

/// FNV-1a, 64-bit: the hash the harness prints over the byte record.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// One `DllRegisterServer` or `DllUnregisterServer` row: the injected results, the HRESULT
/// returned, the call trace, and the hash of the byte record.
struct ServerRow {
    id: &'static str,
    ok: HResult,
    fail_at: usize,
    fail: HResult,
    hr: HResult,
    trace: &'static str,
    hash: u64,
}

/// One `DllGetClassObject` row.
struct ClassRow {
    id: &'static str,
    clsid: Guid,
    riid: Guid,
    hr: HResult,
}

#[rustfmt::skip]
const REGISTER_ROWS: [ServerRow; 9] = [
    ServerRow { id: "R01", ok: 0x0000_0000, fail_at: 0, fail: 0x0000_0000, hr: 0x0000_0000,
        trace: "res,mft+,dmo+,mft+,dmo+", hash: 0x9ef7_0f2b_90ba_1458 },
    ServerRow { id: "R02", ok: 0x0000_0001, fail_at: 0, fail: 0x0000_0000, hr: 0x0000_0000,
        trace: "res,mft+,dmo+,mft+,dmo+", hash: 0x9ef7_0f2b_90ba_1458 },
    ServerRow { id: "R03", ok: 0x0000_0000, fail_at: 1, fail: 0x8000_4005, hr: 0x8000_4005,
        trace: "res", hash: 0xaf63_b84c_8601_af60 },
    ServerRow { id: "R04", ok: 0x0000_0000, fail_at: 2, fail: 0x8007_000e, hr: 0x8007_000e,
        trace: "res,mft+", hash: 0x011b_e7a4_9ed7_a747 },
    ServerRow { id: "R05", ok: 0x0000_0000, fail_at: 3, fail: 0x8007_0005, hr: 0x8007_0005,
        trace: "res,mft+,dmo+", hash: 0xd438_8c74_6a69_048c },
    ServerRow { id: "R06", ok: 0x0000_0000, fail_at: 4, fail: 0x8000_4005, hr: 0x8000_4005,
        trace: "res,mft+,dmo+,mft+", hash: 0x078b_b903_62d5_c6f8 },
    ServerRow { id: "R07", ok: 0x0000_0000, fail_at: 5, fail: 0x8007_000e, hr: 0x8007_000e,
        trace: "res,mft+,dmo+,mft+,dmo+", hash: 0x9ef7_0f2b_90ba_1458 },
    ServerRow { id: "R08", ok: 0x0000_0000, fail_at: 2, fail: 0x7fff_ffff, hr: 0x0000_0000,
        trace: "res,mft+,dmo+,mft+,dmo+", hash: 0x9ef7_0f2b_90ba_1458 },
    ServerRow { id: "R09", ok: 0x0000_0000, fail_at: 1, fail: 0x8000_0000, hr: 0x8000_0000,
        trace: "res", hash: 0xaf63_b84c_8601_af60 },
];

#[rustfmt::skip]
const UNREGISTER_ROWS: [ServerRow; 7] = [
    ServerRow { id: "U01", ok: 0x0000_0000, fail_at: 0, fail: 0x0000_0000, hr: 0x0000_0000,
        trace: "unres,mft-,dmo-,mft-,dmo-", hash: 0x65dd_8e76_a838_d547 },
    ServerRow { id: "U02", ok: 0x0000_0001, fail_at: 0, fail: 0x0000_0000, hr: 0x0000_0000,
        trace: "unres,mft-,dmo-,mft-,dmo-", hash: 0x65dd_8e76_a838_d547 },
    ServerRow { id: "U03", ok: 0x0000_0000, fail_at: 1, fail: 0x8000_4005, hr: 0x8000_4005,
        trace: "unres", hash: 0xaf63_bb4c_8601_b479 },
    ServerRow { id: "U04", ok: 0x0000_0000, fail_at: 2, fail: 0x8007_000e, hr: 0x8007_000e,
        trace: "unres,mft-", hash: 0x8daa_1dd3_1052_7706 },
    ServerRow { id: "U05", ok: 0x0000_0000, fail_at: 3, fail: 0x8007_0005, hr: 0x8007_0005,
        trace: "unres,mft-,dmo-", hash: 0x7c27_10b2_32c9_6eb4 },
    ServerRow { id: "U06", ok: 0x0000_0000, fail_at: 4, fail: 0x8000_4005, hr: 0x8000_4005,
        trace: "unres,mft-,dmo-,mft-", hash: 0x7dff_d605_fbe9_5ca8 },
    ServerRow { id: "U07", ok: 0x0000_0000, fail_at: 5, fail: 0x8007_000e, hr: 0x8007_000e,
        trace: "unres,mft-,dmo-,mft-,dmo-", hash: 0x65dd_8e76_a838_d547 },
];

#[rustfmt::skip]
const CLASS_ROWS: [ClassRow; 4] = [
    ClassRow { id: "G01", clsid: CLSID_C_FRAME_RATE_CONVERT_DMO, riid: IID_CLASS_FACTORY, hr: 0x8004_0111 },
    ClassRow { id: "G02", clsid: CLSID_C_COLOR_CONTROL_DMO, riid: ZERO_GUID, hr: 0x8004_0111 },
    ClassRow { id: "G03", clsid: ZERO_GUID, riid: IID_CLASS_FACTORY, hr: 0x8004_0111 },
    ClassRow { id: "G04", clsid: ONES_GUID, riid: ZERO_GUID, hr: 0x8004_0111 },
];

#[test]
fn oracle_register_server_rows() {
    for row in &REGISTER_ROWS {
        let mut rec = Recorder::new(row.ok, row.fail_at, row.fail);
        let hr = dll_register_server(&mut rec);
        assert_eq!(hr, row.hr, "{}", row.id);
        assert_eq!(rec.trace(), row.trace, "{}", row.id);
        assert_eq!(rec.hash(), row.hash, "{}", row.id);
    }
}

#[test]
fn oracle_unregister_server_rows() {
    for row in &UNREGISTER_ROWS {
        let mut rec = Recorder::new(row.ok, row.fail_at, row.fail);
        let hr = dll_unregister_server(&mut rec);
        assert_eq!(hr, row.hr, "{}", row.id);
        assert_eq!(rec.trace(), row.trace, "{}", row.id);
        assert_eq!(rec.hash(), row.hash, "{}", row.id);
    }
}

#[test]
fn oracle_get_class_object_rows() {
    for row in &CLASS_ROWS {
        let mut out = Some(0_u8);
        let hr = dll_get_class_object(&row.clsid, &row.riid, &mut out);
        assert_eq!(hr, row.hr, "{}", row.id);
        assert!(out.is_none(), "{}", row.id);
    }
}

#[test]
fn oracle_can_unload_now_row() {
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
}
