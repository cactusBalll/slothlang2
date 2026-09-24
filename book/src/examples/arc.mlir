module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Holder\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("bool\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Holder : memref<1xi64> = dense<0> {mutable}
  func.func @sloth_main__ginit() {
    call @sloth_main__anyinit() : () -> ()
    return
  }
  func.func @sloth_main__print(%arg0: i64) {
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %0 = memref.load %alloca[%c0] : memref<1xi64>
    %1 = call @__sloth_rt_write(%0) : (i64) -> i64
    call @__sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @__sloth_rc_release(%1) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  llvm.func @sloth_main_Holder__cascade(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @__sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @__sloth_rc_release(%0) : (i64) -> i64
    %2 = func.call @__sloth_obj_field(%arg0, %c1_i64) : (i64, i64) -> i64
    %3 = func.call @__sloth_rc_release(%2) : (i64) -> i64
    llvm.return %c0_i64 : i64
  }
  func.func private @sloth_vtb_main_Holder() -> i64 {
    %c5_i64 = arith.constant 5 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %0 = memref.get_global @sloth_main_g_vtb_Holder : memref<1xi64>
    %1 = memref.load %0[%c0] : memref<1xi64>
    %2 = arith.cmpi eq, %1, %c0_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = call @__sloth_vt_new(%c5_i64) : (i64) -> i64
    memref.store %3, %0[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %4 = memref.load %0[%c0] : memref<1xi64>
    return %4 : i64
  }
  func.func @sloth_main__churn(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %0 = llvm.mlir.addressof @sloth_main_Holder__cascade : !llvm.ptr
    %c6_i64 = arith.constant 6 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_2[%c0] : memref<1xi64>
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb2
    %2 = memref.load %alloca_3[%c0] : memref<1xi64>
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = arith.cmpi slt, %2, %3 : i64
    cf.cond_br %4, ^bb2, ^bb3
  ^bb2:  // pred: ^bb1
    %5 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %7 = call @__sloth_cls_name(%5, %6, %c6_i64) : (i64, i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = call @__sloth_obj_new(%5, %c2_i64, %8) : (i64, i64, i64) -> i64
    %10 = call @sloth_vtb_main_Holder() : () -> i64
    %11 = call @__sloth_obj_set_vtable(%9, %10) : (i64, i64) -> i64
    call @sloth_main_Holder____init__(%9) : (i64) -> ()
    %alloca_4 = memref.alloca() : memref<1xi64>
    %12 = call @__sloth_rc_retain(%9) : (i64) -> i64
    memref.store %12, %alloca_4[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca_4 : memref<1xi64> -> index
    %13 = arith.index_cast %intptr : index to i64
    %14 = call @__sloth_fiber_track(%13) : (i64) -> i64
    %15 = call @__sloth_rc_release(%9) : (i64) -> i64
    %16 = memref.load %alloca_2[%c0] : memref<1xi64>
    %17 = memref.load %alloca_4[%c0] : memref<1xi64>
    %18 = call @__sloth_obj_field(%17, %c0_i64) : (i64, i64) -> i64
    %19 = call @__sloth_str_len(%18) : (i64) -> i64
    %20 = arith.addi %16, %19 : i64
    %21 = memref.load %alloca_4[%c0] : memref<1xi64>
    %22 = call @__sloth_obj_field(%21, %c1_i64) : (i64, i64) -> i64
    %23 = call @__sloth_arr_len(%22) : (i64) -> i64
    %24 = arith.addi %20, %23 : i64
    memref.store %24, %alloca_2[%c0] : memref<1xi64>
    %25 = memref.load %alloca_3[%c0] : memref<1xi64>
    %26 = arith.addi %25, %c1_i64 : i64
    memref.store %26, %alloca_3[%c0] : memref<1xi64>
    %27 = memref.load %alloca_4[%c0] : memref<1xi64>
    %28 = call @__sloth_rc_release(%27) : (i64) -> i64
    %intptr_5 = memref.extract_aligned_pointer_as_index %alloca_4 : memref<1xi64> -> index
    %29 = arith.index_cast %intptr_5 : index to i64
    %30 = call @__sloth_fiber_untrack(%29) : (i64) -> i64
    cf.br ^bb1
  ^bb3:  // pred: ^bb1
    %31 = memref.load %alloca_2[%c0] : memref<1xi64>
    memref.store %31, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // pred: ^bb3
    %32 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %32 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c0_i64 = arith.constant 0 : i64
    %c1000_i64 = arith.constant 1000 : i64
    %c0 = arith.constant 0 : index
    %c50_i64 = arith.constant 50 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = call @sloth_main__churn(%c50_i64) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %0, %alloca[%c0] : memref<1xi64>
    %1 = call @__sloth_rc_live() : () -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    %2 = call @sloth_main__churn(%c1000_i64) : (i64) -> i64
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %2, %alloca_1[%c0] : memref<1xi64>
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = arith.cmpi sgt, %3, %c0_i64 : i64
    %5 = arith.extui %4 : i1 to i64
    %6 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %6 : memref<11xi64> -> index
    %7 = arith.index_cast %intptr : index to i64
    %8 = call @__sloth_any_from(%7, %5) : (i64, i64) -> i64
    call @sloth_main__print(%8) : (i64) -> ()
    %9 = call @__sloth_rc_release(%8) : (i64) -> i64
    %10 = call @__sloth_rc_live() : () -> i64
    %11 = memref.load %alloca_0[%c0] : memref<1xi64>
    %12 = arith.cmpi eq, %10, %11 : i64
    %13 = arith.extui %12 : i1 to i64
    %14 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %14 : memref<11xi64> -> index
    %15 = arith.index_cast %intptr_2 : index to i64
    %16 = call @__sloth_any_from(%15, %13) : (i64, i64) -> i64
    call @sloth_main__print(%16) : (i64) -> ()
    %17 = call @__sloth_rc_release(%16) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_StrChars____init__(%arg0: i64, %arg1: i64) {
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_0[%c0] : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_0[%c0] : memref<1xi64>
    %2 = call @__sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = call @__sloth_rc_release(%2) : (i64) -> i64
    %4 = call @__sloth_rc_retain(%0) : (i64) -> i64
    %5 = call @__sloth_obj_set_field(%1, %c0_i64, %4) : (i64, i64, i64) -> i64
    %6 = memref.load %alloca_0[%c0] : memref<1xi64>
    %7 = call @__sloth_obj_set_field(%6, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %8 = memref.load %alloca_1[%c0] : memref<1xi64>
    %9 = call @__sloth_str_clen(%8) : (i64) -> i64
    %10 = memref.load %alloca_0[%c0] : memref<1xi64>
    %11 = call @__sloth_obj_set_field(%10, %c2_i64, %9) : (i64, i64, i64) -> i64
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  llvm.func @sloth_main_StrChars__iter(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_rc_retain(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
  }
  llvm.func @sloth_main_StrChars__next(%arg0: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c1_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_1[%c0] : memref<1xi64>
    %3 = func.call @__sloth_obj_field(%2, %c2_i64) : (i64, i64) -> i64
    %4 = arith.cmpi sge, %1, %3 : i64
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %5 = func.call @__sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = func.call @__sloth_obj_field(%6, %c0_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_1[%c0] : memref<1xi64>
    %9 = func.call @__sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %10 = func.call @__sloth_str_codepoint(%7, %9) : (i64, i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %10, %alloca_2[%c0] : memref<1xi64>
    %11 = memref.load %alloca_1[%c0] : memref<1xi64>
    %12 = func.call @__sloth_obj_field(%11, %c1_i64) : (i64, i64) -> i64
    %13 = arith.addi %12, %c1_i64 : i64
    %14 = memref.load %alloca_1[%c0] : memref<1xi64>
    %15 = func.call @__sloth_obj_set_field(%14, %c1_i64, %13) : (i64, i64, i64) -> i64
    %16 = memref.load %alloca_2[%c0] : memref<1xi64>
    %17 = func.call @__sloth_box_new(%16) : (i64) -> i64
    memref.store %17, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %18 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %18 : i64
  }
  func.func @sloth_main_Holder____init__(%arg0: i64) {
    %c3_i64 = arith.constant 3 : i64
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c5_i64 = arith.constant 5 : i64
    %c478560413032_i64 = arith.constant 478560413032 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %0 = call @__sloth_str_push(%c0_i64, %c478560413032_i64, %c5_i64) : (i64, i64, i64) -> i64
    %1 = call @__sloth_str_finish(%0) : (i64) -> i64
    %2 = memref.load %alloca[%c0] : memref<1xi64>
    %3 = call @__sloth_obj_field(%2, %c0_i64) : (i64, i64) -> i64
    %4 = call @__sloth_rc_release(%3) : (i64) -> i64
    %5 = call @__sloth_rc_retain(%1) : (i64) -> i64
    %6 = call @__sloth_obj_set_field(%2, %c0_i64, %5) : (i64, i64, i64) -> i64
    %7 = call @__sloth_rc_release(%1) : (i64) -> i64
    %8 = call @__sloth_arr_new(%c3_i64) : (i64) -> i64
    %9 = call @__sloth_arr_set(%8, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %10 = call @__sloth_arr_set(%8, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %11 = call @__sloth_arr_set(%8, %c2_i64, %c3_i64) : (i64, i64, i64) -> i64
    %12 = memref.load %alloca[%c0] : memref<1xi64>
    %13 = call @__sloth_obj_field(%12, %c1_i64) : (i64, i64) -> i64
    %14 = call @__sloth_rc_release(%13) : (i64) -> i64
    %15 = call @__sloth_rc_retain(%8) : (i64) -> i64
    %16 = call @__sloth_obj_set_field(%12, %c1_i64, %15) : (i64, i64, i64) -> i64
    %17 = call @__sloth_rc_release(%8) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c4_i64 = arith.constant 4 : i64
    %c3 = arith.constant 3 : index
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %1 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c1_i64, %1[%c0] : memref<11xi64>
    memref.store %c0_i64, %1[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %1[%c2] : memref<11xi64>
    %2 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %2, %1[%c3] : memref<11xi64>
    memref.store %c4_i64, %1[%c4] : memref<11xi64>
    memref.store %c0_i64, %1[%c9] : memref<11xi64>
    memref.store %c0_i64, %1[%c10] : memref<11xi64>
    return
  }
}

