module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__run(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 2 : i64
    %v1009 = call @sloth_obj_field(%v1005, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant -2 : i64
    %v1011 = arith.andi %v1007, %v1010 : i64
    %v1012 = llvm.inttoptr %v1011 : i64 to !llvm.ptr
    %v1013 = llvm.call %v1012(%v1009) : !llvm.ptr, (i64) -> i64
    %v1014 = arith.constant 0 : index
    memref.store %v1013, %v1001[%v1014] : memref<1xi64>
    %v1015 = arith.constant 1 : i64
    memref.store %v1015, %v1000[%v1014] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1001[%v1016] : memref<1xi64>
    return %v1017 : i64
  }
  llvm.func @sloth_main__clo0(%p0: i64) -> i64 {
    %v1 = func.call @sloth_main_Counter__bump(%p0) : (i64) -> i64
    llvm.return %v1 : i64
}
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 20 : i64
    %v1002 = arith.constant 0 : i64
    %v1003 = arith.constant 4 : i64
    %v1004 = call @sloth_cls_info(%v1002, %v1003) : (i64, i64) -> i64
    %v1005 = arith.constant 0 : i64
    %v1006 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 2 : i64
    %v1008 = call @sloth_obj_new(%v1004, %v1007) : (i64, i64) -> i64
    %v1009 = arith.constant 0 : i64
    %v1010 = arith.constant 0 : i64
    %v1011 = call @sloth_obj_field(%v1008, %v1010) : (i64, i64) -> i64
    call @sloth_rc_release(%v1011) : (i64) -> i64
    %v1012 = call @sloth_rc_retain(%v1009) : (i64) -> i64
    call @sloth_obj_set_field(%v1008, %v1010, %v1012) : (i64, i64, i64) -> i64
    call @sloth_main_Counter____init__(%v1008, %v1001) : (i64, i64) -> ()
    %v1013 = memref.alloca() : memref<1xi64>
    %v1014 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    %v1015 = arith.constant 0 : index
    memref.store %v1014, %v1013[%v1015] : memref<1xi64>
    call @sloth_rc_release(%v1008) : (i64) -> i64
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1013[%v1016] : memref<1xi64>
    %v1018 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %v1019 = llvm.ptrtoint %v1018 : !llvm.ptr to i64
    %v1020 = arith.constant 1 : i64
    %v1021 = arith.ori %v1019, %v1020 : i64
    %v1022 = call @sloth_rc_retain(%v1017) : (i64) -> i64
    %v1023 = call @sloth_closure_new(%v1021, %v1022) : (i64, i64) -> i64
    %v1024 = memref.alloca() : memref<1xi64>
    %v1025 = call @sloth_rc_retain(%v1023) : (i64) -> i64
    %v1026 = arith.constant 0 : index
    memref.store %v1025, %v1024[%v1026] : memref<1xi64>
    call @sloth_rc_release(%v1023) : (i64) -> i64
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1024[%v1027] : memref<1xi64>
    %v1029 = arith.constant 0 : i64
    %v1030 = call @sloth_obj_field(%v1028, %v1029) : (i64, i64) -> i64
    %v1031 = arith.constant 2 : i64
    %v1032 = call @sloth_obj_field(%v1028, %v1031) : (i64, i64) -> i64
    %v1033 = arith.constant -2 : i64
    %v1034 = arith.andi %v1030, %v1033 : i64
    %v1035 = llvm.inttoptr %v1034 : i64 to !llvm.ptr
    %v1036 = llvm.call %v1035(%v1032) : !llvm.ptr, (i64) -> i64
    %v1037 = call @sloth_rt_print_i64(%v1036) : (i64) -> i64
    %v1038 = arith.constant 0 : index
    %v1039 = memref.load %v1024[%v1038] : memref<1xi64>
    %v1040 = arith.constant 0 : i64
    %v1041 = call @sloth_obj_field(%v1039, %v1040) : (i64, i64) -> i64
    %v1042 = arith.constant 2 : i64
    %v1043 = call @sloth_obj_field(%v1039, %v1042) : (i64, i64) -> i64
    %v1044 = arith.constant -2 : i64
    %v1045 = arith.andi %v1041, %v1044 : i64
    %v1046 = llvm.inttoptr %v1045 : i64 to !llvm.ptr
    %v1047 = llvm.call %v1046(%v1043) : !llvm.ptr, (i64) -> i64
    %v1048 = call @sloth_rt_print_i64(%v1047) : (i64) -> i64
    %v1049 = arith.constant 0 : index
    %v1050 = memref.load %v1013[%v1049] : memref<1xi64>
    %v1051 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %v1052 = llvm.ptrtoint %v1051 : !llvm.ptr to i64
    %v1053 = arith.constant 1 : i64
    %v1054 = arith.ori %v1052, %v1053 : i64
    %v1055 = call @sloth_rc_retain(%v1050) : (i64) -> i64
    %v1056 = call @sloth_closure_new(%v1054, %v1055) : (i64, i64) -> i64
    %v1057 = call @sloth_main__run(%v1056) : (i64) -> i64
    %v1058 = call @sloth_rt_print_i64(%v1057) : (i64) -> i64
    call @sloth_rc_release(%v1056) : (i64) -> i64
    %v1059 = arith.constant 0 : index
    %v1060 = memref.load %v1013[%v1059] : memref<1xi64>
    call @sloth_rc_release(%v1060) : (i64) -> i64
    %v1061 = arith.constant 0 : index
    %v1062 = memref.load %v1024[%v1061] : memref<1xi64>
    call @sloth_rc_release(%v1062) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Counter____init__(%p0: i64, %p1: i64) -> () {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : index
    %v1001 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1001[%v1002] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1003 = memref.alloca() : memref<1xi64>
    memref.store %p1, %v1003[%v1004] : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1003[%v1005] : memref<1xi64>
    %v1007 = arith.constant 0 : index
    %v1008 = memref.load %v1001[%v1007] : memref<1xi64>
    %v1009 = arith.constant 0 : i64
    %v1010 = call @sloth_obj_field(%v1008, %v1009) : (i64, i64) -> i64
    call @sloth_rc_release(%v1010) : (i64) -> i64
    %v1011 = call @sloth_rc_retain(%v1006) : (i64) -> i64
    call @sloth_obj_set_field(%v1008, %v1009, %v1011) : (i64, i64, i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  func.func @sloth_main_Counter__bump(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_field(%v1005, %v1006) : (i64, i64) -> i64
    %v1008 = arith.constant 2 : i64
    %v1009 = arith.constant 1 : i64
    %v1010 = arith.shrsi %v1007, %v1009 : i64
    %v1011 = arith.constant 1 : i64
    %v1012 = arith.shrsi %v1008, %v1011 : i64
    %v1013 = arith.addi %v1010, %v1012 : i64
    %v1014 = arith.constant 1 : i64
    %v1015 = arith.shli %v1013, %v1014 : i64
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1002[%v1016] : memref<1xi64>
    %v1018 = arith.constant 0 : i64
    %v1019 = call @sloth_obj_field(%v1017, %v1018) : (i64, i64) -> i64
    call @sloth_rc_release(%v1019) : (i64) -> i64
    %v1020 = call @sloth_rc_retain(%v1015) : (i64) -> i64
    call @sloth_obj_set_field(%v1017, %v1018, %v1020) : (i64, i64, i64) -> i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1002[%v1021] : memref<1xi64>
    %v1023 = arith.constant 0 : i64
    %v1024 = call @sloth_obj_field(%v1022, %v1023) : (i64, i64) -> i64
    %v1025 = arith.constant 0 : index
    memref.store %v1024, %v1001[%v1025] : memref<1xi64>
    %v1026 = arith.constant 1 : i64
    memref.store %v1026, %v1000[%v1025] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1001[%v1027] : memref<1xi64>
    return %v1028 : i64
  }
}

