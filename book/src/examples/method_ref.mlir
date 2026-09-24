module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Counter\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Counter : memref<1xi64> = dense<0> {mutable}
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
  func.func @sloth_main__run(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = call @__sloth_obj_field(%0, %c1_i64) : (i64, i64) -> i64
    %3 = llvm.inttoptr %1 : i64 to !llvm.ptr
    %4 = llvm.call %3(%2) : !llvm.ptr, (i64) -> i64
    memref.store %4, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %5 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %5 : i64
  }
  func.func private @sloth_vtb_main_Counter() -> i64 {
    %c5_i64 = arith.constant 5 : i64
    %0 = llvm.mlir.addressof @sloth_main_Counter__bump : !llvm.ptr
    %c6_i64 = arith.constant 6 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %1 = memref.get_global @sloth_main_g_vtb_Counter : memref<1xi64>
    %2 = memref.load %1[%c0] : memref<1xi64>
    %3 = arith.cmpi eq, %2, %c0_i64 : i64
    cf.cond_br %3, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %4 = call @__sloth_vt_new(%c6_i64) : (i64) -> i64
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %6 = call @__sloth_vt_set(%4, %c5_i64, %5) : (i64, i64, i64) -> i64
    memref.store %4, %1[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %7 = memref.load %1[%c0] : memref<1xi64>
    return %7 : i64
  }
  llvm.func @sloth_main__clo0(%arg0: i64) -> i64 {
    %0 = llvm.call @sloth_main_Counter__bump(%arg0) : (i64) -> i64
    llvm.return %0 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %0 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %c7_i64 = arith.constant 7 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c10_i64 = arith.constant 10 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @__sloth_cls_info(%c0_i64, %c3_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @__sloth_cls_name(%2, %3, %c7_i64) : (i64, i64, i64) -> i64
    %5 = call @__sloth_obj_new(%2, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_vtb_main_Counter() : () -> i64
    %7 = call @__sloth_obj_set_vtable(%5, %6) : (i64, i64) -> i64
    %8 = call @__sloth_obj_set_field(%5, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    call @sloth_main_Counter____init__(%5, %c10_i64) : (i64, i64) -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %9 = call @__sloth_rc_retain(%5) : (i64) -> i64
    memref.store %9, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %10 = arith.index_cast %intptr : index to i64
    %11 = call @__sloth_fiber_track(%10) : (i64) -> i64
    %12 = call @__sloth_rc_release(%5) : (i64) -> i64
    %13 = memref.load %alloca[%c0] : memref<1xi64>
    %14 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %15 = call @__sloth_rc_retain(%13) : (i64) -> i64
    %16 = call @__sloth_closure_new(%14, %15) : (i64, i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    %17 = call @__sloth_rc_retain(%16) : (i64) -> i64
    memref.store %17, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %18 = arith.index_cast %intptr_1 : index to i64
    %19 = call @__sloth_fiber_track(%18) : (i64) -> i64
    %20 = call @__sloth_rc_release(%16) : (i64) -> i64
    %21 = memref.load %alloca_0[%c0] : memref<1xi64>
    %22 = call @__sloth_obj_field(%21, %c0_i64) : (i64, i64) -> i64
    %23 = call @__sloth_obj_field(%21, %c1_i64) : (i64, i64) -> i64
    %24 = llvm.inttoptr %22 : i64 to !llvm.ptr
    %25 = llvm.call %24(%23) : !llvm.ptr, (i64) -> i64
    %26 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %26 : memref<11xi64> -> index
    %27 = arith.index_cast %intptr_2 : index to i64
    %28 = call @__sloth_any_from(%27, %25) : (i64, i64) -> i64
    call @sloth_main__print(%28) : (i64) -> ()
    %29 = call @__sloth_rc_release(%28) : (i64) -> i64
    %30 = memref.load %alloca_0[%c0] : memref<1xi64>
    %31 = call @__sloth_obj_field(%30, %c0_i64) : (i64, i64) -> i64
    %32 = call @__sloth_obj_field(%30, %c1_i64) : (i64, i64) -> i64
    %33 = llvm.inttoptr %31 : i64 to !llvm.ptr
    %34 = llvm.call %33(%32) : !llvm.ptr, (i64) -> i64
    %35 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %35 : memref<11xi64> -> index
    %36 = arith.index_cast %intptr_3 : index to i64
    %37 = call @__sloth_any_from(%36, %34) : (i64, i64) -> i64
    call @sloth_main__print(%37) : (i64) -> ()
    %38 = call @__sloth_rc_release(%37) : (i64) -> i64
    %39 = memref.load %alloca[%c0] : memref<1xi64>
    %40 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %41 = call @__sloth_rc_retain(%39) : (i64) -> i64
    %42 = call @__sloth_closure_new(%40, %41) : (i64, i64) -> i64
    %43 = call @sloth_main__run(%42) : (i64) -> i64
    %44 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %44 : memref<11xi64> -> index
    %45 = arith.index_cast %intptr_4 : index to i64
    %46 = call @__sloth_any_from(%45, %43) : (i64, i64) -> i64
    call @sloth_main__print(%46) : (i64) -> ()
    %47 = call @__sloth_rc_release(%42) : (i64) -> i64
    %48 = call @__sloth_rc_release(%46) : (i64) -> i64
    %49 = memref.load %alloca[%c0] : memref<1xi64>
    %50 = call @__sloth_rc_release(%49) : (i64) -> i64
    %intptr_5 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %51 = arith.index_cast %intptr_5 : index to i64
    %52 = call @__sloth_fiber_untrack(%51) : (i64) -> i64
    %53 = memref.load %alloca_0[%c0] : memref<1xi64>
    %54 = call @__sloth_rc_release(%53) : (i64) -> i64
    %intptr_6 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %55 = arith.index_cast %intptr_6 : index to i64
    %56 = call @__sloth_fiber_untrack(%55) : (i64) -> i64
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
  func.func @sloth_main_Counter____init__(%arg0: i64, %arg1: i64) {
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_0[%c0] : memref<1xi64>
    %0 = memref.load %alloca_0[%c0] : memref<1xi64>
    %1 = memref.load %alloca[%c0] : memref<1xi64>
    %2 = call @__sloth_obj_set_field(%1, %c0_i64, %0) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  llvm.func @sloth_main_Counter__bump(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = func.call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = arith.addi %1, %c1_i64 : i64
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = func.call @__sloth_obj_set_field(%3, %c0_i64, %2) : (i64, i64, i64) -> i64
    %5 = memref.load %alloca_1[%c0] : memref<1xi64>
    %6 = func.call @__sloth_obj_field(%5, %c0_i64) : (i64, i64) -> i64
    memref.store %6, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %7 = memref.load %alloca_0[%c0] : memref<1xi64>
    llvm.return %7 : i64
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

