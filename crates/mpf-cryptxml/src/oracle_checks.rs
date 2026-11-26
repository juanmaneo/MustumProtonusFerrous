//! Oracle-derived test vectors for the ported cryptxml logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `CryptXmlOpenToDecode` through `CryptXmlVerifySignature` (`dlls/cryptxml/cryptxml.c`,
//! lines 30-204) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`, lines
//! 504-516) with the Windows x64 type shims in `target/canon/cryptxml/shim.h`, using
//! `gcc -fsanitize=address,undefined -g` under `target/canon/cryptxml/`. The run printed a
//! layout line and 25 rows: 12 scenario rows and one row for each of the 13 `stub`
//! exports in `cryptxml.spec`. ASan and UBSan reported nothing, so 0 vectors were excluded.
//! Each scenario test below mirrors one `row_*` function of the harness.

use super::*;

const SIGNED: &[u8] = b"SIGNED";

/// Value the oracle's status buffers hold before a call, so an untouched buffer shows.
const SENTINEL_STATUS: Status = Status {
    cb_size: 0xdead_beef,
    error_status: 0,
    info_status: 0,
};

fn open(store: &mut Cryptxml, blob: &[u8]) -> Handle {
    let mut doc = Handle::NULL;
    assert_eq!(store.open_to_decode(Some(blob), Some(&mut doc)), S_OK);
    doc
}

fn doc_context_of(store: &Cryptxml, doc: Handle) -> DocContext {
    let mut ctx = None;
    assert_eq!(store.get_doc_context(doc, Some(&mut ctx)), S_OK);
    ctx.expect("document context")
}

fn signature_of(store: &Cryptxml, sig: Handle) -> Signature {
    let mut out = None;
    assert_eq!(store.get_signature(sig, Some(&mut out)), S_OK);
    out.expect("signature")
}

fn status_of(store: &Cryptxml, handle: Handle) -> Status {
    let mut st = SENTINEL_STATUS;
    assert_eq!(store.get_status(handle, Some(&mut st)), S_OK);
    st
}

/// The signature of `doc`, as `ctx->rgpSignature[0]->hSignature` in the harness.
fn sig_of(store: &Cryptxml, doc: Handle) -> Handle {
    doc_context_of(store, doc).signatures[0]
}

#[test]
fn oracle_layout() {
    // layout sizeof_status=12 sizeof_doc_ctx=40 sizeof_signature=176
    assert_eq!(STATUS_SIZE, 12);
    assert_eq!(DOC_CONTEXT_SIZE, 40);
    assert_eq!(SIGNATURE_SIZE, 176);
}

#[test]
fn oracle_open_null_blob() {
    // open_null_blob hr=0x80070057 handle_untouched=1
    let untouched = Handle(0xdead_beef);
    let mut handle = untouched;
    let mut store = Cryptxml::default();
    assert_eq!(store.open_to_decode(None, Some(&mut handle)), E_INVALIDARG);
    assert_eq!(handle, untouched);
}

#[test]
fn oracle_open_null_handle() {
    // open_null_handle hr=0x80070057
    let mut store = Cryptxml::default();
    assert_eq!(store.open_to_decode(Some(SIGNED), None), E_INVALIDARG);
}

#[test]
fn oracle_open_signed() {
    // open_signed hr=0 ctx_hr=0 ctx_cb=40 ctx_count=1 ctx_handle_is_doc=1 ctx_sig_is_sig=1
    //   sig_hr=0 sig_cb=176 sig_value=SIGNED sig_value_len=6 key_info_nonnull=1 key_info_cb=0
    //   verify_key_null=1
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let ctx = doc_context_of(&store, doc);
    assert_eq!(ctx.cb_size, 40);
    assert_eq!(ctx.signatures.len(), 1);
    assert_eq!(ctx.handle, doc);
    let sig = signature_of(&store, ctx.signatures[0]);
    assert_eq!(sig.handle, ctx.signatures[0]);
    assert_eq!(sig.cb_size, 176);
    assert_eq!(sig.signature_value, SIGNED);
    assert_eq!(
        sig.key_info,
        KeyInfo {
            cb_size: 0,
            verify_key_set: false
        }
    );
}

#[test]
fn oracle_open_empty() {
    // open_empty hr=0 sig_value_len=0 sig_cb=176
    let mut store = Cryptxml::default();
    let doc = open(&mut store, &[]);
    let sig = signature_of(&store, sig_of(&store, doc));
    assert!(sig.signature_value.is_empty());
    assert_eq!(sig.cb_size, 176);
}

#[test]
fn oracle_status_after_open() {
    // status_after_open doc_hr=0 doc_cb=12 doc_err=0x1 doc_info=0 sig_hr=0 sig_cb=12
    //   sig_err=0x1 sig_info=0
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let initial = Status {
        cb_size: 12,
        error_status: 0x1,
        info_status: 0,
    };
    assert_eq!(status_of(&store, doc), initial);
    assert_eq!(status_of(&store, sig_of(&store, doc)), initial);
}

#[test]
fn oracle_get_status_null() {
    // get_status_null handle_hr=0x80070057 out_untouched=1 out_hr=0x80070057
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let mut st = SENTINEL_STATUS;
    assert_eq!(store.get_status(Handle::NULL, Some(&mut st)), E_INVALIDARG);
    assert_eq!(st, SENTINEL_STATUS);
    assert_eq!(store.get_status(doc, None), E_INVALIDARG);
}

#[test]
fn oracle_doc_context_probe() {
    // doc_context_probe null_handle_hr=0x80070057 null_out_hr=0x80070057 sig_handle_hr=0x80092106
    //   ctx_after_sig=NULL
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    let mut ctx = None;
    assert_eq!(
        store.get_doc_context(Handle::NULL, Some(&mut ctx)),
        E_INVALIDARG
    );
    assert_eq!(store.get_doc_context(doc, None), E_INVALIDARG);
    assert_eq!(
        store.get_doc_context(sig, Some(&mut ctx)),
        CRYPT_XML_E_HANDLE
    );
    assert_eq!(ctx, None);
}

#[test]
fn oracle_signature_probe() {
    // signature_probe null_handle_hr=0x80070057 null_out_hr=0x80070057 doc_handle_hr=0x80092106
    //   sig_after_doc=NULL
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let mut sig = None;
    assert_eq!(
        store.get_signature(Handle::NULL, Some(&mut sig)),
        E_INVALIDARG
    );
    assert_eq!(store.get_signature(doc, None), E_INVALIDARG);
    assert_eq!(store.get_signature(doc, Some(&mut sig)), CRYPT_XML_E_HANDLE);
    assert_eq!(sig, None);
}

#[test]
fn oracle_verify_doc() {
    // verify_doc hr=0x80092106 doc_err=0x1 doc_info=0
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    assert_eq!(store.verify_signature(doc), CRYPT_XML_E_HANDLE);
    let st = status_of(&store, doc);
    assert_eq!((st.error_status, st.info_status), (0x1, 0));
}

#[test]
fn oracle_verify_sig() {
    // verify_sig null_hr=0x80070057 hr=0 doc_err=0 doc_info=0x10000 sig_err=0 sig_info=0x10000
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    assert_eq!(store.verify_signature(Handle::NULL), E_INVALIDARG);
    assert_eq!(store.verify_signature(sig), S_OK);
    let doc_st = status_of(&store, doc);
    let sig_st = status_of(&store, sig);
    assert_eq!((doc_st.error_status, doc_st.info_status), (0, 0x1_0000));
    assert_eq!((sig_st.error_status, sig_st.info_status), (0, 0x1_0000));
}

#[test]
fn oracle_close_sig() {
    // close_sig hr=0x1 sig_status_hr=0 doc_status_hr=0
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    assert_eq!(store.close(sig), S_FALSE);
    assert_eq!(status_of(&store, sig).cb_size, STATUS_SIZE);
    assert_eq!(status_of(&store, doc).cb_size, STATUS_SIZE);
}

#[test]
fn oracle_close_doc() {
    // close_doc hr=0 null_hr=0x80070057
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    assert_eq!(store.close(Handle::NULL), E_INVALIDARG);
    assert_eq!(store.close(doc), S_OK);
}

#[test]
fn oracle_stub_rows() {
    // stub export=<name> code=0x80000100 flags=0x1 params=2 module=cryptxml function=<name>
    //   returned=0
    // params=2 is implied by StubFault carrying exactly two ExceptionInformation values.
    // returned=0: the stub raises and never returns normally.
    const STUB_ROWS: [(&str, u32, u32, &str); 13] = [
        ("CryptXmlAddObject", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlCreateReference", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlDigestReference", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlEncode", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlEnumAlgorithmInfo", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlFindAlgorithmInfo", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlGetAlgorithmInfo", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlGetReference", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlGetTransforms", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlImportPublicKey", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlOpenToEncode", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlSetHMACSecret", 0x8000_0100, 0x1, "cryptxml"),
        ("CryptXmlSign", 0x8000_0100, 0x1, "cryptxml"),
    ];
    assert_eq!(STUB_EXPORTS, STUB_ROWS.map(|row| row.0));
    for (function, code, flags, module) in STUB_ROWS {
        assert_eq!(
            stub_fault(function),
            Some(StubFault {
                code,
                flags,
                module,
                function,
            })
        );
    }
}
