//! rt smoke suite (in-process; tagged-word header rc core).
use sloth_rt::{arrays, maps, objects, rc, strings};

/// tagged len/index/field-count helpers (the word plane encodes ints)
const fn wi(v: i64) -> i64 {
    rc::enc_i(v)
}

fn inter(s: &str) -> i64 {
    strings::sloth_str_intern(s.as_ptr() as i64, wi(s.len() as i64))
}

#[test]
fn rt_smoke() {
    unsafe {
        // ---- arrays: churn fires the rc path when counts reach zero ----
        let before = rc::dec_i(rc::sloth_rc_live());
        let drops0 = rc::sloth_rc_drops();
        for _ in 0..64 {
            let dead = arrays::sloth_arr_new(wi(8192));
            arrays::sloth_arr_set(dead, wi(0), wi(1));
            rc::sloth_rc_release(dead);
        }
        assert!(rc::sloth_rc_drops() - drops0 >= 64, "churn dropped");
        assert_eq!(rc::sloth_rc_live(), before, "churn fully drained");

        // reallocation on push growth keeps contents (header relocation)
        let mut a = arrays::sloth_arr_new(wi(8));
        arrays::sloth_arr_set(a, wi(0), wi(7));
        for i in 8..64 {
            a = arrays::sloth_arr_push(a, wi(i));
        }
        assert_eq!(rc::dec_i(arrays::sloth_arr_len(a)), 64, "push growth len");
        assert_eq!(rc::dec_i(arrays::sloth_arr_get(a, wi(0))), 7, "word 0");
        assert_eq!(rc::dec_i(arrays::sloth_arr_get(a, wi(63))), 63, "last");
        rc::sloth_rc_release(a);

        // ---- tagged int roundtrip: 63-bit wrap + sign decode ----
        assert_eq!(rc::dec_i(rc::enc_i(-1)), -1, "neg int roundtrip");
        assert_eq!(rc::dec_i(rc::enc_i(i64::MIN)), 0, "min wraps to 0");
        assert_eq!(rc::enc_i(3) & 1, 0, "int value tag clear");

        // ---- tagged f64 roundtrip (1 mantissa LSB sacrificed) ----
        for bits in [1.5f64.to_bits(), (-0.5f64).to_bits(), 0f64.to_bits()] {
            let back = rc::dec_f_bits(rc::enc_f_bits(bits));
            assert_eq!(rc::enc_f_bits(back), rc::enc_f_bits(bits), "f64 roundtrip");
        }

        // ---- strings ----
        let s = inter("ab");
        assert_eq!(rc::dec_i(strings::sloth_str_len(s)), 2, "interned len");
        let c = strings::sloth_str_char(s, wi(1));
        let td = rc::w_unref(c) as *const strings::StrT;
        assert_eq!(*((*td).data as *const u8), b'b', "char content");
        rc::sloth_rc_release(c);
        let eq = strings::sloth_str_eq(s, inter("ab"));
        assert_eq!(rc::dec_i(eq), 1, "str content eq");
        rc::sloth_rc_release(s);

        // ---- maps ----
        let m = maps::sloth_map_new(wi(0));
        for i in 0..40 {
            let _ = maps::sloth_map_set(m, wi(i), wi(i * 2));
        }
        assert_eq!(rc::dec_i(maps::sloth_map_len(m)), 40, "map len growth");
        for i in 0..40 {
            assert_eq!(
                rc::dec_i(maps::sloth_map_get(m, wi(i))),
                i * 2,
                "map key {}",
                i
            );
        }
        let ks = maps::sloth_map_keys(m);
        assert_eq!(rc::dec_i(arrays::sloth_arr_len(ks)), 40, "keys arr len");
        rc::sloth_rc_release(ks);
        rc::sloth_rc_release(m);

        // overwrite path: same key updates the value slot
        let m2 = maps::sloth_map_new(wi(0));
        let _ = maps::sloth_map_set(m2, wi(5), wi(50));
        let _ = maps::sloth_map_set(m2, wi(5), wi(99));
        assert_eq!(rc::dec_i(maps::sloth_map_len(m2)), 1);
        assert_eq!(rc::dec_i(maps::sloth_map_get(m2, wi(5))), 99);
        rc::sloth_rc_release(m2);

        // ---- rc core + weak boxes ----
        let before = rc::dec_i(rc::sloth_rc_live());
        let o = objects::sloth_obj_new(0, wi(4));
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), before + 1, "obj tracked");
        let w = rc::sloth_weak_new(o);
        assert_eq!(rc::sloth_weak_upgrade(w), o, "weak on live target");
        rc::sloth_rc_retain(o);
        rc::sloth_rc_release(o);
        assert_eq!(rc::sloth_weak_upgrade(w), o, "shared count alive");
        rc::sloth_rc_release(o);
        assert_eq!(rc::sloth_weak_upgrade(w), 0, "dead target weak");
        rc::sloth_weak_release(w);
        assert_eq!(
            rc::dec_i(rc::sloth_rc_live()),
            before,
            "table fully drained"
        );

        // value words are inert no-ops (tag0 short-circuits)
        rc::sloth_rc_retain(wi(1));
        rc::sloth_rc_release(wi(1));
        rc::sloth_rc_retain(0);
        rc::sloth_rc_release(0);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), before, "value words inert");

        println!("rt smoke OK");
    }
}
