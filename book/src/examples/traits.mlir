module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Cat\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("Dog\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Cat : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Dog : memref<1xi64> = dense<0> {mutable}
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
  func.func @sloth_main__announce(%arg0: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    %1 = call @sloth_obj_vtable(%0) : (i64) -> i64
    %2 = call @sloth_vt_get(%1, %c1_i64) : (i64, i64) -> i64
    %3 = arith.cmpi ne, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = llvm.inttoptr %2 : i64 to !llvm.ptr
    %5 = llvm.call %4(%0) : !llvm.ptr, (i64) -> i64
    memref.store %5, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    %6 = call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %7 = memref.load %alloca_2[%c0] : memref<1xi64>
    memref.store %7, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // pred: ^bb3
    %8 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %8 : i64
  }
  func.func private @sloth_vtb_main_Cat() -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.mlir.addressof @sloth_main_Cat__say : !llvm.ptr
    %1 = llvm.mlir.addressof @sloth_main_Cat__name : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %2 = memref.get_global @sloth_main_g_vtb_Cat : memref<1xi64>
    %3 = memref.load %2[%c0] : memref<1xi64>
    %4 = arith.cmpi eq, %3, %c0_i64 : i64
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %5 = call @sloth_vt_new(%c3_i64) : (i64) -> i64
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %7 = call @sloth_vt_set(%5, %c0_i64, %6) : (i64, i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = call @sloth_vt_set(%5, %c1_i64, %8) : (i64, i64, i64) -> i64
    memref.store %5, %2[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %10 = memref.load %2[%c0] : memref<1xi64>
    return %10 : i64
  }
  func.func private @sloth_vtb_main_Dog() -> i64 {
    %c2_i64 = arith.constant 2 : i64
    %0 = llvm.mlir.addressof @sloth_main_Dog__to_str : !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %1 = llvm.mlir.addressof @sloth_main_Dog__say : !llvm.ptr
    %2 = llvm.mlir.addressof @sloth_main_Dog__name : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %3 = memref.get_global @sloth_main_g_vtb_Dog : memref<1xi64>
    %4 = memref.load %3[%c0] : memref<1xi64>
    %5 = arith.cmpi eq, %4, %c0_i64 : i64
    cf.cond_br %5, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %6 = call @sloth_vt_new(%c3_i64) : (i64) -> i64
    %7 = llvm.ptrtoint %2 : !llvm.ptr to i64
    %8 = call @sloth_vt_set(%6, %c0_i64, %7) : (i64, i64, i64) -> i64
    %9 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %10 = call @sloth_vt_set(%6, %c1_i64, %9) : (i64, i64, i64) -> i64
    %11 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %12 = call @sloth_vt_set(%6, %c2_i64, %11) : (i64, i64, i64) -> i64
    memref.store %6, %3[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %13 = memref.load %3[%c0] : memref<1xi64>
    return %13 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @sloth_cls_name(%2, %3, %c3_i64) : (i64, i64, i64) -> i64
    %5 = call @sloth_cls_refmask(%2, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_obj_new(%2, %c0_i64) : (i64, i64) -> i64
    %7 = call @sloth_vtb_main_Cat() : () -> i64
    %8 = call @sloth_obj_set_vtable(%6, %7) : (i64, i64) -> i64
    %9 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %10 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %11 = call @sloth_cls_name(%9, %10, %c3_i64) : (i64, i64, i64) -> i64
    %12 = call @sloth_cls_refmask(%9, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %13 = call @sloth_obj_new(%9, %c0_i64) : (i64, i64) -> i64
    %14 = call @sloth_vtb_main_Dog() : () -> i64
    %15 = call @sloth_obj_set_vtable(%13, %14) : (i64, i64) -> i64
    %16 = call @sloth_arr_new_k(%c2_i64, %c1_i64) : (i64, i64) -> i64
    %17 = call @sloth_rc_retain(%6) : (i64) -> i64
    %18 = call @sloth_arr_set(%16, %c0_i64, %17) : (i64, i64, i64) -> i64
    %19 = call @sloth_rc_retain(%13) : (i64) -> i64
    %20 = call @sloth_arr_set(%16, %c1_i64, %19) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %21 = call @sloth_rc_retain(%16) : (i64) -> i64
    memref.store %21, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %22 = arith.index_cast %intptr : index to i64
    %23 = call @sloth_fiber_track(%22) : (i64) -> i64
    %24 = call @sloth_rc_release(%6) : (i64) -> i64
    %25 = call @sloth_rc_release(%13) : (i64) -> i64
    %26 = call @sloth_rc_release(%16) : (i64) -> i64
    %27 = memref.load %alloca[%c0] : memref<1xi64>
    %28 = call @sloth_arr_len(%27) : (i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_0[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb6
    %29 = memref.load %alloca_0[%c0] : memref<1xi64>
    %30 = arith.cmpi slt, %29, %28 : i64
    cf.cond_br %30, ^bb2, ^bb7
  ^bb2:  // pred: ^bb1
    %31 = call @sloth_arr_get(%27, %29) : (i64, i64) -> i64
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %31, %alloca_1[%c0] : memref<1xi64>
    %32 = memref.load %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    %33 = call @sloth_obj_vtable(%32) : (i64) -> i64
    %34 = call @sloth_vt_get(%33, %c1_i64) : (i64, i64) -> i64
    %35 = arith.cmpi ne, %34, %c0_i64 : i64
    cf.cond_br %35, ^bb3, ^bb4
  ^bb3:  // pred: ^bb2
    %36 = llvm.inttoptr %34 : i64 to !llvm.ptr
    %37 = llvm.call %36(%32) : !llvm.ptr, (i64) -> i64
    memref.store %37, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb4:  // pred: ^bb2
    %38 = call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb5:  // 2 preds: ^bb3, ^bb4
    %39 = memref.load %alloca_2[%c0] : memref<1xi64>
    %40 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %40 : memref<11xi64> -> index
    %41 = arith.index_cast %intptr_3 : index to i64
    %42 = call @sloth_any_from(%41, %39) : (i64, i64) -> i64
    call @sloth_main__print(%42) : (i64) -> ()
    %43 = call @sloth_rc_release(%42) : (i64) -> i64
    %44 = call @sloth_rc_release(%39) : (i64) -> i64
    cf.br ^bb6
  ^bb6:  // pred: ^bb5
    %45 = arith.addi %29, %c1_i64 : i64
    memref.store %45, %alloca_0[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb7:  // pred: ^bb1
    %46 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %47 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %48 = call @sloth_cls_name(%46, %47, %c3_i64) : (i64, i64, i64) -> i64
    %49 = call @sloth_cls_refmask(%46, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %50 = call @sloth_obj_new(%46, %c0_i64) : (i64, i64) -> i64
    %51 = call @sloth_vtb_main_Dog() : () -> i64
    %52 = call @sloth_obj_set_vtable(%50, %51) : (i64, i64) -> i64
    %53 = call @sloth_main__announce(%50) : (i64) -> i64
    %54 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %54 : memref<11xi64> -> index
    %55 = arith.index_cast %intptr_4 : index to i64
    %56 = call @sloth_any_from(%55, %53) : (i64, i64) -> i64
    call @sloth_main__print(%56) : (i64) -> ()
    %57 = call @sloth_rc_release(%50) : (i64) -> i64
    %58 = call @sloth_rc_release(%56) : (i64) -> i64
    %59 = call @sloth_rc_release(%53) : (i64) -> i64
    %60 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %61 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %62 = call @sloth_cls_name(%60, %61, %c3_i64) : (i64, i64, i64) -> i64
    %63 = call @sloth_cls_refmask(%60, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %64 = call @sloth_obj_new(%60, %c0_i64) : (i64, i64) -> i64
    %65 = call @sloth_vtb_main_Dog() : () -> i64
    %66 = call @sloth_obj_set_vtable(%64, %65) : (i64, i64) -> i64
    %67 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %67 : memref<11xi64> -> index
    %68 = arith.index_cast %intptr_5 : index to i64
    %69 = call @sloth_any_from(%68, %64) : (i64, i64) -> i64
    %70 = call @sloth_rt_write(%69) : (i64) -> i64
    %71 = call @sloth_str_pushp(%c0_i64, %70) : (i64, i64) -> i64
    %72 = call @sloth_str_finish(%71) : (i64) -> i64
    %73 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %73 : memref<11xi64> -> index
    %74 = arith.index_cast %intptr_6 : index to i64
    %75 = call @sloth_any_from(%74, %72) : (i64, i64) -> i64
    call @sloth_main__print(%75) : (i64) -> ()
    %76 = call @sloth_rc_release(%64) : (i64) -> i64
    %77 = call @sloth_rc_release(%69) : (i64) -> i64
    %78 = call @sloth_rc_release(%70) : (i64) -> i64
    %79 = call @sloth_rc_release(%72) : (i64) -> i64
    %80 = call @sloth_rc_release(%75) : (i64) -> i64
    %81 = memref.load %alloca[%c0] : memref<1xi64>
    %82 = call @sloth_rc_release(%81) : (i64) -> i64
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %83 = arith.index_cast %intptr_7 : index to i64
    %84 = call @sloth_fiber_untrack(%83) : (i64) -> i64
    cf.br ^bb8
  ^bb8:  // pred: ^bb7
    return
  }
  llvm.func @sloth_main_Cat__name(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c7627107_i64 = arith.constant 7627107 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = func.call @sloth_str_push(%c0_i64, %c7627107_i64, %c3_i64) : (i64, i64, i64) -> i64
    %1 = func.call @sloth_str_finish(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
  }
  llvm.func @sloth_main_Cat__say(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c5_i64 = arith.constant 5 : i64
    %c139274035273_i64 = arith.constant 139274035273 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = func.call @sloth_str_push(%c0_i64, %c139274035273_i64, %c5_i64) : (i64, i64, i64) -> i64
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    %2 = func.call @sloth_obj_vtable(%1) : (i64) -> i64
    %3 = func.call @sloth_vt_get(%2, %c0_i64) : (i64, i64) -> i64
    %4 = arith.cmpi ne, %3, %c0_i64 : i64
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %5 = llvm.inttoptr %3 : i64 to !llvm.ptr
    %6 = llvm.call %5(%1) : !llvm.ptr, (i64) -> i64
    memref.store %6, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    %7 = func.call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %9 : memref<11xi64> -> index
    %10 = arith.index_cast %intptr : index to i64
    %11 = func.call @sloth_any_from(%10, %8) : (i64, i64) -> i64
    %12 = func.call @sloth_rt_write(%11) : (i64) -> i64
    %13 = func.call @sloth_str_pushp(%0, %12) : (i64, i64) -> i64
    %14 = func.call @sloth_str_finish(%13) : (i64) -> i64
    memref.store %14, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %15 = func.call @sloth_rc_release(%11) : (i64) -> i64
    %16 = func.call @sloth_rc_release(%12) : (i64) -> i64
    %17 = func.call @sloth_rc_release(%8) : (i64) -> i64
    cf.br ^bb4
  ^bb4:  // pred: ^bb3
    %18 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %18 : i64
  }
  llvm.func @sloth_main_Dog__name(%arg0: i64) -> i64 {
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
  llvm.func @sloth_main_Dog__to_str(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c5_i64 = arith.constant 5 : i64
    %c176771526468_i64 = arith.constant 176771526468 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = func.call @sloth_str_push(%c0_i64, %c176771526468_i64, %c5_i64) : (i64, i64, i64) -> i64
    %1 = func.call @sloth_str_finish(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %2 : i64
  }
  llvm.func @sloth_main_Dog__say(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c5_i64 = arith.constant 5 : i64
    %c139274035273_i64 = arith.constant 139274035273 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = func.call @sloth_str_push(%c0_i64, %c139274035273_i64, %c5_i64) : (i64, i64, i64) -> i64
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    %2 = func.call @sloth_obj_vtable(%1) : (i64) -> i64
    %3 = func.call @sloth_vt_get(%2, %c0_i64) : (i64, i64) -> i64
    %4 = arith.cmpi ne, %3, %c0_i64 : i64
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %5 = llvm.inttoptr %3 : i64 to !llvm.ptr
    %6 = llvm.call %5(%1) : !llvm.ptr, (i64) -> i64
    memref.store %6, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    %7 = func.call @sloth_panic_noimpl(%c0_i64) : (i64) -> i64
    memref.store %c0_i64, %alloca_2[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %8 = memref.load %alloca_2[%c0] : memref<1xi64>
    %9 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %9 : memref<11xi64> -> index
    %10 = arith.index_cast %intptr : index to i64
    %11 = func.call @sloth_any_from(%10, %8) : (i64, i64) -> i64
    %12 = func.call @sloth_rt_write(%11) : (i64) -> i64
    %13 = func.call @sloth_str_pushp(%0, %12) : (i64, i64) -> i64
    %14 = func.call @sloth_str_finish(%13) : (i64) -> i64
    memref.store %14, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %15 = func.call @sloth_rc_release(%11) : (i64) -> i64
    %16 = func.call @sloth_rc_release(%12) : (i64) -> i64
    %17 = func.call @sloth_rc_release(%8) : (i64) -> i64
    cf.br ^bb4
  ^bb4:  // pred: ^bb3
    %18 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %18 : i64
  }
  llvm.func @sloth_anydisp_1(%arg0: i64) -> i64 {
    %c2_i64 = arith.constant 2 : i64
    %0 = func.call @sloth_obj_vtable(%arg0) : (i64) -> i64
    %1 = func.call @sloth_vt_get(%0, %c2_i64) : (i64, i64) -> i64
    %2 = llvm.inttoptr %1 : i64 to !llvm.ptr
    %3 = llvm.call %2(%arg0) : !llvm.ptr, (i64) -> i64
    llvm.return %3 : i64
  }
  func.func @sloth_main__anyinit() {
    %c8 = arith.constant 8 : index
    %0 = llvm.mlir.addressof @sloth_anydisp_1 : !llvm.ptr
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c11_i64 = arith.constant 11 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %2 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
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
    memref.store %c11_i64, %5[%c0] : memref<11xi64>
    memref.store %c1_i64, %5[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %5[%c2] : memref<11xi64>
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %6, %5[%c3] : memref<11xi64>
    memref.store %c3_i64, %5[%c4] : memref<11xi64>
    %7 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %7, %5[%c8] : memref<11xi64>
    memref.store %c0_i64, %5[%c9] : memref<11xi64>
    memref.store %c0_i64, %5[%c10] : memref<11xi64>
    return
  }
}

