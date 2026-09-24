#include "Sloth/SlothOps.h"
#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/PatternMatch.h"
#include "mlir/Transforms/GreedyPatternRewriteDriver.h"

namespace sloth {
namespace {

using mlir::OpRewritePattern;
using mlir::PatternRewriter;

/// %r = sloth.rc_retain %h : i64
///   -> %r = func.call @__sloth_rc_retain(%h) : (i64) -> i64
struct LowerRcRetain : public mlir::OpRewritePattern<RcRetainOp> {
  using OpRewritePattern = mlir::OpRewritePattern<RcRetainOp>;
  using OpRewritePattern::OpRewritePattern;

  mlir::LogicalResult matchAndRewrite(RcRetainOp op,
                                      PatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto resTy = op.getOut().getType();
    auto call = rewriter.create<mlir::func::CallOp>(
        loc, rewriter.getStringAttr("__sloth_rc_retain"),
        mlir::TypeRange{resTy}, mlir::ValueRange{op.getRef()});
    rewriter.replaceOp(op, call.getResult(0));
    return mlir::success();
  }
};

/// sloth.rc_release %h : i64
///   -> func.call @__sloth_rc_release(%h) : (i64) -> i64  (result discarded)
struct LowerRcRelease : public OpRewritePattern<RcReleaseOp> {
  using OpRewritePattern::OpRewritePattern;

  mlir::LogicalResult matchAndRewrite(RcReleaseOp op,
                                      PatternRewriter &rewriter) const override {
    auto loc = op.getLoc();
    auto i64 = rewriter.getI64Type();
    rewriter.create<mlir::func::CallOp>(
        loc, rewriter.getStringAttr("__sloth_rc_release"), mlir::TypeRange{i64},
        mlir::ValueRange{op.getRef()});
    rewriter.eraseOp(op);
    return mlir::success();
  }
};

} // namespace

void populateSlothLoweringPatterns(mlir::RewritePatternSet &patterns) {
  patterns.add<LowerRcRetain, LowerRcRelease>(patterns.getContext());
}

} // namespace sloth
