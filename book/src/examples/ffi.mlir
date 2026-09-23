module @main {
  llvm.mlir.global private constant @sloth_tynm_0("float\00") {addr_space = 0 : i32}
  llvm.mlir.global private constant @sloth_tynm_1("int\00") {addr_space = 0 : i32}
  memref.global @sloth_anyd_0 : memref<11xi64> = dense<0> {mutable}
  memref.global @sloth_anyd_1 : memref<11xi64> = dense<0> {mutable}
  func.func @sloth_main__ginit() {
    call @sloth_main__anyinit() : () -> ()
    return
  }
  func.func @sloth_main__print(%arg0: i64) {
    %c0 = arith.constant 0 : index
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %arg0, %alloca[%c0] : memref<1xi64>
    %0 = memref.load %alloca[%c0] : memref<1xi64>
    %1 = call @sloth_rt_write(%0) : (i64) -> i64
    call @sloth_rt_puts(%1) : (i64) -> ()
    %2 = call @sloth_rc_release(%1) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func private @sloth_extern_floor(f64) -> f64
  func.func private @sloth_extern_powf(f64, f64) -> f64
  func.func private @sloth_extern_tok_new() -> i64
  func.func private @sloth_extern_tok_val(i64) -> i64
  func.func @sloth_main() attributes {llvm.emit_c_interface} {
    %c0 = arith.constant 0 : index
    %c4621819117588971520_i64 = arith.constant 4621819117588971520 : i64
    %c4611686018427387904_i64 = arith.constant 4611686018427387904 : i64
    %c4613262278296967578_i64 = arith.constant 4613262278296967578 : i64
    call @sloth_main__anyinit() : () -> ()
    call @sloth_main__ginit() : () -> ()
    %0 = llvm.bitcast %c4613262278296967578_i64 : i64 to f64
    %1 = call @sloth_extern_floor(%0) : (f64) -> f64
    %2 = llvm.bitcast %1 : f64 to i64
    %3 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr = memref.extract_aligned_pointer_as_index %3 : memref<11xi64> -> index
    %4 = arith.index_cast %intptr : index to i64
    %5 = call @sloth_any_from(%4, %2) : (i64, i64) -> i64
    call @sloth_main__print(%5) : (i64) -> ()
    %6 = call @sloth_rc_release(%5) : (i64) -> i64
    %7 = llvm.bitcast %c4611686018427387904_i64 : i64 to f64
    %8 = llvm.bitcast %c4621819117588971520_i64 : i64 to f64
    %9 = call @sloth_extern_powf(%7, %8) : (f64, f64) -> f64
    %10 = llvm.bitcast %9 : f64 to i64
    %11 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    %intptr_0 = memref.extract_aligned_pointer_as_index %11 : memref<11xi64> -> index
    %12 = arith.index_cast %intptr_0 : index to i64
    %13 = call @sloth_any_from(%12, %10) : (i64, i64) -> i64
    call @sloth_main__print(%13) : (i64) -> ()
    %14 = call @sloth_rc_release(%13) : (i64) -> i64
    %15 = call @sloth_extern_tok_new() : () -> i64
    %alloca = memref.alloca() : memref<1xi64>
    memref.store %15, %alloca[%c0] : memref<1xi64>
    %16 = memref.load %alloca[%c0] : memref<1xi64>
    %17 = call @sloth_extern_tok_val(%16) : (i64) -> i64
    %18 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    %intptr_1 = memref.extract_aligned_pointer_as_index %18 : memref<11xi64> -> index
    %19 = arith.index_cast %intptr_1 : index to i64
    %20 = call @sloth_any_from(%19, %17) : (i64, i64) -> i64
    call @sloth_main__print(%20) : (i64) -> ()
    %21 = call @sloth_rc_release(%20) : (i64) -> i64
    cf.br ^bb1
  ^bb1:  // pred: ^bb0
    return
  }
  func.func @sloth_main__anyinit() {
    %0 = llvm.mlir.addressof @sloth_tynm_1 : !llvm.ptr
    %c1099511627777_i64 = arith.constant 1099511627777 : i64
    %c2_i64 = arith.constant 2 : i64
    %c10 = arith.constant 10 : index
    %c9 = arith.constant 9 : index
    %c4 = arith.constant 4 : index
    %c5_i64 = arith.constant 5 : i64
    %c3 = arith.constant 3 : index
    %1 = llvm.mlir.addressof @sloth_tynm_0 : !llvm.ptr
    %c2 = arith.constant 2 : index
    %c1099511627776_i64 = arith.constant 1099511627776 : i64
    %c1 = arith.constant 1 : index
    %c0_i64 = arith.constant 0 : i64
    %c0 = arith.constant 0 : index
    %c3_i64 = arith.constant 3 : i64
    %2 = memref.get_global @sloth_anyd_0 : memref<11xi64>
    memref.store %c3_i64, %2[%c0] : memref<11xi64>
    memref.store %c0_i64, %2[%c1] : memref<11xi64>
    memref.store %c1099511627776_i64, %2[%c2] : memref<11xi64>
    %3 = llvm.ptrtoint %1 : !llvm.ptr to i64
    memref.store %3, %2[%c3] : memref<11xi64>
    memref.store %c5_i64, %2[%c4] : memref<11xi64>
    memref.store %c0_i64, %2[%c9] : memref<11xi64>
    memref.store %c0_i64, %2[%c10] : memref<11xi64>
    %4 = memref.get_global @sloth_anyd_1 : memref<11xi64>
    memref.store %c2_i64, %4[%c0] : memref<11xi64>
    memref.store %c0_i64, %4[%c1] : memref<11xi64>
    memref.store %c1099511627777_i64, %4[%c2] : memref<11xi64>
    %5 = llvm.ptrtoint %0 : !llvm.ptr to i64
    memref.store %5, %4[%c3] : memref<11xi64>
    memref.store %c3_i64, %4[%c4] : memref<11xi64>
    memref.store %c0_i64, %4[%c9] : memref<11xi64>
    memref.store %c0_i64, %4[%c10] : memref<11xi64>
    return
  }
}

