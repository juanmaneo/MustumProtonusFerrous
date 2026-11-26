//! The DVD graph builder from Wine's `dlls/qdvd/graph.c`. The object is created without an outer
//! `IUnknown`, so its `IDvdGraphBuilder` methods delegate to the inner `IUnknown`.

use crate::guid::{is_equal_guid, Guid, IID_IDVDGRAPHBUILDER, IID_IUNKNOWN};
use crate::hresult::{Hresult, E_NOINTERFACE, E_NOTIMPL};

/// The interfaces `inner_QueryInterface` hands out.
#[derive(PartialEq, Eq)]
pub enum GraphBuilderInterface {
    Inner,
    DvdGraphBuilder,
}

/// `struct graph_builder`, without the allocation and the outer object.
pub struct GraphBuilder {
    refcount: i32,
}

impl GraphBuilder {
    /// `graph_builder_create` with no outer object: the new builder holds one reference.
    pub fn create() -> Self {
        Self { refcount: 1 }
    }

    /// The reference count, as the `ULONG` the C code returns.
    pub fn refcount(&self) -> u32 {
        self.refcount as u32
    }

    /// `inner_AddRef`.
    pub fn inner_add_ref(&mut self) -> u32 {
        self.refcount = self.refcount.wrapping_add(1);
        self.refcount()
    }

    /// `inner_Release`. The C code frees the object when the count reaches 0. The port returns
    /// the count, and the owner drops the value.
    pub fn inner_release(&mut self) -> u32 {
        self.refcount = self.refcount.wrapping_sub(1);
        self.refcount()
    }

    /// `inner_QueryInterface`. A successful query adds a reference to the interface it returns.
    pub fn inner_query_interface(&mut self, iid: &Guid) -> Result<GraphBuilderInterface, Hresult> {
        let interface = if is_equal_guid(iid, &IID_IUNKNOWN) {
            GraphBuilderInterface::Inner
        } else if is_equal_guid(iid, &IID_IDVDGRAPHBUILDER) {
            GraphBuilderInterface::DvdGraphBuilder
        } else {
            return Err(E_NOINTERFACE);
        };
        self.inner_add_ref();
        Ok(interface)
    }

    /// `graph_builder_AddRef`, which forwards to the outer object: the inner `IUnknown` here.
    pub fn dvd_add_ref(&mut self) -> u32 {
        self.inner_add_ref()
    }

    /// `graph_builder_Release`, which forwards to the outer object.
    pub fn dvd_release(&mut self) -> u32 {
        self.inner_release()
    }

    /// `graph_builder_QueryInterface`, which forwards to the outer object.
    pub fn dvd_query_interface(&mut self, iid: &Guid) -> Result<GraphBuilderInterface, Hresult> {
        self.inner_query_interface(iid)
    }
}

/// `graph_builder_GetFiltergraph`, which returns `E_NOTIMPL`.
pub fn get_filtergraph() -> Hresult {
    E_NOTIMPL
}

/// `graph_builder_GetDvdInterface`, which returns `E_NOTIMPL`.
pub fn get_dvd_interface() -> Hresult {
    E_NOTIMPL
}

/// `graph_builder_RenderDvdVideoVolume`, which returns `E_NOTIMPL`.
pub fn render_dvd_video_volume() -> Hresult {
    E_NOTIMPL
}
