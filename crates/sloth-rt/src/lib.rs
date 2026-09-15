//! sloth-rt: the sloth2 runtime library (libsloth_rt).
//! Modules by functionality: GC, panics, console, strings, arrays, maps,
//! objects, vtables, externs. All exported symbols keep their C-ABI names.

pub mod arrays;
pub mod console;
pub mod externs;
pub mod gc;
pub mod maps;
pub mod objects;
pub mod panics;
pub mod strings;
pub mod vtable;
