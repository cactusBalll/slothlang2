module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Node\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Node : memref<1xi64> = dense<0> {mutable}
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
  llvm.func @sloth_main_Node__cascade(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @sloth_rc_release(%0) : (i64) -> i64
    %2 = func.call @sloth_obj_field(%arg0, %c1_i64) : (i64, i64) -> i64
    %3 = func.call @sloth_rc_release(%2) : (i64) -> i64
    llvm.return %c0_i64 : i64
  }
  func.func private @sloth_vtb_main_Node() -> i64 {
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %0 = memref.get_global @sloth_main_g_vtb_Node : memref<1xi64>
    %1 = memref.load %0[%c0] : memref<1xi64>
    %2 = arith.cmpi eq, %1, %c0_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = call @sloth_vt_new(%c3_i64) : (i64) -> i64
    memref.store %3, %0[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %4 = memref.load %0[%c0] : memref<1xi64>
    return %4 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c5_i64 = arith.constant 5 : i64
    %c435778317409_i64 = arith.constant 435778317409 : i64
    %c1_i64 = arith.constant 1 : i64
    %c110_i64 = arith.constant 110 : i64
    %0 = llvm.mlir.addressof @sloth_main_Node__cascade : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %2 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %2, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @sloth_fiber_track(%3) : (i64) -> i64
    %5 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %7 = call @sloth_cls_name(%5, %6, %c4_i64) : (i64, i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = call @sloth_obj_new(%5, %c2_i64, %8) : (i64, i64, i64) -> i64
    %10 = call @sloth_vtb_main_Node() : () -> i64
    %11 = call @sloth_obj_set_vtable(%9, %10) : (i64, i64) -> i64
    %12 = call @sloth_str_push(%c0_i64, %c110_i64, %c1_i64) : (i64, i64, i64) -> i64
    %13 = call @sloth_str_finish(%12) : (i64) -> i64
    %14 = call @sloth_obj_field(%9, %c0_i64) : (i64, i64) -> i64
    %15 = call @sloth_rc_release(%14) : (i64) -> i64
    %16 = call @sloth_rc_retain(%13) : (i64) -> i64
    %17 = call @sloth_obj_set_field(%9, %c0_i64, %16) : (i64, i64, i64) -> i64
    %18 = call @sloth_obj_field(%9, %c1_i64) : (i64, i64) -> i64
    %19 = call @sloth_rc_release(%18) : (i64) -> i64
    %20 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    %21 = call @sloth_obj_set_field(%9, %c1_i64, %20) : (i64, i64, i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    %22 = call @sloth_rc_retain(%9) : (i64) -> i64
    memref.store %22, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %23 = arith.index_cast %intptr_1 : index to i64
    %24 = call @sloth_fiber_track(%23) : (i64) -> i64
    %25 = call @sloth_rc_release(%9) : (i64) -> i64
    %26 = call @sloth_rc_release(%13) : (i64) -> i64
    %27 = memref.load %alloca_0[%c0] : memref<1xi64>
    %28 = call @sloth_weak_new(%27) : (i64) -> i64
    %29 = memref.load %alloca[%c0] : memref<1xi64>
    %30 = call @sloth_rc_release(%29) : (i64) -> i64
    %31 = call @sloth_rc_retain(%28) : (i64) -> i64
    memref.store %31, %alloca[%c0] : memref<1xi64>
    %32 = call @sloth_rc_release(%28) : (i64) -> i64
    %33 = memref.load %alloca_0[%c0] : memref<1xi64>
    %34 = call @sloth_rc_release(%33) : (i64) -> i64
    %intptr_2 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %35 = arith.index_cast %intptr_2 : index to i64
    %36 = call @sloth_fiber_untrack(%35) : (i64) -> i64
    %37 = memref.load %alloca[%c0] : memref<1xi64>
    %38 = call @sloth_weak_upgrade(%37) : (i64) -> i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    %39 = call @sloth_rc_retain(%38) : (i64) -> i64
    memref.store %39, %alloca_3[%c0] : memref<1xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %alloca_3 : memref<1xi64> -> index
    %40 = arith.index_cast %intptr_4 : index to i64
    %41 = call @sloth_fiber_track(%40) : (i64) -> i64
    %42 = call @sloth_rc_release(%38) : (i64) -> i64
    %43 = memref.load %alloca_3[%c0] : memref<1xi64>
    %44 = arith.cmpi eq, %43, %c0_i64 : i64
    %45 = arith.extui %44 : i1 to i64
    %46 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %46 : memref<11xi64> -> index
    %47 = arith.index_cast %intptr_5 : index to i64
    %48 = call @sloth_any_from(%47, %45) : (i64, i64) -> i64
    call @sloth_main__print(%48) : (i64) -> ()
    %49 = call @sloth_rc_release(%48) : (i64) -> i64
    %50 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %51 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %52 = call @sloth_cls_name(%50, %51, %c4_i64) : (i64, i64, i64) -> i64
    %53 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %54 = call @sloth_obj_new(%50, %c2_i64, %53) : (i64, i64, i64) -> i64
    %55 = call @sloth_vtb_main_Node() : () -> i64
    %56 = call @sloth_obj_set_vtable(%54, %55) : (i64, i64) -> i64
    %57 = call @sloth_str_push(%c0_i64, %c110_i64, %c1_i64) : (i64, i64, i64) -> i64
    %58 = call @sloth_str_finish(%57) : (i64) -> i64
    %59 = call @sloth_obj_field(%54, %c0_i64) : (i64, i64) -> i64
    %60 = call @sloth_rc_release(%59) : (i64) -> i64
    %61 = call @sloth_rc_retain(%58) : (i64) -> i64
    %62 = call @sloth_obj_set_field(%54, %c0_i64, %61) : (i64, i64, i64) -> i64
    %63 = call @sloth_obj_field(%54, %c1_i64) : (i64, i64) -> i64
    %64 = call @sloth_rc_release(%63) : (i64) -> i64
    %65 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    %66 = call @sloth_obj_set_field(%54, %c1_i64, %65) : (i64, i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %67 = call @sloth_rc_retain(%54) : (i64) -> i64
    memref.store %67, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %68 = arith.index_cast %intptr_7 : index to i64
    %69 = call @sloth_fiber_track(%68) : (i64) -> i64
    %70 = call @sloth_rc_release(%54) : (i64) -> i64
    %71 = call @sloth_rc_release(%58) : (i64) -> i64
    %72 = call @sloth_str_push(%c0_i64, %c435778317409_i64, %c5_i64) : (i64, i64, i64) -> i64
    %73 = call @sloth_str_finish(%72) : (i64) -> i64
    %74 = memref.load %alloca_6[%c0] : memref<1xi64>
    %75 = call @sloth_obj_field(%74, %c0_i64) : (i64, i64) -> i64
    %76 = call @sloth_rc_release(%75) : (i64) -> i64
    %77 = call @sloth_rc_retain(%73) : (i64) -> i64
    %78 = call @sloth_obj_set_field(%74, %c0_i64, %77) : (i64, i64, i64) -> i64
    %79 = call @sloth_rc_release(%73) : (i64) -> i64
    %80 = memref.load %alloca_6[%c0] : memref<1xi64>
    %81 = call @sloth_weak_new(%80) : (i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    %82 = call @sloth_rc_retain(%81) : (i64) -> i64
    memref.store %82, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %83 = arith.index_cast %intptr_9 : index to i64
    %84 = call @sloth_fiber_track(%83) : (i64) -> i64
    %85 = call @sloth_rc_release(%81) : (i64) -> i64
    %86 = memref.load %alloca_8[%c0] : memref<1xi64>
    %87 = call @sloth_weak_upgrade(%86) : (i64) -> i64
    %alloca_10 = memref.alloca() : memref<1xi64>
    %88 = call @sloth_rc_retain(%87) : (i64) -> i64
    memref.store %88, %alloca_10[%c0] : memref<1xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %alloca_10 : memref<1xi64> -> index
    %89 = arith.index_cast %intptr_11 : index to i64
    %90 = call @sloth_fiber_track(%89) : (i64) -> i64
    %91 = call @sloth_rc_release(%87) : (i64) -> i64
    %92 = memref.load %alloca_10[%c0] : memref<1xi64>
    %93 = arith.cmpi eq, %92, %c0_i64 : i64
    %94 = arith.extui %93 : i1 to i64
    %95 = arith.xori %94, %c1_i64 : i64
    %96 = arith.cmpi ne, %95, %c0_i64 : i64
    cf.cond_br %96, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %97 = memref.load %alloca_10[%c0] : memref<1xi64>
    %98 = call @sloth_obj_field(%97, %c0_i64) : (i64, i64) -> i64
    %99 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %99 : memref<11xi64> -> index
    %100 = arith.index_cast %intptr_12 : index to i64
    %101 = call @sloth_any_from(%100, %98) : (i64, i64) -> i64
    call @sloth_main__print(%101) : (i64) -> ()
    %102 = call @sloth_rc_release(%101) : (i64) -> i64
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %103 = memref.load %alloca_8[%c0] : memref<1xi64>
    %104 = call @sloth_rc_release(%103) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %105 = arith.index_cast %intptr_13 : index to i64
    %106 = call @sloth_fiber_untrack(%105) : (i64) -> i64
    %107 = memref.load %alloca_6[%c0] : memref<1xi64>
    %108 = call @sloth_rc_release(%107) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %109 = arith.index_cast %intptr_14 : index to i64
    %110 = call @sloth_fiber_untrack(%109) : (i64) -> i64
    %111 = memref.load %alloca[%c0] : memref<1xi64>
    %112 = call @sloth_rc_release(%111) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %113 = arith.index_cast %intptr_15 : index to i64
    %114 = call @sloth_fiber_untrack(%113) : (i64) -> i64
    %115 = memref.load %alloca_10[%c0] : memref<1xi64>
    %116 = call @sloth_rc_release(%115) : (i64) -> i64
    %intptr_16 = memref.extract_aligned_pointer_as_index %alloca_10 : memref<1xi64> -> index
    %117 = arith.index_cast %intptr_16 : index to i64
    %118 = call @sloth_fiber_untrack(%117) : (i64) -> i64
    %119 = memref.load %alloca_3[%c0] : memref<1xi64>
    %120 = call @sloth_rc_release(%119) : (i64) -> i64
    %intptr_17 = memref.extract_aligned_pointer_as_index %alloca_3 : memref<1xi64> -> index
    %121 = arith.index_cast %intptr_17 : index to i64
    %122 = call @sloth_fiber_untrack(%121) : (i64) -> i64
    cf.br ^bb4
  ^bb4:  // pred: ^bb3
    return
  }
  func.func @sloth_main__anyinit() {
    %c3_i64 = arith.constant 3 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c4_i64 = arith.constant 4 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c1_i64, %2[%c0] : memref<11xi64>
    memref.store %c0_i64, %2[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %2[%c2] : memref<11xi64>
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %3, %2[%c3] : memref<11xi64>
    memref.store %c4_i64, %2[%c4] : memref<11xi64>
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

