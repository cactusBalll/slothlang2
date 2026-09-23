#ifndef SLOTH_SLOTHOPS_H
#define SLOTH_SLOTHOPS_H

#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/Dialect.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/OpImplementation.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"
#include "Sloth/SlothDialect.h"

#define GET_OP_CLASSES
#include "Sloth/SlothOps.h.inc"

#endif // SLOTH_SLOTHOPS_H
