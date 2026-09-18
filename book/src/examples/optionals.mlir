module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__pick(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 14 : i64
    %v1007 = call @sloth_box_get(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.cmpi ne, %v1005, %v1008 : i64
    %v1010 = arith.select %v1009, %v1007, %v1006 : i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1001[%v1011] : memref<1xi64>
    %v1012 = arith.constant 1 : i64
    memref.store %v1012, %v1000[%v1011] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1013 = arith.constant 0 : index
    %v1014 = memref.load %v1001[%v1013] : memref<1xi64>
    return %v1014 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = memref.alloca() : memref<1xi64>
    %v1003 = call @sloth_rc_retain(%v1001) : (i64) -> i64
    %v1004 = arith.constant 0 : index
    memref.store %v1003, %v1002[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1002[%v1005] : memref<1xi64>
    %v1007 = arith.constant 0 : i64
    %v1008 = arith.cmpi eq, %v1006, %v1007 : i64
    %v1009 = arith.extui %v1008 : i1 to i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shli %v1009, %v1010 : i64
    %v1012 = call @sloth_rt_print_bool(%v1011) : (i64) -> i64
    %v1013 = arith.constant 0 : i64
    %v1014 = call @sloth_box_new(%v1013) : (i64) -> i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1002[%v1015] : memref<1xi64>
    call @sloth_rc_release(%v1016) : (i64) -> i64
    %v1017 = call @sloth_rc_retain(%v1014) : (i64) -> i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1002[%v1018] : memref<1xi64>
    call @sloth_rc_release(%v1014) : (i64) -> i64
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1002[%v1019] : memref<1xi64>
    %v1021 = arith.constant 0 : i64
    %v1022 = arith.cmpi eq, %v1020, %v1021 : i64
    %v1023 = arith.extui %v1022 : i1 to i64
    %v1024 = arith.constant 1 : i64
    %v1025 = arith.shli %v1023, %v1024 : i64
    %v1026 = call @sloth_rt_print_bool(%v1025) : (i64) -> i64
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1002[%v1027] : memref<1xi64>
    %v1029 = arith.constant 0 : i64
    %v1030 = arith.cmpi eq, %v1028, %v1029 : i64
    %v1031 = arith.extui %v1030 : i1 to i64
    %v1032 = arith.constant 1 : i64
    %v1033 = arith.shli %v1031, %v1032 : i64
    %v1034 = arith.constant 2 : i64
    %v1035 = arith.xori %v1033, %v1034 : i64
    %v1037 = arith.constant 0 : i64
    %v1036 = arith.cmpi ne, %v1035, %v1037 : i64
    cf.cond_br %v1036, ^t_1, ^e_2
  ^t_1:
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1002[%v1038] : memref<1xi64>
    %v1040 = call @sloth_box_get(%v1039) : (i64) -> i64
    %v1041 = memref.alloca() : memref<1xi64>
    %v1042 = arith.constant 0 : index
    memref.store %v1040, %v1041[%v1042] : memref<1xi64>
    %v1043 = arith.constant 0 : index
    %v1044 = memref.load %v1041[%v1043] : memref<1xi64>
    %v1045 = call @sloth_rt_print_i64(%v1044) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1046 = arith.constant 0 : index
    %v1047 = memref.load %v1002[%v1046] : memref<1xi64>
    %v1048 = arith.constant 0 : i64
    %v1049 = arith.cmpi eq, %v1047, %v1048 : i64
    %v1050 = arith.extui %v1049 : i1 to i64
    %v1051 = arith.constant 1 : i64
    %v1052 = arith.shli %v1050, %v1051 : i64
    %v1054 = arith.constant 0 : i64
    %v1053 = arith.cmpi ne, %v1052, %v1054 : i64
    cf.cond_br %v1053, ^t_4, ^e_5
  ^t_4:
    %v1055 = arith.constant 0 : i64
    %v1056 = arith.constant 1701736302 : i64
    %v1057 = arith.constant 8 : i64
    %v1058 = call @sloth_str_push(%v1055, %v1056, %v1057) : (i64, i64, i64) -> i64
    %v1059 = call @sloth_str_finish(%v1058) : (i64) -> i64
    %v1060 = call @sloth_rt_print_str(%v1059) : (i64) -> i64
    call @sloth_rc_release(%v1059) : (i64) -> i64
    cf.br ^fi_6
  ^e_5:
    %v1061 = arith.constant 0 : index
    %v1062 = memref.load %v1002[%v1061] : memref<1xi64>
    %v1063 = call @sloth_box_get(%v1062) : (i64) -> i64
    %v1064 = memref.alloca() : memref<1xi64>
    %v1065 = arith.constant 0 : index
    memref.store %v1063, %v1064[%v1065] : memref<1xi64>
    %v1066 = arith.constant 0 : index
    %v1067 = memref.load %v1064[%v1066] : memref<1xi64>
    %v1068 = call @sloth_rt_print_i64(%v1067) : (i64) -> i64
    cf.br ^fi_6
  ^fi_6:
    %v1069 = arith.constant 0 : i64
    %v1070 = call @sloth_main__pick(%v1069) : (i64) -> i64
    %v1071 = call @sloth_rt_print_i64(%v1070) : (i64) -> i64
    %v1072 = arith.constant 6 : i64
    %v1073 = call @sloth_box_new(%v1072) : (i64) -> i64
    %v1074 = call @sloth_main__pick(%v1073) : (i64) -> i64
    %v1075 = call @sloth_rt_print_i64(%v1074) : (i64) -> i64
    call @sloth_rc_release(%v1073) : (i64) -> i64
    %v1076 = arith.constant 0 : i64
    %v1077 = memref.alloca() : memref<1xi64>
    %v1078 = call @sloth_rc_retain(%v1076) : (i64) -> i64
    %v1079 = arith.constant 0 : index
    memref.store %v1078, %v1077[%v1079] : memref<1xi64>
    %v1080 = arith.constant 0 : i64
    %v1081 = arith.constant 26984 : i64
    %v1082 = arith.constant 4 : i64
    %v1083 = call @sloth_str_push(%v1080, %v1081, %v1082) : (i64, i64, i64) -> i64
    %v1084 = call @sloth_str_finish(%v1083) : (i64) -> i64
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1077[%v1085] : memref<1xi64>
    call @sloth_rc_release(%v1086) : (i64) -> i64
    %v1087 = call @sloth_rc_retain(%v1084) : (i64) -> i64
    %v1088 = arith.constant 0 : index
    memref.store %v1087, %v1077[%v1088] : memref<1xi64>
    call @sloth_rc_release(%v1084) : (i64) -> i64
    %v1089 = arith.constant 0 : index
    %v1090 = memref.load %v1077[%v1089] : memref<1xi64>
    %v1091 = arith.constant 0 : i64
    %v1092 = arith.cmpi eq, %v1090, %v1091 : i64
    %v1093 = arith.extui %v1092 : i1 to i64
    %v1094 = arith.constant 1 : i64
    %v1095 = arith.shli %v1093, %v1094 : i64
    %v1096 = arith.constant 2 : i64
    %v1097 = arith.xori %v1095, %v1096 : i64
    %v1099 = arith.constant 0 : i64
    %v1098 = arith.cmpi ne, %v1097, %v1099 : i64
    cf.cond_br %v1098, ^t_7, ^e_8
  ^t_7:
    %v1100 = arith.constant 0 : index
    %v1101 = memref.load %v1077[%v1100] : memref<1xi64>
    %v1102 = call @sloth_str_len(%v1101) : (i64) -> i64
    %v1103 = call @sloth_rt_print_i64(%v1102) : (i64) -> i64
    cf.br ^fi_9
  ^e_8:
    cf.br ^fi_9
  ^fi_9:
    %v1104 = arith.constant 0 : index
    %v1105 = memref.load %v1002[%v1104] : memref<1xi64>
    call @sloth_rc_release(%v1105) : (i64) -> i64
    %v1106 = arith.constant 0 : index
    %v1107 = memref.load %v1077[%v1106] : memref<1xi64>
    call @sloth_rc_release(%v1107) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

