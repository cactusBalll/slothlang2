module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 6 : i64
    %v1002 = arith.constant 0 : i64
    %v1003 = arith.constant 6 : i64
    %v1004 = call @sloth_cls_info(%v1002, %v1003) : (i64, i64) -> i64
    %v1005 = arith.constant 0 : i64
    %v1006 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 4 : i64
    %v1008 = call @sloth_obj_new(%v1004, %v1007) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%v1008, %v1001) : (i64, i64) -> ()
    %v1009 = memref.alloca() : memref<1xi64>
    %v1010 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1009[%v1011] : memref<1xi64>
    call @sloth_rc_release(%v1008) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1009[%v1012] : memref<1xi64>
    %v1014 = arith.constant 0 : i64
    %v1015 = call @sloth_obj_field(%v1013, %v1014) : (i64, i64) -> i64
    %v1016 = call @sloth_rt_print_i64(%v1015) : (i64) -> i64
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1009[%v1017] : memref<1xi64>
    %v1019 = call @sloth_main_Dog__who(%v1018) : (i64) -> i64
    %v1020 = call @sloth_rt_print_str(%v1019) : (i64) -> i64
    call @sloth_rc_release(%v1019) : (i64) -> i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1009[%v1021] : memref<1xi64>
    %v1023 = memref.alloca() : memref<1xi64>
    %v1024 = call @sloth_rc_retain(%v1022) : (i64) -> i64
    %v1025 = arith.constant 0 : index
    memref.store %v1024, %v1023[%v1025] : memref<1xi64>
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1023[%v1026] : memref<1xi64>
    %v1028 = call @sloth_main_Dog__who(%v1027) : (i64) -> i64
    %v1029 = call @sloth_rt_print_str(%v1028) : (i64) -> i64
    call @sloth_rc_release(%v1028) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1023[%v1030] : memref<1xi64>
    %v1032 = arith.constant 6 : i64
    %v1033 = call @sloth_obj_cls_id(%v1031) : (i64) -> i64
    %v1034 = arith.cmpi eq, %v1033, %v1032 : i64
    %v1035 = arith.extui %v1034 : i1 to i64
    %v1036 = arith.constant 1 : i64
    %v1037 = arith.shli %v1035, %v1036 : i64
    %v1039 = arith.constant 0 : i64
    %v1038 = arith.cmpi ne, %v1037, %v1039 : i64
    cf.cond_br %v1038, ^t_1, ^e_2
  ^t_1:
    %v1040 = arith.constant 0 : index
    %v1041 = memref.load %v1023[%v1040] : memref<1xi64>
    %v1042 = arith.constant 2 : i64
    %v1043 = call @sloth_obj_field(%v1041, %v1042) : (i64, i64) -> i64
    %v1044 = call @sloth_rt_print_i64(%v1043) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1045 = arith.constant 2 : i64
    %v1046 = arith.constant 0 : i64
    %v1047 = arith.constant 4 : i64
    %v1048 = call @sloth_cls_info(%v1046, %v1047) : (i64, i64) -> i64
    %v1049 = arith.constant 0 : i64
    %v1050 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1048, %v1049, %v1050) : (i64, i64, i64) -> i64
    %v1051 = arith.constant 2 : i64
    %v1052 = call @sloth_obj_new(%v1048, %v1051) : (i64, i64) -> i64
    call @sloth_main_Animal____init__(%v1052, %v1045) : (i64, i64) -> ()
    %v1053 = arith.constant 4 : i64
    %v1054 = arith.constant 0 : i64
    %v1055 = arith.constant 6 : i64
    %v1056 = call @sloth_cls_info(%v1054, %v1055) : (i64, i64) -> i64
    %v1057 = arith.constant 0 : i64
    %v1058 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1056, %v1057, %v1058) : (i64, i64, i64) -> i64
    %v1059 = arith.constant 4 : i64
    %v1060 = call @sloth_obj_new(%v1056, %v1059) : (i64, i64) -> i64
    call @sloth_main_Dog____init__(%v1060, %v1053) : (i64, i64) -> ()
    %v1061 = arith.constant 4 : i64
    %v1063 = arith.constant 2 : i64
    %v1062 = call @sloth_arr_new_k(%v1061, %v1063) : (i64, i64) -> i64
    %v1064 = arith.constant 0 : i64
    %v1065 = call @sloth_rc_retain(%v1052) : (i64) -> i64
    call @sloth_arr_set(%v1062, %v1064, %v1065) : (i64, i64, i64) -> i64
    %v1066 = arith.constant 2 : i64
    %v1067 = call @sloth_rc_retain(%v1060) : (i64) -> i64
    call @sloth_arr_set(%v1062, %v1066, %v1067) : (i64, i64, i64) -> i64
    %v1068 = memref.alloca() : memref<1xi64>
    %v1069 = call @sloth_rc_retain(%v1062) : (i64) -> i64
    %v1070 = arith.constant 0 : index
    memref.store %v1069, %v1068[%v1070] : memref<1xi64>
    call @sloth_rc_release(%v1052) : (i64) -> i64
    call @sloth_rc_release(%v1060) : (i64) -> i64
    call @sloth_rc_release(%v1062) : (i64) -> i64
    %v1071 = arith.constant 0 : index
    %v1072 = memref.load %v1068[%v1071] : memref<1xi64>
    %v1073 = call @sloth_arr_len(%v1072) : (i64) -> i64
    %v1074 = call @sloth_rt_print_i64(%v1073) : (i64) -> i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1068[%v1075] : memref<1xi64>
    %v1077 = arith.constant 2 : i64
    %v1078 = call @sloth_arr_get(%v1076, %v1077) : (i64, i64) -> i64
    %v1079 = memref.alloca() : memref<1xi64>
    %v1080 = call @sloth_obj_cls_id(%v1078) : (i64) -> i64
    %v1081 = arith.constant 6 : i64
    %v1082 = arith.cmpi eq, %v1080, %v1081 : i64
    %v1083 = arith.extui %v1082 : i1 to i64
    %v1085 = arith.constant 0 : i64
    %v1084 = arith.cmpi ne, %v1083, %v1085 : i64
    cf.cond_br %v1084, ^cv_6, ^cv_4
  ^cv_4:
    %v1086 = call @sloth_main_Animal__who(%v1078) : (i64) -> i64
    %v1087 = arith.constant 0 : index
    memref.store %v1086, %v1079[%v1087] : memref<1xi64>
    cf.br ^cv_5
  ^cv_6:
    %v1088 = call @sloth_main_Dog__who(%v1078) : (i64) -> i64
    %v1089 = arith.constant 0 : index
    memref.store %v1088, %v1079[%v1089] : memref<1xi64>
    cf.br ^cv_5
  ^cv_5:
    %v1090 = arith.constant 0 : index
    %v1091 = memref.load %v1079[%v1090] : memref<1xi64>
    %v1092 = call @sloth_rt_print_str(%v1091) : (i64) -> i64
    call @sloth_rc_release(%v1091) : (i64) -> i64
    %v1093 = arith.constant 0 : index
    %v1094 = memref.load %v1009[%v1093] : memref<1xi64>
    call @sloth_rc_release(%v1094) : (i64) -> i64
    %v1095 = arith.constant 0 : index
    %v1096 = memref.load %v1068[%v1095] : memref<1xi64>
    call @sloth_rc_release(%v1096) : (i64) -> i64
    %v1097 = arith.constant 0 : index
    %v1098 = memref.load %v1023[%v1097] : memref<1xi64>
    call @sloth_rc_release(%v1098) : (i64) -> i64
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
    %v1010 = call @sloth_obj_field(%v1008, %v1009) : (i64, i64) -> i64
    call @sloth_rc_release(%v1010) : (i64) -> i64
    %v1011 = call @sloth_rc_retain(%v1006) : (i64) -> i64
    call @sloth_obj_set_field(%v1008, %v1009, %v1011) : (i64, i64, i64) -> i64
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
    %v1006 = arith.constant 12 : i64
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
    %v1007 = arith.constant 14 : i64
    call @sloth_main_Animal____init__(%v1006, %v1007) : (i64, i64) -> ()
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.constant 0 : index
    %v1010 = memref.load %v1003[%v1009] : memref<1xi64>
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    %v1013 = arith.constant 2 : i64
    %v1014 = call @sloth_obj_field(%v1012, %v1013) : (i64, i64) -> i64
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1015 = call @sloth_rc_retain(%v1010) : (i64) -> i64
    call @sloth_obj_set_field(%v1012, %v1013, %v1015) : (i64, i64, i64) -> i64
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
}

