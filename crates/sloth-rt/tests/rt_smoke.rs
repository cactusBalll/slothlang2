//! rt smoke suite (in-process; tagged-word header rc core).
use sloth_rt::{arrays, maps, mmap, objects, rc, strings, tensors};

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
        assert_eq!(rc::dec_i(strings::sloth_str_len(s)), 2, "str len");
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
        // TH: upgrade returns an owned +1 (CAS retain) — release each result
        let u0 = rc::sloth_weak_upgrade(w);
        assert_eq!(u0, o, "weak on live target");
        rc::sloth_rc_release(u0);
        rc::sloth_rc_retain(o);
        rc::sloth_rc_release(o);
        let u1 = rc::sloth_weak_upgrade(w);
        assert_eq!(u1, o, "shared count alive");
        rc::sloth_rc_release(u1);
        rc::sloth_rc_release(o);
        assert_eq!(rc::sloth_weak_upgrade(w), 0, "dead target weak");
        rc::sloth_weak_release(w);
        assert_eq!(
            rc::dec_i(rc::sloth_rc_live()),
            before,
            "table fully drained"
        );

        // TH: a weak box follows its target across a relocating growth
        let mut ga = arrays::sloth_arr_new(wi(2));
        let gwa = rc::sloth_weak_new(ga);
        for i in 0..64 {
            ga = arrays::sloth_arr_push(ga, wi(i));
        }
        let gu = rc::sloth_weak_upgrade(gwa);
        assert_eq!(gu, ga, "weak resolves after relocation");
        rc::sloth_rc_release(gu);
        rc::sloth_rc_release(gwa);
        rc::sloth_rc_release(ga);
        assert_eq!(
            rc::dec_i(rc::sloth_rc_live()),
            before,
            "relocated weak drained"
        );

        // value words are inert no-ops (tag0 short-circuits)
        rc::sloth_rc_retain(wi(1));
        rc::sloth_rc_release(wi(1));
        rc::sloth_rc_retain(0);
        rc::sloth_rc_release(0);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), before, "value words inert");

        // ---- checkpoint IO (design D7): mmap + i32 header + f32 widening ----
        let mb = rc::dec_i(rc::sloth_rc_live());
        let mut bytes: Vec<u8> = Vec::new();
        for v in [4i32, 8, 1, 1, 1, 32, 16] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let path = "/tmp/opencode/rt_smoke_cfg.bin";
        std::fs::write(path, &bytes).expect("write fixture");
        let pth = inter(path);
        let h = mmap::sloth_mmap(pth);
        assert_eq!(mmap::sloth_mmap_len(h), 44, "file length");
        assert_eq!(mmap::sloth_mmap_i32(h, 0), 4, "dim");
        assert_eq!(mmap::sloth_mmap_i32(h, 5 * 4), 32, "vocab");
        assert_eq!(mmap::sloth_mmap_i32(h, 6 * 4), 16, "seq_len");
        let ft = tensors::sloth_tensor_from_f32_ptr(h, 28, 4);
        assert_eq!(rc::dec_i(tensors::sloth_tensor_dim(ft, wi(0))), 4, "len");
        for (i, want) in [1.0f64, 2.0, 3.0, 4.0].iter().enumerate() {
            assert_eq!(
                rc::dec_f_bits(tensors::sloth_tensor_get1(ft, wi(i as i64))),
                want.to_bits(),
                "weight {}",
                i
            );
        }
        rc::sloth_rc_release(ft);
        rc::sloth_rc_release(pth);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), mb, "mmap tensors drained");

        // tokenizer reads: bytes / unaligned f32 / str
        assert_eq!(mmap::sloth_mmap_u8(h, 0), 4, "u8");
        // must not trap: tokenizer entries are variable-length (unaligned)
        let _ = mmap::sloth_mmap_f32(h, 29);
        let hs = mmap::sloth_mmap_str(h, 0, 4);
        let s0 = String::from_utf8_lossy(&bytes[0..4]).to_string();
        let want_s = inter(&s0);
        assert_eq!(strings::sloth_str_eq(hs, want_s), rc::enc_i(1), "mmap str");
        let hlen = mmap::sloth_mmap_str(h, 0, 0);
        rc::sloth_rc_release(hs);
        rc::sloth_rc_release(hlen);
        rc::sloth_rc_release(want_s);

        // ---- TE-P4 string faces (tokenizer) ----
        let sb = rc::dec_i(rc::sloth_rc_live());
        let a = inter("abc");
        let b = inter("abd");
        let c = inter("abc");
        let bc = inter("bc");
        let aa = inter("A");
        assert!(strings::sloth_str_cmp(a, b) < 0, "cmp less");
        assert_eq!(strings::sloth_str_cmp(a, c), 0, "cmp eq");
        assert!(strings::sloth_str_cmp(b, a) > 0, "cmp greater");
        assert_eq!(strings::sloth_str_byte(a, 1), b'b' as i64, "byte");
        let sl = strings::sloth_str_slice(a, 1, 2);
        assert_eq!(strings::sloth_str_eq(sl, bc), rc::enc_i(1), "slice");
        let ob = strings::sloth_str_of_byte(65);
        assert_eq!(strings::sloth_str_eq(ob, aa), rc::enc_i(1), "of_byte");
        for h2 in [a, b, c, bc, aa, sl, ob] {
            rc::sloth_rc_release(h2);
        }
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), sb, "str faces drained");

        // ---- TE-P4 reshape views (shared storage) ----
        let rb = rc::dec_i(rc::sloth_rc_live());
        let flat = tensors::sloth_tensor_new_1(wi(12), wi(1));
        tensors::sloth_tensor_set1(flat, wi(0), rc::enc_f_bits(11.0f64.to_bits()));
        let m = tensors::sloth_tensor_reshape2(flat, 0, 3, 4);
        assert_eq!(rc::dec_i(tensors::sloth_tensor_dim(m, wi(0))), 3, "r2 d0");
        assert_eq!(rc::dec_i(tensors::sloth_tensor_dim(m, wi(1))), 4, "r2 d1");
        tensors::sloth_tensor_set1(m, wi(0), rc::enc_f_bits(99.0f64.to_bits()));
        assert_eq!(
            rc::dec_f_bits(tensors::sloth_tensor_get1(flat, wi(0))),
            99.0f64.to_bits(),
            "reshape shares storage"
        );
        let c3 = tensors::sloth_tensor_reshape3(flat, 0, 2, 2, 3);
        assert_eq!(rc::dec_i(tensors::sloth_tensor_rank(c3)), 3, "r3 rank");
        let f1 = tensors::sloth_tensor_reshape1(flat, 3, 3);
        assert_eq!(rc::dec_i(tensors::sloth_tensor_dim(f1, wi(0))), 3, "r1 len");
        rc::sloth_rc_release(f1);
        rc::sloth_rc_release(c3);
        rc::sloth_rc_release(m);
        rc::sloth_rc_release(flat);
        assert_eq!(rc::dec_i(rc::sloth_rc_live()), rb, "reshape views drained");

        println!("rt smoke OK");
    }
}
