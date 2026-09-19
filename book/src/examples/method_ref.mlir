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
    %v1008 = arith.constant 1 : i64
    %v1009 = call @sloth_obj_field(%v1005, %v1008) : (i64, i64) -> i64
    %v1010 = llvm.inttoptr %v1007 : i64 to !llvm.ptr
    %v1011 = llvm.call %v1010(%v1009) : !llvm.ptr, (i64) -> i64
    %v1012 = arith.constant 0 : index
    memref.store %v1011, %v1001[%v1012] : memref<1xi64>
    %v1013 = arith.constant 1 : i64
    memref.store %v1013, %v1000[%v1012] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1014 = arith.constant 0 : index
    %v1015 = memref.load %v1001[%v1014] : memref<1xi64>
    return %v1015 : i64
  }
  llvm.func @sloth_main__clo0(%p0: i64) -> i64 {
    %v1 = func.call @sloth_main_Counter__bump(%p0) : (i64) -> i64
    llvm.return %v1 : i64
}
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 10 : i64
    %v1002 = arith.constant 0 : i64
    %v1003 = arith.constant 2 : i64
    %v1004 = call @sloth_cls_info(%v1002, %v1003) : (i64, i64) -> i64
    %v1005 = arith.constant 0 : i64
    %v1006 = arith.constant 1 : i64
    call @sloth_cls_refmask(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1007 = arith.constant 1 : i64
    %v1008 = call @sloth_obj_new(%v1004, %v1007) : (i64, i64) -> i64
    %v1009 = arith.constant 0 : i64
    %v1010 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1008, %v1010, %v1009) : (i64, i64, i64) -> i64
    call @sloth_main_Counter____init__(%v1008, %v1001) : (i64, i64) -> ()
    %v1011 = memref.alloca() : memref<1xi64>
    %v1012 = call @sloth_rc_retain(%v1008) : (i64) -> i64
    %v1013 = arith.constant 0 : index
    memref.store %v1012, %v1011[%v1013] : memref<1xi64>
    %v1014 = memref.extract_aligned_pointer_as_index %v1011 : memref<1xi64> -> index
    %v1015 = arith.index_cast %v1014 : index to i64
    call @sloth_fiber_track(%v1015) : (i64) -> i64
    call @sloth_rc_release(%v1008) : (i64) -> i64
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1011[%v1016] : memref<1xi64>
    %v1018 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %v1019 = llvm.ptrtoint %v1018 : !llvm.ptr to i64
    %v1020 = call @sloth_rc_retain(%v1017) : (i64) -> i64
    %v1021 = call @sloth_closure_new(%v1019, %v1020) : (i64, i64) -> i64
    %v1022 = memref.alloca() : memref<1xi64>
    %v1023 = call @sloth_rc_retain(%v1021) : (i64) -> i64
    %v1024 = arith.constant 0 : index
    memref.store %v1023, %v1022[%v1024] : memref<1xi64>
    %v1025 = memref.extract_aligned_pointer_as_index %v1022 : memref<1xi64> -> index
    %v1026 = arith.index_cast %v1025 : index to i64
    call @sloth_fiber_track(%v1026) : (i64) -> i64
    call @sloth_rc_release(%v1021) : (i64) -> i64
    %v1027 = arith.constant 0 : index
    %v1028 = memref.load %v1022[%v1027] : memref<1xi64>
    %v1029 = arith.constant 0 : i64
    %v1030 = call @sloth_obj_field(%v1028, %v1029) : (i64, i64) -> i64
    %v1031 = arith.constant 1 : i64
    %v1032 = call @sloth_obj_field(%v1028, %v1031) : (i64, i64) -> i64
    %v1033 = llvm.inttoptr %v1030 : i64 to !llvm.ptr
    %v1034 = llvm.call %v1033(%v1032) : !llvm.ptr, (i64) -> i64
    %v1035 = call @sloth_rt_print_i64(%v1034) : (i64) -> i64
    %v1036 = arith.constant 0 : index
    %v1037 = memref.load %v1022[%v1036] : memref<1xi64>
    %v1038 = arith.constant 0 : i64
    %v1039 = call @sloth_obj_field(%v1037, %v1038) : (i64, i64) -> i64
    %v1040 = arith.constant 1 : i64
    %v1041 = call @sloth_obj_field(%v1037, %v1040) : (i64, i64) -> i64
    %v1042 = llvm.inttoptr %v1039 : i64 to !llvm.ptr
    %v1043 = llvm.call %v1042(%v1041) : !llvm.ptr, (i64) -> i64
    %v1044 = call @sloth_rt_print_i64(%v1043) : (i64) -> i64
    %v1045 = arith.constant 0 : index
    %v1046 = memref.load %v1011[%v1045] : memref<1xi64>
    %v1047 = llvm.mlir.addressof @sloth_main__clo0 : !llvm.ptr
    %v1048 = llvm.ptrtoint %v1047 : !llvm.ptr to i64
    %v1049 = call @sloth_rc_retain(%v1046) : (i64) -> i64
    %v1050 = call @sloth_closure_new(%v1048, %v1049) : (i64, i64) -> i64
    %v1051 = call @sloth_main__run(%v1050) : (i64) -> i64
    %v1052 = call @sloth_rt_print_i64(%v1051) : (i64) -> i64
    call @sloth_rc_release(%v1050) : (i64) -> i64
    %v1053 = arith.constant 0 : index
    %v1054 = memref.load %v1022[%v1053] : memref<1xi64>
    call @sloth_rc_release(%v1054) : (i64) -> i64
    %v1055 = memref.extract_aligned_pointer_as_index %v1022 : memref<1xi64> -> index
    %v1056 = arith.index_cast %v1055 : index to i64
    call @sloth_fiber_untrack(%v1056) : (i64) -> i64
    %v1057 = arith.constant 0 : index
    %v1058 = memref.load %v1011[%v1057] : memref<1xi64>
    call @sloth_rc_release(%v1058) : (i64) -> i64
    %v1059 = memref.extract_aligned_pointer_as_index %v1011 : memref<1xi64> -> index
    %v1060 = arith.index_cast %v1059 : index to i64
    call @sloth_fiber_untrack(%v1060) : (i64) -> i64
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
    call @sloth_obj_set_field(%v1008, %v1009, %v1006) : (i64, i64, i64) -> i64
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
    %v1008 = arith.constant 1 : i64
    %v1009 = arith.addi %v1007, %v1008 : i64
    %v1010 = arith.constant 0 : index
    %v1011 = memref.load %v1002[%v1010] : memref<1xi64>
    %v1012 = arith.constant 0 : i64
    call @sloth_obj_set_field(%v1011, %v1012, %v1009) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 0 : index
    %v1014 = memref.load %v1002[%v1013] : memref<1xi64>
    %v1015 = arith.constant 0 : i64
    %v1016 = call @sloth_obj_field(%v1014, %v1015) : (i64, i64) -> i64
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
}

