//! nsi subset for MPF: the deterministic logic of the `dlls/nsi` exports that have bodies.
//!
//! Reference Wine's `dlls/nsi/nsi.c` (commit `9f8c4ae4d35c7ff4153267aba9618c026f5f5583`)
//! has bodies for 11 of the 26 entries in `nsi.spec`; the other 15 are spec-only
//! stubs. Eight of the 11 are ported here. They are the input and output sizing,
//! the reply layout, the retry loop of table allocation, and the cancel result.
//! Device I/O, the `\\.\Nsi` handle, and the caller's buffers stay outside the
//! port: the device is a closure and buffers are described by sizes and returned
//! segments. No network, adapter, routing, or registry state is read.
//!
//! Ported (oracle-derived table vectors and edge tests, see `oracle_tests.rs`):
//! - `NsiGetParameter`, `NsiGetParameterEx`: [`get_parameter`]
//! - `NsiGetAllParameters`, `NsiGetAllParametersEx`: [`get_all_parameters`]
//! - `NsiEnumerateObjectsAllParameters`, `NsiEnumerateObjectsAllParametersEx`: [`enumerate_all_parameters`]
//! - `NsiAllocateAndGetTable`: [`allocate_and_get_table`]
//! - `NsiCancelChangeNotification`: [`cancel_change_notification`]
//!
//! Each `Ex` form and its plain form share one function: the plain Wine wrapper only
//! repacks its arguments into the `Ex` struct.
//!
//! Skipped:
//! - 15 `stub` entries in `nsi.spec` (`NsiSetParameter`, `NsiRegisterChangeNotification`, ...):
//!   Wine has no body to port.
//! - `NsiFreeTable`: four `HeapFree` calls. Owned buffers are released by their owner.
//! - `NsiRequestChangeNotification`, `NsiRequestChangeNotificationEx`: overlapped device
//!   I/O and event creation. Wine's `unk` handling has an open FIXME.
//! - `DllMain`, `get_nsi_device`: process lifecycle and the cached `\\.\Nsi` handles.
//!
//! Divergences from Wine. Wine's arithmetic wraps where this port refuses:
//! - DWORD and ULONG sums and products that overflow return `None`, or
//!   `ERROR_OUTOFMEMORY` before any device call. Wine would size a buffer from the
//!   wrapped value.
//! - Wine copies a reply that holds more entries than requested, with nonzero block
//!   sizes, past the requested stride. This port returns `ERROR_INVALID_DATA` for
//!   status 0. For `ERROR_MORE_DATA` it reports the reply count and no blocks.
//! - Allocation failure is modelled only as overflow.
//! - Device I/O and `CancelIoEx` are injected closures. Device open (`get_nsi_device`) is
//!   not modelled: a failed open is whatever status the caller's closure returns.

/// Wine's `ERROR_SUCCESS`.
pub const ERROR_SUCCESS: u32 = 0;
/// Wine's `ERROR_INVALID_DATA`.
pub const ERROR_INVALID_DATA: u32 = 13;
/// Wine's `ERROR_OUTOFMEMORY`.
pub const ERROR_OUTOFMEMORY: u32 = 14;
/// Wine's `ERROR_MORE_DATA`: the reply is partial and the caller retries with a larger count.
pub const ERROR_MORE_DATA: u32 = 234;
/// Wine's `ERROR_NOT_FOUND`.
pub const ERROR_NOT_FOUND: u32 = 1168;

/// `FIELD_OFFSET(struct nsiproxy_get_parameter, key)`: the input bytes before the key.
const GET_PARAMETER_KEY_OFFSET: u32 = 44;
/// `FIELD_OFFSET(struct nsiproxy_get_all_parameters, key)`.
const GET_ALL_PARAMETERS_KEY_OFFSET: u32 = 48;
/// The `DWORD` entry count that starts an enumeration reply.
const ENUMERATE_HEADER: u32 = 4;
/// The entry count that `NsiAllocateAndGetTable` asks for first.
pub const INITIAL_TABLE_COUNT: u32 = 64;
/// How many enumerations `NsiAllocateAndGetTable` attempts before giving up.
pub const TABLE_ATTEMPTS: u32 = 5;

/// Bytes per entry of the four blocks of an enumeration: key, read-write, dynamic, static.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BlockSizes {
    pub key: u32,
    pub rw: u32,
    pub dynamic: u32,
    pub stat: u32,
}

impl BlockSizes {
    fn as_array(self) -> [u32; 4] {
        [self.key, self.rw, self.dynamic, self.stat]
    }

    fn is_empty(self) -> bool {
        self.as_array() == [0; 4]
    }
}

/// Where a block of a device reply lies: `len` bytes starting at `offset`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment {
    pub offset: u32,
    pub len: u32,
}

/// The result of [`enumerate_all_parameters`]. `count` is the reply count, or the
/// requested count when no reply was read. `blocks` is set when the reply can be
/// copied, in key, read-write, dynamic, static order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enumerated {
    pub status: u32,
    pub count: u32,
    pub blocks: Option<[Segment; 4]>,
}

/// The result of [`allocate_and_get_table`]: the entry count, and the bytes reserved for each block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableOutcome {
    pub count: u32,
    pub bytes: [u32; 4],
}

/// The input size of `NsiGetParameter` for a key of `key_size` bytes.
pub fn get_parameter_input_size(key_size: u32) -> Option<u32> {
    GET_PARAMETER_KEY_OFFSET.checked_add(key_size)
}

/// Runs `NsiGetParameter`. `device` receives the input size and returns the status.
pub fn get_parameter(key_size: u32, device: impl FnOnce(u32) -> u32) -> u32 {
    match get_parameter_input_size(key_size) {
        Some(in_size) => device(in_size),
        None => ERROR_OUTOFMEMORY,
    }
}

/// The input size of `NsiGetAllParameters` for a key of `key_size` bytes.
pub fn get_all_parameters_input_size(key_size: u32) -> Option<u32> {
    GET_ALL_PARAMETERS_KEY_OFFSET.checked_add(key_size)
}

/// The output size of `NsiGetAllParameters`: the three data blocks, back to back.
pub fn get_all_parameters_out_size(
    rw_size: u32,
    dynamic_size: u32,
    static_size: u32,
) -> Option<u32> {
    rw_size.checked_add(dynamic_size)?.checked_add(static_size)
}

/// Runs `NsiGetAllParameters`. `device` receives the input and output sizes and returns
/// the status. On success the returned segments locate the rw, dynamic and static blocks
/// in the reply. The caller copies them out, and Wine's copy is the same for each block.
pub fn get_all_parameters(
    key_size: u32,
    rw_size: u32,
    dynamic_size: u32,
    static_size: u32,
    device: impl FnOnce(u32, u32) -> u32,
) -> Result<[Segment; 3], u32> {
    let (Some(in_size), Some(out_size)) = (
        get_all_parameters_input_size(key_size),
        get_all_parameters_out_size(rw_size, dynamic_size, static_size),
    ) else {
        return Err(ERROR_OUTOFMEMORY);
    };
    match device(in_size, out_size) {
        ERROR_SUCCESS => Ok([
            Segment {
                offset: 0,
                len: rw_size,
            },
            Segment {
                offset: rw_size,
                len: dynamic_size,
            },
            Segment {
                offset: rw_size + dynamic_size,
                len: static_size,
            },
        ]),
        status => Err(status),
    }
}

/// The reply size of an enumeration of `requested` entries: the count header and every
/// block, each sized for `requested` entries.
pub fn enumerate_out_size(sizes: BlockSizes, requested: u32) -> Option<u32> {
    let per_entry: u128 = sizes.as_array().iter().map(|&size| u128::from(size)).sum();
    u32::try_from(u128::from(ENUMERATE_HEADER) + per_entry * u128::from(requested)).ok()
}

/// Runs `NsiEnumerateObjectsAllParameters(Ex)`. `device` receives the requested count and
/// the output size, and returns `(status, reply count)`.
///
/// Block `k` of a reply starts after the header and the blocks before it, each at
/// `requested` entries, and is `size_k * reply` bytes long.
pub fn enumerate_all_parameters(
    sizes: BlockSizes,
    requested: u32,
    device: impl FnOnce(u32, u32) -> (u32, u32),
) -> Enumerated {
    let Some(out_size) = enumerate_out_size(sizes, requested) else {
        return Enumerated {
            status: ERROR_OUTOFMEMORY,
            count: requested,
            blocks: None,
        };
    };
    let (status, reply) = device(requested, out_size);
    if status != ERROR_SUCCESS && status != ERROR_MORE_DATA {
        return Enumerated {
            status,
            count: requested,
            blocks: None,
        };
    }
    if reply > requested && !sizes.is_empty() {
        return if status == ERROR_SUCCESS {
            Enumerated {
                status: ERROR_INVALID_DATA,
                count: requested,
                blocks: None,
            }
        } else {
            Enumerated {
                status,
                count: reply,
                blocks: None,
            }
        };
    }
    let mut blocks = [Segment { offset: 0, len: 0 }; 4];
    let mut offset = ENUMERATE_HEADER;
    for (segment, size) in blocks.iter_mut().zip(sizes.as_array()) {
        *segment = Segment {
            offset,
            len: size * reply,
        };
        offset += size * requested;
    }
    Enumerated {
        status,
        count: reply,
        blocks: Some(blocks),
    }
}

/// The bytes that `NsiAllocateAndGetTable` reserves for each block at `count` entries, or
/// `None` if one of them overflows `u32`.
pub fn table_buffer_bytes(sizes: BlockSizes, count: u32) -> Option<[u32; 4]> {
    let mut bytes = [0; 4];
    for (slot, size) in bytes.iter_mut().zip(sizes.as_array()) {
        *slot = u32::try_from(u64::from(size) * u64::from(count)).ok()?;
    }
    Some(bytes)
}

/// Runs the retry loop of `NsiAllocateAndGetTable`.
///
/// `enumerate` runs one enumeration for the block sizes and entry count. It is called
/// with empty sizes to query how many entries exist now. When the enumeration is partial
/// (`ERROR_MORE_DATA`), the count grows by a sixteenth and the loop tries again, up to
/// [`TABLE_ATTEMPTS`] times.
pub fn allocate_and_get_table(
    sizes: BlockSizes,
    mut enumerate: impl FnMut(BlockSizes, u32) -> Enumerated,
) -> Result<TableOutcome, u32> {
    let mut count = INITIAL_TABLE_COUNT;
    for _ in 0..TABLE_ATTEMPTS {
        let bytes = table_buffer_bytes(sizes, count).ok_or(ERROR_OUTOFMEMORY)?;
        let first = enumerate(sizes, count);
        if first.status != ERROR_MORE_DATA {
            return match first.status {
                ERROR_SUCCESS => Ok(TableOutcome {
                    count: first.count,
                    bytes,
                }),
                status => Err(status),
            };
        }
        let query = enumerate(BlockSizes::default(), first.count);
        if query.status != ERROR_SUCCESS {
            return Err(query.status);
        }
        count = query.count.wrapping_add(query.count >> 4);
    }
    Err(ERROR_OUTOFMEMORY)
}

/// Runs `NsiCancelChangeNotification`. `cancel` runs `CancelIoEx` and returns the
/// `GetLastError` value on failure.
pub fn cancel_change_notification(
    ovr_present: bool,
    cancel: impl FnOnce() -> Result<(), u32>,
) -> u32 {
    if !ovr_present {
        return ERROR_NOT_FOUND;
    }
    match cancel() {
        Ok(()) => ERROR_SUCCESS,
        Err(error) => error,
    }
}

#[cfg(test)]
mod oracle_data;
#[cfg(test)]
mod oracle_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn parameter_device<'a>(calls: &'a Cell<u32>, status: u32) -> impl FnOnce(u32) -> u32 + 'a {
        move |_| {
            calls.set(calls.get() + 1);
            status
        }
    }

    fn all_parameters_device<'a>(
        calls: &'a Cell<u32>,
        status: u32,
    ) -> impl FnOnce(u32, u32) -> u32 + 'a {
        move |_, _| {
            calls.set(calls.get() + 1);
            status
        }
    }

    fn enumerate_device<'a>(
        calls: &'a Cell<u32>,
        reply: (u32, u32),
    ) -> impl FnOnce(u32, u32) -> (u32, u32) + 'a {
        move |_, _| {
            calls.set(calls.get() + 1);
            reply
        }
    }

    fn enumerate_script<'a>(
        calls: &'a Cell<u32>,
        script: &'a [Enumerated],
    ) -> impl FnMut(BlockSizes, u32) -> Enumerated + 'a {
        let mut next = 0;
        move |_, _| {
            calls.set(calls.get() + 1);
            next += 1;
            script[next - 1]
        }
    }

    fn cancel_device<'a>(
        calls: &'a Cell<u32>,
        result: Result<(), u32>,
    ) -> impl FnOnce() -> Result<(), u32> + 'a {
        move || {
            calls.set(calls.get() + 1);
            result
        }
    }

    fn done(status: u32, count: u32) -> Enumerated {
        Enumerated {
            status,
            count,
            blocks: None,
        }
    }

    fn segment(offset: u32, len: u32) -> Segment {
        Segment { offset, len }
    }

    #[test]
    fn get_parameter_input_size_adds_the_header() {
        assert_eq!(get_parameter_input_size(0), Some(44));
        assert_eq!(get_parameter_input_size(u32::MAX - 44), Some(u32::MAX));
        assert_eq!(get_parameter_input_size(u32::MAX - 43), None);
    }

    #[test]
    fn get_parameter_returns_the_device_status() {
        let calls = Cell::new(0);
        assert_eq!(
            get_parameter(8, parameter_device(&calls, ERROR_MORE_DATA)),
            ERROR_MORE_DATA
        );
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn get_parameter_overflow_is_out_of_memory_without_a_device_call() {
        let calls = Cell::new(0);
        assert_eq!(
            get_parameter(u32::MAX, parameter_device(&calls, ERROR_SUCCESS)),
            ERROR_OUTOFMEMORY
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn get_all_parameters_sizes_check_every_sum() {
        assert_eq!(get_all_parameters_input_size(0), Some(48));
        assert_eq!(get_all_parameters_input_size(u32::MAX - 48), Some(u32::MAX));
        assert_eq!(get_all_parameters_input_size(u32::MAX - 47), None);
        assert_eq!(get_all_parameters_out_size(u32::MAX, 0, 0), Some(u32::MAX));
        assert_eq!(get_all_parameters_out_size(u32::MAX, 1, 0), None);
        assert_eq!(get_all_parameters_out_size(u32::MAX - 1, 1, 1), None);
    }

    #[test]
    fn get_all_parameters_overflow_is_out_of_memory_without_a_device_call() {
        let calls = Cell::new(0);
        assert_eq!(
            get_all_parameters(u32::MAX, 0, 0, 0, all_parameters_device(&calls, 0)),
            Err(ERROR_OUTOFMEMORY)
        );
        assert_eq!(
            get_all_parameters(0, u32::MAX, 1, 0, all_parameters_device(&calls, 0)),
            Err(ERROR_OUTOFMEMORY)
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn get_all_parameters_lays_the_blocks_back_to_back() {
        let calls = Cell::new(0);
        let segments = get_all_parameters(2, 3, 5, 7, all_parameters_device(&calls, ERROR_SUCCESS));
        assert_eq!(segments, Ok([segment(0, 3), segment(3, 5), segment(8, 7)]));
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn get_all_parameters_returns_non_success_status_without_blocks() {
        let calls = Cell::new(0);
        assert_eq!(
            get_all_parameters(0, 1, 1, 1, all_parameters_device(&calls, ERROR_MORE_DATA)),
            Err(ERROR_MORE_DATA)
        );
        assert_eq!(
            get_all_parameters(0, 1, 1, 1, all_parameters_device(&calls, 87)),
            Err(87)
        );
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn enumerate_out_size_sums_in_wide_arithmetic() {
        let sizes = BlockSizes {
            key: 1,
            rw: 2,
            dynamic: 3,
            stat: 4,
        };
        assert_eq!(enumerate_out_size(sizes, 10), Some(104));
        assert_eq!(enumerate_out_size(BlockSizes::default(), u32::MAX), Some(4));
        assert_eq!(
            enumerate_out_size(
                BlockSizes {
                    key: u32::MAX,
                    rw: u32::MAX,
                    ..Default::default()
                },
                1
            ),
            None
        );
        assert_eq!(
            enumerate_out_size(
                BlockSizes {
                    key: 1,
                    ..Default::default()
                },
                u32::MAX
            ),
            None
        );
    }

    #[test]
    fn enumerate_out_overflow_is_out_of_memory_without_a_device_call() {
        let calls = Cell::new(0);
        let sizes = BlockSizes {
            key: 1,
            ..Default::default()
        };
        let result = enumerate_all_parameters(sizes, u32::MAX, enumerate_device(&calls, (0, 0)));
        assert_eq!(
            result,
            Enumerated {
                status: ERROR_OUTOFMEMORY,
                count: u32::MAX,
                blocks: None
            }
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn enumerate_other_status_returns_the_requested_count() {
        let calls = Cell::new(0);
        let sizes = BlockSizes {
            key: 4,
            ..Default::default()
        };
        let result = enumerate_all_parameters(sizes, 3, enumerate_device(&calls, (87, 2)));
        assert_eq!(
            result,
            Enumerated {
                status: 87,
                count: 3,
                blocks: None
            }
        );
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn enumerate_reply_within_the_request_lays_out_blocks() {
        let sizes = BlockSizes {
            key: 4,
            rw: 8,
            dynamic: 0,
            stat: 0,
        };
        let expected = Some([
            segment(4, 8),
            segment(16, 16),
            segment(40, 0),
            segment(40, 0),
        ]);
        let calls = Cell::new(0);
        assert_eq!(
            enumerate_all_parameters(sizes, 3, enumerate_device(&calls, (0, 2))).blocks,
            expected
        );
        assert_eq!(
            enumerate_all_parameters(sizes, 3, enumerate_device(&calls, (234, 2))).blocks,
            expected
        );
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn enumerate_reply_beyond_the_request_with_blocks_is_refused() {
        let sizes = BlockSizes {
            key: 4,
            ..Default::default()
        };
        let calls = Cell::new(0);
        let refused = enumerate_all_parameters(sizes, 3, enumerate_device(&calls, (0, 4)));
        assert_eq!(
            refused,
            Enumerated {
                status: ERROR_INVALID_DATA,
                count: 3,
                blocks: None
            }
        );
        let partial = enumerate_all_parameters(sizes, 3, enumerate_device(&calls, (234, 4)));
        assert_eq!(
            partial,
            Enumerated {
                status: ERROR_MORE_DATA,
                count: 4,
                blocks: None
            }
        );
    }

    #[test]
    fn enumerate_reply_beyond_the_request_without_blocks_is_copied_as_empty() {
        let calls = Cell::new(0);
        let empty = [segment(4, 0), segment(4, 0), segment(4, 0), segment(4, 0)];
        let result =
            enumerate_all_parameters(BlockSizes::default(), 1, enumerate_device(&calls, (0, 5)));
        assert_eq!(
            result,
            Enumerated {
                status: ERROR_SUCCESS,
                count: 5,
                blocks: Some(empty)
            }
        );
    }

    #[test]
    fn table_buffer_bytes_multiplies_each_block() {
        let sizes = BlockSizes {
            key: 1,
            rw: 2,
            dynamic: 3,
            stat: 4,
        };
        assert_eq!(table_buffer_bytes(sizes, 64), Some([64, 128, 192, 256]));
        assert_eq!(table_buffer_bytes(sizes, 0), Some([0; 4]));
    }

    #[test]
    fn table_buffer_bytes_overflow_in_any_block_is_none() {
        let big = 1 << 26;
        for [key, rw, dynamic, stat] in [
            [big, 0, 0, 0],
            [0, big, 0, 0],
            [0, 0, big, 0],
            [0, 0, 0, big],
        ] {
            assert_eq!(
                table_buffer_bytes(
                    BlockSizes {
                        key,
                        rw,
                        dynamic,
                        stat
                    },
                    64
                ),
                None
            );
        }
    }

    #[test]
    fn allocate_overflowing_table_is_out_of_memory_without_enumerating() {
        let calls = Cell::new(0);
        let sizes = BlockSizes {
            key: 1 << 26,
            ..Default::default()
        };
        assert_eq!(
            allocate_and_get_table(sizes, enumerate_script(&calls, &[])),
            Err(ERROR_OUTOFMEMORY)
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn allocate_first_success_returns_count_and_bytes() {
        let calls = Cell::new(0);
        let sizes = BlockSizes {
            key: 2,
            ..Default::default()
        };
        let outcome = allocate_and_get_table(sizes, enumerate_script(&calls, &[done(0, 10)]));
        assert_eq!(
            outcome,
            Ok(TableOutcome {
                count: 10,
                bytes: [128, 0, 0, 0]
            })
        );
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn allocate_other_status_is_returned() {
        let calls = Cell::new(0);
        let outcome = allocate_and_get_table(
            BlockSizes::default(),
            enumerate_script(&calls, &[done(87, 0)]),
        );
        assert_eq!(outcome, Err(87));
    }

    #[test]
    fn allocate_grows_the_count_from_the_size_query() {
        let calls = Cell::new(0);
        let sizes = BlockSizes {
            key: 1,
            ..Default::default()
        };
        let script = [done(234, 16), done(0, 16), done(0, 17)];
        assert_eq!(
            allocate_and_get_table(sizes, enumerate_script(&calls, &script)),
            Ok(TableOutcome {
                count: 17,
                bytes: [17, 0, 0, 0]
            })
        );
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn allocate_size_query_failure_is_returned() {
        let calls = Cell::new(0);
        let script = [done(234, 1), done(13, 0)];
        assert_eq!(
            allocate_and_get_table(BlockSizes::default(), enumerate_script(&calls, &script)),
            Err(13)
        );
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn allocate_partial_on_every_attempt_is_out_of_memory() {
        let calls = Cell::new(0);
        let script: Vec<Enumerated> = (0..TABLE_ATTEMPTS)
            .flat_map(|_| [done(234, 1), done(0, 1)])
            .collect();
        assert_eq!(
            allocate_and_get_table(BlockSizes::default(), enumerate_script(&calls, &script)),
            Err(ERROR_OUTOFMEMORY)
        );
        assert_eq!(calls.get(), 2 * TABLE_ATTEMPTS);
    }

    #[test]
    fn cancel_without_overlapped_is_not_found() {
        let calls = Cell::new(0);
        assert_eq!(
            cancel_change_notification(false, cancel_device(&calls, Ok(()))),
            ERROR_NOT_FOUND
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn cancel_returns_success_or_the_device_error() {
        let calls = Cell::new(0);
        assert_eq!(
            cancel_change_notification(true, cancel_device(&calls, Ok(()))),
            ERROR_SUCCESS
        );
        assert_eq!(
            cancel_change_notification(true, cancel_device(&calls, Err(5))),
            5
        );
        assert_eq!(calls.get(), 2);
    }
}
