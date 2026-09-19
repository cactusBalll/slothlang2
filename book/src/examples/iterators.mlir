module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 2 : i64
    %v1003 = call @sloth_cls_info(%v1001, %v1002) : (i64, i64) -> i64
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1003, %v1004, %v1005) : (i64, i64, i64) -> i64
    %v1006 = arith.constant 1 : i64
    %v1007 = call @sloth_obj_new(%v1003, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1007, %v1009, %v1008) : (i64, i64, i64) -> i64
    %v1010 = memref.alloca() : memref<1xi64>
    %v1011 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    memref.store %v1011, %v1010[%v1012] : memref<1xi64>
    %v1013 = memref.extract_aligned_pointer_as_index %v1010 : memref<1xi64> -> index
    %v1014 = arith.index_cast %v1013 : index to i64
    call @sloth_fiber_track(%v1014) : (i64) -> i64
    call @sloth_rc_release(%v1007) : (i64) -> i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1010[%v1015] : memref<1xi64>
    %v1017 = call @sloth_main_Range3__iter(%v1016) : (i64) -> i64
    %v1019 = arith.constant 0 : index
    %v1018 = memref.alloca() : memref<1xi64>
    memref.store %v1017, %v1018[%v1019] : memref<1xi64>
    %v1020 = memref.extract_aligned_pointer_as_index %v1018 : memref<1xi64> -> index
    %v1021 = arith.index_cast %v1020 : index to i64
    call @sloth_fiber_track(%v1021) : (i64) -> i64
    cf.br ^if_1
  ^if_1:
    %v1022 = memref.load %v1018[%v1019] : memref<1xi64>
    %v1023 = call @sloth_main_Range3__next(%v1022) : (i64) -> i64
    %v1025 = arith.constant 0 : i64
    %v1024 = arith.cmpi eq, %v1023, %v1025 : i64
    %v1026 = arith.extui %v1024 : i1 to i64
    %v1028 = arith.constant 0 : i64
    %v1027 = arith.cmpi ne, %v1026, %v1028 : i64
    cf.cond_br %v1027, ^ie_5, ^ib_2
  ^ib_2:
    %v1030 = call @sloth_box_get(%v1023) : (i64) -> i64
    %v1029 = memref.alloca() : memref<1xi64>
    memref.store %v1030, %v1029[%v1019] : memref<1xi64>
    call @sloth_rc_release(%v1023) : (i64) -> i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1029[%v1031] : memref<1xi64>
    %v1033 = call @sloth_rt_print_i64(%v1032) : (i64) -> i64
    cf.br ^ic_3
  ^ic_3:
    cf.br ^if_1
  ^ix_4:
    cf.br ^ie_5
  ^ie_5:
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1018[%v1034] : memref<1xi64>
    call @sloth_rc_release(%v1035) : (i64) -> i64
    %v1036 = memref.extract_aligned_pointer_as_index %v1018 : memref<1xi64> -> index
    %v1037 = arith.index_cast %v1036 : index to i64
    call @sloth_fiber_untrack(%v1037) : (i64) -> i64
    %v1038 = arith.constant 0 : i64
    %v1039 = memref.alloca() : memref<1xi64>
    %v1040 = arith.constant 0 : index
    memref.store %v1038, %v1039[%v1040] : memref<1xi64>
    %v1041 = arith.constant 0 : i64
    %v1042 = arith.constant 3 : i64
    %v1043 = arith.constant 1 : i64
    %v1044 = arith.addi %v1042, %v1043 : i64
    %v1046 = arith.constant 0 : index
    %v1045 = memref.alloca() : memref<1xi64>
    memref.store %v1041, %v1045[%v1046] : memref<1xi64>
    cf.br ^fr_6
  ^fr_6:
    %v1047 = memref.load %v1045[%v1046] : memref<1xi64>
    %v1048 = arith.cmpi slt, %v1047, %v1044 : i64
    %v1049 = arith.extui %v1048 : i1 to i64
    %v1051 = arith.constant 0 : i64
    %v1050 = arith.cmpi ne, %v1049, %v1051 : i64
    cf.cond_br %v1050, ^fb_7, ^fd_8
  ^fb_7:
    %v1052 = memref.alloca() : memref<1xi64>
    memref.store %v1047, %v1052[%v1046] : memref<1xi64>
    %v1053 = arith.constant 0 : index
    %v1054 = memref.load %v1039[%v1053] : memref<1xi64>
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1052[%v1055] : memref<1xi64>
    %v1057 = arith.addi %v1054, %v1056 : i64
    %v1058 = arith.constant 0 : index
    memref.store %v1057, %v1039[%v1058] : memref<1xi64>
    cf.br ^fc_9
  ^fc_9:
    %v1059 = arith.constant 1 : i64
    %v1060 = arith.addi %v1047, %v1059 : i64
    memref.store %v1060, %v1045[%v1046] : memref<1xi64>
    cf.br ^fr_6
  ^fd_8:
    %v1061 = arith.constant 0 : index
    %v1062 = memref.load %v1039[%v1061] : memref<1xi64>
    %v1063 = call @sloth_rt_print_i64(%v1062) : (i64) -> i64
    %v1064 = arith.constant 1 : i64
    %v1065 = arith.constant 10 : i64
    %v1066 = arith.constant 0 : i64
    %v1067 = call @sloth_map_new(%v1066) : (i64) -> i64
    call @sloth_map_set(%v1067, %v1064, %v1065) : (i64, i64, i64) -> i64
    %v1068 = call @sloth_map_keys(%v1067) : (i64) -> i64
    %v1069 = call @sloth_arr_len(%v1068) : (i64) -> i64
    %v1071 = arith.constant 0 : index
    %v1070 = memref.alloca() : memref<1xi64>
    %v1072 = arith.constant 0 : i64
    memref.store %v1072, %v1070[%v1071] : memref<1xi64>
    cf.br ^me_10
  ^me_10:
    %v1073 = memref.load %v1070[%v1071] : memref<1xi64>
    %v1074 = arith.cmpi slt, %v1073, %v1069 : i64
    %v1075 = arith.extui %v1074 : i1 to i64
    %v1077 = arith.constant 0 : i64
    %v1076 = arith.cmpi ne, %v1075, %v1077 : i64
    cf.cond_br %v1076, ^mb_11, ^md_12
  ^mb_11:
    %v1078 = call @sloth_arr_get(%v1068, %v1073) : (i64, i64) -> i64
    %v1079 = call @sloth_map_get(%v1067, %v1078) : (i64, i64) -> i64
    %v1080 = arith.constant 0 : i64
    %v1081 = arith.constant 3 : i64
    %v1082 = call @sloth_cls_info(%v1080, %v1081) : (i64, i64) -> i64
    %v1083 = arith.constant 0 : i64
    %v1084 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1082, %v1083, %v1084) : (i64, i64, i64) -> i64
    %v1085 = arith.constant 2 : i64
    %v1086 = call @sloth_obj_new(%v1082, %v1085) : (i64, i64) -> i64
    %v1087 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1086, %v1087, %v1078) : (i64, i64, i64) -> i64
    %v1088 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1086, %v1088, %v1079) : (i64, i64, i64) -> i64
    %v1089 = memref.alloca() : memref<1xi64>
    memref.store %v1086, %v1089[%v1071] : memref<1xi64>
    %v1090 = arith.constant 0 : index
    %v1091 = memref.load %v1089[%v1090] : memref<1xi64>
    %v1092 = arith.constant 0 : i64
    %v1093 = call @sloth_obj_field(%v1091, %v1092) : (i64, i64) -> i64
    %v1094 = arith.constant 0 : index
    %v1095 = memref.load %v1089[%v1094] : memref<1xi64>
    %v1096 = arith.constant 1 : i64
    %v1097 = call @sloth_obj_field(%v1095, %v1096) : (i64, i64) -> i64
    %v1098 = arith.addi %v1093, %v1097 : i64
    %v1099 = call @sloth_rt_print_i64(%v1098) : (i64) -> i64
    cf.br ^mc_13
  ^mc_13:
    call @sloth_rc_release(%v1086) : (i64) -> i64
    %v1100 = arith.constant 1 : i64
    %v1101 = arith.addi %v1073, %v1100 : i64
    memref.store %v1101, %v1070[%v1071] : memref<1xi64>
    cf.br ^me_10
  ^mx_14:
    %v1102 = memref.load %v1089[%v1071] : memref<1xi64>
    call @sloth_rc_release(%v1102) : (i64) -> i64
    cf.br ^md_12
  ^md_12:
    call @sloth_rc_release(%v1068) : (i64) -> i64
    call @sloth_rc_release(%v1067) : (i64) -> i64
    %v1103 = arith.constant 0 : index
    %v1104 = memref.load %v1010[%v1103] : memref<1xi64>
    call @sloth_rc_release(%v1104) : (i64) -> i64
    %v1105 = memref.extract_aligned_pointer_as_index %v1010 : memref<1xi64> -> index
    %v1106 = arith.index_cast %v1105 : index to i64
    call @sloth_fiber_untrack(%v1106) : (i64) -> i64
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
    %v1008 = arith.constant 3 : i64
    %v1010 = arith.constant 0 : i64
    %v1009 = arith.cmpi sge, %v1007, %v1008 : i64
    %v1011 = arith.extui %v1009 : i1 to i64
    %v1013 = arith.constant 0 : i64
    %v1012 = arith.cmpi ne, %v1011, %v1013 : i64
    cf.cond_br %v1012, ^t_1, ^e_2
  ^t_1:
    %v1014 = arith.constant 0 : i64
    %v1015 = call @sloth_rc_retain(%v1014) : (i64) -> i64
    %v1016 = arith.constant 0 : index
    memref.store %v1015, %v1001[%v1016] : memref<1xi64>
    %v1017 = arith.constant 1 : i64
    memref.store %v1017, %v1000[%v1016] : memref<1xi64>
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1018 = arith.constant 0 : index
    %v1019 = memref.load %v1002[%v1018] : memref<1xi64>
    %v1020 = arith.constant 0 : i64
    %v1021 = call @sloth_obj_field(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = arith.constant 1 : i64
    %v1023 = arith.addi %v1021, %v1022 : i64
    %v1024 = arith.constant 0 : index
    %v1025 = memref.load %v1002[%v1024] : memref<1xi64>
    %v1026 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1025, %v1026, %v1023) : (i64, i64, i64) -> i64
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1002[%v1027] : memref<1xi64>
    %v1029 = arith.constant 0 : i64
    %v1030 = call @sloth_obj_field(%v1028, %v1029) : (i64, i64) -> i64
    %v1031 = arith.constant 1 : i64
    %v1032 = arith.subi %v1030, %v1031 : i64
    %v1033 = call @sloth_box_new(%v1032) : (i64) -> i64
    %v1034 = arith.constant 0 : index
    memref.store %v1033, %v1001[%v1034] : memref<1xi64>
    %v1035 = arith.constant 1 : i64
    memref.store %v1035, %v1000[%v1034] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1001[%v1036] : memref<1xi64>
    return %v1037 : i64
  }
}

