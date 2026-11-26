use crate::com::Guid;

/// The four coclasses that `dlls/uianimation/uianimation_reg.idl` registers, in the order
/// `DllGetClassObject` in `dlls/uianimation/main.c` tests them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassKind {
    Manager,
    Timer,
    TransitionFactory,
    TransitionLibrary,
}

/// `CLSID_UIAnimationManager`, from `include/uianimation.idl`.
pub const CLSID_UIANIMATION_MANAGER: Guid = 0x4c1fc63a695c47e8a3391a194be3d0b8;
/// `CLSID_UIAnimationTimer`, from `include/uianimation.idl`.
pub const CLSID_UIANIMATION_TIMER: Guid = 0xbfcd4a0c06b64384b7680daa792c380e;
/// `CLSID_UIAnimationTransitionFactory`, from `include/uianimation.idl`.
pub const CLSID_UIANIMATION_TRANSITION_FACTORY: Guid = 0x8a9b1cddfcd7419c8b4442fd17db1887;
/// `CLSID_UIAnimationTransitionLibrary`, from `include/uianimation.idl`.
pub const CLSID_UIANIMATION_TRANSITION_LIBRARY: Guid = 0x1d6322adaa854ef5a82886d71067d145;

/// `IID_IUIAnimationManager`, from `include/uianimation.idl`.
pub const IID_IUIANIMATION_MANAGER: Guid = 0x9169896cac8d4e7d94e567fa4dc2f2e8;
/// `IID_IUIAnimationTimer`, from `include/uianimation.idl`.
pub const IID_IUIANIMATION_TIMER: Guid = 0x6b0efad1a05341d6908533a689144665;
/// `IID_IUIAnimationTransitionFactory`, from `include/uianimation.idl`.
pub const IID_IUIANIMATION_TRANSITION_FACTORY: Guid = 0xfcd91e033e3b45adbbb16dfc8153743d;
/// `IID_IUIAnimationTransitionLibrary`, from `include/uianimation.idl`.
pub const IID_IUIANIMATION_TRANSITION_LIBRARY: Guid = 0xca5a14b1d24f48b88fe4c78169ba954e;

/// The class that `clsid` names, or `None` when `DllGetClassObject` does not serve it.
pub fn class_for_clsid(clsid: Guid) -> Option<ClassKind> {
    if clsid == CLSID_UIANIMATION_MANAGER {
        Some(ClassKind::Manager)
    } else if clsid == CLSID_UIANIMATION_TIMER {
        Some(ClassKind::Timer)
    } else if clsid == CLSID_UIANIMATION_TRANSITION_FACTORY {
        Some(ClassKind::TransitionFactory)
    } else if clsid == CLSID_UIANIMATION_TRANSITION_LIBRARY {
        Some(ClassKind::TransitionLibrary)
    } else {
        None
    }
}

/// The interface IID that objects of `kind` expose as their own.
pub fn class_iid(kind: ClassKind) -> Guid {
    match kind {
        ClassKind::Manager => IID_IUIANIMATION_MANAGER,
        ClassKind::Timer => IID_IUIANIMATION_TIMER,
        ClassKind::TransitionFactory => IID_IUIANIMATION_TRANSITION_FACTORY,
        ClassKind::TransitionLibrary => IID_IUIANIMATION_TRANSITION_LIBRARY,
    }
}
