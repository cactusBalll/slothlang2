module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("float\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_2 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_3 : memref<11xi64> = dense<0> {mutable}
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
    %c67544357281836_i64 = arith.constant 67544357281836 : i64
    %c3_i64 = arith.constant 3 : i64
    %c2124136_i64 = arith.constant 2124136 : i64
    %c0 = arith.constant 0 : index
    %c5_i64 = arith.constant 5 : i64
    %c448630058099_i64 = arith.constant 448630058099 : i64
    %c2323048382453194801_i64 = arith.constant 2323048382453194801 : i64
    %c6_i64 = arith.constant 6 : i64
    %c111481940307561_i64 = arith.constant 111481940307561 : i64
    %c2315448778539103601_i64 = arith.constant 2315448778539103601 : i64
    %c8_i64 = arith.constant 8 : i64
    %c7310016642684182900_i64 = arith.constant 7310016642684182900 : i64
    %c4598175219545276416_i64 = arith.constant 4598175219545276416 : i64
    %c4652007308841189376_i64 = arith.constant 4652007308841189376 : i64
    %c4_i64 = arith.constant 4 : i64
    %c1954047348_i64 = arith.constant 1954047348 : i64
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c4615063718147915776_i64 = arith.constant 4615063718147915776 : i64
    %c42_i64 = arith.constant 42 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %0 : memref<11xi64> -> index
    %1 = arith.index_cast %intptr : index to i64
    %2 = call @sloth_any_from(%1, %c42_i64) : (i64, i64) -> i64
    call @sloth_main__print(%2) : (i64) -> ()
    %3 = call @sloth_rc_release(%2) : (i64) -> i64
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %4 : memref<11xi64> -> index
    %5 = arith.index_cast %intptr_0 : index to i64
    %6 = call @sloth_any_from(%5, %c4615063718147915776_i64) : (i64, i64) -> i64
    call @sloth_main__print(%6) : (i64) -> ()
    %7 = call @sloth_rc_release(%6) : (i64) -> i64
    %8 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_1 : index to i64
    %10 = call @sloth_any_from(%9, %c1_i64) : (i64, i64) -> i64
    call @sloth_main__print(%10) : (i64) -> ()
    %11 = call @sloth_rc_release(%10) : (i64) -> i64
    %12 = call @sloth_str_push(%c0_i64, %c1954047348_i64, %c4_i64) : (i64, i64, i64) -> i64
    %13 = call @sloth_str_finish(%12) : (i64) -> i64
    %14 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %14 : memref<11xi64> -> index
    %15 = arith.index_cast %intptr_2 : index to i64
    %16 = call @sloth_any_from(%15, %13) : (i64, i64) -> i64
    call @sloth_main__print(%16) : (i64) -> ()
    %17 = call @sloth_rc_release(%13) : (i64) -> i64
    %18 = call @sloth_rc_release(%16) : (i64) -> i64
    %19 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %19 : memref<11xi64> -> index
    %20 = arith.index_cast %intptr_3 : index to i64
    %21 = call @sloth_any_from(%20, %c4652007308841189376_i64) : (i64, i64) -> i64
    call @sloth_main__print(%21) : (i64) -> ()
    %22 = call @sloth_rc_release(%21) : (i64) -> i64
    %23 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %23 : memref<11xi64> -> index
    %24 = arith.index_cast %intptr_4 : index to i64
    %25 = call @sloth_any_from(%24, %c4598175219545276416_i64) : (i64, i64) -> i64
    call @sloth_main__print(%25) : (i64) -> ()
    %26 = call @sloth_rc_release(%25) : (i64) -> i64
    %27 = call @sloth_str_push(%c0_i64, %c7310016642684182900_i64, %c8_i64) : (i64, i64, i64) -> i64
    %28 = call @sloth_str_finish(%27) : (i64) -> i64
    %29 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %29 : memref<11xi64> -> index
    %30 = arith.index_cast %intptr_5 : index to i64
    %31 = call @sloth_any_from(%30, %28) : (i64, i64) -> i64
    call @sloth_main__print(%31) : (i64) -> ()
    %32 = call @sloth_rc_release(%28) : (i64) -> i64
    %33 = call @sloth_rc_release(%31) : (i64) -> i64
    %34 = call @sloth_str_push(%c0_i64, %c2315448778539103601_i64, %c8_i64) : (i64, i64, i64) -> i64
    %35 = call @sloth_str_push(%34, %c111481940307561_i64, %c6_i64) : (i64, i64, i64) -> i64
    %36 = call @sloth_str_finish(%35) : (i64) -> i64
    %37 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %37 : memref<11xi64> -> index
    %38 = arith.index_cast %intptr_6 : index to i64
    %39 = call @sloth_any_from(%38, %36) : (i64, i64) -> i64
    call @sloth_main__print(%39) : (i64) -> ()
    %40 = call @sloth_rc_release(%36) : (i64) -> i64
    %41 = call @sloth_rc_release(%39) : (i64) -> i64
    %42 = call @sloth_str_push(%c0_i64, %c2323048382453194801_i64, %c8_i64) : (i64, i64, i64) -> i64
    %43 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %43 : memref<11xi64> -> index
    %44 = arith.index_cast %intptr_7 : index to i64
    %45 = call @sloth_any_from(%44, %c3_i64) : (i64, i64) -> i64
    %46 = call @sloth_rt_write(%45) : (i64) -> i64
    %47 = call @sloth_str_pushp(%42, %46) : (i64, i64) -> i64
    %48 = call @sloth_str_finish(%47) : (i64) -> i64
    %49 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %49 : memref<11xi64> -> index
    %50 = arith.index_cast %intptr_8 : index to i64
    %51 = call @sloth_any_from(%50, %48) : (i64, i64) -> i64
    call @sloth_main__print(%51) : (i64) -> ()
    %52 = call @sloth_rc_release(%45) : (i64) -> i64
    %53 = call @sloth_rc_release(%46) : (i64) -> i64
    %54 = call @sloth_rc_release(%48) : (i64) -> i64
    %55 = call @sloth_rc_release(%51) : (i64) -> i64
    %56 = call @sloth_str_push(%c0_i64, %c448630058099_i64, %c5_i64) : (i64, i64, i64) -> i64
    %57 = call @sloth_str_finish(%56) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %58 = call @sloth_rc_retain(%57) : (i64) -> i64
    memref.store %58, %alloca[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %59 = arith.index_cast %intptr_9 : index to i64
    %60 = call @sloth_fiber_track(%59) : (i64) -> i64
    %61 = call @sloth_rc_release(%57) : (i64) -> i64
    %62 = call @sloth_str_push(%c0_i64, %c2124136_i64, %c3_i64) : (i64, i64, i64) -> i64
    %63 = memref.load %alloca[%c0] : memref<1xi64>
    %64 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %64 : memref<11xi64> -> index
    %65 = arith.index_cast %intptr_10 : index to i64
    %66 = call @sloth_any_from(%65, %63) : (i64, i64) -> i64
    %67 = call @sloth_rt_write(%66) : (i64) -> i64
    %68 = call @sloth_str_pushp(%62, %67) : (i64, i64) -> i64
    %69 = call @sloth_str_push(%68, %c67544357281836_i64, %c6_i64) : (i64, i64, i64) -> i64
    %70 = memref.load %alloca[%c0] : memref<1xi64>
    %71 = call @sloth_str_len(%70) : (i64) -> i64
    %72 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %72 : memref<11xi64> -> index
    %73 = arith.index_cast %intptr_11 : index to i64
    %74 = call @sloth_any_from(%73, %71) : (i64, i64) -> i64
    %75 = call @sloth_rt_write(%74) : (i64) -> i64
    %76 = call @sloth_str_pushp(%69, %75) : (i64, i64) -> i64
    %77 = call @sloth_str_finish(%76) : (i64) -> i64
    %78 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %78 : memref<11xi64> -> index
    %79 = arith.index_cast %intptr_12 : index to i64
    %80 = call @sloth_any_from(%79, %77) : (i64, i64) -> i64
    call @sloth_main__print(%80) : (i64) -> ()
    %81 = call @sloth_rc_release(%66) : (i64) -> i64
    %82 = call @sloth_rc_release(%67) : (i64) -> i64
    %83 = call @sloth_rc_release(%74) : (i64) -> i64
    %84 = call @sloth_rc_release(%75) : (i64) -> i64
    %85 = call @sloth_rc_release(%77) : (i64) -> i64
    %86 = call @sloth_rc_release(%80) : (i64) -> i64
    %87 = memref.load %alloca[%c0] : memref<1xi64>
    %88 = call @sloth_rc_release(%87) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %89 = arith.index_cast %intptr_13 : index to i64
    %90 = call @sloth_fiber_untrack(%89) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c1099511627779_i64 = arith.constant 1099511627779 : i64
    %c4_i64 = arith.constant 4 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %c1_i64 = arith.constant 1 : i64
    %c5_i64 = arith.constant 5 : i64
    %2 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %3 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %4 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c2_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %3 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    %6 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c3_i64, %6[%c0] : memref<11xi64>
    memref.store %c0_i64, %6[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %6[%c2] : memref<11xi64>
    %7 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %7, %6[%c3] : memref<11xi64>
    memref.store %c5_i64, %6[%c4] : memref<11xi64>
    memref.store %c0_i64, %6[%c9] : memref<11xi64>
    memref.store %c0_i64, %6[%c10] : memref<11xi64>
    %8 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    memref.store %c1_i64, %8[%c0] : memref<11xi64>
    memref.store %c0_i64, %8[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %8[%c2] : memref<11xi64>
    %9 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %9, %8[%c3] : memref<11xi64>
    memref.store %c4_i64, %8[%c4] : memref<11xi64>
    memref.store %c0_i64, %8[%c9] : memref<11xi64>
    memref.store %c0_i64, %8[%c10] : memref<11xi64>
    %10 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    memref.store %c4_i64, %10[%c0] : memref<11xi64>
    memref.store %c1_i64, %10[%c1] : memref<11xi64>
    memref.store %c1099511627779_i64, %10[%c2] : memref<11xi64>
    %11 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %11, %10[%c3] : memref<11xi64>
    memref.store %c3_i64, %10[%c4] : memref<11xi64>
    memref.store %c0_i64, %10[%c9] : memref<11xi64>
    memref.store %c0_i64, %10[%c10] : memref<11xi64>
    return
  }
}

