module @main {
  memref.global @sloth_main_g_g : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_fixed : memref<1xi64> = dense<0> {mutable}

  func.func @sloth_main__ginit() -> () {
    %v1000 = arith.constant 10 : i64
    %v1001 = memref.get_global @sloth_main_g_g : memref<1xi64>
    %v1002 = arith.constant 0 : index
    memref.store %v1000, %v1001[%v1002] : memref<1xi64>
    %v1003 = arith.constant 3 : i64
    %v1004 = memref.get_global @sloth_main_g_fixed : memref<1xi64>
    %v1005 = arith.constant 0 : index
    memref.store %v1003, %v1004[%v1005] : memref<1xi64>
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 1 : i64
    %v1002 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    memref.store %v1001, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 26984 : i64
    %v1006 = arith.constant 2 : i64
    %v1007 = call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = call @sloth_str_finish(%v1007) : (i64) -> i64
    %v1009 = memref.alloca() : memref<1xi64>
    %v1010 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1009[%v1011] : memref<1xi64>
    %v1012 = memref.extract_aligned_pointer_as_index %v1009 : memref<1xi64> -> index
    %v1013 = arith.index_cast %v1012 : index to i64
    call @sloth_fiber_track(%v1013) : (i64) -> i64
    call @sloth_rc_release(%v1008) : (i64) -> i64
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1002[%v1014] : memref<1xi64>
    %v1016 = arith.constant 1 : i64
    %v1017 = arith.addi %v1015, %v1016 : i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1002[%v1018] : memref<1xi64>
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1002[%v1019] : memref<1xi64>
    %v1021 = call @sloth_rt_print_i64(%v1020) : (i64) -> i64
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1009[%v1022] : memref<1xi64>
    %v1024 = call @sloth_rt_print_str(%v1023) : (i64) -> i64
    %v1025 = arith.constant 100 : i64
    %v1026 = memref.alloca() : memref<1xi64>
    %v1027 = arith.constant 0 : index
    memref.store %v1025, %v1026[%v1027] : memref<1xi64>
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1026[%v1028] : memref<1xi64>
    %v1030 = call @sloth_rt_print_i64(%v1029) : (i64) -> i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1002[%v1031] : memref<1xi64>
    %v1033 = call @sloth_rt_print_i64(%v1032) : (i64) -> i64
    %v1034 = memref.get_global @sloth_main_g_g : memref<1xi64>
    %v1035 = arith.constant 0 : index
    %v1036 = memref.load %v1034[%v1035] : memref<1xi64>
    %v1037 = memref.get_global @sloth_main_g_fixed : memref<1xi64>
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1037[%v1038] : memref<1xi64>
    %v1040 = arith.addi %v1036, %v1039 : i64
    %v1041 = call @sloth_rt_print_i64(%v1040) : (i64) -> i64
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1009[%v1042] : memref<1xi64>
    call @sloth_rc_release(%v1043) : (i64) -> i64
    %v1044 = memref.extract_aligned_pointer_as_index %v1009 : memref<1xi64> -> index
    %v1045 = arith.index_cast %v1044 : index to i64
    call @sloth_fiber_untrack(%v1045) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

