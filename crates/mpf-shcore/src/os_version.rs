//! `IsOS` in Wine's `dlls/shcore/main.c`, over the version facts the caller reads from the OS.

/// `VER_PLATFORM_WIN32s`, the platform of Win32s.
pub const VER_PLATFORM_WIN32S: u32 = 0;
/// `VER_PLATFORM_WIN32_WINDOWS`, the platform of Windows 95 and its successors.
pub const VER_PLATFORM_WIN32_WINDOWS: u32 = 1;
/// `VER_PLATFORM_WIN32_NT`, the platform of Windows NT and later.
pub const VER_PLATFORM_WIN32_NT: u32 = 2;

const OS_WIN32SORGREATER: u32 = 0x00;
const OS_NT: u32 = 0x01;
const OS_WIN95ORGREATER: u32 = 0x02;
const OS_NT4ORGREATER: u32 = 0x03;
const OS_WIN2000ORGREATER_ALT: u32 = 0x04;
const OS_WIN98ORGREATER: u32 = 0x05;
const OS_WIN98_GOLD: u32 = 0x06;
const OS_WIN2000ORGREATER: u32 = 0x07;
const OS_WIN2000PRO: u32 = 0x08;
const OS_WIN2000SERVER: u32 = 0x09;
const OS_WIN2000ADVSERVER: u32 = 0x0A;
const OS_WIN2000DATACENTER: u32 = 0x0B;
const OS_WIN2000TERMINAL: u32 = 0x0C;
const OS_EMBEDDED: u32 = 0x0D;
const OS_TERMINALCLIENT: u32 = 0x0E;
const OS_TERMINALREMOTEADMIN: u32 = 0x0F;
const OS_WIN95_GOLD: u32 = 0x10;
const OS_MEORGREATER: u32 = 0x11;
const OS_XPORGREATER: u32 = 0x12;
const OS_HOME: u32 = 0x13;
const OS_PROFESSIONAL: u32 = 0x14;
const OS_DATACENTER: u32 = 0x15;
const OS_ADVSERVER: u32 = 0x16;
const OS_SERVER: u32 = 0x17;
const OS_TERMINALSERVER: u32 = 0x18;
const OS_PERSONALTERMINALSERVER: u32 = 0x19;
const OS_FASTUSERSWITCHING: u32 = 0x1A;
const OS_WELCOMELOGONUI: u32 = 0x1B;
const OS_DOMAINMEMBER: u32 = 0x1C;
const OS_ANYSERVER: u32 = 0x1D;
const OS_WOW6432: u32 = 0x1E;
const OS_WEBSERVER: u32 = 0x1F;
const OS_SMALLBUSINESSSERVER: u32 = 0x20;
const OS_TABLETPC: u32 = 0x21;
const OS_SERVERADMINUI: u32 = 0x22;
const OS_MEDIACENTER: u32 = 0x23;
const OS_APPLIANCE: u32 = 0x24;
const OS_VISTAORGREATER: u32 = 0x25;

/// The version facts that `IsOS` reads: `GetVersionExA`, `IsWow64Process` and `GetSystemMetrics`.
pub struct OsVersion {
    pub platform: u32,
    pub major: u32,
    pub minor: u32,
    pub wow64: bool,
    pub tablet_pc: bool,
    pub media_center: bool,
}

/// `IsOS`: whether the system matches the feature. Features that Wine marks `FIXME` return the
/// same value as Wine, and unknown features return false.
pub fn is_os(feature: u32, os: &OsVersion) -> bool {
    let (platform, major, minor) = (os.platform, os.major, os.minor);
    let nt = platform == VER_PLATFORM_WIN32_NT;
    let win9x = platform == VER_PLATFORM_WIN32_WINDOWS;
    match feature {
        OS_WIN32SORGREATER => platform == VER_PLATFORM_WIN32S || win9x,
        OS_NT => nt,
        OS_WIN95ORGREATER => win9x,
        OS_NT4ORGREATER => nt && major >= 4,
        OS_WIN2000ORGREATER_ALT | OS_WIN2000ORGREATER | OS_WIN2000PRO => nt && major >= 5,
        OS_WIN98ORGREATER => win9x && minor >= 10,
        OS_WIN98_GOLD => win9x && minor == 10,
        OS_WIN2000SERVER | OS_WIN2000ADVSERVER | OS_WIN2000DATACENTER | OS_WIN2000TERMINAL => {
            nt && (minor == 0 || minor == 1)
        }
        OS_EMBEDDED | OS_TERMINALCLIENT | OS_TERMINALREMOTEADMIN => false,
        OS_WIN95_GOLD => win9x && minor == 0,
        OS_MEORGREATER => win9x && minor >= 90,
        OS_XPORGREATER | OS_HOME => nt && major >= 5 && minor >= 1,
        OS_PROFESSIONAL | OS_DATACENTER | OS_SERVER | OS_TERMINALSERVER => nt,
        OS_ADVSERVER => nt && major >= 5,
        OS_PERSONALTERMINALSERVER => nt && minor >= 1 && major >= 5,
        OS_FASTUSERSWITCHING | OS_DOMAINMEMBER => true,
        OS_WELCOMELOGONUI | OS_SERVERADMINUI | OS_APPLIANCE => false,
        OS_ANYSERVER | OS_WEBSERVER | OS_SMALLBUSINESSSERVER => nt,
        OS_WOW6432 => os.wow64,
        OS_TABLETPC => os.tablet_pc,
        OS_MEDIACENTER => os.media_center,
        OS_VISTAORGREATER => nt && major >= 6,
        _ => false,
    }
}
