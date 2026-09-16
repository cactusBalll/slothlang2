//! rt smoke suite (in-process): the deterministic allocator removed libgc,
//! so the td-child workarounds are gone.
use sloth_rt::{arrays, console, maps, objects, rc, strings};

#[test]
fn rt_smoke() {
    unsafe {
        // ---- arrays: churn fires the rc path when counts reach zero ----
        let before = rc::sloth_rc_live();
        let drops0 = rc::sloth_rc_drops();
        for _ in 0..64 {
            let dead = arrays::sloth_arr_new(8192);
            arrays::sloth_arr_set(dead, 0, 1);
            rc::sloth_rc_release(dead);
        }
        assert!(rc::sloth_rc_drops() - drops0 >= 64, "churn dropped");
        assert_eq!(rc::sloth_rc_live(), before, "churn fully drained");

        // reallocation on push growth keeps contents (rt_realloc path)
        let mut a = arrays::sloth_arr_new(8);
        arrays::sloth_arr_set(a, 0, 7);
        for i in 8..64 {
            a = arrays::sloth_arr_push(a, i);
        }
        assert_eq!(arrays::sloth_arr_len(a), 64, "push growth len");
        assert_eq!(arrays::sloth_arr_get(a, 0), 7, "push growth word 0");
        assert_eq!(arrays::sloth_arr_get(a, 63), 63, "push growth last");
        rc::sloth_rc_release(a);

        // ---- strings ----
        let s = strings::sloth_str_intern("ab\0".as_ptr() as i64, 2);
        assert_eq!(strings::sloth_str_len(s), 2, "interned len");
        let c = strings::sloth_str_char(s, 1);
        unsafe {
            let td = c as *const strings::StrT;
            assert_eq!(*((*td).data as *const u8), b'b', "char content");
        }
        rc::sloth_rc_release(c);
        let eq = strings::sloth_str_eq(s, strings::sloth_str_intern("ab\0".as_ptr() as i64, 2));
        assert_eq!(eq, 1, "str content eq");
        rc::sloth_rc_release(s);

        // ---- maps ----
        let m = maps::sloth_map_new(0);
        for i in 0..40 {
            let _ = maps::sloth_map_set(m, i, i * 2);
        }
        assert_eq!(maps::sloth_map_len(m), 40, "map len after growth");
        for i in 0..40 {
            assert_eq!(maps::sloth_map_get(m, i), i * 2, "map key {}", i);
        }
        let ks = maps::sloth_map_keys(m);
        assert_eq!(arrays::sloth_arr_len(ks), 40, "keys array len");
        rc::sloth_rc_release(ks);
        rc::sloth_rc_release(m);

        // overwrite path: same key updates the value slot
        let m2 = maps::sloth_map_new(0);
        let _ = maps::sloth_map_set(m2, 5, 50);
        let _ = maps::sloth_map_set(m2, 5, 99);
        assert_eq!(maps::sloth_map_len(m2), 1);
        assert_eq!(maps::sloth_map_get(m2, 5), 99);
        rc::sloth_rc_release(m2);

        let _ = console::sloth_rt_print_i64(1);

        // ---- rc core + weak boxes ----
        let before = rc::sloth_rc_live();
        let o = objects::sloth_obj_new(0, 4);
        assert_eq!(rc::sloth_rc_live(), before + 1, "obj tracked");
        let w = rc::sloth_weak_new(o);
        assert_eq!(rc::sloth_weak_upgrade(w), o, "weak on live target");
        rc::sloth_rc_retain(o);
        rc::sloth_rc_release(o);
        assert_eq!(rc::sloth_weak_upgrade(w), o, "shared count alive");
        rc::sloth_rc_release(o);
        assert_eq!(rc::sloth_weak_upgrade(w), 0, "dead target weak");
        rc::sloth_weak_release(w);
        assert_eq!(rc::sloth_rc_live(), before, "table fully drained");

        // unknown words are inert no-ops
        rc::sloth_rc_retain(1);
        rc::sloth_rc_release(1);
        assert_eq!(rc::sloth_rc_live(), before, "untracked words inert");

        println!("rt smoke OK");
    }
}
