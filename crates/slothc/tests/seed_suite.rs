//! Seed test set: source files with `// expect:` markers run through the
//! `slothc run` CLI; compile-diag cases use `// diag:` markers verified via
//! `slothc check` (nonzero exit + message on stderr).
//!
//! This mirrors `spec_suite.rs` but reads from `tests/seed`, the staging area
//! for newly authored conformance cases (see `test_workspace/BUGS.md`).
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

const EXPECT_MARK: &str = "// expect: ";
const DIAG_MARK: &str = "// diag: ";

fn seed_files() -> Vec<PathBuf> {
    let md = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = PathBuf::from(md).join("tests").join("seed");
    let mut out: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("seed dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "sl").unwrap_or(false))
        // Files carrying a `// bug:` marker are documented known-bug repros
        // (see test_workspace/BUGS*.md); they intentionally fail today and are
        // not asserted here.
        .filter(|p| {
            !std::fs::read_to_string(p)
                .map(|s| s.contains("// bug:"))
                .unwrap_or(false)
        })
        .collect();
    out.sort();
    out
}

/// (name, ordered (kind, payload) markers) per file
fn collect_markers(path: &PathBuf) -> (String, Vec<(u8, String)>) {
    let src = std::fs::read_to_string(path).expect("read seed");
    let mut marks: Vec<(u8, String)> = Vec::new();
    for line in src.lines() {
        if let Some(r) = line.find(EXPECT_MARK) {
            marks.push((0, line[r + EXPECT_MARK.len()..].trim().to_string()));
        } else if let Some(r) = line.find(DIAG_MARK) {
            marks.push((1, line[r + DIAG_MARK.len()..].trim().to_string()));
        }
    }
    (path.display().to_string(), marks)
}

fn run_expect_lines(path: &PathBuf, want: &[String]) -> Result<(), String> {
    let out = Command::new(env!("CARGO_BIN_EXE_slothc"))
        .arg("run")
        .arg(path)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "run exited nonzero\nstderr: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let mut consumed: HashSet<usize> = HashSet::new();
    for w in want {
        let mut found = false;
        for (i, l) in stdout.lines().enumerate() {
            if consumed.contains(&i) {
                continue;
            }
            if l.trim() == w {
                consumed.insert(i);
                found = true;
                break;
            }
        }
        if !found {
            return Err(format!(
                "expected `{}`, no matching output line\nstdout: {}",
                w, stdout
            ));
        }
    }
    Ok(())
}

fn run_diag(path: &PathBuf, want: &str) -> Result<(), String> {
    let out = Command::new(env!("CARGO_BIN_EXE_slothc"))
        .arg("check")
        .arg(path)
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        return Err("expected compile/surface failure, but check passed".to_string());
    }
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    if !err.contains(want) {
        return Err(format!("diag missing `{}`\nstderr: {}", want, err));
    }
    Ok(())
}

#[test]
fn seed_markers_run() {
    let files = seed_files();
    let mut fails: Vec<String> = Vec::new();
    for f in &files {
        let (p, marks) = collect_markers(f);
        let expects: Vec<String> = marks
            .iter()
            .filter(|(k, _)| *k == 0)
            .map(|(_, v)| v.clone())
            .collect();
        if !expects.is_empty() {
            if let Err(e) = run_expect_lines(f, &expects) {
                fails.push(format!("{}: {}", p, e));
            }
        }
        for (k, v) in &marks {
            if *k == 1 {
                if let Err(e) = run_diag(f, v) {
                    fails.push(format!("{}: {}", p, e));
                }
            }
        }
    }
    if !fails.is_empty() {
        panic!("seed failures:\n{}", fails.join("\n---\n"));
    }
}
