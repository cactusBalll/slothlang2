fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // raw-IR probe: parse the file, print back
        let src = std::fs::read_to_string(&args[1]).expect("read");
        match sloth_codegen::parse_print_raw(&src, "probe.mlir") {
            Ok(p) => println!("{}", p),
            Err(e) => {
                eprintln!("err: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }
    match sloth_codegen::selftest() {
        Ok(()) => println!("selftest ok"),
        Err(e) => {
            eprintln!("selftest err: {}", e);
            std::process::exit(1);
        }
    }
    match sloth_codegen::pass::sloth_main_hello() {
        Ok(()) => println!("hello ok"),
        Err(e) => {
            eprintln!("hello err: {}", e);
            std::process::exit(1);
        }
    }
}
