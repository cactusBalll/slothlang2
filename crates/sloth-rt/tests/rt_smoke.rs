//! rt smoke suite (in-process; tagged-word header rc core).
use sloth_rt::{arrays, maps, objects, rc, strings, tensors};

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

        // ---- tensors (TE-P1): shape/stride, shared-storage views, copy ----
        let tbefore = rc::dec_i(rc::sloth_rc_live());
        let t = tensors::sloth_tensor_new_2(wi(2), wi(3), wi(1)); // float, zeroed
        assert_eq!(rc::dec_i(tensors::sloth_tensor_rank(t)), 2, "rank");
        assert_eq!(rc::dec_i(tensors::sloth_tensor_dim(t, wi(0))), 2, "dim0");
        assert_eq!(rc::dec_i(tensors::sloth_tensor_dim(t, wi(1))), 3, "dim1");
        let row1 = tensors::sloth_tensor_view(t, wi(1), wi(1), wi(0));
        assert_eq!(rc::dec_i(tensors::sloth_tensor_rank(row1)), 1, "view rank");
        // row 1 starts at flat element 3 (stride[0] = 3), not 1
        tensors::sloth_tensor_set1(row1, wi(0), rc::enc_f_bits(5.0f64.to_bits()));
        assert_eq!(
            rc::dec_f_bits(tensors::sloth_tensor_get1(row1, wi(0))),
            5.0f64.to_bits(),
            "view write/read"
        );
        let row0 = tensors::sloth_tensor_view(t, wi(0), wi(1), wi(0));
        assert_eq!(
            rc::dec_f_bits(tensors::sloth_tensor_get1(row0, wi(0))),
            0f64.to_bits(),
            "row0 untouched"
        );
        tensors::sloth_tensor_set1(row1, wi(2), rc::enc_f_bits(7.0f64.to_bits()));
        let row1b = tensors::sloth_tensor_view(t, wi(1), wi(1), wi(0));
        assert_eq!(
            rc::dec_f_bits(tensors::sloth_tensor_get1(row1b, wi(2))),
            7.0f64.to_bits(),
            "row1[2] shares storage"
        );
        // int tensor + keep-rank slice + copy_into
        let it = tensors::sloth_tensor_new_1(wi(4), wi(0));
        tensors::sloth_tensor_set1(it, wi(3), wi(99));
        assert_eq!(
            rc::dec_i(tensors::sloth_tensor_get1(it, wi(3))),
            99,
            "int get"
        );
        let sl = tensors::sloth_tensor_view(it, wi(1), wi(0), wi(2));
        assert_eq!(rc::dec_i(tensors::sloth_tensor_rank(sl)), 1, "slice rank");
        assert_eq!(
            rc::dec_i(tensors::sloth_tensor_dim(sl, wi(0))),
            2,
            "slice len"
        );
        let src = tensors::sloth_tensor_new_1(wi(2), wi(0));
        tensors::sloth_tensor_set1(src, wi(0), wi(11));
        tensors::sloth_tensor_set1(src, wi(1), wi(22));
        tensors::sloth_tensor_copy_into(sl, src);
        assert_eq!(
            rc::dec_i(tensors::sloth_tensor_get1(it, wi(1))),
            11,
            "copy a"
        );
        assert_eq!(
            rc::dec_i(tensors::sloth_tensor_get1(it, wi(2))),
            22,
            "copy b"
        );
        // copy_from_array (int array into f64 tensor? same kind here: float)
        let arr = arrays::sloth_arr_new(wi(3));
        arrays::sloth_arr_set(arr, wi(0), rc::enc_f_bits(1.5f64.to_bits()));
        arrays::sloth_arr_set(arr, wi(1), rc::enc_f_bits(2.5f64.to_bits()));
        arrays::sloth_arr_set(arr, wi(2), rc::enc_f_bits(3.5f64.to_bits()));
        let ft = tensors::sloth_tensor_new_1(wi(3), wi(1));
        tensors::sloth_tensor_copy_from_array(ft, arr);
        assert_eq!(
            rc::dec_f_bits(tensors::sloth_tensor_get1(ft, wi(2))),
            3.5f64.to_bits(),
            "from array"
        );
        // release chain drains fully (views retain their owner)
        for w in [ft, arr, it, sl, src, row0, row1, row1b, t] {
            rc::sloth_rc_release(w);
        }
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), tbefore, "tensors drained");

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
