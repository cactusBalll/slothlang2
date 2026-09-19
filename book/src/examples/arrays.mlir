module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 1 : i64
    %v1002 = arith.constant 2 : i64
    %v1003 = arith.constant 3 : i64
    %v1004 = arith.constant 3 : i64
    %v1005 = call @sloth_arr_new(%v1004) : (i64) -> i64
    %v1006 = arith.constant 0 : i64
    call @sloth_arr_set(%v1005, %v1006, %v1001) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 1 : i64
    call @sloth_arr_set(%v1005, %v1007, %v1002) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 2 : i64
    call @sloth_arr_set(%v1005, %v1008, %v1003) : (i64, i64, i64) -> i64
    %v1009 = memref.alloca() : memref<1xi64>
    %v1010 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1009[%v1011] : memref<1xi64>
    %v1012 = memref.extract_aligned_pointer_as_index %v1009 : memref<1xi64> -> index
    %v1013 = arith.index_cast %v1012 : index to i64
    call @sloth_fiber_track(%v1013) : (i64) -> i64
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1009[%v1014] : memref<1xi64>
    %v1016 = call @sloth_arr_len(%v1015) : (i64) -> i64
    %v1017 = call @sloth_rt_print_i64(%v1016) : (i64) -> i64
    %v1018 = arith.constant 0 : index
    %v1019 = memref.load %v1009[%v1018] : memref<1xi64>
    %v1020 = arith.constant 4 : i64
    %v1021 = call @sloth_arr_push(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = arith.constant 0 : index
    memref.store %v1021, %v1009[%v1022] : memref<1xi64>
    %v1023 = arith.constant 0 : i64
    %v1024 = arith.constant 0 : index
    %v1025 = memref.load %v1009[%v1024] : memref<1xi64>
    %v1026 = call @sloth_arr_len(%v1025) : (i64) -> i64
    %v1027 = call @sloth_rt_print_i64(%v1026) : (i64) -> i64
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1009[%v1028] : memref<1xi64>
    %v1030 = call @sloth_arr_pop(%v1029) : (i64) -> i64
    %v1031 = call @sloth_rt_print_i64(%v1030) : (i64) -> i64
    %v1032 = arith.constant 10 : i64
    %v1033 = arith.constant 0 : index
    %v1034 = memref.load %v1009[%v1033] : memref<1xi64>
    %v1035 = arith.constant 0 : i64
    call @sloth_arr_set(%v1034, %v1035, %v1032) : (i64, i64, i64) -> i64
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1009[%v1036] : memref<1xi64>
    %v1038 = arith.constant 0 : i64
    %v1039 = call @sloth_arr_get(%v1037, %v1038) : (i64, i64) -> i64
    %v1040 = call @sloth_rt_print_i64(%v1039) : (i64) -> i64
    %v1041 = arith.constant 0 : i64
    %v1042 = memref.alloca() : memref<1xi64>
    %v1043 = arith.constant 0 : index
    memref.store %v1041, %v1042[%v1043] : memref<1xi64>
    %v1044 = arith.constant 0 : index
    %v1045 = memref.load %v1009[%v1044] : memref<1xi64>
    %v1046 = call @sloth_arr_len(%v1045) : (i64) -> i64
    %v1048 = arith.constant 0 : index
    %v1047 = memref.alloca() : memref<1xi64>
    %v1049 = arith.constant 0 : i64
    memref.store %v1049, %v1047[%v1048] : memref<1xi64>
    cf.br ^af_1
  ^af_1:
    %v1050 = memref.load %v1047[%v1048] : memref<1xi64>
    %v1051 = arith.cmpi slt, %v1050, %v1046 : i64
    %v1052 = arith.extui %v1051 : i1 to i64
    %v1054 = arith.constant 0 : i64
    %v1053 = arith.cmpi ne, %v1052, %v1054 : i64
    cf.cond_br %v1053, ^ab_2, ^ae_3
  ^ab_2:
    %v1055 = call @sloth_arr_get(%v1045, %v1050) : (i64, i64) -> i64
    %v1056 = memref.alloca() : memref<1xi64>
    memref.store %v1055, %v1056[%v1048] : memref<1xi64>
    %v1057 = arith.constant 0 : index
    %v1058 = memref.load %v1042[%v1057] : memref<1xi64>
    %v1059 = arith.constant 0 : index
    %v1060 = memref.load %v1056[%v1059] : memref<1xi64>
    %v1061 = arith.addi %v1058, %v1060 : i64
    %v1062 = arith.constant 0 : index
    memref.store %v1061, %v1042[%v1062] : memref<1xi64>
    cf.br ^ic_4
  ^ic_4:
    %v1063 = arith.constant 1 : i64
    %v1064 = arith.addi %v1050, %v1063 : i64
    memref.store %v1064, %v1047[%v1048] : memref<1xi64>
    cf.br ^af_1
  ^ix_5:
    cf.br ^ae_3
  ^ae_3:
    %v1065 = arith.constant 0 : index
    %v1066 = memref.load %v1042[%v1065] : memref<1xi64>
    %v1067 = call @sloth_rt_print_i64(%v1066) : (i64) -> i64
    %v1068 = arith.constant 0 : index
    %v1069 = memref.load %v1009[%v1068] : memref<1xi64>
    %v1070 = memref.alloca() : memref<1xi64>
    %v1071 = call @sloth_rc_retain(%v1069) : (i64) -> i64
    %v1072 = arith.constant 0 : index
    memref.store %v1071, %v1070[%v1072] : memref<1xi64>
    %v1073 = memref.extract_aligned_pointer_as_index %v1070 : memref<1xi64> -> index
    %v1074 = arith.index_cast %v1073 : index to i64
    call @sloth_fiber_track(%v1074) : (i64) -> i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1070[%v1075] : memref<1xi64>
    %v1077 = arith.constant 99 : i64
    %v1078 = call @sloth_arr_push(%v1076, %v1077) : (i64, i64) -> i64
    %v1079 = arith.constant 0 : index
    memref.store %v1078, %v1070[%v1079] : memref<1xi64>
    %v1080 = arith.constant 0 : i64
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1009[%v1081] : memref<1xi64>
    %v1083 = call @sloth_arr_len(%v1082) : (i64) -> i64
    %v1084 = call @sloth_rt_print_i64(%v1083) : (i64) -> i64
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1009[%v1085] : memref<1xi64>
    %v1087 = arith.constant 3 : i64
    %v1088 = call @sloth_arr_get(%v1086, %v1087) : (i64, i64) -> i64
    %v1089 = call @sloth_rt_print_i64(%v1088) : (i64) -> i64
    %v1090 = arith.constant 1 : i64
    %v1091 = arith.constant 2 : i64
    %v1092 = arith.constant 2 : i64
    %v1093 = call @sloth_arr_new(%v1092) : (i64) -> i64
    %v1094 = arith.constant 0 : i64
    call @sloth_arr_set(%v1093, %v1094, %v1090) : (i64, i64, i64) -> i64
    %v1095 = arith.constant 1 : i64
    call @sloth_arr_set(%v1093, %v1095, %v1091) : (i64, i64, i64) -> i64
    %v1096 = arith.constant 3 : i64
    %v1097 = arith.constant 4 : i64
    %v1098 = arith.constant 2 : i64
    %v1099 = call @sloth_arr_new(%v1098) : (i64) -> i64
    %v1100 = arith.constant 0 : i64
    call @sloth_arr_set(%v1099, %v1100, %v1096) : (i64, i64, i64) -> i64
    %v1101 = arith.constant 1 : i64
    call @sloth_arr_set(%v1099, %v1101, %v1097) : (i64, i64, i64) -> i64
    %v1102 = arith.constant 2 : i64
    %v1104 = arith.constant 1 : i64
    %v1103 = call @sloth_arr_new_k(%v1102, %v1104) : (i64, i64) -> i64
    %v1105 = arith.constant 0 : i64
    %v1106 = call @sloth_rc_retain(%v1093) : (i64) -> i64
    call @sloth_arr_set(%v1103, %v1105, %v1106) : (i64, i64, i64) -> i64
    %v1107 = arith.constant 1 : i64
    %v1108 = call @sloth_rc_retain(%v1099) : (i64) -> i64
    call @sloth_arr_set(%v1103, %v1107, %v1108) : (i64, i64, i64) -> i64
    %v1109 = memref.alloca() : memref<1xi64>
    %v1110 = call @sloth_rc_retain(%v1103) : (i64) -> i64
    %v1111 = arith.constant 0 : index
    memref.store %v1110, %v1109[%v1111] : memref<1xi64>
    %v1112 = memref.extract_aligned_pointer_as_index %v1109 : memref<1xi64> -> index
    %v1113 = arith.index_cast %v1112 : index to i64
    call @sloth_fiber_track(%v1113) : (i64) -> i64
    call @sloth_rc_release(%v1093) : (i64) -> i64
    call @sloth_rc_release(%v1099) : (i64) -> i64
    call @sloth_rc_release(%v1103) : (i64) -> i64
    %v1114 = arith.constant 9 : i64
    %v1115 = arith.constant 0 : index
    %v1116 = memref.load %v1109[%v1115] : memref<1xi64>
    %v1117 = arith.constant 0 : i64
    %v1118 = call @sloth_arr_get(%v1116, %v1117) : (i64, i64) -> i64
    %v1119 = arith.constant 1 : i64
    call @sloth_arr_set(%v1118, %v1119, %v1114) : (i64, i64, i64) -> i64
    %v1120 = arith.constant 0 : index
    %v1121 = memref.load %v1109[%v1120] : memref<1xi64>
    %v1122 = arith.constant 0 : i64
    %v1123 = call @sloth_arr_get(%v1121, %v1122) : (i64, i64) -> i64
    %v1124 = arith.constant 1 : i64
    %v1125 = call @sloth_arr_get(%v1123, %v1124) : (i64, i64) -> i64
    %v1126 = call @sloth_rt_print_i64(%v1125) : (i64) -> i64
    %v1127 = arith.constant 0 : i64
    %v1128 = call @sloth_arr_new(%v1127) : (i64) -> i64
    %v1129 = memref.alloca() : memref<1xi64>
    %v1130 = call @sloth_rc_retain(%v1128) : (i64) -> i64
    %v1131 = arith.constant 0 : index
    memref.store %v1130, %v1129[%v1131] : memref<1xi64>
    %v1132 = memref.extract_aligned_pointer_as_index %v1129 : memref<1xi64> -> index
    %v1133 = arith.index_cast %v1132 : index to i64
    call @sloth_fiber_track(%v1133) : (i64) -> i64
    call @sloth_rc_release(%v1128) : (i64) -> i64
    %v1134 = arith.constant 0 : index
    %v1135 = memref.load %v1129[%v1134] : memref<1xi64>
    %v1136 = arith.constant 4609434218613702656 : i64
    %v1137 = call @sloth_arr_push(%v1135, %v1136) : (i64, i64) -> i64
    %v1138 = arith.constant 0 : index
    memref.store %v1137, %v1129[%v1138] : memref<1xi64>
    %v1139 = arith.constant 0 : i64
    %v1140 = arith.constant 0 : index
    %v1141 = memref.load %v1129[%v1140] : memref<1xi64>
    %v1142 = arith.constant 0 : i64
    %v1143 = call @sloth_arr_get(%v1141, %v1142) : (i64, i64) -> i64
    %v1145 = llvm.bitcast %v1143 : i64 to f64
    %v1144 = call @sloth_rt_print_f64(%v1145) : (f64) -> i64
    %v1146 = arith.constant 0 : index
    %v1147 = memref.load %v1070[%v1146] : memref<1xi64>
    call @sloth_rc_release(%v1147) : (i64) -> i64
    %v1148 = memref.extract_aligned_pointer_as_index %v1070 : memref<1xi64> -> index
    %v1149 = arith.index_cast %v1148 : index to i64
    call @sloth_fiber_untrack(%v1149) : (i64) -> i64
    %v1150 = arith.constant 0 : index
    %v1151 = memref.load %v1129[%v1150] : memref<1xi64>
    call @sloth_rc_release(%v1151) : (i64) -> i64
    %v1152 = memref.extract_aligned_pointer_as_index %v1129 : memref<1xi64> -> index
    %v1153 = arith.index_cast %v1152 : index to i64
    call @sloth_fiber_untrack(%v1153) : (i64) -> i64
    %v1154 = arith.constant 0 : index
    %v1155 = memref.load %v1109[%v1154] : memref<1xi64>
    call @sloth_rc_release(%v1155) : (i64) -> i64
    %v1156 = memref.extract_aligned_pointer_as_index %v1109 : memref<1xi64> -> index
    %v1157 = arith.index_cast %v1156 : index to i64
    call @sloth_fiber_untrack(%v1157) : (i64) -> i64
    %v1158 = arith.constant 0 : index
    %v1159 = memref.load %v1009[%v1158] : memref<1xi64>
    call @sloth_rc_release(%v1159) : (i64) -> i64
    %v1160 = memref.extract_aligned_pointer_as_index %v1009 : memref<1xi64> -> index
    %v1161 = arith.index_cast %v1160 : index to i64
    call @sloth_fiber_untrack(%v1161) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

