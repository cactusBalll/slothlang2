module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 2 : i64
    %v1002 = arith.constant 4 : i64
    %v1003 = arith.constant 1 : i64
    %v1004 = arith.shrsi %v1001, %v1003 : i64
    %v1005 = arith.constant 1 : i64
    %v1006 = arith.shrsi %v1002, %v1005 : i64
    %v1007 = arith.addi %v1004, %v1006 : i64
    %v1008 = arith.constant 1 : i64
    %v1009 = arith.shli %v1007, %v1008 : i64
    %v1010 = call @sloth_rt_print_i64(%v1009) : (i64) -> i64
    %v1011 = arith.constant 2304717109306851328 : i64
    %v1012 = arith.constant 2299087609772638208 : i64
    %v1013 = arith.constant 1 : i64
    %v1014 = arith.shli %v1011, %v1013 : i64
    %v1015 = llvm.bitcast %v1014 : i64 to f64
    %v1016 = arith.constant 1 : i64
    %v1017 = arith.shli %v1012, %v1016 : i64
    %v1018 = llvm.bitcast %v1017 : i64 to f64
    %v1019 = arith.addf %v1015, %v1018 : f64
    %v1020 = llvm.bitcast %v1019 : f64 to i64
    %v1021 = arith.constant -3 : i64
    %v1022 = arith.andi %v1020, %v1021 : i64
    %v1023 = arith.constant 1 : i64
    %v1024 = arith.shrsi %v1022, %v1023 : i64
    %v1026 = arith.constant 1 : i64
    %v1027 = arith.shli %v1024, %v1026 : i64
    %v1028 = llvm.bitcast %v1027 : i64 to f64
    %v1025 = call @sloth_rt_print_f64(%v1028) : (f64) -> i64
    %v1029 = arith.constant 6 : i64
    %v1030 = arith.constant 8 : i64
    %v1032 = arith.constant 0 : i64
    %v1031 = arith.cmpi slt, %v1029, %v1030 : i64
    %v1033 = arith.extui %v1031 : i1 to i64
    %v1034 = arith.constant 1 : i64
    %v1035 = arith.shli %v1033, %v1034 : i64
    %v1036 = call @sloth_rt_print_bool(%v1035) : (i64) -> i64
    %v1037 = arith.constant 0 : i64
    %v1038 = arith.constant 0 : i64
    %v1039 = arith.cmpi eq, %v1037, %v1038 : i64
    %v1040 = arith.extui %v1039 : i1 to i64
    %v1041 = arith.constant 1 : i64
    %v1042 = arith.shli %v1040, %v1041 : i64
    %v1043 = call @sloth_rt_print_bool(%v1042) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

