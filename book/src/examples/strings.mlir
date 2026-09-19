module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 478560413032 : i64
    %v1003 = arith.constant 5 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1006[%v1008] : memref<1xi64>
    %v1009 = memref.extract_aligned_pointer_as_index %v1006 : memref<1xi64> -> index
    %v1010 = arith.index_cast %v1009 : index to i64
    call @sloth_fiber_track(%v1010) : (i64) -> i64
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1006[%v1011] : memref<1xi64>
    %v1013 = arith.constant 0 : i64
    %v1014 = arith.constant 28266736423215148 : i64
    %v1015 = arith.constant 7 : i64
    %v1016 = call @sloth_str_push(%v1013, %v1014, %v1015) : (i64, i64, i64) -> i64
    %v1017 = call @sloth_str_finish(%v1016) : (i64) -> i64
    %v1018 = call @sloth_str_concat(%v1012, %v1017) : (i64, i64) -> i64
    %v1019 = call @sloth_rt_print_str(%v1018) : (i64) -> i64
    call @sloth_rc_release(%v1017) : (i64) -> i64
    call @sloth_rc_release(%v1018) : (i64) -> i64
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1006[%v1020] : memref<1xi64>
    %v1022 = call @sloth_str_len(%v1021) : (i64) -> i64
    %v1023 = call @sloth_rt_print_i64(%v1022) : (i64) -> i64
    %v1024 = arith.constant 0 : i64
    %v1025 = arith.constant 122511470216040 : i64
    %v1026 = arith.constant 6 : i64
    %v1027 = call @sloth_str_push(%v1024, %v1025, %v1026) : (i64, i64, i64) -> i64
    %v1028 = call @sloth_str_finish(%v1027) : (i64) -> i64
    %v1029 = call @sloth_str_len(%v1028) : (i64) -> i64
    %v1030 = call @sloth_rt_print_i64(%v1029) : (i64) -> i64
    call @sloth_rc_release(%v1028) : (i64) -> i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1006[%v1031] : memref<1xi64>
    %v1033 = arith.constant 0 : i64
    %v1034 = arith.constant 7103848 : i64
    %v1035 = arith.constant 3 : i64
    %v1036 = call @sloth_str_push(%v1033, %v1034, %v1035) : (i64, i64, i64) -> i64
    %v1037 = call @sloth_str_finish(%v1036) : (i64) -> i64
    %v1038 = arith.constant 0 : i64
    %v1039 = arith.constant 28524 : i64
    %v1040 = arith.constant 2 : i64
    %v1041 = call @sloth_str_push(%v1038, %v1039, %v1040) : (i64, i64, i64) -> i64
    %v1042 = call @sloth_str_finish(%v1041) : (i64) -> i64
    %v1043 = call @sloth_str_concat(%v1037, %v1042) : (i64, i64) -> i64
    %v1044 = call @sloth_str_eq(%v1032, %v1043) : (i64, i64) -> i64
    %v1045 = call @sloth_rt_print_bool(%v1044) : (i64) -> i64
    call @sloth_rc_release(%v1037) : (i64) -> i64
    call @sloth_rc_release(%v1042) : (i64) -> i64
    call @sloth_rc_release(%v1043) : (i64) -> i64
    %v1046 = arith.constant 0 : i64
    %v1047 = arith.constant 97 : i64
    %v1048 = arith.constant 1 : i64
    %v1049 = call @sloth_str_push(%v1046, %v1047, %v1048) : (i64, i64, i64) -> i64
    %v1050 = call @sloth_str_finish(%v1049) : (i64) -> i64
    %v1051 = arith.constant 0 : i64
    %v1052 = arith.constant 98 : i64
    %v1053 = arith.constant 1 : i64
    %v1054 = call @sloth_str_push(%v1051, %v1052, %v1053) : (i64, i64, i64) -> i64
    %v1055 = call @sloth_str_finish(%v1054) : (i64) -> i64
    %v1056 = call @sloth_str_eq(%v1050, %v1055) : (i64, i64) -> i64
    %v1057 = arith.constant 1 : i64
    %v1058 = arith.xori %v1056, %v1057 : i64
    %v1059 = call @sloth_rt_print_bool(%v1058) : (i64) -> i64
    call @sloth_rc_release(%v1050) : (i64) -> i64
    call @sloth_rc_release(%v1055) : (i64) -> i64
    %v1060 = arith.constant 0 : i64
    %v1061 = memref.alloca() : memref<1xi64>
    %v1062 = arith.constant 0 : index
    memref.store %v1060, %v1061[%v1062] : memref<1xi64>
    %v1063 = arith.constant 0 : i64
    %v1064 = arith.constant 191009621918561 : i64
    %v1065 = arith.constant 6 : i64
    %v1066 = call @sloth_str_push(%v1063, %v1064, %v1065) : (i64, i64, i64) -> i64
    %v1067 = call @sloth_str_finish(%v1066) : (i64) -> i64
    %v1068 = call @sloth_str_clen(%v1067) : (i64) -> i64
    %v1070 = arith.constant 0 : index
    %v1069 = memref.alloca() : memref<1xi64>
    %v1071 = arith.constant 0 : i64
    memref.store %v1071, %v1069[%v1070] : memref<1xi64>
    cf.br ^af_1
  ^af_1:
    %v1072 = memref.load %v1069[%v1070] : memref<1xi64>
    %v1073 = arith.cmpi slt, %v1072, %v1068 : i64
    %v1074 = arith.extui %v1073 : i1 to i64
    %v1076 = arith.constant 0 : i64
    %v1075 = arith.cmpi ne, %v1074, %v1076 : i64
    cf.cond_br %v1075, ^ab_2, ^ae_3
  ^ab_2:
    %v1077 = call @sloth_str_char(%v1067, %v1072) : (i64, i64) -> i64
    %v1078 = memref.alloca() : memref<1xi64>
    memref.store %v1077, %v1078[%v1070] : memref<1xi64>
    %v1079 = arith.constant 0 : index
    %v1080 = memref.load %v1061[%v1079] : memref<1xi64>
    %v1081 = arith.constant 1 : i64
    %v1082 = arith.addi %v1080, %v1081 : i64
    %v1083 = arith.constant 0 : index
    memref.store %v1082, %v1061[%v1083] : memref<1xi64>
    cf.br ^ic_4
  ^ic_4:
    call @sloth_rc_release(%v1077) : (i64) -> i64
    %v1084 = arith.constant 1 : i64
    %v1085 = arith.addi %v1072, %v1084 : i64
    memref.store %v1085, %v1069[%v1070] : memref<1xi64>
    cf.br ^af_1
  ^ix_5:
    %v1086 = memref.load %v1078[%v1070] : memref<1xi64>
    call @sloth_rc_release(%v1086) : (i64) -> i64
    cf.br ^ae_3
  ^ae_3:
    call @sloth_rc_release(%v1067) : (i64) -> i64
    %v1087 = arith.constant 0 : index
    %v1088 = memref.load %v1061[%v1087] : memref<1xi64>
    %v1089 = call @sloth_rt_print_i64(%v1088) : (i64) -> i64
    %v1090 = arith.constant 0 : i64
    %v1091 = arith.constant 120 : i64
    %v1092 = arith.constant 1 : i64
    %v1093 = call @sloth_str_push(%v1090, %v1091, %v1092) : (i64, i64, i64) -> i64
    %v1094 = call @sloth_str_finish(%v1093) : (i64) -> i64
    %v1095 = memref.alloca() : memref<1xi64>
    %v1096 = call @sloth_rc_retain(%v1094) : (i64) -> i64
    %v1097 = arith.constant 0 : index
    memref.store %v1096, %v1095[%v1097] : memref<1xi64>
    %v1098 = memref.extract_aligned_pointer_as_index %v1095 : memref<1xi64> -> index
    %v1099 = arith.index_cast %v1098 : index to i64
    call @sloth_fiber_track(%v1099) : (i64) -> i64
    call @sloth_rc_release(%v1094) : (i64) -> i64
    %v1100 = arith.constant 0 : index
    %v1101 = memref.load %v1095[%v1100] : memref<1xi64>
    %v1102 = arith.constant 0 : i64
    %v1103 = arith.constant 121 : i64
    %v1104 = arith.constant 1 : i64
    %v1105 = call @sloth_str_push(%v1102, %v1103, %v1104) : (i64, i64, i64) -> i64
    %v1106 = call @sloth_str_finish(%v1105) : (i64) -> i64
    %v1107 = call @sloth_str_concat(%v1101, %v1106) : (i64, i64) -> i64
    %v1108 = arith.constant 0 : i64
    %v1109 = arith.constant 122 : i64
    %v1110 = arith.constant 1 : i64
    %v1111 = call @sloth_str_push(%v1108, %v1109, %v1110) : (i64, i64, i64) -> i64
    %v1112 = call @sloth_str_finish(%v1111) : (i64) -> i64
    %v1113 = call @sloth_str_concat(%v1107, %v1112) : (i64, i64) -> i64
    %v1114 = arith.constant 0 : index
    %v1115 = memref.load %v1095[%v1114] : memref<1xi64>
    call @sloth_rc_release(%v1115) : (i64) -> i64
    %v1116 = call @sloth_rc_retain(%v1113) : (i64) -> i64
    %v1117 = arith.constant 0 : index
    memref.store %v1116, %v1095[%v1117] : memref<1xi64>
    call @sloth_rc_release(%v1106) : (i64) -> i64
    call @sloth_rc_release(%v1107) : (i64) -> i64
    call @sloth_rc_release(%v1112) : (i64) -> i64
    call @sloth_rc_release(%v1113) : (i64) -> i64
    %v1118 = arith.constant 0 : index
    %v1119 = memref.load %v1095[%v1118] : memref<1xi64>
    %v1120 = call @sloth_rt_print_str(%v1119) : (i64) -> i64
    %v1121 = arith.constant 4609434218613702656 : i64
    %v1122 = memref.alloca() : memref<1xi64>
    %v1123 = arith.constant 0 : index
    memref.store %v1121, %v1122[%v1123] : memref<1xi64>
    %v1124 = arith.constant 0 : i64
    %v1125 = arith.constant 15726 : i64
    %v1126 = arith.constant 2 : i64
    %v1127 = call @sloth_str_push(%v1124, %v1125, %v1126) : (i64, i64, i64) -> i64
    %v1128 = arith.constant 0 : index
    %v1129 = memref.load %v1061[%v1128] : memref<1xi64>
    %v1130 = call @sloth_str_push_i(%v1127, %v1129) : (i64, i64) -> i64
    %v1131 = arith.constant 4023840 : i64
    %v1132 = arith.constant 3 : i64
    %v1133 = call @sloth_str_push(%v1130, %v1131, %v1132) : (i64, i64, i64) -> i64
    %v1134 = arith.constant 0 : index
    %v1135 = memref.load %v1122[%v1134] : memref<1xi64>
    %v1137 = llvm.bitcast %v1135 : i64 to f64
    %v1136 = call @sloth_str_push_f(%v1133, %v1137) : (i64, f64) -> i64
    %v1138 = arith.constant 4022816 : i64
    %v1139 = arith.constant 3 : i64
    %v1140 = call @sloth_str_push(%v1136, %v1138, %v1139) : (i64, i64, i64) -> i64
    %v1141 = arith.constant 1 : i64
    %v1142 = call @sloth_str_push_b(%v1140, %v1141) : (i64, i64) -> i64
    %v1143 = call @sloth_str_finish(%v1142) : (i64) -> i64
    %v1144 = call @sloth_rt_print_str(%v1143) : (i64) -> i64
    call @sloth_rc_release(%v1143) : (i64) -> i64
    %v1145 = arith.constant 0 : index
    %v1146 = memref.load %v1006[%v1145] : memref<1xi64>
    call @sloth_rc_release(%v1146) : (i64) -> i64
    %v1147 = memref.extract_aligned_pointer_as_index %v1006 : memref<1xi64> -> index
    %v1148 = arith.index_cast %v1147 : index to i64
    call @sloth_fiber_untrack(%v1148) : (i64) -> i64
    %v1149 = arith.constant 0 : index
    %v1150 = memref.load %v1095[%v1149] : memref<1xi64>
    call @sloth_rc_release(%v1150) : (i64) -> i64
    %v1151 = memref.extract_aligned_pointer_as_index %v1095 : memref<1xi64> -> index
    %v1152 = arith.index_cast %v1151 : index to i64
    call @sloth_fiber_untrack(%v1152) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

