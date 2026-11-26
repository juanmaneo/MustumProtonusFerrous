use crate::com::{failed, Guid, Hresult, CLSID_NULLRENDERER, CLSID_SAMPLEGRABBER, GUID_NULL, S_OK};

/// `MERIT_DO_NOT_USE`, from `include/axextend.idl`.
pub const MERIT_DO_NOT_USE: u32 = 0x0020_0000;

/// `REG_PINFLAG_B_OUTPUT`, from `include/axextend.idl`.
pub const REG_PINFLAG_B_OUTPUT: u32 = 0x8;

/// `REGPINTYPES`: the major and minor media types of a pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegPinTypes {
    pub major: Guid,
    pub minor: Guid,
}

/// `REGFILTERPINS2`. `nMediaTypes` is the length of `media_types`. `clsPinCategory` is always
/// NULL in the C tables, so it is not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegFilterPins2 {
    pub flags: u32,
    pub instances: u32,
    pub media_types: &'static [RegPinTypes],
}

/// `REGFILTER2`. `cPins2` is the length of `pins`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegFilter2 {
    pub version: u32,
    pub merit: u32,
    pub pins: &'static [RegFilterPins2],
}

const NULL_MEDIA_TYPES: &[RegPinTypes] = &[RegPinTypes {
    major: GUID_NULL,
    minor: GUID_NULL,
}];

const SAMPLE_GRABBER_PINS: &[RegFilterPins2] = &[
    RegFilterPins2 {
        flags: 0,
        instances: 1,
        media_types: NULL_MEDIA_TYPES,
    },
    RegFilterPins2 {
        flags: REG_PINFLAG_B_OUTPUT,
        instances: 1,
        media_types: NULL_MEDIA_TYPES,
    },
];

/// `reg_sample_grabber`, registered under the name `SampleGrabber`.
pub const SAMPLE_GRABBER_FILTER: RegFilter2 = RegFilter2 {
    version: 2,
    merit: MERIT_DO_NOT_USE,
    pins: SAMPLE_GRABBER_PINS,
};

const NULL_RENDERER_PINS: &[RegFilterPins2] = &[RegFilterPins2 {
    flags: REG_PINFLAG_B_OUTPUT,
    instances: 1,
    media_types: NULL_MEDIA_TYPES,
}];

/// `reg_null_renderer`, registered under the name `Null Renderer`.
pub const NULL_RENDERER_FILTER: RegFilter2 = RegFilter2 {
    version: 2,
    merit: MERIT_DO_NOT_USE,
    pins: NULL_RENDERER_PINS,
};

/// The `IFilterMapper2` calls that the registration functions make. The C code calls
/// `IFilterMapper2_Release` on the mapper last. The port drops it instead.
pub trait FilterMapper {
    /// `RegisterFilter` with a NULL moniker, category and instance. The C code ignores its result.
    fn register_filter(&mut self, clsid: &Guid, name: &str, filter: &RegFilter2);

    /// `UnregisterFilter` with a NULL moniker and category. The C code ignores its result.
    fn unregister_filter(&mut self, clsid: &Guid);
}

/// `DllRegisterServer`. `resources` stands for `__wine_register_resources`. `create_mapper`
/// stands for `CoCreateInstance(CLSID_FilterMapper2)`: `Err` carries a failed HRESULT, and `Ok`
/// means any HRESULT that does not fail.
pub fn dll_register_server<M: FilterMapper>(
    resources: Hresult,
    create_mapper: impl FnOnce() -> Result<M, Hresult>,
) -> Hresult {
    with_mapper(resources, create_mapper, |mapper| {
        mapper.register_filter(
            &CLSID_SAMPLEGRABBER,
            "SampleGrabber",
            &SAMPLE_GRABBER_FILTER,
        );
        mapper.register_filter(&CLSID_NULLRENDERER, "Null Renderer", &NULL_RENDERER_FILTER);
    })
}

/// `DllUnregisterServer`. The arguments mean the same as for [`dll_register_server`].
pub fn dll_unregister_server<M: FilterMapper>(
    resources: Hresult,
    create_mapper: impl FnOnce() -> Result<M, Hresult>,
) -> Hresult {
    with_mapper(resources, create_mapper, |mapper| {
        mapper.unregister_filter(&CLSID_SAMPLEGRABBER);
        mapper.unregister_filter(&CLSID_NULLRENDERER);
    })
}

fn with_mapper<M>(
    resources: Hresult,
    create_mapper: impl FnOnce() -> Result<M, Hresult>,
    update: impl FnOnce(&mut M),
) -> Hresult {
    if failed(resources) {
        return resources;
    }
    let mut mapper = match create_mapper() {
        Ok(mapper) => mapper,
        Err(hr) => return hr,
    };
    update(&mut mapper);
    S_OK
}
