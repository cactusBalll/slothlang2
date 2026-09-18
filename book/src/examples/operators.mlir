module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__double(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 4 : i64
    %v1007 = arith.constant 1 : i64
    %v1008 = arith.shrsi %v1005, %v1007 : i64
    %v1009 = arith.constant 1 : i64
    %v1010 = arith.shrsi %v1006, %v1009 : i64
    %v1011 = arith.muli %v1008, %v1010 : i64
    %v1012 = arith.constant 1 : i64
    %v1013 = arith.shli %v1011, %v1012 : i64
    %v1014 = arith.constant 0 : index
    memref.store %v1013, %v1001[%v1014] : memref<1xi64>
    %v1015 = arith.constant 1 : i64
    memref.store %v1015, %v1000[%v1014] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1001[%v1016] : memref<1xi64>
    return %v1017 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 14 : i64
    %v1002 = arith.constant 4 : i64
    %v1003 = arith.constant 1 : i64
    %v1004 = arith.shrsi %v1001, %v1003 : i64
    %v1005 = arith.constant 1 : i64
    %v1006 = arith.shrsi %v1002, %v1005 : i64
    %v1007 = memref.alloca() : memref<1xi64>
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.cmpi eq, %v1006, %v1008 : i64
    %v1010 = arith.extui %v1009 : i1 to i64
    %v1012 = arith.constant 0 : i64
    %v1011 = arith.cmpi ne, %v1010, %v1012 : i64
    cf.cond_br %v1011, ^dz_1, ^dz_2
  ^dz_1:
    call @sloth_panic_divzero() : () -> i64
    %v1013 = arith.constant 0 : index
    memref.store %v1008, %v1007[%v1013] : memref<1xi64>
    cf.br ^dz_3
  ^dz_2:
    %v1014 = arith.divsi %v1004, %v1006 : i64
    %v1015 = arith.constant 0 : index
    memref.store %v1014, %v1007[%v1015] : memref<1xi64>
    cf.br ^dz_3
  ^dz_3:
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1007[%v1016] : memref<1xi64>
    %v1018 = arith.constant 1 : i64
    %v1019 = arith.shli %v1017, %v1018 : i64
    %v1020 = call @sloth_rt_print_i64(%v1019) : (i64) -> i64
    %v1021 = arith.constant 14 : i64
    %v1022 = arith.constant 6 : i64
    %v1023 = arith.constant 1 : i64
    %v1024 = arith.shrsi %v1021, %v1023 : i64
    %v1025 = arith.constant 1 : i64
    %v1026 = arith.shrsi %v1022, %v1025 : i64
    %v1027 = memref.alloca() : memref<1xi64>
    %v1028 = arith.constant 0 : i64
    %v1029 = arith.cmpi eq, %v1026, %v1028 : i64
    %v1030 = arith.extui %v1029 : i1 to i64
    %v1032 = arith.constant 0 : i64
    %v1031 = arith.cmpi ne, %v1030, %v1032 : i64
    cf.cond_br %v1031, ^dz_4, ^dz_5
  ^dz_4:
    call @sloth_panic_divzero() : () -> i64
    %v1033 = arith.constant 0 : index
    memref.store %v1028, %v1027[%v1033] : memref<1xi64>
    cf.br ^dz_6
  ^dz_5:
    %v1034 = arith.remsi %v1024, %v1026 : i64
    %v1035 = arith.constant 0 : index
    memref.store %v1034, %v1027[%v1035] : memref<1xi64>
    cf.br ^dz_6
  ^dz_6:
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1027[%v1036] : memref<1xi64>
    %v1038 = arith.constant 1 : i64
    %v1039 = arith.shli %v1037, %v1038 : i64
    %v1040 = call @sloth_rt_print_i64(%v1039) : (i64) -> i64
    %v1041 = arith.constant 4 : i64
    %v1042 = arith.constant 6 : i64
    %v1043 = arith.constant 8 : i64
    %v1044 = arith.constant 1 : i64
    %v1045 = arith.shrsi %v1042, %v1044 : i64
    %v1046 = arith.constant 1 : i64
    %v1047 = arith.shrsi %v1043, %v1046 : i64
    %v1048 = arith.muli %v1045, %v1047 : i64
    %v1049 = arith.constant 1 : i64
    %v1050 = arith.shli %v1048, %v1049 : i64
    %v1051 = arith.constant 1 : i64
    %v1052 = arith.shrsi %v1041, %v1051 : i64
    %v1053 = arith.constant 1 : i64
    %v1054 = arith.shrsi %v1050, %v1053 : i64
    %v1055 = arith.addi %v1052, %v1054 : i64
    %v1056 = arith.constant 1 : i64
    %v1057 = arith.shli %v1055, %v1056 : i64
    %v1058 = call @sloth_rt_print_i64(%v1057) : (i64) -> i64
    %v1059 = arith.constant 4 : i64
    %v1060 = arith.constant 6 : i64
    %v1061 = arith.constant 1 : i64
    %v1062 = arith.shrsi %v1059, %v1061 : i64
    %v1063 = arith.constant 1 : i64
    %v1064 = arith.shrsi %v1060, %v1063 : i64
    %v1065 = arith.addi %v1062, %v1064 : i64
    %v1066 = arith.constant 1 : i64
    %v1067 = arith.shli %v1065, %v1066 : i64
    %v1068 = arith.constant 8 : i64
    %v1069 = arith.constant 1 : i64
    %v1070 = arith.shrsi %v1067, %v1069 : i64
    %v1071 = arith.constant 1 : i64
    %v1072 = arith.shrsi %v1068, %v1071 : i64
    %v1073 = arith.muli %v1070, %v1072 : i64
    %v1074 = arith.constant 1 : i64
    %v1075 = arith.shli %v1073, %v1074 : i64
    %v1076 = call @sloth_rt_print_i64(%v1075) : (i64) -> i64
    %v1077 = arith.constant 0 : i64
    %v1079 = arith.constant 1 : i64
    %v1080 = arith.constant 1 : i64
    %v1081 = arith.shrsi %v1077, %v1080 : i64
    %v1082 = arith.subi %v1079, %v1081 : i64
    %v1083 = arith.constant 1 : i64
    %v1084 = arith.shli %v1082, %v1083 : i64
    %v1085 = call @sloth_rt_print_bool(%v1084) : (i64) -> i64
    %v1086 = arith.constant 2 : i64
    %v1087 = arith.constant 4 : i64
    %v1089 = arith.constant 0 : i64
    %v1088 = arith.cmpi slt, %v1086, %v1087 : i64
    %v1090 = arith.extui %v1088 : i1 to i64
    %v1091 = arith.constant 1 : i64
    %v1092 = arith.shli %v1090, %v1091 : i64
    %v1093 = memref.alloca() : memref<1xi64>
    %v1094 = arith.constant 0 : index
    memref.store %v1092, %v1093[%v1094] : memref<1xi64>
    %v1095 = arith.constant 1 : i64
    %v1096 = arith.shrsi %v1092, %v1095 : i64
    %v1097 = arith.constant 0 : i64
    %v1098 = arith.cmpi ne, %v1096, %v1097 : i64
    %v1099 = arith.extui %v1098 : i1 to i64
    %v1101 = arith.constant 0 : i64
    %v1100 = arith.cmpi ne, %v1099, %v1101 : i64
    cf.cond_br %v1100, ^sc_7, ^sc_8
  ^sc_7:
    %v1102 = arith.constant 4 : i64
    %v1103 = arith.constant 6 : i64
    %v1105 = arith.constant 0 : i64
    %v1104 = arith.cmpi slt, %v1102, %v1103 : i64
    %v1106 = arith.extui %v1104 : i1 to i64
    %v1107 = arith.constant 1 : i64
    %v1108 = arith.shli %v1106, %v1107 : i64
    %v1109 = arith.constant 0 : index
    memref.store %v1108, %v1093[%v1109] : memref<1xi64>
    cf.br ^sc_8
  ^sc_8:
    %v1110 = arith.constant 0 : index
    %v1111 = memref.load %v1093[%v1110] : memref<1xi64>
    %v1112 = call @sloth_rt_print_bool(%v1111) : (i64) -> i64
    %v1113 = arith.constant 2 : i64
    %v1114 = arith.constant 4 : i64
    %v1116 = arith.constant 0 : i64
    %v1115 = arith.cmpi sgt, %v1113, %v1114 : i64
    %v1117 = arith.extui %v1115 : i1 to i64
    %v1118 = arith.constant 1 : i64
    %v1119 = arith.shli %v1117, %v1118 : i64
    %v1120 = memref.alloca() : memref<1xi64>
    %v1121 = arith.constant 0 : index
    memref.store %v1119, %v1120[%v1121] : memref<1xi64>
    %v1122 = arith.constant 1 : i64
    %v1123 = arith.shrsi %v1119, %v1122 : i64
    %v1124 = arith.constant 0 : i64
    %v1125 = arith.cmpi ne, %v1123, %v1124 : i64
    %v1126 = arith.extui %v1125 : i1 to i64
    %v1128 = arith.constant 0 : i64
    %v1127 = arith.cmpi ne, %v1126, %v1128 : i64
    cf.cond_br %v1127, ^sc_10, ^sc_9
  ^sc_9:
    %v1129 = arith.constant 4 : i64
    %v1130 = arith.constant 2 : i64
    %v1132 = arith.constant 0 : i64
    %v1131 = arith.cmpi sgt, %v1129, %v1130 : i64
    %v1133 = arith.extui %v1131 : i1 to i64
    %v1134 = arith.constant 1 : i64
    %v1135 = arith.shli %v1133, %v1134 : i64
    %v1136 = arith.constant 0 : index
    memref.store %v1135, %v1120[%v1136] : memref<1xi64>
    cf.br ^sc_10
  ^sc_10:
    %v1137 = arith.constant 0 : index
    %v1138 = memref.load %v1120[%v1137] : memref<1xi64>
    %v1139 = call @sloth_rt_print_bool(%v1138) : (i64) -> i64
    %v1140 = arith.constant 10 : i64
    %v1141 = call @sloth_main__double(%v1140) : (i64) -> i64
    %v1142 = call @sloth_rt_print_i64(%v1141) : (i64) -> i64
    %v1143 = arith.constant 4 : i64
    %v1144 = call @sloth_main__double(%v1143) : (i64) -> i64
    %v1145 = call @sloth_main__double(%v1144) : (i64) -> i64
    %v1146 = call @sloth_rt_print_i64(%v1145) : (i64) -> i64
    %v1147 = arith.constant 0 : i64
    %v1148 = memref.alloca() : memref<1xi64>
    %v1149 = call @sloth_rc_retain(%v1147) : (i64) -> i64
    %v1150 = arith.constant 0 : index
    memref.store %v1149, %v1148[%v1150] : memref<1xi64>
    %v1151 = arith.constant 0 : index
    %v1152 = memref.load %v1148[%v1151] : memref<1xi64>
    %v1153 = arith.constant 14 : i64
    %v1154 = call @sloth_box_get(%v1152) : (i64) -> i64
    %v1155 = arith.constant 0 : i64
    %v1156 = arith.cmpi ne, %v1152, %v1155 : i64
    %v1157 = arith.select %v1156, %v1154, %v1153 : i64
    %v1158 = call @sloth_rt_print_i64(%v1157) : (i64) -> i64
    %v1159 = arith.constant 6 : i64
    %v1160 = arith.constant 2 : i64
    %v1161 = call @sloth_rt_print_bool(%v1160) : (i64) -> i64
    %v1162 = arith.constant 0 : i64
    %v1163 = arith.constant 120 : i64
    %v1164 = arith.constant 2 : i64
    %v1165 = call @sloth_str_push(%v1162, %v1163, %v1164) : (i64, i64, i64) -> i64
    %v1166 = call @sloth_str_finish(%v1165) : (i64) -> i64
    %v1167 = arith.constant 0 : i64
    %v1168 = call @sloth_rt_print_bool(%v1167) : (i64) -> i64
    call @sloth_rc_release(%v1166) : (i64) -> i64
    %v1169 = arith.constant 0 : i64
    %v1170 = memref.alloca() : memref<1xi64>
    %v1171 = arith.constant 0 : index
    memref.store %v1169, %v1170[%v1171] : memref<1xi64>
    %v1172 = arith.constant 0 : i64
    %v1173 = arith.constant 6 : i64
    %v1175 = arith.constant 0 : index
    %v1174 = memref.alloca() : memref<1xi64>
    memref.store %v1172, %v1174[%v1175] : memref<1xi64>
    cf.br ^fr_11
  ^fr_11:
    %v1176 = memref.load %v1174[%v1175] : memref<1xi64>
    %v1177 = arith.cmpi slt, %v1176, %v1173 : i64
    %v1178 = arith.extui %v1177 : i1 to i64
    %v1180 = arith.constant 0 : i64
    %v1179 = arith.cmpi ne, %v1178, %v1180 : i64
    cf.cond_br %v1179, ^fb_12, ^fd_13
  ^fb_12:
    %v1181 = memref.alloca() : memref<1xi64>
    memref.store %v1176, %v1181[%v1175] : memref<1xi64>
    %v1182 = arith.constant 0 : index
    %v1183 = memref.load %v1170[%v1182] : memref<1xi64>
    %v1184 = arith.constant 0 : index
    %v1185 = memref.load %v1181[%v1184] : memref<1xi64>
    %v1186 = arith.constant 1 : i64
    %v1187 = arith.shrsi %v1183, %v1186 : i64
    %v1188 = arith.constant 1 : i64
    %v1189 = arith.shrsi %v1185, %v1188 : i64
    %v1190 = arith.addi %v1187, %v1189 : i64
    %v1191 = arith.constant 1 : i64
    %v1192 = arith.shli %v1190, %v1191 : i64
    %v1193 = arith.constant 0 : index
    %v1194 = memref.load %v1170[%v1193] : memref<1xi64>
    call @sloth_rc_release(%v1194) : (i64) -> i64
    %v1195 = call @sloth_rc_retain(%v1192) : (i64) -> i64
    %v1196 = arith.constant 0 : index
    memref.store %v1195, %v1170[%v1196] : memref<1xi64>
    cf.br ^fc_14
  ^fc_14:
    %v1197 = arith.constant 2 : i64
    %v1198 = arith.addi %v1176, %v1197 : i64
    memref.store %v1198, %v1174[%v1175] : memref<1xi64>
    cf.br ^fr_11
  ^fd_13:
    %v1199 = arith.constant 0 : index
    %v1200 = memref.load %v1170[%v1199] : memref<1xi64>
    %v1201 = call @sloth_rt_print_i64(%v1200) : (i64) -> i64
    %v1202 = arith.constant 0 : index
    %v1203 = memref.load %v1148[%v1202] : memref<1xi64>
    call @sloth_rc_release(%v1203) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

