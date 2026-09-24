module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_g : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_fixed : memref<1xi64> = dense<0> {mutable}
  func.func @sloth_main__ginit() {
    %c3_i64 = arith.constant 3 : i64
    %c0 = arith.constant 0 : index
    %c10_i64 = arith.constant 10 : i64
    call @sloth_main__anyinit() : () -> ()
    %0 = memref.get_global @sloth_main_g_g : memref<1xi64>
    memref.store %c10_i64, %0[%c0] : memref<1xi64>
    %1 = memref.get_global @sloth_main_g_fixed : memref<1xi64>
    memref.store %c3_i64, %1[%c0] : memref<1xi64>
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
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c100_i64 = arith.constant 100 : i64
    %c2_i64 = arith.constant 2 : i64
    %c26984_i64 = arith.constant 26984 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %0 = call @__sloth_str_push(%c0_i64, %c26984_i64, %c2_i64) : (i64, i64, i64) -> i64
    %1 = call @__sloth_str_finish(%0) : (i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    %2 = call @__sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @__sloth_fiber_track(%3) : (i64) -> i64
    %5 = call @__sloth_rc_release(%1) : (i64) -> i64
    %6 = memref.load %alloca[%c0] : memref<1xi64>
    %7 = arith.addi %6, %c1_i64 : i64
    memref.store %7, %alloca[%c0] : memref<1xi64>
    %8 = memref.load %alloca[%c0] : memref<1xi64>
    %9 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %9 : memref<11xi64> -> index
    %10 = arith.index_cast %intptr_1 : index to i64
    %11 = call @__sloth_any_from(%10, %8) : (i64, i64) -> i64
    call @sloth_main__print(%11) : (i64) -> ()
    %12 = call @__sloth_rc_release(%11) : (i64) -> i64
    %13 = memref.load %alloca_0[%c0] : memref<1xi64>
    %14 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %14 : memref<11xi64> -> index
    %15 = arith.index_cast %intptr_2 : index to i64
    %16 = call @__sloth_any_from(%15, %13) : (i64, i64) -> i64
    call @sloth_main__print(%16) : (i64) -> ()
    %17 = call @__sloth_rc_release(%16) : (i64) -> i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %c100_i64, %alloca_3[%c0] : memref<1xi64>
    %18 = memref.load %alloca_3[%c0] : memref<1xi64>
    %19 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %19 : memref<11xi64> -> index
    %20 = arith.index_cast %intptr_4 : index to i64
    %21 = call @__sloth_any_from(%20, %18) : (i64, i64) -> i64
    call @sloth_main__print(%21) : (i64) -> ()
    %22 = call @__sloth_rc_release(%21) : (i64) -> i64
    %23 = memref.load %alloca[%c0] : memref<1xi64>
    %24 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %24 : memref<11xi64> -> index
    %25 = arith.index_cast %intptr_5 : index to i64
    %26 = call @__sloth_any_from(%25, %23) : (i64, i64) -> i64
    call @sloth_main__print(%26) : (i64) -> ()
    %27 = call @__sloth_rc_release(%26) : (i64) -> i64
    %28 = memref.get_global @sloth_main_g_g : memref<1xi64>
    %29 = memref.load %28[%c0] : memref<1xi64>
    %30 = memref.get_global @sloth_main_g_fixed : memref<1xi64>
    %31 = memref.load %30[%c0] : memref<1xi64>
    %32 = arith.addi %29, %31 : i64
    %33 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %33 : memref<11xi64> -> index
    %34 = arith.index_cast %intptr_6 : index to i64
    %35 = call @__sloth_any_from(%34, %32) : (i64, i64) -> i64
    call @sloth_main__print(%35) : (i64) -> ()
    %36 = call @__sloth_rc_release(%35) : (i64) -> i64
    %37 = memref.load %alloca_0[%c0] : memref<1xi64>
    %38 = call @__sloth_rc_release(%37) : (i64) -> i64
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %39 = arith.index_cast %intptr_7 : index to i64
    %40 = call @__sloth_fiber_untrack(%39) : (i64) -> i64
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
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c1_i64 = arith.constant 1 : i64
    %c4_i64 = arith.constant 4 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c2_i64, %2[%c0] : memref<11xi64>
    memref.store %c0_i64, %2[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %2[%c2] : memref<11xi64>
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %3, %2[%c3] : memref<11xi64>
    memref.store %c3_i64, %2[%c4] : memref<11xi64>
    memref.store %c0_i64, %2[%c9] : memref<11xi64>
    memref.store %c0_i64, %2[%c10] : memref<11xi64>
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c4_i64, %4[%c0] : memref<11xi64>
    memref.store %c1_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

