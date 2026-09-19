module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 3 : i64
    %v1002 = arith.constant 0 : i64
    %v1003 = arith.constant 3 : i64
    %v1004 = call @sloth_cls_info(%v1002, %v1003) : (i64, i64) -> i64
    %v1005 = arith.constant 0 : i64
    %v1006 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 2 : i64
    %v1008 = call @sloth_obj_new(%v1004, %v1007) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%v1008, %v1001) : (i64, i64) -> ()
    %v1009 = memref.alloca() : memref<1xi64>
    %v1010 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1009[%v1011] : memref<1xi64>
    %v1012 = memref.extract_aligned_pointer_as_index %v1009 : memref<1xi64> -> index
    %v1013 = arith.index_cast %v1012 : index to i64
    call @sloth_fiber_track(%v1013) : (i64) -> i64
    call @sloth_rc_release(%v1008) : (i64) -> i64
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1009[%v1014] : memref<1xi64>
    %v1016 = arith.constant 0 : i64
    %v1017 = call @sloth_obj_field(%v1015, %v1016) : (i64, i64) -> i64
    %v1018 = call @sloth_rt_print_i64(%v1017) : (i64) -> i64
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1009[%v1019] : memref<1xi64>
    %v1021 = call @sloth_main_Dog__who(%v1020) : (i64) -> i64
    %v1022 = call @sloth_rt_print_str(%v1021) : (i64) -> i64
    call @sloth_rc_release(%v1021) : (i64) -> i64
    %v1023 = arith.constant 0 : index
    %v1024 = memref.load %v1009[%v1023] : memref<1xi64>
    %v1025 = memref.alloca() : memref<1xi64>
    %v1026 = call @sloth_rc_retain(%v1024) : (i64) -> i64
    %v1027 = arith.constant 0 : index
    memref.store %v1026, %v1025[%v1027] : memref<1xi64>
    %v1028 = memref.extract_aligned_pointer_as_index %v1025 : memref<1xi64> -> index
    %v1029 = arith.index_cast %v1028 : index to i64
    call @sloth_fiber_track(%v1029) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1025[%v1030] : memref<1xi64>
    %v1032 = call @sloth_main_Dog__who(%v1031) : (i64) -> i64
    %v1033 = call @sloth_rt_print_str(%v1032) : (i64) -> i64
    call @sloth_rc_release(%v1032) : (i64) -> i64
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1025[%v1034] : memref<1xi64>
    %v1036 = arith.constant 3 : i64
    %v1037 = call @sloth_obj_cls_id(%v1035) : (i64) -> i64
    %v1038 = arith.cmpi eq, %v1037, %v1036 : i64
    %v1039 = arith.extui %v1038 : i1 to i64
    %v1041 = arith.constant 0 : i64
    %v1040 = arith.cmpi ne, %v1039, %v1041 : i64
    cf.cond_br %v1040, ^t_1, ^e_2
  ^t_1:
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1025[%v1042] : memref<1xi64>
    %v1044 = arith.constant 1 : i64
    %v1045 = call @sloth_obj_field(%v1043, %v1044) : (i64, i64) -> i64
    %v1046 = call @sloth_rt_print_i64(%v1045) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1047 = arith.constant 1 : i64
    %v1048 = arith.constant 0 : i64
    %v1049 = arith.constant 2 : i64
    %v1050 = call @sloth_cls_info(%v1048, %v1049) : (i64, i64) -> i64
    %v1051 = arith.constant 0 : i64
    %v1052 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1050, %v1051, %v1052) : (i64, i64, i64) -> i64
    %v1053 = arith.constant 1 : i64
    %v1054 = call @sloth_obj_new(%v1050, %v1053) : (i64, i64) -> i64
    call @sloth_main_Animal____init__(%v1054, %v1047) : (i64, i64) -> ()
    %v1055 = arith.constant 2 : i64
    %v1056 = arith.constant 0 : i64
    %v1057 = arith.constant 3 : i64
    %v1058 = call @sloth_cls_info(%v1056, %v1057) : (i64, i64) -> i64
    %v1059 = arith.constant 0 : i64
    %v1060 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1058, %v1059, %v1060) : (i64, i64, i64) -> i64
    %v1061 = arith.constant 2 : i64
    %v1062 = call @sloth_obj_new(%v1058, %v1061) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%v1062, %v1055) : (i64, i64) -> ()
    %v1063 = arith.constant 2 : i64
    %v1065 = arith.constant 1 : i64
    %v1064 = call @sloth_arr_new_k(%v1063, %v1065) : (i64, i64) -> i64
    %v1066 = arith.constant 0 : i64
    %v1067 = call @sloth_rc_retain(%v1054) : (i64) -> i64
    call @sloth_arr_set(%v1064, %v1066, %v1067) : (i64, i64, i64) -> i64
    %v1068 = arith.constant 1 : i64
    %v1069 = call @sloth_rc_retain(%v1062) : (i64) -> i64
    call @sloth_arr_set(%v1064, %v1068, %v1069) : (i64, i64, i64) -> i64
    %v1070 = memref.alloca() : memref<1xi64>
    %v1071 = call @sloth_rc_retain(%v1064) : (i64) -> i64
    %v1072 = arith.constant 0 : index
    memref.store %v1071, %v1070[%v1072] : memref<1xi64>
    %v1073 = memref.extract_aligned_pointer_as_index %v1070 : memref<1xi64> -> index
    %v1074 = arith.index_cast %v1073 : index to i64
    call @sloth_fiber_track(%v1074) : (i64) -> i64
    call @sloth_rc_release(%v1054) : (i64) -> i64
    call @sloth_rc_release(%v1062) : (i64) -> i64
    call @sloth_rc_release(%v1064) : (i64) -> i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1070[%v1075] : memref<1xi64>
    %v1077 = call @sloth_arr_len(%v1076) : (i64) -> i64
    %v1078 = call @sloth_rt_print_i64(%v1077) : (i64) -> i64
    %v1079 = arith.constant 0 : index
    %v1080 = memref.load %v1070[%v1079] : memref<1xi64>
    %v1081 = arith.constant 1 : i64
    %v1082 = call @sloth_arr_get(%v1080, %v1081) : (i64, i64) -> i64
    %v1083 = memref.alloca() : memref<1xi64>
    %v1084 = call @sloth_obj_cls_id(%v1082) : (i64) -> i64
    %v1085 = arith.constant 3 : i64
    %v1086 = arith.cmpi eq, %v1084, %v1085 : i64
    %v1087 = arith.extui %v1086 : i1 to i64
    %v1089 = arith.constant 0 : i64
    %v1088 = arith.cmpi ne, %v1087, %v1089 : i64
    cf.cond_br %v1088, ^cv_6, ^cv_4
  ^cv_4:
    %v1090 = call @sloth_main_Animal__who(%v1082) : (i64) -> i64
    %v1091 = arith.constant 0 : index
    memref.store %v1090, %v1083[%v1091] : memref<1xi64>
    cf.br ^cv_5
  ^cv_6:
    %v1092 = call @sloth_main_Dog__who(%v1082) : (i64) -> i64
    %v1093 = arith.constant 0 : index
    memref.store %v1092, %v1083[%v1093] : memref<1xi64>
    cf.br ^cv_5
  ^cv_5:
    %v1094 = arith.constant 0 : index
    %v1095 = memref.load %v1083[%v1094] : memref<1xi64>
    %v1096 = call @sloth_rt_print_str(%v1095) : (i64) -> i64
    call @sloth_rc_release(%v1095) : (i64) -> i64
    %v1097 = arith.constant 0 : index
    %v1098 = memref.load %v1070[%v1097] : memref<1xi64>
    call @sloth_rc_release(%v1098) : (i64) -> i64
    %v1099 = memref.extract_aligned_pointer_as_index %v1070 : memref<1xi64> -> index
    %v1100 = arith.index_cast %v1099 : index to i64
    call @sloth_fiber_untrack(%v1100) : (i64) -> i64
    %v1101 = arith.constant 0 : index
    %v1102 = memref.load %v1009[%v1101] : memref<1xi64>
    call @sloth_rc_release(%v1102) : (i64) -> i64
    %v1103 = memref.extract_aligned_pointer_as_index %v1009 : memref<1xi64> -> index
    %v1104 = arith.index_cast %v1103 : index to i64
    call @sloth_fiber_untrack(%v1104) : (i64) -> i64
    %v1105 = arith.constant 0 : index
    %v1106 = memref.load %v1025[%v1105] : memref<1xi64>
    call @sloth_rc_release(%v1106) : (i64) -> i64
    %v1107 = memref.extract_aligned_pointer_as_index %v1025 : memref<1xi64> -> index
    %v1108 = arith.index_cast %v1107 : index to i64
    call @sloth_fiber_untrack(%v1108) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Animal____init__(%p0: i64, %p1: i64) -> () {
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
  func.func @sloth_main_Animal__who(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 119165703253601 : i64
    %v1006 = arith.constant 6 : i64
    %v1007 = call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = call @sloth_str_finish(%v1007) : (i64) -> i64
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
  func.func @sloth_main_Dog____init__(%p0: i64, %p1: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1003 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1003[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1001[%v1005] : memref<1xi64>
    %v1007 = arith.constant 7 : i64
    call @sloth_main_Animal____init__(%v1006, %v1007) : (i64, i64) -> ()
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1003[%v1009] : memref<1xi64>
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    %v1013 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1012, %v1013, %v1010) : (i64, i64, i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Dog__who(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 6778724 : i64
    %v1006 = arith.constant 3 : i64
    %v1007 = call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = call @sloth_str_finish(%v1007) : (i64) -> i64
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
}

