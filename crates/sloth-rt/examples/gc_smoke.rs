//! sloth-rt runtime suite (child process): all allocations go through the
//! GC heap and libgc expects init on the main thread, so this runs as its
//! own process (driven by tests/gc_smoke_child.rs) instead of libtest.
fn main() {
    unsafe {
        // ---- arrays + GC ----
        // one reference held across a forced collection
        let keep = sloth_rt::arrays::sloth_arr_new(64);
        sloth_rt::arrays::sloth_arr_set(keep, 0, 4242);

        sloth_rt::gc::sloth_gc_collect();
        let after = sloth_rt::gc::sloth_gc_collections();
        assert!(after >= 1, "a full collection ran");

        // churn: allocate many large chunks dropped immediately
        for _ in 0..64 {
            let dead = sloth_rt::arrays::sloth_arr_new(8192);
            sloth_rt::arrays::sloth_arr_set(dead, 0, 1);
        }
        sloth_rt::gc::sloth_gc_collect();
        let churned = sloth_rt::gc::sloth_gc_collections();
        assert!(
            churned > after,
            "churn triggered collections: {after} -> {churned}"
        );

        // live data survived
        assert_eq!(sloth_rt::arrays::sloth_arr_get(keep, 0), 4242);
        assert_eq!(sloth_rt::arrays::sloth_arr_len(keep), 64);

        // push growth past original cap keeps contents (gc_realloc path)
        for i in 64..128 {
            sloth_rt::arrays::sloth_arr_push(keep, i);
        }
        assert_eq!(sloth_rt::arrays::sloth_arr_len(keep), 128);
        assert_eq!(sloth_rt::arrays::sloth_arr_get(keep, 0), 4242);
        sloth_rt::gc::sloth_gc_collect();
        assert_eq!(sloth_rt::arrays::sloth_arr_get(keep, 127), 127);

        // ---- strings ----
        // str iteration returns single-byte strings
        let s = sloth_rt::strings::sloth_str_intern("ab\0".as_ptr() as i64, 2);
        assert_eq!(sloth_rt::strings::sloth_str_len(s), 2, "interned len");
        let wants = [b'a', b'b'];
        for i in 0..2 {
            let c = sloth_rt::strings::sloth_str_char(s, i);
            assert_eq!(sloth_rt::strings::sloth_str_len(c), 1, "char len @{}", i);
            let td = c as *mut sloth_rt::strings::StrT;
            let b = *((*td).data as *const u8);
            assert_eq!(b, wants[i as usize], "char @{}", i);
        }
        let s2 = sloth_rt::strings::sloth_str_intern("hi\0".as_ptr() as i64, 2);
        assert_eq!(sloth_rt::strings::sloth_rt_print_str(s2), 0);
        let _ = sloth_rt::console::sloth_rt_print_i64(1);

        // ---- maps ----
        // 20-key roundtrip: set/get + keys/values arrays
        let m = sloth_rt::maps::sloth_map_new(0);
        for i in 0..20 {
            sloth_rt::maps::sloth_map_set(m, i, i * 2);
        }
        assert_eq!(sloth_rt::maps::sloth_map_len(m), 20, "len after 20 sets");
        for i in 0..20 {
            assert_eq!(sloth_rt::maps::sloth_map_get(m, i), i * 2, "key {}", i);
        }
        let ks = sloth_rt::maps::sloth_map_keys(m);
        assert_eq!(sloth_rt::arrays::sloth_arr_len(ks), 20, "keys array len");
        let vs = sloth_rt::maps::sloth_map_values(m);
        assert_eq!(sloth_rt::arrays::sloth_arr_len(vs), 20, "values array len");

        // two-key map: no phantom used slots, uniform probe
        let m2 = sloth_rt::maps::sloth_map_new(0);
        sloth_rt::maps::sloth_map_set(m2, 5, 50);
        sloth_rt::maps::sloth_map_set(m2, 6, 60);
        assert_eq!(sloth_rt::maps::sloth_map_len(m2), 2);
        assert_eq!(sloth_rt::maps::sloth_map_get(m2, 5), 50);
        assert_eq!(sloth_rt::maps::sloth_map_get(m2, 6), 60);
        let ks2 = sloth_rt::maps::sloth_map_keys(m2);
        assert_eq!(sloth_rt::arrays::sloth_arr_len(ks2), 2, "keys array");
        let ks_len = sloth_rt::arrays::sloth_arr_len(ks2);
        let mut i = 0i64;
        let mut s50 = 0i64;
        let mut s60 = 0i64;
        while i < ks_len {
            let k = sloth_rt::arrays::sloth_arr_get(ks2, i);
            if k == 5 {
                s50 = 1;
            }
            if k == 6 {
                s60 = 1;
            }
            assert_eq!(sloth_rt::maps::sloth_map_get(m2, k), k * 10);
            i += 1;
        }
        assert_eq!(s50, 1, "key 5 present");
        assert_eq!(s60, 1, "key 6 present");

        // patch #35: content-hash keyed object slots; equal hash ⇒ equal slot
        let m3 = sloth_rt::maps::sloth_map_new(2);
        let _ = sloth_rt::maps::sloth_map_set_h(m3, 1001, 31, 10);
        assert_eq!(sloth_rt::maps::sloth_map_len(m3), 1);
        assert_eq!(sloth_rt::maps::sloth_map_get_h(m3, 555, 31), 10);
        let _ = sloth_rt::maps::sloth_map_set_h(m3, 999, 31, 42);
        assert_eq!(sloth_rt::maps::sloth_map_len(m3), 1);
        assert_eq!(sloth_rt::maps::sloth_map_get_h(m3, 999, 31), 42);

        // map growth + rehash keeps entries (grown buckets are fresh chunks)
        let m4 = sloth_rt::maps::sloth_map_new(0);
        for i in 0..64 {
            sloth_rt::maps::sloth_map_set(m4, i, i * 3);
        }
        sloth_rt::gc::sloth_gc_collect();
        assert_eq!(sloth_rt::maps::sloth_map_len(m4), 64);
        for i in 0..64 {
            assert_eq!(sloth_rt::maps::sloth_map_get(m4, i), i * 3, "key {}", i);
        }

        // ---- reference counts + weak boxes (patch A) ----
        let before = sloth_rt::rc::sloth_rc_live();
        let o = sloth_rt::objects::sloth_obj_new(0, 4);
        assert_eq!(sloth_rt::rc::sloth_rc_live(), before + 1, "obj tracked");
        let w = sloth_rt::rc::sloth_weak_new(o);
        assert_eq!(
            sloth_rt::rc::sloth_weak_upgrade(w),
            o,
            "weak upgrade on live target"
        );
        sloth_rt::rc::sloth_rc_retain(o);
        sloth_rt::rc::sloth_rc_release(o);
        assert_eq!(
            sloth_rt::rc::sloth_weak_upgrade(w),
            o,
            "shared count keeps target alive"
        );
        sloth_rt::rc::sloth_rc_release(o);
        assert_eq!(sloth_rt::rc::sloth_weak_upgrade(w), 0, "dead target");
        sloth_rt::rc::sloth_weak_release(w);
        assert_eq!(sloth_rt::rc::sloth_rc_live(), before, "table fully drained");

        // unknown words are inert no-ops
        sloth_rt::rc::sloth_rc_retain(1);
        sloth_rt::rc::sloth_rc_release(1);
        assert_eq!(
            sloth_rt::rc::sloth_rc_live(),
            before,
            "untracked words inert"
        );

        println!("rt smoke OK");
    }
}
