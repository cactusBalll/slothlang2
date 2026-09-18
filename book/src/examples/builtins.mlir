module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 84 : i64
    %v1002 = call @sloth_rt_print_i64(%v1001) : (i64) -> i64
    %v1003 = arith.constant 2306405959167115264 : i64
    %v1005 = arith.constant 1 : i64
    %v1006 = arith.shli %v1003, %v1005 : i64
    %v1007 = llvm.bitcast %v1006 : i64 to f64
    %v1004 = call @sloth_rt_print_f64(%v1007) : (f64) -> i64
    %v1008 = arith.constant 2 : i64
    %v1009 = call @sloth_rt_print_bool(%v1008) : (i64) -> i64
    %v1010 = arith.constant 0 : i64
    %v1011 = arith.constant 115 : i64
    %v1012 = arith.constant 2 : i64
    %v1013 = call @sloth_str_push(%v1010, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1014 = call @sloth_str_finish(%v1013) : (i64) -> i64
    %v1015 = call @sloth_rt_print_str(%v1014) : (i64) -> i64
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1016 = arith.constant 0 : i64
    %v1017 = arith.constant 6513249 : i64
    %v1018 = arith.constant 6 : i64
    %v1019 = call @sloth_str_push(%v1016, %v1017, %v1018) : (i64, i64, i64) -> i64
    %v1020 = call @sloth_str_finish(%v1019) : (i64) -> i64
    %v1021 = call @sloth_str_len(%v1020) : (i64) -> i64
    %v1022 = call @sloth_rt_print_i64(%v1021) : (i64) -> i64
    call @sloth_rc_release(%v1020) : (i64) -> i64
    %v1023 = arith.constant 2 : i64
    %v1024 = arith.constant 4 : i64
    %v1025 = arith.constant 6 : i64
    %v1026 = arith.constant 6 : i64
    %v1027 = call @sloth_arr_new(%v1026) : (i64) -> i64
    %v1028 = arith.constant 0 : i64
    call @sloth_arr_set(%v1027, %v1028, %v1023) : (i64, i64, i64) -> i64
    %v1029 = arith.constant 2 : i64
    call @sloth_arr_set(%v1027, %v1029, %v1024) : (i64, i64, i64) -> i64
    %v1030 = arith.constant 4 : i64
    call @sloth_arr_set(%v1027, %v1030, %v1025) : (i64, i64, i64) -> i64
    %v1031 = call @sloth_arr_len(%v1027) : (i64) -> i64
    %v1032 = call @sloth_rt_print_i64(%v1031) : (i64) -> i64
    call @sloth_rc_release(%v1027) : (i64) -> i64
    %v1033 = arith.constant 2 : i64
    %v1034 = arith.constant 2 : i64
    %v1035 = arith.constant 0 : i64
    %v1036 = call @sloth_map_new(%v1035) : (i64) -> i64
    call @sloth_map_set(%v1036, %v1033, %v1034) : (i64, i64, i64) -> i64
    %v1037 = call @sloth_map_len(%v1036) : (i64) -> i64
    %v1038 = call @sloth_rt_print_i64(%v1037) : (i64) -> i64
    call @sloth_rc_release(%v1036) : (i64) -> i64
    %v1039 = arith.constant 2307982219036694937 : i64
    %v1041 = arith.constant 1 : i64
    %v1042 = arith.shli %v1039, %v1041 : i64
    %v1043 = llvm.bitcast %v1042 : i64 to f64
    %v1044 = arith.fptosi %v1043 : f64 to i64
    %v1045 = arith.constant 1 : i64
    %v1046 = arith.shli %v1044, %v1045 : i64
    %v1047 = call @sloth_rt_print_i64(%v1046) : (i64) -> i64
    %v1048 = arith.constant 6 : i64
    %v1050 = arith.constant 1 : i64
    %v1051 = arith.shrsi %v1048, %v1050 : i64
    %v1052 = arith.sitofp %v1051 : i64 to f64
    %v1053 = llvm.bitcast %v1052 : f64 to i64
    %v1054 = arith.constant -3 : i64
    %v1055 = arith.andi %v1053, %v1054 : i64
    %v1056 = arith.constant 1 : i64
    %v1057 = arith.shrsi %v1055, %v1056 : i64
    %v1059 = arith.constant 1 : i64
    %v1060 = arith.shli %v1057, %v1059 : i64
    %v1061 = llvm.bitcast %v1060 : i64 to f64
    %v1058 = call @sloth_rt_print_f64(%v1061) : (f64) -> i64
    %v1062 = arith.constant 0 : i64
    %v1063 = arith.constant 107 : i64
    %v1064 = arith.constant 2 : i64
    %v1065 = call @sloth_str_push(%v1062, %v1063, %v1064) : (i64, i64, i64) -> i64
    %v1066 = call @sloth_str_finish(%v1065) : (i64) -> i64
    %v1067 = arith.constant 2 : i64
    %v1068 = arith.constant 2 : i64
    %v1069 = call @sloth_map_new(%v1068) : (i64) -> i64
    %v1070 = call @sloth_rc_retain(%v1066) : (i64) -> i64
    call @sloth_map_str_set(%v1069, %v1066, %v1067) : (i64, i64, i64) -> i64
    %v1071 = memref.alloca() : memref<1xi64>
    %v1072 = call @sloth_rc_retain(%v1069) : (i64) -> i64
    %v1073 = arith.constant 0 : index
    memref.store %v1072, %v1071[%v1073] : memref<1xi64>
    call @sloth_rc_release(%v1066) : (i64) -> i64
    call @sloth_rc_release(%v1069) : (i64) -> i64
    %v1074 = arith.constant 0 : index
    %v1075 = memref.load %v1071[%v1074] : memref<1xi64>
    %v1076 = call @sloth_map_keys(%v1075) : (i64) -> i64
    %v1077 = call @sloth_arr_len(%v1076) : (i64) -> i64
    %v1078 = call @sloth_rt_print_i64(%v1077) : (i64) -> i64
    call @sloth_rc_release(%v1076) : (i64) -> i64
    %v1079 = arith.constant 0 : index
    %v1080 = memref.load %v1071[%v1079] : memref<1xi64>
    %v1081 = call @sloth_map_values(%v1080) : (i64) -> i64
    %v1082 = call @sloth_arr_len(%v1081) : (i64) -> i64
    %v1083 = call @sloth_rt_print_i64(%v1082) : (i64) -> i64
    call @sloth_rc_release(%v1081) : (i64) -> i64
    %v1084 = call @sloth_rc_live() : () -> i64
    %v1085 = arith.constant 0 : i64
    %v1087 = arith.constant 0 : i64
    %v1086 = arith.cmpi sge, %v1084, %v1085 : i64
    %v1088 = arith.extui %v1086 : i1 to i64
    %v1089 = arith.constant 1 : i64
    %v1090 = arith.shli %v1088, %v1089 : i64
    %v1091 = call @sloth_rt_print_bool(%v1090) : (i64) -> i64
    %v1092 = arith.constant 0 : index
    %v1093 = memref.load %v1071[%v1092] : memref<1xi64>
    call @sloth_rc_release(%v1093) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

