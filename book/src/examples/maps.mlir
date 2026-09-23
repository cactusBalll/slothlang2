module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("Entry<str, int>\00") {addr_space = 0 : i32}
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
    %c7_i64 = arith.constant 7 : i64
    %c5_i64 = arith.constant 5 : i64
    %c120_i64 = arith.constant 120 : i64
    %c61_i64 = arith.constant 61 : i64
    %c15_i64 = arith.constant 15 : i64
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c99_i64 = arith.constant 99 : i64
    %c3_i64 = arith.constant 3 : i64
    %c0 = arith.constant 0 : index
    %c2_i64 = arith.constant 2 : i64
    %c98_i64 = arith.constant 98 : i64
    %c1_i64 = arith.constant 1 : i64
    %c97_i64 = arith.constant 97 : i64
    %c0_i64 = arith.constant 0 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %1 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %2 = call @sloth_str_finish(%1) : (i64) -> i64
    %3 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %4 = call @sloth_str_finish(%3) : (i64) -> i64
    %5 = call @sloth_map_new(%c1_i64) : (i64) -> i64
    %6 = call @sloth_rc_retain(%2) : (i64) -> i64
    %7 = call @sloth_map_str_set(%5, %2, %c1_i64) : (i64, i64, i64) -> i64
    %8 = call @sloth_rc_retain(%4) : (i64) -> i64
    %9 = call @sloth_map_str_set(%5, %4, %c2_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %10 = call @sloth_rc_retain(%5) : (i64) -> i64
    memref.store %10, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %11 = arith.index_cast %intptr : index to i64
    %12 = call @sloth_fiber_track(%11) : (i64) -> i64
    %13 = call @sloth_rc_release(%2) : (i64) -> i64
    %14 = call @sloth_rc_release(%4) : (i64) -> i64
    %15 = call @sloth_rc_release(%5) : (i64) -> i64
    %16 = memref.load %alloca[%c0] : memref<1xi64>
    %17 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %18 = call @sloth_str_finish(%17) : (i64) -> i64
    %19 = call @sloth_map_str_get(%16, %18) : (i64, i64) -> i64
    %20 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %20 : memref<11xi64> -> index
    %21 = arith.index_cast %intptr_0 : index to i64
    %22 = call @sloth_any_from(%21, %19) : (i64, i64) -> i64
    call @sloth_main__print(%22) : (i64) -> ()
    %23 = call @sloth_rc_release(%18) : (i64) -> i64
    %24 = call @sloth_rc_release(%22) : (i64) -> i64
    %25 = memref.load %alloca[%c0] : memref<1xi64>
    %26 = call @sloth_str_push(%c0_i64, %c99_i64, %c1_i64) : (i64, i64, i64) -> i64
    %27 = call @sloth_str_finish(%26) : (i64) -> i64
    %28 = call @sloth_rc_retain(%27) : (i64) -> i64
    %29 = call @sloth_map_str_set(%25, %27, %c3_i64) : (i64, i64, i64) -> i64
    %30 = call @sloth_rc_release(%27) : (i64) -> i64
    %31 = memref.load %alloca[%c0] : memref<1xi64>
    %32 = call @sloth_map_len(%31) : (i64) -> i64
    %33 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %33 : memref<11xi64> -> index
    %34 = arith.index_cast %intptr_1 : index to i64
    %35 = call @sloth_any_from(%34, %32) : (i64, i64) -> i64
    call @sloth_main__print(%35) : (i64) -> ()
    %36 = call @sloth_rc_release(%35) : (i64) -> i64
    %37 = memref.load %alloca[%c0] : memref<1xi64>
    %38 = call @sloth_map_len(%37) : (i64) -> i64
    %39 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %39 : memref<11xi64> -> index
    %40 = arith.index_cast %intptr_2 : index to i64
    %41 = call @sloth_any_from(%40, %38) : (i64, i64) -> i64
    call @sloth_main__print(%41) : (i64) -> ()
    %42 = call @sloth_rc_release(%41) : (i64) -> i64
    %43 = memref.load %alloca[%c0] : memref<1xi64>
    %44 = call @sloth_map_keys(%43) : (i64) -> i64
    %45 = call @sloth_arr_len(%44) : (i64) -> i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %46 = memref.load %alloca_3[%c0] : memref<1xi64>
    %47 = arith.cmpi slt, %46, %45 : i64
    cf.cond_br %47, ^bb2, ^bb4
  ^bb2:  // pred: ^bb1
    %48 = call @sloth_arr_get(%44, %46) : (i64, i64) -> i64
    %49 = call @sloth_map_str_get(%43, %48) : (i64, i64) -> i64
    %50 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %51 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %52 = call @sloth_cls_name(%50, %51, %c15_i64) : (i64, i64, i64) -> i64
    %53 = call @sloth_cls_refmask(%50, %c1_i64, %c2_i64) : (i64, i64, i64) -> i64
    %54 = call @sloth_obj_new(%50, %c2_i64) : (i64, i64) -> i64
    %55 = call @sloth_obj_field(%54, %c0_i64) : (i64, i64) -> i64
    %56 = call @sloth_rc_release(%55) : (i64) -> i64
    %57 = call @sloth_rc_retain(%48) : (i64) -> i64
    %58 = call @sloth_obj_set_field(%54, %c0_i64, %57) : (i64, i64, i64) -> i64
    %59 = call @sloth_obj_set_field(%54, %c1_i64, %49) : (i64, i64, i64) -> i64
    %alloca_4 = memref.alloca() : memref<1xi64>
    memref.store %54, %alloca_4[%c0] : memref<1xi64>
    %60 = memref.load %alloca_4[%c0] : memref<1xi64>
    %61 = call @sloth_obj_field(%60, %c0_i64) : (i64, i64) -> i64
    %62 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %62 : memref<11xi64> -> index
    %63 = arith.index_cast %intptr_5 : index to i64
    %64 = call @sloth_any_from(%63, %61) : (i64, i64) -> i64
    %65 = call @sloth_rt_write(%64) : (i64) -> i64
    %66 = call @sloth_str_pushp(%c0_i64, %65) : (i64, i64) -> i64
    %67 = call @sloth_str_push(%66, %c61_i64, %c1_i64) : (i64, i64, i64) -> i64
    %68 = memref.load %alloca_4[%c0] : memref<1xi64>
    %69 = call @sloth_obj_field(%68, %c1_i64) : (i64, i64) -> i64
    %70 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %70 : memref<11xi64> -> index
    %71 = arith.index_cast %intptr_6 : index to i64
    %72 = call @sloth_any_from(%71, %69) : (i64, i64) -> i64
    %73 = call @sloth_rt_write(%72) : (i64) -> i64
    %74 = call @sloth_str_pushp(%67, %73) : (i64, i64) -> i64
    %75 = call @sloth_str_finish(%74) : (i64) -> i64
    %76 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %76 : memref<11xi64> -> index
    %77 = arith.index_cast %intptr_7 : index to i64
    %78 = call @sloth_any_from(%77, %75) : (i64, i64) -> i64
    call @sloth_main__print(%78) : (i64) -> ()
    %79 = call @sloth_rc_release(%64) : (i64) -> i64
    %80 = call @sloth_rc_release(%65) : (i64) -> i64
    %81 = call @sloth_rc_release(%72) : (i64) -> i64
    %82 = call @sloth_rc_release(%73) : (i64) -> i64
    %83 = call @sloth_rc_release(%75) : (i64) -> i64
    %84 = call @sloth_rc_release(%78) : (i64) -> i64
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %85 = call @sloth_rc_release(%54) : (i64) -> i64
    %86 = arith.addi %46, %c1_i64 : i64
    memref.store %86, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %87 = call @sloth_rc_release(%44) : (i64) -> i64
    %88 = memref.load %alloca[%c0] : memref<1xi64>
    %89 = call @sloth_map_keys(%88) : (i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    %90 = call @sloth_rc_retain(%89) : (i64) -> i64
    memref.store %90, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %91 = arith.index_cast %intptr_9 : index to i64
    %92 = call @sloth_fiber_track(%91) : (i64) -> i64
    %93 = call @sloth_rc_release(%89) : (i64) -> i64
    %94 = memref.load %alloca_8[%c0] : memref<1xi64>
    %95 = call @sloth_arr_len(%94) : (i64) -> i64
    %96 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %96 : memref<11xi64> -> index
    %97 = arith.index_cast %intptr_10 : index to i64
    %98 = call @sloth_any_from(%97, %95) : (i64, i64) -> i64
    call @sloth_main__print(%98) : (i64) -> ()
    %99 = call @sloth_rc_release(%98) : (i64) -> i64
    %100 = memref.load %alloca[%c0] : memref<1xi64>
    %101 = call @sloth_map_values(%100) : (i64) -> i64
    %alloca_11 = memref.alloca() : memref<1xi64>
    %102 = call @sloth_rc_retain(%101) : (i64) -> i64
    memref.store %102, %alloca_11[%c0] : memref<1xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %103 = arith.index_cast %intptr_12 : index to i64
    %104 = call @sloth_fiber_track(%103) : (i64) -> i64
    %105 = call @sloth_rc_release(%101) : (i64) -> i64
    %106 = memref.load %alloca_11[%c0] : memref<1xi64>
    %107 = call @sloth_arr_len(%106) : (i64) -> i64
    %108 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %108 : memref<11xi64> -> index
    %109 = arith.index_cast %intptr_13 : index to i64
    %110 = call @sloth_any_from(%109, %107) : (i64, i64) -> i64
    call @sloth_main__print(%110) : (i64) -> ()
    %111 = call @sloth_rc_release(%110) : (i64) -> i64
    %112 = call @sloth_map_new(%c1_i64) : (i64) -> i64
    %alloca_14 = memref.alloca() : memref<1xi64>
    %113 = call @sloth_rc_retain(%112) : (i64) -> i64
    memref.store %113, %alloca_14[%c0] : memref<1xi64>
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_14 : memref<1xi64> -> index
    %114 = arith.index_cast %intptr_15 : index to i64
    %115 = call @sloth_fiber_track(%114) : (i64) -> i64
    %116 = call @sloth_rc_release(%112) : (i64) -> i64
    %117 = memref.load %alloca_14[%c0] : memref<1xi64>
    %118 = call @sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %119 = call @sloth_str_finish(%118) : (i64) -> i64
    %120 = call @sloth_rc_retain(%119) : (i64) -> i64
    %121 = call @sloth_map_str_set(%117, %119, %c1_i64) : (i64, i64, i64) -> i64
    %122 = call @sloth_rc_release(%119) : (i64) -> i64
    %123 = memref.load %alloca_14[%c0] : memref<1xi64>
    %124 = call @sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %125 = call @sloth_str_finish(%124) : (i64) -> i64
    %126 = call @sloth_map_str_get(%123, %125) : (i64, i64) -> i64
    %127 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_16 = memref.extract_aligned_pointer_as_index %127 : memref<11xi64> -> index
    %128 = arith.index_cast %intptr_16 : index to i64
    %129 = call @sloth_any_from(%128, %126) : (i64, i64) -> i64
    call @sloth_main__print(%129) : (i64) -> ()
    %130 = call @sloth_rc_release(%125) : (i64) -> i64
    %131 = call @sloth_rc_release(%129) : (i64) -> i64
    %132 = call @sloth_map_new(%c5_i64) : (i64) -> i64
    %alloca_17 = memref.alloca() : memref<1xi64>
    %133 = call @sloth_rc_retain(%132) : (i64) -> i64
    memref.store %133, %alloca_17[%c0] : memref<1xi64>
    %intptr_18 = memref.extract_aligned_pointer_as_index %alloca_17 : memref<1xi64> -> index
    %134 = arith.index_cast %intptr_18 : index to i64
    %135 = call @sloth_fiber_track(%134) : (i64) -> i64
    %136 = call @sloth_rc_release(%132) : (i64) -> i64
    %137 = call @sloth_map_new(%c1_i64) : (i64) -> i64
    %138 = memref.load %alloca_17[%c0] : memref<1xi64>
    %139 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %140 = call @sloth_str_finish(%139) : (i64) -> i64
    %141 = call @sloth_rc_retain(%140) : (i64) -> i64
    %142 = call @sloth_rc_retain(%137) : (i64) -> i64
    %143 = call @sloth_map_str_set(%138, %140, %142) : (i64, i64, i64) -> i64
    %144 = call @sloth_rc_release(%137) : (i64) -> i64
    %145 = call @sloth_rc_release(%140) : (i64) -> i64
    %146 = memref.load %alloca_17[%c0] : memref<1xi64>
    %147 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %148 = call @sloth_str_finish(%147) : (i64) -> i64
    %149 = call @sloth_map_str_get(%146, %148) : (i64, i64) -> i64
    %150 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %151 = call @sloth_str_finish(%150) : (i64) -> i64
    %152 = call @sloth_rc_retain(%151) : (i64) -> i64
    %153 = call @sloth_map_str_set(%149, %151, %c7_i64) : (i64, i64, i64) -> i64
    %154 = call @sloth_rc_release(%148) : (i64) -> i64
    %155 = call @sloth_rc_release(%151) : (i64) -> i64
    %156 = memref.load %alloca_17[%c0] : memref<1xi64>
    %157 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %158 = call @sloth_str_finish(%157) : (i64) -> i64
    %159 = call @sloth_map_str_get(%156, %158) : (i64, i64) -> i64
    %160 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %161 = call @sloth_str_finish(%160) : (i64) -> i64
    %162 = call @sloth_map_str_get(%159, %161) : (i64, i64) -> i64
    %163 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_19 = memref.extract_aligned_pointer_as_index %163 : memref<11xi64> -> index
    %164 = arith.index_cast %intptr_19 : index to i64
    %165 = call @sloth_any_from(%164, %162) : (i64, i64) -> i64
    call @sloth_main__print(%165) : (i64) -> ()
    %166 = call @sloth_rc_release(%158) : (i64) -> i64
    %167 = call @sloth_rc_release(%161) : (i64) -> i64
    %168 = call @sloth_rc_release(%165) : (i64) -> i64
    %169 = memref.load %alloca_14[%c0] : memref<1xi64>
    %170 = call @sloth_rc_release(%169) : (i64) -> i64
    %intptr_20 = memref.extract_aligned_pointer_as_index %alloca_14 : memref<1xi64> -> index
    %171 = arith.index_cast %intptr_20 : index to i64
    %172 = call @sloth_fiber_untrack(%171) : (i64) -> i64
    %173 = memref.load %alloca[%c0] : memref<1xi64>
    %174 = call @sloth_rc_release(%173) : (i64) -> i64
    %intptr_21 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %175 = arith.index_cast %intptr_21 : index to i64
    %176 = call @sloth_fiber_untrack(%175) : (i64) -> i64
    %177 = memref.load %alloca_17[%c0] : memref<1xi64>
    %178 = call @sloth_rc_release(%177) : (i64) -> i64
    %intptr_22 = memref.extract_aligned_pointer_as_index %alloca_17 : memref<1xi64> -> index
    %179 = arith.index_cast %intptr_22 : index to i64
    %180 = call @sloth_fiber_untrack(%179) : (i64) -> i64
    %181 = memref.load %alloca_11[%c0] : memref<1xi64>
    %182 = call @sloth_rc_release(%181) : (i64) -> i64
    %intptr_23 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %183 = arith.index_cast %intptr_23 : index to i64
    %184 = call @sloth_fiber_untrack(%183) : (i64) -> i64
    %185 = memref.load %alloca_8[%c0] : memref<1xi64>
    %186 = call @sloth_rc_release(%185) : (i64) -> i64
    %intptr_24 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %187 = arith.index_cast %intptr_24 : index to i64
    %188 = call @sloth_fiber_untrack(%187) : (i64) -> i64
    cf.br ^bb5
  ^bb5:  // pred: ^bb4
    return
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_2 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c1_i64 = arith.constant 1 : i64
    %c4_i64 = arith.constant 4 : i64
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

