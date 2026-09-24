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
    %1 = call @__sloth_rt_write(%0) : (i64) -> i64
    call @__sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @__sloth_rc_release(%1) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %cst = arith.constant 3.000000e+00 : f64
    %c0 = arith.constant 0 : index
    %c107_i64 = arith.constant 107 : i64
    %c4615964438073389875_i64 = arith.constant 4615964438073389875 : i64
    %c2_i64 = arith.constant 2 : i64
    %c3_i64 = arith.constant 3 : i64
    %c6513249_i64 = arith.constant 6513249 : i64
    %c115_i64 = arith.constant 115 : i64
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c4612811918334230528_i64 = arith.constant 4612811918334230528 : i64
    %c42_i64 = arith.constant 42 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %0 : memref<11xi64> -> index
    %1 = arith.index_cast %intptr : index to i64
    %2 = call @__sloth_any_from(%1, %c42_i64) : (i64, i64) -> i64
    call @sloth_main__print(%2) : (i64) -> ()
    %3 = call @__sloth_rc_release(%2) : (i64) -> i64
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %4 : memref<11xi64> -> index
    %5 = arith.index_cast %intptr_0 : index to i64
    %6 = call @__sloth_any_from(%5, %c4612811918334230528_i64) : (i64, i64) -> i64
    call @sloth_main__print(%6) : (i64) -> ()
    %7 = call @__sloth_rc_release(%6) : (i64) -> i64
    %8 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_1 : index to i64
    %10 = call @__sloth_any_from(%9, %c1_i64) : (i64, i64) -> i64
    call @sloth_main__print(%10) : (i64) -> ()
    %11 = call @__sloth_rc_release(%10) : (i64) -> i64
    %12 = call @__sloth_str_push(%c0_i64, %c115_i64, %c1_i64) : (i64, i64, i64) -> i64
    %13 = call @__sloth_str_finish(%12) : (i64) -> i64
    %14 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %14 : memref<11xi64> -> index
    %15 = arith.index_cast %intptr_2 : index to i64
    %16 = call @__sloth_any_from(%15, %13) : (i64, i64) -> i64
    call @sloth_main__print(%16) : (i64) -> ()
    %17 = call @__sloth_rc_release(%13) : (i64) -> i64
    %18 = call @__sloth_rc_release(%16) : (i64) -> i64
    %19 = call @__sloth_str_push(%c0_i64, %c6513249_i64, %c3_i64) : (i64, i64, i64) -> i64
    %20 = call @__sloth_str_finish(%19) : (i64) -> i64
    %21 = call @__sloth_str_len(%20) : (i64) -> i64
    %22 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %22 : memref<11xi64> -> index
    %23 = arith.index_cast %intptr_3 : index to i64
    %24 = call @__sloth_any_from(%23, %21) : (i64, i64) -> i64
    call @sloth_main__print(%24) : (i64) -> ()
    %25 = call @__sloth_rc_release(%20) : (i64) -> i64
    %26 = call @__sloth_rc_release(%24) : (i64) -> i64
    %27 = call @__sloth_arr_new(%c3_i64) : (i64) -> i64
    %28 = call @__sloth_arr_set(%27, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %29 = call @__sloth_arr_set(%27, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %30 = call @__sloth_arr_set(%27, %c2_i64, %c3_i64) : (i64, i64, i64) -> i64
    %31 = call @__sloth_arr_len(%27) : (i64) -> i64
    %32 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %32 : memref<11xi64> -> index
    %33 = arith.index_cast %intptr_4 : index to i64
    %34 = call @__sloth_any_from(%33, %31) : (i64, i64) -> i64
    call @sloth_main__print(%34) : (i64) -> ()
    %35 = call @__sloth_rc_release(%27) : (i64) -> i64
    %36 = call @__sloth_rc_release(%34) : (i64) -> i64
    %37 = call @__sloth_map_new(%c0_i64) : (i64) -> i64
    %38 = call @__sloth_map_set(%37, %c1_i64, %c1_i64) : (i64, i64, i64) -> i64
    %39 = call @__sloth_map_len(%37) : (i64) -> i64
    %40 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %40 : memref<11xi64> -> index
    %41 = arith.index_cast %intptr_5 : index to i64
    %42 = call @__sloth_any_from(%41, %39) : (i64, i64) -> i64
    call @sloth_main__print(%42) : (i64) -> ()
    %43 = call @__sloth_rc_release(%37) : (i64) -> i64
    %44 = call @__sloth_rc_release(%42) : (i64) -> i64
    %45 = llvm.bitcast %c4615964438073389875_i64 : i64 to f64
    %46 = arith.fptosi %45 : f64 to i64
    %47 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %47 : memref<11xi64> -> index
    %48 = arith.index_cast %intptr_6 : index to i64
    %49 = call @__sloth_any_from(%48, %46) : (i64, i64) -> i64
    call @sloth_main__print(%49) : (i64) -> ()
    %50 = call @__sloth_rc_release(%49) : (i64) -> i64
    %51 = llvm.bitcast %cst : f64 to i64
    %52 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %52 : memref<11xi64> -> index
    %53 = arith.index_cast %intptr_7 : index to i64
    %54 = call @__sloth_any_from(%53, %51) : (i64, i64) -> i64
    call @sloth_main__print(%54) : (i64) -> ()
    %55 = call @__sloth_rc_release(%54) : (i64) -> i64
    %56 = call @__sloth_str_push(%c0_i64, %c107_i64, %c1_i64) : (i64, i64, i64) -> i64
    %57 = call @__sloth_str_finish(%56) : (i64) -> i64
    %58 = call @__sloth_map_new(%c1_i64) : (i64) -> i64
    %59 = call @__sloth_rc_retain(%57) : (i64) -> i64
    %60 = call @__sloth_map_str_set(%58, %57, %c1_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %61 = call @__sloth_rc_retain(%58) : (i64) -> i64
    memref.store %61, %alloca[%c0] : memref<1xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %62 = arith.index_cast %intptr_8 : index to i64
    %63 = call @__sloth_fiber_track(%62) : (i64) -> i64
    %64 = call @__sloth_rc_release(%57) : (i64) -> i64
    %65 = call @__sloth_rc_release(%58) : (i64) -> i64
    %66 = memref.load %alloca[%c0] : memref<1xi64>
    %67 = call @__sloth_map_keys(%66) : (i64) -> i64
    %68 = call @__sloth_arr_len(%67) : (i64) -> i64
    %69 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %69 : memref<11xi64> -> index
    %70 = arith.index_cast %intptr_9 : index to i64
    %71 = call @__sloth_any_from(%70, %68) : (i64, i64) -> i64
    call @sloth_main__print(%71) : (i64) -> ()
    %72 = call @__sloth_rc_release(%67) : (i64) -> i64
    %73 = call @__sloth_rc_release(%71) : (i64) -> i64
    %74 = memref.load %alloca[%c0] : memref<1xi64>
    %75 = call @__sloth_map_values(%74) : (i64) -> i64
    %76 = call @__sloth_arr_len(%75) : (i64) -> i64
    %77 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %77 : memref<11xi64> -> index
    %78 = arith.index_cast %intptr_10 : index to i64
    %79 = call @__sloth_any_from(%78, %76) : (i64, i64) -> i64
    call @sloth_main__print(%79) : (i64) -> ()
    %80 = call @__sloth_rc_release(%75) : (i64) -> i64
    %81 = call @__sloth_rc_release(%79) : (i64) -> i64
    %82 = call @__sloth_rc_live() : () -> i64
    %83 = arith.cmpi sge, %82, %c0_i64 : i64
    %84 = arith.extui %83 : i1 to i64
    %85 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %85 : memref<11xi64> -> index
    %86 = arith.index_cast %intptr_11 : index to i64
    %87 = call @__sloth_any_from(%86, %84) : (i64, i64) -> i64
    call @sloth_main__print(%87) : (i64) -> ()
    %88 = call @__sloth_rc_release(%87) : (i64) -> i64
    %89 = memref.load %alloca[%c0] : memref<1xi64>
    %90 = call @__sloth_rc_release(%89) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %91 = arith.index_cast %intptr_12 : index to i64
    %92 = call @__sloth_fiber_untrack(%91) : (i64) -> i64
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

