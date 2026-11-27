/// The `CTL3D16_is_auto_subclass` flag. Wine keeps it in a process-wide static; this type holds
/// the same single bit per instance, and it starts FALSE as the static does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ctl3d {
    auto_subclass: bool,
}

impl Ctl3d {
    /// `Ctl3dAutoSubclass`, ordinal 16. Sets the flag and returns TRUE. `hinst` is ignored.
    pub fn auto_subclass(&mut self, _hinst: u16) -> bool {
        self.auto_subclass = true;
        true
    }

    /// `Ctl3dAutoSubclassEx`, ordinal 27. Sets the flag and returns TRUE. `hinst` and `ty` are ignored.
    pub fn auto_subclass_ex(&mut self, _hinst: u16, _ty: u32) -> bool {
        self.auto_subclass = true;
        true
    }

    /// `Ctl3dUnAutoSubclass`, ordinal 24. Clears the flag and returns FALSE, as the C body does.
    pub fn un_auto_subclass(&mut self) -> bool {
        self.auto_subclass = false;
        false
    }

    /// `Ctl3dUnregister`, ordinal 13. Clears the flag and returns TRUE. `hinst` is ignored.
    pub fn unregister(&mut self, _hinst: u16) -> bool {
        self.auto_subclass = false;
        true
    }

    /// `Ctl3dIsAutoSubclass`, ordinal 23.
    pub fn is_auto_subclass(&self) -> bool {
        self.auto_subclass
    }
}
