module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 478560413032 : i64
    %v1003 = arith.constant 10 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1006[%v1008] : memref<1xi64>
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1006[%v1009] : memref<1xi64>
    %v1011 = arith.constant 0 : i64
    %v1012 = arith.constant 28266736423215148 : i64
    %v1013 = arith.constant 14 : i64
    %v1014 = call @sloth_str_push(%v1011, %v1012, %v1013) : (i64, i64, i64) -> i64
    %v1015 = call @sloth_str_finish(%v1014) : (i64) -> i64
    %v1016 = call @sloth_str_concat(%v1010, %v1015) : (i64, i64) -> i64
    %v1017 = call @sloth_rt_print_str(%v1016) : (i64) -> i64
    call @sloth_rc_release(%v1015) : (i64) -> i64
    call @sloth_rc_release(%v1016) : (i64) -> i64
    %v1018 = arith.constant 0 : index
    %v1019 = memref.load %v1006[%v1018] : memref<1xi64>
    %v1020 = call @sloth_str_len(%v1019) : (i64) -> i64
    %v1021 = call @sloth_rt_print_i64(%v1020) : (i64) -> i64
    %v1022 = arith.constant 0 : i64
    %v1023 = arith.constant 122511470216040 : i64
    %v1024 = arith.constant 12 : i64
    %v1025 = call @sloth_str_push(%v1022, %v1023, %v1024) : (i64, i64, i64) -> i64
    %v1026 = call @sloth_str_finish(%v1025) : (i64) -> i64
    %v1027 = call @sloth_str_len(%v1026) : (i64) -> i64
    %v1028 = call @sloth_rt_print_i64(%v1027) : (i64) -> i64
    call @sloth_rc_release(%v1026) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1006[%v1029] : memref<1xi64>
    %v1031 = arith.constant 0 : i64
    %v1032 = arith.constant 7103848 : i64
    %v1033 = arith.constant 6 : i64
    %v1034 = call @sloth_str_push(%v1031, %v1032, %v1033) : (i64, i64, i64) -> i64
    %v1035 = call @sloth_str_finish(%v1034) : (i64) -> i64
    %v1036 = arith.constant 0 : i64
    %v1037 = arith.constant 28524 : i64
    %v1038 = arith.constant 4 : i64
    %v1039 = call @sloth_str_push(%v1036, %v1037, %v1038) : (i64, i64, i64) -> i64
    %v1040 = call @sloth_str_finish(%v1039) : (i64) -> i64
    %v1041 = call @sloth_str_concat(%v1035, %v1040) : (i64, i64) -> i64
    %v1042 = call @sloth_str_eq(%v1030, %v1041) : (i64, i64) -> i64
    %v1043 = call @sloth_rt_print_bool(%v1042) : (i64) -> i64
    call @sloth_rc_release(%v1035) : (i64) -> i64
    call @sloth_rc_release(%v1040) : (i64) -> i64
    call @sloth_rc_release(%v1041) : (i64) -> i64
    %v1044 = arith.constant 0 : i64
    %v1045 = arith.constant 97 : i64
    %v1046 = arith.constant 2 : i64
    %v1047 = call @sloth_str_push(%v1044, %v1045, %v1046) : (i64, i64, i64) -> i64
    %v1048 = call @sloth_str_finish(%v1047) : (i64) -> i64
    %v1049 = arith.constant 0 : i64
    %v1050 = arith.constant 98 : i64
    %v1051 = arith.constant 2 : i64
    %v1052 = call @sloth_str_push(%v1049, %v1050, %v1051) : (i64, i64, i64) -> i64
    %v1053 = call @sloth_str_finish(%v1052) : (i64) -> i64
    %v1054 = call @sloth_str_eq(%v1048, %v1053) : (i64, i64) -> i64
    %v1055 = arith.constant 1 : i64
    %v1056 = arith.shrsi %v1054, %v1055 : i64
    %v1057 = arith.constant 1 : i64
    %v1058 = arith.xori %v1056, %v1057 : i64
    %v1059 = arith.constant 1 : i64
    %v1060 = arith.shli %v1058, %v1059 : i64
    %v1061 = call @sloth_rt_print_bool(%v1060) : (i64) -> i64
    call @sloth_rc_release(%v1048) : (i64) -> i64
    call @sloth_rc_release(%v1053) : (i64) -> i64
    %v1062 = arith.constant 0 : i64
    %v1063 = memref.alloca() : memref<1xi64>
    %v1064 = arith.constant 0 : index
    memref.store %v1062, %v1063[%v1064] : memref<1xi64>
    %v1065 = arith.constant 0 : i64
    %v1066 = arith.constant 191009621918561 : i64
    %v1067 = arith.constant 12 : i64
    %v1068 = call @sloth_str_push(%v1065, %v1066, %v1067) : (i64, i64, i64) -> i64
    %v1069 = call @sloth_str_finish(%v1068) : (i64) -> i64
    %v1070 = call @sloth_str_clen(%v1069) : (i64) -> i64
    %v1072 = arith.constant 0 : index
    %v1071 = memref.alloca() : memref<1xi64>
    %v1073 = arith.constant 0 : i64
    memref.store %v1073, %v1071[%v1072] : memref<1xi64>
    cf.br ^af_1
  ^af_1:
    %v1074 = memref.load %v1071[%v1072] : memref<1xi64>
    %v1075 = arith.cmpi slt, %v1074, %v1070 : i64
    %v1076 = arith.extui %v1075 : i1 to i64
    %v1078 = arith.constant 0 : i64
    %v1077 = arith.cmpi ne, %v1076, %v1078 : i64
    cf.cond_br %v1077, ^ab_2, ^ae_3
  ^ab_2:
    %v1079 = call @sloth_str_char(%v1069, %v1074) : (i64, i64) -> i64
    %v1080 = memref.alloca() : memref<1xi64>
    memref.store %v1079, %v1080[%v1072] : memref<1xi64>
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1063[%v1081] : memref<1xi64>
    %v1083 = arith.constant 2 : i64
    %v1084 = arith.constant 1 : i64
    %v1085 = arith.shrsi %v1082, %v1084 : i64
    %v1086 = arith.constant 1 : i64
    %v1087 = arith.shrsi %v1083, %v1086 : i64
    %v1088 = arith.addi %v1085, %v1087 : i64
    %v1089 = arith.constant 1 : i64
    %v1090 = arith.shli %v1088, %v1089 : i64
    %v1091 = arith.constant 0 : index
    %v1092 = memref.load %v1063[%v1091] : memref<1xi64>
    call @sloth_rc_release(%v1092) : (i64) -> i64
    %v1093 = call @sloth_rc_retain(%v1090) : (i64) -> i64
    %v1094 = arith.constant 0 : index
    memref.store %v1093, %v1063[%v1094] : memref<1xi64>
    cf.br ^ic_4
  ^ic_4:
    call @sloth_rc_release(%v1079) : (i64) -> i64
    %v1095 = arith.constant 2 : i64
    %v1096 = arith.addi %v1074, %v1095 : i64
    memref.store %v1096, %v1071[%v1072] : memref<1xi64>
    cf.br ^af_1
  ^ix_5:
    %v1097 = memref.load %v1080[%v1072] : memref<1xi64>
    call @sloth_rc_release(%v1097) : (i64) -> i64
    cf.br ^ae_3
  ^ae_3:
    call @sloth_rc_release(%v1069) : (i64) -> i64
    %v1098 = arith.constant 0 : index
    %v1099 = memref.load %v1063[%v1098] : memref<1xi64>
    %v1100 = call @sloth_rt_print_i64(%v1099) : (i64) -> i64
    %v1101 = arith.constant 0 : i64
    %v1102 = arith.constant 120 : i64
    %v1103 = arith.constant 2 : i64
    %v1104 = call @sloth_str_push(%v1101, %v1102, %v1103) : (i64, i64, i64) -> i64
    %v1105 = call @sloth_str_finish(%v1104) : (i64) -> i64
    %v1106 = memref.alloca() : memref<1xi64>
    %v1107 = call @sloth_rc_retain(%v1105) : (i64) -> i64
    %v1108 = arith.constant 0 : index
    memref.store %v1107, %v1106[%v1108] : memref<1xi64>
    call @sloth_rc_release(%v1105) : (i64) -> i64
    %v1109 = arith.constant 0 : index
    %v1110 = memref.load %v1106[%v1109] : memref<1xi64>
    %v1111 = arith.constant 0 : i64
    %v1112 = arith.constant 121 : i64
    %v1113 = arith.constant 2 : i64
    %v1114 = call @sloth_str_push(%v1111, %v1112, %v1113) : (i64, i64, i64) -> i64
    %v1115 = call @sloth_str_finish(%v1114) : (i64) -> i64
    %v1116 = call @sloth_str_concat(%v1110, %v1115) : (i64, i64) -> i64
    %v1117 = arith.constant 0 : i64
    %v1118 = arith.constant 122 : i64
    %v1119 = arith.constant 2 : i64
    %v1120 = call @sloth_str_push(%v1117, %v1118, %v1119) : (i64, i64, i64) -> i64
    %v1121 = call @sloth_str_finish(%v1120) : (i64) -> i64
    %v1122 = call @sloth_str_concat(%v1116, %v1121) : (i64, i64) -> i64
    %v1123 = arith.constant 0 : index
    %v1124 = memref.load %v1106[%v1123] : memref<1xi64>
    call @sloth_rc_release(%v1124) : (i64) -> i64
    %v1125 = call @sloth_rc_retain(%v1122) : (i64) -> i64
    %v1126 = arith.constant 0 : index
    memref.store %v1125, %v1106[%v1126] : memref<1xi64>
    call @sloth_rc_release(%v1115) : (i64) -> i64
    call @sloth_rc_release(%v1116) : (i64) -> i64
    call @sloth_rc_release(%v1121) : (i64) -> i64
    call @sloth_rc_release(%v1122) : (i64) -> i64
    %v1127 = arith.constant 0 : index
    %v1128 = memref.load %v1106[%v1127] : memref<1xi64>
    %v1129 = call @sloth_rt_print_str(%v1128) : (i64) -> i64
    %v1130 = arith.constant 2304717109306851328 : i64
    %v1131 = memref.alloca() : memref<1xi64>
    %v1132 = arith.constant 0 : index
    memref.store %v1130, %v1131[%v1132] : memref<1xi64>
    %v1133 = arith.constant 0 : i64
    %v1134 = arith.constant 15726 : i64
    %v1135 = arith.constant 4 : i64
    %v1136 = call @sloth_str_push(%v1133, %v1134, %v1135) : (i64, i64, i64) -> i64
    %v1137 = arith.constant 0 : index
    %v1138 = memref.load %v1063[%v1137] : memref<1xi64>
    %v1139 = call @sloth_str_push_i(%v1136, %v1138) : (i64, i64) -> i64
    %v1140 = arith.constant 4023840 : i64
    %v1141 = arith.constant 6 : i64
    %v1142 = call @sloth_str_push(%v1139, %v1140, %v1141) : (i64, i64, i64) -> i64
    %v1143 = arith.constant 0 : index
    %v1144 = memref.load %v1131[%v1143] : memref<1xi64>
    %v1146 = arith.constant 1 : i64
    %v1147 = arith.shli %v1144, %v1146 : i64
    %v1148 = llvm.bitcast %v1147 : i64 to f64
    %v1145 = call @sloth_str_push_f(%v1142, %v1148) : (i64, f64) -> i64
    %v1149 = arith.constant 4022816 : i64
    %v1150 = arith.constant 6 : i64
    %v1151 = call @sloth_str_push(%v1145, %v1149, %v1150) : (i64, i64, i64) -> i64
    %v1152 = arith.constant 2 : i64
    %v1153 = call @sloth_str_push_b(%v1151, %v1152) : (i64, i64) -> i64
    %v1154 = call @sloth_str_finish(%v1153) : (i64) -> i64
    %v1155 = call @sloth_rt_print_str(%v1154) : (i64) -> i64
    call @sloth_rc_release(%v1154) : (i64) -> i64
    %v1156 = arith.constant 0 : index
    %v1157 = memref.load %v1106[%v1156] : memref<1xi64>
    call @sloth_rc_release(%v1157) : (i64) -> i64
    %v1158 = arith.constant 0 : index
    %v1159 = memref.load %v1006[%v1158] : memref<1xi64>
    call @sloth_rc_release(%v1159) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

