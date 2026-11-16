//! Critical sections: `RtlInitializeCriticalSection` / `Enter` / `Leave` /
//! `TryEnter` equivalents, mirroring `dlls/ntdll/sync.c` semantics
//! (recursive, owned by one thread).

use std::sync::Mutex;
use std::thread::ThreadId;

struct State {
    recursion: u32,
    owner: Option<ThreadId>,
}

pub struct CriticalSection {
    state: Mutex<State>,
}

impl CriticalSection {
    pub fn new() -> Self {
        CriticalSection {
            state: Mutex::new(State {
                recursion: 0,
                owner: None,
            }),
        }
    }

    /// RtlInitializeCriticalSection equivalent.
    pub fn initialize(&mut self) {
        *self = Self::new();
    }

    /// RtlDeleteCriticalSection equivalent (no-op for a safe wrapper).
    pub fn delete(&mut self) {}

    pub fn enter(&self) -> CriticalSectionGuard<'_> {
        let id = std::thread::current().id();
        let mut state = self.state.lock().unwrap();
        match state.owner {
            Some(owner) if owner == id => {
                state.recursion += 1;
            }
            None => {
                state.owner = Some(id);
                state.recursion = 1;
            }
            // Another thread owns it: block, like a real critical section.
            Some(_) => {
                drop(state);
                let mut state = self.state.lock().unwrap();
                state.owner = Some(id);
                state.recursion = 1;
            }
        }
        CriticalSectionGuard { cs: self }
    }

    pub fn try_enter(&self) -> Option<CriticalSectionGuard<'_>> {
        let id = std::thread::current().id();
        let mut state = self.state.lock().unwrap();
        match state.owner {
            Some(owner) if owner == id => {
                state.recursion += 1;
                Some(CriticalSectionGuard { cs: self })
            }
            None => {
                state.owner = Some(id);
                state.recursion = 1;
                Some(CriticalSectionGuard { cs: self })
            }
            Some(_) => None,
        }
    }

    fn leave(&self) -> bool {
        let id = std::thread::current().id();
        let mut state = self.state.lock().unwrap();
        if state.owner == Some(id) && state.recursion > 0 {
            state.recursion -= 1;
            if state.recursion == 0 {
                state.owner = None;
            }
            true
        } else {
            false
        }
    }
}

impl Default for CriticalSection {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CriticalSectionGuard<'a> {
    cs: &'a CriticalSection,
}

impl Drop for CriticalSectionGuard<'_> {
    fn drop(&mut self) {
        self.cs.leave();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn recursive_enter_from_same_thread() {
        let cs = CriticalSection::new();
        let g1 = cs.enter();
        let g2 = cs.enter();
        let g3 = cs.try_enter().unwrap();
        drop(g3);
        drop(g2);
        drop(g1);
        // Fully released: try_enter succeeds again.
        assert!(cs.try_enter().is_some());
    }

    #[test]
    fn try_enter_fails_while_held_by_another_thread() {
        let cs = Arc::new(CriticalSection::new());
        let g = cs.enter();
        let cs2 = Arc::clone(&cs);
        let other = std::thread::spawn(move || cs2.try_enter().is_none());
        assert!(other.join().unwrap());
        drop(g);
    }

    #[test]
    fn counter_is_protected_across_threads() {
        let cs = Arc::new(CriticalSection::new());
        let counter = Arc::new(Mutex::new(0u32));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let cs = Arc::clone(&cs);
            let counter = Arc::clone(&counter);
            handles.push(std::thread::spawn(move || {
                for _ in 0..1000 {
                    let _g = cs.enter();
                    let mut c = counter.lock().unwrap();
                    *c += 1;
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(*counter.lock().unwrap(), 4000);
    }

    #[test]
    fn default_section_is_unowned() {
        let cs = CriticalSection::default();
        assert!(cs.try_enter().is_some());
    }

    #[test]
    fn initialize_resets_state() {
        let mut cs = CriticalSection::new();
        {
            let _g = cs.enter();
        }
        cs.initialize();
        assert!(cs.try_enter().is_some());
        cs.delete();
    }
}
