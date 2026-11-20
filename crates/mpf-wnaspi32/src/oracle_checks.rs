//! Twenty rows emitted by the ASan/UBSan-compiled Wine C oracle in
//! `target/canon/wnaspi32/oracle.c`.

use crate::support_info;

#[test]
fn oracle_s00_zero_controllers() {
    assert_eq!(support_info(0), 0x0000_e800);
}

#[test]
fn oracle_s01_one_controller() {
    assert_eq!(support_info(1), 0x0000_0101);
}

#[test]
fn oracle_s02_two_controllers() {
    assert_eq!(support_info(2), 0x0000_0102);
}

#[test]
fn oracle_s03_three_controllers() {
    assert_eq!(support_info(3), 0x0000_0103);
}

#[test]
fn oracle_s04_four_controllers() {
    assert_eq!(support_info(4), 0x0000_0104);
}

#[test]
fn oracle_s05_five_controllers() {
    assert_eq!(support_info(5), 0x0000_0105);
}

#[test]
fn oracle_s06_seven_controllers() {
    assert_eq!(support_info(7), 0x0000_0107);
}

#[test]
fn oracle_s07_eight_controllers() {
    assert_eq!(support_info(8), 0x0000_0108);
}

#[test]
fn oracle_s08_fifteen_controllers() {
    assert_eq!(support_info(15), 0x0000_010f);
}

#[test]
fn oracle_s09_sixteen_controllers() {
    assert_eq!(support_info(16), 0x0000_0110);
}

#[test]
fn oracle_s10_thirty_one_controllers() {
    assert_eq!(support_info(31), 0x0000_011f);
}

#[test]
fn oracle_s11_thirty_two_controllers() {
    assert_eq!(support_info(32), 0x0000_0120);
}

#[test]
fn oracle_s12_two_hundred_fifty_five_controllers() {
    assert_eq!(support_info(255), 0x0000_01ff);
}

#[test]
fn oracle_s13_two_hundred_fifty_six_controllers_overlap() {
    assert_eq!(support_info(256), 0x0000_0100);
}

#[test]
fn oracle_s14_one_thousand_twenty_three_controllers() {
    assert_eq!(support_info(1023), 0x0000_03ff);
}

#[test]
fn oracle_s15_one_thousand_twenty_four_controllers() {
    assert_eq!(support_info(1024), 0x0000_0500);
}

#[test]
fn oracle_s16_sixty_five_thousand_five_hundred_thirty_five_controllers() {
    assert_eq!(support_info(65535), 0x0000_ffff);
}

#[test]
fn oracle_s17_sixty_five_thousand_five_hundred_thirty_six_controllers() {
    assert_eq!(support_info(0x10000), 0x0001_0100);
}

#[test]
fn oracle_s18_sixteen_million_seven_hundred_seventy_seven_thousand_two_hundred_fifteen_controllers()
{
    assert_eq!(support_info(0xFFFFFF), 0x00ff_ffff);
}

#[test]
fn oracle_s19_max_u32_controllers() {
    assert_eq!(support_info(0xFFFF_FFFF), 0xFFFF_FFFF);
}
