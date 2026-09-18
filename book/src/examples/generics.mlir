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
    %v1008 = arith.constant 4 : i64
    %v1010 = arith.constant 2 : i64
    %v1009 = call @sloth_arr_new_k(%v1008, %v1010) : (i64, i64) -> i64
    %v1011 = arith.constant 0 : i64
    %v1012 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    call @sloth_arr_set(%v1009, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 2 : i64
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
    %v1008 = arith.constant 4 : i64
    %v1010 = arith.constant 2 : i64
    %v1009 = call @sloth_arr_new_k(%v1008, %v1010) : (i64, i64) -> i64
    %v1011 = arith.constant 0 : i64
    %v1012 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    call @sloth_arr_set(%v1009, %v1011, %v1012) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 2 : i64
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
    %v1001 = arith.constant 20 : i64
    %v1002 = arith.constant 40 : i64
    %v1003 = arith.constant 4 : i64
    %v1004 = call @sloth_arr_new(%v1003) : (i64) -> i64
    %v1005 = arith.constant 0 : i64
    call @sloth_arr_set(%v1004, %v1005, %v1001) : (i64, i64, i64) -> i64
    %v1006 = arith.constant 2 : i64
    call @sloth_arr_set(%v1004, %v1006, %v1002) : (i64, i64, i64) -> i64
    %v1007 = memref.alloca() : memref<1xi64>
    %v1008 = call @sloth_rc_retain(%v1004) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1007[%v1009] : memref<1xi64>
    call @sloth_rc_release(%v1004) : (i64) -> i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1007[%v1010] : memref<1xi64>
    %v1012 = call @sloth_main__first_int(%v1011) : (i64) -> i64
    %v1013 = call @sloth_rt_print_i64(%v1012) : (i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.constant 120 : i64
    %v1016 = arith.constant 2 : i64
    %v1017 = call @sloth_str_push(%v1014, %v1015, %v1016) : (i64, i64, i64) -> i64
    %v1018 = call @sloth_str_finish(%v1017) : (i64) -> i64
    %v1019 = arith.constant 0 : i64
    %v1020 = arith.constant 121 : i64
    %v1021 = arith.constant 2 : i64
    %v1022 = call @sloth_str_push(%v1019, %v1020, %v1021) : (i64, i64, i64) -> i64
    %v1023 = call @sloth_str_finish(%v1022) : (i64) -> i64
    %v1024 = arith.constant 4 : i64
    %v1026 = arith.constant 2 : i64
    %v1025 = call @sloth_arr_new_k(%v1024, %v1026) : (i64, i64) -> i64
    %v1027 = arith.constant 0 : i64
    %v1028 = call @sloth_rc_retain(%v1018) : (i64) -> i64
    call @sloth_arr_set(%v1025, %v1027, %v1028) : (i64, i64, i64) -> i64
    %v1029 = arith.constant 2 : i64
    %v1030 = call @sloth_rc_retain(%v1023) : (i64) -> i64
    call @sloth_arr_set(%v1025, %v1029, %v1030) : (i64, i64, i64) -> i64
    %v1031 = memref.alloca() : memref<1xi64>
    %v1032 = call @sloth_rc_retain(%v1025) : (i64) -> i64
    %v1033 = arith.constant 0 : index
    memref.store %v1032, %v1031[%v1033] : memref<1xi64>
    call @sloth_rc_release(%v1018) : (i64) -> i64
    call @sloth_rc_release(%v1023) : (i64) -> i64
    call @sloth_rc_release(%v1025) : (i64) -> i64
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1031[%v1034] : memref<1xi64>
    %v1036 = call @sloth_main__first_str(%v1035) : (i64) -> i64
    %v1037 = call @sloth_rt_print_str(%v1036) : (i64) -> i64
    call @sloth_rc_release(%v1036) : (i64) -> i64
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1007[%v1038] : memref<1xi64>
    %v1040 = call @sloth_main__first_int(%v1039) : (i64) -> i64
    %v1041 = call @sloth_rt_print_i64(%v1040) : (i64) -> i64
    %v1042 = arith.constant 0 : i64
    %v1043 = arith.constant 122 : i64
    %v1044 = arith.constant 2 : i64
    %v1045 = call @sloth_str_push(%v1042, %v1043, %v1044) : (i64, i64, i64) -> i64
    %v1046 = call @sloth_str_finish(%v1045) : (i64) -> i64
    %v1047 = call @sloth_main__twice_str(%v1046) : (i64) -> i64
    %v1048 = memref.alloca() : memref<1xi64>
    %v1049 = arith.constant 0 : index
    memref.store %v1047, %v1048[%v1049] : memref<1xi64>
    call @sloth_rc_release(%v1046) : (i64) -> i64
    %v1050 = arith.constant 0 : index
    %v1051 = memref.load %v1048[%v1050] : memref<1xi64>
    %v1052 = arith.constant 0 : i64
    %v1053 = call @sloth_arr_get(%v1051, %v1052) : (i64, i64) -> i64
    %v1054 = arith.constant 0 : index
    %v1055 = memref.load %v1048[%v1054] : memref<1xi64>
    %v1056 = arith.constant 2 : i64
    %v1057 = call @sloth_arr_get(%v1055, %v1056) : (i64, i64) -> i64
    %v1058 = call @sloth_str_concat(%v1053, %v1057) : (i64, i64) -> i64
    %v1059 = call @sloth_rt_print_str(%v1058) : (i64) -> i64
    call @sloth_rc_release(%v1058) : (i64) -> i64
    %v1060 = arith.constant 0 : i64
    %v1061 = arith.constant 6 : i64
    %v1062 = call @sloth_cls_info(%v1060, %v1061) : (i64, i64) -> i64
    %v1063 = arith.constant 0 : i64
    %v1064 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1062, %v1063, %v1064) : (i64, i64, i64) -> i64
    %v1065 = arith.constant 2 : i64
    %v1066 = call @sloth_obj_new(%v1062, %v1065) : (i64, i64) -> i64
    %v1067 = memref.alloca() : memref<1xi64>
    %v1068 = call @sloth_rc_retain(%v1066) : (i64) -> i64
    %v1069 = arith.constant 0 : index
    memref.store %v1068, %v1067[%v1069] : memref<1xi64>
    call @sloth_rc_release(%v1066) : (i64) -> i64
    %v1070 = arith.constant 0 : index
    %v1071 = memref.load %v1067[%v1070] : memref<1xi64>
    %v1072 = arith.constant 14 : i64
    call @sloth_main_Box_int__set(%v1071, %v1072) : (i64, i64) -> ()
    %v1073 = arith.constant 0 : i64
    %v1074 = arith.constant 0 : index
    %v1075 = memref.load %v1067[%v1074] : memref<1xi64>
    %v1076 = call @sloth_main_Box_int__get(%v1075) : (i64) -> i64
    %v1077 = call @sloth_rt_print_i64(%v1076) : (i64) -> i64
    %v1078 = arith.constant 0 : i64
    %v1079 = arith.constant 8 : i64
    %v1080 = call @sloth_cls_info(%v1078, %v1079) : (i64, i64) -> i64
    %v1081 = arith.constant 0 : i64
    %v1082 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1080, %v1081, %v1082) : (i64, i64, i64) -> i64
    %v1083 = arith.constant 2 : i64
    %v1084 = call @sloth_obj_new(%v1080, %v1083) : (i64, i64) -> i64
    %v1085 = memref.alloca() : memref<1xi64>
    %v1086 = call @sloth_rc_retain(%v1084) : (i64) -> i64
    %v1087 = arith.constant 0 : index
    memref.store %v1086, %v1085[%v1087] : memref<1xi64>
    call @sloth_rc_release(%v1084) : (i64) -> i64
    %v1088 = arith.constant 0 : index
    %v1089 = memref.load %v1085[%v1088] : memref<1xi64>
    %v1090 = arith.constant 2304717109306851328 : i64
    call @sloth_main_Box_float__set(%v1089, %v1090) : (i64, i64) -> ()
    %v1091 = arith.constant 0 : i64
    %v1092 = arith.constant 0 : index
    %v1093 = memref.load %v1085[%v1092] : memref<1xi64>
    %v1094 = call @sloth_main_Box_float__get(%v1093) : (i64) -> i64
    %v1096 = arith.constant 1 : i64
    %v1097 = arith.shli %v1094, %v1096 : i64
    %v1098 = llvm.bitcast %v1097 : i64 to f64
    %v1095 = call @sloth_rt_print_f64(%v1098) : (f64) -> i64
    %v1099 = arith.constant 0 : index
    %v1100 = memref.load %v1067[%v1099] : memref<1xi64>
    call @sloth_rc_release(%v1100) : (i64) -> i64
    %v1101 = arith.constant 0 : index
    %v1102 = memref.load %v1085[%v1101] : memref<1xi64>
    call @sloth_rc_release(%v1102) : (i64) -> i64
    %v1103 = arith.constant 0 : index
    %v1104 = memref.load %v1007[%v1103] : memref<1xi64>
    call @sloth_rc_release(%v1104) : (i64) -> i64
    %v1105 = arith.constant 0 : index
    %v1106 = memref.load %v1031[%v1105] : memref<1xi64>
    call @sloth_rc_release(%v1106) : (i64) -> i64
    %v1107 = arith.constant 0 : index
    %v1108 = memref.load %v1048[%v1107] : memref<1xi64>
    call @sloth_rc_release(%v1108) : (i64) -> i64
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
    %v1010 = call @sloth_obj_field(%v1008, %v1009) : (i64, i64) -> i64
    call @sloth_rc_release(%v1010) : (i64) -> i64
    %v1011 = call @sloth_rc_retain(%v1006) : (i64) -> i64
    call @sloth_obj_set_field(%v1008, %v1009, %v1011) : (i64, i64, i64) -> i64
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
    %v1010 = call @sloth_obj_field(%v1008, %v1009) : (i64, i64) -> i64
    call @sloth_rc_release(%v1010) : (i64) -> i64
    %v1011 = call @sloth_rc_retain(%v1006) : (i64) -> i64
    call @sloth_obj_set_field(%v1008, %v1009, %v1011) : (i64, i64, i64) -> i64
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

