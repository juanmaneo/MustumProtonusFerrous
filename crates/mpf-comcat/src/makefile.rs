/// `MODULE` in `dlls/comcat/Makefile.in`.
pub const MODULE: &str = "comcat.dll";

/// `IMPORTS` in `dlls/comcat/Makefile.in`.
pub const IMPORTS: &str = "ole32";

/// `VER_OLESELFREGISTER` in `dlls/comcat/Makefile.in`. `tools/makedep.c` passes it on
/// through its `version_vars` list.
pub const VER_OLESELFREGISTER: &str = "1";

/// The value `dlls/comcat/Makefile.in` assigns to the make variable `name`, or `None` when it does
/// not assign it. `tools/makedep.c` forwards a version variable only when the Makefile sets it.
pub fn makefile_var(name: &str) -> Option<&'static str> {
    match name {
        "MODULE" => Some(MODULE),
        "IMPORTS" => Some(IMPORTS),
        "VER_OLESELFREGISTER" => Some(VER_OLESELFREGISTER),
        _ => None,
    }
}
