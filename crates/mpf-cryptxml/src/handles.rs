//! Document and signature objects behind `HCRYPTXML`.
//!
//! Wine allocates each object with `calloc` and hands out its address. This port
//! keeps the objects in an arena and hands out the arena index instead. Index 0 is
//! the NULL handle and is never issued. An index is never reused, so a closed or
//! unissued handle is always detected.

/// `HRESULT`.
pub type HResult = u32;

pub const S_OK: HResult = 0;
pub const S_FALSE: HResult = 1;
pub const E_INVALIDARG: HResult = 0x8007_0057;
pub const CRYPT_XML_E_HANDLE: HResult = 0x8009_2106;

pub const CRYPT_XML_STATUS_NO_ERROR: u32 = 0;
pub const CRYPT_XML_STATUS_ERROR_NOT_RESOLVED: u32 = 0x1;
pub const CRYPT_XML_STATUS_SIGNATURE_VALID: u32 = 0x1_0000;

/// `sizeof(CRYPT_XML_STATUS)` on Windows x64.
pub const STATUS_SIZE: u32 = 12;
/// `sizeof(CRYPT_XML_DOC_CTXT)` on Windows x64.
pub const DOC_CONTEXT_SIZE: u32 = 40;
/// `sizeof(CRYPT_XML_SIGNATURE)` on Windows x64.
pub const SIGNATURE_SIZE: u32 = 176;

/// An `HCRYPTXML` value. Only [`Cryptxml`] issues non-NULL handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(pub(crate) usize);

impl Handle {
    /// The NULL handle.
    pub const NULL: Handle = Handle(0);

    fn is_null(self) -> bool {
        self == Self::NULL
    }
}

/// `CRYPT_XML_STATUS`, the status a document or signature reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub cb_size: u32,
    pub error_status: u32,
    pub info_status: u32,
}

impl Status {
    const INITIAL: Status = Status {
        cb_size: STATUS_SIZE,
        error_status: CRYPT_XML_STATUS_ERROR_NOT_RESOLVED,
        info_status: 0,
    };

    fn mark_verified(&mut self) {
        self.error_status = CRYPT_XML_STATUS_NO_ERROR;
        self.info_status = CRYPT_XML_STATUS_SIGNATURE_VALID;
    }
}

/// The `CRYPT_XML_KEY_INFO` that a signature points to. Wine's `calloc` zeroes it and
/// `CryptXmlOpenToDecode` never sets it, so only the two fields that the oracle reads are kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyInfo {
    pub cb_size: u32,
    pub verify_key_set: bool,
}

/// Snapshot of `CRYPT_XML_DOC_CTXT`. `pTransformsConfig` is always NULL and is omitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocContext {
    pub cb_size: u32,
    pub handle: Handle,
    /// `rgpSignature` as the handles of its entries; `cSignature` is its length.
    pub signatures: Vec<Handle>,
}

/// Snapshot of `CRYPT_XML_SIGNATURE`. Fields that the port never sets are omitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub cb_size: u32,
    pub handle: Handle,
    pub signature_value: Vec<u8>,
    pub key_info: KeyInfo,
}

#[derive(Debug)]
struct Object {
    status: Status,
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Document { signature: Handle },
    Signature { value: Vec<u8>, document: Handle },
}

/// The document and signature objects of one `CryptXmlOpenToDecode` store.
#[derive(Debug)]
pub struct Cryptxml {
    /// Slot `n` holds the object for handle `n`. A `None` slot has been closed.
    objects: Vec<Option<Object>>,
}

impl Default for Cryptxml {
    fn default() -> Self {
        Cryptxml {
            objects: vec![None],
        }
    }
}

impl Cryptxml {
    fn get(&self, handle: Handle) -> Option<&Object> {
        self.objects.get(handle.0)?.as_ref()
    }

    /// Only called for handles that a preceding `get` found live.
    fn live_mut(&mut self, handle: Handle) -> &mut Object {
        self.objects[handle.0]
            .as_mut()
            .expect("handle checked live")
    }

    /// `CryptXmlOpenToDecode`. `blob` is the blob's bytes, or `None` for a NULL blob
    /// pointer. The signature keeps a copy. The config, flags, property and count
    /// arguments have no effect and are omitted.
    pub fn open_to_decode(
        &mut self,
        blob: Option<&[u8]>,
        handle_out: Option<&mut Handle>,
    ) -> HResult {
        let (Some(blob), Some(handle_out)) = (blob, handle_out) else {
            return E_INVALIDARG;
        };
        let signature = Handle(self.objects.len());
        let document = Handle(signature.0 + 1);
        self.objects.push(Some(Object {
            status: Status::INITIAL,
            kind: Kind::Signature {
                value: blob.to_vec(),
                document,
            },
        }));
        self.objects.push(Some(Object {
            status: Status::INITIAL,
            kind: Kind::Document { signature },
        }));
        *handle_out = document;
        S_OK
    }

    /// `CryptXmlClose`. Closing a document also closes its signature. Closing a
    /// signature has no effect and returns `S_FALSE`.
    pub fn close(&mut self, handle: Handle) -> HResult {
        if handle.is_null() {
            return E_INVALIDARG;
        }
        let signature = match self.get(handle) {
            Some(Object {
                kind: Kind::Document { signature },
                ..
            }) => *signature,
            Some(Object {
                kind: Kind::Signature { .. },
                ..
            }) => return S_FALSE,
            None => return CRYPT_XML_E_HANDLE,
        };
        self.objects[signature.0] = None;
        self.objects[handle.0] = None;
        S_OK
    }

    /// `CryptXmlGetDocContext`. `out` is set to `None` when the handle is not a document.
    pub fn get_doc_context(&self, handle: Handle, out: Option<&mut Option<DocContext>>) -> HResult {
        if handle.is_null() {
            return E_INVALIDARG;
        }
        let Some(out) = out else {
            return E_INVALIDARG;
        };
        match self.get(handle) {
            Some(Object {
                kind: Kind::Document { signature },
                ..
            }) => {
                *out = Some(DocContext {
                    cb_size: DOC_CONTEXT_SIZE,
                    handle,
                    signatures: vec![*signature],
                });
                S_OK
            }
            _ => {
                *out = None;
                CRYPT_XML_E_HANDLE
            }
        }
    }

    /// `CryptXmlGetSignature`. `out` is set to `None` when the handle is not a signature.
    pub fn get_signature(&self, handle: Handle, out: Option<&mut Option<Signature>>) -> HResult {
        if handle.is_null() {
            return E_INVALIDARG;
        }
        let Some(out) = out else {
            return E_INVALIDARG;
        };
        match self.get(handle) {
            Some(Object {
                kind: Kind::Signature { value, .. },
                ..
            }) => {
                *out = Some(Signature {
                    cb_size: SIGNATURE_SIZE,
                    handle,
                    signature_value: value.clone(),
                    key_info: KeyInfo {
                        cb_size: 0,
                        verify_key_set: false,
                    },
                });
                S_OK
            }
            _ => {
                *out = None;
                CRYPT_XML_E_HANDLE
            }
        }
    }

    /// `CryptXmlGetStatus` for a document or a signature.
    pub fn get_status(&self, handle: Handle, out: Option<&mut Status>) -> HResult {
        if handle.is_null() {
            return E_INVALIDARG;
        }
        let Some(out) = out else {
            return E_INVALIDARG;
        };
        match self.get(handle) {
            Some(object) => {
                *out = object.status;
                S_OK
            }
            None => CRYPT_XML_E_HANDLE,
        }
    }

    /// `CryptXmlVerifySignature`. Every signature is reported valid, so the key and
    /// flags have no effect and are omitted. Both the signature and its document get
    /// the verified status.
    pub fn verify_signature(&mut self, handle: Handle) -> HResult {
        if handle.is_null() {
            return E_INVALIDARG;
        }
        let document = match self.get(handle) {
            Some(Object {
                kind: Kind::Signature { document, .. },
                ..
            }) => *document,
            _ => return CRYPT_XML_E_HANDLE,
        };
        self.live_mut(handle).status.mark_verified();
        self.live_mut(document).status.mark_verified();
        S_OK
    }
}
