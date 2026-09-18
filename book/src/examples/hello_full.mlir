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

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func private @sloth_panic_unwrap() -> i64
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 448630058099 : i64
    %v1003 = arith.constant 10 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1006[%v1008] : memref<1xi64>
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1009 = arith.constant 0 : i64
    %v1010 = arith.constant 9056056326776168 : i64
    %v1011 = arith.constant 14 : i64
    %v1012 = call @sloth_str_push(%v1009, %v1010, %v1011) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 0 : index
    %v1014 = memref.load %v1006[%v1013] : memref<1xi64>
    %v1015 = call @sloth_str_pushp(%v1012, %v1014) : (i64, i64) -> i64
    %v1016 = arith.constant 33 : i64
    %v1017 = arith.constant 2 : i64
    %v1018 = call @sloth_str_push(%v1015, %v1016, %v1017) : (i64, i64, i64) -> i64
    %v1019 = call @sloth_str_finish(%v1018) : (i64) -> i64
    %v1020 = call @sloth_rt_print_str(%v1019) : (i64) -> i64
    call @sloth_rc_release(%v1019) : (i64) -> i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1006[%v1021] : memref<1xi64>
    call @sloth_rc_release(%v1022) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

