//! The mapper's table of midi out ports, and `MIDIMAP_FindPort`.

use crate::MidiOut;

/// `MAXPNAMELEN`: the size of the port name buffer, including the terminator.
pub const MAXPNAMELEN: usize = 32;

const HASH: u16 = b'#' as u16;
const DIGIT_ZERO: u16 = b'0' as u16;
const DIGIT_NINE: u16 = b'9' as u16;

/// Where a port is in its open lifecycle. C keeps this as `MIDIOUTPORT.loaded`: 0, 1 and -1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortState {
    /// Not opened yet (`loaded == 0`).
    Idle,
    /// Opened on the host with this handle (`loaded > 0`).
    Open {
        /// The handle returned by `MidiOut::open`.
        handle: usize,
    },
    /// The open failed, or the capability query failed (`loaded < 0`). Never retried.
    Failed,
}

/// One entry of the mapper's port table.
pub struct OutPort {
    /// The device name, clipped to `MAXPNAMELEN - 1` code units. `None` when the
    /// capability query failed.
    pub name: Option<Vec<u16>>,
    /// The open state.
    pub state: PortState,
}

/// `DRVM_INIT`'s table build: one entry per host device, in device index order.
pub fn enumerate(midi: &impl MidiOut) -> Vec<OutPort> {
    (0..midi.num_devs())
        .map(|dev| match midi.dev_name(dev) {
            Some(raw) => OutPort {
                name: Some(clip(&raw)),
                state: PortState::Idle,
            },
            None => OutPort {
                name: None,
                state: PortState::Failed,
            },
        })
        .collect()
}

fn clip(raw: &[u16]) -> Vec<u16> {
    raw.iter()
        .take(MAXPNAMELEN - 1)
        .copied()
        .take_while(|&unit| unit != 0)
        .collect()
}

/// `MIDIMAP_FindPort`: the index of the port whose name equals `name`, else the index
/// that a `#n` name spells, if it is below the port count.
pub fn find_port(ports: &[OutPort], name: &[u16]) -> Option<usize> {
    if let Some(dev) = ports
        .iter()
        .position(|port| port.name.as_deref() == Some(name))
    {
        return Some(dev);
    }
    let [HASH, digits @ ..] = name else {
        return None;
    };
    if !digits.first().is_some_and(|&unit| is_digit(unit)) {
        return None;
    }
    let dev = digits
        .iter()
        .take_while(|&&unit| is_digit(unit))
        .fold(0u32, |dev, &unit| {
            dev.wrapping_mul(10)
                .wrapping_add(u32::from(unit - DIGIT_ZERO))
        });
    ((dev as usize) < ports.len()).then_some(dev as usize)
}

fn is_digit(unit: u16) -> bool {
    (DIGIT_ZERO..=DIGIT_NINE).contains(&unit)
}
