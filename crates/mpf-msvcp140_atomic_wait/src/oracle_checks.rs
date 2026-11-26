//! Oracle-derived test vectors for the ported msvcp140_atomic_wait logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `__std_bulk_submit_threadpool_work`,
//! `__std_acquire_shared_mutex_for_instance`, `__std_release_shared_mutex_for_instance`,
//! `__std_tzdb_get_time_zones`, `__std_tzdb_get_current_zone` and `__std_tzdb_get_leap_seconds`
//! (`dlls/msvcp140_atomic_wait/main.c`), and `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`), with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/msvcp140_atomic_wait/`. The harness prints 25 rows. ASan and UBSan reported
//! nothing, LeakSanitizer found no leaks, so 0 vectors were excluded.
//!
//! The C pointers are fake addresses such as `0x1000`. They are compared, never dereferenced.
//! The harness shims `RtlRaiseException` with a recorder that `longjmp`s out of the stub's
//! `for (;;)` loop, so each stub row reads the record that was raised.

use super::*;

const DLL: &str = "msvcp140_atomic_wait.dll";

/// Encodes `s` as the NUL-terminated WCHAR array that `EnumDynamicTimeZoneInformation` fills.
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Collects the work items that `std_bulk_submit_threadpool_work` submits.
fn submit_log(work: usize, count: usize) -> Vec<usize> {
    let mut submits = Vec::new();
    std_bulk_submit_threadpool_work(work, count, |w| submits.push(w));
    submits
}

fn stub_record(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module: DLL,
        function,
    }
}

#[test]
fn oracle_bulk_submit_rows() {
    // row 01 bulk_submit count=0 work=0x5000 -> submits=0 mismatches=0
    assert!(submit_log(0x5000, 0).is_empty());

    // row 02 bulk_submit count=3 work=0x5000 -> submits=3 mismatches=0
    assert_eq!(submit_log(0x5000, 3), vec![0x5000usize; 3]);
}

#[test]
fn oracle_time_zone_rows() {
    // row 03 get_time_zones zones=[] -> count=0 ver=2022g error=0 links_null=0 names=[]
    let empty = std_tzdb_get_time_zones(&|_| None);
    assert_eq!(
        empty,
        TzdbTimeZones {
            error: TzdbError::Success,
            ver: "2022g",
            names: Vec::new(),
            links: Vec::new(),
        }
    );

    // row 04 get_time_zones zones=[UTC|Pacific Standard Time|Tokyo Standard Time] -> count=3
    // ver=2022g error=0 links_null=3 names=[UTC|Pacific Standard Time|Tokyo Standard Time]
    let zones = [
        wide("UTC"),
        wide("Pacific Standard Time"),
        wide("Tokyo Standard Time"),
    ];
    let three = std_tzdb_get_time_zones(&|i| zones.get(i as usize).cloned());
    assert_eq!(
        three,
        TzdbTimeZones {
            error: TzdbError::Success,
            ver: "2022g",
            names: vec![
                b"UTC\0".to_vec(),
                b"Pacific Standard Time\0".to_vec(),
                b"Tokyo Standard Time\0".to_vec(),
            ],
            links: vec![None; 3],
        }
    );

    // row 05 get_time_zones zones=[Zone<U+0100>X|Q] -> count=2 ver=2022g error=0 links_null=2
    // names=[Zone|Q]. The U+0100 low byte is 0, so "Zone" ends there. The zero fill after it is
    // not in the oracle, which prints only the C string.
    let zones = [wide("Zone\u{100}X"), wide("Q")];
    let truncated = std_tzdb_get_time_zones(&|i| zones.get(i as usize).cloned());
    assert_eq!(
        truncated,
        TzdbTimeZones {
            error: TzdbError::Success,
            ver: "2022g",
            names: vec![b"Zone\0\0\0".to_vec(), b"Q\0".to_vec()],
            links: vec![None; 2],
        }
    );
}

#[test]
fn oracle_current_zone_rows() {
    // row 06 get_current_zone zone=valid -> error=0 name=Pacific Standard Time
    let valid = std_tzdb_get_current_zone(&|| Some(wide("Pacific Standard Time")));
    assert_eq!(
        valid,
        TzdbCurrentZone {
            error: TzdbError::Success,
            name: Some(b"Pacific Standard Time\0".to_vec()),
        }
    );

    // row 07 get_current_zone zone=invalid -> error=1 name=NULL
    let invalid = std_tzdb_get_current_zone(&|| None);
    assert_eq!(
        invalid,
        TzdbCurrentZone {
            error: TzdbError::Win,
            name: None,
        }
    );
}

#[test]
fn oracle_leap_seconds_row() {
    // row 08 get_leap_seconds prev_size=7 -> ret=NULL new_size=0
    assert_eq!(std_tzdb_get_leap_seconds(7), (None, 0));
}

#[test]
fn oracle_shared_mutex_rows() {
    let mut list = SharedMutexList::default();

    // row 09 acquire ptr=0x1000 -> ret_owner=0x1000 refs=1 live=1
    assert_eq!(
        std_acquire_shared_mutex_for_instance(&mut list, 0x1000),
        0x1000
    );
    assert_eq!(list.ref_count(0x1000), Some(1));
    assert_eq!(list.live_count(), 1);

    // row 10 acquire ptr=0x1000 -> ret_owner=0x1000 refs=2 live=1
    assert_eq!(
        std_acquire_shared_mutex_for_instance(&mut list, 0x1000),
        0x1000
    );
    assert_eq!(list.ref_count(0x1000), Some(2));
    assert_eq!(list.live_count(), 1);

    // row 11 acquire ptr=0x2000 -> ret_owner=0x2000 refs=1 live=2
    assert_eq!(
        std_acquire_shared_mutex_for_instance(&mut list, 0x2000),
        0x2000
    );
    assert_eq!(list.ref_count(0x2000), Some(1));
    assert_eq!(list.live_count(), 2);

    // row 12 release ptr=0x1000 -> refs=1 live=2
    std_release_shared_mutex_for_instance(&mut list, 0x1000);
    assert_eq!(list.ref_count(0x1000), Some(1));
    assert_eq!(list.live_count(), 2);

    // row 13 release ptr=0x1000 -> refs=none live=1
    std_release_shared_mutex_for_instance(&mut list, 0x1000);
    assert_eq!(list.ref_count(0x1000), None);
    assert_eq!(list.live_count(), 1);

    // row 14 release ptr=0x3000 -> refs=none live=1
    std_release_shared_mutex_for_instance(&mut list, 0x3000);
    assert_eq!(list.ref_count(0x3000), None);
    assert_eq!(list.live_count(), 1);
}

#[test]
fn oracle_stub_rows() {
    // row 15 stub __std_atomic_compare_exchange_128
    assert_eq!(
        std_atomic_compare_exchange_128(),
        stub_record("__std_atomic_compare_exchange_128")
    );
    // row 16 stub __std_atomic_get_mutex
    assert_eq!(
        std_atomic_get_mutex(),
        stub_record("__std_atomic_get_mutex")
    );
    // row 17 stub __std_atomic_has_cmpxchg16b
    assert_eq!(
        std_atomic_has_cmpxchg16b(),
        stub_record("__std_atomic_has_cmpxchg16b")
    );
    // row 18 stub __std_atomic_notify_all_indirect
    assert_eq!(
        std_atomic_notify_all_indirect(),
        stub_record("__std_atomic_notify_all_indirect")
    );
    // row 19 stub __std_atomic_notify_one_indirect
    assert_eq!(
        std_atomic_notify_one_indirect(),
        stub_record("__std_atomic_notify_one_indirect")
    );
    // row 20 stub __std_atomic_set_api_level
    assert_eq!(
        std_atomic_set_api_level(),
        stub_record("__std_atomic_set_api_level")
    );
    // row 21 stub __std_atomic_wait_get_deadline
    assert_eq!(
        std_atomic_wait_get_deadline(),
        stub_record("__std_atomic_wait_get_deadline")
    );
    // row 22 stub __std_atomic_wait_get_remaining_timeout
    assert_eq!(
        std_atomic_wait_get_remaining_timeout(),
        stub_record("__std_atomic_wait_get_remaining_timeout")
    );
    // row 23 stub __std_atomic_wait_indirect
    assert_eq!(
        std_atomic_wait_indirect(),
        stub_record("__std_atomic_wait_indirect")
    );
    // row 24 stub __std_tzdb_delete_sys_info
    assert_eq!(
        std_tzdb_delete_sys_info(),
        stub_record("__std_tzdb_delete_sys_info")
    );
    // row 25 stub __std_tzdb_get_sys_info
    assert_eq!(
        std_tzdb_get_sys_info(),
        stub_record("__std_tzdb_get_sys_info")
    );
}
