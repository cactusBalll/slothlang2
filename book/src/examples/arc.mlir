module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__churn(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = memref.alloca() : memref<1xi64>
    %v1006 = arith.constant 0 : index
    memref.store %v1004, %v1005[%v1006] : memref<1xi64>
    %v1007 = arith.constant 0 : i64
    %v1008 = memref.alloca() : memref<1xi64>
    %v1009 = arith.constant 0 : index
    memref.store %v1007, %v1008[%v1009] : memref<1xi64>
    cf.br ^wh_1
  ^wh_1:
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1008[%v1010] : memref<1xi64>
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1002[%v1012] : memref<1xi64>
    %v1015 = arith.constant 0 : i64
    %v1014 = arith.cmpi slt, %v1011, %v1013 : i64
    %v1016 = arith.extui %v1014 : i1 to i64
    %v1017 = arith.constant 1 : i64
    %v1018 = arith.shli %v1016, %v1017 : i64
    %v1020 = arith.constant 0 : i64
    %v1019 = arith.cmpi ne, %v1018, %v1020 : i64
    cf.cond_br %v1019, ^do_2, ^wd_3
  ^do_2:
    %v1021 = arith.constant 0 : i64
    %v1022 = arith.constant 4 : i64
    %v1023 = call @sloth_cls_info(%v1021, %v1022) : (i64, i64) -> i64
    %v1024 = arith.constant 3 : i64
    %v1025 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1023, %v1024, %v1025) : (i64, i64, i64) -> i64
    %v1026 = arith.constant 4 : i64
    %v1027 = call @sloth_obj_new(%v1023, %v1026) : (i64, i64) -> i64
    call @sloth_main_Holder____init__(%v1027) : (i64) -> ()
    %v1028 = memref.alloca() : memref<1xi64>
    %v1029 = call @sloth_rc_retain(%v1027) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    memref.store %v1029, %v1028[%v1030] : memref<1xi64>
    call @sloth_rc_release(%v1027) : (i64) -> i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1005[%v1031] : memref<1xi64>
    %v1033 = arith.constant 0 : index
    %v1034 = memref.load %v1028[%v1033] : memref<1xi64>
    %v1035 = arith.constant 0 : i64
    %v1036 = call @sloth_obj_field(%v1034, %v1035) : (i64, i64) -> i64
    %v1037 = call @sloth_str_len(%v1036) : (i64) -> i64
    %v1038 = arith.constant 1 : i64
    %v1039 = arith.shrsi %v1032, %v1038 : i64
    %v1040 = arith.constant 1 : i64
    %v1041 = arith.shrsi %v1037, %v1040 : i64
    %v1042 = arith.addi %v1039, %v1041 : i64
    %v1043 = arith.constant 1 : i64
    %v1044 = arith.shli %v1042, %v1043 : i64
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1028[%v1045] : memref<1xi64>
    %v1047 = arith.constant 2 : i64
    %v1048 = call @sloth_obj_field(%v1046, %v1047) : (i64, i64) -> i64
    %v1049 = call @sloth_arr_len(%v1048) : (i64) -> i64
    %v1050 = arith.constant 1 : i64
    %v1051 = arith.shrsi %v1044, %v1050 : i64
    %v1052 = arith.constant 1 : i64
    %v1053 = arith.shrsi %v1049, %v1052 : i64
    %v1054 = arith.addi %v1051, %v1053 : i64
    %v1055 = arith.constant 1 : i64
    %v1056 = arith.shli %v1054, %v1055 : i64
    %v1057 = arith.constant 0 : index
    %v1058 = memref.load %v1005[%v1057] : memref<1xi64>
    call @sloth_rc_release(%v1058) : (i64) -> i64
    %v1059 = call @sloth_rc_retain(%v1056) : (i64) -> i64
    %v1060 = arith.constant 0 : index
    memref.store %v1059, %v1005[%v1060] : memref<1xi64>
    %v1061 = arith.constant 0 : index
    %v1062 = memref.load %v1008[%v1061] : memref<1xi64>
    %v1063 = arith.constant 2 : i64
    %v1064 = arith.constant 1 : i64
    %v1065 = arith.shrsi %v1062, %v1064 : i64
    %v1066 = arith.constant 1 : i64
    %v1067 = arith.shrsi %v1063, %v1066 : i64
    %v1068 = arith.addi %v1065, %v1067 : i64
    %v1069 = arith.constant 1 : i64
    %v1070 = arith.shli %v1068, %v1069 : i64
    %v1071 = arith.constant 0 : index
    %v1072 = memref.load %v1008[%v1071] : memref<1xi64>
    call @sloth_rc_release(%v1072) : (i64) -> i64
    %v1073 = call @sloth_rc_retain(%v1070) : (i64) -> i64
    %v1074 = arith.constant 0 : index
    memref.store %v1073, %v1008[%v1074] : memref<1xi64>
    %v1075 = arith.constant 0 : index
    %v1076 = memref.load %v1028[%v1075] : memref<1xi64>
    call @sloth_rc_release(%v1076) : (i64) -> i64
    cf.br ^wh_1
  ^wd_3:
    %v1077 = arith.constant 0 : index
    %v1078 = memref.load %v1005[%v1077] : memref<1xi64>
    %v1079 = arith.constant 0 : index
    memref.store %v1078, %v1001[%v1079] : memref<1xi64>
    %v1080 = arith.constant 1 : i64
    memref.store %v1080, %v1000[%v1079] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1081 = arith.constant 0 : index
    %v1082 = memref.load %v1001[%v1081] : memref<1xi64>
    return %v1082 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 100 : i64
    %v1002 = call @sloth_main__churn(%v1001) : (i64) -> i64
    %v1003 = memref.alloca() : memref<1xi64>
    %v1004 = arith.constant 0 : index
    memref.store %v1002, %v1003[%v1004] : memref<1xi64>
    %v1005 = call @sloth_rc_live() : () -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = arith.constant 0 : index
    memref.store %v1005, %v1006[%v1007] : memref<1xi64>
    %v1008 = arith.constant 2000 : i64
    %v1009 = call @sloth_main__churn(%v1008) : (i64) -> i64
    %v1010 = memref.alloca() : memref<1xi64>
    %v1011 = arith.constant 0 : index
    memref.store %v1009, %v1010[%v1011] : memref<1xi64>
    %v1012 = arith.constant 0 : index
    %v1013 = memref.load %v1010[%v1012] : memref<1xi64>
    %v1014 = arith.constant 0 : i64
    %v1016 = arith.constant 0 : i64
    %v1015 = arith.cmpi sgt, %v1013, %v1014 : i64
    %v1017 = arith.extui %v1015 : i1 to i64
    %v1018 = arith.constant 1 : i64
    %v1019 = arith.shli %v1017, %v1018 : i64
    %v1020 = call @sloth_rt_print_bool(%v1019) : (i64) -> i64
    %v1021 = call @sloth_rc_live() : () -> i64
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1006[%v1022] : memref<1xi64>
    %v1025 = arith.constant 0 : i64
    %v1024 = arith.cmpi eq, %v1021, %v1023 : i64
    %v1026 = arith.extui %v1024 : i1 to i64
    %v1027 = arith.constant 1 : i64
    %v1028 = arith.shli %v1026, %v1027 : i64
    %v1029 = call @sloth_rt_print_bool(%v1028) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Holder____init__(%p0: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.constant 478560413032 : i64
    %v1005 = arith.constant 10 : i64
    %v1006 = call @sloth_str_push(%v1003, %v1004, %v1005) : (i64, i64, i64) -> i64
    %v1007 = call @sloth_str_finish(%v1006) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1001[%v1008] : memref<1xi64>
    %v1010 = arith.constant 0 : i64
    %v1011 = call @sloth_obj_field(%v1009, %v1010) : (i64, i64) -> i64
    call @sloth_rc_release(%v1011) : (i64) -> i64
    %v1012 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    call @sloth_obj_set_field(%v1009, %v1010, %v1012) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1007) : (i64) -> i64
    %v1013 = arith.constant 2 : i64
    %v1014 = arith.constant 4 : i64
    %v1015 = arith.constant 6 : i64
    %v1016 = arith.constant 6 : i64
    %v1017 = call @sloth_arr_new(%v1016) : (i64) -> i64
    %v1018 = arith.constant 0 : i64
    call @sloth_arr_set(%v1017, %v1018, %v1013) : (i64, i64, i64) -> i64
    %v1019 = arith.constant 2 : i64
    call @sloth_arr_set(%v1017, %v1019, %v1014) : (i64, i64, i64) -> i64
    %v1020 = arith.constant 4 : i64
    call @sloth_arr_set(%v1017, %v1020, %v1015) : (i64, i64, i64) -> i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1001[%v1021] : memref<1xi64>
    %v1023 = arith.constant 2 : i64
    %v1024 = call @sloth_obj_field(%v1022, %v1023) : (i64, i64) -> i64
    call @sloth_rc_release(%v1024) : (i64) -> i64
    %v1025 = call @sloth_rc_retain(%v1017) : (i64) -> i64
    call @sloth_obj_set_field(%v1022, %v1023, %v1025) : (i64, i64, i64) -> i64
    call @sloth_rc_release(%v1017) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

