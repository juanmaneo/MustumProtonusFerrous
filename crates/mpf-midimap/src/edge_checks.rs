//! Edge cases: the oracle's own replies to the edge scripts, then the paths the oracle cannot
//! run. Those are the partial scheme maps that crash the C `modOpen`, the registry values past
//! the Win32 buffer sizes, and the failed-port and stale-index branches. Their expectations come
//! from reading `midimap.c`.

use super::oracle_checks::replay;
use crate::caps::{mapper_caps, MIDIOUTCAPSW_SIZE};
use crate::ports::{enumerate, find_port, OutPort, PortState};
use crate::settings::{Registry, Root};
use crate::stub::{self, Callback, Stub};
use crate::{mapper, Driver};
use crate::{MidiHdr, MidiOpenDesc, MHDR_PREPARED, MOM_CLOSE, MOM_DONE, MOM_OPEN};

/// Each edge script with a label and the text the C harness prints for it.
#[rustfmt::skip]
const ORACLE_EDGES: &[(&str, &str, &str)] = &[
    ("R1", "devs|Synth A|Midi Through;reg|u||CurrentInstrument|s|Midi Through;init;open|0", "1 ; 0 {o1:0,n3c7/0}"),
    ("R2", "devs|Synth A|Midi Through;reg|u||CurrentInstrument|s|;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("R3", "devs|Synth A|Midi Through;reg|u||UseScheme|d|0;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Midi Through;reg|m|Sch/0|Channels|d|FFFF;reg|u||CurrentInstrument|s|Synth A;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("R4", "devs|Synth A;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|;reg|m|Sch||s|Synth A;reg|m|Sch|Channels|d|FFFF;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("R5", "devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;reset", "1 ; 0 {o0:0,n3c7/0} ; 0 {r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0}"),
    ("R6", "devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;data|7F3C90;data|F8;data|3C", "1 ; 0 {o0:0,n3c7/0} ; 0 {s0:7f3c90:0} ; 0 {s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0} ; 0 {s0:3c90:0}"),
    ("R7", "devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;data|1234567890", "1 ; 0 {o0:0,n3c7/0} ; 0 {s0:34567890:0}"),
    ("R8", "devs|Synth A{o};reg|u||szPname|s|Synth A;init;open|0;data|7F3C90;data|3C;open|0;close", "1 ; 0 {o0:1,n3c7/0} ; 0 ; 0 ; 0 {n3c7/0} ; 0 {n3c8/0}"),
    ("R9", "devs|Synth A{l}|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Synth A;reg|m|Sch/0|Channels|d|1;reg|m|Sch/1||s|Midi Through;reg|m|Sch/1|Channels|d|FFFE;init;open|0;prep|30;long", "1 ; 0 {o0:0,o1:0,n3c7/0} ; 0 ; 1 {p0,l0:1,u0,n3c9/0}"),
    ("R11", "devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;reset", "1 ; 0 {o0:0,n3c7/0} ; 0 {r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0,r0:0}"),
    ("failed port: reset and long skip it", "devs|Synth A{o};reg|u||szPname|s|Synth A;init;open|0;reset;prep|30;long", "1 ; 0 {o0:1,n3c7/0} ; 0 ; 0 ; 0 {n3c9/0}"),
    ("failed port: system message skips it", "devs|Synth A{o};reg|u||szPname|s|Synth A;init;open|0;data|F8", "1 ; 0 {o0:1,n3c7/0} ; 0"),
    ("scheme root missing falls back to port 0", "devs|Synth A|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|u||CurrentInstrument|s|Midi Through;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("no user key falls back to port 0", "devs|Synth A|Midi Through;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
];

#[test]
fn oracle_edge_replays_match() {
    assert_eq!(ORACLE_EDGES.len(), 14);
    for (name, script, expected) in ORACLE_EDGES {
        assert_eq!(replay(script), *expected, "{name}: {script}");
    }
}

#[test]
fn find_port_reads_hash_numbers() {
    let ports = ports_named(&["Synth A", "Midi Through"]);
    assert_eq!(find(&ports, "#1abc"), Some(1));
    assert_eq!(find(&ports, "#1x2"), Some(1));
    assert_eq!(find(&ports, "#2"), None);
    assert_eq!(find(&ports, "#"), None);
    assert_eq!(find(&ports, "#x"), None);
    assert_eq!(find(&ports, "#4294967296"), Some(0));
    assert_eq!(find(&ports, "#4294967297"), Some(1));
    assert_eq!(find(&ports, ""), None);
    assert_eq!(find(&ports, "Nope"), None);
}

#[test]
fn exact_name_beats_hash_form() {
    let ports = ports_named(&["#1", "B"]);
    assert_eq!(find(&ports, "#1"), Some(0));
    assert_eq!(find(&ports, "#0"), Some(0));
    assert_eq!(find(&ports, "B"), Some(1));
}

#[test]
fn failed_ports_match_only_by_number() {
    let ports = vec![
        OutPort {
            name: None,
            state: PortState::Failed,
        },
        OutPort {
            name: Some(Vec::new()),
            state: PortState::Idle,
        },
    ];
    assert_eq!(find_port(&ports, &[]), Some(1));

    let ports = vec![OutPort {
        name: None,
        state: PortState::Failed,
    }];
    assert_eq!(find_port(&ports, &[]), None);
    assert_eq!(find(&ports, "#0"), Some(0));
}

#[test]
fn port_state_compares_clones_and_formats() {
    assert_ne!(PortState::Idle, PortState::Failed);
    assert_ne!(PortState::Open { handle: 1 }, PortState::Open { handle: 2 });
    assert_eq!(PortState::Open { handle: 1 }, PortState::Open { handle: 1 });
    assert_eq!(
        Clone::clone(&PortState::Open { handle: 4 }),
        PortState::Open { handle: 4 }
    );
    assert_eq!(
        format!(
            "{:?} {:?} {:?}",
            PortState::Idle,
            PortState::Open { handle: 4 },
            PortState::Failed
        ),
        "Idle Open { handle: 4 } Failed"
    );
}

#[test]
fn enumerate_clips_names_and_marks_failed_caps() {
    stub::reset();
    stub::add_device("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGH", "");
    stub::add_device("Dev\0Hidden", "");
    stub::add_device("Gone", "c");
    let ports: Vec<(Option<String>, PortState)> = enumerate(&Stub)
        .into_iter()
        .map(|port| {
            (
                port.name.map(|name| String::from_utf16_lossy(&name)),
                port.state,
            )
        })
        .collect();
    assert_eq!(
        ports,
        vec![
            (
                Some("ABCDEFGHIJKLMNOPQRSTUVWXYZ01234".to_string()),
                PortState::Idle
            ),
            (Some("Dev".to_string()), PortState::Idle),
            (None, PortState::Failed),
        ]
    );
}

#[test]
fn dev_caps_checks_arguments_and_clamps() {
    let empty = Driver::new(Stub, Stub);
    let mut buf = [0u8; MIDIOUTCAPSW_SIZE];
    assert_eq!(empty.dev_caps(MIDIOUTCAPSW_SIZE, Some(&mut buf[..])), 2);

    let driver = with_devices(&["Synth A"]);
    assert_eq!(driver.dev_caps(MIDIOUTCAPSW_SIZE, None), 11);

    let mut small = [0xFFu8; 5];
    assert_eq!(driver.dev_caps(10, Some(&mut small[..])), 0);
    assert_eq!(small, [0xFFu8, 0x00, 0x01, 0x00, 0x00]);

    let mut caps = [0u8; MIDIOUTCAPSW_SIZE];
    assert_eq!(driver.dev_caps(MIDIOUTCAPSW_SIZE, Some(&mut caps[..])), 0);
    assert_eq!(caps, mapper_caps());
    assert_eq!(caps[0..2], [0xFFu8, 0x00]);
    assert_eq!(caps[8..12], [0x57u8, 0x00, 0x69, 0x00]);
    assert_eq!(caps[72..74], [5u8, 0]);
    assert_eq!(caps[78..80], [0xFFu8, 0xFF]);
    assert_eq!(caps[80..84], [3u8, 0, 0, 0]);
}

#[test]
fn prepare_and_unprepare_check_arguments() {
    let mut driver = with_devices(&["Synth A"]);
    let inst = driver.open(Some(&MidiOpenDesc::default()), 0).unwrap();
    let mut hdr = MidiHdr {
        flags: 0,
        has_data: true,
    };

    assert_eq!(mapper::prepare(None, Some(&mut hdr), 44), 1);
    assert_eq!(mapper::prepare(Some(&inst), None, 44), 11);
    hdr.has_data = false;
    assert_eq!(mapper::prepare(Some(&inst), Some(&mut hdr), 44), 11);
    hdr.has_data = true;
    assert_eq!(mapper::prepare(Some(&inst), Some(&mut hdr), 43), 11);
    hdr.flags = 3;
    assert_eq!(mapper::prepare(Some(&inst), Some(&mut hdr), 44), 0);
    assert_eq!(hdr.flags, 3);
    hdr.flags = 5;
    assert_eq!(mapper::prepare(Some(&inst), Some(&mut hdr), 44), 0);
    assert_eq!(hdr.flags, 2);

    assert_eq!(mapper::unprepare(None, Some(&mut hdr), 44), 1);
    assert_eq!(mapper::unprepare(Some(&inst), None, 44), 11);
    hdr.flags = 6;
    assert_eq!(mapper::unprepare(Some(&inst), Some(&mut hdr), 44), 65);
    assert_eq!(hdr.flags, 6);
    hdr.flags = 2;
    assert_eq!(mapper::unprepare(Some(&inst), Some(&mut hdr), 44), 0);
    assert_eq!(hdr.flags, 0);
    assert_eq!(mapper::unprepare(Some(&inst), Some(&mut hdr), 44), 0);
}

#[test]
fn missing_instance_is_an_error() {
    let mut driver = with_devices(&["Synth A"]);
    let mut hdr = MidiHdr {
        flags: MHDR_PREPARED,
        has_data: true,
    };
    assert_eq!(driver.data(None, 0x90), 1);
    assert_eq!(driver.close(None), 1);
    assert_eq!(driver.reset(None), 1);
    assert_eq!(driver.long_data(None, &mut hdr), 1);
}

#[test]
fn callbacks_carry_the_open_descriptor() {
    let mut driver = with_devices(&["Synth A"]);
    let desc = MidiOpenDesc {
        handle: 7,
        callback: 0x1234,
        instance: 0x5678,
    };
    let open_cb: Callback = (desc, 3, MOM_OPEN, 0, 0);
    let close_cb: Callback = (desc, 3, MOM_CLOSE, 0, 0);

    let mut inst = driver.open(Some(&desc), 0x3_0000).unwrap();
    assert_eq!(stub::callbacks(), vec![open_cb]);
    assert_eq!(driver.close(Some(&mut inst)), 0);
    assert_eq!(stub::callbacks().last(), Some(&close_cb));
    assert_eq!(Clone::clone(&desc), desc);
    assert!(format!("{desc:?}").contains("handle: 7"));
}

#[test]
fn long_done_reports_the_header_address() {
    let mut driver = with_devices(&["Synth A"]);
    let mut inst = driver.open(Some(&MidiOpenDesc::default()), 0).unwrap();
    let mut hdr = MidiHdr {
        flags: 2,
        has_data: true,
    };
    assert_eq!(driver.long_data(Some(&mut inst), &mut hdr), 0);
    assert_eq!(hdr.flags, 3);
    let done: Callback = (
        MidiOpenDesc::default(),
        0,
        MOM_DONE,
        std::ptr::from_ref(&hdr) as usize,
        0,
    );
    assert_eq!(stub::callbacks().last(), Some(&done));
}

#[test]
fn instance_outlives_a_reloaded_table() {
    stub::reset();
    stub::add_device("Synth A", "");
    stub::add_device("Midi Through", "");
    stub::set_str('u', "", "szPname", "Midi Through");
    let mut driver = Driver::new(Stub, Stub);
    assert_eq!(driver.init(), 1);
    let mut inst = driver.open(Some(&MidiOpenDesc::default()), 0).unwrap();
    assert_eq!(stub::take_log(), "o1:0,n3c7/0");

    assert_eq!(driver.exit(), 1);
    stub::reset();
    stub::add_device("Synth A", "");
    assert_eq!(driver.init(), 1);
    assert_eq!(driver.data(Some(&mut inst), 0x7F3C90), 0);
    assert_eq!(driver.data(Some(&mut inst), 0xF7), 0);
    assert_eq!(driver.reset(Some(&mut inst)), 0);
    let mut hdr = MidiHdr {
        flags: MHDR_PREPARED,
        has_data: true,
    };
    assert_eq!(driver.long_data(Some(&mut inst), &mut hdr), 0);
    assert_eq!(driver.close(Some(&mut inst)), 0);
    assert_eq!(stub::take_log(), "n3c9/0,n3c8/0");
}

#[test]
fn partial_scheme_routes_only_the_mapped_channels() {
    let script = "devs|Synth A;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Synth A;reg|m|Sch/0|Channels|d|81;init;open|0;data|7F3C90;data|7F3C91;data|F7;data|F8;prep|30;long;reset;close";
    assert_eq!(
        replay(script),
        "1 ; 0 {o0:0,n3c7/0} ; 0 {s0:7f3c90:0} ; 0 ; 0 {s0:f7:0,s0:f7:0} ; 0 ; 0 ; 0 {p0,l0:0,u0,p0,l0:0,u0,n3c9/0} ; 0 {r0:0,r0:0} ; 0 {c0:0,n3c8/0}"
    );
}

#[test]
fn scheme_entries_without_a_usable_port_or_mask_are_skipped() {
    stub::reset();
    stub::add_device("Synth A", "");
    stub::add_device("Midi Through", "");
    stub::set_dword('u', "", "UseScheme", 1);
    stub::set_str('u', "", "CurrentScheme", "Sch");
    stub::set_str('m', "Sch/0", "", "Synth A");
    stub::set_dword('m', "Sch/1", "Channels", 0xFFFF);
    stub::set_str('m', "Sch/2", "", "Nope");
    stub::set_dword('m', "Sch/2", "Channels", 0xFFFF);
    stub::set_str('m', "Sch/3", "", "Synth A");
    stub::set_dword('m', "Sch/3", "Channels", 0xFFFF);
    stub::deny('m', "Sch/3");
    stub::set_str('m', "Sch/4", "", "Midi Through");
    stub::set_dword('m', "Sch/4", "Channels", 0x8000);
    stub::set_str('m', "Other/0", "", "Synth A");
    stub::set_str('m', "SchX/0", "", "Synth A");
    stub::set_str('m', "Sch", "", "Synth A");
    let mut driver = Driver::new(Stub, Stub);
    assert_eq!(driver.init(), 1);
    assert_eq!(open_log(&mut driver), "0 {o1:0,n3c7/0}");
}

#[test]
fn oversized_subkey_ends_the_scheme_walk() {
    let long = "x".repeat(300);
    let script = format!(
        "devs|Synth A|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Synth A;reg|m|Sch/0|Channels|d|FF;reg|m|Sch/{long}||s|Midi Through;reg|m|Sch/{long}|Channels|d|FF00;init;open|0"
    );
    assert_eq!(replay(&script), "1 ; 0 {o0:0,n3c7/0}");
}

#[test]
fn oversized_dword_is_absent() {
    let mut driver = with_devices(&["Synth A", "Midi Through"]);
    stub::set_raw('u', "", "UseScheme", &[1, 0, 0, 0, 0]);
    stub::set_str('u', "", "CurrentScheme", "Sch");
    stub::set_str('m', "Sch/0", "", "Synth A");
    stub::set_dword('m', "Sch/0", "Channels", 0xFFFF_FFFF);
    stub::set_str('u', "", "CurrentInstrument", "Midi Through");
    assert_eq!(open_log(&mut driver), "0 {o1:0,n3c7/0}");
}

#[test]
fn short_dword_is_zero_extended() {
    let mut driver = with_devices(&["Synth A", "Midi Through"]);
    stub::set_raw('u', "", "UseScheme", &[1]);
    stub::set_str('u', "", "CurrentScheme", "Sch");
    stub::set_str('m', "Sch/0", "", "Midi Through");
    stub::set_raw('m', "Sch/0", "Channels", &[1]);
    stub::set_str('u', "", "CurrentInstrument", "Synth A");
    assert_eq!(open_log(&mut driver), "0 {o1:0,n3c7/0}");
}

#[test]
fn oversized_string_is_absent() {
    let mut driver = with_devices(&["Synth A", "Midi Through"]);
    stub::set_str('u', "", "szPname", &"x".repeat(300));
    stub::set_str('u', "", "CurrentInstrument", "Midi Through");
    assert_eq!(open_log(&mut driver), "0 {o1:0,n3c7/0}");
}

#[test]
fn string_of_255_units_is_present() {
    let mut driver = with_devices(&["Synth A", "Midi Through"]);
    stub::set_str('u', "", "szPname", &"x".repeat(255));
    stub::set_str('u', "", "CurrentInstrument", "Midi Through");
    assert_eq!(open_log(&mut driver), "0 {o0:0,n3c7/0}");
}

#[test]
fn denied_scheme_falls_back_to_port_zero() {
    let mut driver = with_devices(&["Synth A", "Midi Through"]);
    stub::set_dword('u', "", "UseScheme", 1);
    stub::set_str('u', "", "CurrentScheme", "Sch");
    stub::set_str('m', "Sch/0", "", "Midi Through");
    stub::set_dword('m', "Sch/0", "Channels", 0xFFFF);
    stub::set_str('u', "", "CurrentInstrument", "Midi Through");
    stub::deny('m', "Sch");
    assert_eq!(open_log(&mut driver), "0 {o0:0,n3c7/0}");
}

#[test]
fn only_the_two_scheme_roots_open() {
    stub::reset();
    assert!(Stub.open_root(Root::LocalMachine, "x").is_none());
    assert!(Stub.open_root(Root::CurrentUser, "x").is_none());
}

fn with_devices(names: &[&str]) -> Driver<Stub, Stub> {
    stub::reset();
    for name in names {
        stub::add_device(name, "");
    }
    let mut driver = Driver::new(Stub, Stub);
    assert_eq!(driver.init(), 1);
    driver
}

fn open_log(driver: &mut Driver<Stub, Stub>) -> String {
    let ret = driver
        .open(Some(&MidiOpenDesc::default()), 0)
        .err()
        .unwrap_or(0);
    format!("{ret} {{{}}}", stub::take_log())
}

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

fn ports_named(names: &[&str]) -> Vec<OutPort> {
    names
        .iter()
        .map(|name| OutPort {
            name: Some(utf16(name)),
            state: PortState::Idle,
        })
        .collect()
}

fn find(ports: &[OutPort], name: &str) -> Option<usize> {
    find_port(ports, &utf16(name))
}
