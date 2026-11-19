//! Tests against Wine's oracle vectors.
//!
//! The vectors are the replies of Wine's verbatim `midimap.c`, built with ASan and UBSan by
//! `target/canon/midimap/build.sh` from `oracle.c`. Each vector is a script and the text the C
//! harness prints for it: `;`-separated steps, `|`-separated fields, and the printed results
//! joined by ` ; `. [`replay`] runs a script through the port and prints the results the same
//! way, so every vector is compared as exact text.

use crate::caps::MIDIOUTCAPSW_SIZE;
use crate::stub::{self, Stub};
use crate::{mapper, Driver, Instance};
use crate::{MidiHdr, MidiOpenDesc};

/// Each script with the text the C harness prints for it.
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("init;init;exit;exit", "1 ; 0 ; 1 ; 0"),
    ("caps|84", "2 {caps=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa}"),
    ("devs|Synth A;init;caps|84|null", "1 ; 11"),
    ("devs|Synth A;init;caps|A;caps|C8", "1 ; 0 {caps=ff000100000100005700aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa} ; 0 {caps=ff00010000010000570069006e00650020006d0069006400690020006d00610070007000650072000000000000000000000000000000000000000000000000000000000000000000050000000000ffff03000000}"),
    ("devs|Synth A;init;exit;caps|84", "1 ; 1 ; 0 {caps=ff00010000010000570069006e00650020006d0069006400690020006d00610070007000650072000000000000000000000000000000000000000000000000000000000000000000050000000000ffff03000000}"),
    ("devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;data|7F3C90;data|3C;data|7F;close", "1 ; 0 {o0:0,n3c7/0} ; 0 {s0:7f3c90:0} ; 0 {s0:3c90:0} ; 0 {s0:7f90:0} ; 0 {c0:0,n3c8/0}"),
    ("devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;data|7F3C90;data|F7;data|3C;data|F8", "1 ; 0 {o0:0,n3c7/0} ; 0 {s0:7f3c90:0} ; 0 {s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0,s0:f7:0} ; 0 ; 0 {s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0,s0:f8:0}"),
    ("devs|Synth A{s};reg|u||szPname|s|Synth A;init;open|0;data|7F3C90;data|F8", "1 ; 0 {o0:0,n3c7/0} ; 1 {s0:7f3c90:1} ; 1 {s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1,s0:f8:1}"),
    ("devs|Synth A|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Synth A;reg|m|Sch/0|Channels|d|FFFF;reg|m|Sch/1||s|Midi Through;reg|m|Sch/1|Channels|d|1;init;open|0;data|7F3C90;data|7F3C91;close", "1 ; 0 {o1:0,o0:0,n3c7/0} ; 0 {s1:7f3c90:0} ; 0 {s0:7f3c91:0} ; 0 {c1:0,c0:0,n3c8/0}"),
    ("devs|Synth A|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Nope;reg|u||CurrentInstrument|s|Midi Through;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("devs|Synth A|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentInstrument|s|Midi Through;init;open|0;close", "1 ; 0 {o1:0,n3c7/0} ; 0 {c1:0,n3c8/0}"),
    ("devs|Synth A|Midi Through;reg|u||szPname|s|#1abc;init;open|0", "1 ; 0 {o1:0,n3c7/0}"),
    ("devs|Synth A|Midi Through;reg|u||szPname|s|#4294967296;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("devs|Synth A{c}|Midi Through;reg|u||szPname|s|Synth A;init;open|0;data|7F3C90", "1 ; 0 {n3c7/0} ; 0"),
    ("devs|Synth A|ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789;reg|u||szPname|s|ABCDEFGHIJKLMNOPQRSTUVWXYZ01234;init;open|0", "1 ; 0 {o1:0,n3c7/0}"),
    ("devs|Synth A;reg|u||szPname|s|Synth A;init;open|30000;close", "1 ; 0 {o0:0,n3c7/3} ; 0 {c0:0,n3c8/3}"),
    ("devs|Synth A;reg|u||szPname|s|Synth A;init;open|0|null", "1 ; 11"),
    ("init;data|7F3C90;reset;open|0", "1 ; 1 ; 1 ; 69"),
    ("devs|Synth A{x};reg|u||szPname|s|Synth A;init;open|0;close;close", "1 ; 0 {o0:0,n3c7/0} ; 1 {c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1} ; 1 {c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1}"),
    ("devs|Synth A{x}|Midi Through{x};reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Synth A;reg|m|Sch/0|Channels|d|FF;reg|m|Sch/1||s|Midi Through;reg|m|Sch/1|Channels|d|FF00;init;open|0;close", "1 ; 0 {o0:0,o1:0,n3c7/0} ; 1 {c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c0:1,c1:1,c1:1,c1:1,c1:1,c1:1,c1:1,c1:1,c1:1}"),
    ("devs|Synth A{r};reg|u||szPname|s|Synth A;init;open|0;data|7F3C90;reset;data|3C", "1 ; 0 {o0:0,n3c7/0} ; 0 {s0:7f3c90:0} ; 1 {r0:1} ; 0"),
    ("devs|Synth A;reg|u||szPname|s|Synth A;init;open|0;hdr|0|1;long;prep|2b;prep|2c;long;unprep|30;unprep|30;hdr|6|1;long;hdr|0|0;prep|30;prep|30|null;hdr|5|1;prep|30;long", "1 ; 0 {o0:0,n3c7/0} ; 64 ; 11 ; 0 ; 0 {p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,n3c9/0} ; 0 ; 0 ; 65 ; 11 ; 11 ; 0 ; 0 {p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,p0,l0:0,u0,n3c9/0}"),
    ("devs|Synth A{l};reg|u||szPname|s|Synth A;init;open|0;hdr|0|1;prep|30;long", "1 ; 0 {o0:0,n3c7/0} ; 0 ; 1 {p0,l0:1,u0,n3c9/0}"),
    ("devs|Synth A|Midi Through;reg|u||UseScheme|d|1;reg|u||CurrentScheme|s|Sch;reg|m|Sch/0||s|Synth A;reg|m|Sch/0|Channels|d|FFFF;reg|m|Sch/1||s|Midi Through;reg|m|Sch/2||s|Synth A;reg|m|Sch/3|Channels|d|FFFF;reg|m|Sch/4||s|Nope;reg|m|Sch/4|Channels|d|FF;init;open|0", "1 ; 0 {o0:0,n3c7/0}"),
    ("vol;vol|null", "0 {v=ffffffff} ; 11"),
];

#[test]
fn oracle_vectors_match_wine() {
    assert_eq!(VECTORS.len(), 25);
    for (script, expected) in VECTORS {
        assert_eq!(replay(script), *expected, "scenario: {script}");
    }
}

#[test]
#[should_panic(expected = "unknown step: bogus")]
fn unknown_step_is_rejected() {
    replay("devs|Synth A;bogus");
}

/// The state of one script: the driver, its open instance, the `MIDIHDR` the steps fill in,
/// and the results printed so far.
struct Replay {
    driver: Driver<Stub, Stub>,
    instance: Option<Instance>,
    hdr: MidiHdr,
    outputs: Vec<String>,
}

/// Runs `script` the way the C harness does, and joins the printed results with ` ; `.
pub(crate) fn replay(script: &str) -> String {
    stub::reset();
    let mut run = Replay {
        driver: Driver::new(Stub, Stub),
        instance: None,
        hdr: MidiHdr {
            flags: 0,
            has_data: true,
        },
        outputs: Vec::new(),
    };
    for step in script.split(';') {
        run.step(step);
    }
    run.outputs.join(" ; ")
}

impl Replay {
    fn step(&mut self, step: &str) {
        let f: Vec<&str> = step.split('|').collect();
        let has_null = f.get(2) == Some(&"null");
        let ret = match f[0] {
            "devs" => {
                for &spec in &f[1..] {
                    let (name, flags) = spec.split_once('{').unwrap_or((spec, ""));
                    stub::add_device(name, flags.trim_end_matches('}'));
                }
                None
            }
            "reg" => {
                let hive = f[1].chars().next().expect("registry hive");
                if f[4] == "d" {
                    stub::set_dword(hive, f[2], f[3], hex(f[5]) as u32);
                } else {
                    stub::set_str(hive, f[2], f[3], f[5]);
                }
                None
            }
            "init" => Some(self.driver.init()),
            "exit" => Some(self.driver.exit()),
            "open" => {
                let default_desc = MidiOpenDesc::default();
                let desc = (!has_null).then_some(&default_desc);
                match self.driver.open(desc, hex(f[1]) as u32) {
                    Ok(instance) => {
                        self.instance = Some(instance);
                        Some(0)
                    }
                    Err(ret) => Some(ret),
                }
            }
            "close" => {
                let ret = self.driver.close(self.instance.as_mut());
                if ret == 0 {
                    self.instance = None;
                }
                Some(ret)
            }
            "data" => Some(self.driver.data(self.instance.as_mut(), hex(f[1]))),
            "hdr" => {
                self.hdr = MidiHdr {
                    flags: hex(f[1]) as u32,
                    has_data: f[2].starts_with('1'),
                };
                None
            }
            "prep" => Some(mapper::prepare(
                self.instance.as_ref(),
                (!has_null).then_some(&mut self.hdr),
                hex(f[1]),
            )),
            "unprep" => Some(mapper::unprepare(
                self.instance.as_ref(),
                (!has_null).then_some(&mut self.hdr),
                hex(f[1]),
            )),
            "long" => Some(self.driver.long_data(self.instance.as_mut(), &mut self.hdr)),
            "reset" => Some(self.driver.reset(self.instance.as_mut())),
            "vol" => {
                let is_null = f.get(1) == Some(&"null");
                let mut volume = 0u32;
                let ret = mapper::get_volume((!is_null).then_some(&mut volume));
                if !is_null {
                    stub::log(format!("v={volume:x}"));
                }
                Some(ret)
            }
            "caps" => {
                let mut buf = [0xAAu8; 256];
                let ret = self
                    .driver
                    .dev_caps(hex(f[1]), (!has_null).then_some(&mut buf[..]));
                if !has_null {
                    stub::log(format!("caps={}", hex_bytes(&buf[..MIDIOUTCAPSW_SIZE])));
                }
                Some(ret)
            }
            other => panic!("unknown step: {other}"),
        };
        if let Some(ret) = ret {
            self.outputs.push(emit(ret));
        }
    }
}

/// The printed form of a result: the value, then the calls logged since the last result.
fn emit(ret: u32) -> String {
    let log = stub::take_log();
    if log.is_empty() {
        ret.to_string()
    } else {
        format!("{ret} {{{log}}}")
    }
}

fn hex(field: &str) -> usize {
    usize::from_str_radix(field, 16).expect("hex field")
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
