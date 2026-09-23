//! Build the C++/TableGen `sloth` dialect sidecar (scheme A2) and link it
//! into sloth-codegen. MLIR symbols are provided by mlir-sys at final link;
//! this static library only compiles against MLIR headers.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dialect_src = manifest_dir.join("dialect");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("dialect-build");

    // Re-run when any dialect source changes.
    println!("cargo:rerun-if-changed=dialect/CMakeLists.txt");
    println!("cargo:rerun-if-changed=dialect/include/Sloth/SlothOps.td");
    println!("cargo:rerun-if-changed=dialect/include/Sloth/SlothDialect.h");
    println!("cargo:rerun-if-changed=dialect/include/Sloth/SlothOps.h");
    println!("cargo:rerun-if-changed=dialect/include/Sloth/SlothCAPI.h");
    println!("cargo:rerun-if-changed=dialect/lib/SlothDialect.cpp");
    println!("cargo:rerun-if-changed=dialect/lib/SlothOps.cpp");
    println!("cargo:rerun-if-changed=dialect/lib/SlothLowering.cpp");
    println!("cargo:rerun-if-changed=dialect/lib/SlothCAPI.cpp");
    println!("cargo:rerun-if-env-changed=MLIR_SYS_210_PREFIX");

    let prefix = env::var("MLIR_SYS_210_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());
    let mlir_dir = PathBuf::from(&prefix).join("lib/cmake/mlir");
    if !mlir_dir.join("MLIRConfig.cmake").exists() {
        panic!(
            "MLIRConfig.cmake not found at {:?}; set MLIR_SYS_210_PREFIX",
            mlir_dir
        );
    }

    // Prefer Ninja when available for faster incremental dialect builds.
    let gen = if Command::new("ninja")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        vec!["-G".to_string(), "Ninja".to_string()]
    } else {
        vec![]
    };

    let status = Command::new("cmake")
        .arg("-S")
        .arg(&dialect_src)
        .arg("-B")
        .arg(&build_dir)
        .args(&gen)
        .arg(format!("-DMLIR_DIR={}", mlir_dir.display()))
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .status()
        .expect("failed to run cmake configure for sloth dialect");
    assert!(status.success(), "cmake configure failed");

    let status = Command::new("cmake")
        .arg("--build")
        .arg(&build_dir)
        .arg("--parallel")
        .status()
        .expect("failed to run cmake build for sloth dialect");
    assert!(status.success(), "cmake build failed");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=sloth_dialect");
    // C++ runtime for the dialect sidecar (MLIR C++ is exception-free but
    // still needs libstdc++/libc++).
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
