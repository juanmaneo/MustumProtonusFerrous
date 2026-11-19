//! SoundFont modulator and generator parsing from `dlls/dmusic/instrument.c`, with the DLS connection
//! values from `dls1.h` and the SoundFont enumerations from `dlls/dmusic/soundfont.h` that it uses.

pub const CONN_SRC_NONE: u16 = 0x000;
pub const CONN_SRC_KEYONVELOCITY: u16 = 0x002;
pub const CONN_SRC_KEYNUMBER: u16 = 0x003;
pub const CONN_SRC_PITCHWHEEL: u16 = 0x006;
pub const CONN_SRC_POLYPRESSURE: u16 = 0x007;
pub const CONN_SRC_CHANNELPRESSURE: u16 = 0x008;
pub const CONN_SRC_CC1: u16 = 0x081;
pub const CONN_SRC_RPN0: u16 = 0x100;

pub const CONN_TRN_NONE: u16 = 0x000;
pub const CONN_TRN_CONCAVE: u16 = 0x001;
pub const CONN_TRN_CONVEX: u16 = 0x002;
pub const CONN_TRN_SWITCH: u16 = 0x003;
pub const CONN_TRN_BIPOLAR: u16 = 1 << 4;
pub const CONN_TRN_INVERT: u16 = 1 << 5;

const SF_MOD_CTRL_MIDI: u16 = 0x80;
const SF_MOD_DIR_DECREASING: u16 = 0x100;
const SF_MOD_POL_BIPOLAR: u16 = 0x200;

pub const SF_GEN_START_ADDRS_OFFSET: u16 = 0;
pub const SF_GEN_END_ADDRS_OFFSET: u16 = 1;
pub const SF_GEN_STARTLOOP_ADDRS_OFFSET: u16 = 2;
pub const SF_GEN_ENDLOOP_ADDRS_OFFSET: u16 = 3;
pub const SF_GEN_START_ADDRS_COARSE_OFFSET: u16 = 4;
pub const SF_GEN_END_ADDRS_COARSE_OFFSET: u16 = 12;
pub const SF_GEN_PAN: u16 = 17;
pub const SF_GEN_INSTRUMENT: u16 = 41;
pub const SF_GEN_STARTLOOP_ADDRS_COARSE_OFFSET: u16 = 45;
pub const SF_GEN_KEYNUM: u16 = 46;
pub const SF_GEN_VELOCITY: u16 = 47;
pub const SF_GEN_INITIAL_ATTENUATION: u16 = 48;
pub const SF_GEN_ENDLOOP_ADDRS_COARSE_OFFSET: u16 = 50;
pub const SF_GEN_SAMPLE_ID: u16 = 53;
pub const SF_GEN_SAMPLE_MODES: u16 = 54;
pub const SF_GEN_EXCLUSIVE_CLASS: u16 = 57;
pub const SF_GEN_OVERRIDING_ROOT_KEY: u16 = 58;
pub const SF_GEN_END_OPER: u16 = 60;

/// `mod_ctrl_to_conn_src`, indexed by the controller field of a modulator source. Unlisted entries
/// are `CONN_SRC_NONE`, as in the C designated initializer.
const MOD_CTRL_TO_CONN_SRC: [u16; 17] = [
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_KEYONVELOCITY,
    CONN_SRC_KEYNUMBER,
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_POLYPRESSURE,
    CONN_SRC_NONE,
    CONN_SRC_NONE,
    CONN_SRC_CHANNELPRESSURE,
    CONN_SRC_PITCHWHEEL,
    CONN_SRC_NONE,
    CONN_SRC_RPN0,
];

/// `struct sf_bag`: the first generator and the first modulator of a bag, as table indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SfBag {
    pub gen_ndx: u16,
    pub mod_ndx: u16,
}

/// `struct sf_gen`. `amount` is the `WORD` value. For a range generator it holds the `low` and
/// `high` bytes, which are the same bits on a little-endian host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SfGen {
    pub oper: u16,
    pub amount: u16,
}

/// `struct sf_mod`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SfMod {
    pub src_mod: u16,
    pub dest_gen: u16,
    pub amount: i16,
    pub amount_src_mod: u16,
    pub transform: u16,
}

/// `struct sf_generators`: the value of each generator, indexed by its operator. Zero means unset.
pub type SfGenerators = [u16; SF_GEN_END_OPER as usize];

/// `mod_src_to_conn_src`: the DLS connection source for a SoundFont modulator source. A MIDI
/// controller source (bit 7 set) maps to `CONN_SRC_CC1` plus its controller number. Other sources
/// come from the table, and an index past the table maps to `CONN_SRC_NONE`.
pub fn mod_src_to_conn_src(src: u16) -> u16 {
    if src & SF_MOD_CTRL_MIDI != 0 {
        return CONN_SRC_CC1 - 1 + (src & 0x7F);
    }
    MOD_CTRL_TO_CONN_SRC
        .get(usize::from(src & 0x7F))
        .copied()
        .unwrap_or(CONN_SRC_NONE)
}

/// `mod_src_to_conn_transform`: the DLS transform flags for a SoundFont modulator source. The
/// direction and polarity bits give `CONN_TRN_INVERT` and `CONN_TRN_BIPOLAR`, and the curve bits
/// give the shape.
pub fn mod_src_to_conn_transform(src: u16) -> u16 {
    let mut transform = 0;
    if src & SF_MOD_DIR_DECREASING != 0 {
        transform |= CONN_TRN_INVERT;
    }
    if src & SF_MOD_POL_BIPOLAR != 0 {
        transform |= CONN_TRN_BIPOLAR;
    }
    transform
        | match (src >> 10) & 3 {
            0 => CONN_TRN_NONE,
            1 => CONN_TRN_CONCAVE,
            2 => CONN_TRN_CONVEX,
            _ => CONN_TRN_SWITCH,
        }
}

/// `parse_soundfont_modulators`: appends the modulators of bag `index` to `out`. A modulator with
/// the same destination, source, amount source and transform as one already in `out` replaces it.
pub fn parse_modulators(bags: &[SfBag], mods: &[SfMod], index: usize, out: &mut Vec<SfMod>) {
    let start = usize::from(bags[index].mod_ndx);
    let end = usize::from(bags[index + 1].mod_ndx);
    for &m in &mods[start..end] {
        let found = out.iter().position(|o| {
            o.dest_gen == m.dest_gen
                && o.src_mod == m.src_mod
                && o.amount_src_mod == m.amount_src_mod
                && o.transform == m.transform
        });
        match found {
            Some(i) => out[i] = m,
            None => out.push(m),
        }
    }
}

/// `parse_soundfont_generators`: stores the generators of bag `index` in `out`. Returns `false` when
/// the bag ends early at an `instrument` generator (in a preset) or a `sample_id` generator (in an
/// instrument), which the format requires to be last. Returns `true` otherwise.
pub fn parse_generators(
    bags: &[SfBag],
    gens: &[SfGen],
    index: usize,
    preset: bool,
    out: &mut SfGenerators,
) -> bool {
    let start = usize::from(bags[index].gen_ndx);
    let end = usize::from(bags[index + 1].gen_ndx);
    for &gen in &gens[start..end] {
        match gen.oper {
            SF_GEN_START_ADDRS_OFFSET
            | SF_GEN_END_ADDRS_OFFSET
            | SF_GEN_STARTLOOP_ADDRS_OFFSET
            | SF_GEN_ENDLOOP_ADDRS_OFFSET
            | SF_GEN_START_ADDRS_COARSE_OFFSET
            | SF_GEN_END_ADDRS_COARSE_OFFSET
            | SF_GEN_STARTLOOP_ADDRS_COARSE_OFFSET
            | SF_GEN_KEYNUM
            | SF_GEN_VELOCITY
            | SF_GEN_ENDLOOP_ADDRS_COARSE_OFFSET
            | SF_GEN_SAMPLE_MODES
            | SF_GEN_EXCLUSIVE_CLASS
            | SF_GEN_OVERRIDING_ROOT_KEY => {
                if !preset {
                    set_generator(out, gen);
                }
            }
            SF_GEN_INSTRUMENT => {
                if preset {
                    set_generator(out, gen);
                }
                return false;
            }
            SF_GEN_SAMPLE_ID => {
                if !preset {
                    set_generator(out, gen);
                }
                return false;
            }
            _ => set_generator(out, gen),
        }
    }
    true
}

fn set_generator(out: &mut SfGenerators, gen: SfGen) {
    if let Some(slot) = out.get_mut(usize::from(gen.oper)) {
        *slot = gen.amount;
    }
}
