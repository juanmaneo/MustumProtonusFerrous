use crate::status::Status;

/// `GdipComment` from `dlls/gdiplus/graphics.c`: traces the call and returns `NotImplemented`.
pub fn gdip_comment() -> Status {
    Status::NotImplemented
}

/// `GdipWindingModeOutline` from `dlls/gdiplus/graphicspath.c`: returns `NotImplemented`.
pub fn gdip_winding_mode_outline() -> Status {
    Status::NotImplemented
}

/// `GdipBitmapConvertFormat` from `dlls/gdiplus/image.c`: returns `NotImplemented`.
pub fn gdip_bitmap_convert_format() -> Status {
    Status::NotImplemented
}

/// `GdipEmfToWmfBits` from `dlls/gdiplus/metafile.c`: returns `NotImplemented`.
pub fn gdip_emf_to_wmf_bits() -> Status {
    Status::NotImplemented
}

/// `GdipRecordMetafileFileNameI` from `dlls/gdiplus/metafile.c`: returns `NotImplemented`.
pub fn gdip_record_metafile_file_name_i() -> Status {
    Status::NotImplemented
}

/// `GdipConvertToEmfPlusToFile` from `dlls/gdiplus/metafile.c`: returns `NotImplemented`.
pub fn gdip_convert_to_emf_plus_to_file() -> Status {
    Status::NotImplemented
}
