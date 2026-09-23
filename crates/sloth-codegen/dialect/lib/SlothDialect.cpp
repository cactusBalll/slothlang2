#include "Sloth/SlothDialect.h"
#include "Sloth/SlothOps.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Transforms/GreedyPatternRewriteDriver.h"

using namespace sloth;

#include "Sloth/SlothDialect.cpp.inc"

void SlothDialect::initialize() {
  addOperations<
#define GET_OP_LIST
#include "Sloth/SlothOps.cpp.inc"
  >();
}
