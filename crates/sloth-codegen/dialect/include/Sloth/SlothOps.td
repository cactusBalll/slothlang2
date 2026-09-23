#ifndef SLOTH_SLOTHOPS_TD
#define SLOTH_SLOTHOPS_TD

include "mlir/IR/AttrTypeBase.td"
include "mlir/IR/OpBase.td"
include "mlir/IR/CommonTypeConstraints.td"
include "mlir/Interfaces/SideEffectInterfaces.td"

def Sloth_Dialect : Dialect {
  let name = "sloth";
  let summary = "Sloth language semantic dialect (ARC / word-plane / rt calls)";
  let description = [{
    Carries language-level semantic operations emitted by the Sloth codegen
    and lowered to standard dialects (`func`/`arith`/`memref`/`llvm`) before
    the shared LLVM lowering pipeline.
  }];
  let cppNamespace = "::sloth";
  let useDefaultTypePrinterParser = 0;
}

class Sloth_Op<string mnemonic, list<Trait> traits = []> :
    Op<Sloth_Dialect, mnemonic, traits>;

//===----------------------------------------------------------------------===//
// ARC
//===----------------------------------------------------------------------===//

def RcRetainOp : Sloth_Op<"rc_retain"> {
  let summary = "ARC retain: +1 on a reference word (lowers to @sloth_rc_retain)";
  let arguments = (ins I64:$ref);
  let results = (outs I64:$out);
  let assemblyFormat = "$ref attr-dict `:` type($out)";
}

def RcReleaseOp : Sloth_Op<"rc_release"> {
  let summary = "ARC release: -1 on a reference word (lowers to @sloth_rc_release)";
  let arguments = (ins I64:$ref);
  let assemblyFormat = "$ref attr-dict `:` type($ref)";
}

#endif // SLOTH_SLOTHOPS_TD
