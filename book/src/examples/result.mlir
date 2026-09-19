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
    %v1011 = arith.constant 0 : i64
    %v1010 = arith.cmpi ne, %v1009, %v1011 : i64
    cf.cond_br %v1010, ^t_1, ^e_2
  ^t_1:
    %v1012 = arith.constant 0 : i64
    %v1013 = arith.constant 6776174 : i64
    %v1014 = arith.constant 3 : i64
    %v1015 = call @sloth_str_push(%v1012, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1016 = call @sloth_str_finish(%v1015) : (i64) -> i64
    %v1017 = arith.constant 0 : i64
    %v1018 = arith.constant 2 : i64
    %v1019 = call @sloth_cls_info(%v1017, %v1018) : (i64, i64) -> i64
    %v1020 = arith.constant 4 : i64
    %v1021 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1019, %v1020, %v1021) : (i64, i64, i64) -> i64
    %v1022 = arith.constant 3 : i64
    %v1023 = call @sloth_obj_new(%v1019, %v1022) : (i64, i64) -> i64
    %v1024 = arith.constant 0 : i64
    %v1025 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1023, %v1025, %v1024) : (i64, i64, i64) -> i64
    %v1026 = arith.constant 0 : i64
    %v1028 = arith.constant 0 : i64
    %v1029 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1023, %v1029, %v1028) : (i64, i64, i64) -> i64
    %v1030 = arith.constant 0 : i64
    %v1031 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1023, %v1031, %v1030) : (i64, i64, i64) -> i64
    %v1032 = arith.constant 2 : i64
    %v1033 = call @sloth_obj_field(%v1023, %v1032) : (i64, i64) -> i64
    call @sloth_rc_release(%v1033) : (i64) -> i64
    %v1034 = call @sloth_rc_retain(%v1016) : (i64) -> i64
    call @sloth_obj_set_field(%v1023, %v1032, %v1034) : (i64, i64, i64) -> i64
    %v1035 = arith.constant 0 : index
    memref.store %v1023, %v1001[%v1035] : memref<1xi64>
    %v1036 = arith.constant 1 : i64
    memref.store %v1036, %v1000[%v1035] : memref<1xi64>
    call @sloth_rc_release(%v1016) : (i64) -> i64
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1037 = arith.constant 0 : index
    %v1038 = memref.load %v1002[%v1037] : memref<1xi64>
    %v1039 = arith.constant 2 : i64
    %v1040 = arith.muli %v1038, %v1039 : i64
    %v1041 = arith.constant 0 : i64
    %v1042 = arith.constant 2 : i64
    %v1043 = call @sloth_cls_info(%v1041, %v1042) : (i64, i64) -> i64
    %v1044 = arith.constant 4 : i64
    %v1045 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1043, %v1044, %v1045) : (i64, i64, i64) -> i64
    %v1046 = arith.constant 3 : i64
    %v1047 = call @sloth_obj_new(%v1043, %v1046) : (i64, i64) -> i64
    %v1048 = arith.constant 0 : i64
    %v1049 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1047, %v1049, %v1048) : (i64, i64, i64) -> i64
    %v1050 = arith.constant 0 : i64
    %v1052 = arith.constant 1 : i64
    %v1053 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1047, %v1053, %v1052) : (i64, i64, i64) -> i64
    %v1054 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1047, %v1054, %v1040) : (i64, i64, i64) -> i64
    %v1055 = arith.constant 0 : i64
    %v1056 = arith.constant 2 : i64
    %v1057 = call @sloth_obj_field(%v1047, %v1056) : (i64, i64) -> i64
    call @sloth_rc_release(%v1057) : (i64) -> i64
    %v1058 = call @sloth_rc_retain(%v1055) : (i64) -> i64
    call @sloth_obj_set_field(%v1047, %v1056, %v1058) : (i64, i64, i64) -> i64
    %v1059 = arith.constant 0 : index
    memref.store %v1047, %v1001[%v1059] : memref<1xi64>
    %v1060 = arith.constant 1 : i64
    memref.store %v1060, %v1000[%v1059] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1061 = arith.constant 0 : index
    %v1062 = memref.load %v1001[%v1061] : memref<1xi64>
    return %v1062 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 5 : i64
    %v1002 = call @sloth_main__parse(%v1001) : (i64) -> i64
    %v1003 = memref.alloca() : memref<1xi64>
    %v1004 = arith.constant 0 : index
    memref.store %v1002, %v1003[%v1004] : memref<1xi64>
    %v1005 = memref.extract_aligned_pointer_as_index %v1003 : memref<1xi64> -> index
    %v1006 = arith.index_cast %v1005 : index to i64
    call @sloth_fiber_track(%v1006) : (i64) -> i64
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1003[%v1007] : memref<1xi64>
    %v1009 = call @sloth_main_Result_int_str__is_ok(%v1008) : (i64) -> i64
    %v1010 = call @sloth_rt_print_bool(%v1009) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1003[%v1011] : memref<1xi64>
    %v1013 = call @sloth_main_Result_int_str__unwrap(%v1012) : (i64) -> i64
    %v1014 = call @sloth_rt_print_i64(%v1013) : (i64) -> i64
    %v1015 = arith.constant 1 : i64
    %v1018 = arith.constant 0 : i64
    %v1019 = arith.subi %v1018, %v1015 : i64
    %v1020 = call @sloth_main__parse(%v1019) : (i64) -> i64
    %v1021 = memref.alloca() : memref<1xi64>
    %v1022 = arith.constant 0 : index
    memref.store %v1020, %v1021[%v1022] : memref<1xi64>
    %v1023 = memref.extract_aligned_pointer_as_index %v1021 : memref<1xi64> -> index
    %v1024 = arith.index_cast %v1023 : index to i64
    call @sloth_fiber_track(%v1024) : (i64) -> i64
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1021[%v1025] : memref<1xi64>
    %v1027 = call @sloth_main_Result_int_str__is_ok(%v1026) : (i64) -> i64
    %v1028 = call @sloth_rt_print_bool(%v1027) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1021[%v1029] : memref<1xi64>
    %v1031 = call @sloth_main_Result_int_str__err(%v1030) : (i64) -> i64
    %v1032 = call @sloth_rt_print_str(%v1031) : (i64) -> i64
    call @sloth_rc_release(%v1031) : (i64) -> i64
    %v1033 = arith.constant 4612811918334230528 : i64
    %v1034 = arith.constant 0 : i64
    %v1035 = arith.constant 3 : i64
    %v1036 = call @sloth_cls_info(%v1034, %v1035) : (i64, i64) -> i64
    %v1037 = arith.constant 0 : i64
    %v1038 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1036, %v1037, %v1038) : (i64, i64, i64) -> i64
    %v1039 = arith.constant 3 : i64
    %v1040 = call @sloth_obj_new(%v1036, %v1039) : (i64, i64) -> i64
    %v1041 = arith.constant 0 : i64
    %v1042 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1040, %v1042, %v1041) : (i64, i64, i64) -> i64
    %v1043 = arith.constant 0 : i64
    %v1045 = arith.constant 1 : i64
    %v1046 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1040, %v1046, %v1045) : (i64, i64, i64) -> i64
    %v1047 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1040, %v1047, %v1033) : (i64, i64, i64) -> i64
    %v1048 = arith.constant 0 : i64
    %v1049 = arith.constant 2 : i64
    call @sloth_obj_set_field(%v1040, %v1049, %v1048) : (i64, i64, i64) -> i64
    %v1050 = memref.alloca() : memref<1xi64>
    %v1051 = call @sloth_rc_retain(%v1040) : (i64) -> i64
    %v1052 = arith.constant 0 : index
    memref.store %v1051, %v1050[%v1052] : memref<1xi64>
    %v1053 = memref.extract_aligned_pointer_as_index %v1050 : memref<1xi64> -> index
    %v1054 = arith.index_cast %v1053 : index to i64
    call @sloth_fiber_track(%v1054) : (i64) -> i64
    call @sloth_rc_release(%v1040) : (i64) -> i64
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1050[%v1055] : memref<1xi64>
    %v1057 = call @sloth_main_Result_float_int__unwrap(%v1056) : (i64) -> i64
    %v1059 = llvm.bitcast %v1057 : i64 to f64
    %v1058 = call @sloth_rt_print_f64(%v1059) : (f64) -> i64
    %v1060 = arith.constant 0 : i64
    %v1061 = arith.constant 1684366707 : i64
    %v1062 = arith.constant 4 : i64
    %v1063 = call @sloth_str_push(%v1060, %v1061, %v1062) : (i64, i64, i64) -> i64
    %v1064 = call @sloth_str_finish(%v1063) : (i64) -> i64
    %v1065 = arith.constant 0 : i64
    %v1066 = arith.constant 2 : i64
    %v1067 = call @sloth_cls_info(%v1065, %v1066) : (i64, i64) -> i64
    %v1068 = arith.constant 4 : i64
    %v1069 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1067, %v1068, %v1069) : (i64, i64, i64) -> i64
    %v1070 = arith.constant 3 : i64
    %v1071 = call @sloth_obj_new(%v1067, %v1070) : (i64, i64) -> i64
    %v1072 = arith.constant 0 : i64
    %v1073 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1071, %v1073, %v1072) : (i64, i64, i64) -> i64
    %v1074 = arith.constant 0 : i64
    %v1076 = arith.constant 0 : i64
    %v1077 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1071, %v1077, %v1076) : (i64, i64, i64) -> i64
    %v1078 = arith.constant 0 : i64
    %v1079 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1071, %v1079, %v1078) : (i64, i64, i64) -> i64
    %v1080 = arith.constant 2 : i64
    %v1081 = call @sloth_obj_field(%v1071, %v1080) : (i64, i64) -> i64
    call @sloth_rc_release(%v1081) : (i64) -> i64
    %v1082 = call @sloth_rc_retain(%v1064) : (i64) -> i64
    call @sloth_obj_set_field(%v1071, %v1080, %v1082) : (i64, i64, i64) -> i64
    %v1083 = memref.alloca() : memref<1xi64>
    %v1084 = call @sloth_rc_retain(%v1071) : (i64) -> i64
    %v1085 = arith.constant 0 : index
    memref.store %v1084, %v1083[%v1085] : memref<1xi64>
    %v1086 = memref.extract_aligned_pointer_as_index %v1083 : memref<1xi64> -> index
    %v1087 = arith.index_cast %v1086 : index to i64
    call @sloth_fiber_track(%v1087) : (i64) -> i64
    call @sloth_rc_release(%v1064) : (i64) -> i64
    call @sloth_rc_release(%v1071) : (i64) -> i64
    %v1088 = arith.constant 8 : i64
    %v1089 = arith.constant 0 : i64
    %v1090 = arith.constant 2 : i64
    %v1091 = call @sloth_cls_info(%v1089, %v1090) : (i64, i64) -> i64
    %v1092 = arith.constant 4 : i64
    %v1093 = arith.constant 3 : i64
    call @sloth_cls_refmask(%v1091, %v1092, %v1093) : (i64, i64, i64) -> i64
    %v1094 = arith.constant 3 : i64
    %v1095 = call @sloth_obj_new(%v1091, %v1094) : (i64, i64) -> i64
    %v1096 = arith.constant 0 : i64
    %v1097 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1095, %v1097, %v1096) : (i64, i64, i64) -> i64
    %v1098 = arith.constant 0 : i64
    %v1100 = arith.constant 1 : i64
    %v1101 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1095, %v1101, %v1100) : (i64, i64, i64) -> i64
    %v1102 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1095, %v1102, %v1088) : (i64, i64, i64) -> i64
    %v1103 = arith.constant 0 : i64
    %v1104 = arith.constant 2 : i64
    %v1105 = call @sloth_obj_field(%v1095, %v1104) : (i64, i64) -> i64
    call @sloth_rc_release(%v1105) : (i64) -> i64
    %v1106 = call @sloth_rc_retain(%v1103) : (i64) -> i64
    call @sloth_obj_set_field(%v1095, %v1104, %v1106) : (i64, i64, i64) -> i64
    %v1107 = arith.constant 0 : index
    %v1108 = memref.load %v1083[%v1107] : memref<1xi64>
    call @sloth_rc_release(%v1108) : (i64) -> i64
    %v1109 = call @sloth_rc_retain(%v1095) : (i64) -> i64
    %v1110 = arith.constant 0 : index
    memref.store %v1109, %v1083[%v1110] : memref<1xi64>
    call @sloth_rc_release(%v1095) : (i64) -> i64
    %v1111 = arith.constant 0 : index
    %v1112 = memref.load %v1083[%v1111] : memref<1xi64>
    %v1113 = call @sloth_main_Result_int_str__unwrap(%v1112) : (i64) -> i64
    %v1114 = call @sloth_rt_print_i64(%v1113) : (i64) -> i64
    %v1115 = arith.constant 0 : index
    %v1116 = memref.load %v1083[%v1115] : memref<1xi64>
    call @sloth_rc_release(%v1116) : (i64) -> i64
    %v1117 = memref.extract_aligned_pointer_as_index %v1083 : memref<1xi64> -> index
    %v1118 = arith.index_cast %v1117 : index to i64
    call @sloth_fiber_untrack(%v1118) : (i64) -> i64
    %v1119 = arith.constant 0 : index
    %v1120 = memref.load %v1021[%v1119] : memref<1xi64>
    call @sloth_rc_release(%v1120) : (i64) -> i64
    %v1121 = memref.extract_aligned_pointer_as_index %v1021 : memref<1xi64> -> index
    %v1122 = arith.index_cast %v1121 : index to i64
    call @sloth_fiber_untrack(%v1122) : (i64) -> i64
    %v1123 = arith.constant 0 : index
    %v1124 = memref.load %v1050[%v1123] : memref<1xi64>
    call @sloth_rc_release(%v1124) : (i64) -> i64
    %v1125 = memref.extract_aligned_pointer_as_index %v1050 : memref<1xi64> -> index
    %v1126 = arith.index_cast %v1125 : index to i64
    call @sloth_fiber_untrack(%v1126) : (i64) -> i64
    %v1127 = arith.constant 0 : index
    %v1128 = memref.load %v1003[%v1127] : memref<1xi64>
    call @sloth_rc_release(%v1128) : (i64) -> i64
    %v1129 = memref.extract_aligned_pointer_as_index %v1003 : memref<1xi64> -> index
    %v1130 = arith.index_cast %v1129 : index to i64
    call @sloth_fiber_untrack(%v1130) : (i64) -> i64
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
    %v1016 = arith.constant 1 : i64
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
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1002[%v1022] : memref<1xi64>
    %v1024 = arith.constant 1 : i64
    %v1025 = call @sloth_obj_field(%v1023, %v1024) : (i64, i64) -> i64
    %v1026 = arith.constant 0 : index
    memref.store %v1025, %v1001[%v1026] : memref<1xi64>
    %v1027 = arith.constant 1 : i64
    memref.store %v1027, %v1000[%v1026] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1001[%v1028] : memref<1xi64>
    return %v1029 : i64
  }
  func.func @sloth_main_Result_int_str__err(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 2 : i64
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
    %v1016 = arith.constant 1 : i64
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
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1002[%v1022] : memref<1xi64>
    %v1024 = arith.constant 1 : i64
    %v1025 = call @sloth_obj_field(%v1023, %v1024) : (i64, i64) -> i64
    %v1026 = arith.constant 0 : index
    memref.store %v1025, %v1001[%v1026] : memref<1xi64>
    %v1027 = arith.constant 1 : i64
    memref.store %v1027, %v1000[%v1026] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1001[%v1028] : memref<1xi64>
    return %v1029 : i64
  }
  func.func @sloth_main_Result_float_int__err(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 2 : i64
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

