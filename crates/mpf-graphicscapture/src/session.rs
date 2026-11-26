//! The session object from `dlls/graphicscapture/session.c`. Its `IActivationFactory` and
//! `IGraphicsCaptureSessionStatics` interfaces share one reference count.

/// `S_OK` from `include/winerror.h`.
pub const S_OK: u32 = 0;

/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: u32 = 0x8000_4002;

/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: u32 = 0x8000_4001;

/// A GUID with the field layout of Wine's `GUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IID_IUnknown`, the `uuid` of `IUnknown` in `include/unknwn.idl`.
pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IInspectable`, from `include/inspectable.idl`.
pub const IID_INSPECTABLE: Guid = Guid {
    data1: 0xaf86_e2e0,
    data2: 0xb12d,
    data3: 0x4c6a,
    data4: [0x9c, 0x5a, 0xd7, 0xaa, 0x65, 0x10, 0x1e, 0x90],
};

/// `IID_IAgileObject`, from `include/objidlbase.idl`.
pub const IID_AGILE_OBJECT: Guid = Guid {
    data1: 0x94ea_2b94,
    data2: 0xe9cc,
    data3: 0x49e0,
    data4: [0xc0, 0xff, 0xee, 0x64, 0xca, 0x8f, 0x5b, 0x90],
};

/// `IID_IActivationFactory`, from `include/activation.idl`.
pub const IID_ACTIVATION_FACTORY: Guid = Guid {
    data1: 0x0000_0035,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IGraphicsCaptureSessionStatics`, from `include/windows.graphics.capture.idl`.
pub const IID_GRAPHICS_CAPTURE_SESSION_STATICS: Guid = Guid {
    data1: 0x2224_a540,
    data2: 0x5974,
    data3: 0x49aa,
    data4: [0xb2, 0x32, 0x08, 0x82, 0x53, 0x6f, 0x4c, 0xb5],
};

/// The interface pointers that [`Session::query_interface`] can hand out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    /// `IActivationFactory`, the session's `IActivationFactory_iface`.
    ActivationFactory,
    /// `IGraphicsCaptureSessionStatics`, the session's `IGraphicsCaptureSessionStatics_iface`.
    SessionStatics,
}

/// `struct session` from `dlls/graphicscapture/session.c`, without its vtable pointers. The C
/// object is a static whose reference count starts at 0.
#[derive(Debug, Default)]
pub struct Session {
    ref_count: i32,
}

impl Session {
    /// `factory_QueryInterface`. The statics interface's `QueryInterface` delegates to it.
    pub fn query_interface(&mut self, iid: &Guid, out: &mut Option<Interface>) -> u32 {
        if *iid == IID_UNKNOWN
            || *iid == IID_INSPECTABLE
            || *iid == IID_AGILE_OBJECT
            || *iid == IID_ACTIVATION_FACTORY
        {
            *out = Some(Interface::ActivationFactory);
            self.add_ref();
            return S_OK;
        }
        if *iid == IID_GRAPHICS_CAPTURE_SESSION_STATICS {
            *out = Some(Interface::SessionStatics);
            self.add_ref();
            return S_OK;
        }
        *out = None;
        E_NOINTERFACE
    }

    /// `factory_AddRef`. The statics interface's `AddRef` delegates to it.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `factory_Release`. The statics interface's `Release` delegates to it. A count below zero
    /// is returned as the unsigned value, so it shows as `0xffffffff` at -1.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }

    /// `factory_GetIids`. The statics interface's `GetIids` delegates to it.
    pub fn get_iids(&self) -> u32 {
        E_NOTIMPL
    }

    /// `factory_GetRuntimeClassName`. The statics interface's method delegates to it.
    pub fn get_runtime_class_name(&self) -> u32 {
        E_NOTIMPL
    }

    /// `factory_GetTrustLevel`. The statics interface's method delegates to it.
    pub fn get_trust_level(&self) -> u32 {
        E_NOTIMPL
    }

    /// `factory_ActivateInstance`.
    pub fn activate_instance(&self) -> u32 {
        E_NOTIMPL
    }
}

/// `session_statics_IsSupported`: the session reports that capture is not supported.
pub fn is_supported(result: &mut bool) -> u32 {
    *result = false;
    S_OK
}
