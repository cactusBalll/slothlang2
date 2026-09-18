//! sloth-rt: the sloth2 runtime library (libsloth_rt).
//! Modules by functionality: alloc (deterministic chunks behind the rc
//! core), panics, console, strings, arrays, maps, objects, vtables,
//! externs. All exported symbols keep their C-ABI names.

pub mod alloc;
pub mod arrays;
pub mod boxopt;
pub mod console;
pub mod externs;
pub mod maps;
pub mod objects;
pub mod panics;
pub mod ranges;
pub mod rc;
pub mod strings;
pub mod tensors;
pub mod vtable;
