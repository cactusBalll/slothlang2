module @main {
  llvm.mlir.global private constant @sloth_tynm_0("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
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
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c25185_i64 = arith.constant 25185 : i64
    %c4_i64 = arith.constant 4 : i64
    %c1_i64 = arith.constant 1 : i64
    %c435493693556_i64 = arith.constant 435493693556 : i64
    %c5_i64 = arith.constant 5 : i64
    %c465674792307_i64 = arith.constant 465674792307 : i64
    %c6777186_i64 = arith.constant 6777186 : i64
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %c3_i64 = arith.constant 3 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %c3_i64, %alloca[%c0] : memref<1xi64>
    %0 = memref.load %alloca[%c0] : memref<1xi64>
    %1 = arith.cmpi sgt, %0, %c2_i64 : i64
    cf.cond_br %1, ^bb1(%c6777186_i64, %c3_i64 : i64, i64), ^bb1(%c465674792307_i64, %c5_i64 : i64, i64)
  ^bb1(%2: i64, %3: i64):  // 2 preds: ^bb0, ^bb0
    %4 = call @__sloth_str_push(%c0_i64, %2, %3) : (i64, i64, i64) -> i64
    %5 = call @__sloth_str_finish(%4) : (i64) -> i64
    %6 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %6 : memref<11xi64> -> index
    %7 = arith.index_cast %intptr : index to i64
    %8 = call @__sloth_any_from(%7, %5) : (i64, i64) -> i64
    call @sloth_main__print(%8) : (i64) -> ()
    %9 = call @__sloth_rc_release(%5) : (i64) -> i64
    %10 = call @__sloth_rc_release(%8) : (i64) -> i64
    cf.br ^bb2
  ^bb2:  // pred: ^bb1
    %11 = memref.load %alloca[%c0] : memref<1xi64>
    %12 = arith.cmpi eq, %11, %c3_i64 : i64
    cf.cond_br %12, ^bb3, ^bb4
  ^bb3:  // pred: ^bb2
    %13 = call @__sloth_str_push(%c0_i64, %c435493693556_i64, %c5_i64) : (i64, i64, i64) -> i64
    %14 = call @__sloth_str_finish(%13) : (i64) -> i64
    %15 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %15 : memref<11xi64> -> index
    %16 = arith.index_cast %intptr_0 : index to i64
    %17 = call @__sloth_any_from(%16, %14) : (i64, i64) -> i64
    call @sloth_main__print(%17) : (i64) -> ()
    %18 = call @__sloth_rc_release(%14) : (i64) -> i64
    %19 = call @__sloth_rc_release(%17) : (i64) -> i64
    cf.br ^bb5
  ^bb4:  // pred: ^bb2
    cf.br ^bb5
  ^bb5:  // 2 preds: ^bb3, ^bb4
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_1[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb6:  // 2 preds: ^bb5, ^bb7
    %20 = memref.load %alloca_1[%c0] : memref<1xi64>
    %21 = arith.cmpi slt, %20, %c3_i64 : i64
    cf.cond_br %21, ^bb7, ^bb8
  ^bb7:  // pred: ^bb6
    %22 = memref.load %alloca_1[%c0] : memref<1xi64>
    %23 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %23 : memref<11xi64> -> index
    %24 = arith.index_cast %intptr_2 : index to i64
    %25 = call @__sloth_any_from(%24, %22) : (i64, i64) -> i64
    call @sloth_main__print(%25) : (i64) -> ()
    %26 = call @__sloth_rc_release(%25) : (i64) -> i64
    %27 = memref.load %alloca_1[%c0] : memref<1xi64>
    %28 = arith.addi %27, %c1_i64 : i64
    memref.store %28, %alloca_1[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb8:  // pred: ^bb6
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_3[%c0] : memref<1xi64>
    %alloca_4 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_4[%c0] : memref<1xi64>
    cf.br ^bb9
  ^bb9:  // 2 preds: ^bb8, ^bb17
    %29 = memref.load %alloca_4[%c0] : memref<1xi64>
    %30 = arith.cmpi slt, %29, %c5_i64 : i64
    cf.cond_br %30, ^bb10, ^bb18
  ^bb10:  // pred: ^bb9
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %29, %alloca_5[%c0] : memref<1xi64>
    %31 = memref.load %alloca_5[%c0] : memref<1xi64>
    %32 = arith.cmpi eq, %31, %c1_i64 : i64
    cf.cond_br %32, ^bb11, ^bb12
  ^bb11:  // pred: ^bb10
    cf.br ^bb17
  ^bb12:  // pred: ^bb10
    cf.br ^bb13
  ^bb13:  // pred: ^bb12
    %33 = memref.load %alloca_5[%c0] : memref<1xi64>
    %34 = arith.cmpi eq, %33, %c4_i64 : i64
    cf.cond_br %34, ^bb14, ^bb15
  ^bb14:  // pred: ^bb13
    cf.br ^bb18
  ^bb15:  // pred: ^bb13
    cf.br ^bb16
  ^bb16:  // pred: ^bb15
    %35 = memref.load %alloca_3[%c0] : memref<1xi64>
    %36 = memref.load %alloca_5[%c0] : memref<1xi64>
    %37 = arith.addi %35, %36 : i64
    memref.store %37, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb17
  ^bb17:  // 2 preds: ^bb11, ^bb16
    %38 = arith.addi %29, %c1_i64 : i64
    memref.store %38, %alloca_4[%c0] : memref<1xi64>
    cf.br ^bb9
  ^bb18:  // 2 preds: ^bb9, ^bb14
    %39 = memref.load %alloca_3[%c0] : memref<1xi64>
    %40 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %40 : memref<11xi64> -> index
    %41 = arith.index_cast %intptr_6 : index to i64
    %42 = call @__sloth_any_from(%41, %39) : (i64, i64) -> i64
    call @sloth_main__print(%42) : (i64) -> ()
    %43 = call @__sloth_rc_release(%42) : (i64) -> i64
    %44 = call @__sloth_str_push(%c0_i64, %c25185_i64, %c2_i64) : (i64, i64, i64) -> i64
    %45 = call @__sloth_str_finish(%44) : (i64) -> i64
    %46 = call @__sloth_str_clen(%45) : (i64) -> i64
    %alloca_7 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_7[%c0] : memref<1xi64>
    cf.br ^bb19
  ^bb19:  // 2 preds: ^bb18, ^bb21
    %47 = memref.load %alloca_7[%c0] : memref<1xi64>
    %48 = arith.cmpi slt, %47, %46 : i64
    cf.cond_br %48, ^bb20, ^bb22
  ^bb20:  // pred: ^bb19
    %49 = call @__sloth_str_char(%45, %47) : (i64, i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    memref.store %49, %alloca_8[%c0] : memref<1xi64>
    %50 = memref.load %alloca_8[%c0] : memref<1xi64>
    %51 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %51 : memref<11xi64> -> index
    %52 = arith.index_cast %intptr_9 : index to i64
    %53 = call @__sloth_any_from(%52, %50) : (i64, i64) -> i64
    call @sloth_main__print(%53) : (i64) -> ()
    %54 = call @__sloth_rc_release(%53) : (i64) -> i64
    cf.br ^bb21
  ^bb21:  // pred: ^bb20
    %55 = call @__sloth_rc_release(%49) : (i64) -> i64
    %56 = arith.addi %47, %c1_i64 : i64
    memref.store %56, %alloca_7[%c0] : memref<1xi64>
    cf.br ^bb19
  ^bb22:  // pred: ^bb19
    %57 = call @__sloth_rc_release(%45) : (i64) -> i64
    cf.br ^bb23
  ^bb23:  // pred: ^bb22
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
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c4_i64, %2[%c0] : memref<11xi64>
    memref.store %c1_i64, %2[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %2[%c2] : memref<11xi64>
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %3, %2[%c3] : memref<11xi64>
    memref.store %c3_i64, %2[%c4] : memref<11xi64>
    memref.store %c0_i64, %2[%c9] : memref<11xi64>
    memref.store %c0_i64, %2[%c10] : memref<11xi64>
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c2_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

