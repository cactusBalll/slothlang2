module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Range3\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("Entry<int, int>\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
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
    %c4_i64 = arith.constant 4 : i64
    %c15_i64 = arith.constant 15 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c10_i64 = arith.constant 10 : i64
    %c3_i64 = arith.constant 3 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %c6_i64 = arith.constant 6 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @sloth_cls_name(%2, %3, %c6_i64) : (i64, i64, i64) -> i64
    %5 = call @sloth_cls_refmask(%2, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_obj_new(%2, %c1_i64) : (i64, i64) -> i64
    %7 = call @sloth_obj_set_field(%6, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %8 = call @sloth_rc_retain(%6) : (i64) -> i64
    memref.store %8, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %9 = arith.index_cast %intptr : index to i64
    %10 = call @sloth_fiber_track(%9) : (i64) -> i64
    %11 = call @sloth_rc_release(%6) : (i64) -> i64
    %12 = memref.load %alloca[%c0] : memref<1xi64>
    %13 = call @sloth_main_Range3__iter(%12) : (i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %13, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %14 = arith.index_cast %intptr_1 : index to i64
    %15 = call @sloth_fiber_track(%14) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %16 = memref.load %alloca_0[%c0] : memref<1xi64>
    %17 = call @sloth_main_Range3__next(%16) : (i64) -> i64
    %18 = arith.cmpi eq, %17, %c0_i64 : i64
    cf.cond_br %18, ^bb4, ^bb2
  ^bb2:  // pred: ^bb1
    %19 = call @sloth_box_get(%17) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %19, %alloca_2[%c0] : memref<1xi64>
    %20 = call @sloth_rc_release(%17) : (i64) -> i64
    %21 = memref.load %alloca_2[%c0] : memref<1xi64>
    %22 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %22 : memref<11xi64> -> index
    %23 = arith.index_cast %intptr_3 : index to i64
    %24 = call @sloth_any_from(%23, %21) : (i64, i64) -> i64
    call @sloth_main__print(%24) : (i64) -> ()
    %25 = call @sloth_rc_release(%24) : (i64) -> i64
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %26 = memref.load %alloca_0[%c0] : memref<1xi64>
    %27 = call @sloth_rc_release(%26) : (i64) -> i64
    %intptr_4 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %28 = arith.index_cast %intptr_4 : index to i64
    %29 = call @sloth_fiber_untrack(%28) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    %alloca_6 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb5:  // 2 preds: ^bb4, ^bb7
    %30 = memref.load %alloca_6[%c0] : memref<1xi64>
    %31 = arith.cmpi slt, %30, %c4_i64 : i64
    cf.cond_br %31, ^bb6, ^bb8
  ^bb6:  // pred: ^bb5
    %alloca_7 = memref.alloca() : memref<1xi64>
    memref.store %30, %alloca_7[%c0] : memref<1xi64>
    %32 = memref.load %alloca_5[%c0] : memref<1xi64>
    %33 = memref.load %alloca_7[%c0] : memref<1xi64>
    %34 = arith.addi %32, %33 : i64
    memref.store %34, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb7
  ^bb7:  // pred: ^bb6
    %35 = arith.addi %30, %c1_i64 : i64
    memref.store %35, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb5
  ^bb8:  // pred: ^bb5
    %36 = memref.load %alloca_5[%c0] : memref<1xi64>
    %37 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %37 : memref<11xi64> -> index
    %38 = arith.index_cast %intptr_8 : index to i64
    %39 = call @sloth_any_from(%38, %36) : (i64, i64) -> i64
    call @sloth_main__print(%39) : (i64) -> ()
    %40 = call @sloth_rc_release(%39) : (i64) -> i64
    %41 = call @sloth_map_new(%c0_i64) : (i64) -> i64
    %42 = call @sloth_map_set(%41, %c1_i64, %c10_i64) : (i64, i64, i64) -> i64
    %43 = call @sloth_map_keys(%41) : (i64) -> i64
    %44 = call @sloth_arr_len(%43) : (i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_9[%c0] : memref<1xi64>
    cf.br ^bb9
  ^bb9:  // 2 preds: ^bb8, ^bb11
    %45 = memref.load %alloca_9[%c0] : memref<1xi64>
    %46 = arith.cmpi slt, %45, %44 : i64
    cf.cond_br %46, ^bb10, ^bb12
  ^bb10:  // pred: ^bb9
    %47 = call @sloth_arr_get(%43, %45) : (i64, i64) -> i64
    %48 = call @sloth_map_get(%41, %47) : (i64, i64) -> i64
    %49 = call @sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %50 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %51 = call @sloth_cls_name(%49, %50, %c15_i64) : (i64, i64, i64) -> i64
    %52 = call @sloth_cls_refmask(%49, %c0_i64, %c2_i64) : (i64, i64, i64) -> i64
    %53 = call @sloth_obj_new(%49, %c2_i64) : (i64, i64) -> i64
    %54 = call @sloth_obj_set_field(%53, %c0_i64, %47) : (i64, i64, i64) -> i64
    %55 = call @sloth_obj_set_field(%53, %c1_i64, %48) : (i64, i64, i64) -> i64
    %alloca_10 = memref.alloca() : memref<1xi64>
    memref.store %53, %alloca_10[%c0] : memref<1xi64>
    %56 = memref.load %alloca_10[%c0] : memref<1xi64>
    %57 = call @sloth_obj_field(%56, %c0_i64) : (i64, i64) -> i64
    %58 = memref.load %alloca_10[%c0] : memref<1xi64>
    %59 = call @sloth_obj_field(%58, %c1_i64) : (i64, i64) -> i64
    %60 = arith.addi %57, %59 : i64
    %61 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %61 : memref<11xi64> -> index
    %62 = arith.index_cast %intptr_11 : index to i64
    %63 = call @sloth_any_from(%62, %60) : (i64, i64) -> i64
    call @sloth_main__print(%63) : (i64) -> ()
    %64 = call @sloth_rc_release(%63) : (i64) -> i64
    cf.br ^bb11
  ^bb11:  // pred: ^bb10
    %65 = call @sloth_rc_release(%53) : (i64) -> i64
    %66 = arith.addi %45, %c1_i64 : i64
    memref.store %66, %alloca_9[%c0] : memref<1xi64>
    cf.br ^bb9
  ^bb12:  // pred: ^bb9
    %67 = call @sloth_rc_release(%43) : (i64) -> i64
    %68 = call @sloth_rc_release(%41) : (i64) -> i64
    %69 = memref.load %alloca[%c0] : memref<1xi64>
    %70 = call @sloth_rc_release(%69) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %71 = arith.index_cast %intptr_12 : index to i64
    %72 = call @sloth_fiber_untrack(%71) : (i64) -> i64
    cf.br ^bb13
  ^bb13:  // pred: ^bb12
    return
  }
  func.func @sloth_main_Range3__iter(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_rc_retain(%0) : (i64) -> i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main_Range3__next(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = arith.cmpi sge, %1, %c3_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %3, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = call @sloth_obj_field(%4, %c0_i64) : (i64, i64) -> i64
    %6 = arith.addi %5, %c1_i64 : i64
    %7 = memref.load %alloca_1[%c0] : memref<1xi64>
    %8 = call @sloth_obj_set_field(%7, %c0_i64, %6) : (i64, i64, i64) -> i64
    %9 = memref.load %alloca_1[%c0] : memref<1xi64>
    %10 = call @sloth_obj_field(%9, %c0_i64) : (i64, i64) -> i64
    %11 = arith.subi %10, %c1_i64 : i64
    %12 = call @sloth_box_new(%11) : (i64) -> i64
    memref.store %12, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb4
  ^bb4:  // 2 preds: ^bb1, ^bb3
    %13 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %13 : i64
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

