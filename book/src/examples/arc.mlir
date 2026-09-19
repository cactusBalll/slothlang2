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
    %v1018 = arith.constant 0 : i64
    %v1017 = arith.cmpi ne, %v1016, %v1018 : i64
    cf.cond_br %v1017, ^do_2, ^wd_3
  ^do_2:
    %v1019 = arith.constant 0 : i64
    %v1020 = arith.constant 2 : i64
    %v1021 = call @sloth_cls_info(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = arith.constant 3 : i64
    %v1023 = arith.constant 2 : i64
    call @sloth_cls_refmask(%v1021, %v1022, %v1023) : (i64, i64, i64) -> i64
    %v1024 = arith.constant 2 : i64
    %v1025 = call @sloth_obj_new(%v1021, %v1024) : (i64, i64) -> i64
    call @sloth_main_Holder____init__(%v1025) : (i64) -> ()
    %v1026 = memref.alloca() : memref<1xi64>
    %v1027 = call @sloth_rc_retain(%v1025) : (i64) -> i64
    %v1028 = arith.constant 0 : index
    memref.store %v1027, %v1026[%v1028] : memref<1xi64>
    %v1029 = memref.extract_aligned_pointer_as_index %v1026 : memref<1xi64> -> index
    %v1030 = arith.index_cast %v1029 : index to i64
    call @sloth_fiber_track(%v1030) : (i64) -> i64
    call @sloth_rc_release(%v1025) : (i64) -> i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1005[%v1031] : memref<1xi64>
    %v1033 = arith.constant 0 : index
    %v1034 = memref.load %v1026[%v1033] : memref<1xi64>
    %v1035 = arith.constant 0 : i64
    %v1036 = call @sloth_obj_field(%v1034, %v1035) : (i64, i64) -> i64
    %v1037 = call @sloth_str_len(%v1036) : (i64) -> i64
    %v1038 = arith.addi %v1032, %v1037 : i64
    %v1039 = arith.constant 0 : index
    %v1040 = memref.load %v1026[%v1039] : memref<1xi64>
    %v1041 = arith.constant 1 : i64
    %v1042 = call @sloth_obj_field(%v1040, %v1041) : (i64, i64) -> i64
    %v1043 = call @sloth_arr_len(%v1042) : (i64) -> i64
    %v1044 = arith.addi %v1038, %v1043 : i64
    %v1045 = arith.constant 0 : index
    memref.store %v1044, %v1005[%v1045] : memref<1xi64>
    %v1046 = arith.constant 0 : index
    %v1047 = memref.load %v1008[%v1046] : memref<1xi64>
    %v1048 = arith.constant 1 : i64
    %v1049 = arith.addi %v1047, %v1048 : i64
    %v1050 = arith.constant 0 : index
    memref.store %v1049, %v1008[%v1050] : memref<1xi64>
    %v1051 = arith.constant 0 : index
    %v1052 = memref.load %v1026[%v1051] : memref<1xi64>
    call @sloth_rc_release(%v1052) : (i64) -> i64
    %v1053 = memref.extract_aligned_pointer_as_index %v1026 : memref<1xi64> -> index
    %v1054 = arith.index_cast %v1053 : index to i64
    call @sloth_fiber_untrack(%v1054) : (i64) -> i64
    cf.br ^wh_1
  ^wd_3:
    %v1055 = arith.constant 0 : index
    %v1056 = memref.load %v1005[%v1055] : memref<1xi64>
    %v1057 = arith.constant 0 : index
    memref.store %v1056, %v1001[%v1057] : memref<1xi64>
    %v1058 = arith.constant 1 : i64
    memref.store %v1058, %v1000[%v1057] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1059 = arith.constant 0 : index
    %v1060 = memref.load %v1001[%v1059] : memref<1xi64>
    return %v1060 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 50 : i64
    %v1002 = call @sloth_main__churn(%v1001) : (i64) -> i64
    %v1003 = memref.alloca() : memref<1xi64>
    %v1004 = arith.constant 0 : index
    memref.store %v1002, %v1003[%v1004] : memref<1xi64>
    %v1005 = call @sloth_rc_live() : () -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = arith.constant 0 : index
    memref.store %v1005, %v1006[%v1007] : memref<1xi64>
    %v1008 = arith.constant 1000 : i64
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
    %v1018 = call @sloth_rt_print_bool(%v1017) : (i64) -> i64
    %v1019 = call @sloth_rc_live() : () -> i64
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1006[%v1020] : memref<1xi64>
    %v1023 = arith.constant 0 : i64
    %v1022 = arith.cmpi eq, %v1019, %v1021 : i64
    %v1024 = arith.extui %v1022 : i1 to i64
    %v1025 = call @sloth_rt_print_bool(%v1024) : (i64) -> i64
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
    %v1005 = arith.constant 5 : i64
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
    %v1013 = arith.constant 1 : i64
    %v1014 = arith.constant 2 : i64
    %v1015 = arith.constant 3 : i64
    %v1016 = arith.constant 3 : i64
    %v1017 = call @sloth_arr_new(%v1016) : (i64) -> i64
    %v1018 = arith.constant 0 : i64
    call @sloth_arr_set(%v1017, %v1018, %v1013) : (i64, i64, i64) -> i64
    %v1019 = arith.constant 1 : i64
    call @sloth_arr_set(%v1017, %v1019, %v1014) : (i64, i64, i64) -> i64
    %v1020 = arith.constant 2 : i64
    call @sloth_arr_set(%v1017, %v1020, %v1015) : (i64, i64, i64) -> i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1001[%v1021] : memref<1xi64>
    %v1023 = arith.constant 1 : i64
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

