//! `DllRegisterServer` and `DllUnregisterServer` from `dlls/msauddecmft/msauddecmft.c`, and the
//! `MFTRegister` table of the AAC decoder MFT.

use crate::guid::{Guid, CLSID_MSAAC_DEC_MFT};
use crate::hresult::{failed, Hresult, S_OK};

/// `MFT_ENUM_FLAG_SYNCMFT`, from `include/mfapi.h`.
pub const MFT_ENUM_FLAG_SYNCMFT: u32 = 0x0000_0001;

/// `MFT_CATEGORY_AUDIO_DECODER`, from `include/mfapi.h`.
pub const MFT_CATEGORY_AUDIO_DECODER: Guid = Guid {
    data1: 0x9ea7_3fb4,
    data2: 0xef7a,
    data3: 0x4559,
    data4: [0x8D, 0x5D, 0x71, 0x9D, 0x8F, 0x04, 0x26, 0xC7],
};

/// `MFMediaType_Audio`, from `include/mfapi.h`.
pub const MF_MEDIA_TYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFAudioFormat_AAC`, from `include/mfapi.h`: `DEFINE_MEDIATYPE_GUID` with `WAVE_FORMAT_MPEG_HEAAC`.
pub const MF_AUDIO_FORMAT_AAC: Guid = Guid {
    data1: 0x0000_1610,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFAudioFormat_RAW_AAC`, defined in `dlls/msauddecmft/msauddecmft.c` with `WAVE_FORMAT_RAW_AAC1`.
pub const MF_AUDIO_FORMAT_RAW_AAC: Guid = Guid {
    data1: 0x0000_00FF,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFAudioFormat_ADTS`, from `include/mfapi.h`: `DEFINE_MEDIATYPE_GUID` with `WAVE_FORMAT_MPEG_ADTS_AAC`.
pub const MF_AUDIO_FORMAT_ADTS: Guid = Guid {
    data1: 0x0000_1600,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFAudioFormat_Float`, from `include/mfapi.h`: `DEFINE_MEDIATYPE_GUID` with `WAVE_FORMAT_IEEE_FLOAT`.
pub const MF_AUDIO_FORMAT_FLOAT: Guid = Guid {
    data1: 0x0000_0003,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFAudioFormat_PCM`, from `include/mfapi.h`: `DEFINE_MEDIATYPE_GUID` with `WAVE_FORMAT_PCM`.
pub const MF_AUDIO_FORMAT_PCM: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFT_REGISTER_TYPE_INFO`, from `include/mfobjects.idl`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MftRegisterTypeInfo {
    pub major_type: Guid,
    pub subtype: Guid,
}

/// The arguments of the `MFTRegister` call in `DllRegisterServer`. The `attributes` argument is always NULL,
/// so it is not modelled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MftRegistration {
    pub clsid: Guid,
    pub category: Guid,
    pub name: &'static str,
    pub flags: u32,
    pub inputs: &'static [MftRegisterTypeInfo],
    pub outputs: &'static [MftRegisterTypeInfo],
}

/// `aac_decoder_mft_inputs`, in C order.
const AAC_DECODER_MFT_INPUTS: &[MftRegisterTypeInfo] = &[
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_AUDIO,
        subtype: MF_AUDIO_FORMAT_AAC,
    },
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_AUDIO,
        subtype: MF_AUDIO_FORMAT_RAW_AAC,
    },
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_AUDIO,
        subtype: MF_AUDIO_FORMAT_ADTS,
    },
];

/// `aac_decoder_mft_outputs`, in C order.
const AAC_DECODER_MFT_OUTPUTS: &[MftRegisterTypeInfo] = &[
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_AUDIO,
        subtype: MF_AUDIO_FORMAT_FLOAT,
    },
    MftRegisterTypeInfo {
        major_type: MF_MEDIA_TYPE_AUDIO,
        subtype: MF_AUDIO_FORMAT_PCM,
    },
];

/// The `MFTRegister` arguments that `DllRegisterServer` passes.
pub const AAC_DECODER_REGISTRATION: MftRegistration = MftRegistration {
    clsid: CLSID_MSAAC_DEC_MFT,
    category: MFT_CATEGORY_AUDIO_DECODER,
    name: "Microsoft AAC Audio Decoder MFT",
    flags: MFT_ENUM_FLAG_SYNCMFT,
    inputs: AAC_DECODER_MFT_INPUTS,
    outputs: AAC_DECODER_MFT_OUTPUTS,
};

/// `DllRegisterServer`. `register_resources` is `__wine_register_resources`, and `mft_register` is
/// `MFTRegister` given [`AAC_DECODER_REGISTRATION`]. A failed step returns its own `HRESULT` and skips the
/// rest. Otherwise the result is `S_OK`.
pub fn dll_register_server(
    register_resources: impl FnOnce() -> Hresult,
    mft_register: impl FnOnce(&MftRegistration) -> Hresult,
) -> Hresult {
    let hr = register_resources();
    if failed(hr) {
        return hr;
    }
    let hr = mft_register(&AAC_DECODER_REGISTRATION);
    if failed(hr) {
        return hr;
    }
    S_OK
}

/// `DllUnregisterServer`. `unregister_resources` is `__wine_unregister_resources`, and `mft_unregister` is
/// `MFTUnregister` given `CLSID_MSAACDecMFT`. The failure handling is the same as in
/// [`dll_register_server`].
pub fn dll_unregister_server(
    unregister_resources: impl FnOnce() -> Hresult,
    mft_unregister: impl FnOnce(&Guid) -> Hresult,
) -> Hresult {
    let hr = unregister_resources();
    if failed(hr) {
        return hr;
    }
    let hr = mft_unregister(&CLSID_MSAAC_DEC_MFT);
    if failed(hr) {
        return hr;
    }
    S_OK
}
