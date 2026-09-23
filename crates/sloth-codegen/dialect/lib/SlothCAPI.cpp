#include "Sloth/SlothCAPI.h"
#include "Sloth/SlothDialect.h"
#include "Sloth/SlothOps.h"
#include "mlir/CAPI/IR.h"
#include "mlir/CAPI/Support.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/PatternMatch.h"
#include "mlir/Transforms/GreedyPatternRewriteDriver.h"

namespace sloth {
void populateSlothLoweringPatterns(mlir::RewritePatternSet &patterns);
} // namespace sloth

extern "C" void slothRegisterDialect(MlirDialectRegistry registry) {
  auto *reg = unwrap(registry); // returns DialectRegistry*
  reg->insert<sloth::SlothDialect>();
}

extern "C" MlirLogicalResult slothLowerModule(MlirModule module) {
  auto mod = unwrap(module);
  mlir::RewritePatternSet patterns(mod.getContext());
  sloth::populateSlothLoweringPatterns(patterns);
  mlir::FrozenRewritePatternSet frozen(std::move(patterns));
  (void)mlir::applyPatternsGreedily(mod.getOperation(), frozen);

  // leak gate: any remaining sloth.* op fails the lowering
  mlir::LogicalResult leftover = mlir::success();
  mod->walk([&](mlir::Operation *op) {
    if (auto *dialect = op->getDialect()) {
      if (dialect->getNamespace() == "sloth")
        leftover = mlir::failure();
    }
  });
  return wrap(leftover);
}
