//! A host with scripted devices and registry, standing in for `midiOut*`, `DriverCallback` and
//! the registry in the tests. Its state is per thread, so each test calls [`reset`] first.
//!
//! A device's flags script its failures, as the oracle's `devs` flags do: `c` the capability
//! query, `o` open, `s` short message, `l` long message, `r` reset and `x` close. A handle is
//! the device index.

use crate::settings::{Registry, Root, SCHEMES_KEY, USER_KEY};
use crate::{MidiHdr, MidiOpenDesc, MidiOut};
use std::cell::RefCell;

/// A `DriverCallback` call: the descriptor, the callback flags, the message and its two parameters.
pub type Callback = (MidiOpenDesc, u16, u32, usize, usize);

#[derive(Default)]
struct State {
    devices: Vec<Device>,
    entries: Vec<Entry>,
    denied: Vec<(char, String)>,
    log: Vec<String>,
    callbacks: Vec<Callback>,
}

struct Device {
    name: String,
    flags: String,
}

struct Entry {
    hive: char,
    path: String,
    name: String,
    data: Vec<u8>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

fn update<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with(|state| f(&mut state.borrow_mut()))
}

/// Clears the devices, the registry, the denials, the log and the callbacks.
pub fn reset() {
    update(|state| *state = State::default());
}

/// Adds a device named `name` that fails as `flags` scripts.
pub fn add_device(name: &str, flags: &str) {
    update(|state| {
        state.devices.push(Device {
            name: name.to_string(),
            flags: flags.to_string(),
        });
    });
}

/// Sets a registry value to the raw bytes `data`.
pub fn set_raw(hive: char, path: &str, name: &str, data: &[u8]) {
    update(|state| {
        state.entries.push(Entry {
            hive,
            path: path.to_string(),
            name: name.to_string(),
            data: data.to_vec(),
        });
    });
}

/// Sets a `REG_SZ` value: the UTF-16 units of `value` and a terminating NUL.
pub fn set_str(hive: char, path: &str, name: &str, value: &str) {
    let mut data: Vec<u8> = value.encode_utf16().flat_map(u16::to_le_bytes).collect();
    data.extend_from_slice(&[0, 0]);
    set_raw(hive, path, name, &data);
}

/// Sets a `REG_DWORD` value.
pub fn set_dword(hive: char, path: &str, name: &str, value: u32) {
    set_raw(hive, path, name, &value.to_le_bytes());
}

/// Makes `open_sub` fail for the key at `path` in `hive`.
pub fn deny(hive: char, path: &str) {
    update(|state| state.denied.push((hive, path.to_string())));
}

/// Appends an entry to the call log.
pub fn log(entry: String) {
    update(|state| state.log.push(entry));
}

/// Returns the call log, comma separated, and clears it.
pub fn take_log() -> String {
    update(|state| std::mem::take(&mut state.log).join(","))
}

/// The `DriverCallback` calls so far.
pub fn callbacks() -> Vec<Callback> {
    update(|state| state.callbacks.clone())
}

/// 1 when the device's script sets `flag`, else 0.
fn fails(dev: usize, flag: char) -> u32 {
    update(|state| u32::from(state.devices[dev].flags.contains(flag)))
}

/// The host and registry stub.
pub struct Stub;

impl MidiOut for Stub {
    fn num_devs(&self) -> usize {
        update(|state| state.devices.len())
    }

    fn dev_name(&self, dev: usize) -> Option<Vec<u16>> {
        update(|state| {
            let device = &state.devices[dev];
            (!device.flags.contains('c')).then(|| device.name.encode_utf16().collect())
        })
    }

    fn open(&mut self, dev: usize) -> Option<usize> {
        let ret = fails(dev, 'o');
        log(format!("o{dev}:{ret}"));
        (ret == 0).then_some(dev)
    }

    fn close(&mut self, handle: usize) -> u32 {
        let ret = fails(handle, 'x');
        log(format!("c{handle}:{ret}"));
        ret
    }

    fn short_msg(&mut self, handle: usize, msg: u32) -> u32 {
        let ret = fails(handle, 's');
        log(format!("s{handle}:{msg:x}:{ret}"));
        ret
    }

    fn long_msg(&mut self, handle: usize, _hdr: &MidiHdr) -> u32 {
        let ret = fails(handle, 'l');
        log(format!("l{handle}:{ret}"));
        ret
    }

    fn prepare(&mut self, handle: usize, _hdr: &MidiHdr) {
        log(format!("p{handle}"));
    }

    fn unprepare(&mut self, handle: usize, _hdr: &MidiHdr) {
        log(format!("u{handle}"));
    }

    fn reset(&mut self, handle: usize) -> u32 {
        let ret = fails(handle, 'r');
        log(format!("r{handle}:{ret}"));
        ret
    }

    fn callback(&self, desc: &MidiOpenDesc, flags: u16, msg: u32, p1: usize, p2: usize) {
        update(|state| state.callbacks.push((*desc, flags, msg, p1, p2)));
        log(format!("n{msg:x}/{flags}"));
    }
}

/// A key of the stub registry: its hive and its path below the root.
pub struct StubKey {
    hive: char,
    path: String,
}

impl Registry for Stub {
    type Key = StubKey;

    fn open_root(&self, root: Root, path: &str) -> Option<StubKey> {
        let hive = match (root, path) {
            (Root::CurrentUser, USER_KEY) => 'u',
            (Root::LocalMachine, SCHEMES_KEY) => 'm',
            _ => return None,
        };
        key_exists(hive, "").then(|| StubKey {
            hive,
            path: String::new(),
        })
    }

    fn open_sub(&self, key: &StubKey, name: &[u16]) -> Option<StubKey> {
        let name = String::from_utf16_lossy(name);
        let path = if key.path.is_empty() {
            name
        } else {
            format!("{}/{name}", key.path)
        };
        let denied = update(|state| {
            state
                .denied
                .iter()
                .any(|(hive, denied)| *hive == key.hive && *denied == path)
        });
        (!denied && key_exists(key.hive, &path)).then_some(StubKey {
            hive: key.hive,
            path,
        })
    }

    fn sub_key_name(&self, key: &StubKey, index: u32) -> Option<Vec<u16>> {
        update(|state| {
            let mut children: Vec<&str> = Vec::new();
            for entry in state.entries.iter().filter(|entry| entry.hive == key.hive) {
                if let Some(child) = child_name(&entry.path, &key.path) {
                    if !children.contains(&child) {
                        children.push(child);
                    }
                }
            }
            children
                .get(index as usize)
                .map(|child| child.encode_utf16().collect())
        })
    }

    fn value(&self, key: &StubKey, name: &str) -> Option<Vec<u8>> {
        update(|state| {
            state
                .entries
                .iter()
                .find(|entry| {
                    entry.hive == key.hive && entry.path == key.path && entry.name == name
                })
                .map(|entry| entry.data.clone())
        })
    }

    fn close(&self, _key: StubKey) {}
}

/// The first path component of `path` below `parent`. The scheme root is the empty path, so
/// there every path is below it.
fn child_name<'a>(path: &'a str, parent: &str) -> Option<&'a str> {
    let rest = if parent.is_empty() {
        path
    } else {
        path.strip_prefix(parent)?.strip_prefix('/')?
    };
    rest.split('/').next().filter(|name| !name.is_empty())
}

/// Whether `path` is a key in `hive`: the root exists once the hive has an entry, and any other
/// path exists when an entry lies below it.
fn key_exists(hive: char, path: &str) -> bool {
    update(|state| {
        state.entries.iter().any(|entry| {
            entry.hive == hive
                && (path.is_empty()
                    || entry.path == path
                    || entry
                        .path
                        .strip_prefix(path)
                        .is_some_and(|rest| rest.starts_with('/')))
        })
    })
}
