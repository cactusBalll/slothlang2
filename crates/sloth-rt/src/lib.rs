//! sloth-rt: the sloth2 runtime library (libsloth_rt).
//! Modules by functionality: alloc (deterministic chunks behind the rc
//! core), panics, console, strings, arrays, maps, objects, vtables,
//! externs. All exported symbols keep their C-ABI names.

pub mod alloc;
pub mod any;
pub mod arrays;
pub mod boxopt;
pub mod builtins;
pub mod bytes;
pub mod channel;
pub mod console;
pub mod event;
pub mod externs;
pub mod fiber;
pub mod maps;
pub mod math;
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
