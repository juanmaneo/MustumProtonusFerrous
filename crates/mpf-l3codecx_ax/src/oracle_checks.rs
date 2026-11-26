//! Oracle-derived test vectors for the ported l3codecx.ax logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the `IClassFactory` methods,
//! `DllGetClassObject`, `DllRegisterServer` and `DllUnregisterServer` in
//! `dlls/l3codecx.ax/l3codecx.c` (lines 36 to 156) with `gcc -std=gnu11 -fsanitize=address,undefined
//! -g` under `target/canon/l3codecx.ax/`, and running the harness. It printed 23 rows. ASan and
//! UBSan reported nothing, so no vector was excluded.

use super::*;

/// `CLSID_MPEGLayer3Decoder` with the last byte of `Data4` changed from 0x6d to 0x6e.
const NEAR_MISS_CLSID: Guid = Guid {
    data1: 0x38be_3000,
    data2: 0xdbf4,
    data3: 0x11d0,
    data4: [0x86, 0x0e, 0x00, 0xa0, 0x24, 0xcf, 0xef, 0x6e],
};

const ZERO_GUID: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// `REGDB_E_CLASSNOTREG`: the `CoCreateInstance` failure that rows 13 and 18 feed in.
const REGDB_E_CLASSNOTREG: Hresult = 0x8004_0154_u32 as Hresult;

/// Stands in for the COM, registry and resource calls. Each call appends its token to `trace` and
/// returns the result the test set for it.
#[derive(Default)]
pub(crate) struct Recorder {
    pub res: Hresult,
    pub unres: Hresult,
    pub cc: Hresult,
    pub rf: Hresult,
    pub uf: Hresult,
    pub trace: Vec<&'static str>,
    pub last_cc: Option<(Guid, bool, Guid)>,
    pub last_rf: Option<(Guid, String, RegFilter2)>,
    pub last_uf: Option<Guid>,
}

impl Recorder {
    pub fn trace_text(&self) -> String {
        self.trace.join(" ")
    }
}

impl ComHost for Recorder {
    fn register_resources(&mut self) -> Hresult {
        self.trace.push("RES");
        self.res
    }

    fn unregister_resources(&mut self) -> Hresult {
        self.trace.push("UNRES");
        self.unres
    }

    fn co_create_instance(&mut self, clsid: &Guid, outer: bool, iid: &Guid) -> Hresult {
        self.trace.push("CC");
        self.last_cc = Some((*clsid, outer, *iid));
        self.cc
    }

    fn register_filter(&mut self, clsid: &Guid, name: &str, desc: &RegFilter2) -> Hresult {
        self.trace.push("RF");
        self.last_rf = Some((*clsid, name.to_owned(), *desc));
        self.rf
    }

    fn unregister_filter(&mut self, clsid: &Guid) -> Hresult {
        self.trace.push("UF");
        self.last_uf = Some(*clsid);
        self.uf
    }

    fn release_filter_mapper(&mut self) {
        self.trace.push("REL");
    }
}

pub(crate) fn register_with(res: Hresult, cc: Hresult, rf: Hresult) -> (Hresult, Recorder) {
    let mut host = Recorder {
        res,
        cc,
        rf,
        ..Recorder::default()
    };
    let hr = dll_register_server(&mut host);
    (hr, host)
}

pub(crate) fn unregister_with(unres: Hresult, cc: Hresult, uf: Hresult) -> (Hresult, Recorder) {
    let mut host = Recorder {
        unres,
        cc,
        uf,
        ..Recorder::default()
    };
    let hr = dll_unregister_server(&mut host);
    (hr, host)
}

/// Formats a GUID the way the harness prints it: lower-case hex, with `Data4` split after two bytes.
pub(crate) fn guid_text(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// Formats a `RegisterFilter` call the way the harness prints row 23. The C literal leaves
/// `cInstances` at zero, which the model does not carry, so every pin prints `instances=0`.
pub(crate) fn filter_text(clsid: &Guid, name: &str, desc: &RegFilter2) -> String {
    let mut text = format!(
        "clsid={} name={name} version={} merit={:08x} pins={}",
        guid_text(clsid),
        desc.version,
        desc.merit,
        desc.pins.len()
    );
    for (index, pin) in desc.pins.iter().enumerate() {
        text.push_str(&format!(
            " pin{index} flags={:08x} instances=0 types={}",
            pin.flags,
            pin.media_types.len()
        ));
        for media in pin.media_types {
            text.push_str(&format!(
                " major={} minor={}",
                guid_text(&media.major_type),
                guid_text(&media.minor_type)
            ));
        }
    }
    text
}

#[test]
fn oracle_printed_guids_match_the_model() {
    // The GUID text as printed by the harness in rows 01 to 23.
    assert_eq!(
        guid_text(&CLSID_MPEG_LAYER3_DECODER),
        "38be3000-dbf4-11d0-860e-00a024cfef6d"
    );
    assert_eq!(
        guid_text(&CLSID_WG_MP3_DECODER),
        "84cd8e3e-b221-434a-8882-9d6c8df490e1"
    );
    assert_eq!(
        guid_text(&IID_ICLASS_FACTORY),
        "00000001-0000-0000-c000-000000000046"
    );
    assert_eq!(
        guid_text(&IID_IUNKNOWN),
        "00000000-0000-0000-c000-000000000046"
    );
    assert_eq!(
        guid_text(&IID_IFILTER_MAPPER2),
        "b79bb0b0-33c1-11d1-abe1-00a0c905f375"
    );
    assert_eq!(
        guid_text(&NEAR_MISS_CLSID),
        "38be3000-dbf4-11d0-860e-00a024cfef6e"
    );
    assert_eq!(
        guid_text(&ZERO_GUID),
        "00000000-0000-0000-0000-000000000000"
    );
    assert_eq!(
        guid_text(&MEDIATYPE_AUDIO),
        "73647561-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        guid_text(&MEDIASUBTYPE_MP3),
        "00000055-0000-0010-8000-00aa00389b71"
    );
    assert_eq!(
        guid_text(&MEDIASUBTYPE_PCM),
        "00000001-0000-0010-8000-00aa00389b71"
    );
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // (row, clsid, iid, hr, out is non-NULL)
    let rows = [
        (1, CLSID_MPEG_LAYER3_DECODER, IID_ICLASS_FACTORY, S_OK, true),
        (2, CLSID_MPEG_LAYER3_DECODER, IID_IUNKNOWN, S_OK, true),
        (
            3,
            CLSID_MPEG_LAYER3_DECODER,
            IID_IFILTER_MAPPER2,
            E_NOINTERFACE,
            false,
        ),
        (
            4,
            CLSID_WG_MP3_DECODER,
            IID_ICLASS_FACTORY,
            CLASS_E_CLASSNOTAVAILABLE,
            false,
        ),
        (5, ZERO_GUID, IID_IUNKNOWN, CLASS_E_CLASSNOTAVAILABLE, false),
        (
            6,
            CLSID_MPEG_LAYER3_DECODER,
            ZERO_GUID,
            E_NOINTERFACE,
            false,
        ),
        (
            7,
            NEAR_MISS_CLSID,
            IID_ICLASS_FACTORY,
            CLASS_E_CLASSNOTAVAILABLE,
            false,
        ),
    ];
    for (row, clsid, iid, hr, out) in rows {
        let (got, factory) = dll_get_class_object(&clsid, &iid);
        assert_eq!((got, factory.is_some()), (hr, out), "row {row:02}");
    }
}

#[test]
fn oracle_class_factory_rows() {
    // row 08
    assert_eq!(MPEG_LAYER3_DECODER_FACTORY.add_ref(), 2);
    // row 09
    assert_eq!(MPEG_LAYER3_DECODER_FACTORY.release(), 1);
    // row 10
    assert_eq!(MPEG_LAYER3_DECODER_FACTORY.lock_server(true), S_OK);
    // row 11
    assert_eq!(MPEG_LAYER3_DECODER_FACTORY.lock_server(false), S_OK);
}

#[test]
fn oracle_create_instance_rows() {
    // row 12: outer NULL, riid IUnknown, CoCreateInstance succeeds
    let mut host = Recorder {
        cc: S_OK,
        ..Recorder::default()
    };
    let hr = MPEG_LAYER3_DECODER_FACTORY.create_instance(&mut host, false, &IID_IUNKNOWN);
    assert_eq!(hr, S_OK);
    assert_eq!(host.trace_text(), "CC");
    assert_eq!(
        host.last_cc,
        Some((CLSID_WG_MP3_DECODER, false, IID_IUNKNOWN))
    );

    // row 13: outer non-NULL, riid IClassFactory, CoCreateInstance fails with REGDB_E_CLASSNOTREG
    let mut host = Recorder {
        cc: REGDB_E_CLASSNOTREG,
        ..Recorder::default()
    };
    let hr = MPEG_LAYER3_DECODER_FACTORY.create_instance(&mut host, true, &IID_ICLASS_FACTORY);
    assert_eq!(hr, REGDB_E_CLASSNOTREG);
    assert_eq!(host.trace_text(), "CC");
    assert_eq!(
        host.last_cc,
        Some((CLSID_WG_MP3_DECODER, true, IID_ICLASS_FACTORY))
    );
}

#[test]
fn oracle_dll_register_server_rows() {
    // (row, res, cc, rf, hr, trace)
    let rows = [
        (14, E_FAIL, S_OK, S_OK, E_FAIL, "RES"),
        (15, S_FALSE, E_FAIL, S_OK, E_FAIL, "RES CC"),
        (16, S_OK, S_OK, S_OK, S_OK, "RES CC RF REL"),
        (17, S_FALSE, S_FALSE, E_FAIL, E_FAIL, "RES CC RF REL"),
        (
            18,
            S_OK,
            REGDB_E_CLASSNOTREG,
            S_OK,
            REGDB_E_CLASSNOTREG,
            "RES CC",
        ),
    ];
    for (row, res, cc, rf, hr, trace) in rows {
        let (got, host) = register_with(res, cc, rf);
        assert_eq!(got, hr, "row {row}");
        assert_eq!(host.trace_text(), trace, "row {row}");
    }
}

#[test]
fn oracle_dll_unregister_server_rows() {
    // (row, unres, cc, uf, hr, trace)
    let rows = [
        (19, E_FAIL, S_OK, S_OK, E_FAIL, "UNRES"),
        (20, S_OK, E_FAIL, S_OK, E_FAIL, "UNRES CC"),
        (21, S_OK, S_OK, E_FAIL, S_OK, "UNRES CC UF REL"),
        (22, S_FALSE, S_OK, S_OK, S_OK, "UNRES CC UF REL"),
    ];
    for (row, unres, cc, uf, hr, trace) in rows {
        let (got, host) = unregister_with(unres, cc, uf);
        assert_eq!(got, hr, "row {row}");
        assert_eq!(host.trace_text(), trace, "row {row}");
    }
}

#[test]
fn oracle_register_filter_row() {
    // row 23: the RegisterFilter call made by DllRegisterServer in row 16
    let (hr, host) = register_with(S_OK, S_OK, S_OK);
    assert_eq!(hr, S_OK);
    let (clsid, name, desc) = host.last_rf.expect("RegisterFilter was called");
    assert_eq!(
        filter_text(&clsid, &name, &desc),
        "clsid=38be3000-dbf4-11d0-860e-00a024cfef6d name=MPEG Layer-3 Decoder version=2 \
         merit=00810000 pins=2 pin0 flags=00000000 instances=0 types=1 \
         major=73647561-0000-0010-8000-00aa00389b71 minor=00000055-0000-0010-8000-00aa00389b71 \
         pin1 flags=00000008 instances=0 types=1 \
         major=73647561-0000-0010-8000-00aa00389b71 minor=00000001-0000-0010-8000-00aa00389b71"
    );
}
