module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Dog\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("Animal\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Dog : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Animal : memref<1xi64> = dense<0> {mutable}
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
  func.func private @sloth_vtb_main_Dog() -> i64 {
    %c3_i64 = arith.constant 3 : i64
    %0 = llvm.mlir.addressof @sloth_main_Dog__who : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %1 = memref.get_global @sloth_main_g_vtb_Dog : memref<1xi64>
    %2 = memref.load %1[%c0] : memref<1xi64>
    %3 = arith.cmpi eq, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = call @sloth_vt_new(%c4_i64) : (i64) -> i64
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %6 = call @sloth_vt_set(%4, %c3_i64, %5) : (i64, i64, i64) -> i64
    memref.store %4, %1[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %7 = memref.load %1[%c0] : memref<1xi64>
    return %7 : i64
  }
  func.func private @sloth_vtb_main_Animal() -> i64 {
    %c3_i64 = arith.constant 3 : i64
    %0 = llvm.mlir.addressof @sloth_main_Animal__who : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %1 = memref.get_global @sloth_main_g_vtb_Animal : memref<1xi64>
    %2 = memref.load %1[%c0] : memref<1xi64>
    %3 = arith.cmpi eq, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = call @sloth_vt_new(%c4_i64) : (i64) -> i64
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %6 = call @sloth_vt_set(%4, %c3_i64, %5) : (i64, i64, i64) -> i64
    memref.store %4, %1[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %7 = memref.load %1[%c0] : memref<1xi64>
    return %7 : i64
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
    %5 = call @sloth_obj_new(%2, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_vtb_main_Dog() : () -> i64
    %7 = call @sloth_obj_set_vtable(%5, %6) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%5, %c3_i64) : (i64, i64) -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %8 = call @sloth_rc_retain(%5) : (i64) -> i64
    memref.store %8, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %9 = arith.index_cast %intptr : index to i64
    %10 = call @sloth_fiber_track(%9) : (i64) -> i64
    %11 = call @sloth_rc_release(%5) : (i64) -> i64
    %12 = memref.load %alloca[%c0] : memref<1xi64>
    %13 = call @sloth_obj_field(%12, %c0_i64) : (i64, i64) -> i64
    %14 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %14 : memref<11xi64> -> index
    %15 = arith.index_cast %intptr_0 : index to i64
    %16 = call @sloth_any_from(%15, %13) : (i64, i64) -> i64
    call @sloth_main__print(%16) : (i64) -> ()
    %17 = call @sloth_rc_release(%16) : (i64) -> i64
    %18 = memref.load %alloca[%c0] : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    %19 = call @sloth_obj_vtable(%18) : (i64) -> i64
    %20 = call @sloth_vt_get(%19, %c3_i64) : (i64, i64) -> i64
    %21 = arith.cmpi ne, %20, %c0_i64 : i64
    cf.cond_br %21, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %22 = llvm.inttoptr %20 : i64 to !llvm.ptr
    %23 = llvm.call %22(%18) : !llvm.ptr, (i64) -> i64
    memref.store %23, %alloca_1[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    %24 = call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_1[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %25 = memref.load %alloca_1[%c0] : memref<1xi64>
    %26 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %26 : memref<11xi64> -> index
    %27 = arith.index_cast %intptr_2 : index to i64
    %28 = call @sloth_any_from(%27, %25) : (i64, i64) -> i64
    call @sloth_main__print(%28) : (i64) -> ()
    %29 = call @sloth_rc_release(%28) : (i64) -> i64
    %30 = call @sloth_rc_release(%25) : (i64) -> i64
    %31 = memref.load %alloca[%c0] : memref<1xi64>
    %alloca_3 = memref.alloca() : memref<1xi64>
    %32 = call @sloth_rc_retain(%31) : (i64) -> i64
    memref.store %32, %alloca_3[%c0] : memref<1xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %alloca_3 : memref<1xi64> -> index
    %33 = arith.index_cast %intptr_4 : index to i64
    %34 = call @sloth_fiber_track(%33) : (i64) -> i64
    %35 = memref.load %alloca_3[%c0] : memref<1xi64>
    %alloca_5 = memref.alloca() : memref<1xi64>
    %36 = call @sloth_obj_vtable(%35) : (i64) -> i64
    %37 = call @sloth_vt_get(%36, %c3_i64) : (i64, i64) -> i64
    %38 = arith.cmpi ne, %37, %c0_i64 : i64
    cf.cond_br %38, ^bb4, ^bb5
  ^bb4:  // pred: ^bb3
    %39 = llvm.inttoptr %37 : i64 to !llvm.ptr
    %40 = llvm.call %39(%35) : !llvm.ptr, (i64) -> i64
    memref.store %40, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb5:  // pred: ^bb3
    %41 = call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb6:  // 2 preds: ^bb4, ^bb5
    %42 = memref.load %alloca_5[%c0] : memref<1xi64>
    %43 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %43 : memref<11xi64> -> index
    %44 = arith.index_cast %intptr_6 : index to i64
    %45 = call @sloth_any_from(%44, %42) : (i64, i64) -> i64
    call @sloth_main__print(%45) : (i64) -> ()
    %46 = call @sloth_rc_release(%45) : (i64) -> i64
    %47 = call @sloth_rc_release(%42) : (i64) -> i64
    %48 = memref.load %alloca_3[%c0] : memref<1xi64>
    %49 = call @sloth_obj_cls_id(%48) : (i64) -> i64
    %50 = arith.cmpi eq, %49, %c3_i64 : i64
    cf.cond_br %50, ^bb7, ^bb8
  ^bb7:  // pred: ^bb6
    %51 = memref.load %alloca_3[%c0] : memref<1xi64>
    %52 = call @sloth_obj_field(%51, %c1_i64) : (i64, i64) -> i64
    %53 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %53 : memref<11xi64> -> index
    %54 = arith.index_cast %intptr_7 : index to i64
    %55 = call @sloth_any_from(%54, %52) : (i64, i64) -> i64
    call @sloth_main__print(%55) : (i64) -> ()
    %56 = call @sloth_rc_release(%55) : (i64) -> i64
    cf.br ^bb9
  ^bb8:  // pred: ^bb6
    cf.br ^bb9
  ^bb9:  // 2 preds: ^bb7, ^bb8
    %57 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %58 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %59 = call @sloth_cls_name(%57, %58, %c6_i64) : (i64, i64, i64) -> i64
    %60 = call @sloth_obj_new(%57, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %61 = call @sloth_vtb_main_Animal() : () -> i64
    %62 = call @sloth_obj_set_vtable(%60, %61) : (i64, i64) -> i64
    call @sloth_main_Animal____init__(%60, %c1_i64) : (i64, i64) -> ()
    %63 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %64 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %65 = call @sloth_cls_name(%63, %64, %c3_i64) : (i64, i64, i64) -> i64
    %66 = call @sloth_obj_new(%63, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %67 = call @sloth_vtb_main_Dog() : () -> i64
    %68 = call @sloth_obj_set_vtable(%66, %67) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%66, %c2_i64) : (i64, i64) -> ()
    %69 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %70 = call @sloth_rc_retain(%60) : (i64) -> i64
    %71 = call @sloth_arr_set(%69, %c0_i64, %70) : (i64, i64, i64) -> i64
    %72 = call @sloth_rc_retain(%66) : (i64) -> i64
    %73 = call @sloth_arr_set(%69, %c1_i64, %72) : (i64, i64, i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    %74 = call @sloth_rc_retain(%69) : (i64) -> i64
    memref.store %74, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %75 = arith.index_cast %intptr_9 : index to i64
    %76 = call @sloth_fiber_track(%75) : (i64) -> i64
    %77 = call @sloth_rc_release(%60) : (i64) -> i64
    %78 = call @sloth_rc_release(%66) : (i64) -> i64
    %79 = call @sloth_rc_release(%69) : (i64) -> i64
    %80 = memref.load %alloca_8[%c0] : memref<1xi64>
    %81 = call @sloth_arr_len(%80) : (i64) -> i64
    %82 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %82 : memref<11xi64> -> index
    %83 = arith.index_cast %intptr_10 : index to i64
    %84 = call @sloth_any_from(%83, %81) : (i64, i64) -> i64
    call @sloth_main__print(%84) : (i64) -> ()
    %85 = call @sloth_rc_release(%84) : (i64) -> i64
    %86 = memref.load %alloca_8[%c0] : memref<1xi64>
    %87 = call @sloth_arr_get(%86, %c1_i64) : (i64, i64) -> i64
    %alloca_11 = memref.alloca() : memref<1xi64>
    %88 = call @sloth_obj_vtable(%87) : (i64) -> i64
    %89 = call @sloth_vt_get(%88, %c3_i64) : (i64, i64) -> i64
    %90 = arith.cmpi ne, %89, %c0_i64 : i64
    cf.cond_br %90, ^bb10, ^bb11
  ^bb10:  // pred: ^bb9
    %91 = llvm.inttoptr %89 : i64 to !llvm.ptr
    %92 = llvm.call %91(%87) : !llvm.ptr, (i64) -> i64
    memref.store %92, %alloca_11[%c0] : memref<1xi64>
    cf.br ^bb12
  ^bb11:  // pred: ^bb9
    %93 = call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_11[%c0] : memref<1xi64>
    cf.br ^bb12
  ^bb12:  // 2 preds: ^bb10, ^bb11
    %94 = memref.load %alloca_11[%c0] : memref<1xi64>
    %95 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %95 : memref<11xi64> -> index
    %96 = arith.index_cast %intptr_12 : index to i64
    %97 = call @sloth_any_from(%96, %94) : (i64, i64) -> i64
    call @sloth_main__print(%97) : (i64) -> ()
    %98 = call @sloth_rc_release(%97) : (i64) -> i64
    %99 = call @sloth_rc_release(%94) : (i64) -> i64
    %100 = memref.load %alloca_3[%c0] : memref<1xi64>
    %101 = call @sloth_rc_release(%100) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_3 : memref<1xi64> -> index
    %102 = arith.index_cast %intptr_13 : index to i64
    %103 = call @sloth_fiber_untrack(%102) : (i64) -> i64
    %104 = memref.load %alloca[%c0] : memref<1xi64>
    %105 = call @sloth_rc_release(%104) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %106 = arith.index_cast %intptr_14 : index to i64
    %107 = call @sloth_fiber_untrack(%106) : (i64) -> i64
    %108 = memref.load %alloca_8[%c0] : memref<1xi64>
    %109 = call @sloth_rc_release(%108) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %110 = arith.index_cast %intptr_15 : index to i64
    %111 = call @sloth_fiber_untrack(%110) : (i64) -> i64
    cf.br ^bb13
  ^bb13:  // pred: ^bb12
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
  llvm.func @sloth_main_Animal__who(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c6_i64 = arith.constant 6 : i64
    %c119165703253601_i64 = arith.constant 119165703253601 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = func.call @sloth_str_push(%c0_i64, %c119165703253601_i64, %c6_i64) : (i64, i64, i64) -> i64
    %1 = func.call @sloth_str_finish(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
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
  llvm.func @sloth_main_Dog__who(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c6778724_i64 = arith.constant 6778724 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = func.call @sloth_str_push(%c0_i64, %c6778724_i64, %c3_i64) : (i64, i64, i64) -> i64
    %1 = func.call @sloth_str_finish(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
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

