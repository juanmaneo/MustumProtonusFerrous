use crate::guid::{
    Guid, CLSID_WMA_DEC_MEDIA_OBJECT, DMO_CATEGORY_AUDIO_DECODER, MFT_CATEGORY_AUDIO_DECODER,
};
use crate::hresult::{failed, HResult, S_OK};

/// `MFT_ENUM_FLAG_SYNCMFT` from `include/mfapi.h`.
pub const MFT_ENUM_FLAG_SYNCMFT: u32 = 0x0000_0001;

/// `MFMediaType_Audio` and `MEDIATYPE_Audio`, which have the same GUID.
pub const MEDIA_TYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MFAudioFormat_PCM` and `MEDIASUBTYPE_PCM`.
pub const SUBTYPE_PCM: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MFAudioFormat_Float` and `MEDIASUBTYPE_IEEE_FLOAT`.
pub const SUBTYPE_IEEE_FLOAT: Guid = Guid {
    data1: 0x0000_0003,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MEDIASUBTYPE_MSAUDIO1`.
pub const SUBTYPE_MSAUDIO1: Guid = Guid {
    data1: 0x0000_0160,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MFAudioFormat_WMAudioV8` and `MEDIASUBTYPE_WMAUDIO2`.
pub const SUBTYPE_WMAUDIO2: Guid = Guid {
    data1: 0x0000_0161,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MFAudioFormat_WMAudioV9` and `MEDIASUBTYPE_WMAUDIO3`.
pub const SUBTYPE_WMAUDIO3: Guid = Guid {
    data1: 0x0000_0162,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MFAudioFormat_WMAudio_Lossless` and `MEDIASUBTYPE_WMAUDIO_LOSSLESS`.
pub const SUBTYPE_WMAUDIO_LOSSLESS: Guid = Guid {
    data1: 0x0000_0163,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// A (major type, subtype) pair: `MFT_REGISTER_TYPE_INFO` and `DMO_PARTIAL_MEDIATYPE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediaType {
    pub major: Guid,
    pub subtype: Guid,
}

/// The arguments `DllRegisterServer` passes to `MFTRegister`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MftRegistration {
    pub clsid: Guid,
    pub category: Guid,
    pub name: &'static str,
    pub flags: u32,
    pub inputs: &'static [MediaType],
    pub outputs: &'static [MediaType],
}

/// The arguments `DllRegisterServer` passes to `DMORegister`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DmoRegistration {
    pub name: &'static str,
    pub clsid: Guid,
    pub category: Guid,
    pub flags: u32,
    pub inputs: &'static [MediaType],
    pub outputs: &'static [MediaType],
}

/// The input types of the MFT and of the DMO. The C source writes the MFT list with the
/// `MFAudioFormat_*` names and the DMO list with the `MEDIASUBTYPE_*` names, but the GUIDs match.
pub const WMA_DECODER_INPUTS: [MediaType; 4] = [
    MediaType {
        major: MEDIA_TYPE_AUDIO,
        subtype: SUBTYPE_MSAUDIO1,
    },
    MediaType {
        major: MEDIA_TYPE_AUDIO,
        subtype: SUBTYPE_WMAUDIO2,
    },
    MediaType {
        major: MEDIA_TYPE_AUDIO,
        subtype: SUBTYPE_WMAUDIO3,
    },
    MediaType {
        major: MEDIA_TYPE_AUDIO,
        subtype: SUBTYPE_WMAUDIO_LOSSLESS,
    },
];

/// The output types of the MFT and of the DMO.
pub const WMA_DECODER_OUTPUTS: [MediaType; 2] = [
    MediaType {
        major: MEDIA_TYPE_AUDIO,
        subtype: SUBTYPE_PCM,
    },
    MediaType {
        major: MEDIA_TYPE_AUDIO,
        subtype: SUBTYPE_IEEE_FLOAT,
    },
];

/// The MFT that `DllRegisterServer` registers.
pub const WMA_DECODER_MFT: MftRegistration = MftRegistration {
    clsid: CLSID_WMA_DEC_MEDIA_OBJECT,
    category: MFT_CATEGORY_AUDIO_DECODER,
    name: "WMAudio Decoder MFT",
    flags: MFT_ENUM_FLAG_SYNCMFT,
    inputs: &WMA_DECODER_INPUTS,
    outputs: &WMA_DECODER_OUTPUTS,
};

/// The DMO that `DllRegisterServer` registers.
pub const WMA_DECODER_DMO: DmoRegistration = DmoRegistration {
    name: "WMAudio Decoder DMO",
    clsid: CLSID_WMA_DEC_MEDIA_OBJECT,
    category: DMO_CATEGORY_AUDIO_DECODER,
    flags: 0,
    inputs: &WMA_DECODER_INPUTS,
    outputs: &WMA_DECODER_OUTPUTS,
};

/// `DllRegisterServer`. The resource registration, then the MFT, then the DMO. Each step
/// runs only if the previous one did not fail. The first failing `HRESULT` is returned.
pub fn dll_register_server(
    register_resources: impl FnOnce() -> HResult,
    mft_register: impl FnOnce(&MftRegistration) -> HResult,
    dmo_register: impl FnOnce(&DmoRegistration) -> HResult,
) -> HResult {
    let hr = register_resources();
    if failed(hr) {
        return hr;
    }
    let hr = mft_register(&WMA_DECODER_MFT);
    if failed(hr) {
        return hr;
    }
    let hr = dmo_register(&WMA_DECODER_DMO);
    if failed(hr) {
        return hr;
    }
    S_OK
}

/// `DllUnregisterServer`. The same step order and short-circuit rule as
/// [`dll_register_server`], with the clsid and the DMO category as the arguments.
pub fn dll_unregister_server(
    unregister_resources: impl FnOnce() -> HResult,
    mft_unregister: impl FnOnce(&Guid) -> HResult,
    dmo_unregister: impl FnOnce(&Guid, &Guid) -> HResult,
) -> HResult {
    let hr = unregister_resources();
    if failed(hr) {
        return hr;
    }
    let hr = mft_unregister(&CLSID_WMA_DEC_MEDIA_OBJECT);
    if failed(hr) {
        return hr;
    }
    let hr = dmo_unregister(&CLSID_WMA_DEC_MEDIA_OBJECT, &DMO_CATEGORY_AUDIO_DECODER);
    if failed(hr) {
        return hr;
    }
    S_OK
}
