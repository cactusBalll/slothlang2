module @main {
  memref.global @sloth_main_g_g : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_fixed : memref<1xi64> = dense<0> {mutable}

  func.func @sloth_main__ginit() -> () {
    %v1000 = arith.constant 20 : i64
    %v1001 = memref.get_global @sloth_main_g_g : memref<1xi64>
    %v1002 = arith.constant 0 : index
    memref.store %v1000, %v1001[%v1002] : memref<1xi64>
    %v1003 = arith.constant 6 : i64
    %v1004 = memref.get_global @sloth_main_g_fixed : memref<1xi64>
    %v1005 = arith.constant 0 : index
    memref.store %v1003, %v1004[%v1005] : memref<1xi64>
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 2 : i64
    %v1002 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    memref.store %v1001, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 26984 : i64
    %v1006 = arith.constant 4 : i64
    %v1007 = call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = call @sloth_str_finish(%v1007) : (i64) -> i64
    %v1009 = memref.alloca() : memref<1xi64>
    %v1010 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1009[%v1011] : memref<1xi64>
    call @sloth_rc_release(%v1008) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1002[%v1012] : memref<1xi64>
    %v1014 = arith.constant 2 : i64
    %v1015 = arith.constant 1 : i64
    %v1016 = arith.shrsi %v1013, %v1015 : i64
    %v1017 = arith.constant 1 : i64
    %v1018 = arith.shrsi %v1014, %v1017 : i64
    %v1019 = arith.addi %v1016, %v1018 : i64
    %v1020 = arith.constant 1 : i64
    %v1021 = arith.shli %v1019, %v1020 : i64
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1002[%v1022] : memref<1xi64>
    call @sloth_rc_release(%v1023) : (i64) -> i64
    %v1024 = call @sloth_rc_retain(%v1021) : (i64) -> i64
    %v1025 = arith.constant 0 : index
    memref.store %v1024, %v1002[%v1025] : memref<1xi64>
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1002[%v1026] : memref<1xi64>
    %v1028 = call @sloth_rt_print_i64(%v1027) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1009[%v1029] : memref<1xi64>
    %v1031 = call @sloth_rt_print_str(%v1030) : (i64) -> i64
    %v1032 = arith.constant 200 : i64
    %v1033 = memref.alloca() : memref<1xi64>
    %v1034 = arith.constant 0 : index
    memref.store %v1032, %v1033[%v1034] : memref<1xi64>
    %v1035 = arith.constant 0 : index
    %v1036 = memref.load %v1033[%v1035] : memref<1xi64>
    %v1037 = call @sloth_rt_print_i64(%v1036) : (i64) -> i64
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1002[%v1038] : memref<1xi64>
    %v1040 = call @sloth_rt_print_i64(%v1039) : (i64) -> i64
    %v1041 = memref.get_global @sloth_main_g_g : memref<1xi64>
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1041[%v1042] : memref<1xi64>
    %v1044 = memref.get_global @sloth_main_g_fixed : memref<1xi64>
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1044[%v1045] : memref<1xi64>
    %v1047 = arith.constant 1 : i64
    %v1048 = arith.shrsi %v1043, %v1047 : i64
    %v1049 = arith.constant 1 : i64
    %v1050 = arith.shrsi %v1046, %v1049 : i64
    %v1051 = arith.addi %v1048, %v1050 : i64
    %v1052 = arith.constant 1 : i64
    %v1053 = arith.shli %v1051, %v1052 : i64
    %v1054 = call @sloth_rt_print_i64(%v1053) : (i64) -> i64
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1009[%v1055] : memref<1xi64>
    call @sloth_rc_release(%v1056) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

