//! `ITaskbarList4` method results and `QueryInterface` acceptance from
//! `dlls/explorerframe/taskbarlist.c`. The window-manager messages sent by
//! `AddTab` and `DeleteTab` are not modelled.

use crate::hresult::{Hresult, E_NOINTERFACE, E_NOTIMPL, S_OK};

/// The `ITaskbarList4` methods whose result is modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskbarMethod {
    HrInit,
    AddTab,
    DeleteTab,
    ActivateTab,
    SetActiveAlt,
    MarkFullscreenWindow,
    SetProgressValue,
    SetProgressState,
    RegisterTab,
    UnregisterTab,
    SetTabOrder,
    SetTabActive,
    ThumbBarAddButtons,
    ThumbBarUpdateButtons,
    ThumbBarSetImageList,
    SetOverlayIcon,
    SetThumbnailTooltip,
    SetThumbnailClip,
    SetTabProperties,
}

impl TaskbarMethod {
    /// The `HRESULT` Wine returns. Methods that Wine leaves unimplemented return
    /// `E_NOTIMPL`. The "faking success" stubs return `S_OK`.
    pub fn result(self) -> Hresult {
        match self {
            Self::ActivateTab
            | Self::SetActiveAlt
            | Self::MarkFullscreenWindow
            | Self::SetThumbnailClip
            | Self::SetTabProperties => E_NOTIMPL,
            _ => S_OK,
        }
    }
}

/// The interface IDs `taskbar_list_QueryInterface` compares against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskbarIid {
    ITaskbarList,
    ITaskbarList2,
    ITaskbarList3,
    ITaskbarList4,
    IUnknown,
    Other,
}

/// `taskbar_list_QueryInterface`. Returns `(hr, refcount)`. A successful query
/// AddRefs the object; a failed one leaves the refcount alone.
pub fn query_interface(iid: TaskbarIid, refcount: u32) -> (Hresult, u32) {
    match iid {
        TaskbarIid::Other => (E_NOINTERFACE, refcount),
        _ => (S_OK, refcount.wrapping_add(1)),
    }
}
