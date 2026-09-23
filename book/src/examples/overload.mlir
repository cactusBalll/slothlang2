module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Vec2\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("Bag\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_4("int\00") {addr_space = 0 : i32}
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
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c8_i64 = arith.constant 8 : i64
    %c7_i64 = arith.constant 7 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @sloth_cls_name(%2, %3, %c4_i64) : (i64, i64, i64) -> i64
    %5 = call @sloth_cls_refmask(%2, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_obj_new(%2, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%6, %c1_i64, %c2_i64) : (i64, i64, i64) -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %7 = call @sloth_rc_retain(%6) : (i64) -> i64
    memref.store %7, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %8 = arith.index_cast %intptr : index to i64
    %9 = call @sloth_fiber_track(%8) : (i64) -> i64
    %10 = call @sloth_rc_release(%6) : (i64) -> i64
    %11 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %12 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %13 = call @sloth_cls_name(%11, %12, %c4_i64) : (i64, i64, i64) -> i64
    %14 = call @sloth_cls_refmask(%11, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %15 = call @sloth_obj_new(%11, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%15, %c3_i64, %c4_i64) : (i64, i64, i64) -> ()
    %alloca_0 = memref.alloca() : memref<1xi64>
    %16 = call @sloth_rc_retain(%15) : (i64) -> i64
    memref.store %16, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %17 = arith.index_cast %intptr_1 : index to i64
    %18 = call @sloth_fiber_track(%17) : (i64) -> i64
    %19 = call @sloth_rc_release(%15) : (i64) -> i64
    %20 = memref.load %alloca[%c0] : memref<1xi64>
    %21 = memref.load %alloca_0[%c0] : memref<1xi64>
    %22 = call @sloth_main_Vec2____add__(%20, %21) : (i64, i64) -> i64
    %23 = call @sloth_main_Vec2__show(%22) : (i64) -> i64
    %24 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %24 : memref<11xi64> -> index
    %25 = arith.index_cast %intptr_2 : index to i64
    %26 = call @sloth_any_from(%25, %23) : (i64, i64) -> i64
    call @sloth_main__print(%26) : (i64) -> ()
    %27 = call @sloth_rc_release(%26) : (i64) -> i64
    %28 = call @sloth_rc_release(%22) : (i64) -> i64
    %29 = call @sloth_rc_release(%23) : (i64) -> i64
    %30 = memref.load %alloca_0[%c0] : memref<1xi64>
    %31 = memref.load %alloca[%c0] : memref<1xi64>
    %32 = call @sloth_main_Vec2____sub__(%30, %31) : (i64, i64) -> i64
    %33 = call @sloth_main_Vec2__show(%32) : (i64) -> i64
    %34 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %34 : memref<11xi64> -> index
    %35 = arith.index_cast %intptr_3 : index to i64
    %36 = call @sloth_any_from(%35, %33) : (i64, i64) -> i64
    call @sloth_main__print(%36) : (i64) -> ()
    %37 = call @sloth_rc_release(%36) : (i64) -> i64
    %38 = call @sloth_rc_release(%32) : (i64) -> i64
    %39 = call @sloth_rc_release(%33) : (i64) -> i64
    %40 = memref.load %alloca[%c0] : memref<1xi64>
    %41 = call @sloth_main_Vec2____neg__(%40) : (i64) -> i64
    %42 = call @sloth_main_Vec2__show(%41) : (i64) -> i64
    %43 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %43 : memref<11xi64> -> index
    %44 = arith.index_cast %intptr_4 : index to i64
    %45 = call @sloth_any_from(%44, %42) : (i64, i64) -> i64
    call @sloth_main__print(%45) : (i64) -> ()
    %46 = call @sloth_rc_release(%45) : (i64) -> i64
    %47 = call @sloth_rc_release(%41) : (i64) -> i64
    %48 = call @sloth_rc_release(%42) : (i64) -> i64
    %49 = memref.load %alloca[%c0] : memref<1xi64>
    %50 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %51 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %52 = call @sloth_cls_name(%50, %51, %c4_i64) : (i64, i64, i64) -> i64
    %53 = call @sloth_cls_refmask(%50, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %54 = call @sloth_obj_new(%50, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%54, %c1_i64, %c2_i64) : (i64, i64, i64) -> ()
    %55 = call @sloth_main_Vec2____eq__(%49, %54) : (i64, i64) -> i64
    %56 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %56 : memref<11xi64> -> index
    %57 = arith.index_cast %intptr_5 : index to i64
    %58 = call @sloth_any_from(%57, %55) : (i64, i64) -> i64
    call @sloth_main__print(%58) : (i64) -> ()
    %59 = call @sloth_rc_release(%54) : (i64) -> i64
    %60 = call @sloth_rc_release(%58) : (i64) -> i64
    %61 = memref.load %alloca[%c0] : memref<1xi64>
    %62 = memref.load %alloca_0[%c0] : memref<1xi64>
    %63 = call @sloth_main_Vec2____lt__(%61, %62) : (i64, i64) -> i64
    %64 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %64 : memref<11xi64> -> index
    %65 = arith.index_cast %intptr_6 : index to i64
    %66 = call @sloth_any_from(%65, %63) : (i64, i64) -> i64
    call @sloth_main__print(%66) : (i64) -> ()
    %67 = call @sloth_rc_release(%66) : (i64) -> i64
    %68 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %69 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %70 = call @sloth_cls_name(%68, %69, %c3_i64) : (i64, i64, i64) -> i64
    %71 = call @sloth_cls_refmask(%68, %c1_i64, %c1_i64) : (i64, i64, i64) -> i64
    %72 = call @sloth_obj_new(%68, %c1_i64) : (i64, i64) -> i64
    call @sloth_main_Bag____init__(%72) : (i64) -> ()
    %alloca_7 = memref.alloca() : memref<1xi64>
    %73 = call @sloth_rc_retain(%72) : (i64) -> i64
    memref.store %73, %alloca_7[%c0] : memref<1xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %alloca_7 : memref<1xi64> -> index
    %74 = arith.index_cast %intptr_8 : index to i64
    %75 = call @sloth_fiber_track(%74) : (i64) -> i64
    %76 = call @sloth_rc_release(%72) : (i64) -> i64
    %77 = memref.load %alloca_7[%c0] : memref<1xi64>
    call @sloth_main_Bag____assign__(%77, %c0_i64, %c7_i64) : (i64, i64, i64) -> ()
    %78 = memref.load %alloca_7[%c0] : memref<1xi64>
    call @sloth_main_Bag____assign__(%78, %c1_i64, %c8_i64) : (i64, i64, i64) -> ()
    %79 = memref.load %alloca_7[%c0] : memref<1xi64>
    %80 = call @sloth_main_Bag____index__(%79, %c0_i64) : (i64, i64) -> i64
    %81 = memref.load %alloca_7[%c0] : memref<1xi64>
    %82 = call @sloth_main_Bag____index__(%81, %c1_i64) : (i64, i64) -> i64
    %83 = arith.addi %80, %82 : i64
    %84 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %84 : memref<11xi64> -> index
    %85 = arith.index_cast %intptr_9 : index to i64
    %86 = call @sloth_any_from(%85, %83) : (i64, i64) -> i64
    call @sloth_main__print(%86) : (i64) -> ()
    %87 = call @sloth_rc_release(%86) : (i64) -> i64
    %88 = memref.load %alloca[%c0] : memref<1xi64>
    %89 = call @sloth_rc_release(%88) : (i64) -> i64
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %90 = arith.index_cast %intptr_10 : index to i64
    %91 = call @sloth_fiber_untrack(%90) : (i64) -> i64
    %92 = memref.load %alloca_0[%c0] : memref<1xi64>
    %93 = call @sloth_rc_release(%92) : (i64) -> i64
    %intptr_11 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %94 = arith.index_cast %intptr_11 : index to i64
    %95 = call @sloth_fiber_untrack(%94) : (i64) -> i64
    %96 = memref.load %alloca_7[%c0] : memref<1xi64>
    %97 = call @sloth_rc_release(%96) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_7 : memref<1xi64> -> index
    %98 = arith.index_cast %intptr_12 : index to i64
    %99 = call @sloth_fiber_untrack(%98) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Vec2____init__(%arg0: i64, %arg1: i64, %arg2: i64) {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_0[%c0] : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg2, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_0[%c0] : memref<1xi64>
    %1 = memref.load %alloca[%c0] : memref<1xi64>
    %2 = call @sloth_obj_set_field(%1, %c0_i64, %0) : (i64, i64, i64) -> i64
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = memref.load %alloca[%c0] : memref<1xi64>
    %5 = call @sloth_obj_set_field(%4, %c1_i64, %3) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Vec2____add__(%arg0: i64, %arg1: i64) -> i64 {
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.load %alloca_2[%c0] : memref<1xi64>
    %4 = call @sloth_obj_field(%3, %c0_i64) : (i64, i64) -> i64
    %5 = arith.addi %2, %4 : i64
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = call @sloth_obj_field(%6, %c1_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %10 = arith.addi %7, %9 : i64
    %11 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %12 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %13 = call @sloth_cls_name(%11, %12, %c4_i64) : (i64, i64, i64) -> i64
    %14 = call @sloth_cls_refmask(%11, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %15 = call @sloth_obj_new(%11, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%15, %5, %10) : (i64, i64, i64) -> ()
    memref.store %15, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %16 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %16 : i64
  }
  func.func @sloth_main_Vec2____sub__(%arg0: i64, %arg1: i64) -> i64 {
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.load %alloca_2[%c0] : memref<1xi64>
    %4 = call @sloth_obj_field(%3, %c0_i64) : (i64, i64) -> i64
    %5 = arith.subi %2, %4 : i64
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = call @sloth_obj_field(%6, %c1_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %10 = arith.subi %7, %9 : i64
    %11 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %12 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %13 = call @sloth_cls_name(%11, %12, %c4_i64) : (i64, i64, i64) -> i64
    %14 = call @sloth_cls_refmask(%11, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %15 = call @sloth_obj_new(%11, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%15, %5, %10) : (i64, i64, i64) -> ()
    memref.store %15, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %16 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %16 : i64
  }
  func.func @sloth_main_Vec2____neg__(%arg0: i64) -> i64 {
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = arith.subi %c0_i64, %2 : i64
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = call @sloth_obj_field(%4, %c1_i64) : (i64, i64) -> i64
    %6 = arith.subi %c0_i64, %5 : i64
    %7 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = call @sloth_cls_name(%7, %8, %c4_i64) : (i64, i64, i64) -> i64
    %10 = call @sloth_cls_refmask(%7, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %11 = call @sloth_obj_new(%7, %c2_i64) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%11, %3, %6) : (i64, i64, i64) -> ()
    memref.store %11, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %12 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %12 : i64
  }
  func.func @sloth_main_Vec2____eq__(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = call @sloth_obj_field(%2, %c0_i64) : (i64, i64) -> i64
    %4 = arith.cmpi eq, %1, %3 : i64
    %5 = arith.extui %4 : i1 to i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %5, %alloca_3[%c0] : memref<1xi64>
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = call @sloth_obj_field(%6, %c1_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %10 = arith.cmpi eq, %7, %9 : i64
    %11 = arith.extui %10 : i1 to i64
    memref.store %11, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %12 = memref.load %alloca_3[%c0] : memref<1xi64>
    memref.store %12, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %13 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %13 : i64
  }
  func.func @sloth_main_Vec2____lt__(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = call @sloth_obj_field(%2, %c0_i64) : (i64, i64) -> i64
    %4 = arith.cmpi slt, %1, %3 : i64
    %5 = arith.extui %4 : i1 to i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %6 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %6 : i64
  }
  func.func @sloth_main_Vec2__show(%arg0: i64) -> i64 {
    %c41_i64 = arith.constant 41 : i64
    %c2_i64 = arith.constant 2 : i64
    %c8236_i64 = arith.constant 8236 : i64
    %c1_i64 = arith.constant 1 : i64
    %c40_i64 = arith.constant 40 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = call @sloth_str_push(%c0_i64, %c40_i64, %c1_i64) : (i64, i64, i64) -> i64
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %3 : memref<11xi64> -> index
    %4 = arith.index_cast %intptr : index to i64
    %5 = call @sloth_any_from(%4, %2) : (i64, i64) -> i64
    %6 = call @sloth_rt_write(%5) : (i64) -> i64
    %7 = call @sloth_str_pushp(%0, %6) : (i64, i64) -> i64
    %8 = call @sloth_str_push(%7, %c8236_i64, %c2_i64) : (i64, i64, i64) -> i64
    %9 = memref.load %alloca_1[%c0] : memref<1xi64>
    %10 = call @sloth_obj_field(%9, %c1_i64) : (i64, i64) -> i64
    %11 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %11 : memref<11xi64> -> index
    %12 = arith.index_cast %intptr_2 : index to i64
    %13 = call @sloth_any_from(%12, %10) : (i64, i64) -> i64
    %14 = call @sloth_rt_write(%13) : (i64) -> i64
    %15 = call @sloth_str_pushp(%8, %14) : (i64, i64) -> i64
    %16 = call @sloth_str_push(%15, %c41_i64, %c1_i64) : (i64, i64, i64) -> i64
    %17 = call @sloth_str_finish(%16) : (i64) -> i64
    memref.store %17, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %18 = call @sloth_rc_release(%5) : (i64) -> i64
    %19 = call @sloth_rc_release(%6) : (i64) -> i64
    %20 = call @sloth_rc_release(%13) : (i64) -> i64
    %21 = call @sloth_rc_release(%14) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %22 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %22 : i64
  }
  func.func @sloth_main_Bag____init__(%arg0: i64) {
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %0 = call @sloth_arr_new(%c3_i64) : (i64) -> i64
    %1 = call @sloth_arr_set(%0, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %2 = call @sloth_arr_set(%0, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %3 = call @sloth_arr_set(%0, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %4 = memref.load %alloca[%c0] : memref<1xi64>
    %5 = call @sloth_obj_field(%4, %c0_i64) : (i64, i64) -> i64
    %6 = call @sloth_rc_release(%5) : (i64) -> i64
    %7 = call @sloth_rc_retain(%0) : (i64) -> i64
    %8 = call @sloth_obj_set_field(%4, %c0_i64, %7) : (i64, i64, i64) -> i64
    %9 = call @sloth_rc_release(%0) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Bag____index__(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = call @sloth_arr_get(%1, %2) : (i64, i64) -> i64
    memref.store %3, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %4 : i64
  }
  func.func @sloth_main_Bag____assign__(%arg0: i64, %arg1: i64, %arg2: i64) {
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_0[%c0] : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg2, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca[%c0] : memref<1xi64>
    %2 = call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    %4 = call @sloth_arr_set(%2, %3, %0) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_4 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %c2_i64 = arith.constant 2 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %2 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %3 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c4_i64, %3[%c0] : memref<11xi64>
    memref.store %c1_i64, %3[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %3[%c2] : memref<11xi64>
    %4 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %4, %3[%c3] : memref<11xi64>
    memref.store %c3_i64, %3[%c4] : memref<11xi64>
    memref.store %c0_i64, %3[%c9] : memref<11xi64>
    memref.store %c0_i64, %3[%c10] : memref<11xi64>
    %5 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c1_i64, %5[%c0] : memref<11xi64>
    memref.store %c0_i64, %5[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %5[%c2] : memref<11xi64>
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %6, %5[%c3] : memref<11xi64>
    memref.store %c4_i64, %5[%c4] : memref<11xi64>
    memref.store %c0_i64, %5[%c9] : memref<11xi64>
    memref.store %c0_i64, %5[%c10] : memref<11xi64>
    %7 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    memref.store %c2_i64, %7[%c0] : memref<11xi64>
    memref.store %c0_i64, %7[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %7[%c2] : memref<11xi64>
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %8, %7[%c3] : memref<11xi64>
    memref.store %c3_i64, %7[%c4] : memref<11xi64>
    memref.store %c0_i64, %7[%c9] : memref<11xi64>
    memref.store %c0_i64, %7[%c10] : memref<11xi64>
    return
  }
}

