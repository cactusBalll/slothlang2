module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Result<int, str>\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_4("Result<float, int>\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_5("float\00") {addr_space = 0 : i32}
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
  func.func @sloth_main__parse(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c4_i64 = arith.constant 4 : i64
    %c16_i64 = arith.constant 16 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c3_i64 = arith.constant 3 : i64
    %c6776174_i64 = arith.constant 6776174 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = arith.cmpi slt, %1, %c0_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = call @sloth_str_push(%c0_i64, %c6776174_i64, %c3_i64) : (i64, i64, i64) -> i64
    %4 = call @sloth_str_finish(%3) : (i64) -> i64
    %5 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %6 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %7 = call @sloth_cls_name(%5, %6, %c16_i64) : (i64, i64, i64) -> i64
    %8 = call @sloth_cls_refmask(%5, %c4_i64, %c3_i64) : (i64, i64, i64) -> i64
    %9 = call @sloth_obj_new(%5, %c3_i64) : (i64, i64) -> i64
    %10 = call @sloth_obj_set_field(%9, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %11 = call @sloth_obj_set_field(%9, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %12 = call @sloth_obj_set_field(%9, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %13 = call @sloth_obj_field(%9, %c2_i64) : (i64, i64) -> i64
    %14 = call @sloth_rc_release(%13) : (i64) -> i64
    %15 = call @sloth_rc_retain(%4) : (i64) -> i64
    %16 = call @sloth_obj_set_field(%9, %c2_i64, %15) : (i64, i64, i64) -> i64
    memref.store %9, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %17 = call @sloth_rc_release(%4) : (i64) -> i64
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %18 = memref.load %alloca_1[%c0] : memref<1xi64>
    %19 = arith.muli %18, %c2_i64 : i64
    %20 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %21 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %22 = call @sloth_cls_name(%20, %21, %c16_i64) : (i64, i64, i64) -> i64
    %23 = call @sloth_cls_refmask(%20, %c4_i64, %c3_i64) : (i64, i64, i64) -> i64
    %24 = call @sloth_obj_new(%20, %c3_i64) : (i64, i64) -> i64
    %25 = call @sloth_obj_set_field(%24, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %26 = call @sloth_obj_set_field(%24, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %27 = call @sloth_obj_set_field(%24, %c1_i64, %19) : (i64, i64, i64) -> i64
    %28 = call @sloth_obj_field(%24, %c2_i64) : (i64, i64) -> i64
    %29 = call @sloth_rc_release(%28) : (i64) -> i64
    %30 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    %31 = call @sloth_obj_set_field(%24, %c2_i64, %30) : (i64, i64, i64) -> i64
    memref.store %24, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %32 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %32 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c-1_i64 = arith.constant -1 : i64
    %c8_i64 = arith.constant 8 : i64
    %c16_i64 = arith.constant 16 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %c1684366707_i64 = arith.constant 1684366707 : i64
    %c2_i64 = arith.constant 2 : i64
    %c18_i64 = arith.constant 18 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_4 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c4612811918334230528_i64 = arith.constant 4612811918334230528 : i64
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c5_i64 = arith.constant 5 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_main__parse(%c5_i64) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %2, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @sloth_fiber_track(%3) : (i64) -> i64
    %5 = memref.load %alloca[%c0] : memref<1xi64>
    %6 = call @sloth_main_Result_int_str__is_ok(%5) : (i64) -> i64
    %7 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %7 : memref<11xi64> -> index
    %8 = arith.index_cast %intptr_0 : index to i64
    %9 = call @sloth_any_from(%8, %6) : (i64, i64) -> i64
    call @sloth_main__print(%9) : (i64) -> ()
    %10 = call @sloth_rc_release(%9) : (i64) -> i64
    %11 = memref.load %alloca[%c0] : memref<1xi64>
    %12 = call @sloth_main_Result_int_str__unwrap(%11) : (i64) -> i64
    %13 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %13 : memref<11xi64> -> index
    %14 = arith.index_cast %intptr_1 : index to i64
    %15 = call @sloth_any_from(%14, %12) : (i64, i64) -> i64
    call @sloth_main__print(%15) : (i64) -> ()
    %16 = call @sloth_rc_release(%15) : (i64) -> i64
    %17 = call @sloth_main__parse(%c-1_i64) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %17, %alloca_2[%c0] : memref<1xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %18 = arith.index_cast %intptr_3 : index to i64
    %19 = call @sloth_fiber_track(%18) : (i64) -> i64
    %20 = memref.load %alloca_2[%c0] : memref<1xi64>
    %21 = call @sloth_main_Result_int_str__is_ok(%20) : (i64) -> i64
    %22 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %22 : memref<11xi64> -> index
    %23 = arith.index_cast %intptr_4 : index to i64
    %24 = call @sloth_any_from(%23, %21) : (i64, i64) -> i64
    call @sloth_main__print(%24) : (i64) -> ()
    %25 = call @sloth_rc_release(%24) : (i64) -> i64
    %26 = memref.load %alloca_2[%c0] : memref<1xi64>
    %27 = call @sloth_main_Result_int_str__err(%26) : (i64) -> i64
    %28 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %28 : memref<11xi64> -> index
    %29 = arith.index_cast %intptr_5 : index to i64
    %30 = call @sloth_any_from(%29, %27) : (i64, i64) -> i64
    call @sloth_main__print(%30) : (i64) -> ()
    %31 = call @sloth_rc_release(%30) : (i64) -> i64
    %32 = call @sloth_rc_release(%27) : (i64) -> i64
    %33 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %34 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %35 = call @sloth_cls_name(%33, %34, %c18_i64) : (i64, i64, i64) -> i64
    %36 = call @sloth_cls_refmask(%33, %c0_i64, %c3_i64) : (i64, i64, i64) -> i64
    %37 = call @sloth_obj_new(%33, %c3_i64) : (i64, i64) -> i64
    %38 = call @sloth_obj_set_field(%37, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %39 = call @sloth_obj_set_field(%37, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %40 = call @sloth_obj_set_field(%37, %c1_i64, %c4612811918334230528_i64) : (i64, i64, i64) -> i64
    %41 = call @sloth_obj_set_field(%37, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %42 = call @sloth_rc_retain(%37) : (i64) -> i64
    memref.store %42, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %43 = arith.index_cast %intptr_7 : index to i64
    %44 = call @sloth_fiber_track(%43) : (i64) -> i64
    %45 = call @sloth_rc_release(%37) : (i64) -> i64
    %46 = memref.load %alloca_6[%c0] : memref<1xi64>
    %47 = call @sloth_main_Result_float_int__unwrap(%46) : (i64) -> i64
    %48 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %48 : memref<11xi64> -> index
    %49 = arith.index_cast %intptr_8 : index to i64
    %50 = call @sloth_any_from(%49, %47) : (i64, i64) -> i64
    call @sloth_main__print(%50) : (i64) -> ()
    %51 = call @sloth_rc_release(%50) : (i64) -> i64
    %52 = call @sloth_str_push(%c0_i64, %c1684366707_i64, %c4_i64) : (i64, i64, i64) -> i64
    %53 = call @sloth_str_finish(%52) : (i64) -> i64
    %54 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %55 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %56 = call @sloth_cls_name(%54, %55, %c16_i64) : (i64, i64, i64) -> i64
    %57 = call @sloth_cls_refmask(%54, %c4_i64, %c3_i64) : (i64, i64, i64) -> i64
    %58 = call @sloth_obj_new(%54, %c3_i64) : (i64, i64) -> i64
    %59 = call @sloth_obj_set_field(%58, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %60 = call @sloth_obj_set_field(%58, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %61 = call @sloth_obj_set_field(%58, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %62 = call @sloth_obj_field(%58, %c2_i64) : (i64, i64) -> i64
    %63 = call @sloth_rc_release(%62) : (i64) -> i64
    %64 = call @sloth_rc_retain(%53) : (i64) -> i64
    %65 = call @sloth_obj_set_field(%58, %c2_i64, %64) : (i64, i64, i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    %66 = call @sloth_rc_retain(%58) : (i64) -> i64
    memref.store %66, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %67 = arith.index_cast %intptr_10 : index to i64
    %68 = call @sloth_fiber_track(%67) : (i64) -> i64
    %69 = call @sloth_rc_release(%53) : (i64) -> i64
    %70 = call @sloth_rc_release(%58) : (i64) -> i64
    %71 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %72 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %73 = call @sloth_cls_name(%71, %72, %c16_i64) : (i64, i64, i64) -> i64
    %74 = call @sloth_cls_refmask(%71, %c4_i64, %c3_i64) : (i64, i64, i64) -> i64
    %75 = call @sloth_obj_new(%71, %c3_i64) : (i64, i64) -> i64
    %76 = call @sloth_obj_set_field(%75, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %77 = call @sloth_obj_set_field(%75, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %78 = call @sloth_obj_set_field(%75, %c1_i64, %c8_i64) : (i64, i64, i64) -> i64
    %79 = call @sloth_obj_field(%75, %c2_i64) : (i64, i64) -> i64
    %80 = call @sloth_rc_release(%79) : (i64) -> i64
    %81 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    %82 = call @sloth_obj_set_field(%75, %c2_i64, %81) : (i64, i64, i64) -> i64
    %83 = memref.load %alloca_9[%c0] : memref<1xi64>
    %84 = call @sloth_rc_release(%83) : (i64) -> i64
    %85 = call @sloth_rc_retain(%75) : (i64) -> i64
    memref.store %85, %alloca_9[%c0] : memref<1xi64>
    %86 = call @sloth_rc_release(%75) : (i64) -> i64
    %87 = memref.load %alloca_9[%c0] : memref<1xi64>
    %88 = call @sloth_main_Result_int_str__unwrap(%87) : (i64) -> i64
    %89 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %89 : memref<11xi64> -> index
    %90 = arith.index_cast %intptr_11 : index to i64
    %91 = call @sloth_any_from(%90, %88) : (i64, i64) -> i64
    call @sloth_main__print(%91) : (i64) -> ()
    %92 = call @sloth_rc_release(%91) : (i64) -> i64
    %93 = memref.load %alloca_6[%c0] : memref<1xi64>
    %94 = call @sloth_rc_release(%93) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %95 = arith.index_cast %intptr_12 : index to i64
    %96 = call @sloth_fiber_untrack(%95) : (i64) -> i64
    %97 = memref.load %alloca[%c0] : memref<1xi64>
    %98 = call @sloth_rc_release(%97) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %99 = arith.index_cast %intptr_13 : index to i64
    %100 = call @sloth_fiber_untrack(%99) : (i64) -> i64
    %101 = memref.load %alloca_9[%c0] : memref<1xi64>
    %102 = call @sloth_rc_release(%101) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %103 = arith.index_cast %intptr_14 : index to i64
    %104 = call @sloth_fiber_untrack(%103) : (i64) -> i64
    %105 = memref.load %alloca_2[%c0] : memref<1xi64>
    %106 = call @sloth_rc_release(%105) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %107 = arith.index_cast %intptr_15 : index to i64
    %108 = call @sloth_fiber_untrack(%107) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Result_int_str__is_ok(%arg0: i64) -> i64 {
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
  func.func @sloth_main_Result_int_str__unwrap(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %1, %alloca_2[%c0] : memref<1xi64>
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = arith.cmpi ne, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = call @sloth_obj_field(%4, %c1_i64) : (i64, i64) -> i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %6 = call @sloth_panic_unwrap() : () -> i64
    %7 = memref.load %alloca_1[%c0] : memref<1xi64>
    %8 = call @sloth_obj_field(%7, %c1_i64) : (i64, i64) -> i64
    memref.store %8, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %9 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %9 : i64
  }
  func.func @sloth_main_Result_int_str__err(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c2_i64) : (i64, i64) -> i64
    %2 = call @sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  func.func @sloth_main_Result_float_int__is_ok(%arg0: i64) -> i64 {
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
  func.func @sloth_main_Result_float_int__unwrap(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %1, %alloca_2[%c0] : memref<1xi64>
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = arith.cmpi ne, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = call @sloth_obj_field(%4, %c1_i64) : (i64, i64) -> i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %6 = call @sloth_panic_unwrap() : () -> i64
    %7 = memref.load %alloca_1[%c0] : memref<1xi64>
    %8 = call @sloth_obj_field(%7, %c1_i64) : (i64, i64) -> i64
    memref.store %8, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %9 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %9 : i64
  }
  func.func @sloth_main_Result_float_int__err(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c2_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main__anyinit() {
    %c5_i64 = arith.constant 5 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_5 : !llvm.ptr
    %c1099511627779_i64 = arith.constant 1099511627779 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %c3_i64 = arith.constant 3 : i64
    %2 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c4_i64 = arith.constant 4 : i64
    %c3 = arith.constant 3 : index
    %3 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %4 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c1_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %3 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c4_i64, %4[%c4] : memref<11xi64>
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
    memref.store %c4_i64, %8[%c0] : memref<11xi64>
    memref.store %c1_i64, %8[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %8[%c2] : memref<11xi64>
    %9 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %9, %8[%c3] : memref<11xi64>
    memref.store %c3_i64, %8[%c4] : memref<11xi64>
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

