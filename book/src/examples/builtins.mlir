module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 42 : i64
    %v1002 = call @sloth_rt_print_i64(%v1001) : (i64) -> i64
    %v1003 = arith.constant 4612811918334230528 : i64
    %v1005 = llvm.bitcast %v1003 : i64 to f64
    %v1004 = call @sloth_rt_print_f64(%v1005) : (f64) -> i64
    %v1006 = arith.constant 1 : i64
    %v1007 = call @sloth_rt_print_bool(%v1006) : (i64) -> i64
    %v1008 = arith.constant 0 : i64
    %v1009 = arith.constant 115 : i64
    %v1010 = arith.constant 1 : i64
    %v1011 = call @sloth_str_push(%v1008, %v1009, %v1010) : (i64, i64, i64) -> i64
    %v1012 = call @sloth_str_finish(%v1011) : (i64) -> i64
    %v1013 = call @sloth_rt_print_str(%v1012) : (i64) -> i64
    call @sloth_rc_release(%v1012) : (i64) -> i64
    %v1014 = arith.constant 0 : i64
    %v1015 = arith.constant 6513249 : i64
    %v1016 = arith.constant 3 : i64
    %v1017 = call @sloth_str_push(%v1014, %v1015, %v1016) : (i64, i64, i64) -> i64
    %v1018 = call @sloth_str_finish(%v1017) : (i64) -> i64
    %v1019 = call @sloth_str_len(%v1018) : (i64) -> i64
    %v1020 = call @sloth_rt_print_i64(%v1019) : (i64) -> i64
    call @sloth_rc_release(%v1018) : (i64) -> i64
    %v1021 = arith.constant 1 : i64
    %v1022 = arith.constant 2 : i64
    %v1023 = arith.constant 3 : i64
    %v1024 = arith.constant 3 : i64
    %v1025 = call @sloth_arr_new(%v1024) : (i64) -> i64
    %v1026 = arith.constant 0 : i64
    call @sloth_arr_set(%v1025, %v1026, %v1021) : (i64, i64, i64) -> i64
    %v1027 = arith.constant 1 : i64
    call @sloth_arr_set(%v1025, %v1027, %v1022) : (i64, i64, i64) -> i64
    %v1028 = arith.constant 2 : i64
    call @sloth_arr_set(%v1025, %v1028, %v1023) : (i64, i64, i64) -> i64
    %v1029 = call @sloth_arr_len(%v1025) : (i64) -> i64
    %v1030 = call @sloth_rt_print_i64(%v1029) : (i64) -> i64
    call @sloth_rc_release(%v1025) : (i64) -> i64
    %v1031 = arith.constant 1 : i64
    %v1032 = arith.constant 1 : i64
    %v1033 = arith.constant 0 : i64
    %v1034 = call @sloth_map_new(%v1033) : (i64) -> i64
    call @sloth_map_set(%v1034, %v1031, %v1032) : (i64, i64, i64) -> i64
    %v1035 = call @sloth_map_len(%v1034) : (i64) -> i64
    %v1036 = call @sloth_rt_print_i64(%v1035) : (i64) -> i64
    call @sloth_rc_release(%v1034) : (i64) -> i64
    %v1037 = arith.constant 4615964438073389875 : i64
    %v1039 = llvm.bitcast %v1037 : i64 to f64
    %v1040 = arith.fptosi %v1039 : f64 to i64
    %v1041 = call @sloth_rt_print_i64(%v1040) : (i64) -> i64
    %v1042 = arith.constant 3 : i64
    %v1044 = arith.sitofp %v1042 : i64 to f64
    %v1045 = llvm.bitcast %v1044 : f64 to i64
    %v1047 = llvm.bitcast %v1045 : i64 to f64
    %v1046 = call @sloth_rt_print_f64(%v1047) : (f64) -> i64
    %v1048 = arith.constant 0 : i64
    %v1049 = arith.constant 107 : i64
    %v1050 = arith.constant 1 : i64
    %v1051 = call @sloth_str_push(%v1048, %v1049, %v1050) : (i64, i64, i64) -> i64
    %v1052 = call @sloth_str_finish(%v1051) : (i64) -> i64
    %v1053 = arith.constant 1 : i64
    %v1054 = arith.constant 1 : i64
    %v1055 = call @sloth_map_new(%v1054) : (i64) -> i64
    %v1056 = call @sloth_rc_retain(%v1052) : (i64) -> i64
    call @sloth_map_str_set(%v1055, %v1052, %v1053) : (i64, i64, i64) -> i64
    %v1057 = memref.alloca() : memref<1xi64>
    %v1058 = call @sloth_rc_retain(%v1055) : (i64) -> i64
    %v1059 = arith.constant 0 : index
    memref.store %v1058, %v1057[%v1059] : memref<1xi64>
    %v1060 = memref.extract_aligned_pointer_as_index %v1057 : memref<1xi64> -> index
    %v1061 = arith.index_cast %v1060 : index to i64
    call @sloth_fiber_track(%v1061) : (i64) -> i64
    call @sloth_rc_release(%v1052) : (i64) -> i64
    call @sloth_rc_release(%v1055) : (i64) -> i64
    %v1062 = arith.constant 0 : index
    %v1063 = memref.load %v1057[%v1062] : memref<1xi64>
    %v1064 = call @sloth_map_keys(%v1063) : (i64) -> i64
    %v1065 = call @sloth_arr_len(%v1064) : (i64) -> i64
    %v1066 = call @sloth_rt_print_i64(%v1065) : (i64) -> i64
    call @sloth_rc_release(%v1064) : (i64) -> i64
    %v1067 = arith.constant 0 : index
    %v1068 = memref.load %v1057[%v1067] : memref<1xi64>
    %v1069 = call @sloth_map_values(%v1068) : (i64) -> i64
    %v1070 = call @sloth_arr_len(%v1069) : (i64) -> i64
    %v1071 = call @sloth_rt_print_i64(%v1070) : (i64) -> i64
    call @sloth_rc_release(%v1069) : (i64) -> i64
    %v1072 = call @sloth_rc_live() : () -> i64
    %v1073 = arith.constant 0 : i64
    %v1075 = arith.constant 0 : i64
    %v1074 = arith.cmpi sge, %v1072, %v1073 : i64
    %v1076 = arith.extui %v1074 : i1 to i64
    %v1077 = call @sloth_rt_print_bool(%v1076) : (i64) -> i64
    %v1078 = arith.constant 0 : index
    %v1079 = memref.load %v1057[%v1078] : memref<1xi64>
    call @sloth_rc_release(%v1079) : (i64) -> i64
    %v1080 = memref.extract_aligned_pointer_as_index %v1057 : memref<1xi64> -> index
    %v1081 = arith.index_cast %v1080 : index to i64
    call @sloth_fiber_untrack(%v1081) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

