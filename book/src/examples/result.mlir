module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__parse(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1008 = arith.constant 0 : i64
    %v1007 = arith.cmpi slt, %v1005, %v1006 : i64
    %v1009 = arith.extui %v1007 : i1 to i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shli %v1009, %v1010 : i64
    %v1013 = arith.constant 0 : i64
    %v1012 = arith.cmpi ne, %v1011, %v1013 : i64
    cf.cond_br %v1012, ^t_1, ^e_2
  ^t_1:
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.constant 6776174 : i64
    %v1016 = arith.constant 6 : i64
    %v1017 = call @sloth_str_push(%v1014, %v1015, %v1016) : (i64, i64, i64) -> i64
    %v1018 = call @sloth_str_finish(%v1017) : (i64) -> i64
    %v1019 = arith.constant 0 : i64
    %v1020 = arith.constant 4 : i64
    %v1021 = call @sloth_cls_info(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = arith.constant 4 : i64
    %v1023 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1021, %v1022, %v1023) : (i64, i64, i64) -> i64
    %v1024 = arith.constant 6 : i64
    %v1025 = call @sloth_obj_new(%v1021, %v1024) : (i64, i64) -> i64
    %v1026 = arith.constant 0 : i64
    %v1027 = arith.constant 0 : i64
    %v1028 = call @sloth_obj_field(%v1025, %v1027) : (i64, i64) -> i64
    call @sloth_rc_release(%v1028) : (i64) -> i64
    %v1029 = call @sloth_rc_retain(%v1026) : (i64) -> i64
    call @sloth_obj_set_field(%v1025, %v1027, %v1029) : (i64, i64, i64) -> i64
    %v1030 = arith.constant 0 : i64
    %v1032 = arith.constant 0 : i64
    %v1033 = arith.constant 0 : i64
    %v1034 = call @sloth_obj_field(%v1025, %v1033) : (i64, i64) -> i64
    call @sloth_rc_release(%v1034) : (i64) -> i64
    %v1035 = call @sloth_rc_retain(%v1032) : (i64) -> i64
    call @sloth_obj_set_field(%v1025, %v1033, %v1035) : (i64, i64, i64) -> i64
    %v1036 = arith.constant 0 : i64
    %v1037 = arith.constant 2 : i64
    %v1038 = call @sloth_obj_field(%v1025, %v1037) : (i64, i64) -> i64
    call @sloth_rc_release(%v1038) : (i64) -> i64
    %v1039 = call @sloth_rc_retain(%v1036) : (i64) -> i64
    call @sloth_obj_set_field(%v1025, %v1037, %v1039) : (i64, i64, i64) -> i64
    %v1040 = arith.constant 4 : i64
    %v1041 = call @sloth_obj_field(%v1025, %v1040) : (i64, i64) -> i64
    call @sloth_rc_release(%v1041) : (i64) -> i64
    %v1042 = call @sloth_rc_retain(%v1018) : (i64) -> i64
    call @sloth_obj_set_field(%v1025, %v1040, %v1042) : (i64, i64, i64) -> i64
    %v1043 = arith.constant 0 : index
    memref.store %v1025, %v1001[%v1043] : memref<1xi64>
    %v1044 = arith.constant 1 : i64
    memref.store %v1044, %v1000[%v1043] : memref<1xi64>
    call @sloth_rc_release(%v1018) : (i64) -> i64
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1002[%v1045] : memref<1xi64>
    %v1047 = arith.constant 4 : i64
    %v1048 = arith.constant 1 : i64
    %v1049 = arith.shrsi %v1046, %v1048 : i64
    %v1050 = arith.constant 1 : i64
    %v1051 = arith.shrsi %v1047, %v1050 : i64
    %v1052 = arith.muli %v1049, %v1051 : i64
    %v1053 = arith.constant 1 : i64
    %v1054 = arith.shli %v1052, %v1053 : i64
    %v1055 = arith.constant 0 : i64
    %v1056 = arith.constant 4 : i64
    %v1057 = call @sloth_cls_info(%v1055, %v1056) : (i64, i64) -> i64
    %v1058 = arith.constant 4 : i64
    %v1059 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1057, %v1058, %v1059) : (i64, i64, i64) -> i64
    %v1060 = arith.constant 6 : i64
    %v1061 = call @sloth_obj_new(%v1057, %v1060) : (i64, i64) -> i64
    %v1062 = arith.constant 0 : i64
    %v1063 = arith.constant 0 : i64
    %v1064 = call @sloth_obj_field(%v1061, %v1063) : (i64, i64) -> i64
    call @sloth_rc_release(%v1064) : (i64) -> i64
    %v1065 = call @sloth_rc_retain(%v1062) : (i64) -> i64
    call @sloth_obj_set_field(%v1061, %v1063, %v1065) : (i64, i64, i64) -> i64
    %v1066 = arith.constant 0 : i64
    %v1068 = arith.constant 2 : i64
    %v1069 = arith.constant 0 : i64
    %v1070 = call @sloth_obj_field(%v1061, %v1069) : (i64, i64) -> i64
    call @sloth_rc_release(%v1070) : (i64) -> i64
    %v1071 = call @sloth_rc_retain(%v1068) : (i64) -> i64
    call @sloth_obj_set_field(%v1061, %v1069, %v1071) : (i64, i64, i64) -> i64
    %v1072 = arith.constant 2 : i64
    %v1073 = call @sloth_obj_field(%v1061, %v1072) : (i64, i64) -> i64
    call @sloth_rc_release(%v1073) : (i64) -> i64
    %v1074 = call @sloth_rc_retain(%v1054) : (i64) -> i64
    call @sloth_obj_set_field(%v1061, %v1072, %v1074) : (i64, i64, i64) -> i64
    %v1075 = arith.constant 0 : i64
    %v1076 = arith.constant 4 : i64
    %v1077 = call @sloth_obj_field(%v1061, %v1076) : (i64, i64) -> i64
    call @sloth_rc_release(%v1077) : (i64) -> i64
    %v1078 = call @sloth_rc_retain(%v1075) : (i64) -> i64
    call @sloth_obj_set_field(%v1061, %v1076, %v1078) : (i64, i64, i64) -> i64
    %v1079 = arith.constant 0 : index
    memref.store %v1061, %v1001[%v1079] : memref<1xi64>
    %v1080 = arith.constant 1 : i64
    memref.store %v1080, %v1000[%v1079] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1001[%v1081] : memref<1xi64>
    return %v1082 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 10 : i64
    %v1002 = call @sloth_main__parse(%v1001) : (i64) -> i64
    %v1003 = memref.alloca() : memref<1xi64>
    %v1004 = arith.constant 0 : index
    memref.store %v1002, %v1003[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1003[%v1005] : memref<1xi64>
    %v1007 = call @sloth_main_Result_int_str__is_ok(%v1006) : (i64) -> i64
    %v1008 = call @sloth_rt_print_bool(%v1007) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1003[%v1009] : memref<1xi64>
    %v1011 = call @sloth_main_Result_int_str__unwrap(%v1010) : (i64) -> i64
    %v1012 = call @sloth_rt_print_i64(%v1011) : (i64) -> i64
    %v1013 = arith.constant 2 : i64
    %v1016 = arith.constant 1 : i64
    %v1017 = arith.shrsi %v1013, %v1016 : i64
    %v1018 = arith.constant 0 : i64
    %v1019 = arith.subi %v1018, %v1017 : i64
    %v1020 = arith.constant 1 : i64
    %v1021 = arith.shli %v1019, %v1020 : i64
    %v1022 = call @sloth_main__parse(%v1021) : (i64) -> i64
    %v1023 = memref.alloca() : memref<1xi64>
    %v1024 = arith.constant 0 : index
    memref.store %v1022, %v1023[%v1024] : memref<1xi64>
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1023[%v1025] : memref<1xi64>
    %v1027 = call @sloth_main_Result_int_str__is_ok(%v1026) : (i64) -> i64
    %v1028 = call @sloth_rt_print_bool(%v1027) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1023[%v1029] : memref<1xi64>
    %v1031 = call @sloth_main_Result_int_str__err(%v1030) : (i64) -> i64
    %v1032 = call @sloth_rt_print_str(%v1031) : (i64) -> i64
    call @sloth_rc_release(%v1031) : (i64) -> i64
    %v1033 = arith.constant 2306405959167115264 : i64
    %v1034 = arith.constant 0 : i64
    %v1035 = arith.constant 6 : i64
    %v1036 = call @sloth_cls_info(%v1034, %v1035) : (i64, i64) -> i64
    %v1037 = arith.constant 0 : i64
    %v1038 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1036, %v1037, %v1038) : (i64, i64, i64) -> i64
    %v1039 = arith.constant 6 : i64
    %v1040 = call @sloth_obj_new(%v1036, %v1039) : (i64, i64) -> i64
    %v1041 = arith.constant 0 : i64
    %v1042 = arith.constant 0 : i64
    %v1043 = call @sloth_obj_field(%v1040, %v1042) : (i64, i64) -> i64
    call @sloth_rc_release(%v1043) : (i64) -> i64
    %v1044 = call @sloth_rc_retain(%v1041) : (i64) -> i64
    call @sloth_obj_set_field(%v1040, %v1042, %v1044) : (i64, i64, i64) -> i64
    %v1045 = arith.constant 0 : i64
    %v1047 = arith.constant 2 : i64
    %v1048 = arith.constant 0 : i64
    %v1049 = call @sloth_obj_field(%v1040, %v1048) : (i64, i64) -> i64
    call @sloth_rc_release(%v1049) : (i64) -> i64
    %v1050 = call @sloth_rc_retain(%v1047) : (i64) -> i64
    call @sloth_obj_set_field(%v1040, %v1048, %v1050) : (i64, i64, i64) -> i64
    %v1051 = arith.constant 2 : i64
    %v1052 = call @sloth_obj_field(%v1040, %v1051) : (i64, i64) -> i64
    call @sloth_rc_release(%v1052) : (i64) -> i64
    %v1053 = call @sloth_rc_retain(%v1033) : (i64) -> i64
    call @sloth_obj_set_field(%v1040, %v1051, %v1053) : (i64, i64, i64) -> i64
    %v1054 = arith.constant 0 : i64
    %v1055 = arith.constant 4 : i64
    %v1056 = call @sloth_obj_field(%v1040, %v1055) : (i64, i64) -> i64
    call @sloth_rc_release(%v1056) : (i64) -> i64
    %v1057 = call @sloth_rc_retain(%v1054) : (i64) -> i64
    call @sloth_obj_set_field(%v1040, %v1055, %v1057) : (i64, i64, i64) -> i64
    %v1058 = memref.alloca() : memref<1xi64>
    %v1059 = call @sloth_rc_retain(%v1040) : (i64) -> i64
    %v1060 = arith.constant 0 : index
    memref.store %v1059, %v1058[%v1060] : memref<1xi64>
    call @sloth_rc_release(%v1040) : (i64) -> i64
    %v1061 = arith.constant 0 : index
    %v1062 = memref.load %v1058[%v1061] : memref<1xi64>
    %v1063 = call @sloth_main_Result_float_int__unwrap(%v1062) : (i64) -> i64
    %v1065 = arith.constant 1 : i64
    %v1066 = arith.shli %v1063, %v1065 : i64
    %v1067 = llvm.bitcast %v1066 : i64 to f64
    %v1064 = call @sloth_rt_print_f64(%v1067) : (f64) -> i64
    %v1068 = arith.constant 0 : i64
    %v1069 = arith.constant 1684366707 : i64
    %v1070 = arith.constant 8 : i64
    %v1071 = call @sloth_str_push(%v1068, %v1069, %v1070) : (i64, i64, i64) -> i64
    %v1072 = call @sloth_str_finish(%v1071) : (i64) -> i64
    %v1073 = arith.constant 0 : i64
    %v1074 = arith.constant 4 : i64
    %v1075 = call @sloth_cls_info(%v1073, %v1074) : (i64, i64) -> i64
    %v1076 = arith.constant 4 : i64
    %v1077 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1075, %v1076, %v1077) : (i64, i64, i64) -> i64
    %v1078 = arith.constant 6 : i64
    %v1079 = call @sloth_obj_new(%v1075, %v1078) : (i64, i64) -> i64
    %v1080 = arith.constant 0 : i64
    %v1081 = arith.constant 0 : i64
    %v1082 = call @sloth_obj_field(%v1079, %v1081) : (i64, i64) -> i64
    call @sloth_rc_release(%v1082) : (i64) -> i64
    %v1083 = call @sloth_rc_retain(%v1080) : (i64) -> i64
    call @sloth_obj_set_field(%v1079, %v1081, %v1083) : (i64, i64, i64) -> i64
    %v1084 = arith.constant 0 : i64
    %v1086 = arith.constant 0 : i64
    %v1087 = arith.constant 0 : i64
    %v1088 = call @sloth_obj_field(%v1079, %v1087) : (i64, i64) -> i64
    call @sloth_rc_release(%v1088) : (i64) -> i64
    %v1089 = call @sloth_rc_retain(%v1086) : (i64) -> i64
    call @sloth_obj_set_field(%v1079, %v1087, %v1089) : (i64, i64, i64) -> i64
    %v1090 = arith.constant 0 : i64
    %v1091 = arith.constant 2 : i64
    %v1092 = call @sloth_obj_field(%v1079, %v1091) : (i64, i64) -> i64
    call @sloth_rc_release(%v1092) : (i64) -> i64
    %v1093 = call @sloth_rc_retain(%v1090) : (i64) -> i64
    call @sloth_obj_set_field(%v1079, %v1091, %v1093) : (i64, i64, i64) -> i64
    %v1094 = arith.constant 4 : i64
    %v1095 = call @sloth_obj_field(%v1079, %v1094) : (i64, i64) -> i64
    call @sloth_rc_release(%v1095) : (i64) -> i64
    %v1096 = call @sloth_rc_retain(%v1072) : (i64) -> i64
    call @sloth_obj_set_field(%v1079, %v1094, %v1096) : (i64, i64, i64) -> i64
    %v1097 = memref.alloca() : memref<1xi64>
    %v1098 = call @sloth_rc_retain(%v1079) : (i64) -> i64
    %v1099 = arith.constant 0 : index
    memref.store %v1098, %v1097[%v1099] : memref<1xi64>
    call @sloth_rc_release(%v1072) : (i64) -> i64
    call @sloth_rc_release(%v1079) : (i64) -> i64
    %v1100 = arith.constant 16 : i64
    %v1101 = arith.constant 0 : i64
    %v1102 = arith.constant 4 : i64
    %v1103 = call @sloth_cls_info(%v1101, %v1102) : (i64, i64) -> i64
    %v1104 = arith.constant 4 : i64
    %v1105 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1103, %v1104, %v1105) : (i64, i64, i64) -> i64
    %v1106 = arith.constant 6 : i64
    %v1107 = call @sloth_obj_new(%v1103, %v1106) : (i64, i64) -> i64
    %v1108 = arith.constant 0 : i64
    %v1109 = arith.constant 0 : i64
    %v1110 = call @sloth_obj_field(%v1107, %v1109) : (i64, i64) -> i64
    call @sloth_rc_release(%v1110) : (i64) -> i64
    %v1111 = call @sloth_rc_retain(%v1108) : (i64) -> i64
    call @sloth_obj_set_field(%v1107, %v1109, %v1111) : (i64, i64, i64) -> i64
    %v1112 = arith.constant 0 : i64
    %v1114 = arith.constant 2 : i64
    %v1115 = arith.constant 0 : i64
    %v1116 = call @sloth_obj_field(%v1107, %v1115) : (i64, i64) -> i64
    call @sloth_rc_release(%v1116) : (i64) -> i64
    %v1117 = call @sloth_rc_retain(%v1114) : (i64) -> i64
    call @sloth_obj_set_field(%v1107, %v1115, %v1117) : (i64, i64, i64) -> i64
    %v1118 = arith.constant 2 : i64
    %v1119 = call @sloth_obj_field(%v1107, %v1118) : (i64, i64) -> i64
    call @sloth_rc_release(%v1119) : (i64) -> i64
    %v1120 = call @sloth_rc_retain(%v1100) : (i64) -> i64
    call @sloth_obj_set_field(%v1107, %v1118, %v1120) : (i64, i64, i64) -> i64
    %v1121 = arith.constant 0 : i64
    %v1122 = arith.constant 4 : i64
    %v1123 = call @sloth_obj_field(%v1107, %v1122) : (i64, i64) -> i64
    call @sloth_rc_release(%v1123) : (i64) -> i64
    %v1124 = call @sloth_rc_retain(%v1121) : (i64) -> i64
    call @sloth_obj_set_field(%v1107, %v1122, %v1124) : (i64, i64, i64) -> i64
    %v1125 = arith.constant 0 : index
    %v1126 = memref.load %v1097[%v1125] : memref<1xi64>
    call @sloth_rc_release(%v1126) : (i64) -> i64
    %v1127 = call @sloth_rc_retain(%v1107) : (i64) -> i64
    %v1128 = arith.constant 0 : index
    memref.store %v1127, %v1097[%v1128] : memref<1xi64>
    call @sloth_rc_release(%v1107) : (i64) -> i64
    %v1129 = arith.constant 0 : index
    %v1130 = memref.load %v1097[%v1129] : memref<1xi64>
    %v1131 = call @sloth_main_Result_int_str__unwrap(%v1130) : (i64) -> i64
    %v1132 = call @sloth_rt_print_i64(%v1131) : (i64) -> i64
    %v1133 = arith.constant 0 : index
    %v1134 = memref.load %v1003[%v1133] : memref<1xi64>
    call @sloth_rc_release(%v1134) : (i64) -> i64
    %v1135 = arith.constant 0 : index
    %v1136 = memref.load %v1023[%v1135] : memref<1xi64>
    call @sloth_rc_release(%v1136) : (i64) -> i64
    %v1137 = arith.constant 0 : index
    %v1138 = memref.load %v1058[%v1137] : memref<1xi64>
    call @sloth_rc_release(%v1138) : (i64) -> i64
    %v1139 = arith.constant 0 : index
    %v1140 = memref.load %v1097[%v1139] : memref<1xi64>
    call @sloth_rc_release(%v1140) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Result_int_str__is_ok(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1001[%v1008] : memref<1xi64>
    %v1009 = arith.constant 1 : i64
    memref.store %v1009, %v1000[%v1008] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1001[%v1010] : memref<1xi64>
    return %v1011 : i64
  }
  func.func @sloth_main_Result_int_str__unwrap(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = memref.alloca() : memref<1xi64>
    %v1009 = arith.constant 0 : index
    memref.store %v1007, %v1008[%v1009] : memref<1xi64>
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1008[%v1010] : memref<1xi64>
    %v1013 = arith.constant 0 : i64
    %v1012 = arith.cmpi ne, %v1011, %v1013 : i64
    cf.cond_br %v1012, ^t_1, ^e_2
  ^t_1:
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1002[%v1014] : memref<1xi64>
    %v1016 = arith.constant 2 : i64
    %v1017 = call @sloth_obj_field(%v1015, %v1016) : (i64, i64) -> i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1001[%v1018] : memref<1xi64>
    %v1019 = arith.constant 1 : i64
    memref.store %v1019, %v1000[%v1018] : memref<1xi64>
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1021 = call @sloth_panic_unwrap() : () -> i64
    %v1022 = arith.constant 1 : i64
    %v1023 = arith.shli %v1021, %v1022 : i64
    %v1024 = arith.constant 0 : index
    %v1025 = memref.load %v1002[%v1024] : memref<1xi64>
    %v1026 = arith.constant 2 : i64
    %v1027 = call @sloth_obj_field(%v1025, %v1026) : (i64, i64) -> i64
    %v1028 = arith.constant 0 : index
    memref.store %v1027, %v1001[%v1028] : memref<1xi64>
    %v1029 = arith.constant 1 : i64
    memref.store %v1029, %v1000[%v1028] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1001[%v1030] : memref<1xi64>
    return %v1031 : i64
  }
  func.func @sloth_main_Result_int_str__err(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 4 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1001[%v1009] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    memref.store %v1010, %v1000[%v1009] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    return %v1012 : i64
  }
  func.func @sloth_main_Result_float_int__is_ok(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1001[%v1008] : memref<1xi64>
    %v1009 = arith.constant 1 : i64
    memref.store %v1009, %v1000[%v1008] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1001[%v1010] : memref<1xi64>
    return %v1011 : i64
  }
  func.func @sloth_main_Result_float_int__unwrap(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = memref.alloca() : memref<1xi64>
    %v1009 = arith.constant 0 : index
    memref.store %v1007, %v1008[%v1009] : memref<1xi64>
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1008[%v1010] : memref<1xi64>
    %v1013 = arith.constant 0 : i64
    %v1012 = arith.cmpi ne, %v1011, %v1013 : i64
    cf.cond_br %v1012, ^t_1, ^e_2
  ^t_1:
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1002[%v1014] : memref<1xi64>
    %v1016 = arith.constant 2 : i64
    %v1017 = call @sloth_obj_field(%v1015, %v1016) : (i64, i64) -> i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1001[%v1018] : memref<1xi64>
    %v1019 = arith.constant 1 : i64
    memref.store %v1019, %v1000[%v1018] : memref<1xi64>
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1021 = call @sloth_panic_unwrap() : () -> i64
    %v1022 = arith.constant 1 : i64
    %v1023 = arith.shli %v1021, %v1022 : i64
    %v1024 = arith.constant 0 : index
    %v1025 = memref.load %v1002[%v1024] : memref<1xi64>
    %v1026 = arith.constant 2 : i64
    %v1027 = call @sloth_obj_field(%v1025, %v1026) : (i64, i64) -> i64
    %v1028 = arith.constant 0 : index
    memref.store %v1027, %v1001[%v1028] : memref<1xi64>
    %v1029 = arith.constant 1 : i64
    memref.store %v1029, %v1000[%v1028] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1001[%v1030] : memref<1xi64>
    return %v1031 : i64
  }
  func.func @sloth_main_Result_float_int__err(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 4 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1001[%v1008] : memref<1xi64>
    %v1009 = arith.constant 1 : i64
    memref.store %v1009, %v1000[%v1008] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1001[%v1010] : memref<1xi64>
    return %v1011 : i64
  }
}

