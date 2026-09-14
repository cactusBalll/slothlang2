fn main() {
    let args: Vec<String> = std::env::args().collect();
    // usage: slothc <check|ir|run> file.sl   |   slothc <file.mlir> (raw IR probe)
    let mut raw_probe = String::new();
    let mut mode = String::new();
    let mut path = String::new();
    let mut it = args[2..].iter();
    let mut want_raw = false;
    match args.get(1) {
        Some(a) => match a.as_str() {
            "check" | "ir" | "run" => {
                mode = a.clone();
                if let Some(p) = it.next() {
                    path = p.clone();
                }
            }
            _ => {
                want_raw = true;
                raw_probe = a.clone();
            }
        },
        None => {
            eprintln!("usage: slothc <check|ir|run> file.sl | <file.mlir>");
            std::process::exit(2);
        }
    }
    let _ = want_raw;
    if raw_probe.is_empty() {
        path = params_path(&path, args.len());
    }
    if !raw_probe.is_empty() {
        let src = std::fs::read_to_string(&raw_probe).expect("read");
        match sloth_codegen::parse_print_raw(&src, "probe.mlir") {
            Ok(p) => println!("{}", p),
            Err(e) => {
                eprintln!("err: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }
    if path.is_empty() {
        eprintln!("missing file");
        std::process::exit(2);
    }
    let _ = mode.as_str();
    let src = std::fs::read_to_string(&path).expect("read");
    match run_mode(&mode, &src) {
        Ok(out) => {
            if !out.is_empty() {
                println!("{}", out);
            }
        }
        Err(e) => {
            eprintln!("err: {}", e);
            std::process::exit(1);
        }
    }
}

fn params_path(p: &str, argc: usize) -> String {
    if argc > 2 && !p.is_empty() {
        return p.to_string();
    }
    p.to_string()
}

fn run_mode(mode: &str, src: &str) -> Result<String, String> {
    match mode {
        "check" => {
            let ir = sloth_codegen::irgen::compile_to_ir(src, "main")?;
            let _ = ir;
            Ok("check ok".to_string())
        }
        "ir" => sloth_codegen::irgen::compile_to_ir(src, "main"),
        "run" => {
            sloth_codegen::pass::run_src(src, "main")?;
            Ok(String::new())
        }
        _ => Err("unknown mode".to_string()),
    }
}
