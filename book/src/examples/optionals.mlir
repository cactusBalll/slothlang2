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
    %v1006 = arith.constant 7 : i64
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
    %v1005 = memref.extract_aligned_pointer_as_index %v1002 : memref<1xi64> -> index
    %v1006 = arith.index_cast %v1005 : index to i64
    call @sloth_fiber_track(%v1006) : (i64) -> i64
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1002[%v1007] : memref<1xi64>
    %v1009 = arith.constant 0 : i64
    %v1010 = arith.cmpi eq, %v1008, %v1009 : i64
    %v1011 = arith.extui %v1010 : i1 to i64
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
    %v1024 = call @sloth_rt_print_bool(%v1023) : (i64) -> i64
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1002[%v1025] : memref<1xi64>
    %v1027 = arith.constant 0 : i64
    %v1028 = arith.cmpi eq, %v1026, %v1027 : i64
    %v1029 = arith.extui %v1028 : i1 to i64
    %v1030 = arith.constant 1 : i64
    %v1031 = arith.xori %v1029, %v1030 : i64
    %v1033 = arith.constant 0 : i64
    %v1032 = arith.cmpi ne, %v1031, %v1033 : i64
    cf.cond_br %v1032, ^t_1, ^e_2
  ^t_1:
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1002[%v1034] : memref<1xi64>
    %v1036 = call @sloth_box_get(%v1035) : (i64) -> i64
    %v1037 = memref.alloca() : memref<1xi64>
    %v1038 = arith.constant 0 : index
    memref.store %v1036, %v1037[%v1038] : memref<1xi64>
    %v1039 = arith.constant 0 : index
    %v1040 = memref.load %v1037[%v1039] : memref<1xi64>
    %v1041 = call @sloth_rt_print_i64(%v1040) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1002[%v1042] : memref<1xi64>
    %v1044 = arith.constant 0 : i64
    %v1045 = arith.cmpi eq, %v1043, %v1044 : i64
    %v1046 = arith.extui %v1045 : i1 to i64
    %v1048 = arith.constant 0 : i64
    %v1047 = arith.cmpi ne, %v1046, %v1048 : i64
    cf.cond_br %v1047, ^t_4, ^e_5
  ^t_4:
    %v1049 = arith.constant 0 : i64
    %v1050 = arith.constant 1701736302 : i64
    %v1051 = arith.constant 4 : i64
    %v1052 = call @sloth_str_push(%v1049, %v1050, %v1051) : (i64, i64, i64) -> i64
    %v1053 = call @sloth_str_finish(%v1052) : (i64) -> i64
    %v1054 = call @sloth_rt_print_str(%v1053) : (i64) -> i64
    call @sloth_rc_release(%v1053) : (i64) -> i64
    cf.br ^fi_6
  ^e_5:
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1002[%v1055] : memref<1xi64>
    %v1057 = call @sloth_box_get(%v1056) : (i64) -> i64
    %v1058 = memref.alloca() : memref<1xi64>
    %v1059 = arith.constant 0 : index
    memref.store %v1057, %v1058[%v1059] : memref<1xi64>
    %v1060 = arith.constant 0 : index
    %v1061 = memref.load %v1058[%v1060] : memref<1xi64>
    %v1062 = call @sloth_rt_print_i64(%v1061) : (i64) -> i64
    cf.br ^fi_6
  ^fi_6:
    %v1063 = arith.constant 0 : i64
    %v1064 = call @sloth_main__pick(%v1063) : (i64) -> i64
    %v1065 = call @sloth_rt_print_i64(%v1064) : (i64) -> i64
    %v1066 = arith.constant 3 : i64
    %v1067 = call @sloth_box_new(%v1066) : (i64) -> i64
    %v1068 = call @sloth_main__pick(%v1067) : (i64) -> i64
    %v1069 = call @sloth_rt_print_i64(%v1068) : (i64) -> i64
    call @sloth_rc_release(%v1067) : (i64) -> i64
    %v1070 = arith.constant 0 : i64
    %v1071 = memref.alloca() : memref<1xi64>
    %v1072 = call @sloth_rc_retain(%v1070) : (i64) -> i64
    %v1073 = arith.constant 0 : index
    memref.store %v1072, %v1071[%v1073] : memref<1xi64>
    %v1074 = memref.extract_aligned_pointer_as_index %v1071 : memref<1xi64> -> index
    %v1075 = arith.index_cast %v1074 : index to i64
    call @sloth_fiber_track(%v1075) : (i64) -> i64
    %v1076 = arith.constant 0 : i64
    %v1077 = arith.constant 26984 : i64
    %v1078 = arith.constant 2 : i64
    %v1079 = call @sloth_str_push(%v1076, %v1077, %v1078) : (i64, i64, i64) -> i64
    %v1080 = call @sloth_str_finish(%v1079) : (i64) -> i64
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1071[%v1081] : memref<1xi64>
    call @sloth_rc_release(%v1082) : (i64) -> i64
    %v1083 = call @sloth_rc_retain(%v1080) : (i64) -> i64
    %v1084 = arith.constant 0 : index
    memref.store %v1083, %v1071[%v1084] : memref<1xi64>
    call @sloth_rc_release(%v1080) : (i64) -> i64
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1071[%v1085] : memref<1xi64>
    %v1087 = arith.constant 0 : i64
    %v1088 = arith.cmpi eq, %v1086, %v1087 : i64
    %v1089 = arith.extui %v1088 : i1 to i64
    %v1090 = arith.constant 1 : i64
    %v1091 = arith.xori %v1089, %v1090 : i64
    %v1093 = arith.constant 0 : i64
    %v1092 = arith.cmpi ne, %v1091, %v1093 : i64
    cf.cond_br %v1092, ^t_7, ^e_8
  ^t_7:
    %v1094 = arith.constant 0 : index
    %v1095 = memref.load %v1071[%v1094] : memref<1xi64>
    %v1096 = call @sloth_str_len(%v1095) : (i64) -> i64
    %v1097 = call @sloth_rt_print_i64(%v1096) : (i64) -> i64
    cf.br ^fi_9
  ^e_8:
    cf.br ^fi_9
  ^fi_9:
    %v1098 = arith.constant 0 : index
    %v1099 = memref.load %v1002[%v1098] : memref<1xi64>
    call @sloth_rc_release(%v1099) : (i64) -> i64
    %v1100 = memref.extract_aligned_pointer_as_index %v1002 : memref<1xi64> -> index
    %v1101 = arith.index_cast %v1100 : index to i64
    call @sloth_fiber_untrack(%v1101) : (i64) -> i64
    %v1102 = arith.constant 0 : index
    %v1103 = memref.load %v1071[%v1102] : memref<1xi64>
    call @sloth_rc_release(%v1103) : (i64) -> i64
    %v1104 = memref.extract_aligned_pointer_as_index %v1071 : memref<1xi64> -> index
    %v1105 = arith.index_cast %v1104 : index to i64
    call @sloth_fiber_untrack(%v1105) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

