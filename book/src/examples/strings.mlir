module @main {
  llvm.mlir.global private constant @sloth_tynm_0("str\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("bool\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_3("StrChars\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_4("float\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_2 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_3 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_StrChars : memref<1xi64> = dense<0> {mutable}
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
  llvm.func @sloth_main_StrChars__cascade(%arg0: i64, %arg1: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @__sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @__sloth_rc_release(%0) : (i64) -> i64
    llvm.return %c0_i64 : i64
  }
  func.func private @sloth_vtb_main_StrChars() -> i64 {
    %c1_i64 = arith.constant 1 : i64
    %0 = llvm.mlir.addressof @sloth_main_StrChars__next : !llvm.ptr
    %1 = llvm.mlir.addressof @sloth_main_StrChars__iter : !llvm.ptr
    %c5_i64 = arith.constant 5 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %2 = memref.get_global @sloth_main_g_vtb_StrChars : memref<1xi64>
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
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c4022816_i64 = arith.constant 4022816 : i64
    %c4023840_i64 = arith.constant 4023840 : i64
    %c15726_i64 = arith.constant 15726 : i64
    %c4609434218613702656_i64 = arith.constant 4609434218613702656 : i64
    %c122_i64 = arith.constant 122 : i64
    %c121_i64 = arith.constant 121 : i64
    %c120_i64 = arith.constant 120 : i64
    %0 = llvm.mlir.addressof @sloth_main_StrChars__cascade : !llvm.ptr
    %c8_i64 = arith.constant 8 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_3 : !llvm.ptr
    %c191009621918561_i64 = arith.constant 191009621918561 : i64
    %c98_i64 = arith.constant 98 : i64
    %c1_i64 = arith.constant 1 : i64
    %c97_i64 = arith.constant 97 : i64
    %c2_i64 = arith.constant 2 : i64
    %c28524_i64 = arith.constant 28524 : i64
    %c3_i64 = arith.constant 3 : i64
    %c7103848_i64 = arith.constant 7103848 : i64
    %c6_i64 = arith.constant 6 : i64
    %c122511470216040_i64 = arith.constant 122511470216040 : i64
    %c7_i64 = arith.constant 7 : i64
    %c28266736423215148_i64 = arith.constant 28266736423215148 : i64
    %c0 = arith.constant 0 : index
    %c5_i64 = arith.constant 5 : i64
    %c478560413032_i64 = arith.constant 478560413032 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %2 = call @__sloth_str_push(%c0_i64, %c478560413032_i64, %c5_i64) : (i64, i64, i64) -> i64
    %3 = call @__sloth_str_finish(%2) : (i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %4 = call @__sloth_rc_retain(%3) : (i64) -> i64
    memref.store %4, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %5 = arith.index_cast %intptr : index to i64
    %6 = call @__sloth_fiber_track(%5) : (i64) -> i64
    %7 = call @__sloth_rc_release(%3) : (i64) -> i64
    %8 = memref.load %alloca[%c0] : memref<1xi64>
    %9 = call @__sloth_str_push(%c0_i64, %c28266736423215148_i64, %c7_i64) : (i64, i64, i64) -> i64
    %10 = call @__sloth_str_finish(%9) : (i64) -> i64
    %11 = call @__sloth_str_concat(%8, %10) : (i64, i64) -> i64
    %12 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %12 : memref<11xi64> -> index
    %13 = arith.index_cast %intptr_0 : index to i64
    %14 = call @__sloth_any_from(%13, %11) : (i64, i64) -> i64
    call @sloth_main__print(%14) : (i64) -> ()
    %15 = call @__sloth_rc_release(%10) : (i64) -> i64
    %16 = call @__sloth_rc_release(%11) : (i64) -> i64
    %17 = call @__sloth_rc_release(%14) : (i64) -> i64
    %18 = memref.load %alloca[%c0] : memref<1xi64>
    %19 = call @__sloth_str_len(%18) : (i64) -> i64
    %20 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %20 : memref<11xi64> -> index
    %21 = arith.index_cast %intptr_1 : index to i64
    %22 = call @__sloth_any_from(%21, %19) : (i64, i64) -> i64
    call @sloth_main__print(%22) : (i64) -> ()
    %23 = call @__sloth_rc_release(%22) : (i64) -> i64
    %24 = call @__sloth_str_push(%c0_i64, %c122511470216040_i64, %c6_i64) : (i64, i64, i64) -> i64
    %25 = call @__sloth_str_finish(%24) : (i64) -> i64
    %26 = call @__sloth_str_len(%25) : (i64) -> i64
    %27 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %27 : memref<11xi64> -> index
    %28 = arith.index_cast %intptr_2 : index to i64
    %29 = call @__sloth_any_from(%28, %26) : (i64, i64) -> i64
    call @sloth_main__print(%29) : (i64) -> ()
    %30 = call @__sloth_rc_release(%25) : (i64) -> i64
    %31 = call @__sloth_rc_release(%29) : (i64) -> i64
    %32 = memref.load %alloca[%c0] : memref<1xi64>
    %33 = call @__sloth_str_push(%c0_i64, %c7103848_i64, %c3_i64) : (i64, i64, i64) -> i64
    %34 = call @__sloth_str_finish(%33) : (i64) -> i64
    %35 = call @__sloth_str_push(%c0_i64, %c28524_i64, %c2_i64) : (i64, i64, i64) -> i64
    %36 = call @__sloth_str_finish(%35) : (i64) -> i64
    %37 = call @__sloth_str_concat(%34, %36) : (i64, i64) -> i64
    %38 = call @__sloth_str_eq(%32, %37) : (i64, i64) -> i64
    %39 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_3 = memref.extract_aligned_pointer_as_index %39 : memref<11xi64> -> index
    %40 = arith.index_cast %intptr_3 : index to i64
    %41 = call @__sloth_any_from(%40, %38) : (i64, i64) -> i64
    call @sloth_main__print(%41) : (i64) -> ()
    %42 = call @__sloth_rc_release(%34) : (i64) -> i64
    %43 = call @__sloth_rc_release(%36) : (i64) -> i64
    %44 = call @__sloth_rc_release(%37) : (i64) -> i64
    %45 = call @__sloth_rc_release(%41) : (i64) -> i64
    %46 = call @__sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %47 = call @__sloth_str_finish(%46) : (i64) -> i64
    %48 = call @__sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %49 = call @__sloth_str_finish(%48) : (i64) -> i64
    %50 = call @__sloth_str_eq(%47, %49) : (i64, i64) -> i64
    %51 = arith.xori %50, %c1_i64 : i64
    %52 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_4 = memref.extract_aligned_pointer_as_index %52 : memref<11xi64> -> index
    %53 = arith.index_cast %intptr_4 : index to i64
    %54 = call @__sloth_any_from(%53, %51) : (i64, i64) -> i64
    call @sloth_main__print(%54) : (i64) -> ()
    %55 = call @__sloth_rc_release(%47) : (i64) -> i64
    %56 = call @__sloth_rc_release(%49) : (i64) -> i64
    %57 = call @__sloth_rc_release(%54) : (i64) -> i64
    %alloca_5 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_5[%c0] : memref<1xi64>
    %58 = call @__sloth_str_push(%c0_i64, %c191009621918561_i64, %c6_i64) : (i64, i64, i64) -> i64
    %59 = call @__sloth_str_finish(%58) : (i64) -> i64
    %60 = call @__sloth_str_clen(%59) : (i64) -> i64
    %alloca_6 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %61 = memref.load %alloca_6[%c0] : memref<1xi64>
    %62 = arith.cmpi slt, %61, %60 : i64
    cf.cond_br %62, ^bb2, ^bb4
  ^bb2:  // pred: ^bb1
    %63 = call @__sloth_str_char(%59, %61) : (i64, i64) -> i64
    %alloca_7 = memref.alloca() : memref<1xi64>
    memref.store %63, %alloca_7[%c0] : memref<1xi64>
    %64 = memref.load %alloca_5[%c0] : memref<1xi64>
    %65 = arith.addi %64, %c1_i64 : i64
    memref.store %65, %alloca_5[%c0] : memref<1xi64>
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %66 = call @__sloth_rc_release(%63) : (i64) -> i64
    %67 = arith.addi %61, %c1_i64 : i64
    memref.store %67, %alloca_6[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %68 = call @__sloth_rc_release(%59) : (i64) -> i64
    %69 = memref.load %alloca_5[%c0] : memref<1xi64>
    %70 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_8 = memref.extract_aligned_pointer_as_index %70 : memref<11xi64> -> index
    %71 = arith.index_cast %intptr_8 : index to i64
    %72 = call @__sloth_any_from(%71, %69) : (i64, i64) -> i64
    call @sloth_main__print(%72) : (i64) -> ()
    %73 = call @__sloth_rc_release(%72) : (i64) -> i64
    %74 = call @__sloth_str_push(%c0_i64, %c122511470216040_i64, %c6_i64) : (i64, i64, i64) -> i64
    %75 = call @__sloth_str_finish(%74) : (i64) -> i64
    %alloca_9 = memref.alloca() : memref<1xi64>
    %76 = call @__sloth_rc_retain(%75) : (i64) -> i64
    memref.store %76, %alloca_9[%c0] : memref<1xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %77 = arith.index_cast %intptr_10 : index to i64
    %78 = call @__sloth_fiber_track(%77) : (i64) -> i64
    %79 = call @__sloth_rc_release(%75) : (i64) -> i64
    %80 = memref.load %alloca_9[%c0] : memref<1xi64>
    %81 = call @__sloth_str_byte(%80, %c0_i64) : (i64, i64) -> i64
    %82 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_11 = memref.extract_aligned_pointer_as_index %82 : memref<11xi64> -> index
    %83 = arith.index_cast %intptr_11 : index to i64
    %84 = call @__sloth_any_from(%83, %81) : (i64, i64) -> i64
    call @sloth_main__print(%84) : (i64) -> ()
    %85 = call @__sloth_rc_release(%84) : (i64) -> i64
    %86 = memref.load %alloca_9[%c0] : memref<1xi64>
    %87 = call @__sloth_str_slice(%86, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %88 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %88 : memref<11xi64> -> index
    %89 = arith.index_cast %intptr_12 : index to i64
    %90 = call @__sloth_any_from(%89, %87) : (i64, i64) -> i64
    call @sloth_main__print(%90) : (i64) -> ()
    %91 = call @__sloth_rc_release(%87) : (i64) -> i64
    %92 = call @__sloth_rc_release(%90) : (i64) -> i64
    %93 = memref.load %alloca_9[%c0] : memref<1xi64>
    %94 = call @__sloth_str_slice(%93, %c1_i64, %c3_i64) : (i64, i64, i64) -> i64
    %95 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %95 : memref<11xi64> -> index
    %96 = arith.index_cast %intptr_13 : index to i64
    %97 = call @__sloth_any_from(%96, %94) : (i64, i64) -> i64
    call @sloth_main__print(%97) : (i64) -> ()
    %98 = call @__sloth_rc_release(%94) : (i64) -> i64
    %99 = call @__sloth_rc_release(%97) : (i64) -> i64
    %alloca_14 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_14[%c0] : memref<1xi64>
    %100 = call @__sloth_str_push(%c0_i64, %c191009621918561_i64, %c6_i64) : (i64, i64, i64) -> i64
    %101 = call @__sloth_str_finish(%100) : (i64) -> i64
    %102 = call @__sloth_cls_info(%c0_i64, %c0_i64) : (i64, i64) -> i64
    %103 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %104 = call @__sloth_cls_name(%102, %103, %c8_i64) : (i64, i64, i64) -> i64
    %105 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %106 = call @__sloth_obj_new(%102, %c3_i64, %105) : (i64, i64, i64) -> i64
    %107 = call @sloth_vtb_main_StrChars() : () -> i64
    %108 = call @__sloth_obj_set_vtable(%106, %107) : (i64, i64) -> i64
    call @sloth_main_StrChars____init__(%106, %101) : (i64, i64) -> ()
    %109 = call @__sloth_rc_release(%101) : (i64) -> i64
    %110 = llvm.call @sloth_main_StrChars__iter(%106) : (i64) -> i64
    %alloca_15 = memref.alloca() : memref<1xi64>
    memref.store %110, %alloca_15[%c0] : memref<1xi64>
    %intptr_16 = memref.extract_aligned_pointer_as_index %alloca_15 : memref<1xi64> -> index
    %111 = arith.index_cast %intptr_16 : index to i64
    %112 = call @__sloth_fiber_track(%111) : (i64) -> i64
    cf.br ^bb5
  ^bb5:  // 2 preds: ^bb4, ^bb7
    %113 = memref.load %alloca_15[%c0] : memref<1xi64>
    %114 = llvm.call @sloth_main_StrChars__next(%113) : (i64) -> i64
    %115 = arith.cmpi eq, %114, %c0_i64 : i64
    cf.cond_br %115, ^bb8, ^bb6
  ^bb6:  // pred: ^bb5
    %116 = call @__sloth_box_get(%114) : (i64) -> i64
    %alloca_17 = memref.alloca() : memref<1xi64>
    memref.store %116, %alloca_17[%c0] : memref<1xi64>
    %117 = call @__sloth_rc_release(%114) : (i64) -> i64
    %118 = memref.load %alloca_14[%c0] : memref<1xi64>
    %119 = memref.load %alloca_17[%c0] : memref<1xi64>
    %120 = arith.addi %118, %119 : i64
    memref.store %120, %alloca_14[%c0] : memref<1xi64>
    cf.br ^bb7
  ^bb7:  // pred: ^bb6
    cf.br ^bb5
  ^bb8:  // pred: ^bb5
    %121 = call @__sloth_rc_release(%106) : (i64) -> i64
    %122 = memref.load %alloca_15[%c0] : memref<1xi64>
    %123 = call @__sloth_rc_release(%122) : (i64) -> i64
    %intptr_18 = memref.extract_aligned_pointer_as_index %alloca_15 : memref<1xi64> -> index
    %124 = arith.index_cast %intptr_18 : index to i64
    %125 = call @__sloth_fiber_untrack(%124) : (i64) -> i64
    %126 = memref.load %alloca_14[%c0] : memref<1xi64>
    %127 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_19 = memref.extract_aligned_pointer_as_index %127 : memref<11xi64> -> index
    %128 = arith.index_cast %intptr_19 : index to i64
    %129 = call @__sloth_any_from(%128, %126) : (i64, i64) -> i64
    call @sloth_main__print(%129) : (i64) -> ()
    %130 = call @__sloth_rc_release(%129) : (i64) -> i64
    %131 = call @__sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %132 = call @__sloth_str_finish(%131) : (i64) -> i64
    %alloca_20 = memref.alloca() : memref<1xi64>
    %133 = call @__sloth_rc_retain(%132) : (i64) -> i64
    memref.store %133, %alloca_20[%c0] : memref<1xi64>
    %intptr_21 = memref.extract_aligned_pointer_as_index %alloca_20 : memref<1xi64> -> index
    %134 = arith.index_cast %intptr_21 : index to i64
    %135 = call @__sloth_fiber_track(%134) : (i64) -> i64
    %136 = call @__sloth_rc_release(%132) : (i64) -> i64
    %137 = memref.load %alloca_20[%c0] : memref<1xi64>
    %138 = call @__sloth_str_push(%c0_i64, %c121_i64, %c1_i64) : (i64, i64, i64) -> i64
    %139 = call @__sloth_str_finish(%138) : (i64) -> i64
    %140 = call @__sloth_str_concat(%137, %139) : (i64, i64) -> i64
    %141 = call @__sloth_str_push(%c0_i64, %c122_i64, %c1_i64) : (i64, i64, i64) -> i64
    %142 = call @__sloth_str_finish(%141) : (i64) -> i64
    %143 = call @__sloth_str_concat(%140, %142) : (i64, i64) -> i64
    %144 = memref.load %alloca_20[%c0] : memref<1xi64>
    %145 = call @__sloth_rc_release(%144) : (i64) -> i64
    %146 = call @__sloth_rc_retain(%143) : (i64) -> i64
    memref.store %146, %alloca_20[%c0] : memref<1xi64>
    %147 = call @__sloth_rc_release(%139) : (i64) -> i64
    %148 = call @__sloth_rc_release(%140) : (i64) -> i64
    %149 = call @__sloth_rc_release(%142) : (i64) -> i64
    %150 = call @__sloth_rc_release(%143) : (i64) -> i64
    %151 = memref.load %alloca_20[%c0] : memref<1xi64>
    %152 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_22 = memref.extract_aligned_pointer_as_index %152 : memref<11xi64> -> index
    %153 = arith.index_cast %intptr_22 : index to i64
    %154 = call @__sloth_any_from(%153, %151) : (i64, i64) -> i64
    call @sloth_main__print(%154) : (i64) -> ()
    %155 = call @__sloth_rc_release(%154) : (i64) -> i64
    %alloca_23 = memref.alloca() : memref<1xi64>
    memref.store %c4609434218613702656_i64, %alloca_23[%c0] : memref<1xi64>
    %156 = call @__sloth_str_push(%c0_i64, %c15726_i64, %c2_i64) : (i64, i64, i64) -> i64
    %157 = memref.load %alloca_5[%c0] : memref<1xi64>
    %158 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_24 = memref.extract_aligned_pointer_as_index %158 : memref<11xi64> -> index
    %159 = arith.index_cast %intptr_24 : index to i64
    %160 = call @__sloth_any_from(%159, %157) : (i64, i64) -> i64
    %161 = call @__sloth_rt_write(%160) : (i64) -> i64
    %162 = call @__sloth_str_pushp(%156, %161) : (i64, i64) -> i64
    %163 = call @__sloth_str_push(%162, %c4023840_i64, %c3_i64) : (i64, i64, i64) -> i64
    %164 = memref.load %alloca_23[%c0] : memref<1xi64>
    %165 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    %intptr_25 = memref.extract_aligned_pointer_as_index %165 : memref<11xi64> -> index
    %166 = arith.index_cast %intptr_25 : index to i64
    %167 = call @__sloth_any_from(%166, %164) : (i64, i64) -> i64
    %168 = call @__sloth_rt_write(%167) : (i64) -> i64
    %169 = call @__sloth_str_pushp(%163, %168) : (i64, i64) -> i64
    %170 = call @__sloth_str_push(%169, %c4022816_i64, %c3_i64) : (i64, i64, i64) -> i64
    %171 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    %intptr_26 = memref.extract_aligned_pointer_as_index %171 : memref<11xi64> -> index
    %172 = arith.index_cast %intptr_26 : index to i64
    %173 = call @__sloth_any_from(%172, %c1_i64) : (i64, i64) -> i64
    %174 = call @__sloth_rt_write(%173) : (i64) -> i64
    %175 = call @__sloth_str_pushp(%170, %174) : (i64, i64) -> i64
    %176 = call @__sloth_str_finish(%175) : (i64) -> i64
    %177 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_27 = memref.extract_aligned_pointer_as_index %177 : memref<11xi64> -> index
    %178 = arith.index_cast %intptr_27 : index to i64
    %179 = call @__sloth_any_from(%178, %176) : (i64, i64) -> i64
    call @sloth_main__print(%179) : (i64) -> ()
    %180 = call @__sloth_rc_release(%160) : (i64) -> i64
    %181 = call @__sloth_rc_release(%161) : (i64) -> i64
    %182 = call @__sloth_rc_release(%167) : (i64) -> i64
    %183 = call @__sloth_rc_release(%168) : (i64) -> i64
    %184 = call @__sloth_rc_release(%173) : (i64) -> i64
    %185 = call @__sloth_rc_release(%174) : (i64) -> i64
    %186 = call @__sloth_rc_release(%176) : (i64) -> i64
    %187 = call @__sloth_rc_release(%179) : (i64) -> i64
    %188 = memref.load %alloca_9[%c0] : memref<1xi64>
    %189 = call @__sloth_rc_release(%188) : (i64) -> i64
    %intptr_28 = memref.extract_aligned_pointer_as_index %alloca_9 : memref<1xi64> -> index
    %190 = arith.index_cast %intptr_28 : index to i64
    %191 = call @__sloth_fiber_untrack(%190) : (i64) -> i64
    %192 = memref.load %alloca_20[%c0] : memref<1xi64>
    %193 = call @__sloth_rc_release(%192) : (i64) -> i64
    %intptr_29 = memref.extract_aligned_pointer_as_index %alloca_20 : memref<1xi64> -> index
    %194 = arith.index_cast %intptr_29 : index to i64
    %195 = call @__sloth_fiber_untrack(%194) : (i64) -> i64
    %196 = memref.load %alloca[%c0] : memref<1xi64>
    %197 = call @__sloth_rc_release(%196) : (i64) -> i64
    %intptr_30 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %198 = arith.index_cast %intptr_30 : index to i64
    %199 = call @__sloth_fiber_untrack(%198) : (i64) -> i64
    cf.br ^bb9
  ^bb9:  // pred: ^bb8
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
    %c5_i64 = arith.constant 5 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_4 : !llvm.ptr
    %c1099511627779_i64 = arith.constant 1099511627779 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627778_i64 = arith.constant 1099511627778 : i64
    %2 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c0_i64 = arith.constant 0 : i64
    %c4 = arith.constant 4 : index
    %c3_i64 = arith.constant 3 : i64
    %c3 = arith.constant 3 : index
    %3 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c1_i64 = arith.constant 1 : i64
    %c0 = arith.constant 0 : index
    %c4_i64 = arith.constant 4 : i64
    %4 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c4_i64, %4[%c0] : memref<11xi64>
    memref.store %c1_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %3 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    %6 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c2_i64, %6[%c0] : memref<11xi64>
    memref.store %c0_i64, %6[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %6[%c2] : memref<11xi64>
    %7 = llvm.ptrtoint %2 : !llvm.ptr to i64
    memref.store %7, %6[%c3] : memref<11xi64>
    memref.store %c3_i64, %6[%c4] : memref<11xi64>
    memref.store %c0_i64, %6[%c9] : memref<11xi64>
    memref.store %c0_i64, %6[%c10] : memref<11xi64>
    %8 = memref.get_global @sloth_anyd_2 : memref<11xi64>
    memref.store %c1_i64, %8[%c0] : memref<11xi64>
    memref.store %c0_i64, %8[%c1] : memref<11xi64>
    memref.store %c1099511627778_i64, %8[%c2] : memref<11xi64>
    %9 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %9, %8[%c3] : memref<11xi64>
    memref.store %c4_i64, %8[%c4] : memref<11xi64>
    memref.store %c0_i64, %8[%c9] : memref<11xi64>
    memref.store %c0_i64, %8[%c10] : memref<11xi64>
    %10 = memref.get_global @sloth_anyd_3 : memref<11xi64>
    memref.store %c3_i64, %10[%c0] : memref<11xi64>
    memref.store %c0_i64, %10[%c1] : memref<11xi64>
    memref.store %c1099511627779_i64, %10[%c2] : memref<11xi64>
    %11 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %11, %10[%c3] : memref<11xi64>
    memref.store %c5_i64, %10[%c4] : memref<11xi64>
    memref.store %c0_i64, %10[%c9] : memref<11xi64>
    memref.store %c0_i64, %10[%c10] : memref<11xi64>
    return
  }
}

