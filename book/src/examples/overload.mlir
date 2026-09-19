module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 1 : i64
    %v1002 = arith.constant 2 : i64
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.constant 2 : i64
    %v1005 = call @sloth_cls_info(%v1003, %v1004) : (i64, i64) -> i64
    %v1006 = arith.constant 0 : i64
    %v1007 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1005, %v1006, %v1007) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 2 : i64
    %v1009 = call @sloth_obj_new(%v1005, %v1008) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1009, %v1001, %v1002) : (i64, i64, i64) -> ()
    %v1010 = memref.alloca() : memref<1xi64>
    %v1011 = call @sloth_rc_retain(%v1009) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    memref.store %v1011, %v1010[%v1012] : memref<1xi64>
    %v1013 = memref.extract_aligned_pointer_as_index %v1010 : memref<1xi64> -> index
    %v1014 = arith.index_cast %v1013 : index to i64
    call @sloth_fiber_track(%v1014) : (i64) -> i64
    call @sloth_rc_release(%v1009) : (i64) -> i64
    %v1015 = arith.constant 3 : i64
    %v1016 = arith.constant 4 : i64
    %v1017 = arith.constant 0 : i64
    %v1018 = arith.constant 2 : i64
    %v1019 = call @sloth_cls_info(%v1017, %v1018) : (i64, i64) -> i64
    %v1020 = arith.constant 0 : i64
    %v1021 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1019, %v1020, %v1021) : (i64, i64, i64) -> i64
    %v1022 = arith.constant 2 : i64
    %v1023 = call @sloth_obj_new(%v1019, %v1022) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1023, %v1015, %v1016) : (i64, i64, i64) -> ()
    %v1024 = memref.alloca() : memref<1xi64>
    %v1025 = call @sloth_rc_retain(%v1023) : (i64) -> i64
    %v1026 = arith.constant 0 : index
    memref.store %v1025, %v1024[%v1026] : memref<1xi64>
    %v1027 = memref.extract_aligned_pointer_as_index %v1024 : memref<1xi64> -> index
    %v1028 = arith.index_cast %v1027 : index to i64
    call @sloth_fiber_track(%v1028) : (i64) -> i64
    call @sloth_rc_release(%v1023) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1010[%v1029] : memref<1xi64>
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1024[%v1031] : memref<1xi64>
    %v1033 = call @sloth_main_Vec2____add__(%v1030, %v1032) : (i64, i64) -> i64
    %v1034 = call @sloth_main_Vec2__show(%v1033) : (i64) -> i64
    %v1035 = call @sloth_rt_print_str(%v1034) : (i64) -> i64
    call @sloth_rc_release(%v1033) : (i64) -> i64
    call @sloth_rc_release(%v1034) : (i64) -> i64
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1024[%v1036] : memref<1xi64>
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1010[%v1038] : memref<1xi64>
    %v1040 = call @sloth_main_Vec2____sub__(%v1037, %v1039) : (i64, i64) -> i64
    %v1041 = call @sloth_main_Vec2__show(%v1040) : (i64) -> i64
    %v1042 = call @sloth_rt_print_str(%v1041) : (i64) -> i64
    call @sloth_rc_release(%v1040) : (i64) -> i64
    call @sloth_rc_release(%v1041) : (i64) -> i64
    %v1043 = arith.constant 0 : index
    %v1044 = memref.load %v1010[%v1043] : memref<1xi64>
    %v1046 = call @sloth_main_Vec2____neg__(%v1044) : (i64) -> i64
    %v1047 = call @sloth_main_Vec2__show(%v1046) : (i64) -> i64
    %v1048 = call @sloth_rt_print_str(%v1047) : (i64) -> i64
    call @sloth_rc_release(%v1046) : (i64) -> i64
    call @sloth_rc_release(%v1047) : (i64) -> i64
    %v1049 = arith.constant 0 : index
    %v1050 = memref.load %v1010[%v1049] : memref<1xi64>
    %v1051 = arith.constant 1 : i64
    %v1052 = arith.constant 2 : i64
    %v1053 = arith.constant 0 : i64
    %v1054 = arith.constant 2 : i64
    %v1055 = call @sloth_cls_info(%v1053, %v1054) : (i64, i64) -> i64
    %v1056 = arith.constant 0 : i64
    %v1057 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1055, %v1056, %v1057) : (i64, i64, i64) -> i64
    %v1058 = arith.constant 2 : i64
    %v1059 = call @sloth_obj_new(%v1055, %v1058) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1059, %v1051, %v1052) : (i64, i64, i64) -> ()
    %v1060 = call @sloth_main_Vec2____eq__(%v1050, %v1059) : (i64, i64) -> i64
    %v1061 = call @sloth_rt_print_bool(%v1060) : (i64) -> i64
    call @sloth_rc_release(%v1059) : (i64) -> i64
    %v1062 = arith.constant 0 : index
    %v1063 = memref.load %v1010[%v1062] : memref<1xi64>
    %v1064 = arith.constant 0 : index
    %v1065 = memref.load %v1024[%v1064] : memref<1xi64>
    %v1066 = call @sloth_main_Vec2____lt__(%v1063, %v1065) : (i64, i64) -> i64
    %v1067 = call @sloth_rt_print_bool(%v1066) : (i64) -> i64
    %v1068 = arith.constant 0 : i64
    %v1069 = arith.constant 3 : i64
    %v1070 = call @sloth_cls_info(%v1068, %v1069) : (i64, i64) -> i64
    %v1071 = arith.constant 1 : i64
    %v1072 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1070, %v1071, %v1072) : (i64, i64, i64) -> i64
    %v1073 = arith.constant 1 : i64
    %v1074 = call @sloth_obj_new(%v1070, %v1073) : (i64, i64) -> i64
    call @sloth_main_Bag____init__(%v1074) : (i64) -> ()
    %v1075 = memref.alloca() : memref<1xi64>
    %v1076 = call @sloth_rc_retain(%v1074) : (i64) -> i64
    %v1077 = arith.constant 0 : index
    memref.store %v1076, %v1075[%v1077] : memref<1xi64>
    %v1078 = memref.extract_aligned_pointer_as_index %v1075 : memref<1xi64> -> index
    %v1079 = arith.index_cast %v1078 : index to i64
    call @sloth_fiber_track(%v1079) : (i64) -> i64
    call @sloth_rc_release(%v1074) : (i64) -> i64
    %v1080 = arith.constant 7 : i64
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1075[%v1081] : memref<1xi64>
    %v1083 = arith.constant 0 : i64
    call @sloth_main_Bag____assign__(%v1082, %v1083, %v1080) : (i64, i64, i64) -> ()
    %v1084 = arith.constant 0 : i64
    %v1085 = arith.constant 8 : i64
    %v1086 = arith.constant 0 : index
    %v1087 = memref.load %v1075[%v1086] : memref<1xi64>
    %v1088 = arith.constant 1 : i64
    call @sloth_main_Bag____assign__(%v1087, %v1088, %v1085) : (i64, i64, i64) -> ()
    %v1089 = arith.constant 0 : i64
    %v1090 = arith.constant 0 : index
    %v1091 = memref.load %v1075[%v1090] : memref<1xi64>
    %v1092 = arith.constant 0 : i64
    %v1093 = call @sloth_main_Bag____index__(%v1091, %v1092) : (i64, i64) -> i64
    %v1094 = arith.constant 0 : index
    %v1095 = memref.load %v1075[%v1094] : memref<1xi64>
    %v1096 = arith.constant 1 : i64
    %v1097 = call @sloth_main_Bag____index__(%v1095, %v1096) : (i64, i64) -> i64
    %v1098 = arith.addi %v1093, %v1097 : i64
    %v1099 = call @sloth_rt_print_i64(%v1098) : (i64) -> i64
    %v1100 = arith.constant 0 : index
    %v1101 = memref.load %v1010[%v1100] : memref<1xi64>
    call @sloth_rc_release(%v1101) : (i64) -> i64
    %v1102 = memref.extract_aligned_pointer_as_index %v1010 : memref<1xi64> -> index
    %v1103 = arith.index_cast %v1102 : index to i64
    call @sloth_fiber_untrack(%v1103) : (i64) -> i64
    %v1104 = arith.constant 0 : index
    %v1105 = memref.load %v1075[%v1104] : memref<1xi64>
    call @sloth_rc_release(%v1105) : (i64) -> i64
    %v1106 = memref.extract_aligned_pointer_as_index %v1075 : memref<1xi64> -> index
    %v1107 = arith.index_cast %v1106 : index to i64
    call @sloth_fiber_untrack(%v1107) : (i64) -> i64
    %v1108 = arith.constant 0 : index
    %v1109 = memref.load %v1024[%v1108] : memref<1xi64>
    call @sloth_rc_release(%v1109) : (i64) -> i64
    %v1110 = memref.extract_aligned_pointer_as_index %v1024 : memref<1xi64> -> index
    %v1111 = arith.index_cast %v1110 : index to i64
    call @sloth_fiber_untrack(%v1111) : (i64) -> i64
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
    call @sloth_obj_set_field(%v1010, %v1011, %v1008) : (i64, i64, i64) -> i64
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1005[%v1012] : memref<1xi64>
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1001[%v1014] : memref<1xi64>
    %v1016 = arith.constant 1 : i64
    call @sloth_obj_set_field(%v1015, %v1016, %v1013) : (i64, i64, i64) -> i64
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
    %v1014 = arith.addi %v1009, %v1013 : i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1002[%v1015] : memref<1xi64>
    %v1017 = arith.constant 1 : i64
    %v1018 = call @sloth_obj_field(%v1016, %v1017) : (i64, i64) -> i64
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1004[%v1019] : memref<1xi64>
    %v1021 = arith.constant 1 : i64
    %v1022 = call @sloth_obj_field(%v1020, %v1021) : (i64, i64) -> i64
    %v1023 = arith.addi %v1018, %v1022 : i64
    %v1024 = arith.constant 0 : i64
    %v1025 = arith.constant 2 : i64
    %v1026 = call @sloth_cls_info(%v1024, %v1025) : (i64, i64) -> i64
    %v1027 = arith.constant 0 : i64
    %v1028 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1026, %v1027, %v1028) : (i64, i64, i64) -> i64
    %v1029 = arith.constant 2 : i64
    %v1030 = call @sloth_obj_new(%v1026, %v1029) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1030, %v1014, %v1023) : (i64, i64, i64) -> ()
    %v1031 = arith.constant 0 : index
    memref.store %v1030, %v1001[%v1031] : memref<1xi64>
    %v1032 = arith.constant 1 : i64
    memref.store %v1032, %v1000[%v1031] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1033 = arith.constant 0 : index
    %v1034 = memref.load %v1001[%v1033] : memref<1xi64>
    return %v1034 : i64
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
    %v1014 = arith.subi %v1009, %v1013 : i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1002[%v1015] : memref<1xi64>
    %v1017 = arith.constant 1 : i64
    %v1018 = call @sloth_obj_field(%v1016, %v1017) : (i64, i64) -> i64
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1004[%v1019] : memref<1xi64>
    %v1021 = arith.constant 1 : i64
    %v1022 = call @sloth_obj_field(%v1020, %v1021) : (i64, i64) -> i64
    %v1023 = arith.subi %v1018, %v1022 : i64
    %v1024 = arith.constant 0 : i64
    %v1025 = arith.constant 2 : i64
    %v1026 = call @sloth_cls_info(%v1024, %v1025) : (i64, i64) -> i64
    %v1027 = arith.constant 0 : i64
    %v1028 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1026, %v1027, %v1028) : (i64, i64, i64) -> i64
    %v1029 = arith.constant 2 : i64
    %v1030 = call @sloth_obj_new(%v1026, %v1029) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1030, %v1014, %v1023) : (i64, i64, i64) -> ()
    %v1031 = arith.constant 0 : index
    memref.store %v1030, %v1001[%v1031] : memref<1xi64>
    %v1032 = arith.constant 1 : i64
    memref.store %v1032, %v1000[%v1031] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1033 = arith.constant 0 : index
    %v1034 = memref.load %v1001[%v1033] : memref<1xi64>
    return %v1034 : i64
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
    %v1009 = arith.subi %v1004, %v1008 : i64
    %v1010 = arith.constant 0 : i64
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1002[%v1011] : memref<1xi64>
    %v1013 = arith.constant 1 : i64
    %v1014 = call @sloth_obj_field(%v1012, %v1013) : (i64, i64) -> i64
    %v1015 = arith.subi %v1010, %v1014 : i64
    %v1016 = arith.constant 0 : i64
    %v1017 = arith.constant 2 : i64
    %v1018 = call @sloth_cls_info(%v1016, %v1017) : (i64, i64) -> i64
    %v1019 = arith.constant 0 : i64
    %v1020 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1018, %v1019, %v1020) : (i64, i64, i64) -> i64
    %v1021 = arith.constant 2 : i64
    %v1022 = call @sloth_obj_new(%v1018, %v1021) : (i64, i64) -> i64
    call @sloth_main_Vec2____init__(%v1022, %v1009, %v1015) : (i64, i64, i64) -> ()
    %v1023 = arith.constant 0 : index
    memref.store %v1022, %v1001[%v1023] : memref<1xi64>
    %v1024 = arith.constant 1 : i64
    memref.store %v1024, %v1000[%v1023] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1025 = arith.constant 0 : index
    %v1026 = memref.load %v1001[%v1025] : memref<1xi64>
    return %v1026 : i64
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
    %v1017 = memref.alloca() : memref<1xi64>
    %v1018 = arith.constant 0 : index
    memref.store %v1016, %v1017[%v1018] : memref<1xi64>
    %v1019 = arith.constant 0 : i64
    %v1020 = arith.cmpi ne, %v1016, %v1019 : i64
    %v1021 = arith.extui %v1020 : i1 to i64
    %v1023 = arith.constant 0 : i64
    %v1022 = arith.cmpi ne, %v1021, %v1023 : i64
    cf.cond_br %v1022, ^sc_1, ^sc_2
  ^sc_1:
    %v1024 = arith.constant 0 : index
    %v1025 = memref.load %v1002[%v1024] : memref<1xi64>
    %v1026 = arith.constant 1 : i64
    %v1027 = call @sloth_obj_field(%v1025, %v1026) : (i64, i64) -> i64
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1004[%v1028] : memref<1xi64>
    %v1030 = arith.constant 1 : i64
    %v1031 = call @sloth_obj_field(%v1029, %v1030) : (i64, i64) -> i64
    %v1033 = arith.constant 0 : i64
    %v1032 = arith.cmpi eq, %v1027, %v1031 : i64
    %v1034 = arith.extui %v1032 : i1 to i64
    %v1035 = arith.constant 0 : index
    memref.store %v1034, %v1017[%v1035] : memref<1xi64>
    cf.br ^sc_2
  ^sc_2:
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1017[%v1036] : memref<1xi64>
    %v1038 = arith.constant 0 : index
    memref.store %v1037, %v1001[%v1038] : memref<1xi64>
    %v1039 = arith.constant 1 : i64
    memref.store %v1039, %v1000[%v1038] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1040 = arith.constant 0 : index
    %v1041 = memref.load %v1001[%v1040] : memref<1xi64>
    return %v1041 : i64
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
    %v1017 = arith.constant 0 : index
    memref.store %v1016, %v1001[%v1017] : memref<1xi64>
    %v1018 = arith.constant 1 : i64
    memref.store %v1018, %v1000[%v1017] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1001[%v1019] : memref<1xi64>
    return %v1020 : i64
  }
  func.func @sloth_main_Vec2__show(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 40 : i64
    %v1006 = arith.constant 1 : i64
    %v1007 = call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = arith.constant 0 : i64
    %v1011 = call @sloth_obj_field(%v1009, %v1010) : (i64, i64) -> i64
    %v1012 = call @sloth_str_push_i(%v1007, %v1011) : (i64, i64) -> i64
    %v1013 = arith.constant 8236 : i64
    %v1014 = arith.constant 2 : i64
    %v1015 = call @sloth_str_push(%v1012, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1002[%v1016] : memref<1xi64>
    %v1018 = arith.constant 1 : i64
    %v1019 = call @sloth_obj_field(%v1017, %v1018) : (i64, i64) -> i64
    %v1020 = call @sloth_str_push_i(%v1015, %v1019) : (i64, i64) -> i64
    %v1021 = arith.constant 41 : i64
    %v1022 = arith.constant 1 : i64
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
    %v1006 = arith.constant 3 : i64
    %v1007 = call @sloth_arr_new(%v1006) : (i64) -> i64
    %v1008 = arith.constant 0 : i64
    call @sloth_arr_set(%v1007, %v1008, %v1003) : (i64, i64, i64) -> i64
    %v1009 = arith.constant 1 : i64
    call @sloth_arr_set(%v1007, %v1009, %v1004) : (i64, i64, i64) -> i64
    %v1010 = arith.constant 2 : i64
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

