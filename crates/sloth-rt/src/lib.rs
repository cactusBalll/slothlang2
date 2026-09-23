//! sloth-rt: the sloth2 runtime library (libsloth_rt).
//! Modules by functionality: alloc (deterministic chunks behind the rc
//! core), panics, console, strings, objects, vtables, externs.
//! Container (Array/Map) algorithms live in the self-hosted sloth prelude
//! (`lib/prelude/containers.slt`); the runtime only exposes bare allocation,
//! word memory and the rc core for them.
//! All exported symbols keep their C-ABI names.

pub mod alloc;
pub mod any;
pub mod boxopt;
pub mod builtins;
pub mod bytes;
pub mod channel;
pub mod console;
pub mod event;
pub mod externs;
pub mod fiber;
pub mod math;
pub mod mem;
pub mod mmap;
pub mod net;
pub mod objects;
pub mod panics;
pub mod ranges;
pub mod rc;
pub mod strings;
pub mod sync;
pub mod tensors;
pub mod thread;
pub mod time;
pub mod vtable;
