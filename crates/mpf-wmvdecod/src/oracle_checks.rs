//! Oracle-derived test vectors for the ported `wmvdecod` logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/wmvdecod/wmvdecod.c`,
//! `dlls/wmvdecod/video_decoder.c` and `libs/winecrt0/dll_canunload.c` under `target/canon/wmvdecod/`
//! with `gcc -fsanitize=address,undefined -g`, and running the harness. All 25 printed rows ran clean
//! under ASan and UBSan, so none is excluded. The row text is quoted in each test's comments.

use super::*;

/// `IID_IDispatch`, which `shim.h` defines as an unrelated IID for the negative cases.
fn iid_idispatch() -> Guid {
    Guid::new(0x0002_0400, 0x0000, 0x0000, [0xc0, 0, 0, 0, 0, 0, 0, 0x46])
}

/// One registration call, recorded in the order the C code makes it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Call {
    RegisterResources,
    UnregisterResources,
    MftRegister {
        clsid: Guid,
        category: Guid,
        name: String,
        flags: u32,
        inputs: Vec<MediaType>,
        outputs: Vec<MediaType>,
    },
    MftUnregister {
        clsid: Guid,
    },
    DmoRegister {
        name: String,
        clsid: Guid,
        category: Guid,
        flags: u32,
        inputs: Vec<DmoMediaType>,
        outputs: Vec<DmoMediaType>,
    },
    DmoUnregister {
        clsid: Guid,
        category: Guid,
    },
}

/// A `Registrar` that records each call and returns the HRESULT preset for it. Every preset is `S_OK`.
pub(crate) struct Recorder {
    pub(crate) calls: Vec<Call>,
    pub(crate) ret_register_resources: Hresult,
    pub(crate) ret_unregister_resources: Hresult,
    pub(crate) ret_mft_register: Hresult,
    pub(crate) ret_mft_unregister: Hresult,
    pub(crate) ret_dmo_register: Hresult,
    pub(crate) ret_dmo_unregister: Hresult,
}

impl Recorder {
    pub(crate) fn new() -> Recorder {
        Recorder {
            calls: Vec::new(),
            ret_register_resources: S_OK,
            ret_unregister_resources: S_OK,
            ret_mft_register: S_OK,
            ret_mft_unregister: S_OK,
            ret_dmo_register: S_OK,
            ret_dmo_unregister: S_OK,
        }
    }
}

impl Registrar for Recorder {
    fn register_resources(&mut self) -> Hresult {
        self.calls.push(Call::RegisterResources);
        self.ret_register_resources
    }

    fn unregister_resources(&mut self) -> Hresult {
        self.calls.push(Call::UnregisterResources);
        self.ret_unregister_resources
    }

    fn mft_register(
        &mut self,
        clsid: &Guid,
        category: &Guid,
        name: &str,
        flags: u32,
        inputs: &[MediaType],
        outputs: &[MediaType],
    ) -> Hresult {
        self.calls.push(Call::MftRegister {
            clsid: *clsid,
            category: *category,
            name: name.to_string(),
            flags,
            inputs: inputs.to_vec(),
            outputs: outputs.to_vec(),
        });
        self.ret_mft_register
    }

    fn mft_unregister(&mut self, clsid: &Guid) -> Hresult {
        self.calls.push(Call::MftUnregister { clsid: *clsid });
        self.ret_mft_unregister
    }

    fn dmo_register(
        &mut self,
        name: &str,
        clsid: &Guid,
        category: &Guid,
        flags: u32,
        inputs: &[DmoMediaType],
        outputs: &[DmoMediaType],
    ) -> Hresult {
        self.calls.push(Call::DmoRegister {
            name: name.to_string(),
            clsid: *clsid,
            category: *category,
            flags,
            inputs: inputs.to_vec(),
            outputs: outputs.to_vec(),
        });
        self.ret_dmo_register
    }

    fn dmo_unregister(&mut self, clsid: &Guid, category: &Guid) -> Hresult {
        self.calls.push(Call::DmoUnregister {
            clsid: *clsid,
            category: *category,
        });
        self.ret_dmo_unregister
    }
}

/// The `MFTRegister` call that `DllRegisterServer` makes, with the names and flags from the oracle.
pub(crate) fn mft_call() -> Call {
    Call::MftRegister {
        clsid: CLSID_WMVDECODER_MFT,
        category: MFT_CATEGORY_VIDEO_DECODER,
        name: "WMVideo Decoder MFT".to_string(),
        flags: 1,
        inputs: MFT_INPUT_TYPES.to_vec(),
        outputs: MFT_OUTPUT_TYPES.to_vec(),
    }
}

/// The `DMORegister` call that `DllRegisterServer` makes, with the names and flags from the oracle.
pub(crate) fn dmo_call() -> Call {
    Call::DmoRegister {
        name: "WMVideo Decoder DMO".to_string(),
        clsid: CLSID_WMVDECODER_MFT,
        category: DMOCATEGORY_VIDEO_DECODER,
        flags: 0,
        inputs: DMO_INPUT_TYPES.to_vec(),
        outputs: DMO_OUTPUT_TYPES.to_vec(),
    }
}

/// The calls of a successful `DllRegisterServer`, in order.
pub(crate) fn full_register_calls() -> Vec<Call> {
    vec![Call::RegisterResources, mft_call(), dmo_call()]
}

/// The calls of a successful `DllUnregisterServer`, in order.
pub(crate) fn full_unregister_calls() -> Vec<Call> {
    vec![
        Call::UnregisterResources,
        Call::MftUnregister {
            clsid: CLSID_WMVDECODER_MFT,
        },
        Call::DmoUnregister {
            clsid: CLSID_WMVDECODER_MFT,
            category: DMOCATEGORY_VIDEO_DECODER,
        },
    ]
}

/// The `GUID` text the harness prints, `{%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x}`.
fn guid_text(guid: &Guid) -> String {
    let d = guid.data4;
    format!(
        "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
        guid.data1, guid.data2, guid.data3, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]
    )
}

/// The `{major}/{subtype} ` entries of a table, each followed by a space as the harness prints them.
fn pairs_text(pairs: &[(Guid, Guid)]) -> String {
    pairs
        .iter()
        .map(|(major, subtype)| format!("{}/{} ", guid_text(major), guid_text(subtype)))
        .collect()
}

/// A table line in the harness format: `{label} in n={in} [{entries}] out n={out} [{entries}]`.
fn table_text(label: &str, inputs: &[(Guid, Guid)], outputs: &[(Guid, Guid)]) -> String {
    format!(
        "{label} in n={} [{}] out n={} [{}]",
        inputs.len(),
        pairs_text(inputs),
        outputs.len(),
        pairs_text(outputs)
    )
}

fn mft_pairs(types: &[MediaType]) -> Vec<(Guid, Guid)> {
    types.iter().map(|t| (t.major, t.subtype)).collect()
}

fn dmo_pairs(types: &[DmoMediaType]) -> Vec<(Guid, Guid)> {
    types.iter().map(|t| (t.media_type, t.subtype)).collect()
}

#[test]
fn oracle_class_object_rows_01_to_05() {
    // row 01 DllGetClassObject(CLSID_WMVDecoderMFT, IID_IClassFactory) -> hr=0x00000000 out=wmv
    assert_eq!(
        dll_get_class_object(&CLSID_WMVDECODER_MFT, &IID_ICLASSFACTORY),
        Ok(ClassFactory::Wmv)
    );
    // row 02 DllGetClassObject(CLSID_WMVDecoderMFT, IID_IUnknown) -> hr=0x00000000 out=wmv
    assert_eq!(
        dll_get_class_object(&CLSID_WMVDECODER_MFT, &IID_IUNKNOWN),
        Ok(ClassFactory::Wmv)
    );
    // row 03 DllGetClassObject(CLSID_WMVDecoderMFT, IID_IDispatch) -> hr=0x80004002 out=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_WMVDECODER_MFT, &iid_idispatch()),
        Err(E_NOINTERFACE)
    );
    // row 04 DllGetClassObject(CLSID_wg_wmv_decoder, IID_IClassFactory) -> hr=0x80040111 out=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_WG_WMV_DECODER, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    // row 05 DllGetClassObject(CLSID_wg_h264_decoder, IID_IClassFactory) -> hr=0x80040111 out=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_WG_H264_DECODER, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_factory_rows_06_to_13() {
    // row 06 wmv_decoder_factory.QueryInterface(IID_IClassFactory) -> hr=0x00000000 out=wmv
    assert_eq!(
        factory_query_interface(ClassFactory::Wmv, &IID_ICLASSFACTORY),
        Ok(ClassFactory::Wmv)
    );
    // row 07 h264_decoder_factory.QueryInterface(IID_IClassFactory) -> hr=0x00000000 out=h264
    assert_eq!(
        factory_query_interface(ClassFactory::H264, &IID_ICLASSFACTORY),
        Ok(ClassFactory::H264)
    );
    // row 08 wmv_decoder_factory.QueryInterface(IID_IDispatch) -> hr=0x80004002 out=NULL
    assert_eq!(
        factory_query_interface(ClassFactory::Wmv, &iid_idispatch()),
        Err(E_NOINTERFACE)
    );
    // row 09 wmv_decoder_factory.AddRef() -> ret=2
    assert_eq!(factory_add_ref(), 2);
    // row 10 wmv_decoder_factory.Release() -> ret=1
    assert_eq!(factory_release(), 1);
    // row 11 wmv_decoder_factory.LockServer(TRUE) -> hr=0x00000000
    assert_eq!(factory_lock_server(true), S_OK);
    // row 12 wmv_decoder_factory.CreateInstance(outer=NULL, IID_IUnknown) -> hr=0x80040154 out=NULL
    //   calls=[CCI(clsid=CLSID_wg_wmv_decoder,outer=NULL,ctx=1,riid=IID_IUnknown)]
    // The port keeps the request. The C code returns the CoCreateInstance result unchanged, which the
    // harness makes REGDB_E_CLASSNOTREG. That value is pinned below.
    assert_eq!(
        create_instance_request(ClassFactory::Wmv),
        CoCreateRequest {
            clsid: CLSID_WG_WMV_DECODER,
            clsctx: CLSCTX_INPROC_SERVER,
        }
    );
    assert_eq!(REGDB_E_CLASSNOTREG, 0x8004_0154u32 as i32);
    // row 13 h264_decoder_factory.CreateInstance(outer=NULL, IID_IClassFactory) -> hr=0x00000000 out=NULL
    //   calls=[CCI(clsid=CLSID_wg_h264_decoder,outer=NULL,ctx=1,riid=IID_IClassFactory)]
    assert_eq!(
        create_instance_request(ClassFactory::H264),
        CoCreateRequest {
            clsid: CLSID_WG_H264_DECODER,
            clsctx: CLSCTX_INPROC_SERVER,
        }
    );
}

/// Row 15 as the harness prints it, after the `row 15 ` prefix.
const ROW_15_TEXT: &str = "MFT_REGISTER_TYPE_INFO in n=9 [{73646976-0000-0010-8000-00aa00389b71}/{31564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{32564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{33564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{50564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{32505657-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{52564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{41564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{31435657-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{53314356-0000-0010-8000-00aa00389b71} ] out n=11 [{73646976-0000-0010-8000-00aa00389b71}/{32315659-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{32595559-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{59565955-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{55595659-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{3131564e-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{3231564e-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{00000016-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{00000014-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{00000017-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{00000018-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{00000029-524f-11ce-9f53-0020af0ba770} ]";

/// Row 16 as the harness prints it, after the `row 16 ` prefix.
const ROW_16_TEXT: &str = "DMO_PARTIAL_MEDIATYPE in n=8 [{73646976-0000-0010-8000-00aa00389b71}/{31564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{32564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{33564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{41564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{31435657-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{50564d57-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{32505657-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{53314356-0000-0010-8000-00aa00389b71} ] out n=11 [{73646976-0000-0010-8000-00aa00389b71}/{32315659-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{32595559-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{59565955-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{55595659-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{3131564e-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{3231564e-0000-0010-8000-00aa00389b71} {73646976-0000-0010-8000-00aa00389b71}/{e436eb7e-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{e436eb7d-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{e436eb7b-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{e436eb7c-524f-11ce-9f53-0020af0ba770} {73646976-0000-0010-8000-00aa00389b71}/{e436eb7a-524f-11ce-9f53-0020af0ba770} ]";

#[test]
fn oracle_mft_table_row_15() {
    // row 15 MFT_REGISTER_TYPE_INFO in n=9 [...] out n=11 [...], printed in full in ROW_15_TEXT
    let got = table_text(
        "MFT_REGISTER_TYPE_INFO",
        &mft_pairs(&MFT_INPUT_TYPES),
        &mft_pairs(&MFT_OUTPUT_TYPES),
    );
    assert_eq!(got, ROW_15_TEXT);
}

#[test]
fn oracle_dmo_table_row_16() {
    // row 16 DMO_PARTIAL_MEDIATYPE in n=8 [...] out n=11 [...], printed in full in ROW_16_TEXT
    let got = table_text(
        "DMO_PARTIAL_MEDIATYPE",
        &dmo_pairs(&DMO_INPUT_TYPES),
        &dmo_pairs(&DMO_OUTPUT_TYPES),
    );
    assert_eq!(got, ROW_16_TEXT);
}

#[test]
fn oracle_register_rows_14_and_17_to_20() {
    // row 14 DllRegisterServer() all ok -> hr=0x00000000 calls=[RES_REG;MFT(...,name=WMVideo Decoder MFT,flags=1,in=9,out=11);DMO(name=WMVideo Decoder DMO,flags=0,in=8,out=11);]
    let mut r = Recorder::new();
    assert_eq!(dll_register_server(&mut r), S_OK);
    assert_eq!(r.calls, full_register_calls());

    // row 17 DllRegisterServer() __wine_register_resources=E_FAIL -> hr=0x80004005 calls=[RES_REG;]
    let mut r = Recorder::new();
    r.ret_register_resources = E_FAIL;
    assert_eq!(dll_register_server(&mut r), E_FAIL);
    assert_eq!(r.calls, [Call::RegisterResources]);

    // row 18 DllRegisterServer() __wine_register_resources=S_FALSE -> hr=0x00000000, full sequence
    let mut r = Recorder::new();
    r.ret_register_resources = S_FALSE;
    assert_eq!(dll_register_server(&mut r), S_OK);
    assert_eq!(r.calls, full_register_calls());

    // row 19 DllRegisterServer() MFTRegister=E_OUTOFMEMORY -> hr=0x8007000e calls=[RES_REG;MFT(...);]
    let mut r = Recorder::new();
    r.ret_mft_register = E_OUTOFMEMORY;
    assert_eq!(dll_register_server(&mut r), E_OUTOFMEMORY);
    assert_eq!(r.calls, [Call::RegisterResources, mft_call()]);

    // row 20 DllRegisterServer() DMORegister=E_FAIL -> hr=0x80004005 calls=[RES_REG;MFT(...);DMO(...);]
    let mut r = Recorder::new();
    r.ret_dmo_register = E_FAIL;
    assert_eq!(dll_register_server(&mut r), E_FAIL);
    assert_eq!(r.calls, [Call::RegisterResources, mft_call(), dmo_call()]);
}

#[test]
fn oracle_unregister_rows_21_to_24() {
    // row 21 DllUnregisterServer() all ok -> hr=0x00000000 calls=[RES_UNREG;MFTU;DMOU(cat=DMOCATEGORY_VIDEO_DECODER);]
    let mut r = Recorder::new();
    assert_eq!(dll_unregister_server(&mut r), S_OK);
    assert_eq!(r.calls, full_unregister_calls());

    // row 22 DllUnregisterServer() __wine_unregister_resources=E_FAIL -> hr=0x80004005 calls=[RES_UNREG;]
    let mut r = Recorder::new();
    r.ret_unregister_resources = E_FAIL;
    assert_eq!(dll_unregister_server(&mut r), E_FAIL);
    assert_eq!(r.calls, [Call::UnregisterResources]);

    // row 23 DllUnregisterServer() MFTUnregister=E_FAIL -> hr=0x80004005 calls=[RES_UNREG;MFTU;]
    let mut r = Recorder::new();
    r.ret_mft_unregister = E_FAIL;
    assert_eq!(dll_unregister_server(&mut r), E_FAIL);
    assert_eq!(
        r.calls,
        [
            Call::UnregisterResources,
            Call::MftUnregister {
                clsid: CLSID_WMVDECODER_MFT,
            },
        ]
    );

    // row 24 DllUnregisterServer() DMOUnregister=E_FAIL -> hr=0x80004005 calls=[RES_UNREG;MFTU;DMOU;]
    let mut r = Recorder::new();
    r.ret_dmo_unregister = E_FAIL;
    assert_eq!(dll_unregister_server(&mut r), E_FAIL);
    assert_eq!(r.calls, full_unregister_calls());
}

#[test]
fn oracle_can_unload_now_row_25() {
    // row 25 DllCanUnloadNow() -> hr=0x00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);
    assert_eq!(dll_can_unload_now(), 1);
}
