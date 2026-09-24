module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("float\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("bool\00") {addr_space = 0 : i32}
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
    %1 = call @__sloth_rt_write(%0) : (i64) -> i64
    call @__sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @__sloth_rc_release(%1) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c1_i64 = arith.constant 1 : i64
    %c3_i64 = arith.constant 3 : i64
    %c4598175219545276416_i64 = arith.constant 4598175219545276416 : i64
    %c4609434218613702656_i64 = arith.constant 4609434218613702656 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %0 : memref<11xi64> -> index
    %1 = arith.index_cast %intptr : index to i64
    %2 = call @__sloth_any_from(%1, %c3_i64) : (i64, i64) -> i64
    call @sloth_main__print(%2) : (i64) -> ()
    %3 = call @__sloth_rc_release(%2) : (i64) -> i64
    %4 = llvm.bitcast %c4609434218613702656_i64 : i64 to f64
    %5 = llvm.bitcast %c4598175219545276416_i64 : i64 to f64
    %6 = arith.addf %4, %5 : f64
    %7 = llvm.bitcast %6 : f64 to i64
    %8 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %8 : memref<11xi64> -> index
    %9 = arith.index_cast %intptr_0 : index to i64
    %10 = call @__sloth_any_from(%9, %7) : (i64, i64) -> i64
    call @sloth_main__print(%10) : (i64) -> ()
    %11 = call @__sloth_rc_release(%10) : (i64) -> i64
    %12 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %12 : memref<11xi64> -> index
    %13 = arith.index_cast %intptr_1 : index to i64
    %14 = call @__sloth_any_from(%13, %c1_i64) : (i64, i64) -> i64
    call @sloth_main__print(%14) : (i64) -> ()
    %15 = call @__sloth_rc_release(%14) : (i64) -> i64
    %16 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %16 : memref<11xi64> -> index
    %17 = arith.index_cast %intptr_2 : index to i64
    %18 = call @__sloth_any_from(%17, %c1_i64) : (i64, i64) -> i64
    call @sloth_main__print(%18) : (i64) -> ()
    %19 = call @__sloth_rc_release(%18) : (i64) -> i64
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
  func.func @sloth_main__anyinit() {
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %c1_i64 = arith.constant 1 : i64
    %c5_i64 = arith.constant 5 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %2 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %3 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c2_i64, %3[%c0] : memref<11xi64>
    memref.store %c0_i64, %3[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %3[%c2] : memref<11xi64>
    %4 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %4, %3[%c3] : memref<11xi64>
    memref.store %c3_i64, %3[%c4] : memref<11xi64>
    memref.store %c0_i64, %3[%c9] : memref<11xi64>
    memref.store %c0_i64, %3[%c10] : memref<11xi64>
    %5 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c3_i64, %5[%c0] : memref<11xi64>
    memref.store %c0_i64, %5[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %5[%c2] : memref<11xi64>
    %6 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %6, %5[%c3] : memref<11xi64>
    memref.store %c5_i64, %5[%c4] : memref<11xi64>
    memref.store %c0_i64, %5[%c9] : memref<11xi64>
    memref.store %c0_i64, %5[%c10] : memref<11xi64>
    %7 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    memref.store %c1_i64, %7[%c0] : memref<11xi64>
    memref.store %c0_i64, %7[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %7[%c2] : memref<11xi64>
    %8 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %8, %7[%c3] : memref<11xi64>
    memref.store %c4_i64, %7[%c4] : memref<11xi64>
    memref.store %c0_i64, %7[%c9] : memref<11xi64>
    memref.store %c0_i64, %7[%c10] : memref<11xi64>
    return
  }
}

