module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__apply(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1004[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = arith.constant 0 : i64
    %v1011 = call @sloth_obj_field(%v1009, %v1010) : (i64, i64) -> i64
    %v1012 = arith.constant 2 : i64
    %v1013 = call @sloth_obj_field(%v1009, %v1012) : (i64, i64) -> i64
    %v1014 = arith.constant -2 : i64
    %v1015 = arith.andi %v1011, %v1014 : i64
    %v1016 = llvm.inttoptr %v1015 : i64 to !llvm.ptr
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1004[%v1017] : memref<1xi64>
    %v1019 = llvm.call %v1016(%v1013, %v1018) : !llvm.ptr, (i64, i64) -> i64
    %v1020 = arith.constant 0 : index
    memref.store %v1019, %v1001[%v1020] : memref<1xi64>
    %v1021 = arith.constant 1 : i64
    memref.store %v1021, %v1000[%v1020] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1001[%v1022] : memref<1xi64>
    return %v1023 : i64
  }
  func.func @sloth_main__lam1(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1004[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shrsi %v1007, %v1010 : i64
    %v1012 = arith.constant 1 : i64
    %v1013 = arith.shrsi %v1009, %v1012 : i64
    %v1014 = arith.addi %v1011, %v1013 : i64
    %v1015 = arith.constant 1 : i64
    %v1016 = arith.shli %v1014, %v1015 : i64
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
  llvm.func @sloth_main__clo0(%p0: i64, %p1: i64) -> i64 {
    %v1 = arith.constant 0 : i64
    %v2 = func.call @sloth_obj_field(%p0, %v1) : (i64, i64) -> i64
    %v3 = func.call @sloth_main__lam1(%v2, %p1) : (i64, i64) -> i64
    llvm.return %v3 : i64
}
  func.func @sloth_main__make_adder(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 0 : i64
    %v1006 = call @sloth_cls_info(%v1004, %v1005) : (i64, i64) -> i64
    %v1007 = arith.constant 0 : i64
    %v1008 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1006, %v1007, %v1008) : (i64, i64, i64) -> i64
    %v1009 = arith.constant 2 : i64
    %v1010 = call @sloth_obj_new(%v1006, %v1009) : (i64, i64) -> i64
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1002[%v1011] : memref<1xi64>
    %v1013 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1010, %v1013, %v1012) : (i64, i64, i64) -> i64
    %v1014 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %v1015 = llvm.ptrtoint %v1014 : !llvm.ptr to i64
    %v1016 = arith.constant 1 : i64
    %v1017 = arith.ori %v1015, %v1016 : i64
    %v1018 = call @sloth_rc_retain(%v1010) : (i64) -> i64
    %v1019 = call @sloth_closure_new(%v1017, %v1018) : (i64, i64) -> i64
    %v1020 = arith.constant 0 : index
    memref.store %v1019, %v1001[%v1020] : memref<1xi64>
    %v1021 = arith.constant 1 : i64
    memref.store %v1021, %v1000[%v1020] : memref<1xi64>
    call @sloth_rc_release(%v1010) : (i64) -> i64
    cf.br ^end
  ^end:
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1001[%v1022] : memref<1xi64>
    return %v1023 : i64
  }
  func.func @sloth_main__lam2(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1002[%v1006] : memref<1xi64>
    %v1008 = arith.constant 1 : i64
    %v1009 = arith.shrsi %v1005, %v1008 : i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shrsi %v1007, %v1010 : i64
    %v1012 = arith.muli %v1009, %v1011 : i64
    %v1013 = arith.constant 1 : i64
    %v1014 = arith.shli %v1012, %v1013 : i64
    %v1015 = arith.constant 0 : index
    memref.store %v1014, %v1001[%v1015] : memref<1xi64>
    %v1016 = arith.constant 1 : i64
    memref.store %v1016, %v1000[%v1015] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1001[%v1017] : memref<1xi64>
    return %v1018 : i64
  }
  llvm.func @sloth_main__clo1(%p0: i64, %p1: i64) -> i64 {
    %v1 = func.call @sloth_main__lam2(%p1) : (i64) -> i64
    llvm.return %v1 : i64
}
  func.func @sloth_main__lam3(%p0: i64, %p1: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1004 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1004[%v1005] : memref<1xi64>
    %v1006 = arith.constant 0 : index
    %v1007 = memref.load %v1004[%v1006] : memref<1xi64>
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shrsi %v1007, %v1010 : i64
    %v1012 = arith.constant 1 : i64
    %v1013 = arith.shrsi %v1009, %v1012 : i64
    %v1014 = arith.addi %v1011, %v1013 : i64
    %v1015 = arith.constant 1 : i64
    %v1016 = arith.shli %v1014, %v1015 : i64
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
  llvm.func @sloth_main__clo2(%p0: i64, %p1: i64) -> i64 {
    %v1 = arith.constant 0 : i64
    %v2 = func.call @sloth_obj_field(%p0, %v1) : (i64, i64) -> i64
    %v3 = func.call @sloth_main__lam3(%v2, %p1) : (i64, i64) -> i64
    llvm.return %v3 : i64
}
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 0 : i64
    %v1003 = call @sloth_cls_info(%v1001, %v1002) : (i64, i64) -> i64
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 0 : i64
    call @sloth_cls_refmask(%v1003, %v1004, %v1005) : (i64, i64, i64) -> i64
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_new(%v1003, %v1006) : (i64, i64) -> i64
    %v1008 = llvm.mlir.addressof @sloth_main__clo1 : !llvm.ptr
    %v1009 = llvm.ptrtoint %v1008 : !llvm.ptr to i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.ori %v1009, %v1010 : i64
    %v1012 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    %v1013 = call @sloth_closure_new(%v1011, %v1012) : (i64, i64) -> i64
    %v1014 = memref.alloca() : memref<1xi64>
    %v1015 = call @sloth_rc_retain(%v1013) : (i64) -> i64
    %v1016 = arith.constant 0 : index
    memref.store %v1015, %v1014[%v1016] : memref<1xi64>
    call @sloth_rc_release(%v1007) : (i64) -> i64
    call @sloth_rc_release(%v1013) : (i64) -> i64
    %v1017 = arith.constant 12 : i64
    %v1018 = arith.constant 0 : index
    %v1019 = memref.load %v1014[%v1018] : memref<1xi64>
    %v1020 = arith.constant 0 : i64
    %v1021 = call @sloth_obj_field(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = arith.constant 2 : i64
    %v1023 = call @sloth_obj_field(%v1019, %v1022) : (i64, i64) -> i64
    %v1024 = arith.constant -2 : i64
    %v1025 = arith.andi %v1021, %v1024 : i64
    %v1026 = llvm.inttoptr %v1025 : i64 to !llvm.ptr
    %v1027 = arith.constant 12 : i64
    %v1028 = llvm.call %v1026(%v1023, %v1027) : !llvm.ptr, (i64, i64) -> i64
    %v1029 = call @sloth_rt_print_i64(%v1028) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    %v1031 = memref.load %v1014[%v1030] : memref<1xi64>
    %v1032 = arith.constant 10 : i64
    %v1033 = call @sloth_main__apply(%v1031, %v1032) : (i64, i64) -> i64
    %v1034 = call @sloth_rt_print_i64(%v1033) : (i64) -> i64
    %v1035 = arith.constant 20 : i64
    %v1036 = call @sloth_main__make_adder(%v1035) : (i64) -> i64
    %v1037 = memref.alloca() : memref<1xi64>
    %v1038 = arith.constant 0 : index
    memref.store %v1036, %v1037[%v1038] : memref<1xi64>
    %v1039 = arith.constant 8 : i64
    %v1040 = arith.constant 0 : index
    %v1041 = memref.load %v1037[%v1040] : memref<1xi64>
    %v1042 = arith.constant 0 : i64
    %v1043 = call @sloth_obj_field(%v1041, %v1042) : (i64, i64) -> i64
    %v1044 = arith.constant 2 : i64
    %v1045 = call @sloth_obj_field(%v1041, %v1044) : (i64, i64) -> i64
    %v1046 = arith.constant -2 : i64
    %v1047 = arith.andi %v1043, %v1046 : i64
    %v1048 = llvm.inttoptr %v1047 : i64 to !llvm.ptr
    %v1049 = arith.constant 8 : i64
    %v1050 = llvm.call %v1048(%v1045, %v1049) : !llvm.ptr, (i64, i64) -> i64
    %v1051 = call @sloth_rt_print_i64(%v1050) : (i64) -> i64
    %v1052 = arith.constant 200 : i64
    %v1053 = memref.alloca() : memref<1xi64>
    %v1054 = arith.constant 0 : index
    memref.store %v1052, %v1053[%v1054] : memref<1xi64>
    %v1055 = arith.constant 0 : i64
    %v1056 = arith.constant 0 : i64
    %v1057 = call @sloth_cls_info(%v1055, %v1056) : (i64, i64) -> i64
    %v1058 = arith.constant 0 : i64
    %v1059 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1057, %v1058, %v1059) : (i64, i64, i64) -> i64
    %v1060 = arith.constant 2 : i64
    %v1061 = call @sloth_obj_new(%v1057, %v1060) : (i64, i64) -> i64
    %v1062 = arith.constant 0 : index
    %v1063 = memref.load %v1053[%v1062] : memref<1xi64>
    %v1064 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1061, %v1064, %v1063) : (i64, i64, i64) -> i64
    %v1065 = llvm.mlir.addressof @sloth_main__clo2 : !llvm.ptr
    %v1066 = llvm.ptrtoint %v1065 : !llvm.ptr to i64
    %v1067 = arith.constant 1 : i64
    %v1068 = arith.ori %v1066, %v1067 : i64
    %v1069 = call @sloth_rc_retain(%v1061) : (i64) -> i64
    %v1070 = call @sloth_closure_new(%v1068, %v1069) : (i64, i64) -> i64
    %v1071 = memref.alloca() : memref<1xi64>
    %v1072 = call @sloth_rc_retain(%v1070) : (i64) -> i64
    %v1073 = arith.constant 0 : index
    memref.store %v1072, %v1071[%v1073] : memref<1xi64>
    call @sloth_rc_release(%v1061) : (i64) -> i64
    call @sloth_rc_release(%v1070) : (i64) -> i64
    %v1074 = arith.constant 0 : i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1053[%v1075] : memref<1xi64>
    call @sloth_rc_release(%v1076) : (i64) -> i64
    %v1077 = call @sloth_rc_retain(%v1074) : (i64) -> i64
    %v1078 = arith.constant 0 : index
    memref.store %v1077, %v1053[%v1078] : memref<1xi64>
    %v1079 = arith.constant 2 : i64
    %v1080 = arith.constant 0 : index
    %v1081 = memref.load %v1071[%v1080] : memref<1xi64>
    %v1082 = arith.constant 0 : i64
    %v1083 = call @sloth_obj_field(%v1081, %v1082) : (i64, i64) -> i64
    %v1084 = arith.constant 2 : i64
    %v1085 = call @sloth_obj_field(%v1081, %v1084) : (i64, i64) -> i64
    %v1086 = arith.constant -2 : i64
    %v1087 = arith.andi %v1083, %v1086 : i64
    %v1088 = llvm.inttoptr %v1087 : i64 to !llvm.ptr
    %v1089 = arith.constant 2 : i64
    %v1090 = llvm.call %v1088(%v1085, %v1089) : !llvm.ptr, (i64, i64) -> i64
    %v1091 = call @sloth_rt_print_i64(%v1090) : (i64) -> i64
    %v1092 = arith.constant 0 : index
    %v1093 = memref.load %v1014[%v1092] : memref<1xi64>
    %v1094 = memref.alloca() : memref<1xi64>
    %v1095 = call @sloth_rc_retain(%v1093) : (i64) -> i64
    %v1096 = arith.constant 0 : index
    memref.store %v1095, %v1094[%v1096] : memref<1xi64>
    %v1097 = arith.constant 6 : i64
    %v1098 = arith.constant 0 : index
    %v1099 = memref.load %v1094[%v1098] : memref<1xi64>
    %v1100 = arith.constant 0 : i64
    %v1101 = call @sloth_obj_field(%v1099, %v1100) : (i64, i64) -> i64
    %v1102 = arith.constant 2 : i64
    %v1103 = call @sloth_obj_field(%v1099, %v1102) : (i64, i64) -> i64
    %v1104 = arith.constant -2 : i64
    %v1105 = arith.andi %v1101, %v1104 : i64
    %v1106 = llvm.inttoptr %v1105 : i64 to !llvm.ptr
    %v1107 = arith.constant 6 : i64
    %v1108 = llvm.call %v1106(%v1103, %v1107) : !llvm.ptr, (i64, i64) -> i64
    %v1109 = call @sloth_rt_print_i64(%v1108) : (i64) -> i64
    %v1110 = arith.constant 0 : index
    %v1111 = memref.load %v1014[%v1110] : memref<1xi64>
    call @sloth_rc_release(%v1111) : (i64) -> i64
    %v1112 = arith.constant 0 : index
    %v1113 = memref.load %v1037[%v1112] : memref<1xi64>
    call @sloth_rc_release(%v1113) : (i64) -> i64
    %v1114 = arith.constant 0 : index
    %v1115 = memref.load %v1071[%v1114] : memref<1xi64>
    call @sloth_rc_release(%v1115) : (i64) -> i64
    %v1116 = arith.constant 0 : index
    %v1117 = memref.load %v1094[%v1116] : memref<1xi64>
    call @sloth_rc_release(%v1117) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

