module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 97 : i64
    %v1003 = arith.constant 2 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = arith.constant 2 : i64
    %v1007 = arith.constant 0 : i64
    %v1008 = arith.constant 98 : i64
    %v1009 = arith.constant 2 : i64
    %v1010 = call @sloth_str_push(%v1007, %v1008, %v1009) : (i64, i64, i64) -> i64
    %v1011 = call @sloth_str_finish(%v1010) : (i64) -> i64
    %v1012 = arith.constant 4 : i64
    %v1013 = arith.constant 2 : i64
    %v1014 = call @sloth_map_new(%v1013) : (i64) -> i64
    %v1015 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    call @sloth_map_str_set(%v1014, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1016 = call @sloth_rc_retain(%v1011) : (i64) -> i64
    call @sloth_map_str_set(%v1014, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1017 = memref.alloca() : memref<1xi64>
    %v1018 = call @sloth_rc_retain(%v1014) : (i64) -> i64
    %v1019 = arith.constant 0 : index
    memref.store %v1018, %v1017[%v1019] : memref<1xi64>
    call @sloth_rc_release(%v1005) : (i64) -> i64
    call @sloth_rc_release(%v1011) : (i64) -> i64
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1017[%v1020] : memref<1xi64>
    %v1022 = arith.constant 0 : i64
    %v1023 = arith.constant 97 : i64
    %v1024 = arith.constant 2 : i64
    %v1025 = call @sloth_str_push(%v1022, %v1023, %v1024) : (i64, i64, i64) -> i64
    %v1026 = call @sloth_str_finish(%v1025) : (i64) -> i64
    %v1027 = call @sloth_map_str_get(%v1021, %v1026) : (i64, i64) -> i64
    %v1028 = call @sloth_rt_print_i64(%v1027) : (i64) -> i64
    call @sloth_rc_release(%v1026) : (i64) -> i64
    %v1029 = arith.constant 6 : i64
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1017[%v1030] : memref<1xi64>
    %v1032 = arith.constant 0 : i64
    %v1033 = arith.constant 99 : i64
    %v1034 = arith.constant 2 : i64
    %v1035 = call @sloth_str_push(%v1032, %v1033, %v1034) : (i64, i64, i64) -> i64
    %v1036 = call @sloth_str_finish(%v1035) : (i64) -> i64
    %v1037 = call @sloth_rc_retain(%v1036) : (i64) -> i64
    call @sloth_map_str_set(%v1031, %v1036, %v1029) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1036) : (i64) -> i64
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1017[%v1038] : memref<1xi64>
    %v1040 = call @sloth_map_len(%v1039) : (i64) -> i64
    %v1041 = call @sloth_rt_print_i64(%v1040) : (i64) -> i64
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1017[%v1042] : memref<1xi64>
    %v1044 = call @sloth_map_len(%v1043) : (i64) -> i64
    %v1045 = call @sloth_rt_print_i64(%v1044) : (i64) -> i64
    %v1046 = arith.constant 0 : index
    %v1047 = memref.load %v1017[%v1046] : memref<1xi64>
    %v1048 = call @sloth_map_keys(%v1047) : (i64) -> i64
    %v1049 = call @sloth_arr_len(%v1048) : (i64) -> i64
    %v1051 = arith.constant 0 : index
    %v1050 = memref.alloca() : memref<1xi64>
    %v1052 = arith.constant 0 : i64
    memref.store %v1052, %v1050[%v1051] : memref<1xi64>
    cf.br ^me_1
  ^me_1:
    %v1053 = memref.load %v1050[%v1051] : memref<1xi64>
    %v1054 = arith.cmpi slt, %v1053, %v1049 : i64
    %v1055 = arith.extui %v1054 : i1 to i64
    %v1057 = arith.constant 0 : i64
    %v1056 = arith.cmpi ne, %v1055, %v1057 : i64
    cf.cond_br %v1056, ^mb_2, ^md_3
  ^mb_2:
    %v1058 = call @sloth_arr_get(%v1048, %v1053) : (i64, i64) -> i64
    %v1059 = call @sloth_map_str_get(%v1047, %v1058) : (i64, i64) -> i64
    %v1060 = arith.constant 0 : i64
    %v1061 = arith.constant 4 : i64
    %v1062 = call @sloth_cls_info(%v1060, %v1061) : (i64, i64) -> i64
    %v1063 = arith.constant 1 : i64
    %v1064 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1062, %v1063, %v1064) : (i64, i64, i64) -> i64
    %v1065 = arith.constant 4 : i64
    %v1066 = call @sloth_obj_new(%v1062, %v1065) : (i64, i64) -> i64
    %v1067 = arith.constant 0 : i64
    %v1068 = call @sloth_obj_field(%v1066, %v1067) : (i64, i64) -> i64
    call @sloth_rc_release(%v1068) : (i64) -> i64
    %v1069 = call @sloth_rc_retain(%v1058) : (i64) -> i64
    call @sloth_obj_set_field(%v1066, %v1067, %v1069) : (i64, i64, i64) -> i64
    %v1070 = arith.constant 2 : i64
    %v1071 = call @sloth_obj_field(%v1066, %v1070) : (i64, i64) -> i64
    call @sloth_rc_release(%v1071) : (i64) -> i64
    %v1072 = call @sloth_rc_retain(%v1059) : (i64) -> i64
    call @sloth_obj_set_field(%v1066, %v1070, %v1072) : (i64, i64, i64) -> i64
    %v1073 = memref.alloca() : memref<1xi64>
    memref.store %v1066, %v1073[%v1051] : memref<1xi64>
    %v1074 = arith.constant 0 : i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1073[%v1075] : memref<1xi64>
    %v1077 = arith.constant 0 : i64
    %v1078 = call @sloth_obj_field(%v1076, %v1077) : (i64, i64) -> i64
    %v1079 = call @sloth_str_pushp(%v1074, %v1078) : (i64, i64) -> i64
    %v1080 = arith.constant 61 : i64
    %v1081 = arith.constant 2 : i64
    %v1082 = call @sloth_str_push(%v1079, %v1080, %v1081) : (i64, i64, i64) -> i64
    %v1083 = arith.constant 0 : index
    %v1084 = memref.load %v1073[%v1083] : memref<1xi64>
    %v1085 = arith.constant 2 : i64
    %v1086 = call @sloth_obj_field(%v1084, %v1085) : (i64, i64) -> i64
    %v1087 = call @sloth_str_push_i(%v1082, %v1086) : (i64, i64) -> i64
    %v1088 = call @sloth_str_finish(%v1087) : (i64) -> i64
    %v1089 = call @sloth_rt_print_str(%v1088) : (i64) -> i64
    call @sloth_rc_release(%v1088) : (i64) -> i64
    cf.br ^mc_4
  ^mc_4:
    call @sloth_rc_release(%v1066) : (i64) -> i64
    %v1090 = arith.constant 2 : i64
    %v1091 = arith.addi %v1053, %v1090 : i64
    memref.store %v1091, %v1050[%v1051] : memref<1xi64>
    cf.br ^me_1
  ^mx_5:
    %v1092 = memref.load %v1073[%v1051] : memref<1xi64>
    call @sloth_rc_release(%v1092) : (i64) -> i64
    cf.br ^md_3
  ^md_3:
    call @sloth_rc_release(%v1048) : (i64) -> i64
    %v1093 = arith.constant 0 : index
    %v1094 = memref.load %v1017[%v1093] : memref<1xi64>
    %v1095 = call @sloth_map_keys(%v1094) : (i64) -> i64
    %v1096 = memref.alloca() : memref<1xi64>
    %v1097 = call @sloth_rc_retain(%v1095) : (i64) -> i64
    %v1098 = arith.constant 0 : index
    memref.store %v1097, %v1096[%v1098] : memref<1xi64>
    call @sloth_rc_release(%v1095) : (i64) -> i64
    %v1099 = arith.constant 0 : index
    %v1100 = memref.load %v1096[%v1099] : memref<1xi64>
    %v1101 = call @sloth_arr_len(%v1100) : (i64) -> i64
    %v1102 = call @sloth_rt_print_i64(%v1101) : (i64) -> i64
    %v1103 = arith.constant 0 : index
    %v1104 = memref.load %v1017[%v1103] : memref<1xi64>
    %v1105 = call @sloth_map_values(%v1104) : (i64) -> i64
    %v1106 = memref.alloca() : memref<1xi64>
    %v1107 = call @sloth_rc_retain(%v1105) : (i64) -> i64
    %v1108 = arith.constant 0 : index
    memref.store %v1107, %v1106[%v1108] : memref<1xi64>
    call @sloth_rc_release(%v1105) : (i64) -> i64
    %v1109 = arith.constant 0 : index
    %v1110 = memref.load %v1106[%v1109] : memref<1xi64>
    %v1111 = call @sloth_arr_len(%v1110) : (i64) -> i64
    %v1112 = call @sloth_rt_print_i64(%v1111) : (i64) -> i64
    %v1113 = arith.constant 2 : i64
    %v1114 = call @sloth_map_new(%v1113) : (i64) -> i64
    %v1115 = memref.alloca() : memref<1xi64>
    %v1116 = call @sloth_rc_retain(%v1114) : (i64) -> i64
    %v1117 = arith.constant 0 : index
    memref.store %v1116, %v1115[%v1117] : memref<1xi64>
    call @sloth_rc_release(%v1114) : (i64) -> i64
    %v1118 = arith.constant 2 : i64
    %v1119 = arith.constant 0 : index
    %v1120 = memref.load %v1115[%v1119] : memref<1xi64>
    %v1121 = arith.constant 0 : i64
    %v1122 = arith.constant 120 : i64
    %v1123 = arith.constant 2 : i64
    %v1124 = call @sloth_str_push(%v1121, %v1122, %v1123) : (i64, i64, i64) -> i64
    %v1125 = call @sloth_str_finish(%v1124) : (i64) -> i64
    %v1126 = call @sloth_rc_retain(%v1125) : (i64) -> i64
    call @sloth_map_str_set(%v1120, %v1125, %v1118) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1125) : (i64) -> i64
    %v1127 = arith.constant 0 : index
    %v1128 = memref.load %v1115[%v1127] : memref<1xi64>
    %v1129 = arith.constant 0 : i64
    %v1130 = arith.constant 120 : i64
    %v1131 = arith.constant 2 : i64
    %v1132 = call @sloth_str_push(%v1129, %v1130, %v1131) : (i64, i64, i64) -> i64
    %v1133 = call @sloth_str_finish(%v1132) : (i64) -> i64
    %v1134 = call @sloth_map_str_get(%v1128, %v1133) : (i64, i64) -> i64
    %v1135 = call @sloth_rt_print_i64(%v1134) : (i64) -> i64
    call @sloth_rc_release(%v1133) : (i64) -> i64
    %v1136 = arith.constant 2 : i64
    %v1137 = call @sloth_map_new(%v1136) : (i64) -> i64
    %v1138 = memref.alloca() : memref<1xi64>
    %v1139 = call @sloth_rc_retain(%v1137) : (i64) -> i64
    %v1140 = arith.constant 0 : index
    memref.store %v1139, %v1138[%v1140] : memref<1xi64>
    call @sloth_rc_release(%v1137) : (i64) -> i64
    %v1141 = arith.constant 2 : i64
    %v1142 = call @sloth_map_new(%v1141) : (i64) -> i64
    %v1143 = arith.constant 0 : index
    %v1144 = memref.load %v1138[%v1143] : memref<1xi64>
    %v1145 = arith.constant 0 : i64
    %v1146 = arith.constant 97 : i64
    %v1147 = arith.constant 2 : i64
    %v1148 = call @sloth_str_push(%v1145, %v1146, %v1147) : (i64, i64, i64) -> i64
    %v1149 = call @sloth_str_finish(%v1148) : (i64) -> i64
    %v1150 = call @sloth_rc_retain(%v1149) : (i64) -> i64
    %v1151 = call @sloth_rc_retain(%v1142) : (i64) -> i64
    call @sloth_map_str_set(%v1144, %v1149, %v1151) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1142) : (i64) -> i64
    call @sloth_rc_release(%v1149) : (i64) -> i64
    %v1152 = arith.constant 14 : i64
    %v1153 = arith.constant 0 : index
    %v1154 = memref.load %v1138[%v1153] : memref<1xi64>
    %v1155 = arith.constant 0 : i64
    %v1156 = arith.constant 97 : i64
    %v1157 = arith.constant 2 : i64
    %v1158 = call @sloth_str_push(%v1155, %v1156, %v1157) : (i64, i64, i64) -> i64
    %v1159 = call @sloth_str_finish(%v1158) : (i64) -> i64
    %v1160 = call @sloth_map_str_get(%v1154, %v1159) : (i64, i64) -> i64
    %v1161 = arith.constant 0 : i64
    %v1162 = arith.constant 98 : i64
    %v1163 = arith.constant 2 : i64
    %v1164 = call @sloth_str_push(%v1161, %v1162, %v1163) : (i64, i64, i64) -> i64
    %v1165 = call @sloth_str_finish(%v1164) : (i64) -> i64
    %v1166 = call @sloth_rc_retain(%v1165) : (i64) -> i64
    call @sloth_map_str_set(%v1160, %v1165, %v1152) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1159) : (i64) -> i64
    call @sloth_rc_release(%v1165) : (i64) -> i64
    %v1167 = arith.constant 0 : index
    %v1168 = memref.load %v1138[%v1167] : memref<1xi64>
    %v1169 = arith.constant 0 : i64
    %v1170 = arith.constant 97 : i64
    %v1171 = arith.constant 2 : i64
    %v1172 = call @sloth_str_push(%v1169, %v1170, %v1171) : (i64, i64, i64) -> i64
    %v1173 = call @sloth_str_finish(%v1172) : (i64) -> i64
    %v1174 = call @sloth_map_str_get(%v1168, %v1173) : (i64, i64) -> i64
    %v1175 = arith.constant 0 : i64
    %v1176 = arith.constant 98 : i64
    %v1177 = arith.constant 2 : i64
    %v1178 = call @sloth_str_push(%v1175, %v1176, %v1177) : (i64, i64, i64) -> i64
    %v1179 = call @sloth_str_finish(%v1178) : (i64) -> i64
    %v1180 = call @sloth_map_str_get(%v1174, %v1179) : (i64, i64) -> i64
    %v1181 = call @sloth_rt_print_i64(%v1180) : (i64) -> i64
    call @sloth_rc_release(%v1173) : (i64) -> i64
    call @sloth_rc_release(%v1179) : (i64) -> i64
    %v1182 = arith.constant 0 : index
    %v1183 = memref.load %v1096[%v1182] : memref<1xi64>
    call @sloth_rc_release(%v1183) : (i64) -> i64
    %v1184 = arith.constant 0 : index
    %v1185 = memref.load %v1017[%v1184] : memref<1xi64>
    call @sloth_rc_release(%v1185) : (i64) -> i64
    %v1186 = arith.constant 0 : index
    %v1187 = memref.load %v1115[%v1186] : memref<1xi64>
    call @sloth_rc_release(%v1187) : (i64) -> i64
    %v1188 = arith.constant 0 : index
    %v1189 = memref.load %v1138[%v1188] : memref<1xi64>
    call @sloth_rc_release(%v1189) : (i64) -> i64
    %v1190 = arith.constant 0 : index
    %v1191 = memref.load %v1106[%v1190] : memref<1xi64>
    call @sloth_rc_release(%v1191) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

