//! Port of the portable logic in Wine's `dlls/hvsimanagementapi/host.c`: the static activation
//! factory and statics object for `IsolatedWindowsEnvironmentHost`.

/// `HRESULT`, as a signed 32-bit value.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;

/// The IIDs that `factory_QueryInterface` compares against. Any other IID is [`Iid::Other`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    IUnknown,
    IInspectable,
    IAgileObject,
    IActivationFactory,
    IIsolatedWindowsEnvironmentHostStatics,
    Other,
}

/// The interface view that a successful `QueryInterface` returns. Both views are one object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    ActivationFactory,
    IsolatedWindowsEnvironmentHostStatics,
}

/// `struct isolated_host_statics`. Its two interface views share one `LONG ref` count. The C
/// object is static, so it starts with `ref` = 1 and is never freed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsolatedHostStatics {
    ref_count: i32,
}

impl Default for IsolatedHostStatics {
    fn default() -> Self {
        Self { ref_count: 1 }
    }
}

impl IsolatedHostStatics {
    /// The current `ref` value, as the C `LONG`.
    pub fn ref_count(&self) -> i32 {
        self.ref_count
    }

    /// `factory_QueryInterface`. The statics view's `QueryInterface` forwards here too. On
    /// success the C code calls `IInspectable_AddRef(*out)`, so the count goes up by one.
    pub fn query_interface(&mut self, iid: Iid) -> Result<Interface, Hresult> {
        let view = match iid {
            Iid::IUnknown | Iid::IInspectable | Iid::IAgileObject | Iid::IActivationFactory => {
                Interface::ActivationFactory
            }
            Iid::IIsolatedWindowsEnvironmentHostStatics => {
                Interface::IsolatedWindowsEnvironmentHostStatics
            }
            Iid::Other => return Err(E_NOINTERFACE),
        };
        self.add_ref();
        Ok(view)
    }

    /// `factory_AddRef`. The count is a `LONG` updated by `InterlockedIncrement`, so it wraps,
    /// and the result is returned as a `ULONG`.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_add(1);
        self.ref_count as u32
    }

    /// `factory_Release`. The object is static and never freed, so the count can go below 0.
    /// The result is returned as a `ULONG`, so -1 becomes `0xFFFF_FFFF`.
    pub fn release(&mut self) -> u32 {
        self.ref_count = self.ref_count.wrapping_sub(1);
        self.ref_count as u32
    }

    /// `isolated_host_statics_get_IsReady`: always `S_OK`, with `value` set to `FALSE`.
    pub fn get_is_ready(&self) -> (Hresult, bool) {
        (S_OK, false)
    }
}

/// A method whose Wine body is `FIXME(...); return E_NOTIMPL;`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubMethod {
    FactoryGetIids,
    FactoryGetRuntimeClassName,
    FactoryGetTrustLevel,
    FactoryActivateInstance,
    StaticsGetHostErrors,
}

/// Every stub method returns `E_NOTIMPL` and has no other effect. The statics object's
/// `GetIids`, `GetRuntimeClassName` and `GetTrustLevel` forward to the factory's, so they use
/// the factory variants.
pub fn stub_method(_method: StubMethod) -> Hresult {
    E_NOTIMPL
}
