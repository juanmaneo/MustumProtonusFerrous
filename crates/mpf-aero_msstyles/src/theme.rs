/// `PACKTHEM_VERSION` in `dlls/aero.msstyles/aero.rc`, the theme format version. It equals
/// `MSSTYLES_VERSION` in `dlls/uxtheme/msstyles.c`.
pub const PACKTHEM_VERSION: u16 = 0x0003;

/// `VER_FILEDESCRIPTION_STR` in `dlls/aero.msstyles/Makefile.in`.
pub const VER_FILEDESCRIPTION_STR: &str = "Aero Theme";

/// `VER_PRODUCTVERSION` in `dlls/aero.msstyles/Makefile.in`.
pub const VER_PRODUCTVERSION: [u16; 4] = [1, 0, 0, 1];

/// Whether `data`, the contents of a `PACKTHEM_VERSION` resource, is accepted. `msstyles.c` requires
/// exactly two bytes, reads them as a little-endian `WORD`, and compares the value with the version.
pub fn is_supported_version(data: &[u8]) -> bool {
    data.len() == 2 && u16::from_le_bytes([data[0], data[1]]) == PACKTHEM_VERSION
}
