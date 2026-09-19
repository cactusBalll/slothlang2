module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 97 : i64
    %v1003 = arith.constant 1 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = arith.constant 1 : i64
    %v1007 = arith.constant 0 : i64
    %v1008 = arith.constant 98 : i64
    %v1009 = arith.constant 1 : i64
    %v1010 = call @sloth_str_push(%v1007, %v1008, %v1009) : (i64, i64, i64) -> i64
    %v1011 = call @sloth_str_finish(%v1010) : (i64) -> i64
    %v1012 = arith.constant 2 : i64
    %v1013 = arith.constant 1 : i64
    %v1014 = call @sloth_map_new(%v1013) : (i64) -> i64
    %v1015 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    call @sloth_map_str_set(%v1014, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1016 = call @sloth_rc_retain(%v1011) : (i64) -> i64
    call @sloth_map_str_set(%v1014, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1017 = memref.alloca() : memref<1xi64>
    %v1018 = call @sloth_rc_retain(%v1014) : (i64) -> i64
    %v1019 = arith.constant 0 : index
    memref.store %v1018, %v1017[%v1019] : memref<1xi64>
    %v1020 = memref.extract_aligned_pointer_as_index %v1017 : memref<1xi64> -> index
    %v1021 = arith.index_cast %v1020 : index to i64
    call @sloth_fiber_track(%v1021) : (i64) -> i64
    call @sloth_rc_release(%v1005) : (i64) -> i64
    call @sloth_rc_release(%v1011) : (i64) -> i64
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1017[%v1022] : memref<1xi64>
    %v1024 = arith.constant 0 : i64
    %v1025 = arith.constant 97 : i64
    %v1026 = arith.constant 1 : i64
    %v1027 = call @sloth_str_push(%v1024, %v1025, %v1026) : (i64, i64, i64) -> i64
    %v1028 = call @sloth_str_finish(%v1027) : (i64) -> i64
    %v1029 = call @sloth_map_str_get(%v1023, %v1028) : (i64, i64) -> i64
    %v1030 = call @sloth_rt_print_i64(%v1029) : (i64) -> i64
    call @sloth_rc_release(%v1028) : (i64) -> i64
    %v1031 = arith.constant 3 : i64
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1017[%v1032] : memref<1xi64>
    %v1034 = arith.constant 0 : i64
    %v1035 = arith.constant 99 : i64
    %v1036 = arith.constant 1 : i64
    %v1037 = call @sloth_str_push(%v1034, %v1035, %v1036) : (i64, i64, i64) -> i64
    %v1038 = call @sloth_str_finish(%v1037) : (i64) -> i64
    %v1039 = call @sloth_rc_retain(%v1038) : (i64) -> i64
    call @sloth_map_str_set(%v1033, %v1038, %v1031) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1038) : (i64) -> i64
    %v1040 = arith.constant 0 : index
    %v1041 = memref.load %v1017[%v1040] : memref<1xi64>
    %v1042 = call @sloth_map_len(%v1041) : (i64) -> i64
    %v1043 = call @sloth_rt_print_i64(%v1042) : (i64) -> i64
    %v1044 = arith.constant 0 : index
    %v1045 = memref.load %v1017[%v1044] : memref<1xi64>
    %v1046 = call @sloth_map_len(%v1045) : (i64) -> i64
    %v1047 = call @sloth_rt_print_i64(%v1046) : (i64) -> i64
    %v1048 = arith.constant 0 : index
    %v1049 = memref.load %v1017[%v1048] : memref<1xi64>
    %v1050 = call @sloth_map_keys(%v1049) : (i64) -> i64
    %v1051 = call @sloth_arr_len(%v1050) : (i64) -> i64
    %v1053 = arith.constant 0 : index
    %v1052 = memref.alloca() : memref<1xi64>
    %v1054 = arith.constant 0 : i64
    memref.store %v1054, %v1052[%v1053] : memref<1xi64>
    cf.br ^me_1
  ^me_1:
    %v1055 = memref.load %v1052[%v1053] : memref<1xi64>
    %v1056 = arith.cmpi slt, %v1055, %v1051 : i64
    %v1057 = arith.extui %v1056 : i1 to i64
    %v1059 = arith.constant 0 : i64
    %v1058 = arith.cmpi ne, %v1057, %v1059 : i64
    cf.cond_br %v1058, ^mb_2, ^md_3
  ^mb_2:
    %v1060 = call @sloth_arr_get(%v1050, %v1055) : (i64, i64) -> i64
    %v1061 = call @sloth_map_str_get(%v1049, %v1060) : (i64, i64) -> i64
    %v1062 = arith.constant 0 : i64
    %v1063 = arith.constant 2 : i64
    %v1064 = call @sloth_cls_info(%v1062, %v1063) : (i64, i64) -> i64
    %v1065 = arith.constant 1 : i64
    %v1066 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1064, %v1065, %v1066) : (i64, i64, i64) -> i64
    %v1067 = arith.constant 2 : i64
    %v1068 = call @sloth_obj_new(%v1064, %v1067) : (i64, i64) -> i64
    %v1069 = arith.constant 0 : i64
    %v1070 = call @sloth_obj_field(%v1068, %v1069) : (i64, i64) -> i64
    call @sloth_rc_release(%v1070) : (i64) -> i64
    %v1071 = call @sloth_rc_retain(%v1060) : (i64) -> i64
    call @sloth_obj_set_field(%v1068, %v1069, %v1071) : (i64, i64, i64) -> i64
    %v1072 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1068, %v1072, %v1061) : (i64, i64, i64) -> i64
    %v1073 = memref.alloca() : memref<1xi64>
    memref.store %v1068, %v1073[%v1053] : memref<1xi64>
    %v1074 = arith.constant 0 : i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1073[%v1075] : memref<1xi64>
    %v1077 = arith.constant 0 : i64
    %v1078 = call @sloth_obj_field(%v1076, %v1077) : (i64, i64) -> i64
    %v1079 = call @sloth_str_pushp(%v1074, %v1078) : (i64, i64) -> i64
    %v1080 = arith.constant 61 : i64
    %v1081 = arith.constant 1 : i64
    %v1082 = call @sloth_str_push(%v1079, %v1080, %v1081) : (i64, i64, i64) -> i64
    %v1083 = arith.constant 0 : index
    %v1084 = memref.load %v1073[%v1083] : memref<1xi64>
    %v1085 = arith.constant 1 : i64
    %v1086 = call @sloth_obj_field(%v1084, %v1085) : (i64, i64) -> i64
    %v1087 = call @sloth_str_push_i(%v1082, %v1086) : (i64, i64) -> i64
    %v1088 = call @sloth_str_finish(%v1087) : (i64) -> i64
    %v1089 = call @sloth_rt_print_str(%v1088) : (i64) -> i64
    call @sloth_rc_release(%v1088) : (i64) -> i64
    cf.br ^mc_4
  ^mc_4:
    call @sloth_rc_release(%v1068) : (i64) -> i64
    %v1090 = arith.constant 1 : i64
    %v1091 = arith.addi %v1055, %v1090 : i64
    memref.store %v1091, %v1052[%v1053] : memref<1xi64>
    cf.br ^me_1
  ^mx_5:
    %v1092 = memref.load %v1073[%v1053] : memref<1xi64>
    call @sloth_rc_release(%v1092) : (i64) -> i64
    cf.br ^md_3
  ^md_3:
    call @sloth_rc_release(%v1050) : (i64) -> i64
    %v1093 = arith.constant 0 : index
    %v1094 = memref.load %v1017[%v1093] : memref<1xi64>
    %v1095 = call @sloth_map_keys(%v1094) : (i64) -> i64
    %v1096 = memref.alloca() : memref<1xi64>
    %v1097 = call @sloth_rc_retain(%v1095) : (i64) -> i64
    %v1098 = arith.constant 0 : index
    memref.store %v1097, %v1096[%v1098] : memref<1xi64>
    %v1099 = memref.extract_aligned_pointer_as_index %v1096 : memref<1xi64> -> index
    %v1100 = arith.index_cast %v1099 : index to i64
    call @sloth_fiber_track(%v1100) : (i64) -> i64
    call @sloth_rc_release(%v1095) : (i64) -> i64
    %v1101 = arith.constant 0 : index
    %v1102 = memref.load %v1096[%v1101] : memref<1xi64>
    %v1103 = call @sloth_arr_len(%v1102) : (i64) -> i64
    %v1104 = call @sloth_rt_print_i64(%v1103) : (i64) -> i64
    %v1105 = arith.constant 0 : index
    %v1106 = memref.load %v1017[%v1105] : memref<1xi64>
    %v1107 = call @sloth_map_values(%v1106) : (i64) -> i64
    %v1108 = memref.alloca() : memref<1xi64>
    %v1109 = call @sloth_rc_retain(%v1107) : (i64) -> i64
    %v1110 = arith.constant 0 : index
    memref.store %v1109, %v1108[%v1110] : memref<1xi64>
    %v1111 = memref.extract_aligned_pointer_as_index %v1108 : memref<1xi64> -> index
    %v1112 = arith.index_cast %v1111 : index to i64
    call @sloth_fiber_track(%v1112) : (i64) -> i64
    call @sloth_rc_release(%v1107) : (i64) -> i64
    %v1113 = arith.constant 0 : index
    %v1114 = memref.load %v1108[%v1113] : memref<1xi64>
    %v1115 = call @sloth_arr_len(%v1114) : (i64) -> i64
    %v1116 = call @sloth_rt_print_i64(%v1115) : (i64) -> i64
    %v1117 = arith.constant 1 : i64
    %v1118 = call @sloth_map_new(%v1117) : (i64) -> i64
    %v1119 = memref.alloca() : memref<1xi64>
    %v1120 = call @sloth_rc_retain(%v1118) : (i64) -> i64
    %v1121 = arith.constant 0 : index
    memref.store %v1120, %v1119[%v1121] : memref<1xi64>
    %v1122 = memref.extract_aligned_pointer_as_index %v1119 : memref<1xi64> -> index
    %v1123 = arith.index_cast %v1122 : index to i64
    call @sloth_fiber_track(%v1123) : (i64) -> i64
    call @sloth_rc_release(%v1118) : (i64) -> i64
    %v1124 = arith.constant 1 : i64
    %v1125 = arith.constant 0 : index
    %v1126 = memref.load %v1119[%v1125] : memref<1xi64>
    %v1127 = arith.constant 0 : i64
    %v1128 = arith.constant 120 : i64
    %v1129 = arith.constant 1 : i64
    %v1130 = call @sloth_str_push(%v1127, %v1128, %v1129) : (i64, i64, i64) -> i64
    %v1131 = call @sloth_str_finish(%v1130) : (i64) -> i64
    %v1132 = call @sloth_rc_retain(%v1131) : (i64) -> i64
    call @sloth_map_str_set(%v1126, %v1131, %v1124) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1131) : (i64) -> i64
    %v1133 = arith.constant 0 : index
    %v1134 = memref.load %v1119[%v1133] : memref<1xi64>
    %v1135 = arith.constant 0 : i64
    %v1136 = arith.constant 120 : i64
    %v1137 = arith.constant 1 : i64
    %v1138 = call @sloth_str_push(%v1135, %v1136, %v1137) : (i64, i64, i64) -> i64
    %v1139 = call @sloth_str_finish(%v1138) : (i64) -> i64
    %v1140 = call @sloth_map_str_get(%v1134, %v1139) : (i64, i64) -> i64
    %v1141 = call @sloth_rt_print_i64(%v1140) : (i64) -> i64
    call @sloth_rc_release(%v1139) : (i64) -> i64
    %v1142 = arith.constant 5 : i64
    %v1143 = call @sloth_map_new(%v1142) : (i64) -> i64
    %v1144 = memref.alloca() : memref<1xi64>
    %v1145 = call @sloth_rc_retain(%v1143) : (i64) -> i64
    %v1146 = arith.constant 0 : index
    memref.store %v1145, %v1144[%v1146] : memref<1xi64>
    %v1147 = memref.extract_aligned_pointer_as_index %v1144 : memref<1xi64> -> index
    %v1148 = arith.index_cast %v1147 : index to i64
    call @sloth_fiber_track(%v1148) : (i64) -> i64
    call @sloth_rc_release(%v1143) : (i64) -> i64
    %v1149 = arith.constant 1 : i64
    %v1150 = call @sloth_map_new(%v1149) : (i64) -> i64
    %v1151 = arith.constant 0 : index
    %v1152 = memref.load %v1144[%v1151] : memref<1xi64>
    %v1153 = arith.constant 0 : i64
    %v1154 = arith.constant 97 : i64
    %v1155 = arith.constant 1 : i64
    %v1156 = call @sloth_str_push(%v1153, %v1154, %v1155) : (i64, i64, i64) -> i64
    %v1157 = call @sloth_str_finish(%v1156) : (i64) -> i64
    %v1158 = call @sloth_rc_retain(%v1157) : (i64) -> i64
    %v1159 = call @sloth_rc_retain(%v1150) : (i64) -> i64
    call @sloth_map_str_set(%v1152, %v1157, %v1159) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1150) : (i64) -> i64
    call @sloth_rc_release(%v1157) : (i64) -> i64
    %v1160 = arith.constant 7 : i64
    %v1161 = arith.constant 0 : index
    %v1162 = memref.load %v1144[%v1161] : memref<1xi64>
    %v1163 = arith.constant 0 : i64
    %v1164 = arith.constant 97 : i64
    %v1165 = arith.constant 1 : i64
    %v1166 = call @sloth_str_push(%v1163, %v1164, %v1165) : (i64, i64, i64) -> i64
    %v1167 = call @sloth_str_finish(%v1166) : (i64) -> i64
    %v1168 = call @sloth_map_str_get(%v1162, %v1167) : (i64, i64) -> i64
    %v1169 = arith.constant 0 : i64
    %v1170 = arith.constant 98 : i64
    %v1171 = arith.constant 1 : i64
    %v1172 = call @sloth_str_push(%v1169, %v1170, %v1171) : (i64, i64, i64) -> i64
    %v1173 = call @sloth_str_finish(%v1172) : (i64) -> i64
    %v1174 = call @sloth_rc_retain(%v1173) : (i64) -> i64
    call @sloth_map_str_set(%v1168, %v1173, %v1160) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1167) : (i64) -> i64
    call @sloth_rc_release(%v1173) : (i64) -> i64
    %v1175 = arith.constant 0 : index
    %v1176 = memref.load %v1144[%v1175] : memref<1xi64>
    %v1177 = arith.constant 0 : i64
    %v1178 = arith.constant 97 : i64
    %v1179 = arith.constant 1 : i64
    %v1180 = call @sloth_str_push(%v1177, %v1178, %v1179) : (i64, i64, i64) -> i64
    %v1181 = call @sloth_str_finish(%v1180) : (i64) -> i64
    %v1182 = call @sloth_map_str_get(%v1176, %v1181) : (i64, i64) -> i64
    %v1183 = arith.constant 0 : i64
    %v1184 = arith.constant 98 : i64
    %v1185 = arith.constant 1 : i64
    %v1186 = call @sloth_str_push(%v1183, %v1184, %v1185) : (i64, i64, i64) -> i64
    %v1187 = call @sloth_str_finish(%v1186) : (i64) -> i64
    %v1188 = call @sloth_map_str_get(%v1182, %v1187) : (i64, i64) -> i64
    %v1189 = call @sloth_rt_print_i64(%v1188) : (i64) -> i64
    call @sloth_rc_release(%v1181) : (i64) -> i64
    call @sloth_rc_release(%v1187) : (i64) -> i64
    %v1190 = arith.constant 0 : index
    %v1191 = memref.load %v1017[%v1190] : memref<1xi64>
    call @sloth_rc_release(%v1191) : (i64) -> i64
    %v1192 = memref.extract_aligned_pointer_as_index %v1017 : memref<1xi64> -> index
    %v1193 = arith.index_cast %v1192 : index to i64
    call @sloth_fiber_untrack(%v1193) : (i64) -> i64
    %v1194 = arith.constant 0 : index
    %v1195 = memref.load %v1144[%v1194] : memref<1xi64>
    call @sloth_rc_release(%v1195) : (i64) -> i64
    %v1196 = memref.extract_aligned_pointer_as_index %v1144 : memref<1xi64> -> index
    %v1197 = arith.index_cast %v1196 : index to i64
    call @sloth_fiber_untrack(%v1197) : (i64) -> i64
    %v1198 = arith.constant 0 : index
    %v1199 = memref.load %v1119[%v1198] : memref<1xi64>
    call @sloth_rc_release(%v1199) : (i64) -> i64
    %v1200 = memref.extract_aligned_pointer_as_index %v1119 : memref<1xi64> -> index
    %v1201 = arith.index_cast %v1200 : index to i64
    call @sloth_fiber_untrack(%v1201) : (i64) -> i64
    %v1202 = arith.constant 0 : index
    %v1203 = memref.load %v1096[%v1202] : memref<1xi64>
    call @sloth_rc_release(%v1203) : (i64) -> i64
    %v1204 = memref.extract_aligned_pointer_as_index %v1096 : memref<1xi64> -> index
    %v1205 = arith.index_cast %v1204 : index to i64
    call @sloth_fiber_untrack(%v1205) : (i64) -> i64
    %v1206 = arith.constant 0 : index
    %v1207 = memref.load %v1108[%v1206] : memref<1xi64>
    call @sloth_rc_release(%v1207) : (i64) -> i64
    %v1208 = memref.extract_aligned_pointer_as_index %v1108 : memref<1xi64> -> index
    %v1209 = arith.index_cast %v1208 : index to i64
    call @sloth_fiber_untrack(%v1209) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

