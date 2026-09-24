//! Scalar math faces (design D6): C-ABI libm wrappers for sloth source calls
//! (`float_sqrt` etc.). Tensor kernels use MLIR `math.*` instead, which lowers
//! to the same LLVM intrinsics.

#[no_mangle]
pub extern "C" fn __sloth_rt_sqrt(x: f64) -> f64 {
    x.sqrt()
}

#[no_mangle]
pub extern "C" fn __sloth_rt_exp(x: f64) -> f64 {
    x.exp()
}

#[no_mangle]
pub extern "C" fn __sloth_rt_sin(x: f64) -> f64 {
    x.sin()
}

#[no_mangle]
pub extern "C" fn __sloth_rt_cos(x: f64) -> f64 {
    x.cos()
}

#[no_mangle]
pub extern "C" fn __sloth_rt_tan(x: f64) -> f64 {
    x.tan()
}

#[no_mangle]
pub extern "C" fn __sloth_rt_pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

#[no_mangle]
pub extern "C" fn __sloth_rt_floor(x: f64) -> f64 {
    x.floor()
}
