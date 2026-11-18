use crate::{failed, pixel_format_bpp, Guid, E_FAIL, WINCODEC_ERR_WRONGSTATE};

const CAN_DECODE_ALL_IMAGES: u32 = 0x2;
const CAN_DECODE_SOME_IMAGES: u32 = 0x4;
const CAN_ENUMERATE_METADATA: u32 = 0x8;

/// The decoder calls that wmp_decoder_initialize makes on the JXR decoder object.
pub trait JxrHost {
    /// Seeks the stream to its start and returns the HRESULT of the seek.
    fn rewind(&mut self) -> i32;
    /// Returns false when the JXR decoder rejects the stream.
    fn initialize(&mut self) -> bool;
    fn frame_count(&mut self) -> Option<u32>;
    fn select_first_frame(&mut self) -> bool;
    fn pixel_format(&mut self) -> Option<Guid>;
    fn size(&mut self) -> Option<Dimensions>;
    fn resolution(&mut self) -> Option<Resolution>;
    fn color_context_size(&mut self) -> Option<u32>;
}

/// Decoder status, as in struct decoder_stat.
#[derive(Debug)]
pub struct DecoderStat {
    pub flags: u32,
    pub frame_count: u32,
}

/// Frame description, as in struct decoder_frame without the palette.
#[derive(Debug)]
pub struct FrameInfo {
    pub pixel_format: Guid,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub dpix: f64,
    pub dpiy: f64,
    pub num_colors: u32,
    pub num_color_contexts: u32,
}

#[derive(Debug)]
pub struct Initialized {
    pub stat: DecoderStat,
    pub frame: FrameInfo,
}

/// Frame size as reported by the JXR decoder; the C code stores the signed values as UINT.
#[derive(Clone, Copy, Debug)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    pub dpix: f32,
    pub dpiy: f32,
}

/// Decoder state: whether the stream has been bound by an earlier initialize call.
#[derive(Debug, Default)]
pub struct WmpDecoder {
    stream_bound: bool,
}

impl WmpDecoder {
    /// Binds the stream and reads frame 0, as wmp_decoder_initialize does. A second call is refused.
    pub fn initialize(&mut self, host: &mut impl JxrHost) -> Result<Initialized, i32> {
        if self.stream_bound {
            return Err(WINCODEC_ERR_WRONGSTATE);
        }
        self.stream_bound = true;
        let rewind = host.rewind();
        if failed(rewind) {
            return Err(rewind);
        }
        if !host.initialize() {
            return Err(E_FAIL);
        }
        // Wine reads the count and then reports one frame, since multi-frame JPEG-XR is not implemented.
        host.frame_count().ok_or(E_FAIL)?;
        if !host.select_first_frame() {
            return Err(E_FAIL);
        }
        let format = host.pixel_format().ok_or(E_FAIL)?;
        let size = host.size().ok_or(E_FAIL)?;
        let resolution = host.resolution().ok_or(E_FAIL)?;
        let color_contexts = host.color_context_size().ok_or(E_FAIL)?;
        let bpp = pixel_format_bpp(&format);
        if bpp == 0 {
            return Err(E_FAIL);
        }
        Ok(Initialized {
            stat: DecoderStat {
                flags: CAN_DECODE_ALL_IMAGES | CAN_DECODE_SOME_IMAGES | CAN_ENUMERATE_METADATA,
                frame_count: 1,
            },
            frame: FrameInfo {
                pixel_format: format,
                width: size.width as u32,
                height: size.height as u32,
                bpp,
                dpix: f64::from(resolution.dpix),
                dpiy: f64::from(resolution.dpiy),
                num_colors: 0,
                num_color_contexts: u32::from(color_contexts != 0),
            },
        })
    }
}
