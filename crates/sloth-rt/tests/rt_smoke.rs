//! rt smoke suite (in-process; tagged-word header rc core).
//!
//! Container (Array/Map) algorithms are self-hosted in
//! `lib/prelude/containers.slt` and covered by the language-level suites
//! (`sloth-codegen` / `slothc` spec). This file covers the bare runtime
//! substrate they build on: allocation, word memory and the rc core.
use sloth_rt::{alloc, builtins, mem, mmap, objects, rc, strings, tensors};
use std::sync::Mutex;

/// Live rc accounting (`__sloth_rc_live`) is process-global, so tests in this
/// binary must not run concurrently or their drain assertions race.
static SERIAL: Mutex<()> = Mutex::new(());

/// Serialize a test; a failing test may have poisoned the lock, so recover the
/// guard instead of cascading the failure into every later test.
fn serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

/// tagged len/index/field-count helpers (the word plane encodes ints)
const fn wi(v: i64) -> i64 {
    rc::enc_i(v)
}

/// Build a `str` handle from a Rust string.
///
/// `__sloth_str_intern` reads its first argument as a raw source pointer, so
/// Rust string literals (only byte-aligned) are copied into an aligned heap
/// buffer (capacity >= 1 keeps the pointer non-dangling even for the empty
/// string) and handed over raw. The rt copies synchronously, so dropping the
/// buffer afterwards is safe.
fn inter(s: &str) -> i64 {
    let mut buf: Vec<u8> = Vec::with_capacity(s.len().max(1));
    buf.extend_from_slice(s.as_bytes());
    let ptr = buf.as_ptr() as i64;
    strings::__sloth_str_intern(ptr, wi(s.len() as i64))
}

/// Unique temp file removed on drop, so the suite neither depends on nor
/// leaves behind a fixed, possibly-missing path.
struct TempFile(std::path::PathBuf);

impl TempFile {
    fn write(tag: &str, bytes: &[u8]) -> TempFile {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let p = std::env::temp_dir().join(format!(
            "sloth_rt_{}_{}_{}.bin",
            tag,
            std::process::id(),
            nanos
        ));
        std::fs::write(&p, bytes).expect("write fixture");
        TempFile(p)
    }

    fn as_str(&self) -> std::borrow::Cow<'_, str> {
        self.0.to_string_lossy()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn rt_smoke() {
    let _serial = serial();
    unsafe {
        // ---- bare memory substrate: rc_new chunks + untracked buffers ----
        let before = rc::dec_i(rc::__sloth_rc_live());
        let drops0 = rc::__sloth_rc_drops();
        for _ in 0..64 {
            // 3-word tracked payload, no __dispose__ hook
            let h = rc::__sloth_rc_new(24, 0, 0);
            mem::__sloth_mem_store(h, 0, wi(1));
            assert_eq!(rc::dec_i(mem::__sloth_mem_load(h, 0)), 1, "word store/load");
            rc::__sloth_rc_release(h);
        }
        assert!(rc::__sloth_rc_drops() - drops0 >= 64, "churn dropped");
        assert_eq!(rc::__sloth_rc_live(), before, "churn fully drained");

        // untracked alloc + word memory + copy + free round-trip
        let buf = alloc::__sloth_rt_alloc(64) as i64;
        mem::__sloth_mem_store(buf, 0, wi(7));
        mem::__sloth_mem_store(buf, 7, wi(9));
        assert_eq!(rc::dec_i(mem::__sloth_mem_load(buf, 0)), 7, "buf[0]");
        assert_eq!(rc::dec_i(mem::__sloth_mem_load(buf, 7)), 9, "buf[7]");
        let dst = alloc::__sloth_rt_alloc(64) as i64;
        mem::__sloth_mem_copy(dst, buf, 8);
        assert_eq!(rc::dec_i(mem::__sloth_mem_load(dst, 7)), 9, "copied word");
        mem::__sloth_free(buf);
        mem::__sloth_free(dst);

        // ---- int roundtrip: full 64-bit width, identity codec ----
        assert_eq!(rc::dec_i(rc::enc_i(-1)), -1, "neg int roundtrip");
        assert_eq!(rc::dec_i(rc::enc_i(i64::MIN)), i64::MIN, "full-width int");
        assert_eq!(rc::enc_i(3), 3, "int identity");

        // ---- f64 roundtrip (identity codec: full precision) ----
        for bits in [1.5f64.to_bits(), (-0.5f64).to_bits(), 0f64.to_bits()] {
            let back = rc::dec_f_bits(rc::enc_f_bits(bits));
            assert_eq!(rc::enc_f_bits(back), rc::enc_f_bits(bits), "f64 roundtrip");
        }

        // ---- strings ----
        let s = inter("ab");
        assert_eq!(rc::dec_i(strings::__sloth_str_len(s)), 2, "str len");
        let c = strings::__sloth_str_char(s, wi(1));
        let td = rc::w_unref(c) as *const strings::StrT;
        assert_eq!(*((*td).data as *const u8), b'b', "char content");
        rc::__sloth_rc_release(c);
        let eq = strings::__sloth_str_eq(s, inter("ab"));
        assert_eq!(rc::dec_i(eq), 1, "str content eq");
        rc::__sloth_rc_release(s);

        // ---- str byte index / slice / code point (UTF-8) ----
        // "aé中": bytes a(61) é(c3 a9) 中(e4 b8 ad); chars a / é / 中
        let u = inter("aé中");
        assert_eq!(rc::dec_i(strings::__sloth_str_len(u)), 6, "utf8 byte len");
        assert_eq!(rc::dec_i(strings::__sloth_str_clen(u)), 3, "utf8 char len");
        assert_eq!(strings::__sloth_str_byte(u, 0), b'a' as i64, "byte a");
        assert_eq!(strings::__sloth_str_byte(u, 1), 0xc3, "byte e-acute lead");
        assert_eq!(strings::__sloth_str_byte(u, 2), 0xa9, "byte e-acute cont");
        assert_eq!(strings::__sloth_str_codepoint(u, 0), 0x61, "cp a");
        assert_eq!(strings::__sloth_str_codepoint(u, 1), 0xe9, "cp e-acute");
        assert_eq!(strings::__sloth_str_codepoint(u, 2), 0x4e2d, "cp zhong");
        let sl2 = strings::__sloth_str_slice(u, 1, 2);
        assert_eq!(
            strings::__sloth_str_eq(sl2, inter("é")),
            rc::enc_i(1),
            "byte slice keeps one char"
        );
        rc::__sloth_rc_release(sl2);
        rc::__sloth_rc_release(u);

        // ---- tensors (TE-P1): shape/stride, shared-storage views, copy ----
        let tbefore = rc::dec_i(rc::__sloth_rc_live());
        let t = tensors::__sloth_tensor_new_2(wi(2), wi(3), wi(1)); // float, zeroed
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_rank(t)), 2, "rank");
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_dim(t, wi(0))), 2, "dim0");
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_dim(t, wi(1))), 3, "dim1");
        let row1 = tensors::__sloth_tensor_view(t, wi(1), wi(1), wi(0));
        assert_eq!(
            rc::dec_i(tensors::__sloth_tensor_rank(row1)),
            1,
            "view rank"
        );
        // row 1 starts at flat element 3 (stride[0] = 3), not 1
        tensors::__sloth_tensor_set1(row1, wi(0), rc::enc_f_bits(5.0f64.to_bits()));
        assert_eq!(
            rc::dec_f_bits(tensors::__sloth_tensor_get1(row1, wi(0))),
            5.0f64.to_bits(),
            "view write/read"
        );
        let row0 = tensors::__sloth_tensor_view(t, wi(0), wi(1), wi(0));
        assert_eq!(
            rc::dec_f_bits(tensors::__sloth_tensor_get1(row0, wi(0))),
            0f64.to_bits(),
            "row0 untouched"
        );
        tensors::__sloth_tensor_set1(row1, wi(2), rc::enc_f_bits(7.0f64.to_bits()));
        let row1b = tensors::__sloth_tensor_view(t, wi(1), wi(1), wi(0));
        assert_eq!(
            rc::dec_f_bits(tensors::__sloth_tensor_get1(row1b, wi(2))),
            7.0f64.to_bits(),
            "row1[2] shares storage"
        );
        // int tensor + keep-rank slice + copy_into
        let it = tensors::__sloth_tensor_new_1(wi(4), wi(0));
        tensors::__sloth_tensor_set1(it, wi(3), wi(99));
        assert_eq!(
            rc::dec_i(tensors::__sloth_tensor_get1(it, wi(3))),
            99,
            "int get"
        );
        let sl = tensors::__sloth_tensor_view(it, wi(1), wi(0), wi(2));
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_rank(sl)), 1, "slice rank");
        assert_eq!(
            rc::dec_i(tensors::__sloth_tensor_dim(sl, wi(0))),
            2,
            "slice len"
        );
        let src = tensors::__sloth_tensor_new_1(wi(2), wi(0));
        tensors::__sloth_tensor_set1(src, wi(0), wi(11));
        tensors::__sloth_tensor_set1(src, wi(1), wi(22));
        tensors::__sloth_tensor_copy_into(sl, src);
        assert_eq!(
            rc::dec_i(tensors::__sloth_tensor_get1(it, wi(1))),
            11,
            "copy a"
        );
        assert_eq!(
            rc::dec_i(tensors::__sloth_tensor_get1(it, wi(2))),
            22,
            "copy b"
        );
        // rank-1 float tensor filled element-wise
        let ft = tensors::__sloth_tensor_new_1(wi(3), wi(1));
        tensors::__sloth_tensor_set1(ft, wi(0), rc::enc_f_bits(1.5f64.to_bits()));
        tensors::__sloth_tensor_set1(ft, wi(1), rc::enc_f_bits(2.5f64.to_bits()));
        tensors::__sloth_tensor_set1(ft, wi(2), rc::enc_f_bits(3.5f64.to_bits()));
        assert_eq!(
            rc::dec_f_bits(tensors::__sloth_tensor_get1(ft, wi(2))),
            3.5f64.to_bits(),
            "from array"
        );
        // release chain drains fully (views retain their owner)
        for w in [ft, it, sl, src, row0, row1, row1b, t] {
            rc::__sloth_rc_release(w);
        }
        assert_eq!(rc::dec_i(rc::__sloth_rc_live()), tbefore, "tensors drained");

        // ---- rc core + weak boxes ----
        let before = rc::dec_i(rc::__sloth_rc_live());
        let o = objects::__sloth_obj_new(0, wi(4), 0);
        assert_eq!(rc::dec_i(rc::__sloth_rc_live()), before + 1, "obj tracked");
        let w = rc::__sloth_weak_new(o);
        // TH: upgrade returns an owned +1 (CAS retain) — release each result
        let u0 = rc::__sloth_weak_upgrade(w);
        assert_eq!(u0, o, "weak on live target");
        rc::__sloth_rc_release(u0);
        rc::__sloth_rc_retain(o);
        rc::__sloth_rc_release(o);
        let u1 = rc::__sloth_weak_upgrade(w);
        assert_eq!(u1, o, "shared count alive");
        rc::__sloth_rc_release(u1);
        rc::__sloth_rc_release(o);
        assert_eq!(rc::__sloth_weak_upgrade(w), 0, "dead target weak");
        rc::__sloth_weak_release(w);
        assert_eq!(
            rc::dec_i(rc::__sloth_rc_live()),
            before,
            "table fully drained"
        );

        // TH: a weak box follows its target across a relocating realloc
        let ga = rc::__sloth_rc_new(24, 0, 0);
        let gwa = rc::__sloth_weak_new(ga);
        let ga2 = alloc::__sloth_rt_realloc(ga, 48);
        let gu = rc::__sloth_weak_upgrade(gwa);
        assert_eq!(gu, ga2, "weak resolves after relocation");
        rc::__sloth_rc_release(gu);
        rc::__sloth_rc_release(gwa);
        rc::__sloth_rc_release(ga2);
        assert_eq!(
            rc::dec_i(rc::__sloth_rc_live()),
            before,
            "relocated weak drained"
        );

        // nil words are inert no-ops; value words are never passed to rc
        rc::__sloth_rc_retain(0);
        rc::__sloth_rc_release(0);
        assert_eq!(rc::dec_i(rc::__sloth_rc_live()), before, "nil words inert");

        // ---- checkpoint IO (design D7): mmap + i32 header + f32 widening ----
        let mb = rc::dec_i(rc::__sloth_rc_live());
        let mut bytes: Vec<u8> = Vec::new();
        for v in [4i32, 8, 1, 1, 1, 32, 16] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let fixture = TempFile::write("mmap_cfg", &bytes);
        let pth = inter(&fixture.as_str());
        let h = mmap::__sloth_mmap(pth);
        assert_eq!(mmap::__sloth_mmap_len(h), 44, "file length");
        assert_eq!(mmap::__sloth_mmap_i32(h, 0), 4, "dim");
        assert_eq!(mmap::__sloth_mmap_i32(h, 5 * 4), 32, "vocab");
        assert_eq!(mmap::__sloth_mmap_i32(h, 6 * 4), 16, "seq_len");
        let ft = tensors::__sloth_tensor_from_f32_ptr(h, 28, 4);
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_dim(ft, wi(0))), 4, "len");
        for (i, want) in [1.0f64, 2.0, 3.0, 4.0].iter().enumerate() {
            assert_eq!(
                rc::dec_f_bits(tensors::__sloth_tensor_get1(ft, wi(i as i64))),
                want.to_bits(),
                "weight {}",
                i
            );
        }
        rc::__sloth_rc_release(ft);
        rc::__sloth_rc_release(pth);
        assert_eq!(rc::dec_i(rc::__sloth_rc_live()), mb, "mmap tensors drained");

        // tokenizer reads: bytes / unaligned f32 / str
        assert_eq!(mmap::__sloth_mmap_u8(h, 0), 4, "u8");
        // must not trap: tokenizer entries are variable-length (unaligned)
        let _ = mmap::__sloth_mmap_f32(h, 29);
        let hs = mmap::__sloth_mmap_str(h, 0, 4);
        let s0 = String::from_utf8_lossy(&bytes[0..4]).to_string();
        let want_s = inter(&s0);
        assert_eq!(
            strings::__sloth_str_eq(hs, want_s),
            rc::enc_i(1),
            "mmap str"
        );
        let hlen = mmap::__sloth_mmap_str(h, 0, 0);
        rc::__sloth_rc_release(hs);
        rc::__sloth_rc_release(hlen);
        rc::__sloth_rc_release(want_s);

        // ---- TE-P4 string faces (tokenizer) ----
        let sb = rc::dec_i(rc::__sloth_rc_live());
        let a = inter("abc");
        let b = inter("abd");
        let c = inter("abc");
        let bc = inter("bc");
        let aa = inter("A");
        assert!(strings::__sloth_str_cmp(a, b) < 0, "cmp less");
        assert_eq!(strings::__sloth_str_cmp(a, c), 0, "cmp eq");
        assert!(strings::__sloth_str_cmp(b, a) > 0, "cmp greater");
        assert_eq!(strings::__sloth_str_byte(a, 1), b'b' as i64, "byte");
        let sl = strings::__sloth_str_slice(a, 1, 2);
        assert_eq!(strings::__sloth_str_eq(sl, bc), rc::enc_i(1), "slice");
        let ob = strings::__sloth_str_of_byte(65);
        assert_eq!(strings::__sloth_str_eq(ob, aa), rc::enc_i(1), "of_byte");
        for h2 in [a, b, c, bc, aa, sl, ob] {
            rc::__sloth_rc_release(h2);
        }
        assert_eq!(rc::dec_i(rc::__sloth_rc_live()), sb, "str faces drained");

        // ---- TE-P4 reshape views (shared storage) ----
        let rb = rc::dec_i(rc::__sloth_rc_live());
        let flat = tensors::__sloth_tensor_new_1(wi(12), wi(1));
        tensors::__sloth_tensor_set1(flat, wi(0), rc::enc_f_bits(11.0f64.to_bits()));
        let m = tensors::__sloth_tensor_reshape2(flat, 0, 3, 4);
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_dim(m, wi(0))), 3, "r2 d0");
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_dim(m, wi(1))), 4, "r2 d1");
        tensors::__sloth_tensor_set1(m, wi(0), rc::enc_f_bits(99.0f64.to_bits()));
        assert_eq!(
            rc::dec_f_bits(tensors::__sloth_tensor_get1(flat, wi(0))),
            99.0f64.to_bits(),
            "reshape shares storage"
        );
        let c3 = tensors::__sloth_tensor_reshape3(flat, 0, 2, 2, 3);
        assert_eq!(rc::dec_i(tensors::__sloth_tensor_rank(c3)), 3, "r3 rank");
        let f1 = tensors::__sloth_tensor_reshape1(flat, 3, 3);
        assert_eq!(
            rc::dec_i(tensors::__sloth_tensor_dim(f1, wi(0))),
            3,
            "r1 len"
        );
        rc::__sloth_rc_release(f1);
        rc::__sloth_rc_release(c3);
        rc::__sloth_rc_release(m);
        rc::__sloth_rc_release(flat);
        assert_eq!(
            rc::dec_i(rc::__sloth_rc_live()),
            rb,
            "reshape views drained"
        );

        println!("rt smoke OK");
    }
}

/// `__sloth_str_intern` untags its first argument, so the source must be even.
/// `inter` copies odd-addressed Rust literals into an aligned buffer; both
/// parities must round-trip length and content (the regression that broke the
/// old hardcoded-path fixture).
#[test]
fn string_intern_alignment() {
    let _serial = serial();
    let before = rc::dec_i(rc::__sloth_rc_live());
    for lit in ["", "a", "hello world", "odd", "even", "path/x.bin"] {
        let h = inter(lit);
        assert_eq!(
            rc::dec_i(strings::__sloth_str_len(h)),
            lit.len() as i64,
            "len {lit:?}"
        );
        let h2 = inter(lit);
        assert_eq!(
            rc::dec_i(strings::__sloth_str_eq(h, h2)),
            1,
            "content {lit:?}"
        );
        rc::__sloth_rc_release(h);
        rc::__sloth_rc_release(h2);
    }
    assert_eq!(rc::dec_i(rc::__sloth_rc_live()), before, "intern drained");
}

/// test-only death cascade: release field 1 (payload word 3 = field offset 2
/// + index 1), exercising the `sdtor` slot that codegen fills in real output.
unsafe extern "C" fn rel_field1(p: i64, _aux: i64) -> i64 {
    let w = mem::__sloth_mem_load(p, wi(3));
    if w != 0 {
        rc::__sloth_rc_release(w);
    }
    0
}

/// Class metadata + instance fields: class-id round-trip, int/nil field
/// access, and a generated-code death cascade (registered as the header
/// `sdtor`) releasing an owned reference field.
#[test]
fn object_fields_and_cascade() {
    let _serial = serial();
    let before = rc::dec_i(rc::__sloth_rc_live());
    let ci = objects::__sloth_cls_info(0, wi(7));
    // field 1 is a reference; the cascade releases it
    let cascade: unsafe extern "C" fn(i64, i64) -> i64 = rel_field1;
    let o = objects::__sloth_obj_new(ci, wi(3), cascade as usize as i64);
    assert_eq!(rc::dec_i(objects::__sloth_obj_cls_id(o)), 7, "cls id");
    assert_eq!(rc::dec_i(rc::__sloth_rc_live()), before + 1, "one instance");

    // int field round-trip; an untouched slot stays nil
    objects::__sloth_obj_set_field(o, wi(0), wi(1234));
    assert_eq!(
        rc::dec_i(objects::__sloth_obj_field(o, wi(0))),
        1234,
        "int field"
    );
    assert_eq!(objects::__sloth_obj_field(o, wi(2)), 0, "nil field");

    // reference field: the slot takes the interned +1 (codegen retains
    // before storing); the borrow below proves the slot keeps it alive
    let s = inter("owned-field");
    objects::__sloth_obj_set_field(o, wi(1), s);
    let got = objects::__sloth_obj_field(o, wi(1));
    let want = inter("owned-field");
    assert_eq!(
        rc::dec_i(strings::__sloth_str_eq(got, want)),
        1,
        "ref field"
    );
    rc::__sloth_rc_release(want);

    // death cascade releases the ref field; count returns to baseline
    rc::__sloth_rc_release(o);
    assert_eq!(
        rc::dec_i(rc::__sloth_rc_live()),
        before,
        "obj cascade drained"
    );
}

/// Builtin value-type auto-boxes backing `dyn Trait`: payload round-trip,
/// Display/hash/comparison helpers, and the ordinary object death cascade.
#[test]
fn builtin_dyn_boxes() {
    let _serial = serial();
    let before = rc::dec_i(rc::__sloth_rc_live());
    // int box holding tagged 41
    let info = builtins::__sloth_builtin_info(wi(0));
    let o = objects::__sloth_obj_new(info, wi(1), 0);
    objects::__sloth_obj_set_field(o, wi(0), wi(41));
    assert_eq!(rc::dec_i(builtins::__sloth_dyn_unbox(o)), 41, "int unbox");
    assert_eq!(
        rc::dec_i(builtins::__sloth_dyn_hash(o, wi(0))),
        41,
        "int hash"
    );
    let s = builtins::__sloth_dyn_to_str(o, wi(0));
    let want = inter("41");
    assert_eq!(rc::dec_i(strings::__sloth_str_eq(s, want)), 1, "int to_str");
    rc::__sloth_rc_release(s);
    rc::__sloth_rc_release(want);
    // comparison family (equal / less-than)
    let p = objects::__sloth_obj_new(info, wi(1), 0);
    objects::__sloth_obj_set_field(p, wi(0), wi(41));
    assert_eq!(
        rc::dec_i(builtins::__sloth_dyn_binop(o, p, wi(0), wi(0))),
        1,
        "eq"
    );
    let q = objects::__sloth_obj_new(info, wi(1), 0);
    objects::__sloth_obj_set_field(q, wi(0), wi(9));
    assert_eq!(
        rc::dec_i(builtins::__sloth_dyn_binop(q, o, wi(0), wi(2))),
        1,
        "lt"
    );
    assert_eq!(
        rc::dec_i(builtins::__sloth_dyn_binop(o, q, wi(0), wi(2))),
        0,
        "not lt"
    );
    // float box keeps the encoded word; unbox/compare decode it
    let finfo = builtins::__sloth_builtin_info(wi(1));
    let fw = objects::__sloth_obj_new(finfo, wi(1), 0);
    objects::__sloth_obj_set_field(fw, wi(0), rc::enc_f_bits(2.5f64.to_bits()));
    assert_eq!(
        rc::dec_f_bits(builtins::__sloth_dyn_unbox(fw)),
        2.5f64.to_bits(),
        "float unbox"
    );
    // class metadata is untracked; only the four boxes sit in rc_live
    assert_eq!(rc::dec_i(rc::__sloth_rc_live()), before + 4, "four boxes");
    for w in [o, p, q, fw] {
        rc::__sloth_rc_release(w);
    }
    assert_eq!(
        rc::dec_i(rc::__sloth_rc_live()),
        before,
        "dyn boxes drained"
    );
}

/// The self-hosted container ABI symbols must NOT be exported by the runtime:
/// Array/Map algorithms live in `lib/prelude/containers.slt` and are provided
/// by the compiled module, not `libsloth_rt.so`.
#[test]
fn container_symbols_not_exported() {
    let _serial = serial();
    // `__sloth_rc_new` / `__sloth_mem_load` are the bare substrate and remain.
    let h = rc::__sloth_rc_new(8, 0, 0);
    assert_ne!(h, 0, "rc_new is exported");
    mem::__sloth_mem_store(h, 0, wi(5));
    assert_eq!(rc::dec_i(mem::__sloth_mem_load(h, 0)), 5);
    rc::__sloth_rc_release(h);
}

/// The range/box C-ABI faces are self-hosted in `lib/prelude/core.slt` and
/// must NOT be exported by the runtime. The Rust-internal `boxopt` helpers
/// remain for fiber/channel construction and `any` rendering, over the same
/// frozen `[payload]` layout the sloth prelude writes.
#[test]
fn core_symbols_not_exported() {
    let _serial = serial();
    let before = rc::dec_i(rc::__sloth_rc_live());
    let h = sloth_rt::boxopt::box_new(wi(7));
    assert_eq!(
        rc::dec_i(rc::__sloth_rc_live()),
        before + 1,
        "box rc-tracked"
    );
    assert_ne!(h, 0, "box handle is a real address");
    assert_eq!(
        sloth_rt::boxopt::box_get(h),
        wi(7),
        "frozen [payload] layout"
    );
    assert_eq!(sloth_rt::boxopt::box_get(0), 0, "nil box reads 0");
    rc::__sloth_rc_release(h);
    assert_eq!(rc::dec_i(rc::__sloth_rc_live()), before, "box drained");
}
