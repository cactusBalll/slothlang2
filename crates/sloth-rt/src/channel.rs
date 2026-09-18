//! mpmc channels (TH-P2): bounded ring / unbounded queue behind one mutex
//! and two condvars. Elements are tagged words; `send` transfers an owned
//! +1 in, `recv` transfers an owned +1 (or a boxed value optional) out.

use crate::panics;
use crate::rc::{self, w_is_ref, w_unref};
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

struct ChanState {
    buf: VecDeque<i64>,
    /// 0 = unbounded
    cap: usize,
    closed: bool,
}

struct ChannelObj {
    state: Mutex<ChanState>,
    not_empty: Condvar,
    not_full: Condvar,
}

fn chan_dtor(p: usize, _aux: u64) {
    unsafe {
        let c = &mut *(p as *mut ChannelObj);
        // no waiters can exist once the last reference drops; shed any
        // elements still queued (each carries an owned +1)
        let st = c.state.get_mut().unwrap();
        for w in st.buf.drain(..) {
            if w != 0 {
                rc::sloth_rc_release(w);
            }
        }
        std::ptr::drop_in_place(p as *mut ChannelObj);
    }
}

#[no_mangle]
pub extern "C" fn sloth_chan_new(cap_w: i64) -> i64 {
    unsafe {
        let p = rc::rc_addr(std::mem::size_of::<ChannelObj>(), Some(chan_dtor))
            as *mut ChannelObj;
        std::ptr::write(
            p,
            ChannelObj {
                state: Mutex::new(ChanState {
                    buf: VecDeque::new(),
                    cap: rc::dec_i(cap_w).max(0) as usize,
                    closed: false,
                }),
                not_empty: Condvar::new(),
                not_full: Condvar::new(),
            },
        );
        rc::w_ref(p as usize)
    }
}

#[no_mangle]
pub extern "C" fn sloth_chan_send(ch_w: i64, v_w: i64) -> i64 {
    if !w_is_ref(ch_w) {
        return 0;
    }
    unsafe {
        let c = w_unref(ch_w) as *mut ChannelObj;
        let mut st = (*c).state.lock().unwrap();
        while !st.closed && st.cap > 0 && st.buf.len() >= st.cap {
            st = (*c).not_full.wait(st).unwrap();
        }
        if st.closed {
            drop(st);
            panics::panic_msg("send on a closed channel");
        }
        st.buf.push_back(v_w);
        (*c).not_empty.notify_one();
    }
    0
}

#[no_mangle]
pub extern "C" fn sloth_chan_recv(ch_w: i64, box_w: i64) -> i64 {
    if !w_is_ref(ch_w) {
        return 0;
    }
    unsafe {
        let c = w_unref(ch_w) as *mut ChannelObj;
        let mut st = (*c).state.lock().unwrap();
        while st.buf.is_empty() && !st.closed {
            st = (*c).not_empty.wait(st).unwrap();
        }
        let v = match st.buf.pop_front() {
            Some(v) => v,
            None => return 0, // closed + drained -> nil
        };
        (*c).not_full.notify_one();
        drop(st);
        if box_w != 0 {
            crate::boxopt::sloth_box_new(v)
        } else {
            v
        }
    }
}

#[no_mangle]
pub extern "C" fn sloth_chan_close(ch_w: i64) -> i64 {
    if !w_is_ref(ch_w) {
        return 0;
    }
    unsafe {
        let c = w_unref(ch_w) as *mut ChannelObj;
        let mut st = (*c).state.lock().unwrap();
        st.closed = true;
        drop(st);
        (*c).not_empty.notify_all();
        (*c).not_full.notify_all();
    }
    0
}
