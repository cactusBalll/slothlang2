module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Counter\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
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
  func.func @sloth_main__run(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = call @sloth_obj_field(%0, %c1_i64) : (i64, i64) -> i64
    %3 = llvm.inttoptr %1 : i64 to !llvm.ptr
    %4 = llvm.call %3(%2) : !llvm.ptr, (i64) -> i64
    memref.store %4, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %5 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %5 : i64
  }
  llvm.func @sloth_main__clo0(%arg0: i64) -> i64 {
    %0 = func.call @sloth_main_Counter__bump(%arg0) : (i64) -> i64
    llvm.return %0 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %0 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %c7_i64 = arith.constant 7 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c0_i64 = arith.constant 0 : i64
    %c10_i64 = arith.constant 10 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %4 = call @sloth_cls_name(%2, %3, %c7_i64) : (i64, i64, i64) -> i64
    %5 = call @sloth_cls_refmask(%2, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %6 = call @sloth_obj_new(%2, %c1_i64) : (i64, i64) -> i64
    %7 = call @sloth_obj_set_field(%6, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    call @sloth_main_Counter____init__(%6, %c10_i64) : (i64, i64) -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %8 = call @sloth_rc_retain(%6) : (i64) -> i64
    memref.store %8, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %9 = arith.index_cast %intptr : index to i64
    %10 = call @sloth_fiber_track(%9) : (i64) -> i64
    %11 = call @sloth_rc_release(%6) : (i64) -> i64
    %12 = memref.load %alloca[%c0] : memref<1xi64>
    %13 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %14 = call @sloth_rc_retain(%12) : (i64) -> i64
    %15 = call @sloth_closure_new(%13, %14) : (i64, i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    %16 = call @sloth_rc_retain(%15) : (i64) -> i64
    memref.store %16, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %17 = arith.index_cast %intptr_1 : index to i64
    %18 = call @sloth_fiber_track(%17) : (i64) -> i64
    %19 = call @sloth_rc_release(%15) : (i64) -> i64
    %20 = memref.load %alloca_0[%c0] : memref<1xi64>
    %21 = call @sloth_obj_field(%20, %c0_i64) : (i64, i64) -> i64
    %22 = call @sloth_obj_field(%20, %c1_i64) : (i64, i64) -> i64
    %23 = llvm.inttoptr %21 : i64 to !llvm.ptr
    %24 = llvm.call %23(%22) : !llvm.ptr, (i64) -> i64
    %25 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %25 : memref<11xi64> -> index
    %26 = arith.index_cast %intptr_2 : index to i64
    %27 = call @sloth_any_from(%26, %24) : (i64, i64) -> i64
    call @sloth_main__print(%27) : (i64) -> ()
    %28 = call @sloth_rc_release(%27) : (i64) -> i64
    %29 = memref.load %alloca_0[%c0] : memref<1xi64>
    %30 = call @sloth_obj_field(%29, %c0_i64) : (i64, i64) -> i64
    %31 = call @sloth_obj_field(%29, %c1_i64) : (i64, i64) -> i64
    %32 = llvm.inttoptr %30 : i64 to !llvm.ptr
    %33 = llvm.call %32(%31) : !llvm.ptr, (i64) -> i64
    %34 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %34 : memref<11xi64> -> index
    %35 = arith.index_cast %intptr_3 : index to i64
    %36 = call @sloth_any_from(%35, %33) : (i64, i64) -> i64
    call @sloth_main__print(%36) : (i64) -> ()
    %37 = call @sloth_rc_release(%36) : (i64) -> i64
    %38 = memref.load %alloca[%c0] : memref<1xi64>
    %39 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %40 = call @sloth_rc_retain(%38) : (i64) -> i64
    %41 = call @sloth_closure_new(%39, %40) : (i64, i64) -> i64
    %42 = call @sloth_main__run(%41) : (i64) -> i64
    %43 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %43 : memref<11xi64> -> index
    %44 = arith.index_cast %intptr_4 : index to i64
    %45 = call @sloth_any_from(%44, %42) : (i64, i64) -> i64
    call @sloth_main__print(%45) : (i64) -> ()
    %46 = call @sloth_rc_release(%41) : (i64) -> i64
    %47 = call @sloth_rc_release(%45) : (i64) -> i64
    %48 = memref.load %alloca[%c0] : memref<1xi64>
    %49 = call @sloth_rc_release(%48) : (i64) -> i64
    %intptr_5 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %50 = arith.index_cast %intptr_5 : index to i64
    %51 = call @sloth_fiber_untrack(%50) : (i64) -> i64
    %52 = memref.load %alloca_0[%c0] : memref<1xi64>
    %53 = call @sloth_rc_release(%52) : (i64) -> i64
    %intptr_6 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %54 = arith.index_cast %intptr_6 : index to i64
    %55 = call @sloth_fiber_untrack(%54) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
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
    %2 = call @sloth_obj_set_field(%1, %c0_i64, %0) : (i64, i64, i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main_Counter__bump(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = arith.addi %1, %c1_i64 : i64
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = call @sloth_obj_set_field(%3, %c0_i64, %2) : (i64, i64, i64) -> i64
    %5 = memref.load %alloca_1[%c0] : memref<1xi64>
    %6 = call @sloth_obj_field(%5, %c0_i64) : (i64, i64) -> i64
    memref.store %6, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %7 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %7 : i64
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

