use crate::{Hresult, E_NOTIMPL};

/// An `IUIFramework` method whose Wine body is a `FIXME` followed by `return E_NOTIMPL`. The
/// variant names are the C names without the `UIRibbonFrameworkImpl_` prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIFrameworkMethod {
    Initialize,
    Destroy,
    LoadUI,
    GetView,
    GetUICommandProperty,
    SetUICommandProperty,
    InvalidateUICommand,
    FlushPendingInvalidations,
    SetModes,
}

/// Every [`UIFrameworkMethod`], in the order of the vtable in `dlls/uiribbon/uiribbon.c`.
pub const UI_FRAMEWORK_METHODS: [UIFrameworkMethod; 9] = [
    UIFrameworkMethod::Initialize,
    UIFrameworkMethod::Destroy,
    UIFrameworkMethod::LoadUI,
    UIFrameworkMethod::GetView,
    UIFrameworkMethod::GetUICommandProperty,
    UIFrameworkMethod::SetUICommandProperty,
    UIFrameworkMethod::InvalidateUICommand,
    UIFrameworkMethod::FlushPendingInvalidations,
    UIFrameworkMethod::SetModes,
];

/// Every stub method returns `E_NOTIMPL` and has no other effect.
pub fn ui_framework_method(_method: UIFrameworkMethod) -> Hresult {
    E_NOTIMPL
}
