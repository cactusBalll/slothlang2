module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("float\00") {addr_space = 0 : i32}
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
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c4609434218613702656_i64 = arith.constant 4609434218613702656 : i64
    %c9_i64 = arith.constant 9 : i64
    %c99_i64 = arith.constant 99 : i64
    %c10_i64 = arith.constant 10 : i64
    %c4_i64 = arith.constant 4 : i64
    %c0 = arith.constant 0 : index
    %c0_i64 = arith.constant 0 : i64
    %c3_i64 = arith.constant 3 : i64
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = call @sloth_arr_new(%c3_i64) : (i64) -> i64
    %1 = call @sloth_arr_set(%0, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %2 = call @sloth_arr_set(%0, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %3 = call @sloth_arr_set(%0, %c2_i64, %c3_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %4 = call @sloth_rc_retain(%0) : (i64) -> i64
    memref.store %4, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %5 = arith.index_cast %intptr : index to i64
    %6 = call @sloth_fiber_track(%5) : (i64) -> i64
    %7 = call @sloth_rc_release(%0) : (i64) -> i64
    %8 = memref.load %alloca[%c0] : memref<1xi64>
    %9 = call @sloth_arr_len(%8) : (i64) -> i64
    %10 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %10 : memref<11xi64> -> index
    %11 = arith.index_cast %intptr_0 : index to i64
    %12 = call @sloth_any_from(%11, %9) : (i64, i64) -> i64
    call @sloth_main__print(%12) : (i64) -> ()
    %13 = call @sloth_rc_release(%12) : (i64) -> i64
    %14 = memref.load %alloca[%c0] : memref<1xi64>
    %15 = call @sloth_arr_push(%14, %c4_i64) : (i64, i64) -> i64
    memref.store %15, %alloca[%c0] : memref<1xi64>
    %16 = memref.load %alloca[%c0] : memref<1xi64>
    %17 = call @sloth_arr_len(%16) : (i64) -> i64
    %18 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %18 : memref<11xi64> -> index
    %19 = arith.index_cast %intptr_1 : index to i64
    %20 = call @sloth_any_from(%19, %17) : (i64, i64) -> i64
    call @sloth_main__print(%20) : (i64) -> ()
    %21 = call @sloth_rc_release(%20) : (i64) -> i64
    %22 = memref.load %alloca[%c0] : memref<1xi64>
    %23 = call @sloth_arr_pop(%22) : (i64) -> i64
    %24 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %24 : memref<11xi64> -> index
    %25 = arith.index_cast %intptr_2 : index to i64
    %26 = call @sloth_any_from(%25, %23) : (i64, i64) -> i64
    call @sloth_main__print(%26) : (i64) -> ()
    %27 = call @sloth_rc_release(%26) : (i64) -> i64
    %28 = memref.load %alloca[%c0] : memref<1xi64>
    %29 = call @sloth_arr_set(%28, %c0_i64, %c10_i64) : (i64, i64, i64) -> i64
    %30 = memref.load %alloca[%c0] : memref<1xi64>
    %31 = call @sloth_arr_get(%30, %c0_i64) : (i64, i64) -> i64
    %32 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %32 : memref<11xi64> -> index
    %33 = arith.index_cast %intptr_3 : index to i64
    %34 = call @sloth_any_from(%33, %31) : (i64, i64) -> i64
    call @sloth_main__print(%34) : (i64) -> ()
    %35 = call @sloth_rc_release(%34) : (i64) -> i64
    %alloca_4 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_4[%c0] : memref<1xi64>
    %36 = memref.load %alloca[%c0] : memref<1xi64>
    %37 = call @sloth_arr_len(%36) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %38 = memref.load %alloca_5[%c0] : memref<1xi64>
    %39 = arith.cmpi slt, %38, %37 : i64
    cf.cond_br %39, ^bb2, ^bb4
  ^bb2:  // pred: ^bb1
    %40 = call @sloth_arr_get(%36, %38) : (i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    memref.store %40, %alloca_6[%c0] : memref<1xi64>
    %41 = memref.load %alloca_4[%c0] : memref<1xi64>
    %42 = memref.load %alloca_6[%c0] : memref<1xi64>
    %43 = arith.addi %41, %42 : i64
    memref.store %43, %alloca_4[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %44 = arith.addi %38, %c1_i64 : i64
    memref.store %44, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %45 = memref.load %alloca_4[%c0] : memref<1xi64>
    %46 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %46 : memref<11xi64> -> index
    %47 = arith.index_cast %intptr_7 : index to i64
    %48 = call @sloth_any_from(%47, %45) : (i64, i64) -> i64
    call @sloth_main__print(%48) : (i64) -> ()
    %49 = call @sloth_rc_release(%48) : (i64) -> i64
    %50 = memref.load %alloca[%c0] : memref<1xi64>
    %alloca_8 = memref.alloca() : memref<1xi64>
    %51 = call @sloth_rc_retain(%50) : (i64) -> i64
    memref.store %51, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %52 = arith.index_cast %intptr_9 : index to i64
    %53 = call @sloth_fiber_track(%52) : (i64) -> i64
    %54 = memref.load %alloca_8[%c0] : memref<1xi64>
    %55 = call @sloth_arr_push(%54, %c99_i64) : (i64, i64) -> i64
    memref.store %55, %alloca_8[%c0] : memref<1xi64>
    %56 = memref.load %alloca[%c0] : memref<1xi64>
    %57 = call @sloth_arr_len(%56) : (i64) -> i64
    %58 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %58 : memref<11xi64> -> index
    %59 = arith.index_cast %intptr_10 : index to i64
    %60 = call @sloth_any_from(%59, %57) : (i64, i64) -> i64
    call @sloth_main__print(%60) : (i64) -> ()
    %61 = call @sloth_rc_release(%60) : (i64) -> i64
    %62 = memref.load %alloca[%c0] : memref<1xi64>
    %63 = call @sloth_arr_get(%62, %c3_i64) : (i64, i64) -> i64
    %64 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %64 : memref<11xi64> -> index
    %65 = arith.index_cast %intptr_11 : index to i64
    %66 = call @sloth_any_from(%65, %63) : (i64, i64) -> i64
    call @sloth_main__print(%66) : (i64) -> ()
    %67 = call @sloth_rc_release(%66) : (i64) -> i64
    %68 = call @sloth_arr_new(%c2_i64) : (i64) -> i64
    %69 = call @sloth_arr_set(%68, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %70 = call @sloth_arr_set(%68, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %71 = call @sloth_arr_new(%c2_i64) : (i64) -> i64
    %72 = call @sloth_arr_set(%71, %c0_i64, %c3_i64) : (i64, i64, i64) -> i64
    %73 = call @sloth_arr_set(%71, %c1_i64, %c4_i64) : (i64, i64, i64) -> i64
    %74 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %75 = call @sloth_rc_retain(%68) : (i64) -> i64
    %76 = call @sloth_arr_set(%74, %c0_i64, %75) : (i64, i64, i64) -> i64
    %77 = call @sloth_rc_retain(%71) : (i64) -> i64
    %78 = call @sloth_arr_set(%74, %c1_i64, %77) : (i64, i64, i64) -> i64
    %alloca_12 = memref.alloca() : memref<1xi64>
    %79 = call @sloth_rc_retain(%74) : (i64) -> i64
    memref.store %79, %alloca_12[%c0] : memref<1xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_12 : memref<1xi64> -> index
    %80 = arith.index_cast %intptr_13 : index to i64
    %81 = call @sloth_fiber_track(%80) : (i64) -> i64
    %82 = call @sloth_rc_release(%68) : (i64) -> i64
    %83 = call @sloth_rc_release(%71) : (i64) -> i64
    %84 = call @sloth_rc_release(%74) : (i64) -> i64
    %85 = memref.load %alloca_12[%c0] : memref<1xi64>
    %86 = call @sloth_arr_get(%85, %c0_i64) : (i64, i64) -> i64
    %87 = call @sloth_arr_set(%86, %c1_i64, %c9_i64) : (i64, i64, i64) -> i64
    %88 = memref.load %alloca_12[%c0] : memref<1xi64>
    %89 = call @sloth_arr_get(%88, %c0_i64) : (i64, i64) -> i64
    %90 = call @sloth_arr_get(%89, %c1_i64) : (i64, i64) -> i64
    %91 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_14 = memref.extract_aligned_pointer_as_index %91 : memref<11xi64> -> index
    %92 = arith.index_cast %intptr_14 : index to i64
    %93 = call @sloth_any_from(%92, %90) : (i64, i64) -> i64
    call @sloth_main__print(%93) : (i64) -> ()
    %94 = call @sloth_rc_release(%93) : (i64) -> i64
    %95 = call @sloth_arr_new(%c0_i64) : (i64) -> i64
    %alloca_15 = memref.alloca() : memref<1xi64>
    %96 = call @sloth_rc_retain(%95) : (i64) -> i64
    memref.store %96, %alloca_15[%c0] : memref<1xi64>
    %intptr_16 = memref.extract_aligned_pointer_as_index %alloca_15 : memref<1xi64> -> index
    %97 = arith.index_cast %intptr_16 : index to i64
    %98 = call @sloth_fiber_track(%97) : (i64) -> i64
    %99 = call @sloth_rc_release(%95) : (i64) -> i64
    %100 = memref.load %alloca_15[%c0] : memref<1xi64>
    %101 = call @sloth_arr_push(%100, %c4609434218613702656_i64) : (i64, i64) -> i64
    memref.store %101, %alloca_15[%c0] : memref<1xi64>
    %102 = memref.load %alloca_15[%c0] : memref<1xi64>
    %103 = call @sloth_arr_get(%102, %c0_i64) : (i64, i64) -> i64
    %104 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_17 = memref.extract_aligned_pointer_as_index %104 : memref<11xi64> -> index
    %105 = arith.index_cast %intptr_17 : index to i64
    %106 = call @sloth_any_from(%105, %103) : (i64, i64) -> i64
    call @sloth_main__print(%106) : (i64) -> ()
    %107 = call @sloth_rc_release(%106) : (i64) -> i64
    %108 = memref.load %alloca_15[%c0] : memref<1xi64>
    %109 = call @sloth_rc_release(%108) : (i64) -> i64
    %intptr_18 = memref.extract_aligned_pointer_as_index %alloca_15 : memref<1xi64> -> index
    %110 = arith.index_cast %intptr_18 : index to i64
    %111 = call @sloth_fiber_untrack(%110) : (i64) -> i64
    %112 = memref.load %alloca_8[%c0] : memref<1xi64>
    %113 = call @sloth_rc_release(%112) : (i64) -> i64
    %intptr_19 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %114 = arith.index_cast %intptr_19 : index to i64
    %115 = call @sloth_fiber_untrack(%114) : (i64) -> i64
    %116 = memref.load %alloca_12[%c0] : memref<1xi64>
    %117 = call @sloth_rc_release(%116) : (i64) -> i64
    %intptr_20 = memref.extract_aligned_pointer_as_index %alloca_12 : memref<1xi64> -> index
    %118 = arith.index_cast %intptr_20 : index to i64
    %119 = call @sloth_fiber_untrack(%118) : (i64) -> i64
    %120 = memref.load %alloca[%c0] : memref<1xi64>
    %121 = call @sloth_rc_release(%120) : (i64) -> i64
    %intptr_21 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %122 = arith.index_cast %intptr_21 : index to i64
    %123 = call @sloth_fiber_untrack(%122) : (i64) -> i64
    cf.br ^bb5
  ^bb5:  // pred: ^bb4
    return
  }
  func.func @sloth_main__anyinit() {
    %c5_i64 = arith.constant 5 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
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
    memref.store %c3_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c5_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

