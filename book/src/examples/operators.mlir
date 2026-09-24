module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("bool\00") {addr_space = 0 : i32}
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
  func.func @sloth_main__double(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = arith.muli %0, %c2_i64 : i64
    memref.store %1, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %2 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %2 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %true = arith.constant true
    %c20_i64 = arith.constant 20 : i64
    %c14_i64 = arith.constant 14 : i64
    %false = arith.constant false
    %c120_i64 = arith.constant 120 : i64
    %c5_i64 = arith.constant 5 : i64
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c0 = arith.constant 0 : index
    %c0_i64 = arith.constant 0 : i64
    %c2_i64 = arith.constant 2 : i64
    %c7_i64 = arith.constant 7 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %alloca = memref.alloca() : memref<1xi64>
    cf.cond_br %false, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %0 = call @__sloth_panic_divzero() : () -> i64
    memref.store %c0_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    memref.store %c3_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %1 = memref.load %alloca[%c0] : memref<1xi64>
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %2 : memref<11xi64> -> index
    %3 = arith.index_cast %intptr : index to i64
    %4 = call @__sloth_any_from(%3, %1) : (i64, i64) -> i64
    call @sloth_main__print(%4) : (i64) -> ()
    %5 = call @__sloth_rc_release(%4) : (i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    cf.cond_br %false, ^bb4, ^bb5
  ^bb4:  // pred: ^bb3
    %6 = call @__sloth_panic_divzero() : () -> i64
    memref.store %c0_i64, %alloca_0[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb5:  // pred: ^bb3
    memref.store %c1_i64, %alloca_0[%c0] : memref<1xi64>
    cf.br ^bb6
  ^bb6:  // 2 preds: ^bb4, ^bb5
    %7 = memref.load %alloca_0[%c0] : memref<1xi64>
    %8 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_1 : index to i64
    %10 = call @__sloth_any_from(%9, %7) : (i64, i64) -> i64
    call @sloth_main__print(%10) : (i64) -> ()
    %11 = call @__sloth_rc_release(%10) : (i64) -> i64
    %12 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %12 : memref<11xi64> -> index
    %13 = arith.index_cast %intptr_2 : index to i64
    %14 = call @__sloth_any_from(%13, %c14_i64) : (i64, i64) -> i64
    call @sloth_main__print(%14) : (i64) -> ()
    %15 = call @__sloth_rc_release(%14) : (i64) -> i64
    %16 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %16 : memref<11xi64> -> index
    %17 = arith.index_cast %intptr_3 : index to i64
    %18 = call @__sloth_any_from(%17, %c20_i64) : (i64, i64) -> i64
    call @sloth_main__print(%18) : (i64) -> ()
    %19 = call @__sloth_rc_release(%18) : (i64) -> i64
    %20 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %20 : memref<11xi64> -> index
    %21 = arith.index_cast %intptr_4 : index to i64
    %22 = call @__sloth_any_from(%21, %c1_i64) : (i64, i64) -> i64
    call @sloth_main__print(%22) : (i64) -> ()
    %23 = call @__sloth_rc_release(%22) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c1_i64, %alloca_5[%c0] : memref<1xi64>
    cf.cond_br %true, ^bb7, ^bb8
  ^bb7:  // pred: ^bb6
    memref.store %c1_i64, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb8
  ^bb8:  // 2 preds: ^bb6, ^bb7
    %24 = memref.load %alloca_5[%c0] : memref<1xi64>
    %25 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %25 : memref<11xi64> -> index
    %26 = arith.index_cast %intptr_6 : index to i64
    %27 = call @__sloth_any_from(%26, %24) : (i64, i64) -> i64
    call @sloth_main__print(%27) : (i64) -> ()
    %28 = call @__sloth_rc_release(%27) : (i64) -> i64
    %alloca_7 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_7[%c0] : memref<1xi64>
    cf.cond_br %false, ^bb10, ^bb9
  ^bb9:  // pred: ^bb8
    memref.store %c1_i64, %alloca_7[%c0] : memref<1xi64>
    cf.br ^bb10
  ^bb10:  // 2 preds: ^bb8, ^bb9
    %29 = memref.load %alloca_7[%c0] : memref<1xi64>
    %30 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %30 : memref<11xi64> -> index
    %31 = arith.index_cast %intptr_8 : index to i64
    %32 = call @__sloth_any_from(%31, %29) : (i64, i64) -> i64
    call @sloth_main__print(%32) : (i64) -> ()
    %33 = call @__sloth_rc_release(%32) : (i64) -> i64
    %34 = call @sloth_main__double(%c5_i64) : (i64) -> i64
    %35 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %35 : memref<11xi64> -> index
    %36 = arith.index_cast %intptr_9 : index to i64
    %37 = call @__sloth_any_from(%36, %34) : (i64, i64) -> i64
    call @sloth_main__print(%37) : (i64) -> ()
    %38 = call @__sloth_rc_release(%37) : (i64) -> i64
    %39 = call @sloth_main__double(%c2_i64) : (i64) -> i64
    %40 = call @sloth_main__double(%39) : (i64) -> i64
    %41 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %41 : memref<11xi64> -> index
    %42 = arith.index_cast %intptr_10 : index to i64
    %43 = call @__sloth_any_from(%42, %40) : (i64, i64) -> i64
    call @sloth_main__print(%43) : (i64) -> ()
    %44 = call @__sloth_rc_release(%43) : (i64) -> i64
    %alloca_11 = memref.alloca() : memref<1xi64>
    %45 = call @__sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %45, %alloca_11[%c0] : memref<1xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %46 = arith.index_cast %intptr_12 : index to i64
    %47 = call @__sloth_fiber_track(%46) : (i64) -> i64
    %48 = memref.load %alloca_11[%c0] : memref<1xi64>
    %49 = call @__sloth_box_get(%48) : (i64) -> i64
    %50 = arith.cmpi ne, %48, %c0_i64 : i64
    %51 = arith.select %50, %49, %c7_i64 : i64
    %52 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %52 : memref<11xi64> -> index
    %53 = arith.index_cast %intptr_13 : index to i64
    %54 = call @__sloth_any_from(%53, %51) : (i64, i64) -> i64
    call @sloth_main__print(%54) : (i64) -> ()
    %55 = call @__sloth_rc_release(%54) : (i64) -> i64
    %56 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_14 = memref.extract_aligned_pointer_as_index %56 : memref<11xi64> -> index
    %57 = arith.index_cast %intptr_14 : index to i64
    %58 = call @__sloth_any_from(%57, %c1_i64) : (i64, i64) -> i64
    call @sloth_main__print(%58) : (i64) -> ()
    %59 = call @__sloth_rc_release(%58) : (i64) -> i64
    %60 = call @__sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %61 = call @__sloth_str_finish(%60) : (i64) -> i64
    %62 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_15 = memref.extract_aligned_pointer_as_index %62 : memref<11xi64> -> index
    %63 = arith.index_cast %intptr_15 : index to i64
    %64 = call @__sloth_any_from(%63, %c0_i64) : (i64, i64) -> i64
    call @sloth_main__print(%64) : (i64) -> ()
    %65 = call @__sloth_rc_release(%61) : (i64) -> i64
    %66 = call @__sloth_rc_release(%64) : (i64) -> i64
    %alloca_16 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_16[%c0] : memref<1xi64>
    %alloca_17 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_17[%c0] : memref<1xi64>
    cf.br ^bb11
  ^bb11:  // 2 preds: ^bb10, ^bb13
    %67 = memref.load %alloca_17[%c0] : memref<1xi64>
    %68 = arith.cmpi slt, %67, %c3_i64 : i64
    cf.cond_br %68, ^bb12, ^bb14
  ^bb12:  // pred: ^bb11
    %alloca_18 = memref.alloca() : memref<1xi64>
    memref.store %67, %alloca_18[%c0] : memref<1xi64>
    %69 = memref.load %alloca_16[%c0] : memref<1xi64>
    %70 = memref.load %alloca_18[%c0] : memref<1xi64>
    %71 = arith.addi %69, %70 : i64
    memref.store %71, %alloca_16[%c0] : memref<1xi64>
    cf.br ^bb13
  ^bb13:  // pred: ^bb12
    %72 = arith.addi %67, %c1_i64 : i64
    memref.store %72, %alloca_17[%c0] : memref<1xi64>
    cf.br ^bb11
  ^bb14:  // pred: ^bb11
    %73 = memref.load %alloca_16[%c0] : memref<1xi64>
    %74 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_19 = memref.extract_aligned_pointer_as_index %74 : memref<11xi64> -> index
    %75 = arith.index_cast %intptr_19 : index to i64
    %76 = call @__sloth_any_from(%75, %73) : (i64, i64) -> i64
    call @sloth_main__print(%76) : (i64) -> ()
    %77 = call @__sloth_rc_release(%76) : (i64) -> i64
    %78 = memref.load %alloca_11[%c0] : memref<1xi64>
    %79 = call @__sloth_rc_release(%78) : (i64) -> i64
    %intptr_20 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %80 = arith.index_cast %intptr_20 : index to i64
    %81 = call @__sloth_fiber_untrack(%80) : (i64) -> i64
    cf.br ^bb15
  ^bb15:  // pred: ^bb14
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
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c1_i64 = arith.constant 1 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
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
    memref.store %c1_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c4_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

