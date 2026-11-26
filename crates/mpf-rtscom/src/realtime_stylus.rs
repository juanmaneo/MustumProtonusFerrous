use crate::com::{is_equal_guid, Guid, Interface, IID_IREALTIMESTYLUS, IID_IUNKNOWN};
use crate::{Hresult, E_NOINTERFACE, S_OK};

/// `struct rtstylus`. `rtstylus_Release` frees the object when the count reaches zero. In the
/// port, the caller drops the object at that point.
pub struct RealTimeStylus {
    ref_count: u32,
}

impl RealTimeStylus {
    /// `rtstylus_QueryInterface`. `IUnknown` and `IRealTimeStylus` take a reference.
    pub fn query_interface(&mut self, iid: &Guid, obj: &mut Option<Interface>) -> Hresult {
        if is_equal_guid(iid, &IID_IUNKNOWN) || is_equal_guid(iid, &IID_IREALTIMESTYLUS) {
            self.add_ref();
            *obj = Some(Interface::RealTimeStylus);
            return S_OK;
        }
        *obj = None;
        E_NOINTERFACE
    }

    /// `rtstylus_AddRef`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count += 1;
        self.ref_count
    }

    /// `rtstylus_Release`. The caller drops the object when this returns 0, and must not release
    /// it again after that.
    pub fn release(&mut self) -> u32 {
        self.ref_count -= 1;
        self.ref_count
    }
}

/// `realtime_stylus_create`. The object starts with one reference, and it survives only if the
/// IID query took a reference of its own.
pub fn realtime_stylus_create(iid: &Guid) -> (Hresult, Option<RealTimeStylus>) {
    let mut stylus = RealTimeStylus { ref_count: 1 };
    let mut obj = None;
    let hr = stylus.query_interface(iid, &mut obj);
    if stylus.release() == 0 {
        (hr, None)
    } else {
        (hr, Some(stylus))
    }
}
