module @main {
  llvm.mlir.global private constant @sloth_tynm_0("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
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

