//! The driver context and the `DriverProc` replies that need no decoder: the `DRV_OPEN` type
//! check, `CRAM_GetInfo` and the constant messages.

use crate::abi::{
    compare_fourcc, CRAM_MAGIC, DRV_LOAD, DRV_OPEN, ICERR_BADFORMAT, ICERR_OK, ICERR_UNSUPPORTED,
    ICINFO_SIZE, ICM_COMPRESS_QUERY, ICM_DECOMPRESS, ICM_DECOMPRESSEX, ICM_DECOMPRESS_BEGIN,
    ICM_DECOMPRESS_END, ICM_DECOMPRESS_GET_FORMAT, ICM_DECOMPRESS_QUERY, ICM_GETINFO, ICTYPE_VIDEO,
    ICVERSION,
};
use crate::{IcError, IcResult, LResult};

/// One open driver instance: `Msvideo1Context` in `msvideo1.c`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Msvideo1Context {
    /// The signature that every decode message checks. A valid context holds [CRAM_MAGIC].
    pub magic: u32,
    /// The output depth that [Msvideo1Context::decompress_begin] chose: 8, 16, or 0 when the
    /// output format is unsupported.
    pub depth: i32,
}

impl Msvideo1Context {
    /// The context that `DRV_OPEN` creates: the CRAM signature, with no depth chosen yet.
    pub const fn opened() -> Self {
        Self {
            magic: CRAM_MAGIC,
            depth: 0,
        }
    }

    pub(crate) fn check(&self) -> IcResult<()> {
        if self.magic == CRAM_MAGIC {
            Ok(())
        } else {
            Err(IcError::BadParam)
        }
    }
}

/// `DRV_OPEN`: whether the `ICINFO` that the caller passes asks for a video codec. Without an
/// `ICINFO`, the driver opens.
pub fn open_accepts(info: Option<&IcInfo>) -> bool {
    match info {
        Some(info) => compare_fourcc(info.fcc_type, ICTYPE_VIDEO) == 0,
        None => true,
    }
}

/// The fields of `ICINFO` that `CRAM_GetInfo` sets. The name, the description and the driver
/// path come from resources and from msvfw32, so they are not included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IcInfo {
    pub dw_size: u32,
    pub fcc_type: u32,
    pub fcc_handler: u32,
    pub flags: u32,
    pub version: u32,
    pub version_icm: u32,
}

/// The reply of `CRAM_GetInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoResult {
    /// No buffer was passed. The reply is the size of `ICINFO`, [ICINFO_SIZE].
    SizeQuery,
    /// The buffer is smaller than `ICINFO`. The reply is 0.
    TooSmall,
    /// The buffer is large enough. The reply is [ICINFO_SIZE], and the buffer holds this info.
    Filled(IcInfo),
}

/// `CRAM_GetInfo`. `buffer_size` is `None` when no `ICINFO` buffer was passed, and otherwise the
/// buffer's size in bytes. The handler FOURCC is the context's signature, or `CRAM` without a
/// context.
pub fn get_info(context: Option<&Msvideo1Context>, buffer_size: Option<u32>) -> InfoResult {
    let Some(size) = buffer_size else {
        return InfoResult::SizeQuery;
    };
    if size < ICINFO_SIZE {
        return InfoResult::TooSmall;
    }
    InfoResult::Filled(IcInfo {
        dw_size: ICINFO_SIZE,
        fcc_type: ICTYPE_VIDEO,
        fcc_handler: context.map_or(CRAM_MAGIC, |context| context.magic),
        flags: 0,
        version: ICVERSION,
        version_icm: ICVERSION,
    })
}

/// The reply of `CRAM_DriverProc` to a message that needs no decoder work.
///
/// Returns `None` for the messages that the other functions of this crate answer: `DRV_OPEN`,
/// `ICM_GETINFO` and the decode, query and format messages. Every other message has a fixed
/// reply: `DRV_LOAD` gives 1, `ICM_DECOMPRESS_END` gives [ICERR_OK], `ICM_COMPRESS_QUERY` gives
/// [ICERR_BADFORMAT], and anything else, including Wine's stubs and unknown messages, gives
/// [ICERR_UNSUPPORTED].
pub fn driver_proc_constant_result(msg: u32) -> Option<LResult> {
    match msg {
        DRV_LOAD => Some(1),
        ICM_DECOMPRESS_END => Some(ICERR_OK),
        ICM_COMPRESS_QUERY => Some(ICERR_BADFORMAT),
        DRV_OPEN
        | ICM_GETINFO
        | ICM_DECOMPRESS_QUERY
        | ICM_DECOMPRESS_GET_FORMAT
        | ICM_DECOMPRESS_BEGIN
        | ICM_DECOMPRESS
        | ICM_DECOMPRESSEX => None,
        _ => Some(ICERR_UNSUPPORTED),
    }
}
