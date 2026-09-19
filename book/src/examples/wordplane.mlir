module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 1 : i64
    %v1002 = arith.constant 2 : i64
    %v1003 = arith.addi %v1001, %v1002 : i64
    %v1004 = call @sloth_rt_print_i64(%v1003) : (i64) -> i64
    %v1005 = arith.constant 4609434218613702656 : i64
    %v1006 = arith.constant 4598175219545276416 : i64
    %v1007 = llvm.bitcast %v1005 : i64 to f64
    %v1008 = llvm.bitcast %v1006 : i64 to f64
    %v1009 = arith.addf %v1007, %v1008 : f64
    %v1010 = llvm.bitcast %v1009 : f64 to i64
    %v1012 = llvm.bitcast %v1010 : i64 to f64
    %v1011 = call @sloth_rt_print_f64(%v1012) : (f64) -> i64
    %v1013 = arith.constant 3 : i64
    %v1014 = arith.constant 4 : i64
    %v1016 = arith.constant 0 : i64
    %v1015 = arith.cmpi slt, %v1013, %v1014 : i64
    %v1017 = arith.extui %v1015 : i1 to i64
    %v1018 = call @sloth_rt_print_bool(%v1017) : (i64) -> i64
    %v1019 = arith.constant 0 : i64
    %v1020 = arith.constant 0 : i64
    %v1021 = arith.cmpi eq, %v1019, %v1020 : i64
    %v1022 = arith.extui %v1021 : i1 to i64
    %v1023 = call @sloth_rt_print_bool(%v1022) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

