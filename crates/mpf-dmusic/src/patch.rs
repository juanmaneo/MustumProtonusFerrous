//! `MIDILOCALE2Patch` and `Patch2MIDILOCALE` from `dlls/dmusic/dmusic_main.c`.

/// `F_INSTRUMENT_DRUMS`: the drum bit, which sits in bit 31 of both a bank and a patch.
const F_INSTRUMENT_DRUMS: u32 = 0x8000_0000;

/// `MIDILOCALE`: a MIDI bank and a patch (instrument) number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MidiLocale {
    pub bank: u32,
    pub instrument: u32,
}

/// `MIDILOCALE2Patch`: packs a locale into a patch value. The drum bit stays in bit 31, the two
/// 7-bit bank fields go to bits 8 to 22 and the patch number to bits 0 to 6. A missing locale gives 0.
pub fn midilocale_to_patch(locale: Option<&MidiLocale>) -> u32 {
    locale.map_or(0, |locale| {
        (locale.bank & F_INSTRUMENT_DRUMS)
            | ((locale.bank & 0x0000_7F7F) << 8)
            | (locale.instrument & 0x0000_007F)
    })
}

/// `Patch2MIDILOCALE`: unpacks a patch value into a locale. The bank keeps the drum bit and the
/// two 7-bit fields, so the result is the inverse of `midilocale_to_patch` on those bits.
pub fn patch_to_midilocale(patch: u32) -> MidiLocale {
    MidiLocale {
        bank: ((patch & 0x007F_7F00) >> 8) | (patch & F_INSTRUMENT_DRUMS),
        instrument: patch & 0x0000_007F,
    }
}
