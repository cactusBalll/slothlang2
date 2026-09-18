module @main {
  memref.global @sloth_modules_lib_g_counter : memref<1xi64> = dense<0> {mutable}

  func.func @sloth_modules_lib__double(%p0: i64) -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1003 = arith.constant 0 : index
    %v1002 = memref.alloca() : memref<1xi64>
    memref.store %p0, %v1002[%v1003] : memref<1xi64>
    %v1004 = arith.constant 0 : index
    %v1005 = memref.load %v1002[%v1004] : memref<1xi64>
    %v1006 = arith.constant 4 : i64
    %v1007 = arith.constant 1 : i64
    %v1008 = arith.shrsi %v1005, %v1007 : i64
    %v1009 = arith.constant 1 : i64
    %v1010 = arith.shrsi %v1006, %v1009 : i64
    %v1011 = arith.muli %v1008, %v1010 : i64
    %v1012 = arith.constant 1 : i64
    %v1013 = arith.shli %v1011, %v1012 : i64
    %v1014 = arith.constant 0 : index
    memref.store %v1013, %v1001[%v1014] : memref<1xi64>
    %v1015 = arith.constant 1 : i64
    memref.store %v1015, %v1000[%v1014] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1016 = arith.constant 0 : index
    %v1017 = memref.load %v1001[%v1016] : memref<1xi64>
    return %v1017 : i64
  }
  func.func @sloth_modules_lib__hidden() -> i64 {
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = memref.alloca() : memref<1xi64>
    %v1002 = arith.constant 0 : i64
    %v1003 = arith.constant 0 : index
    memref.store %v1002, %v1001[%v1003] : memref<1xi64>
    %v1004 = arith.constant 1 : i64
    memref.store %v1004, %v1000[%v1003] : memref<1xi64>
    cf.br ^end
  ^end:
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1001[%v1005] : memref<1xi64>
    return %v1006 : i64
  }
  func.func @sloth_modules_lib__ginit() -> () {
    %v1000 = arith.constant 10 : i64
    %v1001 = memref.get_global @sloth_modules_lib_g_counter : memref<1xi64>
    %v1002 = arith.constant 0 : index
    memref.store %v1000, %v1001[%v1002] : memref<1xi64>
    return
  }
  func.func @sloth_main__ginit() -> () {
    return
  }
  func.func @sloth_main() -> () attributes {llvm.emit_c_interface} {
    call @sloth_modules_lib__ginit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %v1000 = memref.alloca() : memref<1xi64>
    %v1001 = arith.constant 42 : i64
    %v1002 = call @sloth_modules_lib__double(%v1001) : (i64) -> i64
    %v1003 = call @sloth_rt_print_i64(%v1002) : (i64) -> i64
    %v1004 = memref.get_global @sloth_modules_lib_g_counter : memref<1xi64>
    %v1005 = arith.constant 0 : index
    %v1006 = memref.load %v1004[%v1005] : memref<1xi64>
    %v1007 = call @sloth_rt_print_i64(%v1006) : (i64) -> i64
    cf.br ^end
  ^end:
    return
  }
}

