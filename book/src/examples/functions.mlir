module @main {
  llvm.mlir.global private constant @sloth_tynm_0("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  func.func @sloth_main__ginit() {
    call @sloth_main__anyinit() : () -> ()
    return
  }
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
  func.func @sloth_main__add(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_2[%c0] : memref<1xi64>
    %2 = arith.addi %0, %1 : i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  func.func @sloth_main__fact(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = arith.cmpi sle, %0, %c1_i64 : i64
    cf.cond_br %1, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    memref.store %c1_i64, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %2 = memref.load %alloca_1[%c0] : memref<1xi64>
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = arith.subi %3, %c1_i64 : i64
    %5 = call @sloth_main__fact(%4) : (i64) -> i64
    %6 = arith.muli %2, %5 : i64
    memref.store %6, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %7 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %7 : i64
  }
  func.func @sloth_main__greet(%arg0: i64) {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c2124136_i64 = arith.constant 2124136 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_0[%c0] : memref<1xi64>
    %0 = call @sloth_str_push(%c0_i64, %c2124136_i64, %c3_i64) : (i64, i64, i64) -> i64
    %1 = memref.load %alloca_0[%c0] : memref<1xi64>
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %2 : memref<11xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @sloth_any_from(%3, %1) : (i64, i64) -> i64
    %5 = call @sloth_rt_write(%4) : (i64) -> i64
    %6 = call @sloth_str_pushp(%0, %5) : (i64, i64) -> i64
    %7 = call @sloth_str_finish(%6) : (i64) -> i64
    %8 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_1 : index to i64
    %10 = call @sloth_any_from(%9, %7) : (i64, i64) -> i64
    call @sloth_main__print(%10) : (i64) -> ()
    %11 = call @sloth_rc_release(%4) : (i64) -> i64
    %12 = call @sloth_rc_release(%5) : (i64) -> i64
    %13 = call @sloth_rc_release(%7) : (i64) -> i64
    %14 = call @sloth_rc_release(%10) : (i64) -> i64
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__sum(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_arr_len(%0) : (i64) -> i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %2 = memref.load %alloca_3[%c0] : memref<1xi64>
    %3 = arith.cmpi slt, %2, %1 : i64
    cf.cond_br %3, ^bb2, ^bb4
  ^bb2:  // pred: ^bb1
    %4 = call @sloth_arr_get(%0, %2) : (i64, i64) -> i64
    %alloca_4 = memref.alloca() : memref<1xi64>
    memref.store %4, %alloca_4[%c0] : memref<1xi64>
    %5 = memref.load %alloca_2[%c0] : memref<1xi64>
    %6 = memref.load %alloca_4[%c0] : memref<1xi64>
    %7 = arith.addi %5, %6 : i64
    memref.store %7, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %8 = arith.addi %2, %c1_i64 : i64
    memref.store %8, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %9 = memref.load %alloca_2[%c0] : memref<1xi64>
    memref.store %9, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb5:  // pred: ^bb4
    %10 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %10 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c4_i64 = arith.constant 4 : i64
    %c448630058099_i64 = arith.constant 448630058099 : i64
    %c0_i64 = arith.constant 0 : i64
    %c5_i64 = arith.constant 5 : i64
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c2_i64 = arith.constant 2 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = call @sloth_main__add(%c2_i64, %c3_i64) : (i64, i64) -> i64
    %1 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %1 : memref<11xi64> -> index
    %2 = arith.index_cast %intptr : index to i64
    %3 = call @sloth_any_from(%2, %0) : (i64, i64) -> i64
    call @sloth_main__print(%3) : (i64) -> ()
    %4 = call @sloth_rc_release(%3) : (i64) -> i64
    %5 = call @sloth_main__add(%c1_i64, %c2_i64) : (i64, i64) -> i64
    %6 = call @sloth_main__add(%5, %c3_i64) : (i64, i64) -> i64
    %7 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %7 : memref<11xi64> -> index
    %8 = arith.index_cast %intptr_0 : index to i64
    %9 = call @sloth_any_from(%8, %6) : (i64, i64) -> i64
    call @sloth_main__print(%9) : (i64) -> ()
    %10 = call @sloth_rc_release(%9) : (i64) -> i64
    %11 = call @sloth_main__fact(%c5_i64) : (i64) -> i64
    %12 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %12 : memref<11xi64> -> index
    %13 = arith.index_cast %intptr_1 : index to i64
    %14 = call @sloth_any_from(%13, %11) : (i64, i64) -> i64
    call @sloth_main__print(%14) : (i64) -> ()
    %15 = call @sloth_rc_release(%14) : (i64) -> i64
    %16 = call @sloth_str_push(%c0_i64, %c448630058099_i64, %c5_i64) : (i64, i64, i64) -> i64
    %17 = call @sloth_str_finish(%16) : (i64) -> i64
    call @sloth_main__greet(%17) : (i64) -> ()
    %18 = call @sloth_rc_release(%17) : (i64) -> i64
    %19 = call @sloth_arr_new_k(%c4_i64, %c0_i64) : (i64, i64) -> i64
    %20 = call @sloth_arr_set(%19, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %21 = call @sloth_arr_set(%19, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %22 = call @sloth_arr_set(%19, %c2_i64, %c3_i64) : (i64, i64, i64) -> i64
    %23 = call @sloth_arr_set(%19, %c3_i64, %c4_i64) : (i64, i64, i64) -> i64
    %24 = call @sloth_main__sum(%19) : (i64) -> i64
    %25 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %25 : memref<11xi64> -> index
    %26 = arith.index_cast %intptr_2 : index to i64
    %27 = call @sloth_any_from(%26, %24) : (i64, i64) -> i64
    call @sloth_main__print(%27) : (i64) -> ()
    %28 = call @sloth_rc_release(%27) : (i64) -> i64
    %29 = call @sloth_arr_new_k(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %30 = call @sloth_main__sum(%29) : (i64) -> i64
    %31 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %31 : memref<11xi64> -> index
    %32 = arith.index_cast %intptr_3 : index to i64
    %33 = call @sloth_any_from(%32, %30) : (i64, i64) -> i64
    call @sloth_main__print(%33) : (i64) -> ()
    %34 = call @sloth_rc_release(%33) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c4_i64, %2[%c0] : memref<11xi64>
    memref.store %c1_i64, %2[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %2[%c2] : memref<11xi64>
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %3, %2[%c3] : memref<11xi64>
    memref.store %c3_i64, %2[%c4] : memref<11xi64>
    memref.store %c0_i64, %2[%c9] : memref<11xi64>
    memref.store %c0_i64, %2[%c10] : memref<11xi64>
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c2_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

