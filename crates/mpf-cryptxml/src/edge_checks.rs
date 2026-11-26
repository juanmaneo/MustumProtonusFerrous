//! Edge-case tests for the ported cryptxml logic.
//!
//! These cover unissued and closed handles, the out-pointer rules, the document and
//! signature lifecycle, verify scope, the derived impls and the stub table. Expectations
//! that are not oracle-derived say so in a comment. Most of them follow from reading
//! `dlls/cryptxml/cryptxml.c` and `cryptxml.spec`.

use super::*;

const SIGNED: &[u8] = b"SIGNED";

fn open(store: &mut Cryptxml, blob: &[u8]) -> Handle {
    let mut doc = Handle::NULL;
    assert_eq!(store.open_to_decode(Some(blob), Some(&mut doc)), S_OK);
    doc
}

fn sig_of(store: &Cryptxml, doc: Handle) -> Handle {
    let mut ctx = None;
    assert_eq!(store.get_doc_context(doc, Some(&mut ctx)), S_OK);
    ctx.expect("document context").signatures[0]
}

fn status_of(store: &Cryptxml, handle: Handle) -> Status {
    let mut st = Status {
        cb_size: 0,
        error_status: 0,
        info_status: 0,
    };
    assert_eq!(store.get_status(handle, Some(&mut st)), S_OK);
    st
}

#[test]
fn unissued_handles_are_invalid() {
    // Not oracle-derived: C reads wild memory for these, so the port's answer is a choice
    // (see the Skipped list in lib.rs).
    let mut store = Cryptxml::default();
    let wild = Handle(99);
    let mut ctx = None;
    let mut sig = None;
    let mut st = Status {
        cb_size: 7,
        error_status: 7,
        info_status: 7,
    };
    assert_eq!(
        store.get_doc_context(wild, Some(&mut ctx)),
        CRYPT_XML_E_HANDLE
    );
    assert_eq!(
        store.get_signature(wild, Some(&mut sig)),
        CRYPT_XML_E_HANDLE
    );
    assert_eq!(store.get_status(wild, Some(&mut st)), CRYPT_XML_E_HANDLE);
    assert_eq!(store.verify_signature(wild), CRYPT_XML_E_HANDLE);
    assert_eq!(store.close(wild), CRYPT_XML_E_HANDLE);
    assert_eq!(st.cb_size, 7);
}

#[test]
fn wrong_kind_clears_the_out_pointer() {
    // Not oracle-derived: the oracle starts both out pointers at NULL. Starting from a
    // non-NULL value checks that the C code's write of NULL is reproduced.
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);

    let mut ctx = None;
    assert_eq!(store.get_doc_context(doc, Some(&mut ctx)), S_OK);
    assert!(ctx.is_some());
    assert_eq!(
        store.get_doc_context(sig, Some(&mut ctx)),
        CRYPT_XML_E_HANDLE
    );
    assert_eq!(ctx, None);

    let mut out = None;
    assert_eq!(store.get_signature(sig, Some(&mut out)), S_OK);
    assert!(out.is_some());
    assert_eq!(store.get_signature(doc, Some(&mut out)), CRYPT_XML_E_HANDLE);
    assert_eq!(out, None);
}

#[test]
fn closing_a_document_closes_its_signature() {
    // Not oracle-derived: the C code frees the signature together with its document.
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    assert_eq!(store.close(doc), S_OK);
    let mut st = Status {
        cb_size: 0,
        error_status: 0,
        info_status: 0,
    };
    assert_eq!(store.get_status(sig, Some(&mut st)), CRYPT_XML_E_HANDLE);
    assert_eq!(store.get_status(doc, Some(&mut st)), CRYPT_XML_E_HANDLE);
    assert_eq!(store.close(sig), CRYPT_XML_E_HANDLE);
    assert_eq!(store.verify_signature(sig), CRYPT_XML_E_HANDLE);
    assert_eq!(store.close(doc), CRYPT_XML_E_HANDLE);
}

#[test]
fn closing_a_signature_is_repeatable() {
    // Not oracle-derived: CryptXmlClose returns S_FALSE for a signature and frees nothing,
    // so the document and the signature stay usable.
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    assert_eq!(store.close(sig), S_FALSE);
    assert_eq!(store.close(sig), S_FALSE);
    assert_eq!(store.verify_signature(sig), S_OK);
    assert_eq!(status_of(&store, doc).cb_size, STATUS_SIZE);
}

#[test]
fn issued_handles_are_not_reused() {
    // Not oracle-derived: a property of the arena. The first open issues signature 1 and
    // document 2. After both are closed, the next open issues 3 and 4.
    let mut store = Cryptxml::default();
    let first = open(&mut store, SIGNED);
    assert_eq!(first, Handle(2));
    assert_eq!(store.close(first), S_OK);
    let second = open(&mut store, SIGNED);
    assert_eq!(second, Handle(4));
}

#[test]
fn failed_open_allocates_nothing() {
    // Not oracle-derived: a failed open returns before any allocation, so the next
    // successful open gets the same handle as in a fresh store.
    let mut store = Cryptxml::default();
    let mut handle = Handle::NULL;
    assert_eq!(store.open_to_decode(None, Some(&mut handle)), E_INVALIDARG);
    assert_eq!(handle, Handle::NULL);
    assert_eq!(store.open_to_decode(Some(SIGNED), None), E_INVALIDARG);
    let doc = open(&mut store, SIGNED);
    assert_eq!(doc, open(&mut Cryptxml::default(), SIGNED));
}

#[test]
fn signature_keeps_its_own_copy_of_the_blob() {
    // Not oracle-derived: the C code memcpy's the blob into the signature. Clearing the
    // caller's buffer after the open must not change the stored value.
    let mut store = Cryptxml::default();
    let mut blob = SIGNED.to_vec();
    let mut doc = Handle::NULL;
    assert_eq!(
        store.open_to_decode(Some(blob.as_slice()), Some(&mut doc)),
        S_OK
    );
    blob.clear();
    let mut out = None;
    assert_eq!(
        store.get_signature(sig_of(&store, doc), Some(&mut out)),
        S_OK
    );
    assert_eq!(out.expect("signature").signature_value, SIGNED);
}

#[test]
fn verify_marks_only_its_own_document() {
    // Not oracle-derived: the oracle verifies one document. This checks that a second
    // document and its signature keep their initial status.
    let mut store = Cryptxml::default();
    let doc_a = open(&mut store, SIGNED);
    let doc_b = open(&mut store, SIGNED);
    let sig_a = sig_of(&store, doc_a);
    let sig_b = sig_of(&store, doc_b);
    assert_eq!(store.verify_signature(sig_a), S_OK);
    assert_eq!(
        status_of(&store, doc_a),
        Status {
            cb_size: 12,
            error_status: 0,
            info_status: 0x1_0000,
        }
    );
    let initial = Status {
        cb_size: 12,
        error_status: 1,
        info_status: 0,
    };
    assert_eq!(status_of(&store, doc_b), initial);
    assert_eq!(status_of(&store, sig_b), initial);
}

#[test]
fn verify_is_idempotent_and_keeps_cb_size() {
    // Not oracle-derived: the oracle verifies once. Verifying twice must leave the same
    // status, with cbSize unchanged.
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    assert_eq!(store.verify_signature(sig), S_OK);
    assert_eq!(store.verify_signature(sig), S_OK);
    let verified = Status {
        cb_size: 12,
        error_status: 0,
        info_status: 0x1_0000,
    };
    assert_eq!(status_of(&store, doc), verified);
    assert_eq!(status_of(&store, sig), verified);
}

#[test]
fn non_stub_exports_raise_no_fault() {
    // Not oracle-derived: cryptxml.spec declares these six exports `stdcall`, not `stub`.
    for name in [
        "CryptXmlClose",
        "CryptXmlGetDocContext",
        "CryptXmlGetSignature",
        "CryptXmlGetStatus",
        "CryptXmlOpenToDecode",
        "CryptXmlVerifySignature",
    ] {
        assert_eq!(stub_fault(name), None);
    }
}

#[test]
fn stub_names_are_matched_exactly() {
    // Not oracle-derived: a prefix, a case change, a trailing space and the empty name
    // are all rejected.
    assert_eq!(stub_fault("cryptxmladdobject"), None);
    assert_eq!(stub_fault("CryptXmlSig"), None);
    assert_eq!(stub_fault("CryptXmlSign "), None);
    assert_eq!(stub_fault(""), None);
}

#[test]
fn stub_exports_are_sorted_and_unique() {
    // Not oracle-derived: the spec lists the stubs alphabetically, so the table is strictly
    // increasing. That rules out duplicates.
    assert!(STUB_EXPORTS.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn derived_impls_are_exercised() {
    // Not oracle-derived: calls the derived impls directly. llvm-cov counts them as
    // functions, and assert_eq! only formats on failure.
    let mut store = Cryptxml::default();
    let doc = open(&mut store, SIGNED);
    let sig = sig_of(&store, doc);
    assert!(format!("{store:?}").contains("Document"));
    assert!(format!("{store:?}").contains("Signature"));

    assert_eq!(Clone::clone(&doc), doc);
    assert!(format!("{doc:?}").starts_with("Handle("));

    let st = status_of(&store, doc);
    assert_eq!(Clone::clone(&st), st);
    assert!(format!("{st:?}").contains("error_status"));

    let mut ctx = None;
    assert_eq!(store.get_doc_context(doc, Some(&mut ctx)), S_OK);
    let ctx = ctx.expect("document context");
    assert_eq!(Clone::clone(&ctx), ctx);
    assert!(format!("{ctx:?}").contains("signatures"));

    let mut out = None;
    assert_eq!(store.get_signature(sig, Some(&mut out)), S_OK);
    let signature = out.expect("signature");
    assert_eq!(Clone::clone(&signature), signature);
    assert!(format!("{signature:?}").contains("key_info"));

    let key = signature.key_info;
    assert_eq!(Clone::clone(&key), key);
    assert!(format!("{key:?}").contains("verify_key_set"));

    let fault = stub_fault("CryptXmlSign").expect("stub export");
    assert_eq!(Clone::clone(&fault), fault);
    assert!(format!("{fault:?}").contains("CryptXmlSign"));
}
