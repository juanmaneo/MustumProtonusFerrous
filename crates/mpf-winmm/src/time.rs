//! `timeSetEvent` and `timeKillEvent` from `dlls/winmm/time.c`, over the 16-slot `timers` table. The
//! callback, `dwUser`, the timer service thread and the synchronous kill are not modelled.

/// `MMSYSTIME_MININTERVAL` and `MMSYSTIME_MAXINTERVAL`, from `dlls/winmm/time.c`.
const MMSYSTIME_MININTERVAL: u32 = 1;
const MMSYSTIME_MAXINTERVAL: u32 = 65535;

/// `TIMERR_NOERROR` and `TIMERR_NOCANDO`, from `include/mmsystem.h`.
pub const TIMERR_NOERROR: u32 = 0;
pub const TIMERR_NOCANDO: u32 = 97;

/// `ARRAY_SIZE(timers)` in `dlls/winmm/time.c`.
const TIMER_SLOTS: usize = 16;

/// One `WINE_TIMERENTRY`, with only the fields that `timeSetEvent` and `timeKillEvent` read or write.
#[derive(Default)]
pub struct TimerEntry {
    pub id: u16,
    pub delay: u32,
    pub resol: u32,
    pub flags: u16,
    pub trigger_time: u32,
}

/// The `timers` array and the `timers_created` counter of `dlls/winmm/time.c`. A slot is free when
/// its id is 0.
#[derive(Default)]
pub struct TimerTable {
    pub timers: [TimerEntry; TIMER_SLOTS],
    created: u32,
}

impl TimerTable {
    /// `timeSetEvent` without the callback. `now` stands in for `timeGetTime()`. Returns the timer
    /// id, or 0 when `delay` is outside `MMSYSTIME_MININTERVAL..=MMSYSTIME_MAXINTERVAL` or every
    /// slot is in use.
    pub fn set_event(&mut self, delay: u32, resol: u32, flags: u32, now: u32) -> u32 {
        if !(MMSYSTIME_MININTERVAL..=MMSYSTIME_MAXINTERVAL).contains(&delay) {
            return 0;
        }
        let Some(slot) = self.timers.iter().position(|timer| timer.id == 0) else {
            return 0;
        };
        let mut id = self.next_id(slot);
        if id == 0 {
            // A zero id would make the slot look free, so the C body takes the next counter value.
            id = self.next_id(slot);
        }
        let timer = &mut self.timers[slot];
        timer.delay = delay;
        timer.trigger_time = now.wrapping_add(delay);
        timer.resol = resol;
        timer.flags = flags as u16;
        timer.id = id;
        u32::from(id)
    }

    /// `timeKillEvent` without the synchronous wait. The slot is `id % 16`, and an id that the slot
    /// does not hold is `TIMERR_NOCANDO`.
    pub fn kill_event(&mut self, id: u32) -> u32 {
        let timer = &mut self.timers[(id as usize) % TIMER_SLOTS];
        if u32::from(timer.id) != id {
            return TIMERR_NOCANDO;
        }
        timer.id = 0;
        TIMERR_NOERROR
    }

    /// `++timers_created` and the id for `slot`. The C body computes the id in `size_t` and stores
    /// it in a `WORD`, so the result is truncated to 16 bits.
    fn next_id(&mut self, slot: usize) -> u16 {
        self.created = self.created.wrapping_add(1);
        (TIMER_SLOTS as u64 * u64::from(self.created) + slot as u64) as u16
    }
}
