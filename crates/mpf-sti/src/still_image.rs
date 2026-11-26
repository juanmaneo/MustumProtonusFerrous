//! The object that `StiCreateInstanceW` creates, with its two interface pointers: the inner
//! `IUnknown` ([`Face::Inner`]) and `IStillImageW` ([`Face::StillImageW`]). The inner object
//! counts the references. The `IStillImageW` face forwards to the outer object, which is the
//! inner object when `pUnkOuter` was NULL.

use crate::com::{
    Guid, Hresult, E_NOINTERFACE, E_OUTOFMEMORY, IID_ISTILLIMAGEW, IID_IUNKNOWN,
    STG_E_UNIMPLEMENTEDFUNCTION, S_OK,
};

/// The interface pointer an `sti` object hands out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Inner,
    StillImageW,
}

/// The `pUnkOuter` object. Only the results of its three methods are modelled: the pointer
/// it writes on a successful `QueryInterface` belongs to it and is not tracked.
pub trait OuterUnknown {
    fn query_interface(&mut self, riid: &Guid) -> Hresult;
    fn add_ref(&mut self) -> u32;
    fn release(&mut self) -> u32;
}

/// The `stillimage` struct from `dlls/sti/sti.c`, without the vtable pointers.
pub struct StillImage {
    ref_count: u32,
    outer: Option<Box<dyn OuterUnknown>>,
}

impl StillImage {
    pub fn ref_count(&self) -> u32 {
        self.ref_count
    }

    /// `Internal_QueryInterface`. A successful query AddRefs the pointer it returns.
    pub fn inner_query_interface(&mut self, riid: &Guid) -> Result<Face, Hresult> {
        let face = if *riid == IID_IUNKNOWN {
            Face::Inner
        } else if *riid == IID_ISTILLIMAGEW {
            Face::StillImageW
        } else {
            return Err(E_NOINTERFACE);
        };
        self.add_ref(face);
        Ok(face)
    }

    /// `QueryInterface` through `face`. The `IStillImageW` face forwards to the outer object;
    /// `Ok(None)` is a successful forwarded query whose pointer is not tracked.
    pub fn query_interface(&mut self, face: Face, riid: &Guid) -> Result<Option<Face>, Hresult> {
        match self.delegate(face) {
            Some(outer) => {
                let hr = outer.query_interface(riid);
                if hr == S_OK {
                    Ok(None)
                } else {
                    Err(hr)
                }
            }
            None => self.inner_query_interface(riid).map(Some),
        }
    }

    /// `AddRef` through `face`. Returns the new count of the object that was AddRef'd.
    pub fn add_ref(&mut self, face: Face) -> u32 {
        match self.delegate(face) {
            Some(outer) => outer.add_ref(),
            None => self.internal_add_ref(),
        }
    }

    /// `Release` through `face`. A count of 0 means the C code frees the object; the caller
    /// drops the value. Releasing an object whose count is already 0 is a use after free in C.
    pub fn release(&mut self, face: Face) -> u32 {
        match self.delegate(face) {
            Some(outer) => outer.release(),
            None => self.internal_release(),
        }
    }

    /// The outer object that `face` forwards to, or `None` when it is handled by the inner
    /// object itself.
    fn delegate(&mut self, face: Face) -> Option<&mut (dyn OuterUnknown + '_)> {
        match face {
            Face::StillImageW => self
                .outer
                .as_deref_mut()
                .map(|outer| outer as &mut dyn OuterUnknown),
            Face::Inner => None,
        }
    }

    fn internal_add_ref(&mut self) -> u32 {
        self.ref_count += 1;
        self.ref_count
    }

    fn internal_release(&mut self) -> u32 {
        self.ref_count -= 1;
        self.ref_count
    }
}

/// `StiCreateInstanceW`. `alloc_ok` is the `HeapAlloc` result. `IStillImage_Initialize`
/// (`stillimagew_Initialize`) always returns `S_OK`, so its `SUCCEEDED` check always passes and
/// is not modelled. `hinst` and `dwVersion` only reach `Initialize`, which ignores them.
pub fn sti_create_instance_w(
    outer: Option<Box<dyn OuterUnknown>>,
    alloc_ok: bool,
) -> Result<(StillImage, Face), Hresult> {
    if !alloc_ok {
        return Err(E_OUTOFMEMORY);
    }
    let face = if outer.is_some() {
        Face::Inner
    } else {
        Face::StillImageW
    };
    Ok((
        StillImage {
            ref_count: 1,
            outer,
        },
        face,
    ))
}

/// `StiCreateInstanceA`: unimplemented on Windows Vista too, so it returns
/// `STG_E_UNIMPLEMENTEDFUNCTION`.
pub fn sti_create_instance_a() -> Hresult {
    STG_E_UNIMPLEMENTEDFUNCTION
}
