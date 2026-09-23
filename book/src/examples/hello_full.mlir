module @main {
  llvm.mlir.global private constant @sloth_tynm_0("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  func.func private @sloth_rc_retain(i64) -> i64
  func.func private @sloth_rc_release(i64) -> i64
  func.func private @sloth_rc_live() -> i64
  func.func private @sloth_rc_drops() -> i64
  func.func private @sloth_weak_new(i64) -> i64
  func.func private @sloth_weak_upgrade(i64) -> i64
  func.func private @sloth_weak_release(i64) -> i64
  func.func private @sloth_box_new(i64) -> i64
  func.func private @sloth_box_get(i64) -> i64
  func.func private @sloth_rt_print_opt(i64, i64) -> i64
  func.func private @sloth_str_push_opt(i64, i64, i64) -> i64
  func.func private @sloth_rt_print_i64(i64) -> i64
  func.func private @sloth_rt_print_f64(f64) -> i64
  func.func private @sloth_rt_print_bool(i64) -> i64
  func.func private @sloth_rt_print_str(i64) -> i64
  func.func private @sloth_any_from(i64, i64) -> i64
  func.func private @sloth_any_desc(i64) -> i64
  func.func private @sloth_any_word(i64) -> i64
  func.func private @sloth_any_kind(i64) -> i64
  func.func private @sloth_any_cls_id(i64) -> i64
  func.func private @sloth_any_ref(i64) -> i64
  func.func private @sloth_any_retain(i64) -> i64
  func.func private @sloth_any_is(i64, i64) -> i64
  func.func private @sloth_any_type_id(i64) -> i64
  func.func private @sloth_any_type_name(i64) -> i64
  func.func private @sloth_rt_write(i64) -> i64
  func.func private @sloth_rt_puts(i64)
  func.func private @sloth_str_intern(i64, i64) -> i64
  func.func private @sloth_range_pack(i64, i64) -> i64
  func.func private @sloth_range_lo(i64) -> i64
  func.func private @sloth_range_hi(i64) -> i64
  func.func private @sloth_str_push(i64, i64, i64) -> i64
  func.func private @sloth_str_finish(i64) -> i64
  func.func private @sloth_str_pushp(i64, i64) -> i64
  func.func private @sloth_str_push_i(i64, i64) -> i64
  func.func private @sloth_str_push_f(i64, f64) -> i64
  func.func private @sloth_str_push_b(i64, i64) -> i64
  func.func private @sloth_str_len(i64) -> i64
  func.func private @sloth_str_clen(i64) -> i64
  func.func private @sloth_str_char(i64, i64) -> i64
  func.func private @sloth_str_concat(i64, i64) -> i64
  func.func private @sloth_str_eq(i64, i64) -> i64
  func.func private @sloth_rt_sqrt(f64) -> f64
  func.func private @sloth_rt_exp(f64) -> f64
  func.func private @sloth_rt_sin(f64) -> f64
  func.func private @sloth_rt_cos(f64) -> f64
  func.func private @sloth_rt_tan(f64) -> f64
  func.func private @sloth_rt_pow(f64, f64) -> f64
  func.func private @sloth_rt_floor(f64) -> f64
  func.func private @sloth_tensor_new_1(i64, i64) -> i64
  func.func private @sloth_tensor_new_2(i64, i64, i64) -> i64
  func.func private @sloth_tensor_new_3(i64, i64, i64, i64) -> i64
  func.func private @sloth_tensor_view(i64, i64, i64, i64) -> i64
  func.func private @sloth_tensor_get1(i64, i64) -> i64
  func.func private @sloth_tensor_set1(i64, i64, i64) -> i64
  func.func private @sloth_tensor_copy_into(i64, i64) -> i64
  func.func private @sloth_tensor_copy_from_array(i64, i64) -> i64
  func.func private @sloth_tensor_rank(i64) -> i64
  func.func private @sloth_tensor_dim(i64, i64) -> i64
  func.func private @sloth_tensor_stride(i64, i64) -> i64
  func.func private @sloth_tensor_fill_zero(i64) -> i64
  func.func private @sloth_tensor_basis_f64(i64) -> memref<?xf64, strided<[?], offset: ?>>
  func.func private @sloth_tensor_basis_i64(i64) -> memref<?xi64, strided<[?], offset: ?>>
  func.func private @sloth_tensor_shape_eq(i64, i64) -> i64
  func.func private @sloth_tensor_dim_eq(i64, i64, i64, i64) -> i64
  func.func private @sloth_fiber_create(i64, i64, i64) -> i64
  func.func private @sloth_fiber_create_with(i64, i64, i64, i64) -> i64
  func.func private @sloth_fiber_resume(i64, i64, i64) -> i64
  func.func private @sloth_fiber_transfer(i64, i64, i64) -> i64
  func.func private @sloth_fiber_yield(i64) -> i64
  func.func private @sloth_fiber_error(i64) -> i64
  func.func private @sloth_fiber_check(i64) -> i64
  func.func private @sloth_fiber_resumable(i64) -> i64
  func.func private @sloth_fiber_cancel(i64) -> i64
  func.func private @sloth_fiber_cancelled() -> i64
  func.func private @sloth_fiber_cancel_abort()
  func.func private @sloth_fiber_track(i64) -> i64
  func.func private @sloth_fiber_untrack(i64) -> i64
  func.func private @sloth_thread_spawn(i64, i64, i64, i64) -> i64
  func.func private @sloth_thread_join(i64) -> i64
  func.func private @sloth_thread_detach(i64) -> i64
  func.func private @sloth_thread_current_id() -> i64
  func.func private @sloth_thread_yield_now() -> i64
  func.func private @sloth_chan_new(i64, i64) -> i64
  func.func private @sloth_chan_send(i64, i64) -> i64
  func.func private @sloth_chan_recv(i64, i64) -> i64
  func.func private @sloth_chan_close(i64) -> i64
  func.func private @sloth_mutex_new() -> i64
  func.func private @sloth_mutex_lock(i64) -> i64
  func.func private @sloth_mutex_unlock(i64) -> i64
  func.func private @sloth_mutex_try_lock(i64) -> i64
  func.func private @sloth_mutex_with(i64, i64) -> i64
  func.func private @sloth_atomic_new(i64) -> i64
  func.func private @sloth_atomic_load(i64) -> i64
  func.func private @sloth_atomic_store(i64, i64) -> i64
  func.func private @sloth_atomic_add(i64, i64) -> i64
  func.func private @sloth_atomic_sub(i64, i64) -> i64
  func.func private @sloth_atomic_cas(i64, i64, i64) -> i64
  func.func private @sloth_obj_new(i64, i64, i64) -> i64
  func.func private @sloth_closure_new(i64, i64) -> i64
  func.func private @sloth_obj_field(i64, i64) -> i64
  func.func private @sloth_obj_set_field(i64, i64, i64) -> i64
  func.func private @sloth_cls_info(i64, i64) -> i64
  func.func private @sloth_cls_name(i64, i64, i64) -> i64
  func.func private @sloth_obj_type_name(i64) -> i64
  func.func private @sloth_type_name_or(i64, i64, i64) -> i64
  func.func private @sloth_obj_cls_id(i64) -> i64
  func.func private @sloth_vt_new(i64) -> i64
  func.func private @sloth_vt_set(i64, i64, i64) -> i64
  func.func private @sloth_vt_get(i64, i64) -> i64
  func.func private @sloth_obj_set_vtable(i64, i64) -> i64
  func.func private @sloth_obj_vtable(i64) -> i64
  func.func private @sloth_panic_noimpl(i64) -> i64
  func.func private @sloth_panic_divzero() -> i64
  func.func private @sloth_builtin_info(i64) -> i64
  func.func private @sloth_dyn_unbox(i64) -> i64
  func.func private @sloth_dyn_to_str(i64, i64) -> i64
  func.func private @sloth_dyn_hash(i64, i64) -> i64
  func.func private @sloth_dyn_binop(i64, i64, i64, i64) -> i64
  func.func @sloth_main__ginit() {
    call @sloth_main__anyinit() : () -> ()
    return
  }
  func.func private @sloth_rt_alloc(i64) -> i64
  func.func private @sloth_free(i64)
  func.func private @sloth_mem_load(i64, i64) -> i64
  func.func private @sloth_mem_store(i64, i64, i64)
  func.func private @sloth_mem_copy(i64, i64, i64)
  func.func private @sloth_rc_new(i64, i64, i64) -> i64
  func.func private @sloth_str_byte(i64, i64) -> i64
  func.func private @sloth_panic_nokey(i64) -> i64
  func.func private @sloth_panic_oob(i64, i64) -> i64
  func.func private @sloth_panic_pop(i64, i64) -> i64
  func.func @sloth_main__print(%arg0: i64) {
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %0 = memref.load %alloca[%c0] : memref<1xi64>
    %1 = call @sloth_rt_write(%0) : (i64) -> i64
    call @sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @sloth_rc_release(%1) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func private @sloth_panic_unwrap() -> i64
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c1_i64 = arith.constant 1 : i64
    %c33_i64 = arith.constant 33 : i64
    %c7_i64 = arith.constant 7 : i64
    %c9056056326776168_i64 = arith.constant 9056056326776168 : i64
    %c0 = arith.constant 0 : index
    %c5_i64 = arith.constant 5 : i64
    %c448630058099_i64 = arith.constant 448630058099 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = call @sloth_str_push(%c0_i64, %c448630058099_i64, %c5_i64) : (i64, i64, i64) -> i64
    %1 = call @sloth_str_finish(%0) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %2 = call @sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @sloth_fiber_track(%3) : (i64) -> i64
    %5 = call @sloth_rc_release(%1) : (i64) -> i64
    %6 = call @sloth_str_push(%c0_i64, %c9056056326776168_i64, %c7_i64) : (i64, i64, i64) -> i64
    %7 = memref.load %alloca[%c0] : memref<1xi64>
    %8 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_0 : index to i64
    %10 = call @sloth_any_from(%9, %7) : (i64, i64) -> i64
    %11 = call @sloth_rt_write(%10) : (i64) -> i64
    %12 = call @sloth_str_pushp(%6, %11) : (i64, i64) -> i64
    %13 = call @sloth_str_push(%12, %c33_i64, %c1_i64) : (i64, i64, i64) -> i64
    %14 = call @sloth_str_finish(%13) : (i64) -> i64
    %15 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %15 : memref<11xi64> -> index
    %16 = arith.index_cast %intptr_1 : index to i64
    %17 = call @sloth_any_from(%16, %14) : (i64, i64) -> i64
    call @sloth_main__print(%17) : (i64) -> ()
    %18 = call @sloth_rc_release(%10) : (i64) -> i64
    %19 = call @sloth_rc_release(%11) : (i64) -> i64
    %20 = call @sloth_rc_release(%14) : (i64) -> i64
    %21 = call @sloth_rc_release(%17) : (i64) -> i64
    %22 = memref.load %alloca[%c0] : memref<1xi64>
    %23 = call @sloth_rc_release(%22) : (i64) -> i64
    %intptr_2 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %24 = arith.index_cast %intptr_2 : index to i64
    %25 = call @sloth_fiber_untrack(%24) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %1 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c4_i64, %1[%c0] : memref<11xi64>
    memref.store %c1_i64, %1[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %1[%c2] : memref<11xi64>
    %2 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %2, %1[%c3] : memref<11xi64>
    memref.store %c3_i64, %1[%c4] : memref<11xi64>
    memref.store %c0_i64, %1[%c9] : memref<11xi64>
    memref.store %c0_i64, %1[%c10] : memref<11xi64>
    return
  }
}

