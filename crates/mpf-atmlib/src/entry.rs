//! `ATMProperlyLoaded` and `ATMFinish` from `dlls/atmlib/main.c`.

/// `ATMProperlyLoaded`: the Wine stub returns `FALSE`.
pub fn atm_properly_loaded() -> bool {
    false
}

/// `ATMFinish`: the Wine stub returns `0`.
pub fn atm_finish() -> i32 {
    0
}
