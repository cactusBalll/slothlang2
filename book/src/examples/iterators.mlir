module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Range3\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("Entry<int, int>\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Range3 : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Entry_int_int : memref<1xi64> = dense<0> {mutable}
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
  func.func private @sloth_vtb_main_Range3() -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.mlir.addressof @sloth_main_Range3__next : !llvm.ptr
    %1 = llvm.mlir.addressof @sloth_main_Range3__iter : !llvm.ptr
    %c5_i64 = arith.constant 5 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %2 = memref.get_global @sloth_main_g_vtb_Range3 : memref<1xi64>
    %3 = memref.load %2[%c0] : memref<1xi64>
    %4 = arith.cmpi eq, %3, %c0_i64 : i64
    cf.cond_br %4, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %5 = call @__sloth_vt_new(%c5_i64) : (i64) -> i64
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %7 = call @__sloth_vt_set(%5, %c0_i64, %6) : (i64, i64, i64) -> i64
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %9 = call @__sloth_vt_set(%5, %c1_i64, %8) : (i64, i64, i64) -> i64
    memref.store %5, %2[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %10 = memref.load %2[%c0] : memref<1xi64>
    return %10 : i64
  }
  func.func private @sloth_vtb_main_Entry_int_int() -> i64 {
    %c5_i64 = arith.constant 5 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %0 = memref.get_global @sloth_main_g_vtb_Entry_int_int : memref<1xi64>
    %1 = memref.load %0[%c0] : memref<1xi64>
    %2 = arith.cmpi eq, %1, %c0_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = call @__sloth_vt_new(%c5_i64) : (i64) -> i64
    memref.store %3, %0[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %4 = memref.load %0[%c0] : memref<1xi64>
    return %4 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c2_i64 = arith.constant 2 : i64
    %c15_i64 = arith.constant 15 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c4_i64 = arith.constant 4 : i64
    %c10_i64 = arith.constant 10 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %c6_i64 = arith.constant 6 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @__sloth_cls_name(%2, %3, %c6_i64) : (i64, i64, i64) -> i64
    %5 = call @__sloth_obj_new(%2, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_vtb_main_Range3() : () -> i64
    %7 = call @__sloth_obj_set_vtable(%5, %6) : (i64, i64) -> i64
    %8 = call @__sloth_obj_set_field(%5, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %9 = call @__sloth_rc_retain(%5) : (i64) -> i64
    memref.store %9, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %10 = arith.index_cast %intptr : index to i64
    %11 = call @__sloth_fiber_track(%10) : (i64) -> i64
    %12 = call @__sloth_rc_release(%5) : (i64) -> i64
    %13 = memref.load %alloca[%c0] : memref<1xi64>
    %14 = llvm.call @sloth_main_Range3__iter(%13) : (i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %14, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %15 = arith.index_cast %intptr_1 : index to i64
    %16 = call @__sloth_fiber_track(%15) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %17 = memref.load %alloca_0[%c0] : memref<1xi64>
    %18 = llvm.call @sloth_main_Range3__next(%17) : (i64) -> i64
    %19 = arith.cmpi eq, %18, %c0_i64 : i64
    cf.cond_br %19, ^bb4, ^bb2
  ^bb2:  // pred: ^bb1
    %20 = call @__sloth_box_get(%18) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %20, %alloca_2[%c0] : memref<1xi64>
    %21 = call @__sloth_rc_release(%18) : (i64) -> i64
    %22 = memref.load %alloca_2[%c0] : memref<1xi64>
    %23 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %23 : memref<11xi64> -> index
    %24 = arith.index_cast %intptr_3 : index to i64
    %25 = call @__sloth_any_from(%24, %22) : (i64, i64) -> i64
    call @sloth_main__print(%25) : (i64) -> ()
    %26 = call @__sloth_rc_release(%25) : (i64) -> i64
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %27 = memref.load %alloca_0[%c0] : memref<1xi64>
    %28 = call @__sloth_rc_release(%27) : (i64) -> i64
    %intptr_4 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %29 = arith.index_cast %intptr_4 : index to i64
    %30 = call @__sloth_fiber_untrack(%29) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    %alloca_6 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb5:  // 2 preds: ^bb4, ^bb7
    %31 = memref.load %alloca_6[%c0] : memref<1xi64>
    %32 = arith.cmpi slt, %31, %c4_i64 : i64
    cf.cond_br %32, ^bb6, ^bb8
  ^bb6:  // pred: ^bb5
    %alloca_7 = memref.alloca() : memref<1xi64>
    memref.store %31, %alloca_7[%c0] : memref<1xi64>
    %33 = memref.load %alloca_5[%c0] : memref<1xi64>
    %34 = memref.load %alloca_7[%c0] : memref<1xi64>
    %35 = arith.addi %33, %34 : i64
    memref.store %35, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb7
  ^bb7:  // pred: ^bb6
    %36 = arith.addi %31, %c1_i64 : i64
    memref.store %36, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb8:  // pred: ^bb5
    %37 = memref.load %alloca_5[%c0] : memref<1xi64>
    %38 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %38 : memref<11xi64> -> index
    %39 = arith.index_cast %intptr_8 : index to i64
    %40 = call @__sloth_any_from(%39, %37) : (i64, i64) -> i64
    call @sloth_main__print(%40) : (i64) -> ()
    %41 = call @__sloth_rc_release(%40) : (i64) -> i64
    %42 = call @__sloth_map_new(%c0_i64) : (i64) -> i64
    %43 = call @__sloth_map_set(%42, %c1_i64, %c10_i64) : (i64, i64, i64) -> i64
    %44 = call @__sloth_map_keys(%42) : (i64) -> i64
    %45 = call @__sloth_arr_len(%44) : (i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_9[%c0] : memref<1xi64>
    cf.br ^bb9
  ^bb9:  // 2 preds: ^bb8, ^bb11
    %46 = memref.load %alloca_9[%c0] : memref<1xi64>
    %47 = arith.cmpi slt, %46, %45 : i64
    cf.cond_br %47, ^bb10, ^bb12
  ^bb10:  // pred: ^bb9
    %48 = call @__sloth_arr_get(%44, %46) : (i64, i64) -> i64
    %49 = call @__sloth_map_get(%42, %48) : (i64, i64) -> i64
    %50 = call @__sloth_cls_info(%c0_i64, %c4_i64) : (i64, i64) -> i64
    %51 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %52 = call @__sloth_cls_name(%50, %51, %c15_i64) : (i64, i64, i64) -> i64
    %53 = call @__sloth_obj_new(%50, %c2_i64, %c0_i64) : (i64, i64, i64) -> i64
    %54 = call @sloth_vtb_main_Entry_int_int() : () -> i64
    %55 = call @__sloth_obj_set_vtable(%53, %54) : (i64, i64) -> i64
    %56 = call @__sloth_obj_set_field(%53, %c0_i64, %48) : (i64, i64, i64) -> i64
    %57 = call @__sloth_obj_set_field(%53, %c1_i64, %49) : (i64, i64, i64) -> i64
    %alloca_10 = memref.alloca() : memref<1xi64>
    memref.store %53, %alloca_10[%c0] : memref<1xi64>
    %58 = memref.load %alloca_10[%c0] : memref<1xi64>
    %59 = call @__sloth_obj_field(%58, %c0_i64) : (i64, i64) -> i64
    %60 = memref.load %alloca_10[%c0] : memref<1xi64>
    %61 = call @__sloth_obj_field(%60, %c1_i64) : (i64, i64) -> i64
    %62 = arith.addi %59, %61 : i64
    %63 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %63 : memref<11xi64> -> index
    %64 = arith.index_cast %intptr_11 : index to i64
    %65 = call @__sloth_any_from(%64, %62) : (i64, i64) -> i64
    call @sloth_main__print(%65) : (i64) -> ()
    %66 = call @__sloth_rc_release(%65) : (i64) -> i64
    cf.br ^bb11
  ^bb11:  // pred: ^bb10
    %67 = call @__sloth_rc_release(%53) : (i64) -> i64
    %68 = arith.addi %46, %c1_i64 : i64
    memref.store %68, %alloca_9[%c0] : memref<1xi64>
    cf.br ^bb9
  ^bb12:  // pred: ^bb9
    %69 = call @__sloth_rc_release(%44) : (i64) -> i64
    %70 = call @__sloth_rc_release(%42) : (i64) -> i64
    %71 = memref.load %alloca[%c0] : memref<1xi64>
    %72 = call @__sloth_rc_release(%71) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %73 = arith.index_cast %intptr_12 : index to i64
    %74 = call @__sloth_fiber_untrack(%73) : (i64) -> i64
    cf.br ^bb13
  ^bb13:  // pred: ^bb12
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
  llvm.func @sloth_main_Range3__iter(%arg0: i64) -> i64 {
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
  llvm.func @sloth_main_Range3__next(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = arith.cmpi sge, %1, %c3_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = func.call @__sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %3, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = func.call @__sloth_obj_field(%4, %c0_i64) : (i64, i64) -> i64
    %6 = arith.addi %5, %c1_i64 : i64
    %7 = memref.load %alloca_1[%c0] : memref<1xi64>
    %8 = func.call @__sloth_obj_set_field(%7, %c0_i64, %6) : (i64, i64, i64) -> i64
    %9 = memref.load %alloca_1[%c0] : memref<1xi64>
    %10 = func.call @__sloth_obj_field(%9, %c0_i64) : (i64, i64) -> i64
    %11 = arith.subi %10, %c1_i64 : i64
    %12 = func.call @__sloth_box_new(%11) : (i64) -> i64
    memref.store %12, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %13 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %13 : i64
  }
  func.func @sloth_main__anyinit() {
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %1 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c2_i64, %1[%c0] : memref<11xi64>
    memref.store %c0_i64, %1[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %1[%c2] : memref<11xi64>
    %2 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %2, %1[%c3] : memref<11xi64>
    memref.store %c3_i64, %1[%c4] : memref<11xi64>
    memref.store %c0_i64, %1[%c9] : memref<11xi64>
    memref.store %c0_i64, %1[%c10] : memref<11xi64>
    return
  }
}

