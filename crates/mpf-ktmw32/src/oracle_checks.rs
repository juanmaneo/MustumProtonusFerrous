//! Oracle-derived test vectors for the ported ktmw32 logic.
//!
//! The vectors are the rows of `target/canon/ktmw32/oracle_rows.txt`, printed by `oracle.c` in
//! the same directory. The harness compiles the verbatim Wine C bodies of `ktmw32_main.c`,
//! `RtlInitUnicodeString`, `InitializeObjectAttributes` and `__wine_spec_unimplemented_stub`
//! with `gcc -fsanitize=address,undefined -g`. It prints 62 rows: 21 function rows (N01-N03,
//! T01-T05, C01-C03, R01-R02, X01-X08) and 41 stub rows (S01-S41, one per `@ stub` export).
//! No row was excluded, because the sanitizers printed nothing.
//!
//! The harness differs from the port in three ways:
//! - `RtlRaiseException` longjmps out of the stub, because the verbatim stub body loops around
//!   it. The port returns the record instead.
//! - `RtlNtStatusToDosError` is a stand-in, `status & 0xffff`, in both the harness and `FakeNt`.
//! - The harness prints `desc_buf=input` for pointer identity. The port compares the buffer by
//!   content. The `access`, `root`, `name`, `sqos` and `tm` columns are fixed by the shape of
//!   `NtCreateTransactionArgs`, so the port has no field to compare them with.

use super::*;

/// The `WCHAR` string that the harness builds with `count` copies of `a` and a NUL.
fn run_of_a(count: usize) -> Vec<u16> {
    let mut text = vec![u16::from(b'a'); count];
    text.push(0);
    text
}

/// The `NtCreateTransaction` arguments that `FakeNt` recorded. `length` and `maximum_length` are the
/// fields of the description, and `buffer` is a copy of its source.
#[derive(Debug, PartialEq, Eq)]
struct CreateCall {
    object_attributes: ObjectAttributes,
    uow: Option<Guid>,
    options: u32,
    level: u32,
    flags: u32,
    timeout: Option<i64>,
    length: u16,
    maximum_length: u16,
    buffer: Option<Vec<u16>>,
}

/// The OS side of `ktmw32_main.c`. `status` and `handle` are what the NT calls return, and
/// `last_error` stays `None` until `set_last_error` runs.
#[derive(Default)]
pub(crate) struct FakeNt {
    status: NtStatus,
    handle: Handle,
    pub(crate) last_error: Option<u32>,
    transaction_calls: Vec<(&'static str, Handle, bool)>,
    create_calls: Vec<CreateCall>,
}

impl Nt for FakeNt {
    fn nt_status_to_dos_error(&mut self, status: NtStatus) -> u32 {
        (status as u32) & 0xffff
    }

    fn set_last_error(&mut self, error: u32) {
        self.last_error = Some(error);
    }

    fn nt_commit_transaction(&mut self, transaction: Handle, wait: bool) -> NtStatus {
        self.transaction_calls
            .push(("NtCommitTransaction", transaction, wait));
        self.status
    }

    fn nt_rollback_transaction(&mut self, transaction: Handle, wait: bool) -> NtStatus {
        self.transaction_calls
            .push(("NtRollbackTransaction", transaction, wait));
        self.status
    }

    fn nt_create_transaction(&mut self, args: &NtCreateTransactionArgs<'_>) -> (NtStatus, Handle) {
        self.create_calls.push(CreateCall {
            object_attributes: args.object_attributes,
            uow: args.uow,
            options: args.options,
            level: args.level,
            flags: args.flags,
            timeout: args.timeout,
            length: args.description.length,
            maximum_length: args.description.maximum_length,
            buffer: args.description.buffer.map(<[u16]>::to_vec),
        });
        (self.status, self.handle)
    }
}

/// The one `NtCreateTransaction` call that `create_transaction` made.
fn only_create(nt: &FakeNt) -> &CreateCall {
    assert_eq!(nt.create_calls.len(), 1);
    &nt.create_calls[0]
}

/// The record that the harness prints for an `@ stub` export: `code=0x80000100 flags=0x1 nparams=2`
/// and `module=ktmw32.dll`, with `function` set to the export name.
fn oracle_stub(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x1,
        number_parameters: 2,
        module: "ktmw32.dll",
        function,
    }
}

#[test]
fn oracle_set_ntstatus_rows() {
    // N01 set_ntstatus status=0x00000000 -> ret=1 lasterr=unset
    let mut nt = FakeNt::default();
    assert!(set_ntstatus(&mut nt, 0));
    assert_eq!(nt.last_error, None);

    // N02 set_ntstatus status=0xc0000001 -> ret=0 lasterr=1
    let mut nt = FakeNt::default();
    assert!(!set_ntstatus(&mut nt, 0xC000_0001_u32 as NtStatus));
    assert_eq!(nt.last_error, Some(1));

    // N03 set_ntstatus status=0x00000103 -> ret=0 lasterr=259
    let mut nt = FakeNt::default();
    assert!(!set_ntstatus(&mut nt, 0x103));
    assert_eq!(nt.last_error, Some(259));
}

#[test]
fn oracle_get_nt_timeout_rows() {
    // T01 get_nt_timeout timeout=0 -> QuadPart=0
    assert_eq!(get_nt_timeout(0), Some(0));

    // T02 get_nt_timeout timeout=1 -> QuadPart=-10000
    assert_eq!(get_nt_timeout(1), Some(-10_000));

    // T03 get_nt_timeout timeout=1000 -> QuadPart=-10000000
    assert_eq!(get_nt_timeout(1000), Some(-10_000_000));

    // T04 get_nt_timeout timeout=4294967294 -> QuadPart=-42949672940000
    assert_eq!(get_nt_timeout(0xFFFF_FFFE), Some(-42_949_672_940_000));

    // T05 get_nt_timeout timeout=4294967295 -> NULL
    assert_eq!(get_nt_timeout(INFINITE), None);
}

#[test]
fn oracle_commit_transaction_rows() {
    // C01 CommitTransaction handle=0x1234 nt_status=0x00000000 -> ret=1 lasterr=unset
    //     nt_calls=1 nt_name=NtCommitTransaction nt_arg=0x1234 nt_wait=1
    let mut nt = FakeNt::default();
    assert!(commit_transaction(&mut nt, 0x1234));
    assert_eq!(nt.last_error, None);
    assert_eq!(
        nt.transaction_calls,
        [("NtCommitTransaction", 0x1234, true)]
    );

    // C02 CommitTransaction handle=0x10 nt_status=0xc0000008 -> ret=0 lasterr=8
    //     nt_calls=1 nt_name=NtCommitTransaction nt_arg=0x10 nt_wait=1
    let mut nt = FakeNt {
        status: 0xC000_0008_u32 as NtStatus,
        ..FakeNt::default()
    };
    assert!(!commit_transaction(&mut nt, 0x10));
    assert_eq!(nt.last_error, Some(8));
    assert_eq!(nt.transaction_calls, [("NtCommitTransaction", 0x10, true)]);

    // C03 CommitTransaction handle=0x0 nt_status=0x80000005 -> ret=0 lasterr=5
    //     nt_calls=1 nt_name=NtCommitTransaction nt_arg=0x0 nt_wait=1
    let mut nt = FakeNt {
        status: 0x8000_0005_u32 as NtStatus,
        ..FakeNt::default()
    };
    assert!(!commit_transaction(&mut nt, 0));
    assert_eq!(nt.last_error, Some(5));
    assert_eq!(nt.transaction_calls, [("NtCommitTransaction", 0, true)]);
}

#[test]
fn oracle_rollback_transaction_rows() {
    // R01 RollbackTransaction handle=0x1234 nt_status=0x00000000 -> ret=1 lasterr=unset
    //     nt_calls=1 nt_name=NtRollbackTransaction nt_arg=0x1234 nt_wait=1
    let mut nt = FakeNt::default();
    assert!(rollback_transaction(&mut nt, 0x1234));
    assert_eq!(nt.last_error, None);
    assert_eq!(
        nt.transaction_calls,
        [("NtRollbackTransaction", 0x1234, true)]
    );

    // R02 RollbackTransaction handle=0x10 nt_status=0xc0000008 -> ret=0 lasterr=8
    //     nt_calls=1 nt_name=NtRollbackTransaction nt_arg=0x10 nt_wait=1
    let mut nt = FakeNt {
        status: 0xC000_0008_u32 as NtStatus,
        ..FakeNt::default()
    };
    assert!(!rollback_transaction(&mut nt, 0x10));
    assert_eq!(nt.last_error, Some(8));
    assert_eq!(
        nt.transaction_calls,
        [("NtRollbackTransaction", 0x10, true)]
    );
}

#[test]
fn oracle_create_transaction_rows() {
    // X01 CreateTransaction nt_status=0x00000000 -> ret=0x77 lasterr=unset nt_calls=1
    //     attr=0x40 sd=0 uow=NULL opts=0 level=0 flags=0 timeout=NULL desc_len=0 desc_max=0
    //     desc_buf=NULL
    let mut nt = FakeNt {
        handle: 0x77,
        ..FakeNt::default()
    };
    assert_eq!(
        create_transaction(&mut nt, None, None, 0, 0, 0, INFINITE, None),
        0x77
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x40,
                security_descriptor: None,
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: None,
            length: 0,
            maximum_length: 0,
            buffer: None,
        }
    );

    // X02 CreateTransaction nt_status=0x00000000 -> ret=0x55 lasterr=unset nt_calls=1
    //     attr=0x42 sd=93 uow=0x11 opts=1 level=2 flags=3 timeout=0 desc_len=0 desc_max=2
    //     desc_buf=input
    let mut nt = FakeNt {
        handle: 0x55,
        ..FakeNt::default()
    };
    let sa = SecurityAttributes {
        inherit_handle: true,
        security_descriptor: Some(0x5d),
    };
    let guid = Guid {
        data1: 0x11,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };
    let empty: Vec<u16> = vec![0];
    assert_eq!(
        create_transaction(
            &mut nt,
            Some(&sa),
            Some(guid),
            1,
            2,
            3,
            0,
            Some(empty.as_slice())
        ),
        0x55
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x42,
                security_descriptor: Some(0x5d),
            },
            uow: Some(guid),
            options: 1,
            level: 2,
            flags: 3,
            timeout: Some(0),
            length: 0,
            maximum_length: 2,
            buffer: Some(vec![0]),
        }
    );

    // X03 CreateTransaction nt_status=0x00000000 -> ret=0x66 lasterr=unset nt_calls=1
    //     attr=0x40 sd=0 uow=NULL opts=0 level=0 flags=0 timeout=-10000000 desc_len=6
    //     desc_max=8 desc_buf=input
    let mut nt = FakeNt {
        handle: 0x66,
        ..FakeNt::default()
    };
    let sa = SecurityAttributes {
        inherit_handle: false,
        security_descriptor: None,
    };
    let abc: Vec<u16> = vec![0x61, 0x62, 0x63, 0];
    assert_eq!(
        create_transaction(
            &mut nt,
            Some(&sa),
            None,
            0,
            0,
            0,
            1000,
            Some(abc.as_slice())
        ),
        0x66
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x40,
                security_descriptor: None,
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: Some(-10_000_000),
            length: 6,
            maximum_length: 8,
            buffer: Some(abc.clone()),
        }
    );

    // X04 CreateTransaction nt_status=0xc0000001 -> ret=0xffffffffffffffff lasterr=1 nt_calls=1
    //     attr=0x40 sd=0 uow=NULL opts=0 level=0 flags=0 timeout=NULL desc_len=6 desc_max=8
    //     desc_buf=input
    let mut nt = FakeNt {
        status: 0xC000_0001_u32 as NtStatus,
        handle: 0x99,
        ..FakeNt::default()
    };
    assert_eq!(
        create_transaction(&mut nt, None, None, 0, 0, 0, INFINITE, Some(abc.as_slice())),
        INVALID_HANDLE_VALUE
    );
    assert_eq!(nt.last_error, Some(1));
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x40,
                security_descriptor: None,
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: None,
            length: 6,
            maximum_length: 8,
            buffer: Some(abc.clone()),
        }
    );

    // X05 CreateTransaction nt_status=0x00000000 -> ret=0x78 lasterr=unset nt_calls=1
    //     attr=0x42 sd=0 uow=NULL opts=0 level=0 flags=0 timeout=-10000 desc_len=0 desc_max=0
    //     desc_buf=NULL
    let mut nt = FakeNt {
        handle: 0x78,
        ..FakeNt::default()
    };
    let sa = SecurityAttributes {
        inherit_handle: true,
        security_descriptor: None,
    };
    assert_eq!(
        create_transaction(&mut nt, Some(&sa), None, 0, 0, 0, 1, None),
        0x78
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x42,
                security_descriptor: None,
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: Some(-10_000),
            length: 0,
            maximum_length: 0,
            buffer: None,
        }
    );

    // X06 CreateTransaction nt_status=0x00000000 -> ret=0x79 lasterr=unset nt_calls=1
    //     attr=0x40 sd=93 uow=NULL opts=0 level=0 flags=0 timeout=-42949672940000 desc_len=0
    //     desc_max=0 desc_buf=NULL
    let mut nt = FakeNt {
        handle: 0x79,
        ..FakeNt::default()
    };
    let sa = SecurityAttributes {
        inherit_handle: false,
        security_descriptor: Some(0x5d),
    };
    assert_eq!(
        create_transaction(&mut nt, Some(&sa), None, 0, 0, 0, 0xFFFF_FFFE, None),
        0x79
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x40,
                security_descriptor: Some(0x5d),
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: Some(-42_949_672_940_000),
            length: 0,
            maximum_length: 0,
            buffer: None,
        }
    );

    // X07 CreateTransaction nt_status=0x00000000 -> ret=0x7a lasterr=unset nt_calls=1
    //     attr=0x40 sd=0 uow=NULL opts=0 level=0 flags=0 timeout=NULL desc_len=65532
    //     desc_max=65534 desc_buf=input. The source has 0x7fff characters, so the cap applies.
    let mut nt = FakeNt {
        handle: 0x7a,
        ..FakeNt::default()
    };
    let cap = run_of_a(0x7fff);
    assert_eq!(
        create_transaction(&mut nt, None, None, 0, 0, 0, INFINITE, Some(cap.as_slice())),
        0x7a
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x40,
                security_descriptor: None,
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: None,
            length: 0xfffc,
            maximum_length: 0xfffe,
            buffer: Some(run_of_a(0x7fff)),
        }
    );

    // X08 CreateTransaction nt_status=0x00000000 -> ret=0x7b lasterr=unset nt_calls=1
    //     attr=0x40 sd=0 uow=NULL opts=0 level=0 flags=0 timeout=NULL desc_len=65532
    //     desc_max=65534 desc_buf=input. The source has 0x7ffe characters, which is exactly the cap.
    let mut nt = FakeNt {
        handle: 0x7b,
        ..FakeNt::default()
    };
    let edge = run_of_a(0x7ffe);
    assert_eq!(
        create_transaction(
            &mut nt,
            None,
            None,
            0,
            0,
            0,
            INFINITE,
            Some(edge.as_slice())
        ),
        0x7b
    );
    assert_eq!(nt.last_error, None);
    assert_eq!(
        only_create(&nt),
        &CreateCall {
            object_attributes: ObjectAttributes {
                attributes: 0x40,
                security_descriptor: None,
            },
            uow: None,
            options: 0,
            level: 0,
            flags: 0,
            timeout: None,
            length: 0xfffc,
            maximum_length: 0xfffe,
            buffer: Some(run_of_a(0x7ffe)),
        }
    );
}

#[test]
fn oracle_stub_rows() {
    // The array length is the `stub_count=41` line of the oracle.
    let rows: [(StubException, &'static str); 41] = [
        (commit_complete(), "CommitComplete"),                  // S01
        (commit_enlistment(), "CommitEnlistment"),              // S02
        (commit_transaction_async(), "CommitTransactionAsync"), // S03
        (create_enlistment(), "CreateEnlistment"),              // S04
        (create_resource_manager(), "CreateResourceManager"),   // S05
        (create_transaction_manager(), "CreateTransactionManager"), // S06
        (
            get_current_clock_transaction_manager(),
            "GetCurrentClockTransactionManager",
        ), // S07
        (get_enlistment_id(), "GetEnlistmentId"),               // S08
        (
            get_enlistment_recovery_information(),
            "GetEnlistmentRecoveryInformation",
        ), // S09
        (
            get_notification_resource_manager(),
            "GetNotificationResourceManager",
        ), // S10
        (
            get_notification_resource_manager_async(),
            "GetNotificationResourceManagerAsync",
        ), // S11
        (get_transaction_id(), "GetTransactionId"),             // S12
        (get_transaction_information(), "GetTransactionInformation"), // S13
        (get_transaction_manager_id(), "GetTransactionManagerId"), // S14
        (open_enlistment(), "OpenEnlistment"),                  // S15
        (open_resource_manager(), "OpenResourceManager"),       // S16
        (open_transaction(), "OpenTransaction"),                // S17
        (open_transaction_manager(), "OpenTransactionManager"), // S18
        (
            open_transaction_manager_by_id(),
            "OpenTransactionManagerById",
        ), // S19
        (pre_prepare_complete(), "PrePrepareComplete"),         // S20
        (pre_prepare_enlistment(), "PrePrepareEnlistment"),     // S21
        (prepare_complete(), "PrepareComplete"),                // S22
        (prepare_enlistment(), "PrepareEnlistment"),            // S23
        (priv_create_transaction(), "PrivCreateTransaction"),   // S24
        (
            priv_is_log_writable_transaction_manager(),
            "PrivIsLogWritableTransactionManager",
        ), // S25
        (priv_propagation_complete(), "PrivPropagationComplete"), // S26
        (priv_propagation_failed(), "PrivPropagationFailed"),   // S27
        (
            priv_register_protocol_address_information(),
            "PrivRegisterProtocolAddressInformation",
        ), // S28
        (read_only_enlistment(), "ReadOnlyEnlistment"),         // S29
        (recover_enlistment(), "RecoverEnlistment"),            // S30
        (recover_resource_manager(), "RecoverResourceManager"), // S31
        (recover_transaction_manager(), "RecoverTransactionManager"), // S32
        (rename_transaction_manager(), "RenameTransactionManager"), // S33
        (rollback_complete(), "RollbackComplete"),              // S34
        (rollback_enlistment(), "RollbackEnlistment"),          // S35
        (rollback_transaction_async(), "RollbackTransactionAsync"), // S36
        (
            rollforward_transaction_manager(),
            "RollforwardTransactionManager",
        ), // S37
        (
            set_enlistment_recovery_information(),
            "SetEnlistmentRecoveryInformation",
        ), // S38
        (
            set_resource_manager_completion_port(),
            "SetResourceManagerCompletionPort",
        ), // S39
        (set_transaction_information(), "SetTransactionInformation"), // S40
        (single_phase_reject(), "SinglePhaseReject"),           // S41
    ];
    for (stub, function) in rows {
        assert_eq!(stub, oracle_stub(function));
    }
}
