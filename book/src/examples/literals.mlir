module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 42 : i64
    %v1002 = call @sloth_rt_print_i64(%v1001) : (i64) -> i64
    %v1003 = arith.constant 4615063718147915776 : i64
    %v1005 = llvm.bitcast %v1003 : i64 to f64
    %v1004 = call @sloth_rt_print_f64(%v1005) : (f64) -> i64
    %v1006 = arith.constant 1 : i64
    %v1007 = call @sloth_rt_print_bool(%v1006) : (i64) -> i64
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.constant 1954047348 : i64
    %v1010 = arith.constant 4 : i64
    %v1011 = call @sloth_str_push(%v1008, %v1009, %v1010) : (i64, i64, i64) -> i64
    %v1012 = call @sloth_str_finish(%v1011) : (i64) -> i64
    %v1013 = call @sloth_rt_print_str(%v1012) : (i64) -> i64
    call @sloth_rc_release(%v1012) : (i64) -> i64
    %v1014 = arith.constant 4652007308841189376 : i64
    %v1016 = llvm.bitcast %v1014 : i64 to f64
    %v1015 = call @sloth_rt_print_f64(%v1016) : (f64) -> i64
    %v1017 = arith.constant 4598175219545276416 : i64
    %v1019 = llvm.bitcast %v1017 : i64 to f64
    %v1018 = call @sloth_rt_print_f64(%v1019) : (f64) -> i64
    %v1020 = arith.constant 0 : i64
    %v1021 = arith.constant 7310016642684182900 : i64
    %v1022 = arith.constant 8 : i64
    %v1023 = call @sloth_str_push(%v1020, %v1021, %v1022) : (i64, i64, i64) -> i64
    %v1024 = call @sloth_str_finish(%v1023) : (i64) -> i64
    %v1025 = call @sloth_rt_print_str(%v1024) : (i64) -> i64
    call @sloth_rc_release(%v1024) : (i64) -> i64
    %v1026 = arith.constant 0 : i64
    %v1027 = arith.constant 2315448778539103601 : i64
    %v1028 = arith.constant 8 : i64
    %v1029 = call @sloth_str_push(%v1026, %v1027, %v1028) : (i64, i64, i64) -> i64
    %v1030 = arith.constant 111481940307561 : i64
    %v1031 = arith.constant 6 : i64
    %v1032 = call @sloth_str_push(%v1029, %v1030, %v1031) : (i64, i64, i64) -> i64
    %v1033 = call @sloth_str_finish(%v1032) : (i64) -> i64
    %v1034 = call @sloth_rt_print_str(%v1033) : (i64) -> i64
    call @sloth_rc_release(%v1033) : (i64) -> i64
    %v1035 = arith.constant 0 : i64
    %v1036 = arith.constant 2323048382453194801 : i64
    %v1037 = arith.constant 8 : i64
    %v1038 = call @sloth_str_push(%v1035, %v1036, %v1037) : (i64, i64, i64) -> i64
    %v1039 = arith.constant 1 : i64
    %v1040 = arith.constant 2 : i64
    %v1041 = arith.addi %v1039, %v1040 : i64
    %v1042 = call @sloth_str_push_i(%v1038, %v1041) : (i64, i64) -> i64
    %v1043 = call @sloth_str_finish(%v1042) : (i64) -> i64
    %v1044 = call @sloth_rt_print_str(%v1043) : (i64) -> i64
    call @sloth_rc_release(%v1043) : (i64) -> i64
    %v1045 = arith.constant 0 : i64
    %v1046 = arith.constant 448630058099 : i64
    %v1047 = arith.constant 5 : i64
    %v1048 = call @sloth_str_push(%v1045, %v1046, %v1047) : (i64, i64, i64) -> i64
    %v1049 = call @sloth_str_finish(%v1048) : (i64) -> i64
    %v1050 = memref.alloca() : memref<1xi64>
    %v1051 = call @sloth_rc_retain(%v1049) : (i64) -> i64
    %v1052 = arith.constant 0 : index
    memref.store %v1051, %v1050[%v1052] : memref<1xi64>
    %v1053 = memref.extract_aligned_pointer_as_index %v1050 : memref<1xi64> -> index
    %v1054 = arith.index_cast %v1053 : index to i64
    call @sloth_fiber_track(%v1054) : (i64) -> i64
    call @sloth_rc_release(%v1049) : (i64) -> i64
    %v1055 = arith.constant 0 : i64
    %v1056 = arith.constant 2124136 : i64
    %v1057 = arith.constant 3 : i64
    %v1058 = call @sloth_str_push(%v1055, %v1056, %v1057) : (i64, i64, i64) -> i64
    %v1059 = arith.constant 0 : index
    %v1060 = memref.load %v1050[%v1059] : memref<1xi64>
    %v1061 = call @sloth_str_pushp(%v1058, %v1060) : (i64, i64) -> i64
    %v1062 = arith.constant 67544357281836 : i64
    %v1063 = arith.constant 6 : i64
    %v1064 = call @sloth_str_push(%v1061, %v1062, %v1063) : (i64, i64, i64) -> i64
    %v1065 = arith.constant 0 : index
    %v1066 = memref.load %v1050[%v1065] : memref<1xi64>
    %v1067 = call @sloth_str_len(%v1066) : (i64) -> i64
    %v1068 = call @sloth_str_push_i(%v1064, %v1067) : (i64, i64) -> i64
    %v1069 = call @sloth_str_finish(%v1068) : (i64) -> i64
    %v1070 = call @sloth_rt_print_str(%v1069) : (i64) -> i64
    call @sloth_rc_release(%v1069) : (i64) -> i64
    %v1071 = arith.constant 0 : index
    %v1072 = memref.load %v1050[%v1071] : memref<1xi64>
    call @sloth_rc_release(%v1072) : (i64) -> i64
    %v1073 = memref.extract_aligned_pointer_as_index %v1050 : memref<1xi64> -> index
    %v1074 = arith.index_cast %v1073 : index to i64
    call @sloth_fiber_untrack(%v1074) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

