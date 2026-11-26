//! The `struct ttsengine` object of `dlls/msttsengine/tts.c`, with its `ISpTTSEngine` and `ISpObjectWithToken`
//! vtables.

use crate::com::{
    is_equal_guid, ComPtr, Guid, Hresult, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY,
    E_POINTER, IID_SP_OBJECT_WITH_TOKEN, IID_SP_TTS_ENGINE, IID_UNKNOWN, SPERR_ALREADY_INITIALIZED,
    S_FALSE, S_OK,
};

/// The interface that a successful `QueryInterface` returns. The C returns a pointer to one of the two
/// interface members of `struct ttsengine`, and the port names the member instead.
#[derive(Debug, PartialEq, Eq)]
pub enum Interface {
    /// The `ISpTTSEngine_iface` member.
    SpTtsEngine,
    /// The `ISpObjectWithToken_iface` member.
    SpObjectWithToken,
}

/// `struct ttsengine`. `ref_count` is the `LONG ref` that both interfaces share. `token` is the
/// `ISpObjectToken *` that `SetObjectToken` stored, or NULL.
pub struct TtsEngine<T: ComPtr> {
    pub(crate) ref_count: i32,
    pub(crate) token: Option<T>,
}

impl<T: ComPtr> TtsEngine<T> {
    /// `ttsengine_QueryInterface`. `objwithtoken_QueryInterface` forwards here too. `IUnknown` and `ISpTTSEngine`
    /// give [`Interface::SpTtsEngine`], and `ISpObjectWithToken` gives [`Interface::SpObjectWithToken`]. Any
    /// other IID gives `E_NOINTERFACE` and takes no reference. A success takes a reference, as the C's
    /// `IUnknown_AddRef` does.
    pub fn query_interface(&mut self, riid: &Guid) -> Result<Interface, Hresult> {
        let iface = if is_equal_guid(riid, &IID_UNKNOWN) || is_equal_guid(riid, &IID_SP_TTS_ENGINE)
        {
            Interface::SpTtsEngine
        } else if is_equal_guid(riid, &IID_SP_OBJECT_WITH_TOKEN) {
            Interface::SpObjectWithToken
        } else {
            return Err(E_NOINTERFACE);
        };
        self.add_ref();
        Ok(iface)
    }

    /// `ttsengine_AddRef`, which both vtables share. Returns the new count. The count wraps on overflow, as
    /// `InterlockedIncrement` does.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `ttsengine_Release`, which both vtables share. Returns the new count. At zero it releases the token. The
    /// C then calls `free`, and the caller of this port drops the value instead.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        if self.ref_count == 0 {
            if let Some(token) = self.token.take() {
                token.release();
            }
        }
        self.ref_count as u32
    }

    /// `objwithtoken_SetObjectToken`. `None` gives `E_INVALIDARG`, and a token that is already set gives
    /// `SPERR_ALREADY_INITIALIZED`. Otherwise the object takes a reference on the token and stores it.
    pub fn set_object_token(&mut self, token: Option<T>) -> Hresult {
        let Some(token) = token else {
            return E_INVALIDARG;
        };
        if self.token.is_some() {
            return SPERR_ALREADY_INITIALIZED;
        }
        token.add_ref();
        self.token = Some(token);
        S_OK
    }

    /// `objwithtoken_GetObjectToken`. `out` is the `ISpObjectToken **` argument, and `None` is NULL, which gives
    /// `E_POINTER`. `*out` is always set to the stored token. A stored token takes a reference and gives `S_OK`.
    /// With no token stored the result is `S_FALSE`.
    pub fn get_object_token(&self, out: Option<&mut Option<T>>) -> Hresult {
        let Some(out) = out else {
            return E_POINTER;
        };
        *out = self.token;
        if let Some(token) = self.token {
            token.add_ref();
            return S_OK;
        }
        S_FALSE
    }
}

/// `ttsengine_create`. `malloc_ok` is the result of `malloc(sizeof(*This))`, and `false` gives `E_OUTOFMEMORY`.
/// The new object starts with one reference. The C then calls `QueryInterface` and `Release` on it, so a
/// successful query leaves one reference, held by the caller. A failed query drops the object, because
/// `Release` frees it at zero. `Ok` is the object and the interface that `riid` selected. `Err` is the
/// `HRESULT` that the C returns with `*obj` NULL.
pub fn ttsengine_create<T: ComPtr>(
    riid: &Guid,
    malloc_ok: bool,
) -> Result<(TtsEngine<T>, Interface), Hresult> {
    if !malloc_ok {
        return Err(E_OUTOFMEMORY);
    }
    let mut engine = TtsEngine {
        ref_count: 1,
        token: None,
    };
    let iface = engine.query_interface(riid);
    engine.release();
    Ok((engine, iface?))
}

/// `ttsengine_Speak`: not implemented. The C ignores its arguments and returns `E_NOTIMPL`.
pub fn speak() -> Hresult {
    E_NOTIMPL
}

/// `ttsengine_GetOutputFormat`: not implemented. The C ignores its arguments and returns `E_NOTIMPL`.
pub fn get_output_format() -> Hresult {
    E_NOTIMPL
}
