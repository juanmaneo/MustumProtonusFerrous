use crate::{E_FAIL, E_INVALIDARG, E_NOTIMPL, HRESULT};

/// A `D3DFORMAT` value. FOURCC formats are the little-endian packing of their four characters.
pub type D3dFormat = u32;

/// `MAKEFOURCC(a, b, c, d)`.
pub const fn fourcc(code: [u8; 4]) -> D3dFormat {
    u32::from_le_bytes(code)
}

/// `D3DFMT_A8R8G8B8`.
pub const A8R8G8B8: D3dFormat = 21;
/// `D3DFMT_X8R8G8B8`.
pub const X8R8G8B8: D3dFormat = 22;
/// `D3DFMT_YUY2`.
pub const YUY2: D3dFormat = fourcc(*b"YUY2");
/// `D3DFMT_NV12`.
pub const NV12: D3dFormat = fourcc(*b"NV12");

const D3DPOOL_DEFAULT: u32 = 0;
const D3DPOOL_SYSTEMMEM: u32 = 2;

const DXVA2_VPDEV_HARDWARE_DEVICE: u32 = 0x0001;
const DXVA2_VPDEV_SOFTWARE_DEVICE: u32 = 0x0004;

const DXVA2_VIDEO_PROCESS_YUV2RGB: u32 = 0x0001;
const DXVA2_VIDEO_PROCESS_STRETCH_X: u32 = 0x0002;
const DXVA2_VIDEO_PROCESS_STRETCH_Y: u32 = 0x0004;
const DXVA2_VIDEO_PROCESS_SUB_RECTS: u32 = 0x0010;
const DXVA2_VIDEO_PROCESS_SUB_STREAMS: u32 = 0x0020;
const DXVA2_VIDEO_PROCESS_SUB_STREAMS_EXTENDED: u32 = 0x0040;
const DXVA2_VIDEO_PROCESS_YUV2RGB_EXTENDED: u32 = 0x0080;
const DXVA2_VIDEO_PROCESS_PLANAR_ALPHA: u32 = 0x1000;

/// The video processor devices that the service knows, identified by the GUIDs
/// `DXVA2_VideoProcProgressiveDevice` and `DXVA2_VideoProcSoftwareDevice`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Device {
    Progressive,
    Software,
}

const SOFTWARE_INPUT_FORMATS: [D3dFormat; 3] = [A8R8G8B8, X8R8G8B8, YUY2];
const PROGRESSIVE_INPUT_FORMATS: [D3dFormat; 4] = [A8R8G8B8, X8R8G8B8, YUY2, NV12];

impl Device {
    fn input_formats(self) -> &'static [D3dFormat] {
        match self {
            Device::Progressive => &PROGRESSIVE_INPUT_FORMATS,
            Device::Software => &SOFTWARE_INPUT_FORMATS,
        }
    }
}

/// The `DXVA2_VideoProcessorCaps` fields that the C tables set. The other fields are zero in
/// both tables.
#[derive(Debug, PartialEq, Eq)]
pub struct ProcessorCaps {
    pub device_caps: u32,
    pub input_pool: u32,
    pub operations: u32,
}

/// `GetVideoProcessorDeviceGuids`: the devices that accept `format` as the input stream
/// format, in table order. Only the format is read from the `DXVA2_VideoDesc`. `E_FAIL` when
/// no device accepts it.
pub fn device_guids(format: D3dFormat) -> Result<Vec<Device>, HRESULT> {
    let devices: Vec<Device> = [Device::Progressive, Device::Software]
        .into_iter()
        .filter(|device| device.input_formats().contains(&format))
        .collect();

    if devices.is_empty() {
        Err(E_FAIL)
    } else {
        Ok(devices)
    }
}

/// `GetVideoProcessorRenderTargets`: the two render target formats for `device` and
/// `format`. `None` stands for a device GUID that is neither processor device, which returns
/// `E_NOTIMPL`. An input format that the device does not accept returns `E_FAIL`.
pub fn render_target_formats(
    device: Option<Device>,
    format: D3dFormat,
) -> Result<[D3dFormat; 2], HRESULT> {
    let device = device.ok_or(E_NOTIMPL)?;

    if !device.input_formats().contains(&format) {
        return Err(E_FAIL);
    }

    Ok(match device {
        Device::Progressive => [X8R8G8B8, NV12],
        Device::Software => [X8R8G8B8, A8R8G8B8],
    })
}

fn caps_of(device: Device) -> ProcessorCaps {
    match device {
        Device::Progressive => ProcessorCaps {
            device_caps: DXVA2_VPDEV_HARDWARE_DEVICE,
            input_pool: D3DPOOL_DEFAULT,
            operations: DXVA2_VIDEO_PROCESS_YUV2RGB
                | DXVA2_VIDEO_PROCESS_STRETCH_X
                | DXVA2_VIDEO_PROCESS_STRETCH_Y,
        },
        Device::Software => ProcessorCaps {
            device_caps: DXVA2_VPDEV_SOFTWARE_DEVICE,
            input_pool: D3DPOOL_SYSTEMMEM,
            operations: DXVA2_VIDEO_PROCESS_PLANAR_ALPHA
                | DXVA2_VIDEO_PROCESS_YUV2RGB
                | DXVA2_VIDEO_PROCESS_STRETCH_X
                | DXVA2_VIDEO_PROCESS_STRETCH_Y
                | DXVA2_VIDEO_PROCESS_SUB_RECTS
                | DXVA2_VIDEO_PROCESS_SUB_STREAMS
                | DXVA2_VIDEO_PROCESS_SUB_STREAMS_EXTENDED
                | DXVA2_VIDEO_PROCESS_YUV2RGB_EXTENDED,
        },
    }
}

/// `device_manager_processor_service_GetVideoProcessorCaps`. `None` returns `E_NOTIMPL`.
pub fn service_caps(device: Option<Device>) -> Result<ProcessorCaps, HRESULT> {
    device.map(caps_of).ok_or(E_NOTIMPL)
}

/// `video_processor_GetVideoProcessorCaps`, for a processor created on `device`. `None`
/// returns `E_FAIL`.
pub fn processor_caps(device: Option<Device>) -> Result<ProcessorCaps, HRESULT> {
    device.map(caps_of).ok_or(E_FAIL)
}

/// The number of surfaces that `CreateSurface` makes: one per back buffer, and one more for
/// the front buffer. `E_INVALIDARG` when that count overflows.
pub fn surface_count(backbuffers: u32) -> Result<u32, HRESULT> {
    backbuffers.checked_add(1).ok_or(E_INVALIDARG)
}

/// The creation loop of `CreateSurface`. Each call of `create` makes one surface. On the first
/// failure, the surfaces made so far are dropped and the error is returned.
pub fn create_surfaces<T>(
    backbuffers: u32,
    mut create: impl FnMut() -> Result<T, HRESULT>,
) -> Result<Vec<T>, HRESULT> {
    let count = surface_count(backbuffers)?;
    (0..count).map(|_| create()).collect()
}

/// The substream limit in `CreateVideoProcessor`. The C body does not check the device GUID,
/// so neither does this check.
pub fn check_substreams(max_substreams: u32) -> Result<(), HRESULT> {
    if max_substreams >= 16 {
        Err(E_INVALIDARG)
    } else {
        Ok(())
    }
}
