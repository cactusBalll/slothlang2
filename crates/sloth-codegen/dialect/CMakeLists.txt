cmake_minimum_required(VERSION 3.20)
project(SlothDialect LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

find_package(MLIR REQUIRED CONFIG)

message(STATUS "Using MLIRConfig.cmake in: ${MLIR_DIR}")

# mlir-tblgen on PATH (mlir-21-tools) or under the MLIR install prefix.
find_program(MLIR_TBLGEN_EXE
  NAMES mlir-tblgen
  PATHS
    "${MLIR_DIR}/../../../bin"
    "${MLIR_INSTALL_PREFIX}/bin"
    "/usr/lib/llvm-21/bin"
    "/usr/bin"
  NO_DEFAULT_PATH
)
if(NOT MLIR_TBLGEN_EXE)
  find_program(MLIR_TBLGEN_EXE NAMES mlir-tblgen)
endif()
if(NOT MLIR_TBLGEN_EXE)
  message(FATAL_ERROR "mlir-tblgen not found")
endif()
message(STATUS "Using mlir-tblgen: ${MLIR_TBLGEN_EXE}")

set(SLOTH_TD_DIR ${CMAKE_CURRENT_SOURCE_DIR}/include)
set(SLOTH_BIN_DIR ${CMAKE_CURRENT_BINARY_DIR})
set(MLIR_TD_INCLUDE ${MLIR_INCLUDE_DIRS})


# --- TableGen rules (standalone; avoid LLVM-internal add_public_tablegen_target) ---
set(SLOTH_TD ${SLOTH_TD_DIR}/Sloth/SlothOps.td)
set(SLOTH_TD_OUTS
  ${SLOTH_BIN_DIR}/Sloth/SlothDialect.h.inc
  ${SLOTH_BIN_DIR}/Sloth/SlothDialect.cpp.inc
  ${SLOTH_BIN_DIR}/Sloth/SlothOps.h.inc
  ${SLOTH_BIN_DIR}/Sloth/SlothOps.cpp.inc
)

add_custom_command(
  OUTPUT
    ${SLOTH_BIN_DIR}/Sloth/SlothDialect.h.inc
    ${SLOTH_BIN_DIR}/Sloth/SlothDialect.cpp.inc
    ${SLOTH_BIN_DIR}/Sloth/SlothOps.h.inc
    ${SLOTH_BIN_DIR}/Sloth/SlothOps.cpp.inc
  COMMAND ${CMAKE_COMMAND} -E make_directory ${SLOTH_BIN_DIR}/Sloth
  COMMAND ${MLIR_TBLGEN_EXE} -gen-dialect-decls -dialect=sloth
          -I ${MLIR_TD_INCLUDE} -I ${SLOTH_TD_DIR}
          ${SLOTH_TD} -o ${SLOTH_BIN_DIR}/Sloth/SlothDialect.h.inc
  COMMAND ${MLIR_TBLGEN_EXE} -gen-dialect-defs -dialect=sloth
          -I ${MLIR_TD_INCLUDE} -I ${SLOTH_TD_DIR}
          ${SLOTH_TD} -o ${SLOTH_BIN_DIR}/Sloth/SlothDialect.cpp.inc
  COMMAND ${MLIR_TBLGEN_EXE} -gen-op-decls
          -I ${MLIR_TD_INCLUDE} -I ${SLOTH_TD_DIR}
          ${SLOTH_TD} -o ${SLOTH_BIN_DIR}/Sloth/SlothOps.h.inc
  COMMAND ${MLIR_TBLGEN_EXE} -gen-op-defs
          -I ${MLIR_TD_INCLUDE} -I ${SLOTH_TD_DIR}
          ${SLOTH_TD} -o ${SLOTH_BIN_DIR}/Sloth/SlothOps.cpp.inc
  DEPENDS ${SLOTH_TD}
          ${SLOTH_TD_DIR}/Sloth/SlothDialect.h
          ${SLOTH_TD_DIR}/Sloth/SlothOps.h
  COMMENT "Generating sloth dialect TableGen outputs"
  VERBATIM
)
add_custom_target(sloth_tablegen DEPENDS ${SLOTH_TD_OUTS})

add_library(sloth_dialect STATIC
  lib/SlothDialect.cpp
  lib/SlothOps.cpp
  lib/SlothLowering.cpp
  lib/SlothCAPI.cpp
)
add_dependencies(sloth_dialect sloth_tablegen)

target_include_directories(sloth_dialect PRIVATE
  ${MLIR_INCLUDE_DIRS}
  ${LLVM_INCLUDE_DIRS}
  ${SLOTH_TD_DIR}
  ${SLOTH_BIN_DIR}
)

# Do NOT link MLIR: symbols resolve at final link via mlir-sys.
