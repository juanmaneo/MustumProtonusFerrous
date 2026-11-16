//! Developer automation for MPF, following the `xtask` pattern:
//! `cargo xtask <command>` runs this binary through the alias in `.cargo/config.toml`.
//!
//! Commands:
//!   compare <wine-subdir> <crate>   diff a reference Wine/Proton directory
//!                                   against an MPF port crate's `src/`
//!   coverage                        run cargo-llvm-cov, failing below 100%

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const USAGE: &str = "usage: cargo xtask <command>
  compare <wine-subdir> <crate>   diff a Wine/Proton dir against a port crate
      e.g. cargo xtask compare dlls/kernel32 mpf-kernel32
  coverage                        cargo-llvm-cov; fails unless 100% per metric

env: MPF_REFS overrides the reference root (default: <project>/../repos)";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    execute(parse(&args), "cargo")
}

/// Parsed command line. Kept separate from execution so every arm can be
/// tested without spawning processes.
#[derive(Debug, PartialEq, Eq)]
enum Action {
    Compare { wine_dir: String, krate: String },
    Coverage,
    Help,
    Usage,
}

fn parse(args: &[String]) -> Action {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["compare", wine_dir, krate] => Action::Compare {
            wine_dir: wine_dir.to_string(),
            krate: krate.to_string(),
        },
        ["coverage"] => Action::Coverage,
        ["-h" | "--help"] | [] => Action::Help,
        _ => Action::Usage,
    }
}

fn execute(action: Action, cargo: &str) -> ExitCode {
    match action {
        Action::Compare { wine_dir, krate } => compare(&wine_dir, &krate),
        Action::Coverage => run_coverage_with(cargo),
        Action::Help => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Action::Usage => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Per-file line threshold. cargo-llvm-cov compares it with a strict `>`
/// (`all_files_above_coverage` in its `json.rs`, still so on `main`), so
/// `--fail-under-file-lines 100` rejects files that are at exactly 100%.
/// This is the largest threshold below 100 that still fails a file with a
/// single missed line: such a file scores `100 - 100/N`, so it would need
/// more than 10^8 lines to slip through.
const FILE_LINES_THRESHOLD: &str = "99.999999";

/// Arguments of the coverage gate: every metric at 100%, plus each file's
/// line coverage and zero uncovered lines/regions.
fn coverage_args() -> Vec<&'static str> {
    vec![
        "llvm-cov",
        "--workspace",
        "--no-cfg-coverage",
        "--fail-under-regions",
        "100",
        "--fail-under-lines",
        "100",
        "--fail-under-functions",
        "100",
        "--fail-under-file-lines",
        FILE_LINES_THRESHOLD,
        "--fail-uncovered-lines",
        "0",
        "--fail-uncovered-regions",
        "0",
    ]
}

fn run_coverage_with(cargo: &str) -> ExitCode {
    let status = Command::new(cargo).args(coverage_args()).status();
    match status {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("error: coverage gate failed with {status}");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("error: failed to run coverage tool: {error}");
            ExitCode::FAILURE
        }
    }
}

fn project_root() -> PathBuf {
    // xtask/ sits directly under the project root.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a parent directory")
        .to_path_buf()
}

fn compare(wine_dir: &str, krate: &str) -> ExitCode {
    let root = project_root();
    let refs = refs_root(&root, env::var_os("MPF_REFS"));
    compare_at(&root, &refs, wine_dir, krate, "diff")
}

fn refs_root(root: &Path, override_dir: Option<OsString>) -> PathBuf {
    override_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("..").join("repos"))
}

fn compare_at(root: &Path, refs: &Path, wine_dir: &str, krate: &str, diff: &str) -> ExitCode {
    let reference = refs.join("wine").join(wine_dir);
    let port = root.join("crates").join(krate).join("src");

    println!("== reference: {}", reference.display());
    println!("== port:      {}", port.display());

    // Like the old script (`diff ... || true`), differences are not a failure:
    // only a diff that could not run at all is reported as an error.
    match Command::new(diff)
        .arg("-ru")
        .arg(&reference)
        .arg(&port)
        .status()
    {
        Ok(status) if status.code().is_some_and(|c| c <= 1) => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("error: diff exited with {status}");
            ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("error: failed to run `diff`: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = env::temp_dir().join(format!("mpf-xtask-{id}"));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn parses_every_command_shape() {
        let args = |v: &[&str]| -> Vec<String> { v.iter().map(|s| s.to_string()).collect() };
        assert_eq!(parse(&args(&[])), Action::Help);
        assert_eq!(parse(&args(&["-h"])), Action::Help);
        assert_eq!(parse(&args(&["--help"])), Action::Help);
        assert_eq!(parse(&args(&["coverage"])), Action::Coverage);
        assert_eq!(
            parse(&args(&["compare", "dlls/x", "mpf-x"])),
            Action::Compare {
                wine_dir: "dlls/x".into(),
                krate: "mpf-x".into()
            }
        );
        assert_eq!(parse(&args(&["unknown"])), Action::Usage);
        assert_eq!(parse(&args(&["compare"])), Action::Usage);
        assert_eq!(parse(&args(&["coverage", "extra"])), Action::Usage);
    }

    #[test]
    fn executes_help_and_usage() {
        assert_eq!(execute(Action::Help, "cargo"), ExitCode::SUCCESS);
        assert_eq!(execute(Action::Usage, "cargo"), ExitCode::from(2));
    }

    #[test]
    fn coverage_gate_reports_success_failure_and_spawn_error() {
        assert_eq!(execute(Action::Coverage, "true"), ExitCode::SUCCESS);
        assert_eq!(execute(Action::Coverage, "false"), ExitCode::FAILURE);
        assert_eq!(
            execute(Action::Coverage, "mpf-no-such-cargo-command"),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn coverage_gate_checks_every_metric_and_every_file() {
        let args = coverage_args();
        for flag in [
            "--fail-under-regions",
            "--fail-under-lines",
            "--fail-under-functions",
            "--fail-under-file-lines",
            "--fail-uncovered-lines",
            "--fail-uncovered-regions",
        ] {
            assert!(args.contains(&flag), "missing {flag}");
        }
        // Every `--fail-*` flag is followed by its value.
        let value = |flag: &str| {
            let at = args.iter().position(|a| *a == flag).unwrap();
            args[at + 1]
        };
        assert_eq!(value("--fail-under-regions"), "100");
        assert_eq!(value("--fail-under-lines"), "100");
        assert_eq!(value("--fail-under-functions"), "100");
        assert_eq!(value("--fail-uncovered-lines"), "0");
        assert_eq!(value("--fail-uncovered-regions"), "0");
        assert_eq!(value("--fail-under-file-lines"), FILE_LINES_THRESHOLD);
    }

    #[test]
    fn file_threshold_is_just_below_100_to_work_around_strict_comparison() {
        let t: f64 = FILE_LINES_THRESHOLD.parse().unwrap();
        // Below 100, so a file at exactly 100% passes the strict `>` check...
        assert!(t < 100.0);
        // ...but above what a file with one missed line out of 10^6 scores.
        assert!(t > 100.0 - 100.0 / 1_000_000.0);
    }

    #[test]
    fn coverage_command_is_listed_in_help() {
        assert!(USAGE.contains("coverage"));
    }

    #[test]
    fn compare_accepts_equal_and_different_trees() {
        let tmp = TempDir::new();
        let root = tmp.0.join("project");
        let refs = tmp.0.join("refs");
        let port = root.join("crates/mpf-demo/src");
        let reference = refs.join("wine/dlls/demo");
        std::fs::create_dir_all(&port).unwrap();
        std::fs::create_dir_all(&reference).unwrap();
        std::fs::write(port.join("lib.rs"), "same").unwrap();
        std::fs::write(reference.join("lib.rs"), "same").unwrap();
        assert_eq!(
            compare_at(&root, &refs, "dlls/demo", "mpf-demo", "diff"),
            ExitCode::SUCCESS
        );

        std::fs::write(reference.join("lib.rs"), "different").unwrap();
        assert_eq!(
            compare_at(&root, &refs, "dlls/demo", "mpf-demo", "diff"),
            ExitCode::SUCCESS
        );
    }

    #[test]
    fn compare_reports_missing_diff_command() {
        let tmp = TempDir::new();
        assert_eq!(
            compare_at(
                &tmp.0,
                &tmp.0,
                "anything",
                "anything",
                "mpf-no-such-diff-command"
            ),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn main_entry_point_runs() {
        // The harness passes its own flags, so this exercises the usage path.
        let _ = main();
    }

    #[test]
    fn project_root_is_workspace_parent() {
        assert!(project_root().join("Cargo.toml").is_file());
    }

    #[test]
    fn refs_root_defaults_to_sibling_repos_or_env_override() {
        let root = Path::new("/project");
        assert_eq!(refs_root(root, None), PathBuf::from("/project/../repos"));
        assert_eq!(
            refs_root(root, Some("/elsewhere".into())),
            PathBuf::from("/elsewhere")
        );
    }

    #[test]
    fn compare_command_fails_for_missing_port_crate() {
        // diff of a nonexistent path exits 2, which compare treats as failure.
        assert_eq!(
            execute(
                Action::Compare {
                    wine_dir: "dlls/none".into(),
                    krate: "mpf-no-such-crate".into()
                },
                "cargo"
            ),
            ExitCode::FAILURE
        );
    }
}
