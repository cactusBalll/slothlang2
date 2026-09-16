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

/// value-optional box printing (kind: 0 = int, 1 = float, 2 = bool):
/// nil prints "nil" — a box never collides with the nil word
#[no_mangle]
pub extern "C" fn sloth_rt_print_opt(h: i64, kind: i64) -> i64 {
    if h == 0 {
        println!("nil");
        return 0;
    }
    match kind {
        1 => {
            let f = unsafe { *(h as *const f64) };
            println!("{}", f)
        }
        2 => {
            let b = unsafe { *(h as *const i64) };
            println!("{}", b != 0)
        }
        _ => {
            let i = unsafe { *(h as *const i64) };
            println!("{}", i)
        }
    }
    0
}
