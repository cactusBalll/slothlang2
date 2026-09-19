module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__first(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_arr_get(%v1005, %v1006) : (i64, i64) -> i64
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
  func.func @sloth_main__twice(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 2 : i64
    %v1010 = arith.constant 1 : i64
    %v1009 = call @sloth_arr_new_k(%v1008, %v1010) : (i64, i64) -> i64
    %v1011 = arith.constant 0 : i64
    %v1012 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    call @sloth_arr_set(%v1009, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 1 : i64
    %v1014 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    call @sloth_arr_set(%v1009, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1015 = arith.constant 0 : index
    memref.store %v1009, %v1001[%v1015] : memref<1xi64>
    %v1016 = arith.constant 1 : i64
    memref.store %v1016, %v1000[%v1015] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1001[%v1017] : memref<1xi64>
    return %v1018 : i64
  }
  func.func @sloth_main__first_int(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_arr_get(%v1005, %v1006) : (i64, i64) -> i64
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
  func.func @sloth_main__first_str(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_arr_get(%v1005, %v1006) : (i64, i64) -> i64
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
  func.func @sloth_main__twice_str(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 2 : i64
    %v1010 = arith.constant 1 : i64
    %v1009 = call @sloth_arr_new_k(%v1008, %v1010) : (i64, i64) -> i64
    %v1011 = arith.constant 0 : i64
    %v1012 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    call @sloth_arr_set(%v1009, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 1 : i64
    %v1014 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    call @sloth_arr_set(%v1009, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1015 = arith.constant 0 : index
    memref.store %v1009, %v1001[%v1015] : memref<1xi64>
    %v1016 = arith.constant 1 : i64
    memref.store %v1016, %v1000[%v1015] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1001[%v1017] : memref<1xi64>
    return %v1018 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 10 : i64
    %v1002 = arith.constant 20 : i64
    %v1003 = arith.constant 2 : i64
    %v1004 = call @sloth_arr_new(%v1003) : (i64) -> i64
    %v1005 = arith.constant 0 : i64
    call @sloth_arr_set(%v1004, %v1005, %v1001) : (i64, i64, i64) -> i64
    %v1006 = arith.constant 1 : i64
    call @sloth_arr_set(%v1004, %v1006, %v1002) : (i64, i64, i64) -> i64
    %v1007 = memref.alloca() : memref<1xi64>
    %v1008 = call @sloth_rc_retain(%v1004) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1007[%v1009] : memref<1xi64>
    %v1010 = memref.extract_aligned_pointer_as_index %v1007 : memref<1xi64> -> index
    %v1011 = arith.index_cast %v1010 : index to i64
    call @sloth_fiber_track(%v1011) : (i64) -> i64
    call @sloth_rc_release(%v1004) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1007[%v1012] : memref<1xi64>
    %v1014 = call @sloth_main__first_int(%v1013) : (i64) -> i64
    %v1015 = call @sloth_rt_print_i64(%v1014) : (i64) -> i64
    %v1016 = arith.constant 0 : i64
    %v1017 = arith.constant 120 : i64
    %v1018 = arith.constant 1 : i64
    %v1019 = call @sloth_str_push(%v1016, %v1017, %v1018) : (i64, i64, i64) -> i64
    %v1020 = call @sloth_str_finish(%v1019) : (i64) -> i64
    %v1021 = arith.constant 0 : i64
    %v1022 = arith.constant 121 : i64
    %v1023 = arith.constant 1 : i64
    %v1024 = call @sloth_str_push(%v1021, %v1022, %v1023) : (i64, i64, i64) -> i64
    %v1025 = call @sloth_str_finish(%v1024) : (i64) -> i64
    %v1026 = arith.constant 2 : i64
    %v1028 = arith.constant 1 : i64
    %v1027 = call @sloth_arr_new_k(%v1026, %v1028) : (i64, i64) -> i64
    %v1029 = arith.constant 0 : i64
    %v1030 = call @sloth_rc_retain(%v1020) : (i64) -> i64
    call @sloth_arr_set(%v1027, %v1029, %v1030) : (i64, i64, i64) -> i64
    %v1031 = arith.constant 1 : i64
    %v1032 = call @sloth_rc_retain(%v1025) : (i64) -> i64
    call @sloth_arr_set(%v1027, %v1031, %v1032) : (i64, i64, i64) -> i64
    %v1033 = memref.alloca() : memref<1xi64>
    %v1034 = call @sloth_rc_retain(%v1027) : (i64) -> i64
    %v1035 = arith.constant 0 : index
    memref.store %v1034, %v1033[%v1035] : memref<1xi64>
    %v1036 = memref.extract_aligned_pointer_as_index %v1033 : memref<1xi64> -> index
    %v1037 = arith.index_cast %v1036 : index to i64
    call @sloth_fiber_track(%v1037) : (i64) -> i64
    call @sloth_rc_release(%v1020) : (i64) -> i64
    call @sloth_rc_release(%v1025) : (i64) -> i64
    call @sloth_rc_release(%v1027) : (i64) -> i64
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1033[%v1038] : memref<1xi64>
    %v1040 = call @sloth_main__first_str(%v1039) : (i64) -> i64
    %v1041 = call @sloth_rt_print_str(%v1040) : (i64) -> i64
    call @sloth_rc_release(%v1040) : (i64) -> i64
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1007[%v1042] : memref<1xi64>
    %v1044 = call @sloth_main__first_int(%v1043) : (i64) -> i64
    %v1045 = call @sloth_rt_print_i64(%v1044) : (i64) -> i64
    %v1046 = arith.constant 0 : i64
    %v1047 = arith.constant 122 : i64
    %v1048 = arith.constant 1 : i64
    %v1049 = call @sloth_str_push(%v1046, %v1047, %v1048) : (i64, i64, i64) -> i64
    %v1050 = call @sloth_str_finish(%v1049) : (i64) -> i64
    %v1051 = call @sloth_main__twice_str(%v1050) : (i64) -> i64
    %v1052 = memref.alloca() : memref<1xi64>
    %v1053 = arith.constant 0 : index
    memref.store %v1051, %v1052[%v1053] : memref<1xi64>
    %v1054 = memref.extract_aligned_pointer_as_index %v1052 : memref<1xi64> -> index
    %v1055 = arith.index_cast %v1054 : index to i64
    call @sloth_fiber_track(%v1055) : (i64) -> i64
    call @sloth_rc_release(%v1050) : (i64) -> i64
    %v1056 = arith.constant 0 : index
    %v1057 = memref.load %v1052[%v1056] : memref<1xi64>
    %v1058 = arith.constant 0 : i64
    %v1059 = call @sloth_arr_get(%v1057, %v1058) : (i64, i64) -> i64
    %v1060 = arith.constant 0 : index
    %v1061 = memref.load %v1052[%v1060] : memref<1xi64>
    %v1062 = arith.constant 1 : i64
    %v1063 = call @sloth_arr_get(%v1061, %v1062) : (i64, i64) -> i64
    %v1064 = call @sloth_str_concat(%v1059, %v1063) : (i64, i64) -> i64
    %v1065 = call @sloth_rt_print_str(%v1064) : (i64) -> i64
    call @sloth_rc_release(%v1064) : (i64) -> i64
    %v1066 = arith.constant 0 : i64
    %v1067 = arith.constant 3 : i64
    %v1068 = call @sloth_cls_info(%v1066, %v1067) : (i64, i64) -> i64
    %v1069 = arith.constant 0 : i64
    %v1070 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1068, %v1069, %v1070) : (i64, i64, i64) -> i64
    %v1071 = arith.constant 1 : i64
    %v1072 = call @sloth_obj_new(%v1068, %v1071) : (i64, i64) -> i64
    %v1073 = memref.alloca() : memref<1xi64>
    %v1074 = call @sloth_rc_retain(%v1072) : (i64) -> i64
    %v1075 = arith.constant 0 : index
    memref.store %v1074, %v1073[%v1075] : memref<1xi64>
    %v1076 = memref.extract_aligned_pointer_as_index %v1073 : memref<1xi64> -> index
    %v1077 = arith.index_cast %v1076 : index to i64
    call @sloth_fiber_track(%v1077) : (i64) -> i64
    call @sloth_rc_release(%v1072) : (i64) -> i64
    %v1078 = arith.constant 0 : index
    %v1079 = memref.load %v1073[%v1078] : memref<1xi64>
    %v1080 = arith.constant 7 : i64
    call @sloth_main_Box_int__set(%v1079, %v1080) : (i64, i64) -> ()
    %v1081 = arith.constant 0 : i64
    %v1082 = arith.constant 0 : index
    %v1083 = memref.load %v1073[%v1082] : memref<1xi64>
    %v1084 = call @sloth_main_Box_int__get(%v1083) : (i64) -> i64
    %v1085 = call @sloth_rt_print_i64(%v1084) : (i64) -> i64
    %v1086 = arith.constant 0 : i64
    %v1087 = arith.constant 4 : i64
    %v1088 = call @sloth_cls_info(%v1086, %v1087) : (i64, i64) -> i64
    %v1089 = arith.constant 0 : i64
    %v1090 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1088, %v1089, %v1090) : (i64, i64, i64) -> i64
    %v1091 = arith.constant 1 : i64
    %v1092 = call @sloth_obj_new(%v1088, %v1091) : (i64, i64) -> i64
    %v1093 = memref.alloca() : memref<1xi64>
    %v1094 = call @sloth_rc_retain(%v1092) : (i64) -> i64
    %v1095 = arith.constant 0 : index
    memref.store %v1094, %v1093[%v1095] : memref<1xi64>
    %v1096 = memref.extract_aligned_pointer_as_index %v1093 : memref<1xi64> -> index
    %v1097 = arith.index_cast %v1096 : index to i64
    call @sloth_fiber_track(%v1097) : (i64) -> i64
    call @sloth_rc_release(%v1092) : (i64) -> i64
    %v1098 = arith.constant 0 : index
    %v1099 = memref.load %v1093[%v1098] : memref<1xi64>
    %v1100 = arith.constant 4609434218613702656 : i64
    call @sloth_main_Box_float__set(%v1099, %v1100) : (i64, i64) -> ()
    %v1101 = arith.constant 0 : i64
    %v1102 = arith.constant 0 : index
    %v1103 = memref.load %v1093[%v1102] : memref<1xi64>
    %v1104 = call @sloth_main_Box_float__get(%v1103) : (i64) -> i64
    %v1106 = llvm.bitcast %v1104 : i64 to f64
    %v1105 = call @sloth_rt_print_f64(%v1106) : (f64) -> i64
    %v1107 = arith.constant 0 : index
    %v1108 = memref.load %v1007[%v1107] : memref<1xi64>
    call @sloth_rc_release(%v1108) : (i64) -> i64
    %v1109 = memref.extract_aligned_pointer_as_index %v1007 : memref<1xi64> -> index
    %v1110 = arith.index_cast %v1109 : index to i64
    call @sloth_fiber_untrack(%v1110) : (i64) -> i64
    %v1111 = arith.constant 0 : index
    %v1112 = memref.load %v1033[%v1111] : memref<1xi64>
    call @sloth_rc_release(%v1112) : (i64) -> i64
    %v1113 = memref.extract_aligned_pointer_as_index %v1033 : memref<1xi64> -> index
    %v1114 = arith.index_cast %v1113 : index to i64
    call @sloth_fiber_untrack(%v1114) : (i64) -> i64
    %v1115 = arith.constant 0 : index
    %v1116 = memref.load %v1052[%v1115] : memref<1xi64>
    call @sloth_rc_release(%v1116) : (i64) -> i64
    %v1117 = memref.extract_aligned_pointer_as_index %v1052 : memref<1xi64> -> index
    %v1118 = arith.index_cast %v1117 : index to i64
    call @sloth_fiber_untrack(%v1118) : (i64) -> i64
    %v1119 = arith.constant 0 : index
    %v1120 = memref.load %v1073[%v1119] : memref<1xi64>
    call @sloth_rc_release(%v1120) : (i64) -> i64
    %v1121 = memref.extract_aligned_pointer_as_index %v1073 : memref<1xi64> -> index
    %v1122 = arith.index_cast %v1121 : index to i64
    call @sloth_fiber_untrack(%v1122) : (i64) -> i64
    %v1123 = arith.constant 0 : index
    %v1124 = memref.load %v1093[%v1123] : memref<1xi64>
    call @sloth_rc_release(%v1124) : (i64) -> i64
    %v1125 = memref.extract_aligned_pointer_as_index %v1093 : memref<1xi64> -> index
    %v1126 = arith.index_cast %v1125 : index to i64
    call @sloth_fiber_untrack(%v1126) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Box_int__set(%p0: i64, %p1: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1003 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1003[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1003[%v1005] : memref<1xi64>
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1001[%v1007] : memref<1xi64>
    %v1009 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1008, %v1009, %v1006) : (i64, i64, i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Box_int__get(%p0: i64) -> i64 {
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
  func.func @sloth_main_Box_float__set(%p0: i64, %p1: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1003 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1003[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1003[%v1005] : memref<1xi64>
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1001[%v1007] : memref<1xi64>
    %v1009 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1008, %v1009, %v1006) : (i64, i64, i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Box_float__get(%p0: i64) -> i64 {
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
}

