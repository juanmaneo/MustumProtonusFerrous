//! The state of one open waveform-audio device, and its notification queue.

use crate::format::{WaveFormat, PCM_DEFAULT};
use crate::{
    MCI_FORMAT_MILLISECONDS, MCI_MODE_STOP, MCI_NOTIFY, MCI_NOTIFY_ABORTED, MCI_NOTIFY_FAILURE,
    MCI_NOTIFY_SUCCESSFUL, MCI_NOTIFY_SUPERSEDED, WAVE_MAPPER,
};

/// The results of the winmm calls that the C code would make.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Env {
    /// The return code of waveOutPause, waveInStop, waveOutRestart, waveInStart
    /// and the reset calls. Zero means success.
    pub control_ret: u32,
    /// Whether waveOutOpen or waveInOpen succeeds.
    pub open_ok: bool,
    pub wave_in_devs: u32,
    pub wave_out_devs: u32,
}

/// The fields of WINE_MCIWAVE that the commands read and write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveDevice {
    pub time_format: u32,
    pub format: WaveFormat,
    /// True while the format is the default, which is when the C code points
    /// lpWaveFormat at wfxRef.
    pub format_is_default: bool,
    pub input: u16,
    pub output: u16,
    pub status: u32,
    /// True when the last play or record used the input side.
    pub f_input: bool,
    pub position: u32,
    /// The size of the data chunk.
    pub cksize: u32,
    pub has_file: bool,
    pub file_name: Option<String>,
    /// True while a notify callback is pending (the hCallback field is set).
    pub has_callback: bool,
    /// The notifications sent, in order.
    pub sent: Vec<u32>,
}

impl Default for WaveDevice {
    fn default() -> Self {
        Self {
            time_format: MCI_FORMAT_MILLISECONDS,
            format: PCM_DEFAULT,
            format_is_default: true,
            input: WAVE_MAPPER,
            output: WAVE_MAPPER,
            status: MCI_MODE_STOP,
            f_input: false,
            position: 0,
            cksize: 0,
            has_file: false,
            file_name: None,
            has_callback: false,
            sent: Vec::new(),
        }
    }
}

impl WaveDevice {
    /// WAVE_mciNotify: a pending callback is superseded before the new status is sent.
    fn notify(&mut self, status: u32) {
        if self.has_callback {
            self.has_callback = false;
            self.sent.push(MCI_NOTIFY_SUPERSEDED);
        }
        self.sent.push(status);
    }

    /// Sends MCI_NOTIFY_SUCCESSFUL when the command succeeded with MCI_NOTIFY.
    pub fn notify_success(&mut self, flags: u32, success: bool) {
        if success && (flags & MCI_NOTIFY) != 0 {
            self.notify(MCI_NOTIFY_SUCCESSFUL);
        }
    }

    /// Aborts the pending callback, if any.
    pub fn abort_callback(&mut self) {
        if self.has_callback {
            self.has_callback = false;
            self.sent.push(MCI_NOTIFY_ABORTED);
        }
    }

    /// Replaces the pending callback with the one of a new command. The old one
    /// is aborted. The new one is kept only with MCI_NOTIFY and a non-zero
    /// callback window (its low word, as the C code takes it).
    pub fn swap_callback(&mut self, flags: u32, callback: u32) {
        self.abort_callback();
        self.has_callback = (flags & MCI_NOTIFY) != 0 && (callback & 0xFFFF) != 0;
    }

    /// The cleanup of play and record: the device stops, and a pending callback
    /// reports FAILURE or SUCCESSFUL. Returns `ret` unchanged.
    pub fn end_playback(&mut self, flags: u32, ret: u32) -> u32 {
        self.status = MCI_MODE_STOP;
        if (flags & MCI_NOTIFY) != 0 && self.has_callback {
            self.has_callback = false;
            self.sent.push(if ret == 0 {
                MCI_NOTIFY_SUCCESSFUL
            } else {
                MCI_NOTIFY_FAILURE
            });
        }
        ret
    }

    /// The state part of WAVE_mciCreateRIFFSkeleton: the format is normalised
    /// and no longer the default, and the data chunk starts empty.
    pub fn riff_skeleton(&mut self) {
        self.format.normalize_pcm();
        self.format_is_default = false;
        self.cksize = 0;
    }

    /// WAVE_mciDefaultFmt.
    pub fn set_default_format(&mut self) {
        self.format = PCM_DEFAULT;
        self.format_is_default = true;
    }
}
