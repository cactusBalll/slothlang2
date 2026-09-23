module @main {
  llvm.mlir.global private constant @sloth_tynm_0("Node\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
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
    %1 = call @sloth_rt_write(%0) : (i64) -> i64
    call @sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @sloth_rc_release(%1) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c5_i64 = arith.constant 5 : i64
    %c435778317409_i64 = arith.constant 435778317409 : i64
    %c1_i64 = arith.constant 1 : i64
    %c110_i64 = arith.constant 110 : i64
    %c3_i64 = arith.constant 3 : i64
    %c4_i64 = arith.constant 4 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2_i64 = arith.constant 2 : i64
    %c0 = arith.constant 0 : index
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %alloca = memref.alloca() : memref<1xi64>
    %1 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    memref.store %1, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %2 = arith.index_cast %intptr : index to i64
    %3 = call @sloth_fiber_track(%2) : (i64) -> i64
    %4 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %6 = call @sloth_cls_name(%4, %5, %c4_i64) : (i64, i64, i64) -> i64
    %7 = call @sloth_cls_refmask(%4, %c3_i64, %c2_i64) : (i64, i64, i64) -> i64
    %8 = call @sloth_obj_new(%4, %c2_i64) : (i64, i64) -> i64
    %9 = call @sloth_str_push(%c0_i64, %c110_i64, %c1_i64) : (i64, i64, i64) -> i64
    %10 = call @sloth_str_finish(%9) : (i64) -> i64
    %11 = call @sloth_obj_field(%8, %c0_i64) : (i64, i64) -> i64
    %12 = call @sloth_rc_release(%11) : (i64) -> i64
    %13 = call @sloth_rc_retain(%10) : (i64) -> i64
    %14 = call @sloth_obj_set_field(%8, %c0_i64, %13) : (i64, i64, i64) -> i64
    %15 = call @sloth_obj_field(%8, %c1_i64) : (i64, i64) -> i64
    %16 = call @sloth_rc_release(%15) : (i64) -> i64
    %17 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    %18 = call @sloth_obj_set_field(%8, %c1_i64, %17) : (i64, i64, i64) -> i64
    %alloca_0 = memref.alloca() : memref<1xi64>
    %19 = call @sloth_rc_retain(%8) : (i64) -> i64
    memref.store %19, %alloca_0[%c0] : memref<1xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %20 = arith.index_cast %intptr_1 : index to i64
    %21 = call @sloth_fiber_track(%20) : (i64) -> i64
    %22 = call @sloth_rc_release(%8) : (i64) -> i64
    %23 = call @sloth_rc_release(%10) : (i64) -> i64
    %24 = memref.load %alloca_0[%c0] : memref<1xi64>
    %25 = call @sloth_weak_new(%24) : (i64) -> i64
    %26 = memref.load %alloca[%c0] : memref<1xi64>
    %27 = call @sloth_rc_release(%26) : (i64) -> i64
    %28 = call @sloth_rc_retain(%25) : (i64) -> i64
    memref.store %28, %alloca[%c0] : memref<1xi64>
    %29 = call @sloth_rc_release(%25) : (i64) -> i64
    %30 = memref.load %alloca_0[%c0] : memref<1xi64>
    %31 = call @sloth_rc_release(%30) : (i64) -> i64
    %intptr_2 = memref.extract_aligned_pointer_as_index %alloca_0 : memref<1xi64> -> index
    %32 = arith.index_cast %intptr_2 : index to i64
    %33 = call @sloth_fiber_untrack(%32) : (i64) -> i64
    %34 = memref.load %alloca[%c0] : memref<1xi64>
    %35 = call @sloth_weak_upgrade(%34) : (i64) -> i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    %36 = call @sloth_rc_retain(%35) : (i64) -> i64
    memref.store %36, %alloca_3[%c0] : memref<1xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %alloca_3 : memref<1xi64> -> index
    %37 = arith.index_cast %intptr_4 : index to i64
    %38 = call @sloth_fiber_track(%37) : (i64) -> i64
    %39 = call @sloth_rc_release(%35) : (i64) -> i64
    %40 = memref.load %alloca_3[%c0] : memref<1xi64>
    %41 = arith.cmpi eq, %40, %c0_i64 : i64
    %42 = arith.extui %41 : i1 to i64
    %43 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %43 : memref<11xi64> -> index
    %44 = arith.index_cast %intptr_5 : index to i64
    %45 = call @sloth_any_from(%44, %42) : (i64, i64) -> i64
    call @sloth_main__print(%45) : (i64) -> ()
    %46 = call @sloth_rc_release(%45) : (i64) -> i64
    %47 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %48 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %49 = call @sloth_cls_name(%47, %48, %c4_i64) : (i64, i64, i64) -> i64
    %50 = call @sloth_cls_refmask(%47, %c3_i64, %c2_i64) : (i64, i64, i64) -> i64
    %51 = call @sloth_obj_new(%47, %c2_i64) : (i64, i64) -> i64
    %52 = call @sloth_str_push(%c0_i64, %c110_i64, %c1_i64) : (i64, i64, i64) -> i64
    %53 = call @sloth_str_finish(%52) : (i64) -> i64
    %54 = call @sloth_obj_field(%51, %c0_i64) : (i64, i64) -> i64
    %55 = call @sloth_rc_release(%54) : (i64) -> i64
    %56 = call @sloth_rc_retain(%53) : (i64) -> i64
    %57 = call @sloth_obj_set_field(%51, %c0_i64, %56) : (i64, i64, i64) -> i64
    %58 = call @sloth_obj_field(%51, %c1_i64) : (i64, i64) -> i64
    %59 = call @sloth_rc_release(%58) : (i64) -> i64
    %60 = call @sloth_rc_retain(%c0_i64) : (i64) -> i64
    %61 = call @sloth_obj_set_field(%51, %c1_i64, %60) : (i64, i64, i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    %62 = call @sloth_rc_retain(%51) : (i64) -> i64
    memref.store %62, %alloca_6[%c0] : memref<1xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %63 = arith.index_cast %intptr_7 : index to i64
    %64 = call @sloth_fiber_track(%63) : (i64) -> i64
    %65 = call @sloth_rc_release(%51) : (i64) -> i64
    %66 = call @sloth_rc_release(%53) : (i64) -> i64
    %67 = call @sloth_str_push(%c0_i64, %c435778317409_i64, %c5_i64) : (i64, i64, i64) -> i64
    %68 = call @sloth_str_finish(%67) : (i64) -> i64
    %69 = memref.load %alloca_6[%c0] : memref<1xi64>
    %70 = call @sloth_obj_field(%69, %c0_i64) : (i64, i64) -> i64
    %71 = call @sloth_rc_release(%70) : (i64) -> i64
    %72 = call @sloth_rc_retain(%68) : (i64) -> i64
    %73 = call @sloth_obj_set_field(%69, %c0_i64, %72) : (i64, i64, i64) -> i64
    %74 = call @sloth_rc_release(%68) : (i64) -> i64
    %75 = memref.load %alloca_6[%c0] : memref<1xi64>
    %76 = call @sloth_weak_new(%75) : (i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    %77 = call @sloth_rc_retain(%76) : (i64) -> i64
    memref.store %77, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %78 = arith.index_cast %intptr_9 : index to i64
    %79 = call @sloth_fiber_track(%78) : (i64) -> i64
    %80 = call @sloth_rc_release(%76) : (i64) -> i64
    %81 = memref.load %alloca_8[%c0] : memref<1xi64>
    %82 = call @sloth_weak_upgrade(%81) : (i64) -> i64
    %alloca_10 = memref.alloca() : memref<1xi64>
    %83 = call @sloth_rc_retain(%82) : (i64) -> i64
    memref.store %83, %alloca_10[%c0] : memref<1xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %alloca_10 : memref<1xi64> -> index
    %84 = arith.index_cast %intptr_11 : index to i64
    %85 = call @sloth_fiber_track(%84) : (i64) -> i64
    %86 = call @sloth_rc_release(%82) : (i64) -> i64
    %87 = memref.load %alloca_10[%c0] : memref<1xi64>
    %88 = arith.cmpi eq, %87, %c0_i64 : i64
    %89 = arith.extui %88 : i1 to i64
    %90 = arith.xori %89, %c1_i64 : i64
    %91 = arith.cmpi ne, %90, %c0_i64 : i64
    cf.cond_br %91, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %92 = memref.load %alloca_10[%c0] : memref<1xi64>
    %93 = call @sloth_obj_field(%92, %c0_i64) : (i64, i64) -> i64
    %94 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %94 : memref<11xi64> -> index
    %95 = arith.index_cast %intptr_12 : index to i64
    %96 = call @sloth_any_from(%95, %93) : (i64, i64) -> i64
    call @sloth_main__print(%96) : (i64) -> ()
    %97 = call @sloth_rc_release(%96) : (i64) -> i64
    cf.br ^bb3
  ^bb2:  // pred: ^bb0
    cf.br ^bb3
  ^bb3:  // 2 preds: ^bb1, ^bb2
    %98 = memref.load %alloca_6[%c0] : memref<1xi64>
    %99 = call @sloth_rc_release(%98) : (i64) -> i64
    %intptr_13 = memref.extract_aligned_pointer_as_index %alloca_6 : memref<1xi64> -> index
    %100 = arith.index_cast %intptr_13 : index to i64
    %101 = call @sloth_fiber_untrack(%100) : (i64) -> i64
    %102 = memref.load %alloca_8[%c0] : memref<1xi64>
    %103 = call @sloth_rc_release(%102) : (i64) -> i64
    %intptr_14 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %104 = arith.index_cast %intptr_14 : index to i64
    %105 = call @sloth_fiber_untrack(%104) : (i64) -> i64
    %106 = memref.load %alloca_10[%c0] : memref<1xi64>
    %107 = call @sloth_rc_release(%106) : (i64) -> i64
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_10 : memref<1xi64> -> index
    %108 = arith.index_cast %intptr_15 : index to i64
    %109 = call @sloth_fiber_untrack(%108) : (i64) -> i64
    %110 = memref.load %alloca[%c0] : memref<1xi64>
    %111 = call @sloth_rc_release(%110) : (i64) -> i64
    %intptr_16 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %112 = arith.index_cast %intptr_16 : index to i64
    %113 = call @sloth_fiber_untrack(%112) : (i64) -> i64
    %114 = memref.load %alloca_3[%c0] : memref<1xi64>
    %115 = call @sloth_rc_release(%114) : (i64) -> i64
    %intptr_17 = memref.extract_aligned_pointer_as_index %alloca_3 : memref<1xi64> -> index
    %116 = arith.index_cast %intptr_17 : index to i64
    %117 = call @sloth_fiber_untrack(%116) : (i64) -> i64
    cf.br ^bb4
  ^bb4:  // pred: ^bb3
    return
  }
  func.func @sloth_main__anyinit() {
    %c3_i64 = arith.constant 3 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c4_i64 = arith.constant 4 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c1_i64 = arith.constant 1 : i64
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c1_i64, %2[%c0] : memref<11xi64>
    memref.store %c0_i64, %2[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %2[%c2] : memref<11xi64>
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %3, %2[%c3] : memref<11xi64>
    memref.store %c4_i64, %2[%c4] : memref<11xi64>
    memref.store %c0_i64, %2[%c9] : memref<11xi64>
    memref.store %c0_i64, %2[%c10] : memref<11xi64>
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c4_i64, %4[%c0] : memref<11xi64>
    memref.store %c1_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

