//! Oracle-derived test vectors for the modelled difxapi stubs.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of
//! `dlls/difxapi/main.c` and the `DIFXAPI` typedefs from `include/difxapi.h`
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/difxapi/`
//! and running the harness. All 23 printed rows are used. ASan and UBSan
//! flagged none of them, so none is excluded.
//!
//! The log-callback rows print `calls=0`. The port takes the callback as an
//! [`Opaque`] that nothing can invoke, so those tests only check the call.

use super::*;

fn utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn oracle_preinstall_rows() {
    // P1 preinstall_a inf=c:\a.inf flags=0 => ret=0
    assert_eq!(driver_package_preinstall_a(Some(&b"c:\\a.inf"[..]), 0), 0);
    // P2 preinstall_a inf=(null) flags=4294967295 => ret=0
    assert_eq!(driver_package_preinstall_a(None, 0xffff_ffff), 0);
    // P3 preinstall_w inf=c:\b.inf flags=1 => ret=0
    assert_eq!(driver_package_preinstall_w(Some(&utf16("c:\\b.inf")), 1), 0);
    // P4 preinstall_w inf=(null) flags=2147483648 => ret=0
    assert_eq!(driver_package_preinstall_w(None, 0x8000_0000), 0);
}

#[test]
fn oracle_install_rows() {
    let mut reboot = 1;
    // I1 install_a inf=x.inf flags=1 info=(null) reboot_in=1 => ret=0 reboot=0
    assert_eq!(
        driver_package_install_a(Some(&b"x.inf"[..]), 1, None, Some(&mut reboot)),
        0
    );
    assert_eq!(reboot, 0);
    // I2 install_a inf=x.inf flags=0 info=(null) reboot=(null) => ret=0
    assert_eq!(
        driver_package_install_a(Some(&b"x.inf"[..]), 0, None, None),
        0
    );
    let mut reboot = -1;
    // I3 install_a inf=(null) flags=4294967295 info=set reboot_in=-1 => ret=0 reboot=0
    assert_eq!(
        driver_package_install_a(None, 0xffff_ffff, Some(&Opaque), Some(&mut reboot)),
        0
    );
    assert_eq!(reboot, 0);
    let mut reboot = 1;
    // I4 install_w inf=x.inf flags=2 info=(null) reboot_in=1 => ret=0 reboot=0
    assert_eq!(
        driver_package_install_w(Some(&utf16("x.inf")), 2, None, Some(&mut reboot)),
        0
    );
    assert_eq!(reboot, 0);
    // I5 install_w inf=(null) flags=0 info=set reboot=(null) => ret=0
    assert_eq!(driver_package_install_w(None, 0, Some(&Opaque), None), 0);
}

#[test]
fn oracle_uninstall_rows() {
    let mut reboot = 1;
    // U1 uninstall_a inf=y.inf flags=4 info=(null) reboot_in=1 => ret=0 reboot=0
    assert_eq!(
        driver_package_uninstall_a(Some(&b"y.inf"[..]), 4, None, Some(&mut reboot)),
        0
    );
    assert_eq!(reboot, 0);
    // U2 uninstall_a inf=y.inf flags=0 info=(null) reboot=(null) => ret=0
    assert_eq!(
        driver_package_uninstall_a(Some(&b"y.inf"[..]), 0, None, None),
        0
    );
    let mut reboot = -1;
    // U3 uninstall_a inf=(null) flags=4294967295 info=set reboot_in=-1 => ret=0 reboot=0
    assert_eq!(
        driver_package_uninstall_a(None, 0xffff_ffff, Some(&Opaque), Some(&mut reboot)),
        0
    );
    assert_eq!(reboot, 0);
    let mut reboot = 1;
    // U4 uninstall_w inf=y.inf flags=8 info=(null) reboot_in=1 => ret=0 reboot=0
    assert_eq!(
        driver_package_uninstall_w(Some(&utf16("y.inf")), 8, None, Some(&mut reboot)),
        0
    );
    assert_eq!(reboot, 0);
    // U5 uninstall_w inf=(null) flags=0 info=(null) reboot=(null) => ret=0
    assert_eq!(driver_package_uninstall_w(None, 0, None, None), 0);
}

#[test]
fn oracle_get_path_rows() {
    let mut dest = *b"ABCDEFG\0";
    let mut count = 8;
    // G1 getpath_a inf=z.inf count_in=8 => ret=1630 count=8 dest=ABCDEFG
    assert_eq!(
        driver_package_get_path_a(Some(&b"z.inf"[..]), Some(&mut dest[..]), Some(&mut count)),
        1630
    );
    assert_eq!(count, 8);
    assert_eq!(&dest, b"ABCDEFG\0");
    // G2 getpath_a inf=(null) dest=(null) count=(null) => ret=1630
    assert_eq!(driver_package_get_path_a(None, None, None), 1630);
    let mut wdest = [
        u16::from(b'W'),
        u16::from(b'X'),
        u16::from(b'Y'),
        u16::from(b'Z'),
        0,
        0,
        0,
        0,
    ];
    let mut count = 8;
    // G3 getpath_w inf=z.inf count_in=8 => ret=1630 count=8 dest=WXYZ
    assert_eq!(
        driver_package_get_path_w(
            Some(&utf16("z.inf")),
            Some(&mut wdest[..]),
            Some(&mut count)
        ),
        1630
    );
    assert_eq!(count, 8);
    assert_eq!(
        wdest,
        [
            u16::from(b'W'),
            u16::from(b'X'),
            u16::from(b'Y'),
            u16::from(b'Z'),
            0,
            0,
            0,
            0
        ]
    );
    let mut count = 0;
    // G4 getpath_w inf=(null) dest=(null) count_in=0 => ret=1630 count=0
    assert_eq!(
        driver_package_get_path_w(None, None, Some(&mut count)),
        1630
    );
    assert_eq!(count, 0);
}

#[test]
fn oracle_log_callback_rows() {
    // L1 set_log_callback_a cb=set ctx=set => calls=0
    difxapi_set_log_callback_a(Some(&Opaque), Some(&Opaque));
    // L2 set_log_callback_a cb=(null) ctx=(null) => calls=0
    difxapi_set_log_callback_a(None, None);
    // L3 set_log_callback_w cb=set ctx=(null) => calls=0
    difxapi_set_log_callback_w(Some(&Opaque), None);
    // L4 set_difx_log_callback_a cb=set ctx=set => calls=0
    set_difx_log_callback_a(Some(&Opaque), Some(&Opaque));
    // L5 set_difx_log_callback_w cb=set ctx=(null) => calls=0
    set_difx_log_callback_w(Some(&Opaque), None);
}
