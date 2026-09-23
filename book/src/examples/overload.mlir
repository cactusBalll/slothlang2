module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Vec2\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("Bag\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_4("int\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_2 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Vec2 : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Bag : memref<1xi64> = dense<0> {mutable}
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
  func.func private @sloth_vtb_main_Vec2() -> i64 {
    %c8_i64 = arith.constant 8 : i64
    %0 = llvm.mlir.addressof @sloth_main_Vec2__show : !llvm.ptr
    %c7_i64 = arith.constant 7 : i64
    %1 = llvm.mlir.addressof @sloth_main_Vec2____lt__ : !llvm.ptr
    %c6_i64 = arith.constant 6 : i64
    %2 = llvm.mlir.addressof @sloth_main_Vec2____eq__ : !llvm.ptr
    %c5_i64 = arith.constant 5 : i64
    %3 = llvm.mlir.addressof @sloth_main_Vec2____neg__ : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %4 = llvm.mlir.addressof @sloth_main_Vec2____sub__ : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %5 = llvm.mlir.addressof @sloth_main_Vec2____add__ : !llvm.ptr
    %c11_i64 = arith.constant 11 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %6 = memref.get_global @sloth_main_g_vtb_Vec2 : memref<1xi64>
    %7 = memref.load %6[%c0] : memref<1xi64>
    %8 = arith.cmpi eq, %7, %c0_i64 : i64
    cf.cond_br %8, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %9 = call @sloth_vt_new(%c11_i64) : (i64) -> i64
    %10 = llvm.ptrtoint %5 : !llvm.ptr to i64
    %11 = call @sloth_vt_set(%9, %c3_i64, %10) : (i64, i64, i64) -> i64
    %12 = llvm.ptrtoint %4 : !llvm.ptr to i64
    %13 = call @sloth_vt_set(%9, %c4_i64, %12) : (i64, i64, i64) -> i64
    %14 = llvm.ptrtoint %3 : !llvm.ptr to i64
    %15 = call @sloth_vt_set(%9, %c5_i64, %14) : (i64, i64, i64) -> i64
    %16 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %17 = call @sloth_vt_set(%9, %c6_i64, %16) : (i64, i64, i64) -> i64
    %18 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %19 = call @sloth_vt_set(%9, %c7_i64, %18) : (i64, i64, i64) -> i64
    %20 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %21 = call @sloth_vt_set(%9, %c8_i64, %20) : (i64, i64, i64) -> i64
    memref.store %9, %6[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %22 = memref.load %6[%c0] : memref<1xi64>
    return %22 : i64
  }
  llvm.func @sloth_main_Bag__cascade(%arg0: i64, %arg1: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @sloth_rc_release(%0) : (i64) -> i64
    llvm.return %c0_i64 : i64
  }
  func.func private @sloth_vtb_main_Bag() -> i64 {
    %c10_i64 = arith.constant 10 : i64
    %0 = llvm.mlir.addressof @sloth_main_Bag____assign__ : !llvm.ptr
    %c9_i64 = arith.constant 9 : i64
    %1 = llvm.mlir.addressof @sloth_main_Bag____index__ : !llvm.ptr
    %c11_i64 = arith.constant 11 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %2 = memref.get_global @sloth_main_g_vtb_Bag : memref<1xi64>
    %3 = memref.load %2[%c0] : memref<1xi64>
    %4 = arith.cmpi eq, %3, %c0_i64 : i64
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %5 = call @sloth_vt_new(%c11_i64) : (i64) -> i64
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %7 = call @sloth_vt_set(%5, %c9_i64, %6) : (i64, i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = call @sloth_vt_set(%5, %c10_i64, %8) : (i64, i64, i64) -> i64
    memref.store %5, %2[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %10 = memref.load %2[%c0] : memref<1xi64>
    return %10 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c8_i64 = arith.constant 8 : i64
    %c7_i64 = arith.constant 7 : i64
    %0 = llvm.mlir.addressof @sloth_main_Bag__cascade : !llvm.ptr
    %1 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %2 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %c1_i64 = arith.constant 1 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %3 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %4 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %5 = call @sloth_cls_name(%3, %4, %c4_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_obj_new(%3, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %7 = call @sloth_vtb_main_Vec2() : () -> i64
    %8 = call @sloth_obj_set_vtable(%6, %7) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%6, %c1_i64, %c2_i64) : (i64, i64, i64) -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %9 = call @sloth_rc_retain(%6) : (i64) -> i64
    memref.store %9, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %10 = arith.index_cast %intptr : index to i64
    %11 = call @sloth_fiber_track(%10) : (i64) -> i64
    %12 = call @sloth_rc_release(%6) : (i64) -> i64
    %13 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %14 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %15 = call @sloth_cls_name(%13, %14, %c4_i64) : (i64, i64, i64) -> i64
    %16 = call @sloth_obj_new(%13, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %17 = call @sloth_vtb_main_Vec2() : () -> i64
    %18 = call @sloth_obj_set_vtable(%16, %17) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%16, %c3_i64, %c4_i64) : (i64, i64, i64) -> ()
    %alloca_0 = memref.alloca() : memref<1xi64>
    %19 = call @sloth_rc_retain(%16) : (i64) -> i64
    memref.store %19, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %20 = arith.index_cast %intptr_1 : index to i64
    %21 = call @sloth_fiber_track(%20) : (i64) -> i64
    %22 = call @sloth_rc_release(%16) : (i64) -> i64
    %23 = memref.load %alloca[%c0] : memref<1xi64>
    %24 = memref.load %alloca_0[%c0] : memref<1xi64>
    %25 = llvm.call @sloth_main_Vec2____add__(%23, %24) : (i64, i64) -> i64
    %26 = llvm.call @sloth_main_Vec2__show(%25) : (i64) -> i64
    %27 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %27 : memref<11xi64> -> index
    %28 = arith.index_cast %intptr_2 : index to i64
    %29 = call @sloth_any_from(%28, %26) : (i64, i64) -> i64
    call @sloth_main__print(%29) : (i64) -> ()
    %30 = call @sloth_rc_release(%29) : (i64) -> i64
    %31 = call @sloth_rc_release(%25) : (i64) -> i64
    %32 = call @sloth_rc_release(%26) : (i64) -> i64
    %33 = memref.load %alloca_0[%c0] : memref<1xi64>
    %34 = memref.load %alloca[%c0] : memref<1xi64>
    %35 = llvm.call @sloth_main_Vec2____sub__(%33, %34) : (i64, i64) -> i64
    %36 = llvm.call @sloth_main_Vec2__show(%35) : (i64) -> i64
    %37 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %37 : memref<11xi64> -> index
    %38 = arith.index_cast %intptr_3 : index to i64
    %39 = call @sloth_any_from(%38, %36) : (i64, i64) -> i64
    call @sloth_main__print(%39) : (i64) -> ()
    %40 = call @sloth_rc_release(%39) : (i64) -> i64
    %41 = call @sloth_rc_release(%35) : (i64) -> i64
    %42 = call @sloth_rc_release(%36) : (i64) -> i64
    %43 = memref.load %alloca[%c0] : memref<1xi64>
    %44 = llvm.call @sloth_main_Vec2____neg__(%43) : (i64) -> i64
    %45 = llvm.call @sloth_main_Vec2__show(%44) : (i64) -> i64
    %46 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %46 : memref<11xi64> -> index
    %47 = arith.index_cast %intptr_4 : index to i64
    %48 = call @sloth_any_from(%47, %45) : (i64, i64) -> i64
    call @sloth_main__print(%48) : (i64) -> ()
    %49 = call @sloth_rc_release(%48) : (i64) -> i64
    %50 = call @sloth_rc_release(%44) : (i64) -> i64
    %51 = call @sloth_rc_release(%45) : (i64) -> i64
    %52 = memref.load %alloca[%c0] : memref<1xi64>
    %53 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %54 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %55 = call @sloth_cls_name(%53, %54, %c4_i64) : (i64, i64, i64) -> i64
    %56 = call @sloth_obj_new(%53, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %57 = call @sloth_vtb_main_Vec2() : () -> i64
    %58 = call @sloth_obj_set_vtable(%56, %57) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%56, %c1_i64, %c2_i64) : (i64, i64, i64) -> ()
    %59 = llvm.call @sloth_main_Vec2____eq__(%52, %56) : (i64, i64) -> i64
    %60 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %60 : memref<11xi64> -> index
    %61 = arith.index_cast %intptr_5 : index to i64
    %62 = call @sloth_any_from(%61, %59) : (i64, i64) -> i64
    call @sloth_main__print(%62) : (i64) -> ()
    %63 = call @sloth_rc_release(%56) : (i64) -> i64
    %64 = call @sloth_rc_release(%62) : (i64) -> i64
    %65 = memref.load %alloca[%c0] : memref<1xi64>
    %66 = memref.load %alloca_0[%c0] : memref<1xi64>
    %67 = llvm.call @sloth_main_Vec2____lt__(%65, %66) : (i64, i64) -> i64
    %68 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %68 : memref<11xi64> -> index
    %69 = arith.index_cast %intptr_6 : index to i64
    %70 = call @sloth_any_from(%69, %67) : (i64, i64) -> i64
    call @sloth_main__print(%70) : (i64) -> ()
    %71 = call @sloth_rc_release(%70) : (i64) -> i64
    %72 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %73 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %74 = call @sloth_cls_name(%72, %73, %c3_i64) : (i64, i64, i64) -> i64
    %75 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %76 = call @sloth_obj_new(%72, %c1_i64, %75) : (i64, i64, i64) -> i64
    %77 = call @sloth_vtb_main_Bag() : () -> i64
    %78 = call @sloth_obj_set_vtable(%76, %77) : (i64, i64) -> i64
    call @sloth_main_Bag____init__(%76) : (i64) -> ()
    %alloca_7 = memref.alloca() : memref<1xi64>
    %79 = call @sloth_rc_retain(%76) : (i64) -> i64
    memref.store %79, %alloca_7[%c0] : memref<1xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %alloca_7 : memref<1xi64> -> index
    %80 = arith.index_cast %intptr_8 : index to i64
    %81 = call @sloth_fiber_track(%80) : (i64) -> i64
    %82 = call @sloth_rc_release(%76) : (i64) -> i64
    %83 = memref.load %alloca_7[%c0] : memref<1xi64>
    llvm.call @sloth_main_Bag____assign__(%83, %c0_i64, %c7_i64) : (i64, i64, i64) -> ()
    %84 = memref.load %alloca_7[%c0] : memref<1xi64>
    llvm.call @sloth_main_Bag____assign__(%84, %c1_i64, %c8_i64) : (i64, i64, i64) -> ()
    %85 = memref.load %alloca_7[%c0] : memref<1xi64>
    %86 = llvm.call @sloth_main_Bag____index__(%85, %c0_i64) : (i64, i64) -> i64
    %87 = memref.load %alloca_7[%c0] : memref<1xi64>
    %88 = llvm.call @sloth_main_Bag____index__(%87, %c1_i64) : (i64, i64) -> i64
    %89 = arith.addi %86, %88 : i64
    %90 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %90 : memref<11xi64> -> index
    %91 = arith.index_cast %intptr_9 : index to i64
    %92 = call @sloth_any_from(%91, %89) : (i64, i64) -> i64
    call @sloth_main__print(%92) : (i64) -> ()
    %93 = call @sloth_rc_release(%92) : (i64) -> i64
    %94 = memref.load %alloca[%c0] : memref<1xi64>
    %95 = call @sloth_rc_release(%94) : (i64) -> i64
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %96 = arith.index_cast %intptr_10 : index to i64
    %97 = call @sloth_fiber_untrack(%96) : (i64) -> i64
    %98 = memref.load %alloca_7[%c0] : memref<1xi64>
    %99 = call @sloth_rc_release(%98) : (i64) -> i64
    %intptr_11 = memref.extract_aligned_pointer_as_index %alloca_7 : memref<1xi64> -> index
    %100 = arith.index_cast %intptr_11 : index to i64
    %101 = call @sloth_fiber_untrack(%100) : (i64) -> i64
    %102 = memref.load %alloca_0[%c0] : memref<1xi64>
    %103 = call @sloth_rc_release(%102) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %104 = arith.index_cast %intptr_12 : index to i64
    %105 = call @sloth_fiber_untrack(%104) : (i64) -> i64
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
  llvm.func @sloth_main_Vec2____add__(%arg0: i64, %arg1: i64) -> i64 {
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
    %2 = func.call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.load %alloca_2[%c0] : memref<1xi64>
    %4 = func.call @sloth_obj_field(%3, %c0_i64) : (i64, i64) -> i64
    %5 = arith.addi %2, %4 : i64
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = func.call @sloth_obj_field(%6, %c1_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = func.call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %10 = arith.addi %7, %9 : i64
    %11 = func.call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %12 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %13 = func.call @sloth_cls_name(%11, %12, %c4_i64) : (i64, i64, i64) -> i64
    %14 = func.call @sloth_obj_new(%11, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %15 = func.call @sloth_vtb_main_Vec2() : () -> i64
    %16 = func.call @sloth_obj_set_vtable(%14, %15) : (i64, i64) -> i64
    func.call @sloth_main_Vec2____init__(%14, %5, %10) : (i64, i64, i64) -> ()
    memref.store %14, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %17 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %17 : i64
  }
  llvm.func @sloth_main_Vec2____sub__(%arg0: i64, %arg1: i64) -> i64 {
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
    %2 = func.call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.load %alloca_2[%c0] : memref<1xi64>
    %4 = func.call @sloth_obj_field(%3, %c0_i64) : (i64, i64) -> i64
    %5 = arith.subi %2, %4 : i64
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = func.call @sloth_obj_field(%6, %c1_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = func.call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %10 = arith.subi %7, %9 : i64
    %11 = func.call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %12 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %13 = func.call @sloth_cls_name(%11, %12, %c4_i64) : (i64, i64, i64) -> i64
    %14 = func.call @sloth_obj_new(%11, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %15 = func.call @sloth_vtb_main_Vec2() : () -> i64
    %16 = func.call @sloth_obj_set_vtable(%14, %15) : (i64, i64) -> i64
    func.call @sloth_main_Vec2____init__(%14, %5, %10) : (i64, i64, i64) -> ()
    memref.store %14, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %17 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %17 : i64
  }
  llvm.func @sloth_main_Vec2____neg__(%arg0: i64) -> i64 {
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
    %2 = func.call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = arith.subi %c0_i64, %2 : i64
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = func.call @sloth_obj_field(%4, %c1_i64) : (i64, i64) -> i64
    %6 = arith.subi %c0_i64, %5 : i64
    %7 = func.call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = func.call @sloth_cls_name(%7, %8, %c4_i64) : (i64, i64, i64) -> i64
    %10 = func.call @sloth_obj_new(%7, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %11 = func.call @sloth_vtb_main_Vec2() : () -> i64
    %12 = func.call @sloth_obj_set_vtable(%10, %11) : (i64, i64) -> i64
    func.call @sloth_main_Vec2____init__(%10, %3, %6) : (i64, i64, i64) -> ()
    memref.store %10, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %13 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %13 : i64
  }
  llvm.func @sloth_main_Vec2____eq__(%arg0: i64, %arg1: i64) -> i64 {
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
    %1 = func.call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = func.call @sloth_obj_field(%2, %c0_i64) : (i64, i64) -> i64
    %4 = arith.cmpi eq, %1, %3 : i64
    %5 = arith.extui %4 : i1 to i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %5, %alloca_3[%c0] : memref<1xi64>
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %6 = memref.load %alloca_1[%c0] : memref<1xi64>
    %7 = func.call @sloth_obj_field(%6, %c1_i64) : (i64, i64) -> i64
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = func.call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
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
    llvm.return %13 : i64
  }
  llvm.func @sloth_main_Vec2____lt__(%arg0: i64, %arg1: i64) -> i64 {
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
    %1 = func.call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = func.call @sloth_obj_field(%2, %c0_i64) : (i64, i64) -> i64
    %4 = arith.cmpi slt, %1, %3 : i64
    %5 = arith.extui %4 : i1 to i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %6 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %6 : i64
  }
  llvm.func @sloth_main_Vec2__show(%arg0: i64) -> i64 {
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
    %0 = func.call @sloth_str_push(%c0_i64, %c40_i64, %c1_i64) : (i64, i64, i64) -> i64
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = func.call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %3 : memref<11xi64> -> index
    %4 = arith.index_cast %intptr : index to i64
    %5 = func.call @sloth_any_from(%4, %2) : (i64, i64) -> i64
    %6 = func.call @sloth_rt_write(%5) : (i64) -> i64
    %7 = func.call @sloth_str_pushp(%0, %6) : (i64, i64) -> i64
    %8 = func.call @sloth_str_push(%7, %c8236_i64, %c2_i64) : (i64, i64, i64) -> i64
    %9 = memref.load %alloca_1[%c0] : memref<1xi64>
    %10 = func.call @sloth_obj_field(%9, %c1_i64) : (i64, i64) -> i64
    %11 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %11 : memref<11xi64> -> index
    %12 = arith.index_cast %intptr_2 : index to i64
    %13 = func.call @sloth_any_from(%12, %10) : (i64, i64) -> i64
    %14 = func.call @sloth_rt_write(%13) : (i64) -> i64
    %15 = func.call @sloth_str_pushp(%8, %14) : (i64, i64) -> i64
    %16 = func.call @sloth_str_push(%15, %c41_i64, %c1_i64) : (i64, i64, i64) -> i64
    %17 = func.call @sloth_str_finish(%16) : (i64) -> i64
    memref.store %17, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %18 = func.call @sloth_rc_release(%5) : (i64) -> i64
    %19 = func.call @sloth_rc_release(%6) : (i64) -> i64
    %20 = func.call @sloth_rc_release(%13) : (i64) -> i64
    %21 = func.call @sloth_rc_release(%14) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %22 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %22 : i64
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
  llvm.func @sloth_main_Bag____index__(%arg0: i64, %arg1: i64) -> i64 {
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
    %1 = func.call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = memref.load %alloca_2[%c0] : memref<1xi64>
    %3 = func.call @sloth_arr_get(%1, %2) : (i64, i64) -> i64
    memref.store %3, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %4 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %4 : i64
  }
  llvm.func @sloth_main_Bag____assign__(%arg0: i64, %arg1: i64, %arg2: i64) {
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
    %2 = func.call @sloth_obj_field(%1, %c0_i64) : (i64, i64) -> i64
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    %4 = func.call @sloth_arr_set(%2, %3, %0) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    llvm.return
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

