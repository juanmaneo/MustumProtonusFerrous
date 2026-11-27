/// LANG_ENGLISH from include/winnt.rh.
pub const LANG_ENGLISH: u16 = 0x09;

/// SUBLANG_DEFAULT from include/winnt.rh.
pub const SUBLANG_DEFAULT: u16 = 0x01;

/// MAKELANGID from include/winnt.h. The sublanguage goes above the 10-bit primary language.
pub const fn makelangid(primary: u16, sublang: u16) -> u16 {
    (sublang << 10) | primary
}

/// The LANGUAGE LANG_ENGLISH, SUBLANG_DEFAULT statement of shdoclc.rc. It sets the language of
/// every resource in the DLL.
pub const LANG_ID: u16 = makelangid(LANG_ENGLISH, SUBLANG_DEFAULT);
