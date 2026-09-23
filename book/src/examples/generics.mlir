module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("Box<int>\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("Box<float>\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_4("float\00") {addr_space = 0 : i32}
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
  func.func @sloth_main__first(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_arr_get(%0, %c0_i64) : (i64, i64) -> i64
    %2 = call @sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  func.func @sloth_main__twice(%arg0: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %3 = call @sloth_rc_retain(%0) : (i64) -> i64
    %4 = call @sloth_arr_set(%2, %c0_i64, %3) : (i64, i64, i64) -> i64
    %5 = call @sloth_rc_retain(%1) : (i64) -> i64
    %6 = call @sloth_arr_set(%2, %c1_i64, %5) : (i64, i64, i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %7 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %7 : i64
  }
  func.func @sloth_main__first_int(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_arr_get(%0, %c0_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main__first_str(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_arr_get(%0, %c0_i64) : (i64, i64) -> i64
    %2 = call @sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  func.func @sloth_main__twice_str(%arg0: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %3 = call @sloth_rc_retain(%0) : (i64) -> i64
    %4 = call @sloth_arr_set(%2, %c0_i64, %3) : (i64, i64, i64) -> i64
    %5 = call @sloth_rc_retain(%1) : (i64) -> i64
    %6 = call @sloth_arr_set(%2, %c1_i64, %5) : (i64, i64, i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %7 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %7 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c4609434218613702656_i64 = arith.constant 4609434218613702656 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %c7_i64 = arith.constant 7 : i64
    %c8_i64 = arith.constant 8 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c122_i64 = arith.constant 122 : i64
    %c121_i64 = arith.constant 121 : i64
    %c120_i64 = arith.constant 120 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %c20_i64 = arith.constant 20 : i64
    %c10_i64 = arith.constant 10 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_arr_new(%c2_i64) : (i64) -> i64
    %3 = call @sloth_arr_set(%2, %c0_i64, %c10_i64) : (i64, i64, i64) -> i64
    %4 = call @sloth_arr_set(%2, %c1_i64, %c20_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %5 = call @sloth_rc_retain(%2) : (i64) -> i64
    memref.store %5, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %6 = arith.index_cast %intptr : index to i64
    %7 = call @sloth_fiber_track(%6) : (i64) -> i64
    %8 = call @sloth_rc_release(%2) : (i64) -> i64
    %9 = memref.load %alloca[%c0] : memref<1xi64>
    %10 = call @sloth_main__first_int(%9) : (i64) -> i64
    %11 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %11 : memref<11xi64> -> index
    %12 = arith.index_cast %intptr_0 : index to i64
    %13 = call @sloth_any_from(%12, %10) : (i64, i64) -> i64
    call @sloth_main__print(%13) : (i64) -> ()
    %14 = call @sloth_rc_release(%13) : (i64) -> i64
    %15 = call @sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %16 = call @sloth_str_finish(%15) : (i64) -> i64
    %17 = call @sloth_str_push(%c0_i64, %c121_i64, %c1_i64) : (i64, i64, i64) -> i64
    %18 = call @sloth_str_finish(%17) : (i64) -> i64
    %19 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %20 = call @sloth_rc_retain(%16) : (i64) -> i64
    %21 = call @sloth_arr_set(%19, %c0_i64, %20) : (i64, i64, i64) -> i64
    %22 = call @sloth_rc_retain(%18) : (i64) -> i64
    %23 = call @sloth_arr_set(%19, %c1_i64, %22) : (i64, i64, i64) -> i64
    %alloca_1 = memref.alloca() : memref<1xi64>
    %24 = call @sloth_rc_retain(%19) : (i64) -> i64
    memref.store %24, %alloca_1[%c0] : memref<1xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %alloca_1 : memref<1xi64> -> index
    %25 = arith.index_cast %intptr_2 : index to i64
    %26 = call @sloth_fiber_track(%25) : (i64) -> i64
    %27 = call @sloth_rc_release(%16) : (i64) -> i64
    %28 = call @sloth_rc_release(%18) : (i64) -> i64
    %29 = call @sloth_rc_release(%19) : (i64) -> i64
    %30 = memref.load %alloca_1[%c0] : memref<1xi64>
    %31 = call @sloth_main__first_str(%30) : (i64) -> i64
    %32 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %32 : memref<11xi64> -> index
    %33 = arith.index_cast %intptr_3 : index to i64
    %34 = call @sloth_any_from(%33, %31) : (i64, i64) -> i64
    call @sloth_main__print(%34) : (i64) -> ()
    %35 = call @sloth_rc_release(%34) : (i64) -> i64
    %36 = call @sloth_rc_release(%31) : (i64) -> i64
    %37 = memref.load %alloca[%c0] : memref<1xi64>
    %38 = call @sloth_main__first_int(%37) : (i64) -> i64
    %39 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %39 : memref<11xi64> -> index
    %40 = arith.index_cast %intptr_4 : index to i64
    %41 = call @sloth_any_from(%40, %38) : (i64, i64) -> i64
    call @sloth_main__print(%41) : (i64) -> ()
    %42 = call @sloth_rc_release(%41) : (i64) -> i64
    %43 = call @sloth_str_push(%c0_i64, %c122_i64, %c1_i64) : (i64, i64, i64) -> i64
    %44 = call @sloth_str_finish(%43) : (i64) -> i64
    %45 = call @sloth_main__twice_str(%44) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %45, %alloca_5[%c0] : memref<1xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %alloca_5 : memref<1xi64> -> index
    %46 = arith.index_cast %intptr_6 : index to i64
    %47 = call @sloth_fiber_track(%46) : (i64) -> i64
    %48 = call @sloth_rc_release(%44) : (i64) -> i64
    %49 = memref.load %alloca_5[%c0] : memref<1xi64>
    %50 = call @sloth_arr_get(%49, %c0_i64) : (i64, i64) -> i64
    %51 = memref.load %alloca_5[%c0] : memref<1xi64>
    %52 = call @sloth_arr_get(%51, %c1_i64) : (i64, i64) -> i64
    %53 = call @sloth_str_concat(%50, %52) : (i64, i64) -> i64
    %54 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %54 : memref<11xi64> -> index
    %55 = arith.index_cast %intptr_7 : index to i64
    %56 = call @sloth_any_from(%55, %53) : (i64, i64) -> i64
    call @sloth_main__print(%56) : (i64) -> ()
    %57 = call @sloth_rc_release(%53) : (i64) -> i64
    %58 = call @sloth_rc_release(%56) : (i64) -> i64
    %59 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %60 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %61 = call @sloth_cls_name(%59, %60, %c8_i64) : (i64, i64, i64) -> i64
    %62 = call @sloth_cls_refmask(%59, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %63 = call @sloth_obj_new(%59, %c1_i64) : (i64, i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    %64 = call @sloth_rc_retain(%63) : (i64) -> i64
    memref.store %64, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %65 = arith.index_cast %intptr_9 : index to i64
    %66 = call @sloth_fiber_track(%65) : (i64) -> i64
    %67 = call @sloth_rc_release(%63) : (i64) -> i64
    %68 = memref.load %alloca_8[%c0] : memref<1xi64>
    call @sloth_main_Box_int__set(%68, %c7_i64) : (i64, i64) -> ()
    %69 = memref.load %alloca_8[%c0] : memref<1xi64>
    %70 = call @sloth_main_Box_int__get(%69) : (i64) -> i64
    %71 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %71 : memref<11xi64> -> index
    %72 = arith.index_cast %intptr_10 : index to i64
    %73 = call @sloth_any_from(%72, %70) : (i64, i64) -> i64
    call @sloth_main__print(%73) : (i64) -> ()
    %74 = call @sloth_rc_release(%73) : (i64) -> i64
    %75 = call @sloth_cls_info(%c0_i64, %c4_i64) : (i64, i64) -> i64
    %76 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %77 = call @sloth_cls_name(%75, %76, %c10_i64) : (i64, i64, i64) -> i64
    %78 = call @sloth_cls_refmask(%75, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %79 = call @sloth_obj_new(%75, %c1_i64) : (i64, i64) -> i64
    %alloca_11 = memref.alloca() : memref<1xi64>
    %80 = call @sloth_rc_retain(%79) : (i64) -> i64
    memref.store %80, %alloca_11[%c0] : memref<1xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %81 = arith.index_cast %intptr_12 : index to i64
    %82 = call @sloth_fiber_track(%81) : (i64) -> i64
    %83 = call @sloth_rc_release(%79) : (i64) -> i64
    %84 = memref.load %alloca_11[%c0] : memref<1xi64>
    call @sloth_main_Box_float__set(%84, %c4609434218613702656_i64) : (i64, i64) -> ()
    %85 = memref.load %alloca_11[%c0] : memref<1xi64>
    %86 = call @sloth_main_Box_float__get(%85) : (i64) -> i64
    %87 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %87 : memref<11xi64> -> index
    %88 = arith.index_cast %intptr_13 : index to i64
    %89 = call @sloth_any_from(%88, %86) : (i64, i64) -> i64
    call @sloth_main__print(%89) : (i64) -> ()
    %90 = call @sloth_rc_release(%89) : (i64) -> i64
    %91 = memref.load %alloca[%c0] : memref<1xi64>
    %92 = call @sloth_rc_release(%91) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %93 = arith.index_cast %intptr_14 : index to i64
    %94 = call @sloth_fiber_untrack(%93) : (i64) -> i64
    %95 = memref.load %alloca_1[%c0] : memref<1xi64>
    %96 = call @sloth_rc_release(%95) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_1 : memref<1xi64> -> index
    %97 = arith.index_cast %intptr_15 : index to i64
    %98 = call @sloth_fiber_untrack(%97) : (i64) -> i64
    %99 = memref.load %alloca_8[%c0] : memref<1xi64>
    %100 = call @sloth_rc_release(%99) : (i64) -> i64
    %intptr_16 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %101 = arith.index_cast %intptr_16 : index to i64
    %102 = call @sloth_fiber_untrack(%101) : (i64) -> i64
    %103 = memref.load %alloca_11[%c0] : memref<1xi64>
    %104 = call @sloth_rc_release(%103) : (i64) -> i64
    %intptr_17 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %105 = arith.index_cast %intptr_17 : index to i64
    %106 = call @sloth_fiber_untrack(%105) : (i64) -> i64
    %107 = memref.load %alloca_5[%c0] : memref<1xi64>
    %108 = call @sloth_rc_release(%107) : (i64) -> i64
    %intptr_18 = memref.extract_aligned_pointer_as_index %alloca_5 : memref<1xi64> -> index
    %109 = arith.index_cast %intptr_18 : index to i64
    %110 = call @sloth_fiber_untrack(%109) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Box_int__set(%arg0: i64, %arg1: i64) {
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
  func.func @sloth_main_Box_int__get(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main_Box_float__set(%arg0: i64, %arg1: i64) {
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
  func.func @sloth_main_Box_float__get(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main__anyinit() {
    %c5_i64 = arith.constant 5 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_4 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c1_i64 = arith.constant 1 : i64
    %c4_i64 = arith.constant 4 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %2 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %3 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c2_i64, %3[%c0] : memref<11xi64>
    memref.store %c0_i64, %3[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %3[%c2] : memref<11xi64>
    %4 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %4, %3[%c3] : memref<11xi64>
    memref.store %c3_i64, %3[%c4] : memref<11xi64>
    memref.store %c0_i64, %3[%c9] : memref<11xi64>
    memref.store %c0_i64, %3[%c10] : memref<11xi64>
    %5 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c4_i64, %5[%c0] : memref<11xi64>
    memref.store %c1_i64, %5[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %5[%c2] : memref<11xi64>
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %6, %5[%c3] : memref<11xi64>
    memref.store %c3_i64, %5[%c4] : memref<11xi64>
    memref.store %c0_i64, %5[%c9] : memref<11xi64>
    memref.store %c0_i64, %5[%c10] : memref<11xi64>
    %7 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    memref.store %c3_i64, %7[%c0] : memref<11xi64>
    memref.store %c0_i64, %7[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %7[%c2] : memref<11xi64>
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %8, %7[%c3] : memref<11xi64>
    memref.store %c5_i64, %7[%c4] : memref<11xi64>
    memref.store %c0_i64, %7[%c9] : memref<11xi64>
    memref.store %c0_i64, %7[%c10] : memref<11xi64>
    return
  }
}

