module @main {
  llvm.mlir.global private constant @sloth_tynm_0("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("float\00") {addr_space = 0 : i32}
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
    %c4022816_i64 = arith.constant 4022816 : i64
    %c4023840_i64 = arith.constant 4023840 : i64
    %c15726_i64 = arith.constant 15726 : i64
    %c4609434218613702656_i64 = arith.constant 4609434218613702656 : i64
    %c122_i64 = arith.constant 122 : i64
    %c121_i64 = arith.constant 121 : i64
    %c120_i64 = arith.constant 120 : i64
    %c191009621918561_i64 = arith.constant 191009621918561 : i64
    %c98_i64 = arith.constant 98 : i64
    %c1_i64 = arith.constant 1 : i64
    %c97_i64 = arith.constant 97 : i64
    %c2_i64 = arith.constant 2 : i64
    %c28524_i64 = arith.constant 28524 : i64
    %c3_i64 = arith.constant 3 : i64
    %c7103848_i64 = arith.constant 7103848 : i64
    %c6_i64 = arith.constant 6 : i64
    %c122511470216040_i64 = arith.constant 122511470216040 : i64
    %c7_i64 = arith.constant 7 : i64
    %c28266736423215148_i64 = arith.constant 28266736423215148 : i64
    %c0 = arith.constant 0 : index
    %c5_i64 = arith.constant 5 : i64
    %c478560413032_i64 = arith.constant 478560413032 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = call @sloth_str_push(%c0_i64, %c478560413032_i64, %c5_i64) : (i64, i64, i64) -> i64
    %1 = call @sloth_str_finish(%0) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %2 = call @sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @sloth_fiber_track(%3) : (i64) -> i64
    %5 = call @sloth_rc_release(%1) : (i64) -> i64
    %6 = memref.load %alloca[%c0] : memref<1xi64>
    %7 = call @sloth_str_push(%c0_i64, %c28266736423215148_i64, %c7_i64) : (i64, i64, i64) -> i64
    %8 = call @sloth_str_finish(%7) : (i64) -> i64
    %9 = call @sloth_str_concat(%6, %8) : (i64, i64) -> i64
    %10 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %10 : memref<11xi64> -> index
    %11 = arith.index_cast %intptr_0 : index to i64
    %12 = call @sloth_any_from(%11, %9) : (i64, i64) -> i64
    call @sloth_main__print(%12) : (i64) -> ()
    %13 = call @sloth_rc_release(%8) : (i64) -> i64
    %14 = call @sloth_rc_release(%9) : (i64) -> i64
    %15 = call @sloth_rc_release(%12) : (i64) -> i64
    %16 = memref.load %alloca[%c0] : memref<1xi64>
    %17 = call @sloth_str_len(%16) : (i64) -> i64
    %18 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %18 : memref<11xi64> -> index
    %19 = arith.index_cast %intptr_1 : index to i64
    %20 = call @sloth_any_from(%19, %17) : (i64, i64) -> i64
    call @sloth_main__print(%20) : (i64) -> ()
    %21 = call @sloth_rc_release(%20) : (i64) -> i64
    %22 = call @sloth_str_push(%c0_i64, %c122511470216040_i64, %c6_i64) : (i64, i64, i64) -> i64
    %23 = call @sloth_str_finish(%22) : (i64) -> i64
    %24 = call @sloth_str_len(%23) : (i64) -> i64
    %25 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %25 : memref<11xi64> -> index
    %26 = arith.index_cast %intptr_2 : index to i64
    %27 = call @sloth_any_from(%26, %24) : (i64, i64) -> i64
    call @sloth_main__print(%27) : (i64) -> ()
    %28 = call @sloth_rc_release(%23) : (i64) -> i64
    %29 = call @sloth_rc_release(%27) : (i64) -> i64
    %30 = memref.load %alloca[%c0] : memref<1xi64>
    %31 = call @sloth_str_push(%c0_i64, %c7103848_i64, %c3_i64) : (i64, i64, i64) -> i64
    %32 = call @sloth_str_finish(%31) : (i64) -> i64
    %33 = call @sloth_str_push(%c0_i64, %c28524_i64, %c2_i64) : (i64, i64, i64) -> i64
    %34 = call @sloth_str_finish(%33) : (i64) -> i64
    %35 = call @sloth_str_concat(%32, %34) : (i64, i64) -> i64
    %36 = call @sloth_str_eq(%30, %35) : (i64, i64) -> i64
    %37 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %37 : memref<11xi64> -> index
    %38 = arith.index_cast %intptr_3 : index to i64
    %39 = call @sloth_any_from(%38, %36) : (i64, i64) -> i64
    call @sloth_main__print(%39) : (i64) -> ()
    %40 = call @sloth_rc_release(%32) : (i64) -> i64
    %41 = call @sloth_rc_release(%34) : (i64) -> i64
    %42 = call @sloth_rc_release(%35) : (i64) -> i64
    %43 = call @sloth_rc_release(%39) : (i64) -> i64
    %44 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %45 = call @sloth_str_finish(%44) : (i64) -> i64
    %46 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %47 = call @sloth_str_finish(%46) : (i64) -> i64
    %48 = call @sloth_str_eq(%45, %47) : (i64, i64) -> i64
    %49 = arith.xori %48, %c1_i64 : i64
    %50 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %50 : memref<11xi64> -> index
    %51 = arith.index_cast %intptr_4 : index to i64
    %52 = call @sloth_any_from(%51, %49) : (i64, i64) -> i64
    call @sloth_main__print(%52) : (i64) -> ()
    %53 = call @sloth_rc_release(%45) : (i64) -> i64
    %54 = call @sloth_rc_release(%47) : (i64) -> i64
    %55 = call @sloth_rc_release(%52) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    %56 = call @sloth_str_push(%c0_i64, %c191009621918561_i64, %c6_i64) : (i64, i64, i64) -> i64
    %57 = call @sloth_str_finish(%56) : (i64) -> i64
    %58 = call @sloth_str_clen(%57) : (i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %59 = memref.load %alloca_6[%c0] : memref<1xi64>
    %60 = arith.cmpi slt, %59, %58 : i64
    cf.cond_br %60, ^bb2, ^bb4
  ^bb2:  // pred: ^bb1
    %61 = call @sloth_str_char(%57, %59) : (i64, i64) -> i64
    %alloca_7 = memref.alloca() : memref<1xi64>
    memref.store %61, %alloca_7[%c0] : memref<1xi64>
    %62 = memref.load %alloca_5[%c0] : memref<1xi64>
    %63 = arith.addi %62, %c1_i64 : i64
    memref.store %63, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %64 = call @sloth_rc_release(%61) : (i64) -> i64
    %65 = arith.addi %59, %c1_i64 : i64
    memref.store %65, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %66 = call @sloth_rc_release(%57) : (i64) -> i64
    %67 = memref.load %alloca_5[%c0] : memref<1xi64>
    %68 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %68 : memref<11xi64> -> index
    %69 = arith.index_cast %intptr_8 : index to i64
    %70 = call @sloth_any_from(%69, %67) : (i64, i64) -> i64
    call @sloth_main__print(%70) : (i64) -> ()
    %71 = call @sloth_rc_release(%70) : (i64) -> i64
    %72 = call @sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %73 = call @sloth_str_finish(%72) : (i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    %74 = call @sloth_rc_retain(%73) : (i64) -> i64
    memref.store %74, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %75 = arith.index_cast %intptr_10 : index to i64
    %76 = call @sloth_fiber_track(%75) : (i64) -> i64
    %77 = call @sloth_rc_release(%73) : (i64) -> i64
    %78 = memref.load %alloca_9[%c0] : memref<1xi64>
    %79 = call @sloth_str_push(%c0_i64, %c121_i64, %c1_i64) : (i64, i64, i64) -> i64
    %80 = call @sloth_str_finish(%79) : (i64) -> i64
    %81 = call @sloth_str_concat(%78, %80) : (i64, i64) -> i64
    %82 = call @sloth_str_push(%c0_i64, %c122_i64, %c1_i64) : (i64, i64, i64) -> i64
    %83 = call @sloth_str_finish(%82) : (i64) -> i64
    %84 = call @sloth_str_concat(%81, %83) : (i64, i64) -> i64
    %85 = memref.load %alloca_9[%c0] : memref<1xi64>
    %86 = call @sloth_rc_release(%85) : (i64) -> i64
    %87 = call @sloth_rc_retain(%84) : (i64) -> i64
    memref.store %87, %alloca_9[%c0] : memref<1xi64>
    %88 = call @sloth_rc_release(%80) : (i64) -> i64
    %89 = call @sloth_rc_release(%81) : (i64) -> i64
    %90 = call @sloth_rc_release(%83) : (i64) -> i64
    %91 = call @sloth_rc_release(%84) : (i64) -> i64
    %92 = memref.load %alloca_9[%c0] : memref<1xi64>
    %93 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %93 : memref<11xi64> -> index
    %94 = arith.index_cast %intptr_11 : index to i64
    %95 = call @sloth_any_from(%94, %92) : (i64, i64) -> i64
    call @sloth_main__print(%95) : (i64) -> ()
    %96 = call @sloth_rc_release(%95) : (i64) -> i64
    %alloca_12 = memref.alloca() : memref<1xi64>
    memref.store %c4609434218613702656_i64, %alloca_12[%c0] : memref<1xi64>
    %97 = call @sloth_str_push(%c0_i64, %c15726_i64, %c2_i64) : (i64, i64, i64) -> i64
    %98 = memref.load %alloca_5[%c0] : memref<1xi64>
    %99 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %99 : memref<11xi64> -> index
    %100 = arith.index_cast %intptr_13 : index to i64
    %101 = call @sloth_any_from(%100, %98) : (i64, i64) -> i64
    %102 = call @sloth_rt_write(%101) : (i64) -> i64
    %103 = call @sloth_str_pushp(%97, %102) : (i64, i64) -> i64
    %104 = call @sloth_str_push(%103, %c4023840_i64, %c3_i64) : (i64, i64, i64) -> i64
    %105 = memref.load %alloca_12[%c0] : memref<1xi64>
    %106 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_14 = memref.extract_aligned_pointer_as_index %106 : memref<11xi64> -> index
    %107 = arith.index_cast %intptr_14 : index to i64
    %108 = call @sloth_any_from(%107, %105) : (i64, i64) -> i64
    %109 = call @sloth_rt_write(%108) : (i64) -> i64
    %110 = call @sloth_str_pushp(%104, %109) : (i64, i64) -> i64
    %111 = call @sloth_str_push(%110, %c4022816_i64, %c3_i64) : (i64, i64, i64) -> i64
    %112 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_15 = memref.extract_aligned_pointer_as_index %112 : memref<11xi64> -> index
    %113 = arith.index_cast %intptr_15 : index to i64
    %114 = call @sloth_any_from(%113, %c1_i64) : (i64, i64) -> i64
    %115 = call @sloth_rt_write(%114) : (i64) -> i64
    %116 = call @sloth_str_pushp(%111, %115) : (i64, i64) -> i64
    %117 = call @sloth_str_finish(%116) : (i64) -> i64
    %118 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_16 = memref.extract_aligned_pointer_as_index %118 : memref<11xi64> -> index
    %119 = arith.index_cast %intptr_16 : index to i64
    %120 = call @sloth_any_from(%119, %117) : (i64, i64) -> i64
    call @sloth_main__print(%120) : (i64) -> ()
    %121 = call @sloth_rc_release(%101) : (i64) -> i64
    %122 = call @sloth_rc_release(%102) : (i64) -> i64
    %123 = call @sloth_rc_release(%108) : (i64) -> i64
    %124 = call @sloth_rc_release(%109) : (i64) -> i64
    %125 = call @sloth_rc_release(%114) : (i64) -> i64
    %126 = call @sloth_rc_release(%115) : (i64) -> i64
    %127 = call @sloth_rc_release(%117) : (i64) -> i64
    %128 = call @sloth_rc_release(%120) : (i64) -> i64
    %129 = memref.load %alloca_9[%c0] : memref<1xi64>
    %130 = call @sloth_rc_release(%129) : (i64) -> i64
    %intptr_17 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %131 = arith.index_cast %intptr_17 : index to i64
    %132 = call @sloth_fiber_untrack(%131) : (i64) -> i64
    %133 = memref.load %alloca[%c0] : memref<1xi64>
    %134 = call @sloth_rc_release(%133) : (i64) -> i64
    %intptr_18 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %135 = arith.index_cast %intptr_18 : index to i64
    %136 = call @sloth_fiber_untrack(%135) : (i64) -> i64
    cf.br ^bb5
  ^bb5:  // pred: ^bb4
    return
  }
  func.func @sloth_main__anyinit() {
    %c5_i64 = arith.constant 5 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c1099511627779_i64 = arith.constant 1099511627779 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %2 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %3 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %4 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c4_i64, %4[%c0] : memref<11xi64>
    memref.store %c1_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %3 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    %6 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c2_i64, %6[%c0] : memref<11xi64>
    memref.store %c0_i64, %6[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %6[%c2] : memref<11xi64>
    %7 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %7, %6[%c3] : memref<11xi64>
    memref.store %c3_i64, %6[%c4] : memref<11xi64>
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
    memref.store %c3_i64, %10[%c0] : memref<11xi64>
    memref.store %c0_i64, %10[%c1] : memref<11xi64>
    memref.store %c1099511627779_i64, %10[%c2] : memref<11xi64>
    %11 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %11, %10[%c3] : memref<11xi64>
    memref.store %c5_i64, %10[%c4] : memref<11xi64>
    memref.store %c0_i64, %10[%c9] : memref<11xi64>
    memref.store %c0_i64, %10[%c10] : memref<11xi64>
    return
  }
}

