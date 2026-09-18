module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func private @sloth_extern_floor(f64) -> f64
  func.func private @sloth_extern_powf(f64, f64) -> f64
  func.func private @sloth_extern_tok_new() -> i64
  func.func private @sloth_extern_tok_val(i64) -> i64
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 2306631139148483789 : i64
    %v1003 = arith.constant 1 : i64
    %v1004 = arith.shli %v1001, %v1003 : i64
    %v1005 = llvm.bitcast %v1004 : i64 to f64
    %v1006 = call @sloth_extern_floor(%v1005) : (f64) -> f64
    %v1007 = llvm.bitcast %v1006 : f64 to i64
    %v1008 = arith.constant -3 : i64
    %v1009 = arith.andi %v1007, %v1008 : i64
    %v1010 = arith.constant 1 : i64
    %v1011 = arith.shrsi %v1009, %v1010 : i64
    %v1013 = arith.constant 1 : i64
    %v1014 = arith.shli %v1011, %v1013 : i64
    %v1015 = llvm.bitcast %v1014 : i64 to f64
    %v1012 = call @sloth_rt_print_f64(%v1015) : (f64) -> i64
    %v1016 = arith.constant 2305843009213693952 : i64
    %v1017 = arith.constant 2310909558794485760 : i64
    %v1019 = arith.constant 1 : i64
    %v1020 = arith.shli %v1016, %v1019 : i64
    %v1021 = llvm.bitcast %v1020 : i64 to f64
    %v1022 = arith.constant 1 : i64
    %v1023 = arith.shli %v1017, %v1022 : i64
    %v1024 = llvm.bitcast %v1023 : i64 to f64
    %v1025 = call @sloth_extern_powf(%v1021, %v1024) : (f64, f64) -> f64
    %v1026 = llvm.bitcast %v1025 : f64 to i64
    %v1027 = arith.constant -3 : i64
    %v1028 = arith.andi %v1026, %v1027 : i64
    %v1029 = arith.constant 1 : i64
    %v1030 = arith.shrsi %v1028, %v1029 : i64
    %v1032 = arith.constant 1 : i64
    %v1033 = arith.shli %v1030, %v1032 : i64
    %v1034 = llvm.bitcast %v1033 : i64 to f64
    %v1031 = call @sloth_rt_print_f64(%v1034) : (f64) -> i64
    %v1036 = call @sloth_extern_tok_new() : () -> i64
    %v1037 = memref.alloca() : memref<1xi64>
    %v1038 = call @sloth_rc_retain(%v1036) : (i64) -> i64
    %v1039 = arith.constant 0 : index
    memref.store %v1038, %v1037[%v1039] : memref<1xi64>
    %v1040 = arith.constant 0 : index
    %v1041 = memref.load %v1037[%v1040] : memref<1xi64>
    %v1043 = call @sloth_extern_tok_val(%v1041) : (i64) -> i64
    %v1044 = arith.constant 1 : i64
    %v1045 = arith.shli %v1043, %v1044 : i64
    %v1046 = call @sloth_rt_print_i64(%v1045) : (i64) -> i64
    %v1047 = arith.constant 0 : index
    %v1048 = memref.load %v1037[%v1047] : memref<1xi64>
    call @sloth_rc_release(%v1048) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

