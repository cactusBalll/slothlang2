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
  memref.global @sloth_main_g_vtb_Result_int_str : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Result_float_int : memref<1xi64> = dense<0> {mutable}
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
  llvm.func @sloth_main_Result_int_str__cascade(%arg0: i64, %arg1: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %0 = func.call @__sloth_obj_field(%arg0, %c2_i64) : (i64, i64) -> i64
    %1 = func.call @__sloth_rc_release(%0) : (i64) -> i64
    llvm.return %c0_i64 : i64
  }
  func.func private @sloth_vtb_main_Result_int_str() -> i64 {
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_main_Result_int_str__err : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %1 = llvm.mlir.addressof @sloth_main_Result_int_str__unwrap : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %2 = llvm.mlir.addressof @sloth_main_Result_int_str__is_ok : !llvm.ptr
    %c5_i64 = arith.constant 5 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %3 = memref.get_global @sloth_main_g_vtb_Result_int_str : memref<1xi64>
    %4 = memref.load %3[%c0] : memref<1xi64>
    %5 = arith.cmpi eq, %4, %c0_i64 : i64
    cf.cond_br %5, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %6 = call @__sloth_vt_new(%c5_i64) : (i64) -> i64
    %7 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %8 = call @__sloth_vt_set(%6, %c2_i64, %7) : (i64, i64, i64) -> i64
    %9 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %10 = call @__sloth_vt_set(%6, %c3_i64, %9) : (i64, i64, i64) -> i64
    %11 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %12 = call @__sloth_vt_set(%6, %c4_i64, %11) : (i64, i64, i64) -> i64
    memref.store %6, %3[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %13 = memref.load %3[%c0] : memref<1xi64>
    return %13 : i64
  }
  func.func @sloth_main__parse(%arg0: i64) -> i64 {
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.mlir.addressof @sloth_main_Result_int_str__cascade : !llvm.ptr
    %c16_i64 = arith.constant 16 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c6776174_i64 = arith.constant 6776174 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %2 = memref.load %alloca_1[%c0] : memref<1xi64>
    %3 = arith.cmpi slt, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = call @__sloth_str_push(%c0_i64, %c6776174_i64, %c3_i64) : (i64, i64, i64) -> i64
    %5 = call @__sloth_str_finish(%4) : (i64) -> i64
    %6 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %7 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %8 = call @__sloth_cls_name(%6, %7, %c16_i64) : (i64, i64, i64) -> i64
    %9 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %10 = call @__sloth_obj_new(%6, %c3_i64, %9) : (i64, i64, i64) -> i64
    %11 = call @sloth_vtb_main_Result_int_str() : () -> i64
    %12 = call @__sloth_obj_set_vtable(%10, %11) : (i64, i64) -> i64
    %13 = call @__sloth_obj_set_field(%10, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %14 = call @__sloth_obj_set_field(%10, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %15 = call @__sloth_obj_set_field(%10, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %16 = call @__sloth_obj_field(%10, %c2_i64) : (i64, i64) -> i64
    %17 = call @__sloth_rc_release(%16) : (i64) -> i64
    %18 = call @__sloth_rc_retain(%5) : (i64) -> i64
    %19 = call @__sloth_obj_set_field(%10, %c2_i64, %18) : (i64, i64, i64) -> i64
    memref.store %10, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %20 = call @__sloth_rc_release(%5) : (i64) -> i64
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %21 = memref.load %alloca_1[%c0] : memref<1xi64>
    %22 = arith.muli %21, %c2_i64 : i64
    %23 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %24 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %25 = call @__sloth_cls_name(%23, %24, %c16_i64) : (i64, i64, i64) -> i64
    %26 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %27 = call @__sloth_obj_new(%23, %c3_i64, %26) : (i64, i64, i64) -> i64
    %28 = call @sloth_vtb_main_Result_int_str() : () -> i64
    %29 = call @__sloth_obj_set_vtable(%27, %28) : (i64, i64) -> i64
    %30 = call @__sloth_obj_set_field(%27, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %31 = call @__sloth_obj_set_field(%27, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %32 = call @__sloth_obj_set_field(%27, %c1_i64, %22) : (i64, i64, i64) -> i64
    %33 = call @__sloth_obj_field(%27, %c2_i64) : (i64, i64) -> i64
    %34 = call @__sloth_rc_release(%33) : (i64) -> i64
    %35 = call @__sloth_rc_retain(%c0_i64) : (i64) -> i64
    %36 = call @__sloth_obj_set_field(%27, %c2_i64, %35) : (i64, i64, i64) -> i64
    memref.store %27, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %37 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %37 : i64
  }
  func.func private @sloth_vtb_main_Result_float_int() -> i64 {
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_main_Result_float_int__err : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %1 = llvm.mlir.addressof @sloth_main_Result_float_int__unwrap : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %2 = llvm.mlir.addressof @sloth_main_Result_float_int__is_ok : !llvm.ptr
    %c5_i64 = arith.constant 5 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %3 = memref.get_global @sloth_main_g_vtb_Result_float_int : memref<1xi64>
    %4 = memref.load %3[%c0] : memref<1xi64>
    %5 = arith.cmpi eq, %4, %c0_i64 : i64
    cf.cond_br %5, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %6 = call @__sloth_vt_new(%c5_i64) : (i64) -> i64
    %7 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %8 = call @__sloth_vt_set(%6, %c2_i64, %7) : (i64, i64, i64) -> i64
    %9 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %10 = call @__sloth_vt_set(%6, %c3_i64, %9) : (i64, i64, i64) -> i64
    %11 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %12 = call @__sloth_vt_set(%6, %c4_i64, %11) : (i64, i64, i64) -> i64
    memref.store %6, %3[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %13 = memref.load %3[%c0] : memref<1xi64>
    return %13 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c-1_i64 = arith.constant -1 : i64
    %c8_i64 = arith.constant 8 : i64
    %0 = llvm.mlir.addressof @sloth_main_Result_int_str__cascade : !llvm.ptr
    %c16_i64 = arith.constant 16 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c1684366707_i64 = arith.constant 1684366707 : i64
    %c2_i64 = arith.constant 2 : i64
    %c3_i64 = arith.constant 3 : i64
    %c18_i64 = arith.constant 18 : i64
    %2 = llvm.mlir.addressof @sloth_tynm_4 : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %c4612811918334230528_i64 = arith.constant 4612811918334230528 : i64
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c5_i64 = arith.constant 5 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %3 = call @sloth_main__parse(%c5_i64) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %3, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %4 = arith.index_cast %intptr : index to i64
    %5 = call @__sloth_fiber_track(%4) : (i64) -> i64
    %6 = memref.load %alloca[%c0] : memref<1xi64>
    %7 = llvm.call @sloth_main_Result_int_str__is_ok(%6) : (i64) -> i64
    %8 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_0 : index to i64
    %10 = call @__sloth_any_from(%9, %7) : (i64, i64) -> i64
    call @sloth_main__print(%10) : (i64) -> ()
    %11 = call @__sloth_rc_release(%10) : (i64) -> i64
    %12 = memref.load %alloca[%c0] : memref<1xi64>
    %13 = llvm.call @sloth_main_Result_int_str__unwrap(%12) : (i64) -> i64
    %14 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %14 : memref<11xi64> -> index
    %15 = arith.index_cast %intptr_1 : index to i64
    %16 = call @__sloth_any_from(%15, %13) : (i64, i64) -> i64
    call @sloth_main__print(%16) : (i64) -> ()
    %17 = call @__sloth_rc_release(%16) : (i64) -> i64
    %18 = call @sloth_main__parse(%c-1_i64) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %18, %alloca_2[%c0] : memref<1xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %19 = arith.index_cast %intptr_3 : index to i64
    %20 = call @__sloth_fiber_track(%19) : (i64) -> i64
    %21 = memref.load %alloca_2[%c0] : memref<1xi64>
    %22 = llvm.call @sloth_main_Result_int_str__is_ok(%21) : (i64) -> i64
    %23 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %23 : memref<11xi64> -> index
    %24 = arith.index_cast %intptr_4 : index to i64
    %25 = call @__sloth_any_from(%24, %22) : (i64, i64) -> i64
    call @sloth_main__print(%25) : (i64) -> ()
    %26 = call @__sloth_rc_release(%25) : (i64) -> i64
    %27 = memref.load %alloca_2[%c0] : memref<1xi64>
    %28 = llvm.call @sloth_main_Result_int_str__err(%27) : (i64) -> i64
    %29 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %29 : memref<11xi64> -> index
    %30 = arith.index_cast %intptr_5 : index to i64
    %31 = call @__sloth_any_from(%30, %28) : (i64, i64) -> i64
    call @sloth_main__print(%31) : (i64) -> ()
    %32 = call @__sloth_rc_release(%31) : (i64) -> i64
    %33 = call @__sloth_rc_release(%28) : (i64) -> i64
    %34 = call @__sloth_cls_info(%c0_i64, %c4_i64) : (i64, i64) -> i64
    %35 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %36 = call @__sloth_cls_name(%34, %35, %c18_i64) : (i64, i64, i64) -> i64
    %37 = call @__sloth_obj_new(%34, %c3_i64, %c0_i64) : (i64, i64, i64) -> i64
    %38 = call @sloth_vtb_main_Result_float_int() : () -> i64
    %39 = call @__sloth_obj_set_vtable(%37, %38) : (i64, i64) -> i64
    %40 = call @__sloth_obj_set_field(%37, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %41 = call @__sloth_obj_set_field(%37, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %42 = call @__sloth_obj_set_field(%37, %c1_i64, %c4612811918334230528_i64) : (i64, i64, i64) -> i64
    %43 = call @__sloth_obj_set_field(%37, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %44 = call @__sloth_rc_retain(%37) : (i64) -> i64
    memref.store %44, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %45 = arith.index_cast %intptr_7 : index to i64
    %46 = call @__sloth_fiber_track(%45) : (i64) -> i64
    %47 = call @__sloth_rc_release(%37) : (i64) -> i64
    %48 = memref.load %alloca_6[%c0] : memref<1xi64>
    %49 = llvm.call @sloth_main_Result_float_int__unwrap(%48) : (i64) -> i64
    %50 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %50 : memref<11xi64> -> index
    %51 = arith.index_cast %intptr_8 : index to i64
    %52 = call @__sloth_any_from(%51, %49) : (i64, i64) -> i64
    call @sloth_main__print(%52) : (i64) -> ()
    %53 = call @__sloth_rc_release(%52) : (i64) -> i64
    %54 = call @__sloth_str_push(%c0_i64, %c1684366707_i64, %c4_i64) : (i64, i64, i64) -> i64
    %55 = call @__sloth_str_finish(%54) : (i64) -> i64
    %56 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %57 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %58 = call @__sloth_cls_name(%56, %57, %c16_i64) : (i64, i64, i64) -> i64
    %59 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %60 = call @__sloth_obj_new(%56, %c3_i64, %59) : (i64, i64, i64) -> i64
    %61 = call @sloth_vtb_main_Result_int_str() : () -> i64
    %62 = call @__sloth_obj_set_vtable(%60, %61) : (i64, i64) -> i64
    %63 = call @__sloth_obj_set_field(%60, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %64 = call @__sloth_obj_set_field(%60, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %65 = call @__sloth_obj_set_field(%60, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %66 = call @__sloth_obj_field(%60, %c2_i64) : (i64, i64) -> i64
    %67 = call @__sloth_rc_release(%66) : (i64) -> i64
    %68 = call @__sloth_rc_retain(%55) : (i64) -> i64
    %69 = call @__sloth_obj_set_field(%60, %c2_i64, %68) : (i64, i64, i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    %70 = call @__sloth_rc_retain(%60) : (i64) -> i64
    memref.store %70, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %71 = arith.index_cast %intptr_10 : index to i64
    %72 = call @__sloth_fiber_track(%71) : (i64) -> i64
    %73 = call @__sloth_rc_release(%55) : (i64) -> i64
    %74 = call @__sloth_rc_release(%60) : (i64) -> i64
    %75 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %76 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %77 = call @__sloth_cls_name(%75, %76, %c16_i64) : (i64, i64, i64) -> i64
    %78 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %79 = call @__sloth_obj_new(%75, %c3_i64, %78) : (i64, i64, i64) -> i64
    %80 = call @sloth_vtb_main_Result_int_str() : () -> i64
    %81 = call @__sloth_obj_set_vtable(%79, %80) : (i64, i64) -> i64
    %82 = call @__sloth_obj_set_field(%79, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %83 = call @__sloth_obj_set_field(%79, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %84 = call @__sloth_obj_set_field(%79, %c1_i64, %c8_i64) : (i64, i64, i64) -> i64
    %85 = call @__sloth_obj_field(%79, %c2_i64) : (i64, i64) -> i64
    %86 = call @__sloth_rc_release(%85) : (i64) -> i64
    %87 = call @__sloth_rc_retain(%c0_i64) : (i64) -> i64
    %88 = call @__sloth_obj_set_field(%79, %c2_i64, %87) : (i64, i64, i64) -> i64
    %89 = memref.load %alloca_9[%c0] : memref<1xi64>
    %90 = call @__sloth_rc_release(%89) : (i64) -> i64
    %91 = call @__sloth_rc_retain(%79) : (i64) -> i64
    memref.store %91, %alloca_9[%c0] : memref<1xi64>
    %92 = call @__sloth_rc_release(%79) : (i64) -> i64
    %93 = memref.load %alloca_9[%c0] : memref<1xi64>
    %94 = llvm.call @sloth_main_Result_int_str__unwrap(%93) : (i64) -> i64
    %95 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %95 : memref<11xi64> -> index
    %96 = arith.index_cast %intptr_11 : index to i64
    %97 = call @__sloth_any_from(%96, %94) : (i64, i64) -> i64
    call @sloth_main__print(%97) : (i64) -> ()
    %98 = call @__sloth_rc_release(%97) : (i64) -> i64
    %99 = memref.load %alloca[%c0] : memref<1xi64>
    %100 = call @__sloth_rc_release(%99) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %101 = arith.index_cast %intptr_12 : index to i64
    %102 = call @__sloth_fiber_untrack(%101) : (i64) -> i64
    %103 = memref.load %alloca_2[%c0] : memref<1xi64>
    %104 = call @__sloth_rc_release(%103) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %105 = arith.index_cast %intptr_13 : index to i64
    %106 = call @__sloth_fiber_untrack(%105) : (i64) -> i64
    %107 = memref.load %alloca_9[%c0] : memref<1xi64>
    %108 = call @__sloth_rc_release(%107) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %109 = arith.index_cast %intptr_14 : index to i64
    %110 = call @__sloth_fiber_untrack(%109) : (i64) -> i64
    %111 = memref.load %alloca_6[%c0] : memref<1xi64>
    %112 = call @__sloth_rc_release(%111) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %113 = arith.index_cast %intptr_15 : index to i64
    %114 = call @__sloth_fiber_untrack(%113) : (i64) -> i64
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
  llvm.func @sloth_main_Result_int_str__is_ok(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
  }
  llvm.func @sloth_main_Result_int_str__unwrap(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %1, %alloca_2[%c0] : memref<1xi64>
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = arith.cmpi ne, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = func.call @__sloth_obj_field(%4, %c1_i64) : (i64, i64) -> i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %6 = func.call @__sloth_panic_unwrap() : () -> i64
    %7 = memref.load %alloca_1[%c0] : memref<1xi64>
    %8 = func.call @__sloth_obj_field(%7, %c1_i64) : (i64, i64) -> i64
    memref.store %8, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %9 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %9 : i64
  }
  llvm.func @sloth_main_Result_int_str__err(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c2_i64) : (i64, i64) -> i64
    %2 = func.call @__sloth_rc_retain(%1) : (i64) -> i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %3 : i64
  }
  llvm.func @sloth_main_Result_float_int__is_ok(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
  }
  llvm.func @sloth_main_Result_float_int__unwrap(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %1, %alloca_2[%c0] : memref<1xi64>
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = arith.cmpi ne, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = func.call @__sloth_obj_field(%4, %c1_i64) : (i64, i64) -> i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %6 = func.call @__sloth_panic_unwrap() : () -> i64
    %7 = memref.load %alloca_1[%c0] : memref<1xi64>
    %8 = func.call @__sloth_obj_field(%7, %c1_i64) : (i64, i64) -> i64
    memref.store %8, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %9 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %9 : i64
  }
  llvm.func @sloth_main_Result_float_int__err(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c2_i64) : (i64, i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
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

