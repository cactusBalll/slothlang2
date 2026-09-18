module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__add(%p0: i64, %p1: i64) -> i64 {
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
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1004[%v1008] : memref<1xi64>
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
  func.func @sloth_main__fact(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 2 : i64
    %v1008 = arith.constant 0 : i64
    %v1007 = arith.cmpi sle, %v1005, %v1006 : i64
    %v1009 = arith.extui %v1007 : i1 to i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shli %v1009, %v1010 : i64
    %v1013 = arith.constant 0 : i64
    %v1012 = arith.cmpi ne, %v1011, %v1013 : i64
    cf.cond_br %v1012, ^t_1, ^e_2
  ^t_1:
    %v1014 = arith.constant 2 : i64
    %v1015 = arith.constant 0 : index
    memref.store %v1014, %v1001[%v1015] : memref<1xi64>
    %v1016 = arith.constant 1 : i64
    memref.store %v1016, %v1000[%v1015] : memref<1xi64>
    cf.br ^end
  ^e_2:
    cf.br ^fi_3
  ^fi_3:
    %v1017 = arith.constant 0 : index
    %v1018 = memref.load %v1002[%v1017] : memref<1xi64>
    %v1019 = arith.constant 0 : index
    %v1020 = memref.load %v1002[%v1019] : memref<1xi64>
    %v1021 = arith.constant 2 : i64
    %v1022 = arith.constant 1 : i64
    %v1023 = arith.shrsi %v1020, %v1022 : i64
    %v1024 = arith.constant 1 : i64
    %v1025 = arith.shrsi %v1021, %v1024 : i64
    %v1026 = arith.subi %v1023, %v1025 : i64
    %v1027 = arith.constant 1 : i64
    %v1028 = arith.shli %v1026, %v1027 : i64
    %v1029 = call @sloth_main__fact(%v1028) : (i64) -> i64
    %v1030 = arith.constant 1 : i64
    %v1031 = arith.shrsi %v1018, %v1030 : i64
    %v1032 = arith.constant 1 : i64
    %v1033 = arith.shrsi %v1029, %v1032 : i64
    %v1034 = arith.muli %v1031, %v1033 : i64
    %v1035 = arith.constant 1 : i64
    %v1036 = arith.shli %v1034, %v1035 : i64
    %v1037 = arith.constant 0 : index
    memref.store %v1036, %v1001[%v1037] : memref<1xi64>
    %v1038 = arith.constant 1 : i64
    memref.store %v1038, %v1000[%v1037] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1039 = arith.constant 0 : index
    %v1040 = memref.load %v1001[%v1039] : memref<1xi64>
    return %v1040 : i64
  }
  func.func @sloth_main__greet(%p0: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.constant 2124136 : i64
    %v1005 = arith.constant 6 : i64
    %v1006 = call @sloth_str_push(%v1003, %v1004, %v1005) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1001[%v1007] : memref<1xi64>
    %v1009 = call @sloth_str_pushp(%v1006, %v1008) : (i64, i64) -> i64
    %v1010 = call @sloth_str_finish(%v1009) : (i64) -> i64
    %v1011 = call @sloth_rt_print_str(%v1010) : (i64) -> i64
    call @sloth_rc_release(%v1010) : (i64) -> i64
    %v1012 = arith.constant 0 : index
    %v1013 = arith.constant 1 : i64
    memref.store %v1013, %v1000[%v1012] : memref<1xi64>
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main__sum(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = memref.alloca() : memref<1xi64>
    %v1006 = arith.constant 0 : index
    memref.store %v1004, %v1005[%v1006] : memref<1xi64>
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1002[%v1007] : memref<1xi64>
    %v1009 = call @sloth_arr_len(%v1008) : (i64) -> i64
    %v1011 = arith.constant 0 : index
    %v1010 = memref.alloca() : memref<1xi64>
    %v1012 = arith.constant 0 : i64
    memref.store %v1012, %v1010[%v1011] : memref<1xi64>
    cf.br ^af_1
  ^af_1:
    %v1013 = memref.load %v1010[%v1011] : memref<1xi64>
    %v1014 = arith.cmpi slt, %v1013, %v1009 : i64
    %v1015 = arith.extui %v1014 : i1 to i64
    %v1017 = arith.constant 0 : i64
    %v1016 = arith.cmpi ne, %v1015, %v1017 : i64
    cf.cond_br %v1016, ^ab_2, ^ae_3
  ^ab_2:
    %v1018 = call @sloth_arr_get(%v1008, %v1013) : (i64, i64) -> i64
    %v1019 = memref.alloca() : memref<1xi64>
    memref.store %v1018, %v1019[%v1011] : memref<1xi64>
    %v1020 = arith.constant 0 : index
    %v1021 = memref.load %v1005[%v1020] : memref<1xi64>
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1019[%v1022] : memref<1xi64>
    %v1024 = arith.constant 1 : i64
    %v1025 = arith.shrsi %v1021, %v1024 : i64
    %v1026 = arith.constant 1 : i64
    %v1027 = arith.shrsi %v1023, %v1026 : i64
    %v1028 = arith.addi %v1025, %v1027 : i64
    %v1029 = arith.constant 1 : i64
    %v1030 = arith.shli %v1028, %v1029 : i64
    %v1031 = arith.constant 0 : index
    %v1032 = memref.load %v1005[%v1031] : memref<1xi64>
    call @sloth_rc_release(%v1032) : (i64) -> i64
    %v1033 = call @sloth_rc_retain(%v1030) : (i64) -> i64
    %v1034 = arith.constant 0 : index
    memref.store %v1033, %v1005[%v1034] : memref<1xi64>
    cf.br ^ic_4
  ^ic_4:
    %v1035 = arith.constant 2 : i64
    %v1036 = arith.addi %v1013, %v1035 : i64
    memref.store %v1036, %v1010[%v1011] : memref<1xi64>
    cf.br ^af_1
  ^ix_5:
    cf.br ^ae_3
  ^ae_3:
    %v1037 = arith.constant 0 : index
    %v1038 = memref.load %v1005[%v1037] : memref<1xi64>
    %v1039 = arith.constant 0 : index
    memref.store %v1038, %v1001[%v1039] : memref<1xi64>
    %v1040 = arith.constant 1 : i64
    memref.store %v1040, %v1000[%v1039] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1041 = arith.constant 0 : index
    %v1042 = memref.load %v1001[%v1041] : memref<1xi64>
    return %v1042 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 4 : i64
    %v1002 = arith.constant 6 : i64
    %v1003 = call @sloth_main__add(%v1001, %v1002) : (i64, i64) -> i64
    %v1004 = call @sloth_rt_print_i64(%v1003) : (i64) -> i64
    %v1005 = arith.constant 2 : i64
    %v1006 = arith.constant 4 : i64
    %v1007 = call @sloth_main__add(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 6 : i64
    %v1009 = call @sloth_main__add(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = call @sloth_rt_print_i64(%v1009) : (i64) -> i64
    %v1011 = arith.constant 10 : i64
    %v1012 = call @sloth_main__fact(%v1011) : (i64) -> i64
    %v1013 = call @sloth_rt_print_i64(%v1012) : (i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.constant 448630058099 : i64
    %v1016 = arith.constant 10 : i64
    %v1017 = call @sloth_str_push(%v1014, %v1015, %v1016) : (i64, i64, i64) -> i64
    %v1018 = call @sloth_str_finish(%v1017) : (i64) -> i64
    call @sloth_main__greet(%v1018) : (i64) -> ()
    call @sloth_rc_release(%v1018) : (i64) -> i64
    %v1020 = arith.constant 2 : i64
    %v1021 = arith.constant 4 : i64
    %v1022 = arith.constant 6 : i64
    %v1023 = arith.constant 8 : i64
    %v1025 = arith.constant 8 : i64
    %v1026 = call @sloth_arr_new(%v1025) : (i64) -> i64
    %v1027 = arith.constant 0 : i64
    call @sloth_arr_set(%v1026, %v1027, %v1020) : (i64, i64, i64) -> i64
    %v1028 = arith.constant 2 : i64
    call @sloth_arr_set(%v1026, %v1028, %v1021) : (i64, i64, i64) -> i64
    %v1029 = arith.constant 4 : i64
    call @sloth_arr_set(%v1026, %v1029, %v1022) : (i64, i64, i64) -> i64
    %v1030 = arith.constant 6 : i64
    call @sloth_arr_set(%v1026, %v1030, %v1023) : (i64, i64, i64) -> i64
    %v1024 = call @sloth_main__sum(%v1026) : (i64) -> i64
    %v1031 = call @sloth_rt_print_i64(%v1024) : (i64) -> i64
    %v1033 = arith.constant 0 : i64
    %v1034 = call @sloth_arr_new(%v1033) : (i64) -> i64
    %v1032 = call @sloth_main__sum(%v1034) : (i64) -> i64
    %v1035 = call @sloth_rt_print_i64(%v1032) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

