module @main {

  func.func @sloth_main__ginit() -> () {
    return
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
    %v1007 = arith.constant 0 : i64
    %v1008 = arith.constant 2 : i64
    %v1009 = call @sloth_cls_info(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 3 : i64
    %v1011 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1009, %v1010, %v1011) : (i64, i64, i64) -> i64
    %v1012 = arith.constant 2 : i64
    %v1013 = call @sloth_obj_new(%v1009, %v1012) : (i64, i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.constant 110 : i64
    %v1016 = arith.constant 1 : i64
    %v1017 = call @sloth_str_push(%v1014, %v1015, %v1016) : (i64, i64, i64) -> i64
    %v1018 = call @sloth_str_finish(%v1017) : (i64) -> i64
    %v1019 = arith.constant 0 : i64
    %v1020 = call @sloth_obj_field(%v1013, %v1019) : (i64, i64) -> i64
    call @sloth_rc_release(%v1020) : (i64) -> i64
    %v1021 = call @sloth_rc_retain(%v1018) : (i64) -> i64
    call @sloth_obj_set_field(%v1013, %v1019, %v1021) : (i64, i64, i64) -> i64
    %v1022 = arith.constant 0 : i64
    %v1023 = arith.constant 1 : i64
    %v1024 = call @sloth_obj_field(%v1013, %v1023) : (i64, i64) -> i64
    call @sloth_rc_release(%v1024) : (i64) -> i64
    %v1025 = call @sloth_rc_retain(%v1022) : (i64) -> i64
    call @sloth_obj_set_field(%v1013, %v1023, %v1025) : (i64, i64, i64) -> i64
    %v1026 = memref.alloca() : memref<1xi64>
    %v1027 = call @sloth_rc_retain(%v1013) : (i64) -> i64
    %v1028 = arith.constant 0 : index
    memref.store %v1027, %v1026[%v1028] : memref<1xi64>
    %v1029 = memref.extract_aligned_pointer_as_index %v1026 : memref<1xi64> -> index
    %v1030 = arith.index_cast %v1029 : index to i64
    call @sloth_fiber_track(%v1030) : (i64) -> i64
    call @sloth_rc_release(%v1013) : (i64) -> i64
    call @sloth_rc_release(%v1018) : (i64) -> i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1026[%v1031] : memref<1xi64>
    %v1033 = call @sloth_weak_new(%v1032) : (i64) -> i64
    %v1034 = arith.constant 0 : index
    %v1035 = memref.load %v1002[%v1034] : memref<1xi64>
    call @sloth_rc_release(%v1035) : (i64) -> i64
    %v1036 = call @sloth_rc_retain(%v1033) : (i64) -> i64
    %v1037 = arith.constant 0 : index
    memref.store %v1036, %v1002[%v1037] : memref<1xi64>
    call @sloth_rc_release(%v1033) : (i64) -> i64
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1026[%v1038] : memref<1xi64>
    call @sloth_rc_release(%v1039) : (i64) -> i64
    %v1040 = memref.extract_aligned_pointer_as_index %v1026 : memref<1xi64> -> index
    %v1041 = arith.index_cast %v1040 : index to i64
    call @sloth_fiber_untrack(%v1041) : (i64) -> i64
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1002[%v1042] : memref<1xi64>
    %v1044 = call @sloth_weak_upgrade(%v1043) : (i64) -> i64
    %v1045 = memref.alloca() : memref<1xi64>
    %v1046 = call @sloth_rc_retain(%v1044) : (i64) -> i64
    %v1047 = arith.constant 0 : index
    memref.store %v1046, %v1045[%v1047] : memref<1xi64>
    %v1048 = memref.extract_aligned_pointer_as_index %v1045 : memref<1xi64> -> index
    %v1049 = arith.index_cast %v1048 : index to i64
    call @sloth_fiber_track(%v1049) : (i64) -> i64
    call @sloth_rc_release(%v1044) : (i64) -> i64
    %v1050 = arith.constant 0 : index
    %v1051 = memref.load %v1045[%v1050] : memref<1xi64>
    %v1052 = arith.constant 0 : i64
    %v1053 = arith.cmpi eq, %v1051, %v1052 : i64
    %v1054 = arith.extui %v1053 : i1 to i64
    %v1055 = call @sloth_rt_print_bool(%v1054) : (i64) -> i64
    %v1056 = arith.constant 0 : i64
    %v1057 = arith.constant 2 : i64
    %v1058 = call @sloth_cls_info(%v1056, %v1057) : (i64, i64) -> i64
    %v1059 = arith.constant 3 : i64
    %v1060 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1058, %v1059, %v1060) : (i64, i64, i64) -> i64
    %v1061 = arith.constant 2 : i64
    %v1062 = call @sloth_obj_new(%v1058, %v1061) : (i64, i64) -> i64
    %v1063 = arith.constant 0 : i64
    %v1064 = arith.constant 110 : i64
    %v1065 = arith.constant 1 : i64
    %v1066 = call @sloth_str_push(%v1063, %v1064, %v1065) : (i64, i64, i64) -> i64
    %v1067 = call @sloth_str_finish(%v1066) : (i64) -> i64
    %v1068 = arith.constant 0 : i64
    %v1069 = call @sloth_obj_field(%v1062, %v1068) : (i64, i64) -> i64
    call @sloth_rc_release(%v1069) : (i64) -> i64
    %v1070 = call @sloth_rc_retain(%v1067) : (i64) -> i64
    call @sloth_obj_set_field(%v1062, %v1068, %v1070) : (i64, i64, i64) -> i64
    %v1071 = arith.constant 0 : i64
    %v1072 = arith.constant 1 : i64
    %v1073 = call @sloth_obj_field(%v1062, %v1072) : (i64, i64) -> i64
    call @sloth_rc_release(%v1073) : (i64) -> i64
    %v1074 = call @sloth_rc_retain(%v1071) : (i64) -> i64
    call @sloth_obj_set_field(%v1062, %v1072, %v1074) : (i64, i64, i64) -> i64
    %v1075 = memref.alloca() : memref<1xi64>
    %v1076 = call @sloth_rc_retain(%v1062) : (i64) -> i64
    %v1077 = arith.constant 0 : index
    memref.store %v1076, %v1075[%v1077] : memref<1xi64>
    %v1078 = memref.extract_aligned_pointer_as_index %v1075 : memref<1xi64> -> index
    %v1079 = arith.index_cast %v1078 : index to i64
    call @sloth_fiber_track(%v1079) : (i64) -> i64
    call @sloth_rc_release(%v1062) : (i64) -> i64
    call @sloth_rc_release(%v1067) : (i64) -> i64
    %v1080 = arith.constant 0 : i64
    %v1081 = arith.constant 435778317409 : i64
    %v1082 = arith.constant 5 : i64
    %v1083 = call @sloth_str_push(%v1080, %v1081, %v1082) : (i64, i64, i64) -> i64
    %v1084 = call @sloth_str_finish(%v1083) : (i64) -> i64
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1075[%v1085] : memref<1xi64>
    %v1087 = arith.constant 0 : i64
    %v1088 = call @sloth_obj_field(%v1086, %v1087) : (i64, i64) -> i64
    call @sloth_rc_release(%v1088) : (i64) -> i64
    %v1089 = call @sloth_rc_retain(%v1084) : (i64) -> i64
    call @sloth_obj_set_field(%v1086, %v1087, %v1089) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1084) : (i64) -> i64
    %v1090 = arith.constant 0 : index
    %v1091 = memref.load %v1075[%v1090] : memref<1xi64>
    %v1092 = call @sloth_weak_new(%v1091) : (i64) -> i64
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
    %v1100 = call @sloth_weak_upgrade(%v1099) : (i64) -> i64
    %v1101 = memref.alloca() : memref<1xi64>
    %v1102 = call @sloth_rc_retain(%v1100) : (i64) -> i64
    %v1103 = arith.constant 0 : index
    memref.store %v1102, %v1101[%v1103] : memref<1xi64>
    %v1104 = memref.extract_aligned_pointer_as_index %v1101 : memref<1xi64> -> index
    %v1105 = arith.index_cast %v1104 : index to i64
    call @sloth_fiber_track(%v1105) : (i64) -> i64
    call @sloth_rc_release(%v1100) : (i64) -> i64
    %v1106 = arith.constant 0 : index
    %v1107 = memref.load %v1101[%v1106] : memref<1xi64>
    %v1108 = arith.constant 0 : i64
    %v1109 = arith.cmpi eq, %v1107, %v1108 : i64
    %v1110 = arith.extui %v1109 : i1 to i64
    %v1111 = arith.constant 1 : i64
    %v1112 = arith.xori %v1110, %v1111 : i64
    %v1114 = arith.constant 0 : i64
    %v1113 = arith.cmpi ne, %v1112, %v1114 : i64
    cf.cond_br %v1113, ^t_1, ^e_2
  ^t_1:
    %v1115 = arith.constant 0 : index
    %v1116 = memref.load %v1101[%v1115] : memref<1xi64>
    %v1117 = arith.constant 0 : i64
    %v1118 = call @sloth_obj_field(%v1116, %v1117) : (i64, i64) -> i64
    %v1119 = call @sloth_rt_print_str(%v1118) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1120 = arith.constant 0 : index
    %v1121 = memref.load %v1045[%v1120] : memref<1xi64>
    call @sloth_rc_release(%v1121) : (i64) -> i64
    %v1122 = memref.extract_aligned_pointer_as_index %v1045 : memref<1xi64> -> index
    %v1123 = arith.index_cast %v1122 : index to i64
    call @sloth_fiber_untrack(%v1123) : (i64) -> i64
    %v1124 = arith.constant 0 : index
    %v1125 = memref.load %v1075[%v1124] : memref<1xi64>
    call @sloth_rc_release(%v1125) : (i64) -> i64
    %v1126 = memref.extract_aligned_pointer_as_index %v1075 : memref<1xi64> -> index
    %v1127 = arith.index_cast %v1126 : index to i64
    call @sloth_fiber_untrack(%v1127) : (i64) -> i64
    %v1128 = arith.constant 0 : index
    %v1129 = memref.load %v1093[%v1128] : memref<1xi64>
    call @sloth_rc_release(%v1129) : (i64) -> i64
    %v1130 = memref.extract_aligned_pointer_as_index %v1093 : memref<1xi64> -> index
    %v1131 = arith.index_cast %v1130 : index to i64
    call @sloth_fiber_untrack(%v1131) : (i64) -> i64
    %v1132 = arith.constant 0 : index
    %v1133 = memref.load %v1002[%v1132] : memref<1xi64>
    call @sloth_rc_release(%v1133) : (i64) -> i64
    %v1134 = memref.extract_aligned_pointer_as_index %v1002 : memref<1xi64> -> index
    %v1135 = arith.index_cast %v1134 : index to i64
    call @sloth_fiber_untrack(%v1135) : (i64) -> i64
    %v1136 = arith.constant 0 : index
    %v1137 = memref.load %v1101[%v1136] : memref<1xi64>
    call @sloth_rc_release(%v1137) : (i64) -> i64
    %v1138 = memref.extract_aligned_pointer_as_index %v1101 : memref<1xi64> -> index
    %v1139 = arith.index_cast %v1138 : index to i64
    call @sloth_fiber_untrack(%v1139) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

