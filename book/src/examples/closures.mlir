module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
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
  func.func @sloth_main__apply(%arg0: i64, %arg1: i64) -> i64 {
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
    %1 = call @sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = call @sloth_obj_field(%0, %c1_i64) : (i64, i64) -> i64
    %3 = llvm.inttoptr %1 : i64 to !llvm.ptr
    %4 = memref.load %alloca_2[%c0] : memref<1xi64>
    %5 = llvm.call %3(%2, %4) : !llvm.ptr, (i64, i64) -> i64
    memref.store %5, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %6 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %6 : i64
  }
  func.func @sloth_main__lam1(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_2[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = arith.addi %0, %1 : i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  llvm.func @sloth_main__clo0(%arg0: i64, %arg1: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @sloth_main__lam1(%0, %arg1) : (i64, i64) -> i64
    llvm.return %1 : i64
  }
  func.func @sloth_main__make_adder(%arg0: i64) -> i64 {
    %0 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %c1_i64 = arith.constant 1 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %1 = call @sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %2 = call @sloth_cls_refmask(%1, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %3 = call @sloth_obj_new(%1, %c1_i64) : (i64, i64) -> i64
    %4 = memref.load %alloca_1[%c0] : memref<1xi64>
    %5 = call @sloth_obj_set_field(%3, %c0_i64, %4) : (i64, i64, i64) -> i64
    %6 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %7 = call @sloth_rc_retain(%3) : (i64) -> i64
    %8 = call @sloth_closure_new(%6, %7) : (i64, i64) -> i64
    memref.store %8, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %9 = call @sloth_rc_release(%3) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %10 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %10 : i64
  }
  func.func @sloth_main__lam2(%arg0: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %0 = memref.load %alloca_1[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = arith.muli %0, %1 : i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  llvm.func @sloth_main__clo1(%arg0: i64, %arg1: i64) -> i64 {
    %0 = func.call @sloth_main__lam2(%arg1) : (i64) -> i64
    llvm.return %0 : i64
  }
  func.func @sloth_main__lam3(%arg0: i64, %arg1: i64) -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    %alloca_0 = memref.alloca() : memref<1xi64>
    %alloca_1 = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca_1[%c0] : memref<1xi64>
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %arg1, %alloca_2[%c0] : memref<1xi64>
    %0 = memref.load %alloca_2[%c0] : memref<1xi64>
    %1 = memref.load %alloca_1[%c0] : memref<1xi64>
    %2 = arith.addi %0, %1 : i64
    memref.store %2, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %3 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %3 : i64
  }
  llvm.func @sloth_main__clo2(%arg0: i64, %arg1: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @sloth_main__lam3(%0, %arg1) : (i64, i64) -> i64
    llvm.return %1 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c3_i64 = arith.constant 3 : i64
    %0 = llvm.mlir.addressof @sloth_main__clo2 : !llvm.ptr
    %c100_i64 = arith.constant 100 : i64
    %c4_i64 = arith.constant 4 : i64
    %c10_i64 = arith.constant 10 : i64
    %c5_i64 = arith.constant 5 : i64
    %c1_i64 = arith.constant 1 : i64
    %c6_i64 = arith.constant 6 : i64
    %c0 = arith.constant 0 : index
    %1 = llvm.mlir.addressof @sloth_main__clo1 : !llvm.ptr
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %3 = call @sloth_cls_refmask(%2, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %4 = call @sloth_obj_new(%2, %c0_i64) : (i64, i64) -> i64
    %5 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %6 = call @sloth_rc_retain(%4) : (i64) -> i64
    %7 = call @sloth_closure_new(%5, %6) : (i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %8 = call @sloth_rc_retain(%7) : (i64) -> i64
    memref.store %8, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %9 = arith.index_cast %intptr : index to i64
    %10 = call @sloth_fiber_track(%9) : (i64) -> i64
    %11 = call @sloth_rc_release(%4) : (i64) -> i64
    %12 = call @sloth_rc_release(%7) : (i64) -> i64
    %13 = memref.load %alloca[%c0] : memref<1xi64>
    %14 = call @sloth_obj_field(%13, %c0_i64) : (i64, i64) -> i64
    %15 = call @sloth_obj_field(%13, %c1_i64) : (i64, i64) -> i64
    %16 = llvm.inttoptr %14 : i64 to !llvm.ptr
    %17 = llvm.call %16(%15, %c6_i64) : !llvm.ptr, (i64, i64) -> i64
    %18 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %18 : memref<11xi64> -> index
    %19 = arith.index_cast %intptr_0 : index to i64
    %20 = call @sloth_any_from(%19, %17) : (i64, i64) -> i64
    call @sloth_main__print(%20) : (i64) -> ()
    %21 = call @sloth_rc_release(%20) : (i64) -> i64
    %22 = memref.load %alloca[%c0] : memref<1xi64>
    %23 = call @sloth_main__apply(%22, %c5_i64) : (i64, i64) -> i64
    %24 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %24 : memref<11xi64> -> index
    %25 = arith.index_cast %intptr_1 : index to i64
    %26 = call @sloth_any_from(%25, %23) : (i64, i64) -> i64
    call @sloth_main__print(%26) : (i64) -> ()
    %27 = call @sloth_rc_release(%26) : (i64) -> i64
    %28 = call @sloth_main__make_adder(%c10_i64) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %28, %alloca_2[%c0] : memref<1xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %29 = arith.index_cast %intptr_3 : index to i64
    %30 = call @sloth_fiber_track(%29) : (i64) -> i64
    %31 = memref.load %alloca_2[%c0] : memref<1xi64>
    %32 = call @sloth_obj_field(%31, %c0_i64) : (i64, i64) -> i64
    %33 = call @sloth_obj_field(%31, %c1_i64) : (i64, i64) -> i64
    %34 = llvm.inttoptr %32 : i64 to !llvm.ptr
    %35 = llvm.call %34(%33, %c4_i64) : !llvm.ptr, (i64, i64) -> i64
    %36 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %36 : memref<11xi64> -> index
    %37 = arith.index_cast %intptr_4 : index to i64
    %38 = call @sloth_any_from(%37, %35) : (i64, i64) -> i64
    call @sloth_main__print(%38) : (i64) -> ()
    %39 = call @sloth_rc_release(%38) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c100_i64, %alloca_5[%c0] : memref<1xi64>
    %40 = call @sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %41 = call @sloth_cls_refmask(%40, %c0_i64, %c1_i64) : (i64, i64, i64) -> i64
    %42 = call @sloth_obj_new(%40, %c1_i64) : (i64, i64) -> i64
    %43 = memref.load %alloca_5[%c0] : memref<1xi64>
    %44 = call @sloth_obj_set_field(%42, %c0_i64, %43) : (i64, i64, i64) -> i64
    %45 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %46 = call @sloth_rc_retain(%42) : (i64) -> i64
    %47 = call @sloth_closure_new(%45, %46) : (i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %48 = call @sloth_rc_retain(%47) : (i64) -> i64
    memref.store %48, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %49 = arith.index_cast %intptr_7 : index to i64
    %50 = call @sloth_fiber_track(%49) : (i64) -> i64
    %51 = call @sloth_rc_release(%42) : (i64) -> i64
    %52 = call @sloth_rc_release(%47) : (i64) -> i64
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    %53 = memref.load %alloca_6[%c0] : memref<1xi64>
    %54 = call @sloth_obj_field(%53, %c0_i64) : (i64, i64) -> i64
    %55 = call @sloth_obj_field(%53, %c1_i64) : (i64, i64) -> i64
    %56 = llvm.inttoptr %54 : i64 to !llvm.ptr
    %57 = llvm.call %56(%55, %c1_i64) : !llvm.ptr, (i64, i64) -> i64
    %58 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %58 : memref<11xi64> -> index
    %59 = arith.index_cast %intptr_8 : index to i64
    %60 = call @sloth_any_from(%59, %57) : (i64, i64) -> i64
    call @sloth_main__print(%60) : (i64) -> ()
    %61 = call @sloth_rc_release(%60) : (i64) -> i64
    %62 = memref.load %alloca[%c0] : memref<1xi64>
    %alloca_9 = memref.alloca() : memref<1xi64>
    %63 = call @sloth_rc_retain(%62) : (i64) -> i64
    memref.store %63, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %64 = arith.index_cast %intptr_10 : index to i64
    %65 = call @sloth_fiber_track(%64) : (i64) -> i64
    %66 = memref.load %alloca_9[%c0] : memref<1xi64>
    %67 = call @sloth_obj_field(%66, %c0_i64) : (i64, i64) -> i64
    %68 = call @sloth_obj_field(%66, %c1_i64) : (i64, i64) -> i64
    %69 = llvm.inttoptr %67 : i64 to !llvm.ptr
    %70 = llvm.call %69(%68, %c3_i64) : !llvm.ptr, (i64, i64) -> i64
    %71 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %71 : memref<11xi64> -> index
    %72 = arith.index_cast %intptr_11 : index to i64
    %73 = call @sloth_any_from(%72, %70) : (i64, i64) -> i64
    call @sloth_main__print(%73) : (i64) -> ()
    %74 = call @sloth_rc_release(%73) : (i64) -> i64
    %75 = memref.load %alloca_2[%c0] : memref<1xi64>
    %76 = call @sloth_rc_release(%75) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %77 = arith.index_cast %intptr_12 : index to i64
    %78 = call @sloth_fiber_untrack(%77) : (i64) -> i64
    %79 = memref.load %alloca_9[%c0] : memref<1xi64>
    %80 = call @sloth_rc_release(%79) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %81 = arith.index_cast %intptr_13 : index to i64
    %82 = call @sloth_fiber_untrack(%81) : (i64) -> i64
    %83 = memref.load %alloca_6[%c0] : memref<1xi64>
    %84 = call @sloth_rc_release(%83) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %85 = arith.index_cast %intptr_14 : index to i64
    %86 = call @sloth_fiber_untrack(%85) : (i64) -> i64
    %87 = memref.load %alloca[%c0] : memref<1xi64>
    %88 = call @sloth_rc_release(%87) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %89 = arith.index_cast %intptr_15 : index to i64
    %90 = call @sloth_fiber_untrack(%89) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
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

