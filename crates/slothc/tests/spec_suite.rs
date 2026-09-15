//! P0 spec test set (design §8): source files with `// expect:` markers,
//! run through the `slothc run` CLI; compile-diag cases use `// diag:`
//! markers verified via `slothc check` (nonzero exit + message on stderr).
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

const EXPECT_MARK: &str = "// expect: ";
const DIAG_MARK: &str = "// diag: ";

fn spec_files() -> Vec<PathBuf> {
    let md = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = PathBuf::from(md).join("tests").join("spec");
    let mut out: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("spec dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "sl").unwrap_or(false))
        .collect();
    out.sort();
    out
}

/// (name, ordered (kind, payload) markers) per file
fn collect_markers(path: &PathBuf) -> (String, Vec<(u8, String)>) {
    let src = std::fs::read_to_string(path).expect("read spec");
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
        // order-insensitive multiset match: prints inside helper functions
        // run at call time, not at marker time
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
                "expected `{}` (marker {}), no matching output line\nstdout: {}",
                w,
                want.iter().position(|x| x == w).unwrap_or(0),
                stdout
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
        return Err(format!(
            "expected compile/surface failure, but check passed"
        ));
    }
    let err = String::from_utf8_lossy(&out.stderr).to_string();
    if !err.contains(want) {
        return Err(format!("diag missing `{}`\nstderr: {}", want, err));
    }
    Ok(())
}

#[test]
fn spec_case_count() {
    let files = spec_files();
    assert!(!files.is_empty(), "no spec files");
    let mut n = 0;
    for f in &files {
        let (_p, marks) = collect_markers(f);
        n += marks.len();
    }
    assert!(
        n >= 200,
        "spec suite has {} cases; design P0 wants >= 200",
        n
    );
}

#[test]
fn spec_markers_run() {
    let files = spec_files();
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
        panic!("spec failures:\n{}", fails.join("\n---\n"));
    }
}
