fn main() {
    let rc = match sloth_codegen::selftest() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("slothc smoke test failed: {}", e);
            1
        }
    };
    std::process::exit(rc);
}
