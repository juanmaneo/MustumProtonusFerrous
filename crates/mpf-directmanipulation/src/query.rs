//! `QueryInterface` routing: the interface pointer each object hands out for an IID.

use crate::class::Factory;
use crate::guid::{classify_iid, Guid, Iid};
use crate::hresult::{HResult, E_NOINTERFACE};
use crate::refcount::RefCount;

/// The object a vtable belongs to. The Content interface and the FrameInfoProvider interface
/// are reached through `PrimaryContext` and `Compositor` respectively, because they share a count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Object {
    Manager,
    UpdateManager,
    PrimaryContext,
    Viewport,
    Compositor,
}

/// An interface pointer returned by `QueryInterface`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iface {
    Manager,
    UpdateManager,
    PrimaryContent,
    Content,
    Viewport,
    Compositor,
    FrameInfoProvider,
    ClassFactory(Factory),
}

pub fn query_interface(object: Object, iid: &Guid) -> Option<Iface> {
    match (object, classify_iid(iid)?) {
        (Object::Manager, Iid::Unknown | Iid::Manager | Iid::Manager2) => Some(Iface::Manager),
        (Object::UpdateManager, Iid::Unknown | Iid::UpdateManager) => Some(Iface::UpdateManager),
        (Object::PrimaryContext, Iid::Unknown | Iid::PrimaryContent) => Some(Iface::PrimaryContent),
        (Object::PrimaryContext, Iid::Content) => Some(Iface::Content),
        (Object::Viewport, Iid::Unknown | Iid::Viewport | Iid::Viewport2) => Some(Iface::Viewport),
        (Object::Compositor, Iid::Unknown | Iid::Compositor | Iid::Compositor2) => {
            Some(Iface::Compositor)
        }
        (Object::Compositor, Iid::FrameInfoProvider) => Some(Iface::FrameInfoProvider),
        _ => None,
    }
}

/// `QueryInterface` on a counted object: the count rises only when the IID is supported.
pub fn query_counted(object: Object, iid: &Guid, refs: &mut RefCount) -> Result<Iface, HResult> {
    let iface = query_interface(object, iid).ok_or(E_NOINTERFACE)?;
    refs.add_ref();
    Ok(iface)
}
