fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = dispatch(&args) {
        eprintln!("err: {}", e);
        std::process::exit(1);
    } else {
        print_out(&args);
    }
}

fn dispatch(args: &Vec<String>) -> Result<String, String> {
    if args.len() < 2 {
        return Err("usage: slothc <check|ir|run|build> file.sl [out]".to_string());
    }
    let mode = args[1].clone();
    match mode.as_str() {
        "check" | "ir" | "run" | "build" => {
            if args.len() < 3 {
                return Err("missing file".to_string());
            }
            let path = args[2].clone();
            let out_path = if mode == "build" && args.len() > 3 {
                args[3].clone()
            } else {
                String::new()
            };
            let src = std::fs::read_to_string(&path).map_err(|e| format!("read: {}", e))?;
            let imports = src.contains("import");
            let base = std::path::Path::new(&path)
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            match mode.as_str() {
                "check" => {
                    // Pass 1 only: no IR is produced for `slothc check`.
                    if imports {
                        sloth_codegen::sem::check_multimod(&src, &base)?;
                    } else {
                        sloth_codegen::sem::check_src(&src, "main")?;
                    }
                    Ok("check ok".to_string())
                }
                "ir" => {
                    if imports {
                        let mm = sloth_codegen::irgen::compile_multimod(&src, &base)?;
                        Ok(mm)
                    } else {
                        sloth_codegen::irgen::compile_to_ir(&src, "main")
                    }
                }
                "run" => {
                    if imports {
                        sloth_codegen::pass::run_src_multimod(&src, &base)?;
                    } else {
                        sloth_codegen::pass::run_src(&src, "main")?;
                    }
                    Ok(String::new())
                }
                "build" => {
                    let ir0 = if imports {
                        sloth_codegen::irgen::compile_multimod(&src, &base)?
                    } else {
                        sloth_codegen::irgen::compile_to_ir(&src, "main")?
                    };
                    build_mode_r(
                        &src,
                        &ir0,
                        if out_path.is_empty() {
                            "sloth_app"
                        } else {
                            &out_path
                        },
                    )
                }
                _ => unreachable!(),
            }
        }
        _ => {
            let src = std::fs::read_to_string(&args[1]).map_err(|e| format!("read: {}", e))?;
            let p = sloth_codegen::parse_print_raw(&src, "probe.mlir")?;
            Ok(p)
        }
    }
}

fn print_out(args: &Vec<String>) {
    if let Some(mode) = args.get(1) {
        if mode == "ir" || mode == "check" {
            if let Ok(out) = dispatch(args) {
                if !out.is_empty() {
                    println!("{}", out);
                }
            }
        }
    }
}

fn build_mode_r(_src: &str, ir: &str, out_path: &str) -> Result<String, String> {
    let wrapper = "  func.func @main() -> i32 attributes {llvm.emit_c_interface} {\n    func.call @sloth_main() : () -> ()\n    %z = arith.constant 0 : i32\n    return %z : i32\n  }\n";
    let closed = ir.strip_suffix("}\n").unwrap_or(&ir);
    let full = format!("{}\n{}\n}}\n", closed, wrapper);
    std::fs::write("/tmp/opencode/app.mlir", &full).map_err(|e| e.to_string())?;
    // single source of truth shared with the JIT (crate::pipeline)
    let mut opt_args: Vec<String> = vec![
        "/tmp/opencode/app.mlir".to_string(),
        "-o".to_string(),
        "/tmp/opencode/app-llvm.mlir".to_string(),
    ];
    for p in sloth_codegen::pipeline::pass_names() {
        opt_args.push(format!("--{}", p));
    }
    let st = std::process::Command::new("/usr/lib/llvm-21/bin/mlir-opt")
        .args(&opt_args)
        .status()
        .map_err(|e| e.to_string())?;
    if !st.success() {
        return Err("mlir-opt failed".to_string());
    }
    let st2 = std::process::Command::new("/usr/lib/llvm-21/bin/mlir-translate")
        .args([
            "--mlir-to-llvmir",
            "/tmp/opencode/app-llvm.mlir",
            "-o",
            "/tmp/opencode/app.ll",
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !st2.success() {
        return Err("mlir-translate failed".to_string());
    }
    let st3 = std::process::Command::new("clang")
        .args([
            "-O3",
            "/tmp/opencode/app.ll",
            "/home/undatus63/slothlang2/target/debug/libsloth_rt.so",
            "-lm",
            "-o",
            out_path,
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if !st3.success() {
        return Err("clang link failed".to_string());
    }
    Ok(format!("built: {}", out_path))
}
