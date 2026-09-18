module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 6 : i64
    %v1002 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    memref.store %v1001, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 4 : i64
    %v1008 = arith.constant 0 : i64
    %v1007 = arith.cmpi sgt, %v1005, %v1006 : i64
    %v1009 = arith.extui %v1007 : i1 to i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shli %v1009, %v1010 : i64
    %v1013 = arith.constant 0 : i64
    %v1012 = arith.cmpi ne, %v1011, %v1013 : i64
    cf.cond_br %v1012, ^t_1, ^e_2
  ^t_1:
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.constant 6777186 : i64
    %v1016 = arith.constant 6 : i64
    %v1017 = call @sloth_str_push(%v1014, %v1015, %v1016) : (i64, i64, i64) -> i64
    %v1018 = call @sloth_str_finish(%v1017) : (i64) -> i64
    %v1019 = call @sloth_rt_print_str(%v1018) : (i64) -> i64
    call @sloth_rc_release(%v1018) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    %v1020 = arith.constant 0 : i64
    %v1021 = arith.constant 465674792307 : i64
    %v1022 = arith.constant 10 : i64
    %v1023 = call @sloth_str_push(%v1020, %v1021, %v1022) : (i64, i64, i64) -> i64
    %v1024 = call @sloth_str_finish(%v1023) : (i64) -> i64
    %v1025 = call @sloth_rt_print_str(%v1024) : (i64) -> i64
    call @sloth_rc_release(%v1024) : (i64) -> i64
    cf.br ^fi_3
  ^fi_3:
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1002[%v1026] : memref<1xi64>
    %v1028 = arith.constant 6 : i64
    %v1030 = arith.constant 0 : i64
    %v1029 = arith.cmpi eq, %v1027, %v1028 : i64
    %v1031 = arith.extui %v1029 : i1 to i64
    %v1032 = arith.constant 1 : i64
    %v1033 = arith.shli %v1031, %v1032 : i64
    %v1035 = arith.constant 0 : i64
    %v1034 = arith.cmpi ne, %v1033, %v1035 : i64
    cf.cond_br %v1034, ^t_4, ^e_5
  ^t_4:
    %v1036 = arith.constant 0 : i64
    %v1037 = arith.constant 435493693556 : i64
    %v1038 = arith.constant 10 : i64
    %v1039 = call @sloth_str_push(%v1036, %v1037, %v1038) : (i64, i64, i64) -> i64
    %v1040 = call @sloth_str_finish(%v1039) : (i64) -> i64
    %v1041 = call @sloth_rt_print_str(%v1040) : (i64) -> i64
    call @sloth_rc_release(%v1040) : (i64) -> i64
    cf.br ^fi_6
  ^e_5:
    cf.br ^fi_6
  ^fi_6:
    %v1042 = arith.constant 0 : i64
    %v1043 = memref.alloca() : memref<1xi64>
    %v1044 = arith.constant 0 : index
    memref.store %v1042, %v1043[%v1044] : memref<1xi64>
    cf.br ^wh_7
  ^wh_7:
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1043[%v1045] : memref<1xi64>
    %v1047 = arith.constant 6 : i64
    %v1049 = arith.constant 0 : i64
    %v1048 = arith.cmpi slt, %v1046, %v1047 : i64
    %v1050 = arith.extui %v1048 : i1 to i64
    %v1051 = arith.constant 1 : i64
    %v1052 = arith.shli %v1050, %v1051 : i64
    %v1054 = arith.constant 0 : i64
    %v1053 = arith.cmpi ne, %v1052, %v1054 : i64
    cf.cond_br %v1053, ^do_8, ^wd_9
  ^do_8:
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1043[%v1055] : memref<1xi64>
    %v1057 = call @sloth_rt_print_i64(%v1056) : (i64) -> i64
    %v1058 = arith.constant 0 : index
    %v1059 = memref.load %v1043[%v1058] : memref<1xi64>
    %v1060 = arith.constant 2 : i64
    %v1061 = arith.constant 1 : i64
    %v1062 = arith.shrsi %v1059, %v1061 : i64
    %v1063 = arith.constant 1 : i64
    %v1064 = arith.shrsi %v1060, %v1063 : i64
    %v1065 = arith.addi %v1062, %v1064 : i64
    %v1066 = arith.constant 1 : i64
    %v1067 = arith.shli %v1065, %v1066 : i64
    %v1068 = arith.constant 0 : index
    %v1069 = memref.load %v1043[%v1068] : memref<1xi64>
    call @sloth_rc_release(%v1069) : (i64) -> i64
    %v1070 = call @sloth_rc_retain(%v1067) : (i64) -> i64
    %v1071 = arith.constant 0 : index
    memref.store %v1070, %v1043[%v1071] : memref<1xi64>
    cf.br ^wh_7
  ^wd_9:
    %v1072 = arith.constant 0 : i64
    %v1073 = memref.alloca() : memref<1xi64>
    %v1074 = arith.constant 0 : index
    memref.store %v1072, %v1073[%v1074] : memref<1xi64>
    %v1075 = arith.constant 0 : i64
    %v1076 = arith.constant 10 : i64
    %v1078 = arith.constant 0 : index
    %v1077 = memref.alloca() : memref<1xi64>
    memref.store %v1075, %v1077[%v1078] : memref<1xi64>
    cf.br ^fr_10
  ^fr_10:
    %v1079 = memref.load %v1077[%v1078] : memref<1xi64>
    %v1080 = arith.cmpi slt, %v1079, %v1076 : i64
    %v1081 = arith.extui %v1080 : i1 to i64
    %v1083 = arith.constant 0 : i64
    %v1082 = arith.cmpi ne, %v1081, %v1083 : i64
    cf.cond_br %v1082, ^fb_11, ^fd_12
  ^fb_11:
    %v1084 = memref.alloca() : memref<1xi64>
    memref.store %v1079, %v1084[%v1078] : memref<1xi64>
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1084[%v1085] : memref<1xi64>
    %v1087 = arith.constant 2 : i64
    %v1089 = arith.constant 0 : i64
    %v1088 = arith.cmpi eq, %v1086, %v1087 : i64
    %v1090 = arith.extui %v1088 : i1 to i64
    %v1091 = arith.constant 1 : i64
    %v1092 = arith.shli %v1090, %v1091 : i64
    %v1094 = arith.constant 0 : i64
    %v1093 = arith.cmpi ne, %v1092, %v1094 : i64
    cf.cond_br %v1093, ^t_14, ^e_15
  ^t_14:
    cf.br ^fc_13
  ^e_15:
    cf.br ^fi_16
  ^fi_16:
    %v1095 = arith.constant 0 : index
    %v1096 = memref.load %v1084[%v1095] : memref<1xi64>
    %v1097 = arith.constant 8 : i64
    %v1099 = arith.constant 0 : i64
    %v1098 = arith.cmpi eq, %v1096, %v1097 : i64
    %v1100 = arith.extui %v1098 : i1 to i64
    %v1101 = arith.constant 1 : i64
    %v1102 = arith.shli %v1100, %v1101 : i64
    %v1104 = arith.constant 0 : i64
    %v1103 = arith.cmpi ne, %v1102, %v1104 : i64
    cf.cond_br %v1103, ^t_17, ^e_18
  ^t_17:
    cf.br ^fd_12
  ^e_18:
    cf.br ^fi_19
  ^fi_19:
    %v1105 = arith.constant 0 : index
    %v1106 = memref.load %v1073[%v1105] : memref<1xi64>
    %v1107 = arith.constant 0 : index
    %v1108 = memref.load %v1084[%v1107] : memref<1xi64>
    %v1109 = arith.constant 1 : i64
    %v1110 = arith.shrsi %v1106, %v1109 : i64
    %v1111 = arith.constant 1 : i64
    %v1112 = arith.shrsi %v1108, %v1111 : i64
    %v1113 = arith.addi %v1110, %v1112 : i64
    %v1114 = arith.constant 1 : i64
    %v1115 = arith.shli %v1113, %v1114 : i64
    %v1116 = arith.constant 0 : index
    %v1117 = memref.load %v1073[%v1116] : memref<1xi64>
    call @sloth_rc_release(%v1117) : (i64) -> i64
    %v1118 = call @sloth_rc_retain(%v1115) : (i64) -> i64
    %v1119 = arith.constant 0 : index
    memref.store %v1118, %v1073[%v1119] : memref<1xi64>
    cf.br ^fc_13
  ^fc_13:
    %v1120 = arith.constant 2 : i64
    %v1121 = arith.addi %v1079, %v1120 : i64
    memref.store %v1121, %v1077[%v1078] : memref<1xi64>
    cf.br ^fr_10
  ^fd_12:
    %v1122 = arith.constant 0 : index
    %v1123 = memref.load %v1073[%v1122] : memref<1xi64>
    %v1124 = call @sloth_rt_print_i64(%v1123) : (i64) -> i64
    %v1125 = arith.constant 0 : i64
    %v1126 = arith.constant 25185 : i64
    %v1127 = arith.constant 4 : i64
    %v1128 = call @sloth_str_push(%v1125, %v1126, %v1127) : (i64, i64, i64) -> i64
    %v1129 = call @sloth_str_finish(%v1128) : (i64) -> i64
    %v1130 = call @sloth_str_clen(%v1129) : (i64) -> i64
    %v1132 = arith.constant 0 : index
    %v1131 = memref.alloca() : memref<1xi64>
    %v1133 = arith.constant 0 : i64
    memref.store %v1133, %v1131[%v1132] : memref<1xi64>
    cf.br ^af_20
  ^af_20:
    %v1134 = memref.load %v1131[%v1132] : memref<1xi64>
    %v1135 = arith.cmpi slt, %v1134, %v1130 : i64
    %v1136 = arith.extui %v1135 : i1 to i64
    %v1138 = arith.constant 0 : i64
    %v1137 = arith.cmpi ne, %v1136, %v1138 : i64
    cf.cond_br %v1137, ^ab_21, ^ae_22
  ^ab_21:
    %v1139 = call @sloth_str_char(%v1129, %v1134) : (i64, i64) -> i64
    %v1140 = memref.alloca() : memref<1xi64>
    memref.store %v1139, %v1140[%v1132] : memref<1xi64>
    %v1141 = arith.constant 0 : index
    %v1142 = memref.load %v1140[%v1141] : memref<1xi64>
    %v1143 = call @sloth_rt_print_str(%v1142) : (i64) -> i64
    cf.br ^ic_23
  ^ic_23:
    call @sloth_rc_release(%v1139) : (i64) -> i64
    %v1144 = arith.constant 2 : i64
    %v1145 = arith.addi %v1134, %v1144 : i64
    memref.store %v1145, %v1131[%v1132] : memref<1xi64>
    cf.br ^af_20
  ^ix_24:
    %v1146 = memref.load %v1140[%v1132] : memref<1xi64>
    call @sloth_rc_release(%v1146) : (i64) -> i64
    cf.br ^ae_22
  ^ae_22:
    call @sloth_rc_release(%v1129) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

