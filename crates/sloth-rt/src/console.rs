//! Console I/O: i64/f64/bool primitives plus a hello marker.

#[no_mangle]
pub extern "C" fn sloth_rt_hello() {
    println!("libsloth_rt linked");
}

/// print an i64 value (display form); never returns useful value
#[no_mangle]
pub extern "C" fn sloth_rt_print_i64(v: i64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_f64(v: f64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_bool(v: i64) -> i64 {
    println!("{}", v != 0);
    0
}
