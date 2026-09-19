module @main {
  memref.global @sloth_main_g_vtb_Cat : memref<1xi64> = dense<0> {mutable}
  memref.global @sloth_main_g_vtb_Dog : memref<1xi64> = dense<0> {mutable}

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main__announce(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_obj_vtable(%v1005) : (i64) -> i64
    %v1008 = arith.constant 1 : i64
    %v1009 = call @sloth_vt_get(%v1007, %v1008) : (i64, i64) -> i64
    %v1010 = arith.constant 0 : i64
    %v1011 = arith.cmpi ne, %v1009, %v1010 : i64
    %v1012 = arith.extui %v1011 : i1 to i64
    %v1014 = arith.constant 0 : i64
    %v1013 = arith.cmpi ne, %v1012, %v1014 : i64
    cf.cond_br %v1013, ^dc_1, ^dp_2
  ^dc_1:
    %v1015 = llvm.inttoptr %v1009 : i64 to !llvm.ptr
    %v1016 = llvm.call %v1015(%v1005) : !llvm.ptr, (i64) -> i64
    %v1017 = arith.constant 0 : index
    memref.store %v1016, %v1006[%v1017] : memref<1xi64>
    cf.br ^de_3
  ^dp_2:
    %v1018 = arith.constant 0 : i64
    %v1019 = call @sloth_panic_noimpl(%v1018) : (i64) -> i64
    %v1020 = arith.constant 0 : index
    %v1021 = arith.constant 0 : i64
    memref.store %v1021, %v1006[%v1020] : memref<1xi64>
    cf.br ^de_3
  ^de_3:
    %v1022 = arith.constant 0 : index
    %v1023 = memref.load %v1006[%v1022] : memref<1xi64>
    %v1024 = arith.constant 0 : index
    memref.store %v1023, %v1001[%v1024] : memref<1xi64>
    %v1025 = arith.constant 1 : i64
    memref.store %v1025, %v1000[%v1024] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1001[%v1026] : memref<1xi64>
    return %v1027 : i64
  }
  func.func private @sloth_vtb_main_Cat() -> i64 {
    %v1000 = memref.get_global @sloth_main_g_vtb_Cat : memref<1xi64>
    %v1001 = arith.constant 0 : index
    %v1002 = memref.load %v1000[%v1001] : memref<1xi64>
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.cmpi eq, %v1002, %v1003 : i64
    %v1005 = arith.extui %v1004 : i1 to i64
    %v1007 = arith.constant 0 : i64
    %v1006 = arith.cmpi ne, %v1005, %v1007 : i64
    cf.cond_br %v1006, ^vtb_1, ^vtb_2
  ^vtb_1:
    %v1008 = arith.constant 3 : i64
    %v1009 = call @sloth_vt_new(%v1008) : (i64) -> i64
    %v1010 = llvm.mlir.addressof @sloth_main_Cat__name : !llvm.ptr
    %v1011 = llvm.ptrtoint %v1010 : !llvm.ptr to i64
    %v1012 = arith.constant 0 : i64
    call @sloth_vt_set(%v1009, %v1012, %v1011) : (i64, i64, i64) -> i64
    %v1013 = llvm.mlir.addressof @sloth_main_Cat__say : !llvm.ptr
    %v1014 = llvm.ptrtoint %v1013 : !llvm.ptr to i64
    %v1015 = arith.constant 1 : i64
    call @sloth_vt_set(%v1009, %v1015, %v1014) : (i64, i64, i64) -> i64
    memref.store %v1009, %v1000[%v1001] : memref<1xi64>
    cf.br ^vtb_2
  ^vtb_2:
    %v1016 = memref.load %v1000[%v1001] : memref<1xi64>
    return %v1016 : i64
  }
  func.func private @sloth_vtb_main_Dog() -> i64 {
    %v1000 = memref.get_global @sloth_main_g_vtb_Dog : memref<1xi64>
    %v1001 = arith.constant 0 : index
    %v1002 = memref.load %v1000[%v1001] : memref<1xi64>
    %v1003 = arith.constant 0 : i64
    %v1004 = arith.cmpi eq, %v1002, %v1003 : i64
    %v1005 = arith.extui %v1004 : i1 to i64
    %v1007 = arith.constant 0 : i64
    %v1006 = arith.cmpi ne, %v1005, %v1007 : i64
    cf.cond_br %v1006, ^vtb_1, ^vtb_2
  ^vtb_1:
    %v1008 = arith.constant 3 : i64
    %v1009 = call @sloth_vt_new(%v1008) : (i64) -> i64
    %v1010 = llvm.mlir.addressof @sloth_main_Dog__name : !llvm.ptr
    %v1011 = llvm.ptrtoint %v1010 : !llvm.ptr to i64
    %v1012 = arith.constant 0 : i64
    call @sloth_vt_set(%v1009, %v1012, %v1011) : (i64, i64, i64) -> i64
    %v1013 = llvm.mlir.addressof @sloth_main_Dog__say : !llvm.ptr
    %v1014 = llvm.ptrtoint %v1013 : !llvm.ptr to i64
    %v1015 = arith.constant 1 : i64
    call @sloth_vt_set(%v1009, %v1015, %v1014) : (i64, i64, i64) -> i64
    %v1016 = llvm.mlir.addressof @sloth_main_Dog__to_str : !llvm.ptr
    %v1017 = llvm.ptrtoint %v1016 : !llvm.ptr to i64
    %v1018 = arith.constant 2 : i64
    call @sloth_vt_set(%v1009, %v1018, %v1017) : (i64, i64, i64) -> i64
    memref.store %v1009, %v1000[%v1001] : memref<1xi64>
    cf.br ^vtb_2
  ^vtb_2:
    %v1019 = memref.load %v1000[%v1001] : memref<1xi64>
    return %v1019 : i64
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 2 : i64
    %v1003 = call @sloth_cls_info(%v1001, %v1002) : (i64, i64) -> i64
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 0 : i64
    call @sloth_cls_refmask(%v1003, %v1004, %v1005) : (i64, i64, i64) -> i64
    %v1006 = arith.constant 0 : i64
    %v1007 = call @sloth_obj_new(%v1003, %v1006) : (i64, i64) -> i64
    %v1008 = call @sloth_vtb_main_Cat() : () -> i64
    call @sloth_obj_set_vtable(%v1007, %v1008) : (i64, i64) -> i64
    %v1009 = arith.constant 0 : i64
    %v1010 = arith.constant 3 : i64
    %v1011 = call @sloth_cls_info(%v1009, %v1010) : (i64, i64) -> i64
    %v1012 = arith.constant 0 : i64
    %v1013 = arith.constant 0 : i64
    call @sloth_cls_refmask(%v1011, %v1012, %v1013) : (i64, i64, i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = call @sloth_obj_new(%v1011, %v1014) : (i64, i64) -> i64
    %v1016 = call @sloth_vtb_main_Dog() : () -> i64
    call @sloth_obj_set_vtable(%v1015, %v1016) : (i64, i64) -> i64
    %v1017 = arith.constant 2 : i64
    %v1019 = arith.constant 1 : i64
    %v1018 = call @sloth_arr_new_k(%v1017, %v1019) : (i64, i64) -> i64
    %v1020 = arith.constant 0 : i64
    %v1021 = call @sloth_rc_retain(%v1007) : (i64) -> i64
    call @sloth_arr_set(%v1018, %v1020, %v1021) : (i64, i64, i64) -> i64
    %v1022 = arith.constant 1 : i64
    %v1023 = call @sloth_rc_retain(%v1015) : (i64) -> i64
    call @sloth_arr_set(%v1018, %v1022, %v1023) : (i64, i64, i64) -> i64
    %v1024 = memref.alloca() : memref<1xi64>
    %v1025 = call @sloth_rc_retain(%v1018) : (i64) -> i64
    %v1026 = arith.constant 0 : index
    memref.store %v1025, %v1024[%v1026] : memref<1xi64>
    %v1027 = memref.extract_aligned_pointer_as_index %v1024 : memref<1xi64> -> index
    %v1028 = arith.index_cast %v1027 : index to i64
    call @sloth_fiber_track(%v1028) : (i64) -> i64
    call @sloth_rc_release(%v1007) : (i64) -> i64
    call @sloth_rc_release(%v1015) : (i64) -> i64
    call @sloth_rc_release(%v1018) : (i64) -> i64
    %v1029 = arith.constant 0 : index
    %v1030 = memref.load %v1024[%v1029] : memref<1xi64>
    %v1031 = call @sloth_arr_len(%v1030) : (i64) -> i64
    %v1033 = arith.constant 0 : index
    %v1032 = memref.alloca() : memref<1xi64>
    %v1034 = arith.constant 0 : i64
    memref.store %v1034, %v1032[%v1033] : memref<1xi64>
    cf.br ^af_1
  ^af_1:
    %v1035 = memref.load %v1032[%v1033] : memref<1xi64>
    %v1036 = arith.cmpi slt, %v1035, %v1031 : i64
    %v1037 = arith.extui %v1036 : i1 to i64
    %v1039 = arith.constant 0 : i64
    %v1038 = arith.cmpi ne, %v1037, %v1039 : i64
    cf.cond_br %v1038, ^ab_2, ^ae_3
  ^ab_2:
    %v1040 = call @sloth_arr_get(%v1030, %v1035) : (i64, i64) -> i64
    %v1041 = memref.alloca() : memref<1xi64>
    memref.store %v1040, %v1041[%v1033] : memref<1xi64>
    %v1042 = arith.constant 0 : index
    %v1043 = memref.load %v1041[%v1042] : memref<1xi64>
    %v1044 = memref.alloca() : memref<1xi64>
    %v1045 = call @sloth_obj_vtable(%v1043) : (i64) -> i64
    %v1046 = arith.constant 1 : i64
    %v1047 = call @sloth_vt_get(%v1045, %v1046) : (i64, i64) -> i64
    %v1048 = arith.constant 0 : i64
    %v1049 = arith.cmpi ne, %v1047, %v1048 : i64
    %v1050 = arith.extui %v1049 : i1 to i64
    %v1052 = arith.constant 0 : i64
    %v1051 = arith.cmpi ne, %v1050, %v1052 : i64
    cf.cond_br %v1051, ^dc_6, ^dp_7
  ^dc_6:
    %v1053 = llvm.inttoptr %v1047 : i64 to !llvm.ptr
    %v1054 = llvm.call %v1053(%v1043) : !llvm.ptr, (i64) -> i64
    %v1055 = arith.constant 0 : index
    memref.store %v1054, %v1044[%v1055] : memref<1xi64>
    cf.br ^de_8
  ^dp_7:
    %v1056 = arith.constant 0 : i64
    %v1057 = call @sloth_panic_noimpl(%v1056) : (i64) -> i64
    %v1058 = arith.constant 0 : index
    %v1059 = arith.constant 0 : i64
    memref.store %v1059, %v1044[%v1058] : memref<1xi64>
    cf.br ^de_8
  ^de_8:
    %v1060 = arith.constant 0 : index
    %v1061 = memref.load %v1044[%v1060] : memref<1xi64>
    %v1062 = call @sloth_rt_print_str(%v1061) : (i64) -> i64
    call @sloth_rc_release(%v1061) : (i64) -> i64
    cf.br ^ic_4
  ^ic_4:
    %v1063 = arith.constant 1 : i64
    %v1064 = arith.addi %v1035, %v1063 : i64
    memref.store %v1064, %v1032[%v1033] : memref<1xi64>
    cf.br ^af_1
  ^ix_5:
    cf.br ^ae_3
  ^ae_3:
    %v1065 = arith.constant 0 : i64
    %v1066 = arith.constant 3 : i64
    %v1067 = call @sloth_cls_info(%v1065, %v1066) : (i64, i64) -> i64
    %v1068 = arith.constant 0 : i64
    %v1069 = arith.constant 0 : i64
    call @sloth_cls_refmask(%v1067, %v1068, %v1069) : (i64, i64, i64) -> i64
    %v1070 = arith.constant 0 : i64
    %v1071 = call @sloth_obj_new(%v1067, %v1070) : (i64, i64) -> i64
    %v1072 = call @sloth_vtb_main_Dog() : () -> i64
    call @sloth_obj_set_vtable(%v1071, %v1072) : (i64, i64) -> i64
    %v1073 = call @sloth_main__announce(%v1071) : (i64) -> i64
    %v1074 = call @sloth_rt_print_str(%v1073) : (i64) -> i64
    call @sloth_rc_release(%v1071) : (i64) -> i64
    call @sloth_rc_release(%v1073) : (i64) -> i64
    %v1075 = arith.constant 0 : i64
    %v1076 = arith.constant 0 : i64
    %v1077 = arith.constant 3 : i64
    %v1078 = call @sloth_cls_info(%v1076, %v1077) : (i64, i64) -> i64
    %v1079 = arith.constant 0 : i64
    %v1080 = arith.constant 0 : i64
    call @sloth_cls_refmask(%v1078, %v1079, %v1080) : (i64, i64, i64) -> i64
    %v1081 = arith.constant 0 : i64
    %v1082 = call @sloth_obj_new(%v1078, %v1081) : (i64, i64) -> i64
    %v1083 = call @sloth_vtb_main_Dog() : () -> i64
    call @sloth_obj_set_vtable(%v1082, %v1083) : (i64, i64) -> i64
    %v1085 = llvm.call @sloth_main_Dog__to_str(%v1082) : (i64) -> i64
    %v1084 = call @sloth_str_pushp(%v1075, %v1085) : (i64, i64) -> i64
    %v1086 = call @sloth_str_finish(%v1084) : (i64) -> i64
    %v1087 = call @sloth_rt_print_str(%v1086) : (i64) -> i64
    call @sloth_rc_release(%v1082) : (i64) -> i64
    call @sloth_rc_release(%v1086) : (i64) -> i64
    call @sloth_rc_release(%v1085) : (i64) -> i64
    %v1088 = arith.constant 0 : index
    %v1089 = memref.load %v1024[%v1088] : memref<1xi64>
    call @sloth_rc_release(%v1089) : (i64) -> i64
    %v1090 = memref.extract_aligned_pointer_as_index %v1024 : memref<1xi64> -> index
    %v1091 = arith.index_cast %v1090 : index to i64
    call @sloth_fiber_untrack(%v1091) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
  llvm.func @sloth_main_Cat__name(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 7627107 : i64
    %v1006 = arith.constant 3 : i64
    %v1007 = func.call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = func.call @sloth_str_finish(%v1007) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1001[%v1009] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    memref.store %v1010, %v1000[%v1009] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    llvm.return %v1012 : i64
  }
  llvm.func @sloth_main_Cat__say(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 139274035273 : i64
    %v1006 = arith.constant 5 : i64
    %v1007 = func.call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = memref.alloca() : memref<1xi64>
    %v1011 = func.call @sloth_obj_vtable(%v1009) : (i64) -> i64
    %v1012 = arith.constant 0 : i64
    %v1013 = func.call @sloth_vt_get(%v1011, %v1012) : (i64, i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.cmpi ne, %v1013, %v1014 : i64
    %v1016 = arith.extui %v1015 : i1 to i64
    %v1018 = arith.constant 0 : i64
    %v1017 = arith.cmpi ne, %v1016, %v1018 : i64
    cf.cond_br %v1017, ^dc_1, ^dp_2
  ^dc_1:
    %v1019 = llvm.inttoptr %v1013 : i64 to !llvm.ptr
    %v1020 = llvm.call %v1019(%v1009) : !llvm.ptr, (i64) -> i64
    %v1021 = arith.constant 0 : index
    memref.store %v1020, %v1010[%v1021] : memref<1xi64>
    cf.br ^de_3
  ^dp_2:
    %v1022 = arith.constant 0 : i64
    %v1023 = func.call @sloth_panic_noimpl(%v1022) : (i64) -> i64
    %v1024 = arith.constant 0 : index
    %v1025 = arith.constant 0 : i64
    memref.store %v1025, %v1010[%v1024] : memref<1xi64>
    cf.br ^de_3
  ^de_3:
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1010[%v1026] : memref<1xi64>
    %v1028 = func.call @sloth_str_pushp(%v1007, %v1027) : (i64, i64) -> i64
    %v1029 = func.call @sloth_str_finish(%v1028) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    memref.store %v1029, %v1001[%v1030] : memref<1xi64>
    %v1031 = arith.constant 1 : i64
    memref.store %v1031, %v1000[%v1030] : memref<1xi64>
    func.call @sloth_rc_release(%v1027) : (i64) -> i64
    cf.br ^end
  ^end:
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1001[%v1032] : memref<1xi64>
    llvm.return %v1033 : i64
  }
  llvm.func @sloth_main_Dog__name(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 6778724 : i64
    %v1006 = arith.constant 3 : i64
    %v1007 = func.call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = func.call @sloth_str_finish(%v1007) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1001[%v1009] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    memref.store %v1010, %v1000[%v1009] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    llvm.return %v1012 : i64
  }
  llvm.func @sloth_main_Dog__to_str(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 176771526468 : i64
    %v1006 = arith.constant 5 : i64
    %v1007 = func.call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = func.call @sloth_str_finish(%v1007) : (i64) -> i64
    %v1009 = arith.constant 0 : index
    memref.store %v1008, %v1001[%v1009] : memref<1xi64>
    %v1010 = arith.constant 1 : i64
    memref.store %v1010, %v1000[%v1009] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1011 = arith.constant 0 : index
    %v1012 = memref.load %v1001[%v1011] : memref<1xi64>
    llvm.return %v1012 : i64
  }
  llvm.func @sloth_main_Dog__say(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : i64
    %v1005 = arith.constant 139274035273 : i64
    %v1006 = arith.constant 5 : i64
    %v1007 = func.call @sloth_str_push(%v1004, %v1005, %v1006) : (i64, i64, i64) -> i64
    %v1008 = arith.constant 0 : index
    %v1009 = memref.load %v1002[%v1008] : memref<1xi64>
    %v1010 = memref.alloca() : memref<1xi64>
    %v1011 = func.call @sloth_obj_vtable(%v1009) : (i64) -> i64
    %v1012 = arith.constant 0 : i64
    %v1013 = func.call @sloth_vt_get(%v1011, %v1012) : (i64, i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.cmpi ne, %v1013, %v1014 : i64
    %v1016 = arith.extui %v1015 : i1 to i64
    %v1018 = arith.constant 0 : i64
    %v1017 = arith.cmpi ne, %v1016, %v1018 : i64
    cf.cond_br %v1017, ^dc_1, ^dp_2
  ^dc_1:
    %v1019 = llvm.inttoptr %v1013 : i64 to !llvm.ptr
    %v1020 = llvm.call %v1019(%v1009) : !llvm.ptr, (i64) -> i64
    %v1021 = arith.constant 0 : index
    memref.store %v1020, %v1010[%v1021] : memref<1xi64>
    cf.br ^de_3
  ^dp_2:
    %v1022 = arith.constant 0 : i64
    %v1023 = func.call @sloth_panic_noimpl(%v1022) : (i64) -> i64
    %v1024 = arith.constant 0 : index
    %v1025 = arith.constant 0 : i64
    memref.store %v1025, %v1010[%v1024] : memref<1xi64>
    cf.br ^de_3
  ^de_3:
    %v1026 = arith.constant 0 : index
    %v1027 = memref.load %v1010[%v1026] : memref<1xi64>
    %v1028 = func.call @sloth_str_pushp(%v1007, %v1027) : (i64, i64) -> i64
    %v1029 = func.call @sloth_str_finish(%v1028) : (i64) -> i64
    %v1030 = arith.constant 0 : index
    memref.store %v1029, %v1001[%v1030] : memref<1xi64>
    %v1031 = arith.constant 1 : i64
    memref.store %v1031, %v1000[%v1030] : memref<1xi64>
    func.call @sloth_rc_release(%v1027) : (i64) -> i64
    cf.br ^end
  ^end:
    %v1032 = arith.constant 0 : index
    %v1033 = memref.load %v1001[%v1032] : memref<1xi64>
    llvm.return %v1033 : i64
  }
}

