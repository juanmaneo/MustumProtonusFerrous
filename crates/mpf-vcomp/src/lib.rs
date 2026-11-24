//! Port of the deterministic logic in Wine's `dlls/vcomp/main.c`: the reduction combines, the
//! static and dynamic loop schedulers, the `C2VectParallel` range split, and the `single` and
//! `sections` counters.
//!
//! Ported:
//! - [`reduction`]: the value stored by each `_vcomp_reduction_{i1,i2,i4,i8,r4,r8}` loop.
//! - [`schedule`]: `_vcomp_for_static_simple_init{,_i8}`, `_vcomp_for_static_init{,_i8}`,
//!   `_vcomp_for_dynamic_init` and `_vcomp_for_dynamic_next`.
//! - [`vect`]: the `nargs` check and serial-fallback test of `C2VectParallel`, and the static and
//!   dynamic range split of `c2vectparallel_wrapper`.
//! - [`workshare`]: `_vcomp_single_begin`, `_vcomp_sections_init` and `_vcomp_sections_next`.
//!
//! Skipped:
//! - Fork, team and worker-thread plumbing, barriers, TLS and the critical section. The ported
//!   functions are the state transitions only. The caller provides the mutual exclusion that Wine
//!   takes with `vcomp_section`.
//! - The `Interlocked*` retry loops. They store the value that the ported combine returns.
//! - `_vcomp_atomic_sub_*`, `_vcomp_atomic_div_*` and `_vcomp_atomic_shl_*`/`shr_*`. Division by
//!   zero and shifts by the operand width or more are undefined in C.
//! - The `omp_*` getters and setters and `_vcomp_master_*`, which carry no logic beyond thread state.
//!
//! Divergences from C:
//! - Thread numbers and team sizes are `u32`. Signed arithmetic that can overflow in C is done with
//!   wrapping operations, which is what the compiled code computes. The oracle excludes those inputs.
//! - Where the C leaves an output untouched on an early return, the port returns `None` or an
//!   explicit `Option`.
//! - `num_threads` and `thread_count` must be at least 1, and `step` must be nonzero where the C
//!   divides by it. Violating either panics.

pub mod reduction;
pub mod schedule;
pub mod vect;
pub mod workshare;

/// `VCOMP_DYNAMIC_FLAGS_STATIC`.
pub const DYNAMIC_FLAGS_STATIC: u32 = 0x01;
/// `VCOMP_DYNAMIC_FLAGS_CHUNKED`.
pub const DYNAMIC_FLAGS_CHUNKED: u32 = 0x02;
/// `VCOMP_DYNAMIC_FLAGS_GUIDED`.
pub const DYNAMIC_FLAGS_GUIDED: u32 = 0x03;
/// `VCOMP_DYNAMIC_FLAGS_INCREMENT`.
pub const DYNAMIC_FLAGS_INCREMENT: u32 = 0x40;

/// `(int)(a - b) > 0`: the wrap-aware ordering of the generation counters.
pub fn is_after(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) > 0
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
