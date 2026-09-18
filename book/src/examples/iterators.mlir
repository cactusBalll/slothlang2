module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 4 : i64
    %v1003 = call @sloth_cls_info(%v1001, %v1002) : (i64, i64) -> i64
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1003, %v1004, %v1005) : (i64, i64, i64) -> i64
    %v1006 = arith.constant 2 : i64
    %v1007 = call @sloth_obj_new(%v1003, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.constant 0 : i64
    %v1010 = call @sloth_obj_field(%v1007, %v1009) : (i64, i64) -> i64
    call @sloth_rc_release(%v1010) : (i64) -> i64
    %v1011 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    call @sloth_obj_set_field(%v1007, %v1009, %v1011) : (i64, i64, i64) -> i64
    %v1012 = memref.alloca() : memref<1xi64>
    %v1013 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    %v1014 = arith.constant 0 : index
    memref.store %v1013, %v1012[%v1014] : memref<1xi64>
    call @sloth_rc_release(%v1007) : (i64) -> i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1012[%v1015] : memref<1xi64>
    %v1017 = call @sloth_main_Range3__iter(%v1016) : (i64) -> i64
    %v1019 = arith.constant 0 : index
    %v1018 = memref.alloca() : memref<1xi64>
    memref.store %v1017, %v1018[%v1019] : memref<1xi64>
    cf.br ^if_1
  ^if_1:
    %v1020 = memref.load %v1018[%v1019] : memref<1xi64>
    %v1021 = call @sloth_main_Range3__next(%v1020) : (i64) -> i64
    %v1023 = arith.constant 0 : i64
    %v1022 = arith.cmpi eq, %v1021, %v1023 : i64
    %v1024 = arith.extui %v1022 : i1 to i64
    %v1026 = arith.constant 0 : i64
    %v1025 = arith.cmpi ne, %v1024, %v1026 : i64
    cf.cond_br %v1025, ^ie_5, ^ib_2
  ^ib_2:
    %v1028 = call @sloth_box_get(%v1021) : (i64) -> i64
    %v1027 = memref.alloca() : memref<1xi64>
    memref.store %v1028, %v1027[%v1019] : memref<1xi64>
    call @sloth_rc_release(%v1021) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1027[%v1029] : memref<1xi64>
    %v1031 = call @sloth_rt_print_i64(%v1030) : (i64) -> i64
    cf.br ^ic_3
  ^ic_3:
    cf.br ^if_1
  ^ix_4:
    cf.br ^ie_5
  ^ie_5:
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1018[%v1032] : memref<1xi64>
    call @sloth_rc_release(%v1033) : (i64) -> i64
    %v1034 = arith.constant 0 : i64
    %v1035 = memref.alloca() : memref<1xi64>
    %v1036 = arith.constant 0 : index
    memref.store %v1034, %v1035[%v1036] : memref<1xi64>
    %v1037 = arith.constant 0 : i64
    %v1038 = arith.constant 6 : i64
    %v1039 = arith.constant 2 : i64
    %v1040 = arith.addi %v1038, %v1039 : i64
    %v1042 = arith.constant 0 : index
    %v1041 = memref.alloca() : memref<1xi64>
    memref.store %v1037, %v1041[%v1042] : memref<1xi64>
    cf.br ^fr_6
  ^fr_6:
    %v1043 = memref.load %v1041[%v1042] : memref<1xi64>
    %v1044 = arith.cmpi slt, %v1043, %v1040 : i64
    %v1045 = arith.extui %v1044 : i1 to i64
    %v1047 = arith.constant 0 : i64
    %v1046 = arith.cmpi ne, %v1045, %v1047 : i64
    cf.cond_br %v1046, ^fb_7, ^fd_8
  ^fb_7:
    %v1048 = memref.alloca() : memref<1xi64>
    memref.store %v1043, %v1048[%v1042] : memref<1xi64>
    %v1049 = arith.constant 0 : index
    %v1050 = memref.load %v1035[%v1049] : memref<1xi64>
    %v1051 = arith.constant 0 : index
    %v1052 = memref.load %v1048[%v1051] : memref<1xi64>
    %v1053 = arith.constant 1 : i64
    %v1054 = arith.shrsi %v1050, %v1053 : i64
    %v1055 = arith.constant 1 : i64
    %v1056 = arith.shrsi %v1052, %v1055 : i64
    %v1057 = arith.addi %v1054, %v1056 : i64
    %v1058 = arith.constant 1 : i64
    %v1059 = arith.shli %v1057, %v1058 : i64
    %v1060 = arith.constant 0 : index
    %v1061 = memref.load %v1035[%v1060] : memref<1xi64>
    call @sloth_rc_release(%v1061) : (i64) -> i64
    %v1062 = call @sloth_rc_retain(%v1059) : (i64) -> i64
    %v1063 = arith.constant 0 : index
    memref.store %v1062, %v1035[%v1063] : memref<1xi64>
    cf.br ^fc_9
  ^fc_9:
    %v1064 = arith.constant 2 : i64
    %v1065 = arith.addi %v1043, %v1064 : i64
    memref.store %v1065, %v1041[%v1042] : memref<1xi64>
    cf.br ^fr_6
  ^fd_8:
    %v1066 = arith.constant 0 : index
    %v1067 = memref.load %v1035[%v1066] : memref<1xi64>
    %v1068 = call @sloth_rt_print_i64(%v1067) : (i64) -> i64
    %v1069 = arith.constant 2 : i64
    %v1070 = arith.constant 20 : i64
    %v1071 = arith.constant 0 : i64
    %v1072 = call @sloth_map_new(%v1071) : (i64) -> i64
    call @sloth_map_set(%v1072, %v1069, %v1070) : (i64, i64, i64) -> i64
    %v1073 = call @sloth_map_keys(%v1072) : (i64) -> i64
    %v1074 = call @sloth_arr_len(%v1073) : (i64) -> i64
    %v1076 = arith.constant 0 : index
    %v1075 = memref.alloca() : memref<1xi64>
    %v1077 = arith.constant 0 : i64
    memref.store %v1077, %v1075[%v1076] : memref<1xi64>
    cf.br ^me_10
  ^me_10:
    %v1078 = memref.load %v1075[%v1076] : memref<1xi64>
    %v1079 = arith.cmpi slt, %v1078, %v1074 : i64
    %v1080 = arith.extui %v1079 : i1 to i64
    %v1082 = arith.constant 0 : i64
    %v1081 = arith.cmpi ne, %v1080, %v1082 : i64
    cf.cond_br %v1081, ^mb_11, ^md_12
  ^mb_11:
    %v1083 = call @sloth_arr_get(%v1073, %v1078) : (i64, i64) -> i64
    %v1084 = call @sloth_map_get(%v1072, %v1083) : (i64, i64) -> i64
    %v1085 = arith.constant 0 : i64
    %v1086 = arith.constant 6 : i64
    %v1087 = call @sloth_cls_info(%v1085, %v1086) : (i64, i64) -> i64
    %v1088 = arith.constant 0 : i64
    %v1089 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1087, %v1088, %v1089) : (i64, i64, i64) -> i64
    %v1090 = arith.constant 4 : i64
    %v1091 = call @sloth_obj_new(%v1087, %v1090) : (i64, i64) -> i64
    %v1092 = arith.constant 0 : i64
    %v1093 = call @sloth_obj_field(%v1091, %v1092) : (i64, i64) -> i64
    call @sloth_rc_release(%v1093) : (i64) -> i64
    %v1094 = call @sloth_rc_retain(%v1083) : (i64) -> i64
    call @sloth_obj_set_field(%v1091, %v1092, %v1094) : (i64, i64, i64) -> i64
    %v1095 = arith.constant 2 : i64
    %v1096 = call @sloth_obj_field(%v1091, %v1095) : (i64, i64) -> i64
    call @sloth_rc_release(%v1096) : (i64) -> i64
    %v1097 = call @sloth_rc_retain(%v1084) : (i64) -> i64
    call @sloth_obj_set_field(%v1091, %v1095, %v1097) : (i64, i64, i64) -> i64
    %v1098 = memref.alloca() : memref<1xi64>
    memref.store %v1091, %v1098[%v1076] : memref<1xi64>
    %v1099 = arith.constant 0 : index
    %v1100 = memref.load %v1098[%v1099] : memref<1xi64>
    %v1101 = arith.constant 0 : i64
    %v1102 = call @sloth_obj_field(%v1100, %v1101) : (i64, i64) -> i64
    %v1103 = arith.constant 0 : index
    %v1104 = memref.load %v1098[%v1103] : memref<1xi64>
    %v1105 = arith.constant 2 : i64
    %v1106 = call @sloth_obj_field(%v1104, %v1105) : (i64, i64) -> i64
    %v1107 = arith.constant 1 : i64
    %v1108 = arith.shrsi %v1102, %v1107 : i64
    %v1109 = arith.constant 1 : i64
    %v1110 = arith.shrsi %v1106, %v1109 : i64
    %v1111 = arith.addi %v1108, %v1110 : i64
    %v1112 = arith.constant 1 : i64
    %v1113 = arith.shli %v1111, %v1112 : i64
    %v1114 = call @sloth_rt_print_i64(%v1113) : (i64) -> i64
    cf.br ^mc_13
  ^mc_13:
    call @sloth_rc_release(%v1091) : (i64) -> i64
    %v1115 = arith.constant 2 : i64
    %v1116 = arith.addi %v1078, %v1115 : i64
    memref.store %v1116, %v1075[%v1076] : memref<1xi64>
    cf.br ^me_10
  ^mx_14:
    %v1117 = memref.load %v1098[%v1076] : memref<1xi64>
    call @sloth_rc_release(%v1117) : (i64) -> i64
    cf.br ^md_12
  ^md_12:
    call @sloth_rc_release(%v1073) : (i64) -> i64
    call @sloth_rc_release(%v1072) : (i64) -> i64
    %v1118 = arith.constant 0 : index
    %v1119 = memref.load %v1012[%v1118] : memref<1xi64>
    call @sloth_rc_release(%v1119) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Range3__iter(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1007 = arith.constant 0 : index
    memref.store %v1006, %v1001[%v1007] : memref<1xi64>
    %v1008 = arith.constant 1 : i64
    memref.store %v1008, %v1000[%v1007] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1001[%v1009] : memref<1xi64>
    return %v1010 : i64
  }
  func.func @sloth_main_Range3__next(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 6 : i64
    %v1010 = arith.constant 0 : i64
    %v1009 = arith.cmpi sge, %v1007, %v1008 : i64
    %v1011 = arith.extui %v1009 : i1 to i64
    %v1012 = arith.constant 1 : i64
    %v1013 = arith.shli %v1011, %v1012 : i64
    %v1015 = arith.constant 0 : i64
    %v1014 = arith.cmpi ne, %v1013, %v1015 : i64
    cf.cond_br %v1014, ^t_1, ^e_2
  ^t_1:
    %v1016 = arith.constant 0 : i64
    %v1017 = call @sloth_rc_retain(%v1016) : (i64) -> i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1001[%v1018] : memref<1xi64>
    %v1019 = arith.constant 1 : i64
    memref.store %v1019, %v1000[%v1018] : memref<1xi64>
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1002[%v1020] : memref<1xi64>
    %v1022 = arith.constant 0 : i64
    %v1023 = call @sloth_obj_field(%v1021, %v1022) : (i64, i64) -> i64
    %v1024 = arith.constant 2 : i64
    %v1025 = arith.constant 1 : i64
    %v1026 = arith.shrsi %v1023, %v1025 : i64
    %v1027 = arith.constant 1 : i64
    %v1028 = arith.shrsi %v1024, %v1027 : i64
    %v1029 = arith.addi %v1026, %v1028 : i64
    %v1030 = arith.constant 1 : i64
    %v1031 = arith.shli %v1029, %v1030 : i64
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1002[%v1032] : memref<1xi64>
    %v1034 = arith.constant 0 : i64
    %v1035 = call @sloth_obj_field(%v1033, %v1034) : (i64, i64) -> i64
    call @sloth_rc_release(%v1035) : (i64) -> i64
    %v1036 = call @sloth_rc_retain(%v1031) : (i64) -> i64
    call @sloth_obj_set_field(%v1033, %v1034, %v1036) : (i64, i64, i64) -> i64
    %v1037 = arith.constant 0 : index
    %v1038 = memref.load %v1002[%v1037] : memref<1xi64>
    %v1039 = arith.constant 0 : i64
    %v1040 = call @sloth_obj_field(%v1038, %v1039) : (i64, i64) -> i64
    %v1041 = arith.constant 2 : i64
    %v1042 = arith.constant 1 : i64
    %v1043 = arith.shrsi %v1040, %v1042 : i64
    %v1044 = arith.constant 1 : i64
    %v1045 = arith.shrsi %v1041, %v1044 : i64
    %v1046 = arith.subi %v1043, %v1045 : i64
    %v1047 = arith.constant 1 : i64
    %v1048 = arith.shli %v1046, %v1047 : i64
    %v1049 = call @sloth_box_new(%v1048) : (i64) -> i64
    %v1050 = arith.constant 0 : index
    memref.store %v1049, %v1001[%v1050] : memref<1xi64>
    %v1051 = arith.constant 1 : i64
    memref.store %v1051, %v1000[%v1050] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1052 = arith.constant 0 : index
    %v1053 = memref.load %v1001[%v1052] : memref<1xi64>
    return %v1053 : i64
  }
}

