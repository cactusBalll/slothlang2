//! Console I/O: value-plane primitives take raw words. The f64 print keeps
//! the typed route — callers pass the scalar at that edge.

#[no_mangle]
pub extern "C" fn sloth_rt_hello() {
    println!("libsloth_rt linked");
}

/// print an i64 value (display form); never returns useful value
#[no_mangle]
pub extern "C" fn sloth_rt_print_i64(v_w: i64) -> i64 {
    println!("{}", crate::rc::dec_i(v_w));
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_f64(v: f64) -> i64 {
    println!("{}", v);
    0
}

#[no_mangle]
pub extern "C" fn sloth_rt_print_bool(v_w: i64) -> i64 {
    println!("{}", crate::rc::dec_i(v_w) != 0);
    0
}

/// value-optional box printing (kind: 0 = int, 1 = float, 2 = bool — the
/// kind arrives decoded): nil prints "nil" — a box never collides with the
/// nil word
#[no_mangle]
pub extern "C" fn sloth_rt_print_opt(h_w: i64, kind_w: i64) -> i64 {
    if h_w == 0 {
        println!("nil");
        return 0;
    }
    let kind = crate::rc::dec_i(kind_w);
    let payload = unsafe { *(crate::rc::w_unref(h_w) as *const i64) };
    match kind {
        1 => println!("{}", f64::from_bits(crate::rc::dec_f_bits(payload))),
        2 => println!("{}", crate::rc::dec_i(payload) != 0),
        _ => println!("{}", crate::rc::dec_i(payload)),
    }
    0
}
