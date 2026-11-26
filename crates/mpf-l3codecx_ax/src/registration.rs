//! Port of `DllRegisterServer` and `DllUnregisterServer` in `l3codecx.c`, with the `REGFILTER2`
//! descriptor that `DllRegisterServer` registers.

use crate::com::{is_failed, ComHost, Guid, Hresult, S_OK};

/// `MEDIATYPE_Audio`, from include/uuids.h.
pub const MEDIATYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MEDIASUBTYPE_MP3`: `WAVE_FORMAT_MPEGLAYER3` (0x0055) in the `DEFINE_WAVEFORMATEX_GUID` layout.
pub const MEDIASUBTYPE_MP3: Guid = Guid {
    data1: 0x0000_0055,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `MEDIASUBTYPE_PCM`: `WAVE_FORMAT_PCM` (1) in the `DEFINE_WAVEFORMATEX_GUID` layout.
pub const MEDIASUBTYPE_PCM: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `CLSID_MPEGLayer3Decoder`, the filter class this DLL registers.
pub const CLSID_MPEG_LAYER3_DECODER: Guid = Guid {
    data1: 0x38be_3000,
    data2: 0xdbf4,
    data3: 0x11d0,
    data4: [0x86, 0x0e, 0x00, 0xa0, 0x24, 0xcf, 0xef, 0x6d],
};

/// `CLSID_FilterMapper2`, from include/uuids.h.
pub const CLSID_FILTER_MAPPER2: Guid = Guid {
    data1: 0xcda4_2200,
    data2: 0xbd88,
    data3: 0x11d0,
    data4: [0xbd, 0x4e, 0x00, 0xa0, 0xc9, 0x11, 0xce, 0x86],
};

/// `IID_IFilterMapper2`, from include/axextend.idl.
pub const IID_IFILTER_MAPPER2: Guid = Guid {
    data1: 0xb79b_b0b0,
    data2: 0x33c1,
    data3: 0x11d1,
    data4: [0xab, 0xe1, 0x00, 0xa0, 0xc9, 0x05, 0xf3, 0x75],
};

/// The name passed to `RegisterFilter`.
pub const MPEG_LAYER3_FILTER_NAME: &str = "MPEG Layer-3 Decoder";

/// `REG_PINFLAG_B_OUTPUT`, from include/axextend.idl.
pub const REG_PINFLAG_B_OUTPUT: u32 = 0x8;

/// `REGPINTYPES`: a major and a minor media type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegPinTypes {
    pub major_type: Guid,
    pub minor_type: Guid,
}

/// `REGFILTERPINS2`. The C literal sets `dwFlags`, `nMediaTypes` and `lpMediaType` only, so
/// `cInstances` and `clsPinCategory` are zero. `nMediaTypes` is the length of `media_types`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegFilterPins2 {
    pub flags: u32,
    pub media_types: &'static [RegPinTypes],
}

/// `REGFILTER2` in version 2 form. `cPins2` is the length of `pins`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegFilter2 {
    pub version: u32,
    pub merit: u32,
    pub pins: &'static [RegFilterPins2],
}

/// `mpeg_layer3_decoder_desc`: an MP3 input pin and a PCM output pin.
pub const MPEG_LAYER3_DECODER_FILTER: RegFilter2 = RegFilter2 {
    version: 2,
    merit: 0x0081_0000,
    pins: &[
        RegFilterPins2 {
            flags: 0,
            media_types: &[RegPinTypes {
                major_type: MEDIATYPE_AUDIO,
                minor_type: MEDIASUBTYPE_MP3,
            }],
        },
        RegFilterPins2 {
            flags: REG_PINFLAG_B_OUTPUT,
            media_types: &[RegPinTypes {
                major_type: MEDIATYPE_AUDIO,
                minor_type: MEDIASUBTYPE_PCM,
            }],
        },
    ],
};

/// `DllRegisterServer`. A failed resource registration or mapper creation is returned as it is,
/// and the mapper is released only once it was created. The `RegisterFilter` result is returned.
pub fn dll_register_server(host: &mut impl ComHost) -> Hresult {
    let hr = host.register_resources();
    if is_failed(hr) {
        return hr;
    }
    let hr = host.co_create_instance(&CLSID_FILTER_MAPPER2, false, &IID_IFILTER_MAPPER2);
    if is_failed(hr) {
        return hr;
    }
    let hr = host.register_filter(
        &CLSID_MPEG_LAYER3_DECODER,
        MPEG_LAYER3_FILTER_NAME,
        &MPEG_LAYER3_DECODER_FILTER,
    );
    host.release_filter_mapper();
    hr
}

/// `DllUnregisterServer`. The `UnregisterFilter` result is discarded, and `S_OK` is returned once
/// the mapper was created, as the C does.
pub fn dll_unregister_server(host: &mut impl ComHost) -> Hresult {
    let hr = host.unregister_resources();
    if is_failed(hr) {
        return hr;
    }
    let hr = host.co_create_instance(&CLSID_FILTER_MAPPER2, false, &IID_IFILTER_MAPPER2);
    if is_failed(hr) {
        return hr;
    }
    host.unregister_filter(&CLSID_MPEG_LAYER3_DECODER);
    host.release_filter_mapper();
    S_OK
}
