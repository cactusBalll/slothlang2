module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = memref.alloca() : memref<1xi64>
    %v1003 = call @sloth_rc_retain(%v1001) : (i64) -> i64
    %v1004 = arith.constant 0 : index
    memref.store %v1003, %v1002[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : i64
    %v1006 = arith.constant 4 : i64
    %v1007 = call @sloth_cls_info(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 3 : i64
    %v1009 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1007, %v1008, %v1009) : (i64, i64, i64) -> i64
    %v1010 = arith.constant 4 : i64
    %v1011 = call @sloth_obj_new(%v1007, %v1010) : (i64, i64) -> i64
    %v1012 = arith.constant 0 : i64
    %v1013 = arith.constant 110 : i64
    %v1014 = arith.constant 2 : i64
    %v1015 = call @sloth_str_push(%v1012, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1016 = call @sloth_str_finish(%v1015) : (i64) -> i64
    %v1017 = arith.constant 0 : i64
    %v1018 = call @sloth_obj_field(%v1011, %v1017) : (i64, i64) -> i64
    call @sloth_rc_release(%v1018) : (i64) -> i64
    %v1019 = call @sloth_rc_retain(%v1016) : (i64) -> i64
    call @sloth_obj_set_field(%v1011, %v1017, %v1019) : (i64, i64, i64) -> i64
    %v1020 = arith.constant 0 : i64
    %v1021 = arith.constant 2 : i64
    %v1022 = call @sloth_obj_field(%v1011, %v1021) : (i64, i64) -> i64
    call @sloth_rc_release(%v1022) : (i64) -> i64
    %v1023 = call @sloth_rc_retain(%v1020) : (i64) -> i64
    call @sloth_obj_set_field(%v1011, %v1021, %v1023) : (i64, i64, i64) -> i64
    %v1024 = memref.alloca() : memref<1xi64>
    %v1025 = call @sloth_rc_retain(%v1011) : (i64) -> i64
    %v1026 = arith.constant 0 : index
    memref.store %v1025, %v1024[%v1026] : memref<1xi64>
    call @sloth_rc_release(%v1011) : (i64) -> i64
    call @sloth_rc_release(%v1016) : (i64) -> i64
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1024[%v1027] : memref<1xi64>
    %v1029 = call @sloth_weak_new(%v1028) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1002[%v1030] : memref<1xi64>
    call @sloth_rc_release(%v1031) : (i64) -> i64
    %v1032 = call @sloth_rc_retain(%v1029) : (i64) -> i64
    %v1033 = arith.constant 0 : index
    memref.store %v1032, %v1002[%v1033] : memref<1xi64>
    call @sloth_rc_release(%v1029) : (i64) -> i64
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1024[%v1034] : memref<1xi64>
    call @sloth_rc_release(%v1035) : (i64) -> i64
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1002[%v1036] : memref<1xi64>
    %v1038 = call @sloth_weak_upgrade(%v1037) : (i64) -> i64
    %v1039 = call @sloth_rc_retain(%v1038) : (i64) -> i64
    %v1040 = memref.alloca() : memref<1xi64>
    %v1041 = call @sloth_rc_retain(%v1039) : (i64) -> i64
    %v1042 = arith.constant 0 : index
    memref.store %v1041, %v1040[%v1042] : memref<1xi64>
    call @sloth_rc_release(%v1039) : (i64) -> i64
    %v1043 = arith.constant 0 : index
    %v1044 = memref.load %v1040[%v1043] : memref<1xi64>
    %v1045 = arith.constant 0 : i64
    %v1046 = arith.cmpi eq, %v1044, %v1045 : i64
    %v1047 = arith.extui %v1046 : i1 to i64
    %v1048 = arith.constant 1 : i64
    %v1049 = arith.shli %v1047, %v1048 : i64
    %v1050 = call @sloth_rt_print_bool(%v1049) : (i64) -> i64
    %v1051 = arith.constant 0 : i64
    %v1052 = arith.constant 4 : i64
    %v1053 = call @sloth_cls_info(%v1051, %v1052) : (i64, i64) -> i64
    %v1054 = arith.constant 3 : i64
    %v1055 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1053, %v1054, %v1055) : (i64, i64, i64) -> i64
    %v1056 = arith.constant 4 : i64
    %v1057 = call @sloth_obj_new(%v1053, %v1056) : (i64, i64) -> i64
    %v1058 = arith.constant 0 : i64
    %v1059 = arith.constant 110 : i64
    %v1060 = arith.constant 2 : i64
    %v1061 = call @sloth_str_push(%v1058, %v1059, %v1060) : (i64, i64, i64) -> i64
    %v1062 = call @sloth_str_finish(%v1061) : (i64) -> i64
    %v1063 = arith.constant 0 : i64
    %v1064 = call @sloth_obj_field(%v1057, %v1063) : (i64, i64) -> i64
    call @sloth_rc_release(%v1064) : (i64) -> i64
    %v1065 = call @sloth_rc_retain(%v1062) : (i64) -> i64
    call @sloth_obj_set_field(%v1057, %v1063, %v1065) : (i64, i64, i64) -> i64
    %v1066 = arith.constant 0 : i64
    %v1067 = arith.constant 2 : i64
    %v1068 = call @sloth_obj_field(%v1057, %v1067) : (i64, i64) -> i64
    call @sloth_rc_release(%v1068) : (i64) -> i64
    %v1069 = call @sloth_rc_retain(%v1066) : (i64) -> i64
    call @sloth_obj_set_field(%v1057, %v1067, %v1069) : (i64, i64, i64) -> i64
    %v1070 = memref.alloca() : memref<1xi64>
    %v1071 = call @sloth_rc_retain(%v1057) : (i64) -> i64
    %v1072 = arith.constant 0 : index
    memref.store %v1071, %v1070[%v1072] : memref<1xi64>
    call @sloth_rc_release(%v1057) : (i64) -> i64
    call @sloth_rc_release(%v1062) : (i64) -> i64
    %v1073 = arith.constant 0 : i64
    %v1074 = arith.constant 435778317409 : i64
    %v1075 = arith.constant 10 : i64
    %v1076 = call @sloth_str_push(%v1073, %v1074, %v1075) : (i64, i64, i64) -> i64
    %v1077 = call @sloth_str_finish(%v1076) : (i64) -> i64
    %v1078 = arith.constant 0 : index
    %v1079 = memref.load %v1070[%v1078] : memref<1xi64>
    %v1080 = arith.constant 0 : i64
    %v1081 = call @sloth_obj_field(%v1079, %v1080) : (i64, i64) -> i64
    call @sloth_rc_release(%v1081) : (i64) -> i64
    %v1082 = call @sloth_rc_retain(%v1077) : (i64) -> i64
    call @sloth_obj_set_field(%v1079, %v1080, %v1082) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1077) : (i64) -> i64
    %v1083 = arith.constant 0 : index
    %v1084 = memref.load %v1070[%v1083] : memref<1xi64>
    %v1085 = call @sloth_weak_new(%v1084) : (i64) -> i64
    %v1086 = memref.alloca() : memref<1xi64>
    %v1087 = call @sloth_rc_retain(%v1085) : (i64) -> i64
    %v1088 = arith.constant 0 : index
    memref.store %v1087, %v1086[%v1088] : memref<1xi64>
    call @sloth_rc_release(%v1085) : (i64) -> i64
    %v1089 = arith.constant 0 : index
    %v1090 = memref.load %v1086[%v1089] : memref<1xi64>
    %v1091 = call @sloth_weak_upgrade(%v1090) : (i64) -> i64
    %v1092 = call @sloth_rc_retain(%v1091) : (i64) -> i64
    %v1093 = memref.alloca() : memref<1xi64>
    %v1094 = call @sloth_rc_retain(%v1092) : (i64) -> i64
    %v1095 = arith.constant 0 : index
    memref.store %v1094, %v1093[%v1095] : memref<1xi64>
    call @sloth_rc_release(%v1092) : (i64) -> i64
    %v1096 = arith.constant 0 : index
    %v1097 = memref.load %v1093[%v1096] : memref<1xi64>
    %v1098 = arith.constant 0 : i64
    %v1099 = arith.cmpi eq, %v1097, %v1098 : i64
    %v1100 = arith.extui %v1099 : i1 to i64
    %v1101 = arith.constant 1 : i64
    %v1102 = arith.shli %v1100, %v1101 : i64
    %v1103 = arith.constant 2 : i64
    %v1104 = arith.xori %v1102, %v1103 : i64
    %v1106 = arith.constant 0 : i64
    %v1105 = arith.cmpi ne, %v1104, %v1106 : i64
    cf.cond_br %v1105, ^t_1, ^e_2
  ^t_1:
    %v1107 = arith.constant 0 : index
    %v1108 = memref.load %v1093[%v1107] : memref<1xi64>
    %v1109 = arith.constant 0 : i64
    %v1110 = call @sloth_obj_field(%v1108, %v1109) : (i64, i64) -> i64
    %v1111 = call @sloth_rt_print_str(%v1110) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1112 = arith.constant 0 : index
    %v1113 = memref.load %v1070[%v1112] : memref<1xi64>
    call @sloth_rc_release(%v1113) : (i64) -> i64
    %v1114 = arith.constant 0 : index
    %v1115 = memref.load %v1086[%v1114] : memref<1xi64>
    call @sloth_rc_release(%v1115) : (i64) -> i64
    %v1116 = arith.constant 0 : index
    %v1117 = memref.load %v1040[%v1116] : memref<1xi64>
    call @sloth_rc_release(%v1117) : (i64) -> i64
    %v1118 = arith.constant 0 : index
    %v1119 = memref.load %v1002[%v1118] : memref<1xi64>
    call @sloth_rc_release(%v1119) : (i64) -> i64
    %v1120 = arith.constant 0 : index
    %v1121 = memref.load %v1093[%v1120] : memref<1xi64>
    call @sloth_rc_release(%v1121) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

