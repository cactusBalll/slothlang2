module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 2 : i64
    %v1002 = arith.constant 4 : i64
    %v1003 = arith.constant 6 : i64
    %v1004 = arith.constant 6 : i64
    %v1005 = call @sloth_arr_new(%v1004) : (i64) -> i64
    %v1006 = arith.constant 0 : i64
    call @sloth_arr_set(%v1005, %v1006, %v1001) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 2 : i64
    call @sloth_arr_set(%v1005, %v1007, %v1002) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 4 : i64
    call @sloth_arr_set(%v1005, %v1008, %v1003) : (i64, i64, i64) -> i64
    %v1009 = memref.alloca() : memref<1xi64>
    %v1010 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1009[%v1011] : memref<1xi64>
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1009[%v1012] : memref<1xi64>
    %v1014 = call @sloth_arr_len(%v1013) : (i64) -> i64
    %v1015 = call @sloth_rt_print_i64(%v1014) : (i64) -> i64
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1009[%v1016] : memref<1xi64>
    %v1018 = arith.constant 8 : i64
    %v1019 = call @sloth_arr_push(%v1017, %v1018) : (i64, i64) -> i64
    %v1020 = arith.constant 0 : index
    memref.store %v1019, %v1009[%v1020] : memref<1xi64>
    %v1021 = arith.constant 0 : i64
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1009[%v1022] : memref<1xi64>
    %v1024 = call @sloth_arr_len(%v1023) : (i64) -> i64
    %v1025 = call @sloth_rt_print_i64(%v1024) : (i64) -> i64
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1009[%v1026] : memref<1xi64>
    %v1028 = call @sloth_arr_pop(%v1027) : (i64) -> i64
    %v1029 = call @sloth_rt_print_i64(%v1028) : (i64) -> i64
    %v1030 = arith.constant 20 : i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1009[%v1031] : memref<1xi64>
    %v1033 = arith.constant 0 : i64
    call @sloth_arr_set(%v1032, %v1033, %v1030) : (i64, i64, i64) -> i64
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1009[%v1034] : memref<1xi64>
    %v1036 = arith.constant 0 : i64
    %v1037 = call @sloth_arr_get(%v1035, %v1036) : (i64, i64) -> i64
    %v1038 = call @sloth_rt_print_i64(%v1037) : (i64) -> i64
    %v1039 = arith.constant 0 : i64
    %v1040 = memref.alloca() : memref<1xi64>
    %v1041 = arith.constant 0 : index
    memref.store %v1039, %v1040[%v1041] : memref<1xi64>
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1009[%v1042] : memref<1xi64>
    %v1044 = call @sloth_arr_len(%v1043) : (i64) -> i64
    %v1046 = arith.constant 0 : index
    %v1045 = memref.alloca() : memref<1xi64>
    %v1047 = arith.constant 0 : i64
    memref.store %v1047, %v1045[%v1046] : memref<1xi64>
    cf.br ^af_1
  ^af_1:
    %v1048 = memref.load %v1045[%v1046] : memref<1xi64>
    %v1049 = arith.cmpi slt, %v1048, %v1044 : i64
    %v1050 = arith.extui %v1049 : i1 to i64
    %v1052 = arith.constant 0 : i64
    %v1051 = arith.cmpi ne, %v1050, %v1052 : i64
    cf.cond_br %v1051, ^ab_2, ^ae_3
  ^ab_2:
    %v1053 = call @sloth_arr_get(%v1043, %v1048) : (i64, i64) -> i64
    %v1054 = memref.alloca() : memref<1xi64>
    memref.store %v1053, %v1054[%v1046] : memref<1xi64>
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1040[%v1055] : memref<1xi64>
    %v1057 = arith.constant 0 : index
    %v1058 = memref.load %v1054[%v1057] : memref<1xi64>
    %v1059 = arith.constant 1 : i64
    %v1060 = arith.shrsi %v1056, %v1059 : i64
    %v1061 = arith.constant 1 : i64
    %v1062 = arith.shrsi %v1058, %v1061 : i64
    %v1063 = arith.addi %v1060, %v1062 : i64
    %v1064 = arith.constant 1 : i64
    %v1065 = arith.shli %v1063, %v1064 : i64
    %v1066 = arith.constant 0 : index
    %v1067 = memref.load %v1040[%v1066] : memref<1xi64>
    call @sloth_rc_release(%v1067) : (i64) -> i64
    %v1068 = call @sloth_rc_retain(%v1065) : (i64) -> i64
    %v1069 = arith.constant 0 : index
    memref.store %v1068, %v1040[%v1069] : memref<1xi64>
    cf.br ^ic_4
  ^ic_4:
    %v1070 = arith.constant 2 : i64
    %v1071 = arith.addi %v1048, %v1070 : i64
    memref.store %v1071, %v1045[%v1046] : memref<1xi64>
    cf.br ^af_1
  ^ix_5:
    cf.br ^ae_3
  ^ae_3:
    %v1072 = arith.constant 0 : index
    %v1073 = memref.load %v1040[%v1072] : memref<1xi64>
    %v1074 = call @sloth_rt_print_i64(%v1073) : (i64) -> i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1009[%v1075] : memref<1xi64>
    %v1077 = memref.alloca() : memref<1xi64>
    %v1078 = call @sloth_rc_retain(%v1076) : (i64) -> i64
    %v1079 = arith.constant 0 : index
    memref.store %v1078, %v1077[%v1079] : memref<1xi64>
    %v1080 = arith.constant 0 : index
    %v1081 = memref.load %v1077[%v1080] : memref<1xi64>
    %v1082 = arith.constant 198 : i64
    %v1083 = call @sloth_arr_push(%v1081, %v1082) : (i64, i64) -> i64
    %v1084 = arith.constant 0 : index
    memref.store %v1083, %v1077[%v1084] : memref<1xi64>
    %v1085 = arith.constant 0 : i64
    %v1086 = arith.constant 0 : index
    %v1087 = memref.load %v1009[%v1086] : memref<1xi64>
    %v1088 = call @sloth_arr_len(%v1087) : (i64) -> i64
    %v1089 = call @sloth_rt_print_i64(%v1088) : (i64) -> i64
    %v1090 = arith.constant 0 : index
    %v1091 = memref.load %v1009[%v1090] : memref<1xi64>
    %v1092 = arith.constant 6 : i64
    %v1093 = call @sloth_arr_get(%v1091, %v1092) : (i64, i64) -> i64
    %v1094 = call @sloth_rt_print_i64(%v1093) : (i64) -> i64
    %v1095 = arith.constant 2 : i64
    %v1096 = arith.constant 4 : i64
    %v1097 = arith.constant 4 : i64
    %v1098 = call @sloth_arr_new(%v1097) : (i64) -> i64
    %v1099 = arith.constant 0 : i64
    call @sloth_arr_set(%v1098, %v1099, %v1095) : (i64, i64, i64) -> i64
    %v1100 = arith.constant 2 : i64
    call @sloth_arr_set(%v1098, %v1100, %v1096) : (i64, i64, i64) -> i64
    %v1101 = arith.constant 6 : i64
    %v1102 = arith.constant 8 : i64
    %v1103 = arith.constant 4 : i64
    %v1104 = call @sloth_arr_new(%v1103) : (i64) -> i64
    %v1105 = arith.constant 0 : i64
    call @sloth_arr_set(%v1104, %v1105, %v1101) : (i64, i64, i64) -> i64
    %v1106 = arith.constant 2 : i64
    call @sloth_arr_set(%v1104, %v1106, %v1102) : (i64, i64, i64) -> i64
    %v1107 = arith.constant 4 : i64
    %v1109 = arith.constant 2 : i64
    %v1108 = call @sloth_arr_new_k(%v1107, %v1109) : (i64, i64) -> i64
    %v1110 = arith.constant 0 : i64
    %v1111 = call @sloth_rc_retain(%v1098) : (i64) -> i64
    call @sloth_arr_set(%v1108, %v1110, %v1111) : (i64, i64, i64) -> i64
    %v1112 = arith.constant 2 : i64
    %v1113 = call @sloth_rc_retain(%v1104) : (i64) -> i64
    call @sloth_arr_set(%v1108, %v1112, %v1113) : (i64, i64, i64) -> i64
    %v1114 = memref.alloca() : memref<1xi64>
    %v1115 = call @sloth_rc_retain(%v1108) : (i64) -> i64
    %v1116 = arith.constant 0 : index
    memref.store %v1115, %v1114[%v1116] : memref<1xi64>
    call @sloth_rc_release(%v1098) : (i64) -> i64
    call @sloth_rc_release(%v1104) : (i64) -> i64
    call @sloth_rc_release(%v1108) : (i64) -> i64
    %v1117 = arith.constant 18 : i64
    %v1118 = arith.constant 0 : index
    %v1119 = memref.load %v1114[%v1118] : memref<1xi64>
    %v1120 = arith.constant 0 : i64
    %v1121 = call @sloth_arr_get(%v1119, %v1120) : (i64, i64) -> i64
    %v1122 = arith.constant 2 : i64
    call @sloth_arr_set(%v1121, %v1122, %v1117) : (i64, i64, i64) -> i64
    %v1123 = arith.constant 0 : index
    %v1124 = memref.load %v1114[%v1123] : memref<1xi64>
    %v1125 = arith.constant 0 : i64
    %v1126 = call @sloth_arr_get(%v1124, %v1125) : (i64, i64) -> i64
    %v1127 = arith.constant 2 : i64
    %v1128 = call @sloth_arr_get(%v1126, %v1127) : (i64, i64) -> i64
    %v1129 = call @sloth_rt_print_i64(%v1128) : (i64) -> i64
    %v1130 = arith.constant 0 : i64
    %v1131 = call @sloth_arr_new(%v1130) : (i64) -> i64
    %v1132 = memref.alloca() : memref<1xi64>
    %v1133 = call @sloth_rc_retain(%v1131) : (i64) -> i64
    %v1134 = arith.constant 0 : index
    memref.store %v1133, %v1132[%v1134] : memref<1xi64>
    call @sloth_rc_release(%v1131) : (i64) -> i64
    %v1135 = arith.constant 0 : index
    %v1136 = memref.load %v1132[%v1135] : memref<1xi64>
    %v1137 = arith.constant 2304717109306851328 : i64
    %v1138 = call @sloth_arr_push(%v1136, %v1137) : (i64, i64) -> i64
    %v1139 = arith.constant 0 : index
    memref.store %v1138, %v1132[%v1139] : memref<1xi64>
    %v1140 = arith.constant 0 : i64
    %v1141 = arith.constant 0 : index
    %v1142 = memref.load %v1132[%v1141] : memref<1xi64>
    %v1143 = arith.constant 0 : i64
    %v1144 = call @sloth_arr_get(%v1142, %v1143) : (i64, i64) -> i64
    %v1146 = arith.constant 1 : i64
    %v1147 = arith.shli %v1144, %v1146 : i64
    %v1148 = llvm.bitcast %v1147 : i64 to f64
    %v1145 = call @sloth_rt_print_f64(%v1148) : (f64) -> i64
    %v1149 = arith.constant 0 : index
    %v1150 = memref.load %v1132[%v1149] : memref<1xi64>
    call @sloth_rc_release(%v1150) : (i64) -> i64
    %v1151 = arith.constant 0 : index
    %v1152 = memref.load %v1009[%v1151] : memref<1xi64>
    call @sloth_rc_release(%v1152) : (i64) -> i64
    %v1153 = arith.constant 0 : index
    %v1154 = memref.load %v1077[%v1153] : memref<1xi64>
    call @sloth_rc_release(%v1154) : (i64) -> i64
    %v1155 = arith.constant 0 : index
    %v1156 = memref.load %v1114[%v1155] : memref<1xi64>
    call @sloth_rc_release(%v1156) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

