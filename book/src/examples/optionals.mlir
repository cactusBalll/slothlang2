module @main {
  llvm.mlir.global private constant @sloth_tynm_0("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_2 : memref<11xi64> = dense<0> {mutable}
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
  func.func @sloth_main__pick(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c7_i64 = arith.constant 7 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_box_get(%0) : (i64) -> i64
    %2 = arith.cmpi ne, %0, %c0_i64 : i64
    %3 = arith.select %2, %1, %c7_i64 : i64
    memref.store %3, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %4 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c2_i64 = arith.constant 2 : i64
    %c26984_i64 = arith.constant 26984 : i64
    %c3_i64 = arith.constant 3 : i64
    %c4_i64 = arith.constant 4 : i64
    %c1701736302_i64 = arith.constant 1701736302 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %0 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %0, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %1 = arith.index_cast %intptr : index to i64
    %2 = call @sloth_fiber_track(%1) : (i64) -> i64
    %3 = memref.load %alloca[%c0] : memref<1xi64>
    %4 = arith.cmpi eq, %3, %c0_i64 : i64
    %5 = arith.extui %4 : i1 to i64
    %6 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %6 : memref<11xi64> -> index
    %7 = arith.index_cast %intptr_0 : index to i64
    %8 = call @sloth_any_from(%7, %5) : (i64, i64) -> i64
    call @sloth_main__print(%8) : (i64) -> ()
    %9 = call @sloth_rc_release(%8) : (i64) -> i64
    %10 = call @sloth_box_new(%c0_i64) : (i64) -> i64
    %11 = memref.load %alloca[%c0] : memref<1xi64>
    %12 = call @sloth_rc_release(%11) : (i64) -> i64
    %13 = call @sloth_rc_retain(%10) : (i64) -> i64
    memref.store %13, %alloca[%c0] : memref<1xi64>
    %14 = call @sloth_rc_release(%10) : (i64) -> i64
    %15 = memref.load %alloca[%c0] : memref<1xi64>
    %16 = arith.cmpi eq, %15, %c0_i64 : i64
    %17 = arith.extui %16 : i1 to i64
    %18 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %18 : memref<11xi64> -> index
    %19 = arith.index_cast %intptr_1 : index to i64
    %20 = call @sloth_any_from(%19, %17) : (i64, i64) -> i64
    call @sloth_main__print(%20) : (i64) -> ()
    %21 = call @sloth_rc_release(%20) : (i64) -> i64
    %22 = memref.load %alloca[%c0] : memref<1xi64>
    %23 = arith.cmpi eq, %22, %c0_i64 : i64
    %24 = arith.extui %23 : i1 to i64
    %25 = arith.xori %24, %c1_i64 : i64
    %26 = arith.cmpi ne, %25, %c0_i64 : i64
    cf.cond_br %26, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %27 = memref.load %alloca[%c0] : memref<1xi64>
    %28 = call @sloth_box_get(%27) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %28, %alloca_2[%c0] : memref<1xi64>
    %29 = memref.load %alloca_2[%c0] : memref<1xi64>
    %30 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %30 : memref<11xi64> -> index
    %31 = arith.index_cast %intptr_3 : index to i64
    %32 = call @sloth_any_from(%31, %29) : (i64, i64) -> i64
    call @sloth_main__print(%32) : (i64) -> ()
    %33 = call @sloth_rc_release(%32) : (i64) -> i64
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %34 = memref.load %alloca[%c0] : memref<1xi64>
    %35 = arith.cmpi eq, %34, %c0_i64 : i64
    cf.cond_br %35, ^bb4, ^bb5
  ^bb4:  // pred: ^bb3
    %36 = call @sloth_str_push(%c0_i64, %c1701736302_i64, %c4_i64) : (i64, i64, i64) -> i64
    %37 = call @sloth_str_finish(%36) : (i64) -> i64
    %38 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %38 : memref<11xi64> -> index
    %39 = arith.index_cast %intptr_4 : index to i64
    %40 = call @sloth_any_from(%39, %37) : (i64, i64) -> i64
    call @sloth_main__print(%40) : (i64) -> ()
    %41 = call @sloth_rc_release(%37) : (i64) -> i64
    %42 = call @sloth_rc_release(%40) : (i64) -> i64
    cf.br ^bb6
  ^bb5:  // pred: ^bb3
    %43 = memref.load %alloca[%c0] : memref<1xi64>
    %44 = call @sloth_box_get(%43) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %44, %alloca_5[%c0] : memref<1xi64>
    %45 = memref.load %alloca_5[%c0] : memref<1xi64>
    %46 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %46 : memref<11xi64> -> index
    %47 = arith.index_cast %intptr_6 : index to i64
    %48 = call @sloth_any_from(%47, %45) : (i64, i64) -> i64
    call @sloth_main__print(%48) : (i64) -> ()
    %49 = call @sloth_rc_release(%48) : (i64) -> i64
    cf.br ^bb6
  ^bb6:  // 2 preds: ^bb4, ^bb5
    %50 = call @sloth_main__pick(%c0_i64) : (i64) -> i64
    %51 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %51 : memref<11xi64> -> index
    %52 = arith.index_cast %intptr_7 : index to i64
    %53 = call @sloth_any_from(%52, %50) : (i64, i64) -> i64
    call @sloth_main__print(%53) : (i64) -> ()
    %54 = call @sloth_rc_release(%53) : (i64) -> i64
    %55 = call @sloth_box_new(%c3_i64) : (i64) -> i64
    %56 = call @sloth_main__pick(%55) : (i64) -> i64
    %57 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %57 : memref<11xi64> -> index
    %58 = arith.index_cast %intptr_8 : index to i64
    %59 = call @sloth_any_from(%58, %56) : (i64, i64) -> i64
    call @sloth_main__print(%59) : (i64) -> ()
    %60 = call @sloth_rc_release(%55) : (i64) -> i64
    %61 = call @sloth_rc_release(%59) : (i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    %62 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %62, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %63 = arith.index_cast %intptr_10 : index to i64
    %64 = call @sloth_fiber_track(%63) : (i64) -> i64
    %65 = call @sloth_str_push(%c0_i64, %c26984_i64, %c2_i64) : (i64, i64, i64) -> i64
    %66 = call @sloth_str_finish(%65) : (i64) -> i64
    %67 = memref.load %alloca_9[%c0] : memref<1xi64>
    %68 = call @sloth_rc_release(%67) : (i64) -> i64
    %69 = call @sloth_rc_retain(%66) : (i64) -> i64
    memref.store %69, %alloca_9[%c0] : memref<1xi64>
    %70 = call @sloth_rc_release(%66) : (i64) -> i64
    %71 = memref.load %alloca_9[%c0] : memref<1xi64>
    %72 = arith.cmpi eq, %71, %c0_i64 : i64
    %73 = arith.extui %72 : i1 to i64
    %74 = arith.xori %73, %c1_i64 : i64
    %75 = arith.cmpi ne, %74, %c0_i64 : i64
    cf.cond_br %75, ^bb7, ^bb8
  ^bb7:  // pred: ^bb6
    %76 = memref.load %alloca_9[%c0] : memref<1xi64>
    %77 = call @sloth_str_len(%76) : (i64) -> i64
    %78 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %78 : memref<11xi64> -> index
    %79 = arith.index_cast %intptr_11 : index to i64
    %80 = call @sloth_any_from(%79, %77) : (i64, i64) -> i64
    call @sloth_main__print(%80) : (i64) -> ()
    %81 = call @sloth_rc_release(%80) : (i64) -> i64
    cf.br ^bb9
  ^bb8:  // pred: ^bb6
    cf.br ^bb9
  ^bb9:  // 2 preds: ^bb7, ^bb8
    %82 = memref.load %alloca_9[%c0] : memref<1xi64>
    %83 = call @sloth_rc_release(%82) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %84 = arith.index_cast %intptr_12 : index to i64
    %85 = call @sloth_fiber_untrack(%84) : (i64) -> i64
    %86 = memref.load %alloca[%c0] : memref<1xi64>
    %87 = call @sloth_rc_release(%86) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %88 = arith.index_cast %intptr_13 : index to i64
    %89 = call @sloth_fiber_untrack(%88) : (i64) -> i64
    cf.br ^bb10
  ^bb10:  // pred: ^bb9
    return
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %c3_i64 = arith.constant 3 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c4_i64 = arith.constant 4 : i64
    %c3 = arith.constant 3 : index
    %2 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %3 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c1_i64, %3[%c0] : memref<11xi64>
    memref.store %c0_i64, %3[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %3[%c2] : memref<11xi64>
    %4 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %4, %3[%c3] : memref<11xi64>
    memref.store %c4_i64, %3[%c4] : memref<11xi64>
    memref.store %c0_i64, %3[%c9] : memref<11xi64>
    memref.store %c0_i64, %3[%c10] : memref<11xi64>
    %5 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c2_i64, %5[%c0] : memref<11xi64>
    memref.store %c0_i64, %5[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %5[%c2] : memref<11xi64>
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %6, %5[%c3] : memref<11xi64>
    memref.store %c3_i64, %5[%c4] : memref<11xi64>
    memref.store %c0_i64, %5[%c9] : memref<11xi64>
    memref.store %c0_i64, %5[%c10] : memref<11xi64>
    %7 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    memref.store %c4_i64, %7[%c0] : memref<11xi64>
    memref.store %c1_i64, %7[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %7[%c2] : memref<11xi64>
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %8, %7[%c3] : memref<11xi64>
    memref.store %c3_i64, %7[%c4] : memref<11xi64>
    memref.store %c0_i64, %7[%c9] : memref<11xi64>
    memref.store %c0_i64, %7[%c10] : memref<11xi64>
    return
  }
}

