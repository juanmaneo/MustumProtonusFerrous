//! Oracle-derived test vectors for the ported netutils logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of the four `NetApiBuffer*`
//! functions (`dlls/netutils/main.c`) and of `__wine_spec_unimplemented_stub`
//! (`dlls/ntdll/exception.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/netutils/`, and running the harness. The 25 printed rows are 8 function rows
//! (01 to 08) and 17 `@ stub` rows (09 to 25). ASan and UBSan reported nothing and the run leaked
//! nothing, so no row is excluded.
//!
//! The harness heap is a malloc-backed shim. [`FakeHeap`] mirrors its results. Block handles are
//! compared by NULL-ness, because the C pointers are not stable values.

use super::*;

/// `ERROR_NOT_ENOUGH_MEMORY` from `include/winerror.h`. The harness heap sets it on a forced
/// allocation failure.
pub(super) const ERROR_NOT_ENOUGH_MEMORY: u32 = 8;

/// The process heap of the oracle harness, mirrored in Rust. Blocks are `(handle, size)` pairs
/// and handles count up from 1. Handle 0 is never issued.
#[derive(Default)]
pub(super) struct FakeHeap {
    /// The live blocks, as `(handle, size)`.
    pub(super) live: Vec<(usize, usize)>,
    /// The last handle issued, which is also the number of successful allocations.
    pub(super) next: usize,
    /// Makes `alloc` and `realloc` fail with `ERROR_NOT_ENOUGH_MEMORY`.
    pub(super) fail_alloc: bool,
    last_error: u32,
}

impl ProcessHeap for FakeHeap {
    type Block = usize;

    fn alloc(&mut self, size: usize) -> Option<usize> {
        if self.fail_alloc {
            self.last_error = ERROR_NOT_ENOUGH_MEMORY;
            return None;
        }
        self.next += 1;
        self.live.push((self.next, size));
        Some(self.next)
    }

    fn realloc(&mut self, block: usize, size: usize) -> Option<usize> {
        if self.fail_alloc {
            self.last_error = ERROR_NOT_ENOUGH_MEMORY;
            return None;
        }
        let (_, live_size) = self
            .live
            .iter_mut()
            .find(|(handle, _)| *handle == block)
            .expect("realloc of a live block");
        *live_size = size;
        Some(block)
    }

    fn free(&mut self, block: Option<usize>) -> bool {
        let Some(block) = block else {
            return true;
        };
        match self.live.iter().position(|&(handle, _)| handle == block) {
            Some(index) => {
                self.live.swap_remove(index);
                true
            }
            None => {
                self.last_error = ERROR_INVALID_PARAMETER;
                false
            }
        }
    }

    fn size(&mut self, block: usize) -> usize {
        match self.live.iter().find(|&&(handle, _)| handle == block) {
            Some(&(_, size)) => size,
            None => {
                self.last_error = ERROR_INVALID_PARAMETER;
                usize::MAX
            }
        }
    }

    fn last_error(&self) -> u32 {
        self.last_error
    }
}

/// The `byte_count` value the harness sets before the size rows, `0xfffffffe`.
const SENTINEL: u32 = 0xffff_fffe;

#[test]
fn oracle_buffer_rows() {
    let mut heap = FakeHeap::default();

    // row 01 NetApiBufferAllocate(16, &buf) -> ret=0 buf=ok
    let mut a = None;
    assert_eq!(net_api_buffer_allocate(&mut heap, 16, Some(&mut a)), 0);
    assert!(a.is_some());

    // row 02 NetApiBufferAllocate(16, &buf) heap=oom -> ret=8 buf=NULL
    let mut b = Some(0);
    heap.fail_alloc = true;
    assert_eq!(
        net_api_buffer_allocate(&mut heap, 16, Some(&mut b)),
        ERROR_NOT_ENOUGH_MEMORY
    );
    heap.fail_alloc = false;
    assert_eq!(b, None);

    // row 03 NetApiBufferAllocate(16, NULL) -> ret=87
    assert_eq!(
        net_api_buffer_allocate(&mut heap, 16, None),
        ERROR_INVALID_PARAMETER
    );

    // row 04 NetApiBufferReallocate(buf01, 32, &new) -> ret=0 new=ok
    let mut new = None;
    assert_eq!(net_api_buffer_reallocate(&mut heap, a, 32, &mut new), 0);
    assert!(new.is_some());

    // row 05 NetApiBufferSize(new04, &bytes) -> ret=0 bytes=32
    let mut bytes = SENTINEL;
    assert_eq!(net_api_buffer_size(&mut heap, new, &mut bytes), 0);
    assert_eq!(bytes, 32);

    // row 06 NetApiBufferSize(NULL, &bytes) -> ret=87 bytes=4294967294
    let mut bytes = SENTINEL;
    assert_eq!(
        net_api_buffer_size(&mut heap, None, &mut bytes),
        ERROR_INVALID_PARAMETER
    );
    assert_eq!(bytes, 4_294_967_294);

    // row 07 NetApiBufferFree(new04) -> ret=0
    assert_eq!(net_api_buffer_free(&mut heap, new), 0);

    // row 08 NetApiBufferReallocate(NULL, 0, &new) -> ret=0 new=NULL
    let mut nb = Some(0);
    assert_eq!(net_api_buffer_reallocate(&mut heap, None, 0, &mut nb), 0);
    assert_eq!(nb, None);

    assert!(heap.live.is_empty());
}

/// A `@ stub` helper such as [`net_remote_computer_supports`].
type StubHelper = fn() -> StubException;

/// Rows 09 to 25. Each row is the `@ stub` helper that models the export, and the `function`
/// string the harness passed to `__wine_spec_unimplemented_stub`. Every row has code `0x80000100`,
/// flags `0x1`, two parameters, and module `netutils.dll`.
pub(super) const STUB_ROWS: [(StubHelper, &str); 17] = [
    (net_remote_computer_supports, "NetRemoteComputerSupports"), // row 09
    (netp_is_computer_name_valid, "NetpIsComputerNameValid"),    // row 10
    (netp_is_domain_name_valid, "NetpIsDomainNameValid"),        // row 11
    (netp_is_group_name_valid, "NetpIsGroupNameValid"),          // row 12
    (netp_is_remote, "NetpIsRemote"),                            // row 13
    (netp_is_remote_name_valid, "NetpIsRemoteNameValid"),        // row 14
    (netp_is_share_name_valid, "NetpIsShareNameValid"),          // row 15
    (
        netp_is_unc_computer_name_valid,
        "NetpIsUncComputerNameValid",
    ), // row 16
    (netp_is_user_name_valid, "NetpIsUserNameValid"),            // row 17
    (netpw_list_canonicalize, "NetpwListCanonicalize"),          // row 18
    (netpw_list_traverse, "NetpwListTraverse"),                  // row 19
    (netpw_name_canonicalize, "NetpwNameCanonicalize"),          // row 20
    (netpw_name_compare, "NetpwNameCompare"),                    // row 21
    (netpw_name_validate, "NetpwNameValidate"),                  // row 22
    (netpw_path_canonicalize, "NetpwPathCanonicalize"),          // row 23
    (netpw_path_compare, "NetpwPathCompare"),                    // row 24
    (netpw_path_type, "NetpwPathType"),                          // row 25
];

#[test]
fn oracle_stub_rows() {
    for (helper, function) in STUB_ROWS {
        assert_eq!(
            helper(),
            StubException {
                code: 0x8000_0100,
                flags: 0x1,
                number_parameters: 2,
                module: "netutils.dll",
                function,
            }
        );
    }
}
