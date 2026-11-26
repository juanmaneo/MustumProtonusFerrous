use crate::hresult::{S_OK, XAUDIO2_E_INVALID_CALL};

/// Port of the callback table and mastering-voice latch in `IXAudio2Impl` (`xaudio_dll.c`).
///
/// A callback is modelled by its pointer value and a NULL callback by `None`. Each `Vec` entry
/// is one of the `ncbs` slots, so NULL slots remain until a later shift overwrites them.
#[derive(Default)]
pub struct Engine {
    callbacks: Vec<Option<usize>>,
    mastering_in_use: bool,
}

impl Engine {
    pub fn callbacks(&self) -> &[Option<usize>] {
        &self.callbacks
    }

    pub fn mastering_in_use(&self) -> bool {
        self.mastering_in_use
    }

    /// Port of `IXAudio2Impl_RegisterForCallbacks`. Reuses the first slot that is NULL or already
    /// holds `callback`, and otherwise appends a slot.
    pub fn register_for_callbacks(&mut self, callback: Option<usize>) -> u32 {
        for slot in self.callbacks.iter_mut() {
            if slot.is_none() || *slot == callback {
                *slot = callback;
                return S_OK;
            }
        }
        self.callbacks.push(callback);
        S_OK
    }

    /// Port of `IXAudio2Impl_UnregisterForCallbacks`. Finds the first slot equal to `callback`
    /// (or the end of the table if there is none), shifts the following entries left while they
    /// are non-NULL, then clears the slot that was vacated. The C `ncbs == 0` early return is not
    /// kept as a separate branch: with no slots, the search and shift loops do nothing.
    pub fn unregister_for_callbacks(&mut self, callback: Option<usize>) {
        let n = self.callbacks.len();
        let mut i = self
            .callbacks
            .iter()
            .position(|slot| *slot == callback)
            .unwrap_or(n);
        while i + 1 < n && self.callbacks[i + 1].is_some() {
            self.callbacks[i] = self.callbacks[i + 1];
            i += 1;
        }
        if i < n {
            self.callbacks[i] = None;
        }
    }

    /// Port of the in-use check in `IXAudio2Impl_CreateMasteringVoice`. The latch is set after
    /// the FAudio call whatever its result, which the C code ignores. The effect chain argument
    /// is not modelled.
    pub fn create_mastering_voice(&mut self) -> u32 {
        if self.mastering_in_use {
            return XAUDIO2_E_INVALID_CALL;
        }
        self.mastering_in_use = true;
        S_OK
    }
}
