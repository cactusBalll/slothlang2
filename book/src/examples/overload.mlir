module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 2 : i64
    %v1002 = arith.constant 4 : i64
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.constant 4 : i64
    %v1005 = call @sloth_cls_info(%v1003, %v1004) : (i64, i64) -> i64
    %v1006 = arith.constant 0 : i64
    %v1007 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1005, %v1006, %v1007) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 4 : i64
    %v1009 = call @sloth_obj_new(%v1005, %v1008) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1009, %v1001, %v1002) : (i64, i64, i64) -> ()
    %v1010 = memref.alloca() : memref<1xi64>
    %v1011 = call @sloth_rc_retain(%v1009) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    memref.store %v1011, %v1010[%v1012] : memref<1xi64>
    call @sloth_rc_release(%v1009) : (i64) -> i64
    %v1013 = arith.constant 6 : i64
    %v1014 = arith.constant 8 : i64
    %v1015 = arith.constant 0 : i64
    %v1016 = arith.constant 4 : i64
    %v1017 = call @sloth_cls_info(%v1015, %v1016) : (i64, i64) -> i64
    %v1018 = arith.constant 0 : i64
    %v1019 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1017, %v1018, %v1019) : (i64, i64, i64) -> i64
    %v1020 = arith.constant 4 : i64
    %v1021 = call @sloth_obj_new(%v1017, %v1020) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1021, %v1013, %v1014) : (i64, i64, i64) -> ()
    %v1022 = memref.alloca() : memref<1xi64>
    %v1023 = call @sloth_rc_retain(%v1021) : (i64) -> i64
    %v1024 = arith.constant 0 : index
    memref.store %v1023, %v1022[%v1024] : memref<1xi64>
    call @sloth_rc_release(%v1021) : (i64) -> i64
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1010[%v1025] : memref<1xi64>
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1022[%v1027] : memref<1xi64>
    %v1029 = call @sloth_main_Vec2____add__(%v1026, %v1028) : (i64, i64) -> i64
    %v1030 = call @sloth_main_Vec2__show(%v1029) : (i64) -> i64
    %v1031 = call @sloth_rt_print_str(%v1030) : (i64) -> i64
    call @sloth_rc_release(%v1029) : (i64) -> i64
    call @sloth_rc_release(%v1030) : (i64) -> i64
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1022[%v1032] : memref<1xi64>
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1010[%v1034] : memref<1xi64>
    %v1036 = call @sloth_main_Vec2____sub__(%v1033, %v1035) : (i64, i64) -> i64
    %v1037 = call @sloth_main_Vec2__show(%v1036) : (i64) -> i64
    %v1038 = call @sloth_rt_print_str(%v1037) : (i64) -> i64
    call @sloth_rc_release(%v1036) : (i64) -> i64
    call @sloth_rc_release(%v1037) : (i64) -> i64
    %v1039 = arith.constant 0 : index
    %v1040 = memref.load %v1010[%v1039] : memref<1xi64>
    %v1042 = call @sloth_main_Vec2____neg__(%v1040) : (i64) -> i64
    %v1043 = call @sloth_main_Vec2__show(%v1042) : (i64) -> i64
    %v1044 = call @sloth_rt_print_str(%v1043) : (i64) -> i64
    call @sloth_rc_release(%v1042) : (i64) -> i64
    call @sloth_rc_release(%v1043) : (i64) -> i64
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1010[%v1045] : memref<1xi64>
    %v1047 = arith.constant 2 : i64
    %v1048 = arith.constant 4 : i64
    %v1049 = arith.constant 0 : i64
    %v1050 = arith.constant 4 : i64
    %v1051 = call @sloth_cls_info(%v1049, %v1050) : (i64, i64) -> i64
    %v1052 = arith.constant 0 : i64
    %v1053 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1051, %v1052, %v1053) : (i64, i64, i64) -> i64
    %v1054 = arith.constant 4 : i64
    %v1055 = call @sloth_obj_new(%v1051, %v1054) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1055, %v1047, %v1048) : (i64, i64, i64) -> ()
    %v1056 = call @sloth_main_Vec2____eq__(%v1046, %v1055) : (i64, i64) -> i64
    %v1057 = call @sloth_rt_print_bool(%v1056) : (i64) -> i64
    call @sloth_rc_release(%v1055) : (i64) -> i64
    %v1058 = arith.constant 0 : index
    %v1059 = memref.load %v1010[%v1058] : memref<1xi64>
    %v1060 = arith.constant 0 : index
    %v1061 = memref.load %v1022[%v1060] : memref<1xi64>
    %v1062 = call @sloth_main_Vec2____lt__(%v1059, %v1061) : (i64, i64) -> i64
    %v1063 = call @sloth_rt_print_bool(%v1062) : (i64) -> i64
    %v1064 = arith.constant 0 : i64
    %v1065 = arith.constant 6 : i64
    %v1066 = call @sloth_cls_info(%v1064, %v1065) : (i64, i64) -> i64
    %v1067 = arith.constant 1 : i64
    %v1068 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1066, %v1067, %v1068) : (i64, i64, i64) -> i64
    %v1069 = arith.constant 2 : i64
    %v1070 = call @sloth_obj_new(%v1066, %v1069) : (i64, i64) -> i64
    call @sloth_main_Bag____init__(%v1070) : (i64) -> ()
    %v1071 = memref.alloca() : memref<1xi64>
    %v1072 = call @sloth_rc_retain(%v1070) : (i64) -> i64
    %v1073 = arith.constant 0 : index
    memref.store %v1072, %v1071[%v1073] : memref<1xi64>
    call @sloth_rc_release(%v1070) : (i64) -> i64
    %v1074 = arith.constant 14 : i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1071[%v1075] : memref<1xi64>
    %v1077 = arith.constant 0 : i64
    call @sloth_main_Bag____assign__(%v1076, %v1077, %v1074) : (i64, i64, i64) -> ()
    %v1078 = arith.constant 0 : i64
    %v1079 = arith.constant 16 : i64
    %v1080 = arith.constant 0 : index
    %v1081 = memref.load %v1071[%v1080] : memref<1xi64>
    %v1082 = arith.constant 2 : i64
    call @sloth_main_Bag____assign__(%v1081, %v1082, %v1079) : (i64, i64, i64) -> ()
    %v1083 = arith.constant 0 : i64
    %v1084 = arith.constant 0 : index
    %v1085 = memref.load %v1071[%v1084] : memref<1xi64>
    %v1086 = arith.constant 0 : i64
    %v1087 = call @sloth_main_Bag____index__(%v1085, %v1086) : (i64, i64) -> i64
    %v1088 = arith.constant 0 : index
    %v1089 = memref.load %v1071[%v1088] : memref<1xi64>
    %v1090 = arith.constant 2 : i64
    %v1091 = call @sloth_main_Bag____index__(%v1089, %v1090) : (i64, i64) -> i64
    %v1092 = arith.constant 1 : i64
    %v1093 = arith.shrsi %v1087, %v1092 : i64
    %v1094 = arith.constant 1 : i64
    %v1095 = arith.shrsi %v1091, %v1094 : i64
    %v1096 = arith.addi %v1093, %v1095 : i64
    %v1097 = arith.constant 1 : i64
    %v1098 = arith.shli %v1096, %v1097 : i64
    %v1099 = call @sloth_rt_print_i64(%v1098) : (i64) -> i64
    %v1100 = arith.constant 0 : index
    %v1101 = memref.load %v1022[%v1100] : memref<1xi64>
    call @sloth_rc_release(%v1101) : (i64) -> i64
    %v1102 = arith.constant 0 : index
    %v1103 = memref.load %v1071[%v1102] : memref<1xi64>
    call @sloth_rc_release(%v1103) : (i64) -> i64
    %v1104 = arith.constant 0 : index
    %v1105 = memref.load %v1010[%v1104] : memref<1xi64>
    call @sloth_rc_release(%v1105) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Vec2____init__(%p0: i64, %p1: i64, %p2: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1003 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1003[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1005 = memref.alloca() : memref<1xi64>
    memref.store %p2, %v1005[%v1006] : memref<1xi64>
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1003[%v1007] : memref<1xi64>
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1001[%v1009] : memref<1xi64>
    %v1011 = arith.constant 0 : i64
    %v1012 = call @sloth_obj_field(%v1010, %v1011) : (i64, i64) -> i64
    call @sloth_rc_release(%v1012) : (i64) -> i64
    %v1013 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    call @sloth_obj_set_field(%v1010, %v1011, %v1013) : (i64, i64, i64) -> i64
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1005[%v1014] : memref<1xi64>
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1001[%v1016] : memref<1xi64>
    %v1018 = arith.constant 2 : i64
    %v1019 = call @sloth_obj_field(%v1017, %v1018) : (i64, i64) -> i64
    call @sloth_rc_release(%v1019) : (i64) -> i64
    %v1020 = call @sloth_rc_retain(%v1015) : (i64) -> i64
    call @sloth_obj_set_field(%v1017, %v1018, %v1020) : (i64, i64, i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Vec2____add__(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : i64
    %v1009 = call @sloth_obj_field(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1004[%v1010] : memref<1xi64>
    %v1012 = arith.constant 0 : i64
    %v1013 = call @sloth_obj_field(%v1011, %v1012) : (i64, i64) -> i64
    %v1014 = arith.constant 1 : i64
    %v1015 = arith.shrsi %v1009, %v1014 : i64
    %v1016 = arith.constant 1 : i64
    %v1017 = arith.shrsi %v1013, %v1016 : i64
    %v1018 = arith.addi %v1015, %v1017 : i64
    %v1019 = arith.constant 1 : i64
    %v1020 = arith.shli %v1018, %v1019 : i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1002[%v1021] : memref<1xi64>
    %v1023 = arith.constant 2 : i64
    %v1024 = call @sloth_obj_field(%v1022, %v1023) : (i64, i64) -> i64
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1004[%v1025] : memref<1xi64>
    %v1027 = arith.constant 2 : i64
    %v1028 = call @sloth_obj_field(%v1026, %v1027) : (i64, i64) -> i64
    %v1029 = arith.constant 1 : i64
    %v1030 = arith.shrsi %v1024, %v1029 : i64
    %v1031 = arith.constant 1 : i64
    %v1032 = arith.shrsi %v1028, %v1031 : i64
    %v1033 = arith.addi %v1030, %v1032 : i64
    %v1034 = arith.constant 1 : i64
    %v1035 = arith.shli %v1033, %v1034 : i64
    %v1036 = arith.constant 0 : i64
    %v1037 = arith.constant 4 : i64
    %v1038 = call @sloth_cls_info(%v1036, %v1037) : (i64, i64) -> i64
    %v1039 = arith.constant 0 : i64
    %v1040 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1038, %v1039, %v1040) : (i64, i64, i64) -> i64
    %v1041 = arith.constant 4 : i64
    %v1042 = call @sloth_obj_new(%v1038, %v1041) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1042, %v1020, %v1035) : (i64, i64, i64) -> ()
    %v1043 = arith.constant 0 : index
    memref.store %v1042, %v1001[%v1043] : memref<1xi64>
    %v1044 = arith.constant 1 : i64
    memref.store %v1044, %v1000[%v1043] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1001[%v1045] : memref<1xi64>
    return %v1046 : i64
  }
  func.func @sloth_main_Vec2____sub__(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : i64
    %v1009 = call @sloth_obj_field(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1004[%v1010] : memref<1xi64>
    %v1012 = arith.constant 0 : i64
    %v1013 = call @sloth_obj_field(%v1011, %v1012) : (i64, i64) -> i64
    %v1014 = arith.constant 1 : i64
    %v1015 = arith.shrsi %v1009, %v1014 : i64
    %v1016 = arith.constant 1 : i64
    %v1017 = arith.shrsi %v1013, %v1016 : i64
    %v1018 = arith.subi %v1015, %v1017 : i64
    %v1019 = arith.constant 1 : i64
    %v1020 = arith.shli %v1018, %v1019 : i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1002[%v1021] : memref<1xi64>
    %v1023 = arith.constant 2 : i64
    %v1024 = call @sloth_obj_field(%v1022, %v1023) : (i64, i64) -> i64
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1004[%v1025] : memref<1xi64>
    %v1027 = arith.constant 2 : i64
    %v1028 = call @sloth_obj_field(%v1026, %v1027) : (i64, i64) -> i64
    %v1029 = arith.constant 1 : i64
    %v1030 = arith.shrsi %v1024, %v1029 : i64
    %v1031 = arith.constant 1 : i64
    %v1032 = arith.shrsi %v1028, %v1031 : i64
    %v1033 = arith.subi %v1030, %v1032 : i64
    %v1034 = arith.constant 1 : i64
    %v1035 = arith.shli %v1033, %v1034 : i64
    %v1036 = arith.constant 0 : i64
    %v1037 = arith.constant 4 : i64
    %v1038 = call @sloth_cls_info(%v1036, %v1037) : (i64, i64) -> i64
    %v1039 = arith.constant 0 : i64
    %v1040 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1038, %v1039, %v1040) : (i64, i64, i64) -> i64
    %v1041 = arith.constant 4 : i64
    %v1042 = call @sloth_obj_new(%v1038, %v1041) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1042, %v1020, %v1035) : (i64, i64, i64) -> ()
    %v1043 = arith.constant 0 : index
    memref.store %v1042, %v1001[%v1043] : memref<1xi64>
    %v1044 = arith.constant 1 : i64
    memref.store %v1044, %v1000[%v1043] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1001[%v1045] : memref<1xi64>
    return %v1046 : i64
  }
  func.func @sloth_main_Vec2____neg__(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1002[%v1005] : memref<1xi64>
    %v1007 = arith.constant 0 : i64
    %v1008 = call @sloth_obj_field(%v1006, %v1007) : (i64, i64) -> i64
    %v1009 = arith.constant 1 : i64
    %v1010 = arith.shrsi %v1004, %v1009 : i64
    %v1011 = arith.constant 1 : i64
    %v1012 = arith.shrsi %v1008, %v1011 : i64
    %v1013 = arith.subi %v1010, %v1012 : i64
    %v1014 = arith.constant 1 : i64
    %v1015 = arith.shli %v1013, %v1014 : i64
    %v1016 = arith.constant 0 : i64
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1002[%v1017] : memref<1xi64>
    %v1019 = arith.constant 2 : i64
    %v1020 = call @sloth_obj_field(%v1018, %v1019) : (i64, i64) -> i64
    %v1021 = arith.constant 1 : i64
    %v1022 = arith.shrsi %v1016, %v1021 : i64
    %v1023 = arith.constant 1 : i64
    %v1024 = arith.shrsi %v1020, %v1023 : i64
    %v1025 = arith.subi %v1022, %v1024 : i64
    %v1026 = arith.constant 1 : i64
    %v1027 = arith.shli %v1025, %v1026 : i64
    %v1028 = arith.constant 0 : i64
    %v1029 = arith.constant 4 : i64
    %v1030 = call @sloth_cls_info(%v1028, %v1029) : (i64, i64) -> i64
    %v1031 = arith.constant 0 : i64
    %v1032 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1030, %v1031, %v1032) : (i64, i64, i64) -> i64
    %v1033 = arith.constant 4 : i64
    %v1034 = call @sloth_obj_new(%v1030, %v1033) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1034, %v1015, %v1027) : (i64, i64, i64) -> ()
    %v1035 = arith.constant 0 : index
    memref.store %v1034, %v1001[%v1035] : memref<1xi64>
    %v1036 = arith.constant 1 : i64
    memref.store %v1036, %v1000[%v1035] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1037 = arith.constant 0 : index
    %v1038 = memref.load %v1001[%v1037] : memref<1xi64>
    return %v1038 : i64
  }
  func.func @sloth_main_Vec2____eq__(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : i64
    %v1009 = call @sloth_obj_field(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1004[%v1010] : memref<1xi64>
    %v1012 = arith.constant 0 : i64
    %v1013 = call @sloth_obj_field(%v1011, %v1012) : (i64, i64) -> i64
    %v1015 = arith.constant 0 : i64
    %v1014 = arith.cmpi eq, %v1009, %v1013 : i64
    %v1016 = arith.extui %v1014 : i1 to i64
    %v1017 = arith.constant 1 : i64
    %v1018 = arith.shli %v1016, %v1017 : i64
    %v1019 = memref.alloca() : memref<1xi64>
    %v1020 = arith.constant 0 : index
    memref.store %v1018, %v1019[%v1020] : memref<1xi64>
    %v1021 = arith.constant 1 : i64
    %v1022 = arith.shrsi %v1018, %v1021 : i64
    %v1023 = arith.constant 0 : i64
    %v1024 = arith.cmpi ne, %v1022, %v1023 : i64
    %v1025 = arith.extui %v1024 : i1 to i64
    %v1027 = arith.constant 0 : i64
    %v1026 = arith.cmpi ne, %v1025, %v1027 : i64
    cf.cond_br %v1026, ^sc_1, ^sc_2
  ^sc_1:
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1002[%v1028] : memref<1xi64>
    %v1030 = arith.constant 2 : i64
    %v1031 = call @sloth_obj_field(%v1029, %v1030) : (i64, i64) -> i64
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1004[%v1032] : memref<1xi64>
    %v1034 = arith.constant 2 : i64
    %v1035 = call @sloth_obj_field(%v1033, %v1034) : (i64, i64) -> i64
    %v1037 = arith.constant 0 : i64
    %v1036 = arith.cmpi eq, %v1031, %v1035 : i64
    %v1038 = arith.extui %v1036 : i1 to i64
    %v1039 = arith.constant 1 : i64
    %v1040 = arith.shli %v1038, %v1039 : i64
    %v1041 = arith.constant 0 : index
    memref.store %v1040, %v1019[%v1041] : memref<1xi64>
    cf.br ^sc_2
  ^sc_2:
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1019[%v1042] : memref<1xi64>
    %v1044 = arith.constant 0 : index
    memref.store %v1043, %v1001[%v1044] : memref<1xi64>
    %v1045 = arith.constant 1 : i64
    memref.store %v1045, %v1000[%v1044] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1046 = arith.constant 0 : index
    %v1047 = memref.load %v1001[%v1046] : memref<1xi64>
    return %v1047 : i64
  }
  func.func @sloth_main_Vec2____lt__(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : i64
    %v1009 = call @sloth_obj_field(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1004[%v1010] : memref<1xi64>
    %v1012 = arith.constant 0 : i64
    %v1013 = call @sloth_obj_field(%v1011, %v1012) : (i64, i64) -> i64
    %v1015 = arith.constant 0 : i64
    %v1014 = arith.cmpi slt, %v1009, %v1013 : i64
    %v1016 = arith.extui %v1014 : i1 to i64
    %v1017 = arith.constant 1 : i64
    %v1018 = arith.shli %v1016, %v1017 : i64
    %v1019 = arith.constant 0 : index
    memref.store %v1018, %v1001[%v1019] : memref<1xi64>
    %v1020 = arith.constant 1 : i64
    memref.store %v1020, %v1000[%v1019] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1001[%v1021] : memref<1xi64>
    return %v1022 : i64
  }
  func.func @sloth_main_Vec2__show(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 40 : i64
    %v1006 = arith.constant 2 : i64
    %v1007 = call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = arith.constant 0 : i64
    %v1011 = call @sloth_obj_field(%v1009, %v1010) : (i64, i64) -> i64
    %v1012 = call @sloth_str_push_i(%v1007, %v1011) : (i64, i64) -> i64
    %v1013 = arith.constant 8236 : i64
    %v1014 = arith.constant 4 : i64
    %v1015 = call @sloth_str_push(%v1012, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1002[%v1016] : memref<1xi64>
    %v1018 = arith.constant 2 : i64
    %v1019 = call @sloth_obj_field(%v1017, %v1018) : (i64, i64) -> i64
    %v1020 = call @sloth_str_push_i(%v1015, %v1019) : (i64, i64) -> i64
    %v1021 = arith.constant 41 : i64
    %v1022 = arith.constant 2 : i64
    %v1023 = call @sloth_str_push(%v1020, %v1021, %v1022) : (i64, i64, i64) -> i64
    %v1024 = call @sloth_str_finish(%v1023) : (i64) -> i64
    %v1025 = arith.constant 0 : index
    memref.store %v1024, %v1001[%v1025] : memref<1xi64>
    %v1026 = arith.constant 1 : i64
    memref.store %v1026, %v1000[%v1025] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1001[%v1027] : memref<1xi64>
    return %v1028 : i64
  }
  func.func @sloth_main_Bag____init__(%p0: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 0 : i64
    %v1006 = arith.constant 6 : i64
    %v1007 = call @sloth_arr_new(%v1006) : (i64) -> i64
    %v1008 = arith.constant 0 : i64
    call @sloth_arr_set(%v1007, %v1008, %v1003) : (i64, i64, i64) -> i64
    %v1009 = arith.constant 2 : i64
    call @sloth_arr_set(%v1007, %v1009, %v1004) : (i64, i64, i64) -> i64
    %v1010 = arith.constant 4 : i64
    call @sloth_arr_set(%v1007, %v1010, %v1005) : (i64, i64, i64) -> i64
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    %v1013 = arith.constant 0 : i64
    %v1014 = call @sloth_obj_field(%v1012, %v1013) : (i64, i64) -> i64
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1015 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    call @sloth_obj_set_field(%v1012, %v1013, %v1015) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1007) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Bag____index__(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : i64
    %v1009 = call @sloth_obj_field(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1004[%v1010] : memref<1xi64>
    %v1012 = call @sloth_arr_get(%v1009, %v1011) : (i64, i64) -> i64
    %v1013 = arith.constant 0 : index
    memref.store %v1012, %v1001[%v1013] : memref<1xi64>
    %v1014 = arith.constant 1 : i64
    memref.store %v1014, %v1000[%v1013] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1001[%v1015] : memref<1xi64>
    return %v1016 : i64
  }
  func.func @sloth_main_Bag____assign__(%p0: i64, %p1: i64, %p2: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1003 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1003[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1005 = memref.alloca() : memref<1xi64>
    memref.store %p2, %v1005[%v1006] : memref<1xi64>
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1005[%v1007] : memref<1xi64>
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1001[%v1009] : memref<1xi64>
    %v1011 = arith.constant 0 : i64
    %v1012 = call @sloth_obj_field(%v1010, %v1011) : (i64, i64) -> i64
    %v1013 = arith.constant 0 : index
    %v1014 = memref.load %v1003[%v1013] : memref<1xi64>
    call @sloth_arr_set(%v1012, %v1014, %v1008) : (i64, i64, i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

