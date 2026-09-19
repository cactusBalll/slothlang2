module @main {

  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 0 : i64
    %v1002 = arith.constant 448630058099 : i64
    %v1003 = arith.constant 5 : i64
    %v1004 = call @sloth_str_push(%v1001, %v1002, %v1003) : (i64, i64, i64) -> i64
    %v1005 = call @sloth_str_finish(%v1004) : (i64) -> i64
    %v1006 = memref.alloca() : memref<1xi64>
    %v1007 = call @sloth_rc_retain(%v1005) : (i64) -> i64
    %v1008 = arith.constant 0 : index
    memref.store %v1007, %v1006[%v1008] : memref<1xi64>
    %v1009 = memref.extract_aligned_pointer_as_index %v1006 : memref<1xi64> -> index
    %v1010 = arith.index_cast %v1009 : index to i64
    call @sloth_fiber_track(%v1010) : (i64) -> i64
    call @sloth_rc_release(%v1005) : (i64) -> i64
    %v1011 = arith.constant 0 : i64
    %v1012 = arith.constant 9056056326776168 : i64
    %v1013 = arith.constant 7 : i64
    %v1014 = call @sloth_str_push(%v1011, %v1012, %v1013) : (i64, i64, i64) -> i64
    %v1015 = arith.constant 0 : index
    %v1016 = memref.load %v1006[%v1015] : memref<1xi64>
    %v1017 = call @sloth_str_pushp(%v1014, %v1016) : (i64, i64) -> i64
    %v1018 = arith.constant 33 : i64
    %v1019 = arith.constant 1 : i64
    %v1020 = call @sloth_str_push(%v1017, %v1018, %v1019) : (i64, i64, i64) -> i64
    %v1021 = call @sloth_str_finish(%v1020) : (i64) -> i64
    %v1022 = call @sloth_rt_print_str(%v1021) : (i64) -> i64
    call @sloth_rc_release(%v1021) : (i64) -> i64
    %v1023 = arith.constant 0 : index
    %v1024 = memref.load %v1006[%v1023] : memref<1xi64>
    call @sloth_rc_release(%v1024) : (i64) -> i64
    %v1025 = memref.extract_aligned_pointer_as_index %v1006 : memref<1xi64> -> index
    %v1026 = arith.index_cast %v1025 : index to i64
    call @sloth_fiber_untrack(%v1026) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

