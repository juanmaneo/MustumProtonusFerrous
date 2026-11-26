//! Port of the portable logic in Wine's dlls/bcryptprimitives/main.c.
//!
//! Ported:
//! - ProcessPrng(data, size): fill size bytes in chunks of at most ULONG_MAX,
//!   stopping at the first failed RtlGenRandom call: [process_prng],
//!   [process_prng_chunks], [next_chunk_len], [ULONG_MAX]
//!
//! Skipped:
//! - RtlGenRandom (SystemFunction036, advapi32): the OS random source. It is
//!   injected as a callback, so the port never calls the OS.
//! - The stdcall export and BOOL return ABI (bcryptprimitives.spec). The Rust
//!   API returns bool, and the export wrapper is not ported.

mod prng;

pub use prng::{next_chunk_len, process_prng, process_prng_chunks, ULONG_MAX};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
