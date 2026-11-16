//! gdi32 subset, mirroring `dlls/gdi32/` in reference Wine — headless:
//! device contexts and GDI objects are in-memory records with fixed
//! device metrics until a real graphics driver (dxvk companion) lands.

use mpf_ntdll::object::ObjectManager;
use std::collections::HashMap;
use std::sync::Mutex;

/// Device context handle (0 = NULL/screen DC base).
pub type Hdc = u64;

/// GetDeviceCaps indices we answer.
pub const HORZRES: i32 = 8;
pub const VERTRES: i32 = 10;
pub const BITSPIXEL: i32 = 12;
pub const LOGPIXELSX: i32 = 88;
pub const LOGPIXELSY: i32 = 90;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdiObjectKind {
    Bitmap,
    Brush,
    Pen,
    Font,
}

#[derive(Debug, Clone)]
struct GdiObject {
    kind: GdiObjectKind,
    width: i32,
    height: i32,
    bpp: i32,
}

struct GdiState {
    om: ObjectManager,
    dcs: HashMap<Hdc, Hdc>, // dc -> selected object (0 = none)
    objects: HashMap<Hdc, GdiObject>,
}

static STATE: Mutex<Option<GdiState>> = Mutex::new(None);

fn with_state<R>(f: impl FnOnce(&mut GdiState) -> R) -> R {
    let mut guard = STATE.lock().unwrap();
    f(guard.get_or_insert(GdiState {
        om: ObjectManager::new(),
        dcs: HashMap::new(),
        objects: HashMap::new(),
    }))
}

/// GetDeviceCaps: fixed headless metrics (1920x1080x32 @ 96 DPI).
pub fn get_device_caps(_hdc: Hdc, index: i32) -> i32 {
    match index {
        HORZRES => 1920,
        VERTRES => 1080,
        BITSPIXEL => 32,
        LOGPIXELSX | LOGPIXELSY => 96,
        _ => 0,
    }
}

/// CreateCompatibleDC: returns a new memory DC handle.
pub fn create_compatible_dc(_hdc: Hdc) -> Hdc {
    with_state(|st| {
        let h = st.om.allocate_handle(10);
        st.dcs.insert(h, 0);
        h
    })
}

/// CreateBitmap: creates an in-memory bitmap object, returns its handle.
pub fn create_bitmap(width: i32, height: i32, bpp: i32) -> Hdc {
    if width <= 0 || height <= 0 || bpp <= 0 {
        return 0;
    }
    with_state(|st| {
        let h = st.om.allocate_handle(11);
        st.objects.insert(
            h,
            GdiObject {
                kind: GdiObjectKind::Bitmap,
                width,
                height,
                bpp,
            },
        );
        h
    })
}

/// SelectObject: selects `obj` into `dc`, returns the previous selection.
pub fn select_object(dc: Hdc, obj: Hdc) -> Hdc {
    with_state(|st| match st.dcs.get_mut(&dc) {
        Some(slot) => std::mem::replace(slot, obj),
        None => 0,
    })
}

/// DeleteObject: deletes a GDI object; true on success.
pub fn delete_object(obj: Hdc) -> bool {
    with_state(|st| match st.objects.remove(&obj) {
        Some(_) => {
            st.om.close_handle(obj);
            true
        }
        None => false,
    })
}

/// Queries a GDI object's kind and bitmap geometry (width/height/bpp are 0
/// for non-bitmaps).
pub fn object_info(obj: Hdc) -> Option<(GdiObjectKind, i32, i32, i32)> {
    with_state(|st| {
        st.objects
            .get(&obj)
            .map(|o| (o.kind, o.width, o.height, o.bpp))
    })
}

/// DeleteDC.
pub fn delete_dc(dc: Hdc) -> bool {
    with_state(|st| match st.dcs.remove(&dc) {
        Some(_) => {
            st.om.close_handle(dc);
            true
        }
        None => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_caps_fixed_metrics() {
        assert_eq!(get_device_caps(0, HORZRES), 1920);
        assert_eq!(get_device_caps(0, VERTRES), 1080);
        assert_eq!(get_device_caps(0, BITSPIXEL), 32);
        assert_eq!(get_device_caps(0, LOGPIXELSX), 96);
        assert_eq!(get_device_caps(0, LOGPIXELSY), 96);
        assert_eq!(get_device_caps(0, 9999), 0);
    }

    #[test]
    fn dc_and_object_lifecycle() {
        let dc = create_compatible_dc(0);
        assert_ne!(dc, 0);
        let bmp = create_bitmap(64, 64, 32);
        assert_ne!(bmp, 0);
        assert_eq!(select_object(dc, bmp), 0);
        assert_eq!(select_object(dc, 0), bmp);
        assert!(delete_object(bmp));
        assert!(!delete_object(bmp));
        assert!(delete_dc(dc));
        assert!(!delete_dc(dc));
        assert_eq!(select_object(9999, bmp), 0);
    }

    #[test]
    fn object_info_reports_geometry() {
        let bmp = create_bitmap(32, 16, 24);
        assert_eq!(object_info(bmp), Some((GdiObjectKind::Bitmap, 32, 16, 24)));
        assert_eq!(object_info(9999), None);
        assert!(delete_object(bmp));
    }

    #[test]
    fn invalid_bitmap_params_fail() {
        assert_eq!(create_bitmap(0, 10, 32), 0);
        assert_eq!(create_bitmap(10, -1, 32), 0);
        assert_eq!(create_bitmap(10, 10, 0), 0);
    }
}
