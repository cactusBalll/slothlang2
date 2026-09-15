//! Drive examples/gc_smoke.rs as a child process: libgc requires init on
//! the main thread; running the suite inside libtest threads is unreliable
use std::path::PathBuf;
use std::process::Command;

fn gc_smoke_exe() -> PathBuf {
    let md = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    PathBuf::from(md)
        .join("..")
        .join("..")
        .join("target")
        .join("debug")
        .join("examples")
        .join("gc_smoke")
}

#[test]
fn gc_smoke_in_child() {
    let exe = gc_smoke_exe();
    assert!(exe.exists(), "examples/gc_smoke not built at {:?}", exe);
    let out = Command::new(&exe).output().expect("spawn gc_smoke");
    assert!(
        out.status.success(),
        "gc_smoke failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("rt smoke OK"),
        "missing success marker"
    );
}
