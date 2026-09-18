module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 84 : i64
    %v1002 = call @sloth_rt_print_i64(%v1001) : (i64) -> i64
    %v1003 = arith.constant 2307531859073957888 : i64
    %v1005 = arith.constant 1 : i64
    %v1006 = arith.shli %v1003, %v1005 : i64
    %v1007 = llvm.bitcast %v1006 : i64 to f64
    %v1004 = call @sloth_rt_print_f64(%v1007) : (f64) -> i64
    %v1008 = arith.constant 2 : i64
    %v1009 = call @sloth_rt_print_bool(%v1008) : (i64) -> i64
    %v1010 = arith.constant 0 : i64
    %v1011 = arith.constant 1954047348 : i64
    %v1012 = arith.constant 8 : i64
    %v1013 = call @sloth_str_push(%v1010, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1014 = call @sloth_str_finish(%v1013) : (i64) -> i64
    %v1015 = call @sloth_rt_print_str(%v1014) : (i64) -> i64
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1016 = arith.constant 2326003654420594688 : i64
    %v1018 = arith.constant 1 : i64
    %v1019 = arith.shli %v1016, %v1018 : i64
    %v1020 = llvm.bitcast %v1019 : i64 to f64
    %v1017 = call @sloth_rt_print_f64(%v1020) : (f64) -> i64
    %v1021 = arith.constant 2299087609772638208 : i64
    %v1023 = arith.constant 1 : i64
    %v1024 = arith.shli %v1021, %v1023 : i64
    %v1025 = llvm.bitcast %v1024 : i64 to f64
    %v1022 = call @sloth_rt_print_f64(%v1025) : (f64) -> i64
    %v1026 = arith.constant 0 : i64
    %v1027 = arith.constant 7310016642684182900 : i64
    %v1028 = arith.constant 16 : i64
    %v1029 = call @sloth_str_push(%v1026, %v1027, %v1028) : (i64, i64, i64) -> i64
    %v1030 = call @sloth_str_finish(%v1029) : (i64) -> i64
    %v1031 = call @sloth_rt_print_str(%v1030) : (i64) -> i64
    call @sloth_rc_release(%v1030) : (i64) -> i64
    %v1032 = arith.constant 0 : i64
    %v1033 = arith.constant 2315448778539103601 : i64
    %v1034 = arith.constant 16 : i64
    %v1035 = call @sloth_str_push(%v1032, %v1033, %v1034) : (i64, i64, i64) -> i64
    %v1036 = arith.constant 111481940307561 : i64
    %v1037 = arith.constant 12 : i64
    %v1038 = call @sloth_str_push(%v1035, %v1036, %v1037) : (i64, i64, i64) -> i64
    %v1039 = call @sloth_str_finish(%v1038) : (i64) -> i64
    %v1040 = call @sloth_rt_print_str(%v1039) : (i64) -> i64
    call @sloth_rc_release(%v1039) : (i64) -> i64
    %v1041 = arith.constant 0 : i64
    %v1042 = arith.constant 2323048382453194801 : i64
    %v1043 = arith.constant 16 : i64
    %v1044 = call @sloth_str_push(%v1041, %v1042, %v1043) : (i64, i64, i64) -> i64
    %v1045 = arith.constant 2 : i64
    %v1046 = arith.constant 4 : i64
    %v1047 = arith.constant 1 : i64
    %v1048 = arith.shrsi %v1045, %v1047 : i64
    %v1049 = arith.constant 1 : i64
    %v1050 = arith.shrsi %v1046, %v1049 : i64
    %v1051 = arith.addi %v1048, %v1050 : i64
    %v1052 = arith.constant 1 : i64
    %v1053 = arith.shli %v1051, %v1052 : i64
    %v1054 = call @sloth_str_push_i(%v1044, %v1053) : (i64, i64) -> i64
    %v1055 = call @sloth_str_finish(%v1054) : (i64) -> i64
    %v1056 = call @sloth_rt_print_str(%v1055) : (i64) -> i64
    call @sloth_rc_release(%v1055) : (i64) -> i64
    %v1057 = arith.constant 0 : i64
    %v1058 = arith.constant 448630058099 : i64
    %v1059 = arith.constant 10 : i64
    %v1060 = call @sloth_str_push(%v1057, %v1058, %v1059) : (i64, i64, i64) -> i64
    %v1061 = call @sloth_str_finish(%v1060) : (i64) -> i64
    %v1062 = memref.alloca() : memref<1xi64>
    %v1063 = call @sloth_rc_retain(%v1061) : (i64) -> i64
    %v1064 = arith.constant 0 : index
    memref.store %v1063, %v1062[%v1064] : memref<1xi64>
    call @sloth_rc_release(%v1061) : (i64) -> i64
    %v1065 = arith.constant 0 : i64
    %v1066 = arith.constant 2124136 : i64
    %v1067 = arith.constant 6 : i64
    %v1068 = call @sloth_str_push(%v1065, %v1066, %v1067) : (i64, i64, i64) -> i64
    %v1069 = arith.constant 0 : index
    %v1070 = memref.load %v1062[%v1069] : memref<1xi64>
    %v1071 = call @sloth_str_pushp(%v1068, %v1070) : (i64, i64) -> i64
    %v1072 = arith.constant 67544357281836 : i64
    %v1073 = arith.constant 12 : i64
    %v1074 = call @sloth_str_push(%v1071, %v1072, %v1073) : (i64, i64, i64) -> i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1062[%v1075] : memref<1xi64>
    %v1077 = call @sloth_str_len(%v1076) : (i64) -> i64
    %v1078 = call @sloth_str_push_i(%v1074, %v1077) : (i64, i64) -> i64
    %v1079 = call @sloth_str_finish(%v1078) : (i64) -> i64
    %v1080 = call @sloth_rt_print_str(%v1079) : (i64) -> i64
    call @sloth_rc_release(%v1079) : (i64) -> i64
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1062[%v1081] : memref<1xi64>
    call @sloth_rc_release(%v1082) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

