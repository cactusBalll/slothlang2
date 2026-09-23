module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Dog\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("Animal\00") {addr_space = 0 : i32}
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
    %c6_i64 = arith.constant 6 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    %c3_i64 = arith.constant 3 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @sloth_cls_name(%2, %3, %c3_i64) : (i64, i64, i64) -> i64
    %5 = call @sloth_cls_refmask(%2, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_obj_new(%2, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%6, %c3_i64) : (i64, i64) -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %7 = call @sloth_rc_retain(%6) : (i64) -> i64
    memref.store %7, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %8 = arith.index_cast %intptr : index to i64
    %9 = call @sloth_fiber_track(%8) : (i64) -> i64
    %10 = call @sloth_rc_release(%6) : (i64) -> i64
    %11 = memref.load %alloca[%c0] : memref<1xi64>
    %12 = call @sloth_obj_field(%11, %c0_i64) : (i64, i64) -> i64
    %13 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %13 : memref<11xi64> -> index
    %14 = arith.index_cast %intptr_0 : index to i64
    %15 = call @sloth_any_from(%14, %12) : (i64, i64) -> i64
    call @sloth_main__print(%15) : (i64) -> ()
    %16 = call @sloth_rc_release(%15) : (i64) -> i64
    %17 = memref.load %alloca[%c0] : memref<1xi64>
    %18 = call @sloth_main_Dog__who(%17) : (i64) -> i64
    %19 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %19 : memref<11xi64> -> index
    %20 = arith.index_cast %intptr_1 : index to i64
    %21 = call @sloth_any_from(%20, %18) : (i64, i64) -> i64
    call @sloth_main__print(%21) : (i64) -> ()
    %22 = call @sloth_rc_release(%21) : (i64) -> i64
    %23 = call @sloth_rc_release(%18) : (i64) -> i64
    %24 = memref.load %alloca[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    %25 = call @sloth_rc_retain(%24) : (i64) -> i64
    memref.store %25, %alloca_2[%c0] : memref<1xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %26 = arith.index_cast %intptr_3 : index to i64
    %27 = call @sloth_fiber_track(%26) : (i64) -> i64
    %28 = memref.load %alloca_2[%c0] : memref<1xi64>
    %29 = call @sloth_main_Dog__who(%28) : (i64) -> i64
    %30 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %30 : memref<11xi64> -> index
    %31 = arith.index_cast %intptr_4 : index to i64
    %32 = call @sloth_any_from(%31, %29) : (i64, i64) -> i64
    call @sloth_main__print(%32) : (i64) -> ()
    %33 = call @sloth_rc_release(%32) : (i64) -> i64
    %34 = call @sloth_rc_release(%29) : (i64) -> i64
    %35 = memref.load %alloca_2[%c0] : memref<1xi64>
    %36 = call @sloth_obj_cls_id(%35) : (i64) -> i64
    %37 = arith.cmpi eq, %36, %c3_i64 : i64
    cf.cond_br %37, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %38 = memref.load %alloca_2[%c0] : memref<1xi64>
    %39 = call @sloth_obj_field(%38, %c1_i64) : (i64, i64) -> i64
    %40 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %40 : memref<11xi64> -> index
    %41 = arith.index_cast %intptr_5 : index to i64
    %42 = call @sloth_any_from(%41, %39) : (i64, i64) -> i64
    call @sloth_main__print(%42) : (i64) -> ()
    %43 = call @sloth_rc_release(%42) : (i64) -> i64
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %44 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %45 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %46 = call @sloth_cls_name(%44, %45, %c6_i64) : (i64, i64, i64) -> i64
    %47 = call @sloth_cls_refmask(%44, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %48 = call @sloth_obj_new(%44, %c1_i64) : (i64, i64) -> i64
    call @sloth_main_Animal____init__(%48, %c1_i64) : (i64, i64) -> ()
    %49 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %50 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %51 = call @sloth_cls_name(%49, %50, %c3_i64) : (i64, i64, i64) -> i64
    %52 = call @sloth_cls_refmask(%49, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %53 = call @sloth_obj_new(%49, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%53, %c2_i64) : (i64, i64) -> ()
    %54 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %55 = call @sloth_rc_retain(%48) : (i64) -> i64
    %56 = call @sloth_arr_set(%54, %c0_i64, %55) : (i64, i64, i64) -> i64
    %57 = call @sloth_rc_retain(%53) : (i64) -> i64
    %58 = call @sloth_arr_set(%54, %c1_i64, %57) : (i64, i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %59 = call @sloth_rc_retain(%54) : (i64) -> i64
    memref.store %59, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %60 = arith.index_cast %intptr_7 : index to i64
    %61 = call @sloth_fiber_track(%60) : (i64) -> i64
    %62 = call @sloth_rc_release(%48) : (i64) -> i64
    %63 = call @sloth_rc_release(%53) : (i64) -> i64
    %64 = call @sloth_rc_release(%54) : (i64) -> i64
    %65 = memref.load %alloca_6[%c0] : memref<1xi64>
    %66 = call @sloth_arr_len(%65) : (i64) -> i64
    %67 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %67 : memref<11xi64> -> index
    %68 = arith.index_cast %intptr_8 : index to i64
    %69 = call @sloth_any_from(%68, %66) : (i64, i64) -> i64
    call @sloth_main__print(%69) : (i64) -> ()
    %70 = call @sloth_rc_release(%69) : (i64) -> i64
    %71 = memref.load %alloca_6[%c0] : memref<1xi64>
    %72 = call @sloth_arr_get(%71, %c1_i64) : (i64, i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    %73 = call @sloth_obj_cls_id(%72) : (i64) -> i64
    %74 = arith.cmpi eq, %73, %c3_i64 : i64
    cf.cond_br %74, ^bb5, ^bb4
  ^bb4:  // pred: ^bb3
    %75 = call @sloth_main_Animal__who(%72) : (i64) -> i64
    memref.store %75, %alloca_9[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb5:  // pred: ^bb3
    %76 = call @sloth_main_Dog__who(%72) : (i64) -> i64
    memref.store %76, %alloca_9[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb6:  // 2 preds: ^bb4, ^bb5
    %77 = memref.load %alloca_9[%c0] : memref<1xi64>
    %78 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %78 : memref<11xi64> -> index
    %79 = arith.index_cast %intptr_10 : index to i64
    %80 = call @sloth_any_from(%79, %77) : (i64, i64) -> i64
    call @sloth_main__print(%80) : (i64) -> ()
    %81 = call @sloth_rc_release(%80) : (i64) -> i64
    %82 = call @sloth_rc_release(%77) : (i64) -> i64
    %83 = memref.load %alloca_6[%c0] : memref<1xi64>
    %84 = call @sloth_rc_release(%83) : (i64) -> i64
    %intptr_11 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %85 = arith.index_cast %intptr_11 : index to i64
    %86 = call @sloth_fiber_untrack(%85) : (i64) -> i64
    %87 = memref.load %alloca[%c0] : memref<1xi64>
    %88 = call @sloth_rc_release(%87) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %89 = arith.index_cast %intptr_12 : index to i64
    %90 = call @sloth_fiber_untrack(%89) : (i64) -> i64
    %91 = memref.load %alloca_2[%c0] : memref<1xi64>
    %92 = call @sloth_rc_release(%91) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %93 = arith.index_cast %intptr_13 : index to i64
    %94 = call @sloth_fiber_untrack(%93) : (i64) -> i64
    cf.br ^bb7
  ^bb7:  // pred: ^bb6
    return
  }
  func.func @sloth_main_Animal____init__(%arg0: i64, %arg1: i64) {
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_0[%c0] : memref<1xi64>
    %0 = memref.load %alloca_0[%c0] : memref<1xi64>
    %1 = memref.load %alloca[%c0] : memref<1xi64>
    %2 = call @sloth_obj_set_field(%1, %c0_i64, %0) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Animal__who(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c6_i64 = arith.constant 6 : i64
    %c119165703253601_i64 = arith.constant 119165703253601 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = call @sloth_str_push(%c0_i64, %c119165703253601_i64, %c6_i64) : (i64, i64, i64) -> i64
    %1 = call @sloth_str_finish(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main_Dog____init__(%arg0: i64, %arg1: i64) {
    %c1_i64 = arith.constant 1 : i64
    %c7_i64 = arith.constant 7 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_0[%c0] : memref<1xi64>
    %0 = memref.load %alloca[%c0] : memref<1xi64>
    call @sloth_main_Animal____init__(%0, %c7_i64) : (i64, i64) -> ()
    %1 = memref.load %alloca_0[%c0] : memref<1xi64>
    %2 = memref.load %alloca[%c0] : memref<1xi64>
    %3 = call @sloth_obj_set_field(%2, %c1_i64, %1) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Dog__who(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c6778724_i64 = arith.constant 6778724 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = call @sloth_str_push(%c0_i64, %c6778724_i64, %c3_i64) : (i64, i64, i64) -> i64
    %1 = call @sloth_str_finish(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c1_i64 = arith.constant 1 : i64
    %c4_i64 = arith.constant 4 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
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

