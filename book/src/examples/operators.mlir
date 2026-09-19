module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__double(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 2 : i64
    %v1007 = arith.muli %v1005, %v1006 : i64
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
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 7 : i64
    %v1002 = arith.constant 2 : i64
    %v1003 = memref.alloca() : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.cmpi eq, %v1002, %v1004 : i64
    %v1006 = arith.extui %v1005 : i1 to i64
    %v1008 = arith.constant 0 : i64
    %v1007 = arith.cmpi ne, %v1006, %v1008 : i64
    cf.cond_br %v1007, ^dz_1, ^dz_2
  ^dz_1:
    call @sloth_panic_divzero() : () -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1004, %v1003[%v1009] : memref<1xi64>
    cf.br ^dz_3
  ^dz_2:
    %v1010 = arith.divsi %v1001, %v1002 : i64
    %v1011 = arith.constant 0 : index
    memref.store %v1010, %v1003[%v1011] : memref<1xi64>
    cf.br ^dz_3
  ^dz_3:
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1003[%v1012] : memref<1xi64>
    %v1014 = call @sloth_rt_print_i64(%v1013) : (i64) -> i64
    %v1015 = arith.constant 7 : i64
    %v1016 = arith.constant 3 : i64
    %v1017 = memref.alloca() : memref<1xi64>
    %v1018 = arith.constant 0 : i64
    %v1019 = arith.cmpi eq, %v1016, %v1018 : i64
    %v1020 = arith.extui %v1019 : i1 to i64
    %v1022 = arith.constant 0 : i64
    %v1021 = arith.cmpi ne, %v1020, %v1022 : i64
    cf.cond_br %v1021, ^dz_4, ^dz_5
  ^dz_4:
    call @sloth_panic_divzero() : () -> i64
    %v1023 = arith.constant 0 : index
    memref.store %v1018, %v1017[%v1023] : memref<1xi64>
    cf.br ^dz_6
  ^dz_5:
    %v1024 = arith.remsi %v1015, %v1016 : i64
    %v1025 = arith.constant 0 : index
    memref.store %v1024, %v1017[%v1025] : memref<1xi64>
    cf.br ^dz_6
  ^dz_6:
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1017[%v1026] : memref<1xi64>
    %v1028 = call @sloth_rt_print_i64(%v1027) : (i64) -> i64
    %v1029 = arith.constant 2 : i64
    %v1030 = arith.constant 3 : i64
    %v1031 = arith.constant 4 : i64
    %v1032 = arith.muli %v1030, %v1031 : i64
    %v1033 = arith.addi %v1029, %v1032 : i64
    %v1034 = call @sloth_rt_print_i64(%v1033) : (i64) -> i64
    %v1035 = arith.constant 2 : i64
    %v1036 = arith.constant 3 : i64
    %v1037 = arith.addi %v1035, %v1036 : i64
    %v1038 = arith.constant 4 : i64
    %v1039 = arith.muli %v1037, %v1038 : i64
    %v1040 = call @sloth_rt_print_i64(%v1039) : (i64) -> i64
    %v1041 = arith.constant 0 : i64
    %v1043 = arith.constant 1 : i64
    %v1044 = arith.subi %v1043, %v1041 : i64
    %v1045 = call @sloth_rt_print_bool(%v1044) : (i64) -> i64
    %v1046 = arith.constant 1 : i64
    %v1047 = arith.constant 2 : i64
    %v1049 = arith.constant 0 : i64
    %v1048 = arith.cmpi slt, %v1046, %v1047 : i64
    %v1050 = arith.extui %v1048 : i1 to i64
    %v1051 = memref.alloca() : memref<1xi64>
    %v1052 = arith.constant 0 : index
    memref.store %v1050, %v1051[%v1052] : memref<1xi64>
    %v1053 = arith.constant 0 : i64
    %v1054 = arith.cmpi ne, %v1050, %v1053 : i64
    %v1055 = arith.extui %v1054 : i1 to i64
    %v1057 = arith.constant 0 : i64
    %v1056 = arith.cmpi ne, %v1055, %v1057 : i64
    cf.cond_br %v1056, ^sc_7, ^sc_8
  ^sc_7:
    %v1058 = arith.constant 2 : i64
    %v1059 = arith.constant 3 : i64
    %v1061 = arith.constant 0 : i64
    %v1060 = arith.cmpi slt, %v1058, %v1059 : i64
    %v1062 = arith.extui %v1060 : i1 to i64
    %v1063 = arith.constant 0 : index
    memref.store %v1062, %v1051[%v1063] : memref<1xi64>
    cf.br ^sc_8
  ^sc_8:
    %v1064 = arith.constant 0 : index
    %v1065 = memref.load %v1051[%v1064] : memref<1xi64>
    %v1066 = call @sloth_rt_print_bool(%v1065) : (i64) -> i64
    %v1067 = arith.constant 1 : i64
    %v1068 = arith.constant 2 : i64
    %v1070 = arith.constant 0 : i64
    %v1069 = arith.cmpi sgt, %v1067, %v1068 : i64
    %v1071 = arith.extui %v1069 : i1 to i64
    %v1072 = memref.alloca() : memref<1xi64>
    %v1073 = arith.constant 0 : index
    memref.store %v1071, %v1072[%v1073] : memref<1xi64>
    %v1074 = arith.constant 0 : i64
    %v1075 = arith.cmpi ne, %v1071, %v1074 : i64
    %v1076 = arith.extui %v1075 : i1 to i64
    %v1078 = arith.constant 0 : i64
    %v1077 = arith.cmpi ne, %v1076, %v1078 : i64
    cf.cond_br %v1077, ^sc_10, ^sc_9
  ^sc_9:
    %v1079 = arith.constant 2 : i64
    %v1080 = arith.constant 1 : i64
    %v1082 = arith.constant 0 : i64
    %v1081 = arith.cmpi sgt, %v1079, %v1080 : i64
    %v1083 = arith.extui %v1081 : i1 to i64
    %v1084 = arith.constant 0 : index
    memref.store %v1083, %v1072[%v1084] : memref<1xi64>
    cf.br ^sc_10
  ^sc_10:
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1072[%v1085] : memref<1xi64>
    %v1087 = call @sloth_rt_print_bool(%v1086) : (i64) -> i64
    %v1088 = arith.constant 5 : i64
    %v1089 = call @sloth_main__double(%v1088) : (i64) -> i64
    %v1090 = call @sloth_rt_print_i64(%v1089) : (i64) -> i64
    %v1091 = arith.constant 2 : i64
    %v1092 = call @sloth_main__double(%v1091) : (i64) -> i64
    %v1093 = call @sloth_main__double(%v1092) : (i64) -> i64
    %v1094 = call @sloth_rt_print_i64(%v1093) : (i64) -> i64
    %v1095 = arith.constant 0 : i64
    %v1096 = memref.alloca() : memref<1xi64>
    %v1097 = call @sloth_rc_retain(%v1095) : (i64) -> i64
    %v1098 = arith.constant 0 : index
    memref.store %v1097, %v1096[%v1098] : memref<1xi64>
    %v1099 = memref.extract_aligned_pointer_as_index %v1096 : memref<1xi64> -> index
    %v1100 = arith.index_cast %v1099 : index to i64
    call @sloth_fiber_track(%v1100) : (i64) -> i64
    %v1101 = arith.constant 0 : index
    %v1102 = memref.load %v1096[%v1101] : memref<1xi64>
    %v1103 = arith.constant 7 : i64
    %v1104 = call @sloth_box_get(%v1102) : (i64) -> i64
    %v1105 = arith.constant 0 : i64
    %v1106 = arith.cmpi ne, %v1102, %v1105 : i64
    %v1107 = arith.select %v1106, %v1104, %v1103 : i64
    %v1108 = call @sloth_rt_print_i64(%v1107) : (i64) -> i64
    %v1109 = arith.constant 3 : i64
    %v1110 = arith.constant 1 : i64
    %v1111 = call @sloth_rt_print_bool(%v1110) : (i64) -> i64
    %v1112 = arith.constant 0 : i64
    %v1113 = arith.constant 120 : i64
    %v1114 = arith.constant 1 : i64
    %v1115 = call @sloth_str_push(%v1112, %v1113, %v1114) : (i64, i64, i64) -> i64
    %v1116 = call @sloth_str_finish(%v1115) : (i64) -> i64
    %v1117 = arith.constant 0 : i64
    %v1118 = call @sloth_rt_print_bool(%v1117) : (i64) -> i64
    call @sloth_rc_release(%v1116) : (i64) -> i64
    %v1119 = arith.constant 0 : i64
    %v1120 = memref.alloca() : memref<1xi64>
    %v1121 = arith.constant 0 : index
    memref.store %v1119, %v1120[%v1121] : memref<1xi64>
    %v1122 = arith.constant 0 : i64
    %v1123 = arith.constant 3 : i64
    %v1125 = arith.constant 0 : index
    %v1124 = memref.alloca() : memref<1xi64>
    memref.store %v1122, %v1124[%v1125] : memref<1xi64>
    cf.br ^fr_11
  ^fr_11:
    %v1126 = memref.load %v1124[%v1125] : memref<1xi64>
    %v1127 = arith.cmpi slt, %v1126, %v1123 : i64
    %v1128 = arith.extui %v1127 : i1 to i64
    %v1130 = arith.constant 0 : i64
    %v1129 = arith.cmpi ne, %v1128, %v1130 : i64
    cf.cond_br %v1129, ^fb_12, ^fd_13
  ^fb_12:
    %v1131 = memref.alloca() : memref<1xi64>
    memref.store %v1126, %v1131[%v1125] : memref<1xi64>
    %v1132 = arith.constant 0 : index
    %v1133 = memref.load %v1120[%v1132] : memref<1xi64>
    %v1134 = arith.constant 0 : index
    %v1135 = memref.load %v1131[%v1134] : memref<1xi64>
    %v1136 = arith.addi %v1133, %v1135 : i64
    %v1137 = arith.constant 0 : index
    memref.store %v1136, %v1120[%v1137] : memref<1xi64>
    cf.br ^fc_14
  ^fc_14:
    %v1138 = arith.constant 1 : i64
    %v1139 = arith.addi %v1126, %v1138 : i64
    memref.store %v1139, %v1124[%v1125] : memref<1xi64>
    cf.br ^fr_11
  ^fd_13:
    %v1140 = arith.constant 0 : index
    %v1141 = memref.load %v1120[%v1140] : memref<1xi64>
    %v1142 = call @sloth_rt_print_i64(%v1141) : (i64) -> i64
    %v1143 = arith.constant 0 : index
    %v1144 = memref.load %v1096[%v1143] : memref<1xi64>
    call @sloth_rc_release(%v1144) : (i64) -> i64
    %v1145 = memref.extract_aligned_pointer_as_index %v1096 : memref<1xi64> -> index
    %v1146 = arith.index_cast %v1145 : index to i64
    call @sloth_fiber_untrack(%v1146) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

