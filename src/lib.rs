#![no_std]
//! Compatibility shim. atomic-polyfill is deprecated upstream (RUSTSEC-2023-0089); its own
//! README points at portable-atomic. This fork re-exports portable-atomic under the
//! atomic-polyfill name so a transitive consumer (heapless 0.7) builds against maintained
//! code with no unmaintained advisory. Retire when the consumer migrates to portable-atomic.
pub use portable_atomic::*;
