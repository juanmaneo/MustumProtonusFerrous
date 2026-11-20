//! Oracle tests from `target/canon/wow64/output.txt`, produced by running the verbatim
//! `get_machine_wow64_dir` from Wine's `dlls/wow64/wow64_private.h` at commit
//! `e2aa6cd75ae60acd90ff384feee171ce994f0f6d`, compiled with
//! `gcc -fsanitize=address,undefined -g`. No sanitizer report was produced, so no
//! vector was excluded.

use super::*;

const ROWS: &[(u16, Option<&str>)] = &[
    (0x0000, None),
    (0x0001, Some("\\??\\C:\\windows\\system32")),
    (0x0002, None),
    (0x0032, None),
    (0x003e, None),
    (0x014c, Some("\\??\\C:\\windows\\syswow64")),
    (0x0166, None),
    (0x01a2, None),
    (0x01c0, None),
    (0x01c2, None),
    (0x01c4, Some("\\??\\C:\\windows\\sysarm32")),
    (0x0200, None),
    (0x0266, None),
    (0x0284, None),
    (0x0366, None),
    (0x8664, None),
    (0xaa64, None),
    (0xffff, None),
];

#[test]
fn get_machine_wow64_dir_matches_wine_oracle() {
    for &(machine, dir) in ROWS {
        assert_eq!(
            get_machine_wow64_dir(machine),
            dir,
            "machine {machine:#06x}"
        );
    }
}
