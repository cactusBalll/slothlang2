#ifndef SLOTH_SLOTHCAPI_H
#define SLOTH_SLOTHCAPI_H

#include <mlir-c/IR.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Register the sloth dialect into a dialect registry (call after
/// mlirRegisterAllDialects, before context load).
void slothRegisterDialect(MlirDialectRegistry registry);

/// Lower every `sloth.*` op in the module to standard dialects.
/// Returns mlirLogicalResultSuccess when no `sloth.*` op remains.
MlirLogicalResult slothLowerModule(MlirModule module);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // SLOTH_SLOTHCAPI_H
