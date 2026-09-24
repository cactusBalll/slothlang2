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
    %1 = call @__sloth_rt_write(%0) : (i64) -> i64
    call @__sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @__sloth_rc_release(%1) : (i64) -> i64
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
    %1 = call @__sloth_obj_field(%0, %c0_i64) : (i64, i64) -> i64
    %2 = call @__sloth_obj_field(%0, %c1_i64) : (i64, i64) -> i64
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
    %0 = func.call @__sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
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
    %1 = call @__sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %2 = call @__sloth_obj_new(%1, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %3 = memref.load %alloca_1[%c0] : memref<1xi64>
    %4 = call @__sloth_obj_set_field(%2, %c0_i64, %3) : (i64, i64, i64) -> i64
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %6 = call @__sloth_rc_retain(%2) : (i64) -> i64
    %7 = call @__sloth_closure_new(%5, %6) : (i64, i64) -> i64
    memref.store %7, %alloca_0[%c0] : memref<1xi64>
    memref.store %c1_i64, %alloca[%c0] : memref<1xi64>
    %8 = call @__sloth_rc_release(%2) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    %9 = memref.load %alloca_0[%c0] : memref<1xi64>
    return %9 : i64
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
    %0 = func.call @__sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
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
    %2 = call @__sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %3 = call @__sloth_obj_new(%2, %c0_i64, %c0_i64) : (i64, i64, i64) -> i64
    %4 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %5 = call @__sloth_rc_retain(%3) : (i64) -> i64
    %6 = call @__sloth_closure_new(%4, %5) : (i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %7 = call @__sloth_rc_retain(%6) : (i64) -> i64
    memref.store %7, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %8 = arith.index_cast %intptr : index to i64
    %9 = call @__sloth_fiber_track(%8) : (i64) -> i64
    %10 = call @__sloth_rc_release(%3) : (i64) -> i64
    %11 = call @__sloth_rc_release(%6) : (i64) -> i64
    %12 = memref.load %alloca[%c0] : memref<1xi64>
    %13 = call @__sloth_obj_field(%12, %c0_i64) : (i64, i64) -> i64
    %14 = call @__sloth_obj_field(%12, %c1_i64) : (i64, i64) -> i64
    %15 = llvm.inttoptr %13 : i64 to !llvm.ptr
    %16 = llvm.call %15(%14, %c6_i64) : !llvm.ptr, (i64, i64) -> i64
    %17 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %17 : memref<11xi64> -> index
    %18 = arith.index_cast %intptr_0 : index to i64
    %19 = call @__sloth_any_from(%18, %16) : (i64, i64) -> i64
    call @sloth_main__print(%19) : (i64) -> ()
    %20 = call @__sloth_rc_release(%19) : (i64) -> i64
    %21 = memref.load %alloca[%c0] : memref<1xi64>
    %22 = call @sloth_main__apply(%21, %c5_i64) : (i64, i64) -> i64
    %23 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %23 : memref<11xi64> -> index
    %24 = arith.index_cast %intptr_1 : index to i64
    %25 = call @__sloth_any_from(%24, %22) : (i64, i64) -> i64
    call @sloth_main__print(%25) : (i64) -> ()
    %26 = call @__sloth_rc_release(%25) : (i64) -> i64
    %27 = call @sloth_main__make_adder(%c10_i64) : (i64) -> i64
    %alloca_2 = memref.alloca() : memref<1xi64>
    memref.store %27, %alloca_2[%c0] : memref<1xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %28 = arith.index_cast %intptr_3 : index to i64
    %29 = call @__sloth_fiber_track(%28) : (i64) -> i64
    %30 = memref.load %alloca_2[%c0] : memref<1xi64>
    %31 = call @__sloth_obj_field(%30, %c0_i64) : (i64, i64) -> i64
    %32 = call @__sloth_obj_field(%30, %c1_i64) : (i64, i64) -> i64
    %33 = llvm.inttoptr %31 : i64 to !llvm.ptr
    %34 = llvm.call %33(%32, %c4_i64) : !llvm.ptr, (i64, i64) -> i64
    %35 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %35 : memref<11xi64> -> index
    %36 = arith.index_cast %intptr_4 : index to i64
    %37 = call @__sloth_any_from(%36, %34) : (i64, i64) -> i64
    call @sloth_main__print(%37) : (i64) -> ()
    %38 = call @__sloth_rc_release(%37) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c100_i64, %alloca_5[%c0] : memref<1xi64>
    %39 = call @__sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %40 = call @__sloth_obj_new(%39, %c1_i64, %c0_i64) : (i64, i64, i64) -> i64
    %41 = memref.load %alloca_5[%c0] : memref<1xi64>
    %42 = call @__sloth_obj_set_field(%40, %c0_i64, %41) : (i64, i64, i64) -> i64
    %43 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %44 = call @__sloth_rc_retain(%40) : (i64) -> i64
    %45 = call @__sloth_closure_new(%43, %44) : (i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %46 = call @__sloth_rc_retain(%45) : (i64) -> i64
    memref.store %46, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %47 = arith.index_cast %intptr_7 : index to i64
    %48 = call @__sloth_fiber_track(%47) : (i64) -> i64
    %49 = call @__sloth_rc_release(%40) : (i64) -> i64
    %50 = call @__sloth_rc_release(%45) : (i64) -> i64
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    %51 = memref.load %alloca_6[%c0] : memref<1xi64>
    %52 = call @__sloth_obj_field(%51, %c0_i64) : (i64, i64) -> i64
    %53 = call @__sloth_obj_field(%51, %c1_i64) : (i64, i64) -> i64
    %54 = llvm.inttoptr %52 : i64 to !llvm.ptr
    %55 = llvm.call %54(%53, %c1_i64) : !llvm.ptr, (i64, i64) -> i64
    %56 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %56 : memref<11xi64> -> index
    %57 = arith.index_cast %intptr_8 : index to i64
    %58 = call @__sloth_any_from(%57, %55) : (i64, i64) -> i64
    call @sloth_main__print(%58) : (i64) -> ()
    %59 = call @__sloth_rc_release(%58) : (i64) -> i64
    %60 = memref.load %alloca[%c0] : memref<1xi64>
    %alloca_9 = memref.alloca() : memref<1xi64>
    %61 = call @__sloth_rc_retain(%60) : (i64) -> i64
    memref.store %61, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %62 = arith.index_cast %intptr_10 : index to i64
    %63 = call @__sloth_fiber_track(%62) : (i64) -> i64
    %64 = memref.load %alloca_9[%c0] : memref<1xi64>
    %65 = call @__sloth_obj_field(%64, %c0_i64) : (i64, i64) -> i64
    %66 = call @__sloth_obj_field(%64, %c1_i64) : (i64, i64) -> i64
    %67 = llvm.inttoptr %65 : i64 to !llvm.ptr
    %68 = llvm.call %67(%66, %c3_i64) : !llvm.ptr, (i64, i64) -> i64
    %69 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %69 : memref<11xi64> -> index
    %70 = arith.index_cast %intptr_11 : index to i64
    %71 = call @__sloth_any_from(%70, %68) : (i64, i64) -> i64
    call @sloth_main__print(%71) : (i64) -> ()
    %72 = call @__sloth_rc_release(%71) : (i64) -> i64
    %73 = memref.load %alloca_9[%c0] : memref<1xi64>
    %74 = call @__sloth_rc_release(%73) : (i64) -> i64
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %75 = arith.index_cast %intptr_12 : index to i64
    %76 = call @__sloth_fiber_untrack(%75) : (i64) -> i64
    %77 = memref.load %alloca[%c0] : memref<1xi64>
    %78 = call @__sloth_rc_release(%77) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %79 = arith.index_cast %intptr_13 : index to i64
    %80 = call @__sloth_fiber_untrack(%79) : (i64) -> i64
    %81 = memref.load %alloca_2[%c0] : memref<1xi64>
    %82 = call @__sloth_rc_release(%81) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca_2 : memref<1xi64> -> index
    %83 = arith.index_cast %intptr_14 : index to i64
    %84 = call @__sloth_fiber_untrack(%83) : (i64) -> i64
    %85 = memref.load %alloca_6[%c0] : memref<1xi64>
    %86 = call @__sloth_rc_release(%85) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %87 = arith.index_cast %intptr_15 : index to i64
    %88 = call @__sloth_fiber_untrack(%87) : (i64) -> i64
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

