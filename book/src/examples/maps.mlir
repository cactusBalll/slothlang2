module @main {
  llvm.mlir.global private constant @sloth_tynm_0("int\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("Entry<str, int>\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_2("str\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Entry_str_int : memref<1xi64> = dense<0> {mutable}
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
  llvm.func @sloth_main_Entry_str_int__cascade(%arg0: i64, %arg1: i64) -> i64 {
    %c0_i64 = arith.constant 0 : i64
    %0 = func.call @sloth_obj_field(%arg0, %c0_i64) : (i64, i64) -> i64
    %1 = func.call @sloth_rc_release(%0) : (i64) -> i64
    llvm.return %c0_i64 : i64
  }
  func.func private @sloth_vtb_main_Entry_str_int() -> i64 {
    %c3_i64 = arith.constant 3 : i64
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %0 = memref.get_global @sloth_main_g_vtb_Entry_str_int : memref<1xi64>
    %1 = memref.load %0[%c0] : memref<1xi64>
    %2 = arith.cmpi eq, %1, %c0_i64 : i64
    cf.cond_br %2, ^bb1, ^bb2
  ^bb1:  // pred: ^bb0
    %3 = call @sloth_vt_new(%c3_i64) : (i64) -> i64
    memref.store %3, %0[%c0] : memref<1xi64>
    cf.br ^bb2
  ^bb2:  // 2 preds: ^bb0, ^bb1
    %4 = memref.load %0[%c0] : memref<1xi64>
    return %4 : i64
  }
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c7_i64 = arith.constant 7 : i64
    %c5_i64 = arith.constant 5 : i64
    %c120_i64 = arith.constant 120 : i64
    %c61_i64 = arith.constant 61 : i64
    %0 = llvm.mlir.addressof @sloth_main_Entry_str_int__cascade : !llvm.ptr
    %c15_i64 = arith.constant 15 : i64
    %1 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
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
    %2 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %3 = call @sloth_str_finish(%2) : (i64) -> i64
    %4 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %5 = call @sloth_str_finish(%4) : (i64) -> i64
    %6 = call @sloth_map_new(%c1_i64) : (i64) -> i64
    %7 = call @sloth_rc_retain(%3) : (i64) -> i64
    %8 = call @sloth_map_str_set(%6, %3, %c1_i64) : (i64, i64, i64) -> i64
    %9 = call @sloth_rc_retain(%5) : (i64) -> i64
    %10 = call @sloth_map_str_set(%6, %5, %c2_i64) : (i64, i64, i64) -> i64
    %alloca = memref.alloca() : memref<1xi64>
    %11 = call @sloth_rc_retain(%6) : (i64) -> i64
    memref.store %11, %alloca[%c0] : memref<1xi64>
    %intptr = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %12 = arith.index_cast %intptr : index to i64
    %13 = call @sloth_fiber_track(%12) : (i64) -> i64
    %14 = call @sloth_rc_release(%3) : (i64) -> i64
    %15 = call @sloth_rc_release(%5) : (i64) -> i64
    %16 = call @sloth_rc_release(%6) : (i64) -> i64
    %17 = memref.load %alloca[%c0] : memref<1xi64>
    %18 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %19 = call @sloth_str_finish(%18) : (i64) -> i64
    %20 = call @sloth_map_str_get(%17, %19) : (i64, i64) -> i64
    %21 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %21 : memref<11xi64> -> index
    %22 = arith.index_cast %intptr_0 : index to i64
    %23 = call @sloth_any_from(%22, %20) : (i64, i64) -> i64
    call @sloth_main__print(%23) : (i64) -> ()
    %24 = call @sloth_rc_release(%19) : (i64) -> i64
    %25 = call @sloth_rc_release(%23) : (i64) -> i64
    %26 = memref.load %alloca[%c0] : memref<1xi64>
    %27 = call @sloth_str_push(%c0_i64, %c99_i64, %c1_i64) : (i64, i64, i64) -> i64
    %28 = call @sloth_str_finish(%27) : (i64) -> i64
    %29 = call @sloth_rc_retain(%28) : (i64) -> i64
    %30 = call @sloth_map_str_set(%26, %28, %c3_i64) : (i64, i64, i64) -> i64
    %31 = call @sloth_rc_release(%28) : (i64) -> i64
    %32 = memref.load %alloca[%c0] : memref<1xi64>
    %33 = call @sloth_map_len(%32) : (i64) -> i64
    %34 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %34 : memref<11xi64> -> index
    %35 = arith.index_cast %intptr_1 : index to i64
    %36 = call @sloth_any_from(%35, %33) : (i64, i64) -> i64
    call @sloth_main__print(%36) : (i64) -> ()
    %37 = call @sloth_rc_release(%36) : (i64) -> i64
    %38 = memref.load %alloca[%c0] : memref<1xi64>
    %39 = call @sloth_map_len(%38) : (i64) -> i64
    %40 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_2 = memref.extract_aligned_pointer_as_index %40 : memref<11xi64> -> index
    %41 = arith.index_cast %intptr_2 : index to i64
    %42 = call @sloth_any_from(%41, %39) : (i64, i64) -> i64
    call @sloth_main__print(%42) : (i64) -> ()
    %43 = call @sloth_rc_release(%42) : (i64) -> i64
    %44 = memref.load %alloca[%c0] : memref<1xi64>
    %45 = call @sloth_map_keys(%44) : (i64) -> i64
    %46 = call @sloth_arr_len(%45) : (i64) -> i64
    %alloca_3 = memref.alloca() : memref<1xi64>
    memref.store %c0_i64, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb1:  // 2 preds: ^bb0, ^bb3
    %47 = memref.load %alloca_3[%c0] : memref<1xi64>
    %48 = arith.cmpi slt, %47, %46 : i64
    cf.cond_br %48, ^bb2, ^bb4
  ^bb2:  // pred: ^bb1
    %49 = call @sloth_arr_get(%45, %47) : (i64, i64) -> i64
    %50 = call @sloth_map_str_get(%44, %49) : (i64, i64) -> i64
    %51 = call @sloth_cls_info(%c0_i64, %c2_i64) : (i64, i64) -> i64
    %52 = llvm.ptrtoint %1 : !llvm.ptr to i64
    %53 = call @sloth_cls_name(%51, %52, %c15_i64) : (i64, i64, i64) -> i64
    %54 = llvm.ptrtoint %0 : !llvm.ptr to i64
    %55 = call @sloth_obj_new(%51, %c2_i64, %54) : (i64, i64, i64) -> i64
    %56 = call @sloth_vtb_main_Entry_str_int() : () -> i64
    %57 = call @sloth_obj_set_vtable(%55, %56) : (i64, i64) -> i64
    %58 = call @sloth_obj_field(%55, %c0_i64) : (i64, i64) -> i64
    %59 = call @sloth_rc_release(%58) : (i64) -> i64
    %60 = call @sloth_rc_retain(%49) : (i64) -> i64
    %61 = call @sloth_obj_set_field(%55, %c0_i64, %60) : (i64, i64, i64) -> i64
    %62 = call @sloth_obj_set_field(%55, %c1_i64, %50) : (i64, i64, i64) -> i64
    %alloca_4 = memref.alloca() : memref<1xi64>
    memref.store %55, %alloca_4[%c0] : memref<1xi64>
    %63 = memref.load %alloca_4[%c0] : memref<1xi64>
    %64 = call @sloth_obj_field(%63, %c0_i64) : (i64, i64) -> i64
    %65 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_5 = memref.extract_aligned_pointer_as_index %65 : memref<11xi64> -> index
    %66 = arith.index_cast %intptr_5 : index to i64
    %67 = call @sloth_any_from(%66, %64) : (i64, i64) -> i64
    %68 = call @sloth_rt_write(%67) : (i64) -> i64
    %69 = call @sloth_str_pushp(%c0_i64, %68) : (i64, i64) -> i64
    %70 = call @sloth_str_push(%69, %c61_i64, %c1_i64) : (i64, i64, i64) -> i64
    %71 = memref.load %alloca_4[%c0] : memref<1xi64>
    %72 = call @sloth_obj_field(%71, %c1_i64) : (i64, i64) -> i64
    %73 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_6 = memref.extract_aligned_pointer_as_index %73 : memref<11xi64> -> index
    %74 = arith.index_cast %intptr_6 : index to i64
    %75 = call @sloth_any_from(%74, %72) : (i64, i64) -> i64
    %76 = call @sloth_rt_write(%75) : (i64) -> i64
    %77 = call @sloth_str_pushp(%70, %76) : (i64, i64) -> i64
    %78 = call @sloth_str_finish(%77) : (i64) -> i64
    %79 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_7 = memref.extract_aligned_pointer_as_index %79 : memref<11xi64> -> index
    %80 = arith.index_cast %intptr_7 : index to i64
    %81 = call @sloth_any_from(%80, %78) : (i64, i64) -> i64
    call @sloth_main__print(%81) : (i64) -> ()
    %82 = call @sloth_rc_release(%67) : (i64) -> i64
    %83 = call @sloth_rc_release(%68) : (i64) -> i64
    %84 = call @sloth_rc_release(%75) : (i64) -> i64
    %85 = call @sloth_rc_release(%76) : (i64) -> i64
    %86 = call @sloth_rc_release(%78) : (i64) -> i64
    %87 = call @sloth_rc_release(%81) : (i64) -> i64
    cf.br ^bb3
  ^bb3:  // pred: ^bb2
    %88 = call @sloth_rc_release(%55) : (i64) -> i64
    %89 = arith.addi %47, %c1_i64 : i64
    memref.store %89, %alloca_3[%c0] : memref<1xi64>
    cf.br ^bb1
  ^bb4:  // pred: ^bb1
    %90 = call @sloth_rc_release(%45) : (i64) -> i64
    %91 = memref.load %alloca[%c0] : memref<1xi64>
    %92 = call @sloth_map_keys(%91) : (i64) -> i64
    %alloca_8 = memref.alloca() : memref<1xi64>
    %93 = call @sloth_rc_retain(%92) : (i64) -> i64
    memref.store %93, %alloca_8[%c0] : memref<1xi64>
    %intptr_9 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %94 = arith.index_cast %intptr_9 : index to i64
    %95 = call @sloth_fiber_track(%94) : (i64) -> i64
    %96 = call @sloth_rc_release(%92) : (i64) -> i64
    %97 = memref.load %alloca_8[%c0] : memref<1xi64>
    %98 = call @sloth_arr_len(%97) : (i64) -> i64
    %99 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_10 = memref.extract_aligned_pointer_as_index %99 : memref<11xi64> -> index
    %100 = arith.index_cast %intptr_10 : index to i64
    %101 = call @sloth_any_from(%100, %98) : (i64, i64) -> i64
    call @sloth_main__print(%101) : (i64) -> ()
    %102 = call @sloth_rc_release(%101) : (i64) -> i64
    %103 = memref.load %alloca[%c0] : memref<1xi64>
    %104 = call @sloth_map_values(%103) : (i64) -> i64
    %alloca_11 = memref.alloca() : memref<1xi64>
    %105 = call @sloth_rc_retain(%104) : (i64) -> i64
    memref.store %105, %alloca_11[%c0] : memref<1xi64>
    %intptr_12 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %106 = arith.index_cast %intptr_12 : index to i64
    %107 = call @sloth_fiber_track(%106) : (i64) -> i64
    %108 = call @sloth_rc_release(%104) : (i64) -> i64
    %109 = memref.load %alloca_11[%c0] : memref<1xi64>
    %110 = call @sloth_arr_len(%109) : (i64) -> i64
    %111 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_13 = memref.extract_aligned_pointer_as_index %111 : memref<11xi64> -> index
    %112 = arith.index_cast %intptr_13 : index to i64
    %113 = call @sloth_any_from(%112, %110) : (i64, i64) -> i64
    call @sloth_main__print(%113) : (i64) -> ()
    %114 = call @sloth_rc_release(%113) : (i64) -> i64
    %115 = call @sloth_map_new(%c1_i64) : (i64) -> i64
    %alloca_14 = memref.alloca() : memref<1xi64>
    %116 = call @sloth_rc_retain(%115) : (i64) -> i64
    memref.store %116, %alloca_14[%c0] : memref<1xi64>
    %intptr_15 = memref.extract_aligned_pointer_as_index %alloca_14 : memref<1xi64> -> index
    %117 = arith.index_cast %intptr_15 : index to i64
    %118 = call @sloth_fiber_track(%117) : (i64) -> i64
    %119 = call @sloth_rc_release(%115) : (i64) -> i64
    %120 = memref.load %alloca_14[%c0] : memref<1xi64>
    %121 = call @sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %122 = call @sloth_str_finish(%121) : (i64) -> i64
    %123 = call @sloth_rc_retain(%122) : (i64) -> i64
    %124 = call @sloth_map_str_set(%120, %122, %c1_i64) : (i64, i64, i64) -> i64
    %125 = call @sloth_rc_release(%122) : (i64) -> i64
    %126 = memref.load %alloca_14[%c0] : memref<1xi64>
    %127 = call @sloth_str_push(%c0_i64, %c120_i64, %c1_i64) : (i64, i64, i64) -> i64
    %128 = call @sloth_str_finish(%127) : (i64) -> i64
    %129 = call @sloth_map_str_get(%126, %128) : (i64, i64) -> i64
    %130 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_16 = memref.extract_aligned_pointer_as_index %130 : memref<11xi64> -> index
    %131 = arith.index_cast %intptr_16 : index to i64
    %132 = call @sloth_any_from(%131, %129) : (i64, i64) -> i64
    call @sloth_main__print(%132) : (i64) -> ()
    %133 = call @sloth_rc_release(%128) : (i64) -> i64
    %134 = call @sloth_rc_release(%132) : (i64) -> i64
    %135 = call @sloth_map_new(%c5_i64) : (i64) -> i64
    %alloca_17 = memref.alloca() : memref<1xi64>
    %136 = call @sloth_rc_retain(%135) : (i64) -> i64
    memref.store %136, %alloca_17[%c0] : memref<1xi64>
    %intptr_18 = memref.extract_aligned_pointer_as_index %alloca_17 : memref<1xi64> -> index
    %137 = arith.index_cast %intptr_18 : index to i64
    %138 = call @sloth_fiber_track(%137) : (i64) -> i64
    %139 = call @sloth_rc_release(%135) : (i64) -> i64
    %140 = call @sloth_map_new(%c1_i64) : (i64) -> i64
    %141 = memref.load %alloca_17[%c0] : memref<1xi64>
    %142 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %143 = call @sloth_str_finish(%142) : (i64) -> i64
    %144 = call @sloth_rc_retain(%143) : (i64) -> i64
    %145 = call @sloth_rc_retain(%140) : (i64) -> i64
    %146 = call @sloth_map_str_set(%141, %143, %145) : (i64, i64, i64) -> i64
    %147 = call @sloth_rc_release(%140) : (i64) -> i64
    %148 = call @sloth_rc_release(%143) : (i64) -> i64
    %149 = memref.load %alloca_17[%c0] : memref<1xi64>
    %150 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %151 = call @sloth_str_finish(%150) : (i64) -> i64
    %152 = call @sloth_map_str_get(%149, %151) : (i64, i64) -> i64
    %153 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %154 = call @sloth_str_finish(%153) : (i64) -> i64
    %155 = call @sloth_rc_retain(%154) : (i64) -> i64
    %156 = call @sloth_map_str_set(%152, %154, %c7_i64) : (i64, i64, i64) -> i64
    %157 = call @sloth_rc_release(%151) : (i64) -> i64
    %158 = call @sloth_rc_release(%154) : (i64) -> i64
    %159 = memref.load %alloca_17[%c0] : memref<1xi64>
    %160 = call @sloth_str_push(%c0_i64, %c97_i64, %c1_i64) : (i64, i64, i64) -> i64
    %161 = call @sloth_str_finish(%160) : (i64) -> i64
    %162 = call @sloth_map_str_get(%159, %161) : (i64, i64) -> i64
    %163 = call @sloth_str_push(%c0_i64, %c98_i64, %c1_i64) : (i64, i64, i64) -> i64
    %164 = call @sloth_str_finish(%163) : (i64) -> i64
    %165 = call @sloth_map_str_get(%162, %164) : (i64, i64) -> i64
    %166 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_19 = memref.extract_aligned_pointer_as_index %166 : memref<11xi64> -> index
    %167 = arith.index_cast %intptr_19 : index to i64
    %168 = call @sloth_any_from(%167, %165) : (i64, i64) -> i64
    call @sloth_main__print(%168) : (i64) -> ()
    %169 = call @sloth_rc_release(%161) : (i64) -> i64
    %170 = call @sloth_rc_release(%164) : (i64) -> i64
    %171 = call @sloth_rc_release(%168) : (i64) -> i64
    %172 = memref.load %alloca_14[%c0] : memref<1xi64>
    %173 = call @sloth_rc_release(%172) : (i64) -> i64
    %intptr_20 = memref.extract_aligned_pointer_as_index %alloca_14 : memref<1xi64> -> index
    %174 = arith.index_cast %intptr_20 : index to i64
    %175 = call @sloth_fiber_untrack(%174) : (i64) -> i64
    %176 = memref.load %alloca_17[%c0] : memref<1xi64>
    %177 = call @sloth_rc_release(%176) : (i64) -> i64
    %intptr_21 = memref.extract_aligned_pointer_as_index %alloca_17 : memref<1xi64> -> index
    %178 = arith.index_cast %intptr_21 : index to i64
    %179 = call @sloth_fiber_untrack(%178) : (i64) -> i64
    %180 = memref.load %alloca[%c0] : memref<1xi64>
    %181 = call @sloth_rc_release(%180) : (i64) -> i64
    %intptr_22 = memref.extract_aligned_pointer_as_index %alloca : memref<1xi64> -> index
    %182 = arith.index_cast %intptr_22 : index to i64
    %183 = call @sloth_fiber_untrack(%182) : (i64) -> i64
    %184 = memref.load %alloca_8[%c0] : memref<1xi64>
    %185 = call @sloth_rc_release(%184) : (i64) -> i64
    %intptr_23 = memref.extract_aligned_pointer_as_index %alloca_8 : memref<1xi64> -> index
    %186 = arith.index_cast %intptr_23 : index to i64
    %187 = call @sloth_fiber_untrack(%186) : (i64) -> i64
    %188 = memref.load %alloca_11[%c0] : memref<1xi64>
    %189 = call @sloth_rc_release(%188) : (i64) -> i64
    %intptr_24 = memref.extract_aligned_pointer_as_index %alloca_11 : memref<1xi64> -> index
    %190 = arith.index_cast %intptr_24 : index to i64
    %191 = call @sloth_fiber_untrack(%190) : (i64) -> i64
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

