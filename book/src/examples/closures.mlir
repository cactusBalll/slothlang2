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
    %v1012 = arith.constant 1 : i64
    %v1013 = call @sloth_obj_field(%v1009, %v1012) : (i64, i64) -> i64
    %v1014 = llvm.inttoptr %v1011 : i64 to !llvm.ptr
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1004[%v1015] : memref<1xi64>
    %v1017 = llvm.call %v1014(%v1013, %v1016) : !llvm.ptr, (i64, i64) -> i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1001[%v1018] : memref<1xi64>
    %v1019 = arith.constant 1 : i64
    memref.store %v1019, %v1000[%v1018] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1001[%v1020] : memref<1xi64>
    return %v1021 : i64
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
    %v1010 = arith.addi %v1007, %v1009 : i64
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
    %v1009 = arith.constant 1 : i64
    %v1010 = call @sloth_obj_new(%v1006, %v1009) : (i64, i64) -> i64
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1002[%v1011] : memref<1xi64>
    %v1013 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1010, %v1013, %v1012) : (i64, i64, i64) -> i64
    %v1014 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %v1015 = llvm.ptrtoint %v1014 : !llvm.ptr to i64
    %v1016 = call @sloth_rc_retain(%v1010) : (i64) -> i64
    %v1017 = call @sloth_closure_new(%v1015, %v1016) : (i64, i64) -> i64
    %v1018 = arith.constant 0 : index
    memref.store %v1017, %v1001[%v1018] : memref<1xi64>
    %v1019 = arith.constant 1 : i64
    memref.store %v1019, %v1000[%v1018] : memref<1xi64>
    call @sloth_rc_release(%v1010) : (i64) -> i64
    cf.br ^end
  ^end:
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1001[%v1020] : memref<1xi64>
    return %v1021 : i64
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
    %v1008 = arith.muli %v1005, %v1007 : i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1001[%v1009] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    memref.store %v1010, %v1000[%v1009] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    return %v1012 : i64
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
    %v1010 = arith.addi %v1007, %v1009 : i64
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
    %v1010 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    %v1011 = call @sloth_closure_new(%v1009, %v1010) : (i64, i64) -> i64
    %v1012 = memref.alloca() : memref<1xi64>
    %v1013 = call @sloth_rc_retain(%v1011) : (i64) -> i64
    %v1014 = arith.constant 0 : index
    memref.store %v1013, %v1012[%v1014] : memref<1xi64>
    %v1015 = memref.extract_aligned_pointer_as_index %v1012 : memref<1xi64> -> index
    %v1016 = arith.index_cast %v1015 : index to i64
    call @sloth_fiber_track(%v1016) : (i64) -> i64
    call @sloth_rc_release(%v1007) : (i64) -> i64
    call @sloth_rc_release(%v1011) : (i64) -> i64
    %v1017 = arith.constant 6 : i64
    %v1018 = arith.constant 0 : index
    %v1019 = memref.load %v1012[%v1018] : memref<1xi64>
    %v1020 = arith.constant 0 : i64
    %v1021 = call @sloth_obj_field(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = arith.constant 1 : i64
    %v1023 = call @sloth_obj_field(%v1019, %v1022) : (i64, i64) -> i64
    %v1024 = llvm.inttoptr %v1021 : i64 to !llvm.ptr
    %v1025 = arith.constant 6 : i64
    %v1026 = llvm.call %v1024(%v1023, %v1025) : !llvm.ptr, (i64, i64) -> i64
    %v1027 = call @sloth_rt_print_i64(%v1026) : (i64) -> i64
    %v1028 = arith.constant 0 : index
    %v1029 = memref.load %v1012[%v1028] : memref<1xi64>
    %v1030 = arith.constant 5 : i64
    %v1031 = call @sloth_main__apply(%v1029, %v1030) : (i64, i64) -> i64
    %v1032 = call @sloth_rt_print_i64(%v1031) : (i64) -> i64
    %v1033 = arith.constant 10 : i64
    %v1034 = call @sloth_main__make_adder(%v1033) : (i64) -> i64
    %v1035 = memref.alloca() : memref<1xi64>
    %v1036 = arith.constant 0 : index
    memref.store %v1034, %v1035[%v1036] : memref<1xi64>
    %v1037 = memref.extract_aligned_pointer_as_index %v1035 : memref<1xi64> -> index
    %v1038 = arith.index_cast %v1037 : index to i64
    call @sloth_fiber_track(%v1038) : (i64) -> i64
    %v1039 = arith.constant 4 : i64
    %v1040 = arith.constant 0 : index
    %v1041 = memref.load %v1035[%v1040] : memref<1xi64>
    %v1042 = arith.constant 0 : i64
    %v1043 = call @sloth_obj_field(%v1041, %v1042) : (i64, i64) -> i64
    %v1044 = arith.constant 1 : i64
    %v1045 = call @sloth_obj_field(%v1041, %v1044) : (i64, i64) -> i64
    %v1046 = llvm.inttoptr %v1043 : i64 to !llvm.ptr
    %v1047 = arith.constant 4 : i64
    %v1048 = llvm.call %v1046(%v1045, %v1047) : !llvm.ptr, (i64, i64) -> i64
    %v1049 = call @sloth_rt_print_i64(%v1048) : (i64) -> i64
    %v1050 = arith.constant 100 : i64
    %v1051 = memref.alloca() : memref<1xi64>
    %v1052 = arith.constant 0 : index
    memref.store %v1050, %v1051[%v1052] : memref<1xi64>
    %v1053 = arith.constant 0 : i64
    %v1054 = arith.constant 0 : i64
    %v1055 = call @sloth_cls_info(%v1053, %v1054) : (i64, i64) -> i64
    %v1056 = arith.constant 0 : i64
    %v1057 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1055, %v1056, %v1057) : (i64, i64, i64) -> i64
    %v1058 = arith.constant 1 : i64
    %v1059 = call @sloth_obj_new(%v1055, %v1058) : (i64, i64) -> i64
    %v1060 = arith.constant 0 : index
    %v1061 = memref.load %v1051[%v1060] : memref<1xi64>
    %v1062 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1059, %v1062, %v1061) : (i64, i64, i64) -> i64
    %v1063 = llvm.mlir.addressof @sloth_main__clo2 : !llvm.ptr
    %v1064 = llvm.ptrtoint %v1063 : !llvm.ptr to i64
    %v1065 = call @sloth_rc_retain(%v1059) : (i64) -> i64
    %v1066 = call @sloth_closure_new(%v1064, %v1065) : (i64, i64) -> i64
    %v1067 = memref.alloca() : memref<1xi64>
    %v1068 = call @sloth_rc_retain(%v1066) : (i64) -> i64
    %v1069 = arith.constant 0 : index
    memref.store %v1068, %v1067[%v1069] : memref<1xi64>
    %v1070 = memref.extract_aligned_pointer_as_index %v1067 : memref<1xi64> -> index
    %v1071 = arith.index_cast %v1070 : index to i64
    call @sloth_fiber_track(%v1071) : (i64) -> i64
    call @sloth_rc_release(%v1059) : (i64) -> i64
    call @sloth_rc_release(%v1066) : (i64) -> i64
    %v1072 = arith.constant 0 : i64
    %v1073 = arith.constant 0 : index
    memref.store %v1072, %v1051[%v1073] : memref<1xi64>
    %v1074 = arith.constant 1 : i64
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1067[%v1075] : memref<1xi64>
    %v1077 = arith.constant 0 : i64
    %v1078 = call @sloth_obj_field(%v1076, %v1077) : (i64, i64) -> i64
    %v1079 = arith.constant 1 : i64
    %v1080 = call @sloth_obj_field(%v1076, %v1079) : (i64, i64) -> i64
    %v1081 = llvm.inttoptr %v1078 : i64 to !llvm.ptr
    %v1082 = arith.constant 1 : i64
    %v1083 = llvm.call %v1081(%v1080, %v1082) : !llvm.ptr, (i64, i64) -> i64
    %v1084 = call @sloth_rt_print_i64(%v1083) : (i64) -> i64
    %v1085 = arith.constant 0 : index
    %v1086 = memref.load %v1012[%v1085] : memref<1xi64>
    %v1087 = memref.alloca() : memref<1xi64>
    %v1088 = call @sloth_rc_retain(%v1086) : (i64) -> i64
    %v1089 = arith.constant 0 : index
    memref.store %v1088, %v1087[%v1089] : memref<1xi64>
    %v1090 = memref.extract_aligned_pointer_as_index %v1087 : memref<1xi64> -> index
    %v1091 = arith.index_cast %v1090 : index to i64
    call @sloth_fiber_track(%v1091) : (i64) -> i64
    %v1092 = arith.constant 3 : i64
    %v1093 = arith.constant 0 : index
    %v1094 = memref.load %v1087[%v1093] : memref<1xi64>
    %v1095 = arith.constant 0 : i64
    %v1096 = call @sloth_obj_field(%v1094, %v1095) : (i64, i64) -> i64
    %v1097 = arith.constant 1 : i64
    %v1098 = call @sloth_obj_field(%v1094, %v1097) : (i64, i64) -> i64
    %v1099 = llvm.inttoptr %v1096 : i64 to !llvm.ptr
    %v1100 = arith.constant 3 : i64
    %v1101 = llvm.call %v1099(%v1098, %v1100) : !llvm.ptr, (i64, i64) -> i64
    %v1102 = call @sloth_rt_print_i64(%v1101) : (i64) -> i64
    %v1103 = arith.constant 0 : index
    %v1104 = memref.load %v1067[%v1103] : memref<1xi64>
    call @sloth_rc_release(%v1104) : (i64) -> i64
    %v1105 = memref.extract_aligned_pointer_as_index %v1067 : memref<1xi64> -> index
    %v1106 = arith.index_cast %v1105 : index to i64
    call @sloth_fiber_untrack(%v1106) : (i64) -> i64
    %v1107 = arith.constant 0 : index
    %v1108 = memref.load %v1035[%v1107] : memref<1xi64>
    call @sloth_rc_release(%v1108) : (i64) -> i64
    %v1109 = memref.extract_aligned_pointer_as_index %v1035 : memref<1xi64> -> index
    %v1110 = arith.index_cast %v1109 : index to i64
    call @sloth_fiber_untrack(%v1110) : (i64) -> i64
    %v1111 = arith.constant 0 : index
    %v1112 = memref.load %v1012[%v1111] : memref<1xi64>
    call @sloth_rc_release(%v1112) : (i64) -> i64
    %v1113 = memref.extract_aligned_pointer_as_index %v1012 : memref<1xi64> -> index
    %v1114 = arith.index_cast %v1113 : index to i64
    call @sloth_fiber_untrack(%v1114) : (i64) -> i64
    %v1115 = arith.constant 0 : index
    %v1116 = memref.load %v1087[%v1115] : memref<1xi64>
    call @sloth_rc_release(%v1116) : (i64) -> i64
    %v1117 = memref.extract_aligned_pointer_as_index %v1087 : memref<1xi64> -> index
    %v1118 = arith.index_cast %v1117 : index to i64
    call @sloth_fiber_untrack(%v1118) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

