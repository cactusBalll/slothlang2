module @main {
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
  func.func private @sloth_str_intern(i64, i64) -> i64
  func.func private @sloth_range_pack(i64, i64) -> i64
  func.func private @sloth_range_lo(i64) -> i64
  func.func private @sloth_range_hi(i64) -> i64
  func.func private @sloth_str_push(i64, i64, i64) -> i64
  func.func private @sloth_arr_push(i64, i64) -> i64
  func.func private @sloth_arr_pop(i64) -> i64
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
  func.func private @sloth_arr_new(i64) -> i64
  func.func private @sloth_arr_len(i64) -> i64
  func.func private @sloth_arr_get(i64, i64) -> i64
  func.func private @sloth_arr_set(i64, i64, i64) -> i64
  func.func private @sloth_map_new(i64) -> i64
  func.func private @sloth_map_len(i64) -> i64
  func.func private @sloth_map_get(i64, i64) -> i64
  func.func private @sloth_map_set(i64, i64, i64) -> i64
  func.func private @sloth_map_get_h(i64, i64, i64) -> i64
  func.func private @sloth_map_set_h(i64, i64, i64, i64) -> i64
  func.func private @sloth_map_str_get(i64, i64) -> i64
  func.func private @sloth_map_str_set(i64, i64, i64) -> i64
  func.func private @sloth_map_keys(i64) -> i64
  func.func private @sloth_map_values(i64) -> i64
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
  func.func private @sloth_fiber_cancel_abort() -> ()
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
  func.func private @sloth_obj_new(i64, i64) -> i64
  func.func private @sloth_closure_new(i64, i64) -> i64
  func.func private @sloth_obj_field(i64, i64) -> i64
  func.func private @sloth_obj_set_field(i64, i64, i64) -> i64
  func.func private @sloth_cls_info(i64, i64) -> i64
  func.func private @sloth_cls_refmask(i64, i64, i64) -> i64
  func.func private @sloth_arr_new_k(i64, i64) -> i64
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

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func private @sloth_panic_unwrap() -> i64
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 448630058099 : i64
    %v1003 = arith.constant 5 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1006[%v1008] : memref<1xi64>
    %v1009 = memref.extract_aligned_pointer_as_index %v1006 : memref<1xi64> -> index
    %v1010 = arith.index_cast %v1009 : index to i64
    call @sloth_fiber_track(%v1010) : (i64) -> i64
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1011 = arith.constant 0 : i64
    %v1012 = arith.constant 9056056326776168 : i64
    %v1013 = arith.constant 7 : i64
    %v1014 = call @sloth_str_push(%v1011, %v1012, %v1013) : (i64, i64, i64) -> i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1006[%v1015] : memref<1xi64>
    %v1017 = call @sloth_str_pushp(%v1014, %v1016) : (i64, i64) -> i64
    %v1018 = arith.constant 33 : i64
    %v1019 = arith.constant 1 : i64
    %v1020 = call @sloth_str_push(%v1017, %v1018, %v1019) : (i64, i64, i64) -> i64
    %v1021 = call @sloth_str_finish(%v1020) : (i64) -> i64
    %v1022 = call @sloth_rt_print_str(%v1021) : (i64) -> i64
    call @sloth_rc_release(%v1021) : (i64) -> i64
    %v1023 = arith.constant 0 : index
    %v1024 = memref.load %v1006[%v1023] : memref<1xi64>
    call @sloth_rc_release(%v1024) : (i64) -> i64
    %v1025 = memref.extract_aligned_pointer_as_index %v1006 : memref<1xi64> -> index
    %v1026 = arith.index_cast %v1025 : index to i64
    call @sloth_fiber_untrack(%v1026) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

