use super::*;

#[test]
fn machine_constants_match_winnt_header() {
    assert_eq!(IMAGE_FILE_MACHINE_TARGET_HOST, 0x0001);
    assert_eq!(IMAGE_FILE_MACHINE_I386, 0x014c);
    assert_eq!(IMAGE_FILE_MACHINE_ARMNT, 0x01c4);
}

#[test]
fn lookup_is_exact_and_rejects_neighbouring_values() {
    for machine in [0x0000u16, 0x0002, 0x014d, 0x01c3, 0x01c5, 0xffff] {
        assert_eq!(
            get_machine_wow64_dir(machine),
            None,
            "machine {machine:#06x}"
        );
    }
}

#[test]
fn every_mapped_directory_is_an_nt_path() {
    for machine in [
        IMAGE_FILE_MACHINE_TARGET_HOST,
        IMAGE_FILE_MACHINE_I386,
        IMAGE_FILE_MACHINE_ARMNT,
    ] {
        let dir = get_machine_wow64_dir(machine).expect("mapped machine");
        assert!(dir.starts_with("\\??\\C:\\windows\\"), "{dir}");
        assert!(dir.is_ascii(), "{dir}");
    }
}

#[test]
fn mapped_directories_are_distinct() {
    let host = get_machine_wow64_dir(IMAGE_FILE_MACHINE_TARGET_HOST);
    let i386 = get_machine_wow64_dir(IMAGE_FILE_MACHINE_I386);
    let arm = get_machine_wow64_dir(IMAGE_FILE_MACHINE_ARMNT);
    assert_ne!(host, i386);
    assert_ne!(i386, arm);
    assert_ne!(host, arm);
}
