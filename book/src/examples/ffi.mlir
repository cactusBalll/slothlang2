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
    %v1001 = arith.constant 4613262278296967578 : i64
    %v1003 = llvm.bitcast %v1001 : i64 to f64
    %v1004 = call @sloth_extern_floor(%v1003) : (f64) -> f64
    %v1005 = llvm.bitcast %v1004 : f64 to i64
    %v1007 = llvm.bitcast %v1005 : i64 to f64
    %v1006 = call @sloth_rt_print_f64(%v1007) : (f64) -> i64
    %v1008 = arith.constant 4611686018427387904 : i64
    %v1009 = arith.constant 4621819117588971520 : i64
    %v1011 = llvm.bitcast %v1008 : i64 to f64
    %v1012 = llvm.bitcast %v1009 : i64 to f64
    %v1013 = call @sloth_extern_powf(%v1011, %v1012) : (f64, f64) -> f64
    %v1014 = llvm.bitcast %v1013 : f64 to i64
    %v1016 = llvm.bitcast %v1014 : i64 to f64
    %v1015 = call @sloth_rt_print_f64(%v1016) : (f64) -> i64
    %v1018 = call @sloth_extern_tok_new() : () -> i64
    %v1019 = memref.alloca() : memref<1xi64>
    %v1020 = arith.constant 0 : index
    memref.store %v1018, %v1019[%v1020] : memref<1xi64>
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1019[%v1021] : memref<1xi64>
    %v1024 = call @sloth_extern_tok_val(%v1022) : (i64) -> i64
    %v1025 = call @sloth_rt_print_i64(%v1024) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

