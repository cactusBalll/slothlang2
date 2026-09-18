module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 448630058099 : i64
    %v1003 = arith.constant 10 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1006[%v1008] : memref<1xi64>
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1009 = arith.constant 0 : i64
    %v1010 = arith.constant 9056056326776168 : i64
    %v1011 = arith.constant 14 : i64
    %v1012 = call @sloth_str_push(%v1009, %v1010, %v1011) : (i64, i64, i64) -> i64
    %v1013 = arith.constant 0 : index
    %v1014 = memref.load %v1006[%v1013] : memref<1xi64>
    %v1015 = call @sloth_str_pushp(%v1012, %v1014) : (i64, i64) -> i64
    %v1016 = arith.constant 33 : i64
    %v1017 = arith.constant 2 : i64
    %v1018 = call @sloth_str_push(%v1015, %v1016, %v1017) : (i64, i64, i64) -> i64
    %v1019 = call @sloth_str_finish(%v1018) : (i64) -> i64
    %v1020 = call @sloth_rt_print_str(%v1019) : (i64) -> i64
    call @sloth_rc_release(%v1019) : (i64) -> i64
    %v1021 = arith.constant 0 : index
    %v1022 = memref.load %v1006[%v1021] : memref<1xi64>
    call @sloth_rc_release(%v1022) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

