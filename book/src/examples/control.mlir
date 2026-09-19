module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 3 : i64
    %v1002 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    memref.store %v1001, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 2 : i64
    %v1008 = arith.constant 0 : i64
    %v1007 = arith.cmpi sgt, %v1005, %v1006 : i64
    %v1009 = arith.extui %v1007 : i1 to i64
    %v1011 = arith.constant 0 : i64
    %v1010 = arith.cmpi ne, %v1009, %v1011 : i64
    cf.cond_br %v1010, ^t_1, ^e_2
  ^t_1:
    %v1012 = arith.constant 0 : i64
    %v1013 = arith.constant 6777186 : i64
    %v1014 = arith.constant 3 : i64
    %v1015 = call @sloth_str_push(%v1012, %v1013, %v1014) : (i64, i64, i64) -> i64
    %v1016 = call @sloth_str_finish(%v1015) : (i64) -> i64
    %v1017 = call @sloth_rt_print_str(%v1016) : (i64) -> i64
    call @sloth_rc_release(%v1016) : (i64) -> i64
    cf.br ^fi_3
  ^e_2:
    %v1018 = arith.constant 0 : i64
    %v1019 = arith.constant 465674792307 : i64
    %v1020 = arith.constant 5 : i64
    %v1021 = call @sloth_str_push(%v1018, %v1019, %v1020) : (i64, i64, i64) -> i64
    %v1022 = call @sloth_str_finish(%v1021) : (i64) -> i64
    %v1023 = call @sloth_rt_print_str(%v1022) : (i64) -> i64
    call @sloth_rc_release(%v1022) : (i64) -> i64
    cf.br ^fi_3
  ^fi_3:
    %v1024 = arith.constant 0 : index
    %v1025 = memref.load %v1002[%v1024] : memref<1xi64>
    %v1026 = arith.constant 3 : i64
    %v1028 = arith.constant 0 : i64
    %v1027 = arith.cmpi eq, %v1025, %v1026 : i64
    %v1029 = arith.extui %v1027 : i1 to i64
    %v1031 = arith.constant 0 : i64
    %v1030 = arith.cmpi ne, %v1029, %v1031 : i64
    cf.cond_br %v1030, ^t_4, ^e_5
  ^t_4:
    %v1032 = arith.constant 0 : i64
    %v1033 = arith.constant 435493693556 : i64
    %v1034 = arith.constant 5 : i64
    %v1035 = call @sloth_str_push(%v1032, %v1033, %v1034) : (i64, i64, i64) -> i64
    %v1036 = call @sloth_str_finish(%v1035) : (i64) -> i64
    %v1037 = call @sloth_rt_print_str(%v1036) : (i64) -> i64
    call @sloth_rc_release(%v1036) : (i64) -> i64
    cf.br ^fi_6
  ^e_5:
    cf.br ^fi_6
  ^fi_6:
    %v1038 = arith.constant 0 : i64
    %v1039 = memref.alloca() : memref<1xi64>
    %v1040 = arith.constant 0 : index
    memref.store %v1038, %v1039[%v1040] : memref<1xi64>
    cf.br ^wh_7
  ^wh_7:
    %v1041 = arith.constant 0 : index
    %v1042 = memref.load %v1039[%v1041] : memref<1xi64>
    %v1043 = arith.constant 3 : i64
    %v1045 = arith.constant 0 : i64
    %v1044 = arith.cmpi slt, %v1042, %v1043 : i64
    %v1046 = arith.extui %v1044 : i1 to i64
    %v1048 = arith.constant 0 : i64
    %v1047 = arith.cmpi ne, %v1046, %v1048 : i64
    cf.cond_br %v1047, ^do_8, ^wd_9
  ^do_8:
    %v1049 = arith.constant 0 : index
    %v1050 = memref.load %v1039[%v1049] : memref<1xi64>
    %v1051 = call @sloth_rt_print_i64(%v1050) : (i64) -> i64
    %v1052 = arith.constant 0 : index
    %v1053 = memref.load %v1039[%v1052] : memref<1xi64>
    %v1054 = arith.constant 1 : i64
    %v1055 = arith.addi %v1053, %v1054 : i64
    %v1056 = arith.constant 0 : index
    memref.store %v1055, %v1039[%v1056] : memref<1xi64>
    cf.br ^wh_7
  ^wd_9:
    %v1057 = arith.constant 0 : i64
    %v1058 = memref.alloca() : memref<1xi64>
    %v1059 = arith.constant 0 : index
    memref.store %v1057, %v1058[%v1059] : memref<1xi64>
    %v1060 = arith.constant 0 : i64
    %v1061 = arith.constant 5 : i64
    %v1063 = arith.constant 0 : index
    %v1062 = memref.alloca() : memref<1xi64>
    memref.store %v1060, %v1062[%v1063] : memref<1xi64>
    cf.br ^fr_10
  ^fr_10:
    %v1064 = memref.load %v1062[%v1063] : memref<1xi64>
    %v1065 = arith.cmpi slt, %v1064, %v1061 : i64
    %v1066 = arith.extui %v1065 : i1 to i64
    %v1068 = arith.constant 0 : i64
    %v1067 = arith.cmpi ne, %v1066, %v1068 : i64
    cf.cond_br %v1067, ^fb_11, ^fd_12
  ^fb_11:
    %v1069 = memref.alloca() : memref<1xi64>
    memref.store %v1064, %v1069[%v1063] : memref<1xi64>
    %v1070 = arith.constant 0 : index
    %v1071 = memref.load %v1069[%v1070] : memref<1xi64>
    %v1072 = arith.constant 1 : i64
    %v1074 = arith.constant 0 : i64
    %v1073 = arith.cmpi eq, %v1071, %v1072 : i64
    %v1075 = arith.extui %v1073 : i1 to i64
    %v1077 = arith.constant 0 : i64
    %v1076 = arith.cmpi ne, %v1075, %v1077 : i64
    cf.cond_br %v1076, ^t_14, ^e_15
  ^t_14:
    cf.br ^fc_13
  ^e_15:
    cf.br ^fi_16
  ^fi_16:
    %v1078 = arith.constant 0 : index
    %v1079 = memref.load %v1069[%v1078] : memref<1xi64>
    %v1080 = arith.constant 4 : i64
    %v1082 = arith.constant 0 : i64
    %v1081 = arith.cmpi eq, %v1079, %v1080 : i64
    %v1083 = arith.extui %v1081 : i1 to i64
    %v1085 = arith.constant 0 : i64
    %v1084 = arith.cmpi ne, %v1083, %v1085 : i64
    cf.cond_br %v1084, ^t_17, ^e_18
  ^t_17:
    cf.br ^fd_12
  ^e_18:
    cf.br ^fi_19
  ^fi_19:
    %v1086 = arith.constant 0 : index
    %v1087 = memref.load %v1058[%v1086] : memref<1xi64>
    %v1088 = arith.constant 0 : index
    %v1089 = memref.load %v1069[%v1088] : memref<1xi64>
    %v1090 = arith.addi %v1087, %v1089 : i64
    %v1091 = arith.constant 0 : index
    memref.store %v1090, %v1058[%v1091] : memref<1xi64>
    cf.br ^fc_13
  ^fc_13:
    %v1092 = arith.constant 1 : i64
    %v1093 = arith.addi %v1064, %v1092 : i64
    memref.store %v1093, %v1062[%v1063] : memref<1xi64>
    cf.br ^fr_10
  ^fd_12:
    %v1094 = arith.constant 0 : index
    %v1095 = memref.load %v1058[%v1094] : memref<1xi64>
    %v1096 = call @sloth_rt_print_i64(%v1095) : (i64) -> i64
    %v1097 = arith.constant 0 : i64
    %v1098 = arith.constant 25185 : i64
    %v1099 = arith.constant 2 : i64
    %v1100 = call @sloth_str_push(%v1097, %v1098, %v1099) : (i64, i64, i64) -> i64
    %v1101 = call @sloth_str_finish(%v1100) : (i64) -> i64
    %v1102 = call @sloth_str_clen(%v1101) : (i64) -> i64
    %v1104 = arith.constant 0 : index
    %v1103 = memref.alloca() : memref<1xi64>
    %v1105 = arith.constant 0 : i64
    memref.store %v1105, %v1103[%v1104] : memref<1xi64>
    cf.br ^af_20
  ^af_20:
    %v1106 = memref.load %v1103[%v1104] : memref<1xi64>
    %v1107 = arith.cmpi slt, %v1106, %v1102 : i64
    %v1108 = arith.extui %v1107 : i1 to i64
    %v1110 = arith.constant 0 : i64
    %v1109 = arith.cmpi ne, %v1108, %v1110 : i64
    cf.cond_br %v1109, ^ab_21, ^ae_22
  ^ab_21:
    %v1111 = call @sloth_str_char(%v1101, %v1106) : (i64, i64) -> i64
    %v1112 = memref.alloca() : memref<1xi64>
    memref.store %v1111, %v1112[%v1104] : memref<1xi64>
    %v1113 = arith.constant 0 : index
    %v1114 = memref.load %v1112[%v1113] : memref<1xi64>
    %v1115 = call @sloth_rt_print_str(%v1114) : (i64) -> i64
    cf.br ^ic_23
  ^ic_23:
    call @sloth_rc_release(%v1111) : (i64) -> i64
    %v1116 = arith.constant 1 : i64
    %v1117 = arith.addi %v1106, %v1116 : i64
    memref.store %v1117, %v1103[%v1104] : memref<1xi64>
    cf.br ^af_20
  ^ix_24:
    %v1118 = memref.load %v1112[%v1104] : memref<1xi64>
    call @sloth_rc_release(%v1118) : (i64) -> i64
    cf.br ^ae_22
  ^ae_22:
    call @sloth_rc_release(%v1101) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

