//! Multithreading smoke suite (TH): atomic ARC churn, mpmc channel semantics,
//! mutex/atomic contention — all against the runtime entry points directly.

use sloth_rt::{channel, objects, rc, sync};

const fn wi(v: i64) -> i64 {
    rc::enc_i(v)
}

#[test]
fn arc_stress_mt() {
    let base = rc::dec_i(rc::__sloth_rc_live());
    let o = objects::__sloth_obj_new(0, wi(1), 0);
    let mut hs = Vec::new();
    for _ in 0..8 {
        let w = rc::__sloth_rc_retain(o);
        hs.push(std::thread::spawn(move || {
            for _ in 0..200_000 {
                rc::__sloth_rc_retain(w);
                rc::__sloth_rc_release(w);
            }
            rc::__sloth_rc_release(w);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    rc::__sloth_rc_release(o);
    assert_eq!(rc::dec_i(rc::__sloth_rc_live()), base, "arc churn drained");
}

#[test]
fn channel_mpmc() {
    const P: usize = 4;
    const C: usize = 4;
    const N: i64 = 25_000;
    let ch = channel::__sloth_chan_new(wi(64), 0);
    let mut hs = Vec::new();
    for _ in 0..P {
        let c = rc::__sloth_rc_retain(ch);
        hs.push(std::thread::spawn(move || {
            for i in 0..N {
                channel::__sloth_chan_send(c, wi(i));
            }
            rc::__sloth_rc_release(c);
        }));
    }
    for _ in 0..C {
        let c = rc::__sloth_rc_retain(ch);
        hs.push(std::thread::spawn(move || {
            let mut got = 0i64;
            while got < N {
                let w = channel::__sloth_chan_recv(c, 0);
                // sentinel: producers never send negative values
                assert!(rc::dec_i(w) >= 0, "unexpected recv word");
                got += 1;
            }
            rc::__sloth_rc_release(c);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    rc::__sloth_rc_release(ch);
}

#[test]
fn channel_close_drain() {
    let ch = channel::__sloth_chan_new(wi(0), 0);
    channel::__sloth_chan_send(ch, wi(7));
    channel::__sloth_chan_send(ch, wi(8));
    channel::__sloth_chan_close(ch);
    assert_eq!(rc::dec_i(channel::__sloth_chan_recv(ch, 0)), 7);
    assert_eq!(rc::dec_i(channel::__sloth_chan_recv(ch, 0)), 8);
    assert_eq!(channel::__sloth_chan_recv(ch, 0), 0, "closed+empty -> nil");
    rc::__sloth_rc_release(ch);
}

#[test]
fn atomic_contention() {
    let a = sync::__sloth_atomic_new(wi(0));
    let mut hs = Vec::new();
    for _ in 0..8 {
        let aw = rc::__sloth_rc_retain(a);
        hs.push(std::thread::spawn(move || {
            for _ in 0..100_000 {
                sync::__sloth_atomic_add(aw, wi(1));
            }
            rc::__sloth_rc_release(aw);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(
        rc::dec_i(sync::__sloth_atomic_load(a)),
        800_000,
        "atomic sum"
    );
    rc::__sloth_rc_release(a);
}

#[test]
fn mutex_guards_critical_section() {
    let m = sync::__sloth_mutex_new();
    let a = sync::__sloth_atomic_new(wi(0));
    let mut hs = Vec::new();
    for _ in 0..8 {
        let mw = rc::__sloth_rc_retain(m);
        let aw = rc::__sloth_rc_retain(a);
        hs.push(std::thread::spawn(move || {
            for _ in 0..100_000 {
                sync::__sloth_mutex_lock(mw);
                let v = rc::dec_i(sync::__sloth_atomic_load(aw));
                sync::__sloth_atomic_store(aw, wi(v + 1));
                sync::__sloth_mutex_unlock(mw);
            }
            rc::__sloth_rc_release(mw);
            rc::__sloth_rc_release(aw);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(
        rc::dec_i(sync::__sloth_atomic_load(a)),
        800_000,
        "mutex sum"
    );
    rc::__sloth_rc_release(m);
    rc::__sloth_rc_release(a);
}
