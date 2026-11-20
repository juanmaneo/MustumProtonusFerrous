//! The QTM model state of `fdi.c`: the adaptive frequency models that the QTM
//! decoder updates after each symbol (`QTMupdatemodel`), and the length,
//! position and model tables that `QTMfdi_init` sets up. The decoder and the
//! window buffer are not ported.

use crate::Error;

/// One symbol of a [`Model`] with its cumulative frequency (`struct QTMmodelsym`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sym {
    pub sym: u16,
    pub cumfreq: u16,
}

/// An adaptive model over `entries` symbols (`struct QTMmodel`). `syms` holds
/// `entries + 1` records, the last one a zero sentinel. `tabloc` maps each
/// symbol to its record in `syms`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    pub shiftsleft: u32,
    pub entries: usize,
    pub syms: Vec<Sym>,
    pub tabloc: [u16; 256],
}

impl Model {
    /// `QTMfdi_initmodel`: a model over the symbols `start..start + entries`,
    /// where symbol `i` starts with frequency `entries - i`. Panics if
    /// `start + entries` exceeds the 256-entry lookup table.
    pub fn new(entries: usize, start: usize) -> Model {
        let mut tabloc = [u16::MAX; 256];
        let mut syms = Vec::with_capacity(entries + 1);
        for i in 0..entries {
            tabloc[start + i] = i as u16;
            syms.push(Sym {
                sym: (start + i) as u16,
                cumfreq: (entries - i) as u16,
            });
        }
        syms.push(Sym { sym: 0, cumfreq: 0 });
        Model {
            shiftsleft: 4,
            entries,
            syms,
            tabloc,
        }
    }

    /// `QTMupdatemodel`: adds 8 to the cumulative frequency of each record
    /// before index `sym`, then rescales once `syms[0]` passes 3800. Panics if
    /// `sym` is greater than `entries + 1`, the length of `syms`.
    pub fn update(&mut self, sym: usize) {
        for s in &mut self.syms[..sym] {
            s.cumfreq = s.cumfreq.wrapping_add(8);
        }
        if self.syms[0].cumfreq > 3800 {
            self.shiftsleft = self.shiftsleft.wrapping_sub(1);
            if self.shiftsleft != 0 {
                self.halve();
            } else {
                self.shiftsleft = 50;
                self.rebuild();
            }
        }
    }

    /// The `--shiftsleft` branch of `QTMupdatemodel`: halves each cumulative
    /// frequency and keeps the table strictly decreasing.
    fn halve(&mut self) {
        for i in (0..self.entries).rev() {
            let halved = self.syms[i].cumfreq >> 1;
            let next = self.syms[i + 1].cumfreq;
            self.syms[i].cumfreq = if halved <= next {
                next.wrapping_add(1)
            } else {
                halved
            };
        }
    }

    /// The rebuild branch of `QTMupdatemodel`: turns cumulative frequencies
    /// into halved frequencies, sorts them into decreasing order with a
    /// selection sort and turns them back into cumulative frequencies.
    /// Finally it updates `tabloc`.
    fn rebuild(&mut self) {
        for i in 0..self.entries {
            let freq = self.syms[i].cumfreq.wrapping_sub(self.syms[i + 1].cumfreq);
            self.syms[i].cumfreq = freq.wrapping_add(1) >> 1;
        }
        for i in 0..self.entries.saturating_sub(1) {
            for j in i + 1..self.entries {
                if self.syms[i].cumfreq < self.syms[j].cumfreq {
                    self.syms.swap(i, j);
                }
            }
        }
        for i in (0..self.entries).rev() {
            self.syms[i].cumfreq = self.syms[i].cumfreq.wrapping_add(self.syms[i + 1].cumfreq);
        }
        for (i, s) in self.syms[..self.entries].iter().enumerate() {
            self.tabloc[usize::from(s.sym)] = i as u16;
        }
    }
}

/// The QTM tables and models that `QTMfdi_init` sets up, without the window
/// buffer (`struct QTMstate` and the `q_*` tables of `fdi_decomp_state`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QtmState {
    pub window_size: u32,
    pub length_base: [u32; 27],
    pub length_extra: [u8; 27],
    pub extra_bits: [u8; 42],
    pub position_base: [u32; 42],
    pub model7: Model,
    pub model00: Model,
    pub model40: Model,
    pub model80: Model,
    pub model_c0: Model,
    pub model4: Model,
    pub model5: Model,
    pub model6pos: Model,
    pub model6len: Model,
}

impl QtmState {
    /// `QTMfdi_init` for a window of `2^window` bytes. The window buffer is not
    /// allocated. Windows outside 2^10 to 2^21 are a [`Error::DataFormat`].
    pub fn init(window: i32) -> Result<QtmState, Error> {
        if !(10..=21).contains(&window) {
            return Err(Error::DataFormat);
        }
        let msz = (2 * window) as usize;

        let length_extra: [u8; 27] = std::array::from_fn(|i| {
            if i == 26 {
                0
            } else {
                ((i.saturating_sub(2)) >> 2) as u8
            }
        });
        let mut length_base = [0u32; 27];
        let mut base = 0u32;
        for (i, slot) in length_base.iter_mut().enumerate() {
            *slot = base;
            let bits = if i == 26 { 5 } else { length_extra[i] };
            base += 1 << bits;
        }

        let extra_bits: [u8; 42] = std::array::from_fn(|i| (i.saturating_sub(2) >> 1) as u8);
        let mut position_base = [0u32; 42];
        let mut base = 0u32;
        for (slot, &bits) in position_base.iter_mut().zip(&extra_bits) {
            *slot = base;
            base += 1 << bits;
        }

        Ok(QtmState {
            window_size: 1 << window,
            length_base,
            length_extra,
            extra_bits,
            position_base,
            model7: Model::new(7, 0),
            model00: Model::new(0x40, 0x00),
            model40: Model::new(0x40, 0x40),
            model80: Model::new(0x40, 0x80),
            model_c0: Model::new(0x40, 0xC0),
            model4: Model::new(msz.min(24), 0),
            model5: Model::new(msz.min(36), 0),
            model6pos: Model::new(msz, 0),
            model6len: Model::new(27, 0),
        })
    }
}
