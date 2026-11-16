//! The `compare-all` command: the reference-vs-port diff of every mapped
//! component (`coverage/mapping.txt`), aggregated into one report at
//! `target/compare/report.txt`. Run once, or with `--watch SECONDS` in a loop
//! for the continuous comparison of the dev container.

use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::Duration;

use crate::dlls::{parse_mapping, Row};

/// What one mapped component produced.
#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
    /// The reference subdir, e.g. `dlls/kernel32`.
    pub wine_dir: String,
    /// The port crate.
    pub krate: String,
    /// Files in the reference directory.
    pub reference_files: usize,
    /// Files in the port crate's `src/`.
    pub port_files: usize,
    /// Files that differ between the two trees (or exist on one side only).
    pub differing: usize,
    /// Why the component was skipped, when it was (`None` = compared).
    pub note: Option<String>,
    /// The raw `diff -ru` output, when anything differed.
    pub diff: String,
}

/// Counts the files under `dir`, recursively. A directory entry that fails
/// to read is skipped, not fatal.
fn count_files(dir: &Path) -> Result<usize, String> {
    let mut total = 0;
    for path in fs::read_dir(dir)
        .map_err(|error| format!("cannot read {}: {error}", dir.display()))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
    {
        if path.is_dir() {
            total += count_files(&path)?;
        } else {
            total += 1;
        }
    }
    Ok(total)
}

/// Compares every mapping row that names a crate (`krate != "-"`), in
/// `wine_dir` order. Returns the outcomes and whether any `diff` run itself
/// failed (exit code above 1). Differences between the trees are not an
/// error — they are the report. A missing reference tree as a whole is.
pub fn compare_all(
    root: &Path,
    refs: &Path,
    mapping: &[Row],
    diff: &str,
) -> Result<(Vec<Outcome>, bool), String> {
    let wine = refs.join("wine");
    if !wine.is_dir() {
        return Err(format!(
            "reference wine tree not found at {} (set MPF_REFS)",
            wine.display()
        ));
    }
    let mut rows: Vec<&Row> = mapping.iter().filter(|row| row.krate != "-").collect();
    rows.sort_by(|a, b| a.wine_dir.cmp(&b.wine_dir));
    let mut outcomes = Vec::new();
    let mut had_error = false;
    for row in rows {
        let reference = wine.join(&row.wine_dir);
        let port = root.join("crates").join(&row.krate).join("src");
        let skip = |note: &str| Outcome {
            wine_dir: row.wine_dir.clone(),
            krate: row.krate.clone(),
            reference_files: 0,
            port_files: 0,
            differing: 0,
            note: Some(note.to_string()),
            diff: String::new(),
        };
        if !reference.is_dir() {
            outcomes.push(skip("missing in the reference tree"));
            continue;
        }
        if !port.is_dir() {
            outcomes.push(skip("no port crate src/"));
            continue;
        }
        let reference_files = count_files(&reference)?;
        let port_files = count_files(&port)?;
        let output = Command::new(diff)
            .arg("-ru")
            .arg(&reference)
            .arg(&port)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
            .map_err(|error| format!("failed to run `{diff}`: {error}"))?;
        let text = String::from_utf8_lossy(&output.stdout).into_owned();
        let differing = text
            .lines()
            .filter(|line| line.starts_with("diff ") || line.starts_with("Only in "))
            .count();
        let mut outcome = Outcome {
            wine_dir: row.wine_dir.clone(),
            krate: row.krate.clone(),
            reference_files,
            port_files,
            differing,
            note: None,
            diff: String::new(),
        };
        match output.status.code() {
            Some(0) | Some(1) => {
                if differing > 0 {
                    outcome.diff = text;
                }
                outcomes.push(outcome);
            }
            code => {
                had_error = true;
                outcome.note = Some(format!("`diff` exited with {code:?}"));
                outcome.diff = text;
                outcomes.push(outcome);
            }
        }
    }
    Ok((outcomes, had_error))
}

/// The report: one line per component, the raw diff under the components
/// that differ, and a totals line.
pub fn render_report(refs: &Path, outcomes: &[Outcome]) -> String {
    let mut out = format!(
        "== reference-vs-port comparison: {}\n",
        refs.join("wine").display()
    );
    let mut compared = 0;
    let mut skipped = 0;
    let mut reference_files = 0;
    let mut port_files = 0;
    let mut differing = 0;
    for outcome in outcomes {
        match &outcome.note {
            Some(note) => {
                skipped += 1;
                out.push_str(&format!(
                    "{:32} -> {:16} skipped: {note}\n",
                    outcome.wine_dir, outcome.krate
                ));
            }
            None => {
                compared += 1;
                reference_files += outcome.reference_files;
                port_files += outcome.port_files;
                differing += outcome.differing;
                out.push_str(&format!(
                    "{:32} -> {:16} {} ref files, {} port files, {} differing\n",
                    outcome.wine_dir,
                    outcome.krate,
                    outcome.reference_files,
                    outcome.port_files,
                    outcome.differing
                ));
            }
        }
        if !outcome.diff.is_empty() {
            out.push_str(&outcome.diff);
            out.push('\n');
        }
    }
    out.push_str(&format!(
        "== {compared} compared, {skipped} skipped, {reference_files} reference files, {port_files} port files, {differing} differing files\n"
    ));
    out
}

/// One full pass: read the mapping, compare everything, write and print the
/// report. `Err` only for setup problems (no mapping file, no reference
/// tree, `diff` missing); a failed `diff` run is a written report plus
/// `ExitCode::FAILURE`.
pub fn run_once(root: &Path, refs: &Path, diff: &str, report: &Path) -> Result<ExitCode, String> {
    let mapping_path = root.join("coverage").join("mapping.txt");
    let mapping = fs::read_to_string(&mapping_path)
        .map_err(|error| format!("cannot read {}: {error}", mapping_path.display()))
        .and_then(|text| {
            parse_mapping(&text).map_err(|error| format!("{}: {error}", mapping_path.display()))
        })?;
    let (outcomes, had_error) = compare_all(root, refs, &mapping, diff)?;
    let text = render_report(refs, &outcomes);
    if let Some(parent) = report.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    }
    fs::write(report, &text)
        .map_err(|error| format!("cannot write {}: {error}", report.display()))?;
    print!("{text}");
    println!("== report written to {}", report.display());
    Ok(if had_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// The continuous mode: re-run the comparison every `secs`, forever. Stops
/// on the first setup error or failed `diff` run — a misconfiguration should
/// not spin silently.
fn watch_loop(root: &Path, refs: &Path, secs: u64, diff: &str, report: &Path) -> ExitCode {
    loop {
        match run_once(root, refs, diff, report) {
            Ok(code) if code == ExitCode::SUCCESS => {}
            Ok(_) => return ExitCode::FAILURE,
            Err(error) => {
                eprintln!("error: {error}");
                return ExitCode::FAILURE;
            }
        }
        thread::sleep(Duration::from_secs(secs));
    }
}

/// The `compare-all` command.
pub fn run(root: &Path, refs: &Path, watch: Option<u64>, diff: &str) -> ExitCode {
    let report = root.join("target").join("compare").join("report.txt");
    match watch {
        None => match run_once(root, refs, diff, &report) {
            Ok(code) => code,
            Err(error) => {
                eprintln!("error: {error}");
                ExitCode::FAILURE
            }
        },
        Some(secs) => watch_loop(root, refs, secs, diff, &report),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dlls::{Priority, Status};
    use crate::testing::TempDir;
    use std::path::PathBuf;

    /// A temp project with `coverage/mapping.txt`, a reference tree at
    /// `refs/wine` and port crates under `crates/`.
    struct Fixture {
        tmp: TempDir,
        root: PathBuf,
        refs: PathBuf,
    }

    /// Writes `file` under `dir` with `content`, creating parent directories.
    fn write_under(dir: &Path, file: &str, content: &str) {
        let path = dir.join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn fixture(mapping: &str, reference: &[(&str, &[&str])], ports: &[(&str, &[&str])]) -> Fixture {
        let tmp = TempDir::new();
        let root = tmp.0.join("project");
        let refs = tmp.0.join("refs");
        fs::create_dir_all(root.join("coverage")).unwrap();
        fs::write(root.join("coverage/mapping.txt"), mapping).unwrap();
        for (wine_dir, files) in reference {
            let dir = refs.join("wine").join(wine_dir);
            fs::create_dir_all(&dir).unwrap();
            for file in *files {
                write_under(&dir, file, "reference\n");
            }
        }
        for (krate, files) in ports {
            let dir = root.join("crates").join(krate).join("src");
            fs::create_dir_all(&dir).unwrap();
            for file in *files {
                write_under(&dir, file, "port\n");
            }
        }
        Fixture { tmp, root, refs }
    }

    fn rows(text: &str) -> Vec<Row> {
        parse_mapping(text).unwrap()
    }

    #[test]
    fn count_files_walks_recursively_and_reports_errors() {
        let tmp = TempDir::new();
        let dir = tmp.0.join("tree");
        fs::create_dir_all(dir.join("sub/deep")).unwrap();
        fs::write(dir.join("a.rs"), "").unwrap();
        fs::write(dir.join("sub/b.rs"), "").unwrap();
        fs::write(dir.join("sub/deep/c.rs"), "").unwrap();
        assert_eq!(count_files(&dir), Ok(3));
        let error = count_files(&tmp.0.join("missing")).unwrap_err();
        assert!(error.starts_with("cannot read "), "{error}");
    }

    #[cfg(unix)]
    #[test]
    fn count_files_propagates_an_unreadable_subdirectory() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = TempDir::new();
        let dir = tmp.0.join("tree");
        let locked = dir.join("locked");
        fs::create_dir_all(&locked).unwrap();
        fs::write(locked.join("a.rs"), "").unwrap();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
        let error = count_files(&dir).unwrap_err();
        assert!(error.contains("locked"), "{error}");
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
    }

    #[test]
    fn compare_all_diffs_every_mapped_row_and_counts() {
        let f = fixture(
            "dlls/a mpf-a ported P1\ndlls/b mpf-b ported P1\ndlls/ghost mpf-ghost ported P1\ndlls/noport - todo P2\n",
            &[("dlls/a", &["lib.rs"]), ("dlls/b", &["lib.rs", "sub/mod.rs"])],
            &[("mpf-a", &["lib.rs"]), ("mpf-b", &["lib.rs"])],
        );
        let (outcomes, had_error) = compare_all(
            &f.root,
            &f.refs,
            &rows(&fs::read_to_string(f.root.join("coverage/mapping.txt")).unwrap()),
            "diff",
        )
        .unwrap();
        assert!(!had_error);
        assert_eq!(outcomes.len(), 3, "the `-` row is not compared");
        let names: Vec<&str> = outcomes.iter().map(|o| o.wine_dir.as_str()).collect();
        assert_eq!(names, ["dlls/a", "dlls/b", "dlls/ghost"]);
        // dlls/a: one differing file (the contents differ).
        assert_eq!(outcomes[0].reference_files, 1);
        assert_eq!(outcomes[0].port_files, 1);
        assert_eq!(outcomes[0].differing, 1);
        assert_eq!(outcomes[0].note, None);
        assert!(outcomes[0].diff.contains("diff -ru"));
        // dlls/b: the port is missing sub/mod.rs (`Only in`).
        assert_eq!(outcomes[1].reference_files, 2);
        assert_eq!(outcomes[1].port_files, 1);
        assert_eq!(
            outcomes[1].differing, 2,
            "lib.rs differs and sub/mod.rs is only in the reference"
        );
        // dlls/ghost: no reference dir.
        assert_eq!(
            outcomes[2].note.as_deref(),
            Some("missing in the reference tree")
        );
        assert_eq!(outcomes[2].differing, 0);
    }

    #[test]
    fn compare_all_skips_rows_without_a_port_src() {
        let f = fixture(
            "dlls/a mpf-missing ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[],
        );
        let (outcomes, had_error) = compare_all(
            &f.root,
            &f.refs,
            &rows("dlls/a mpf-missing ported P1\n"),
            "diff",
        )
        .unwrap();
        assert!(!had_error);
        assert_eq!(outcomes[0].note.as_deref(), Some("no port crate src/"));
    }

    #[test]
    fn compare_all_reports_identical_trees_as_zero_differing() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        // Make the trees identical.
        fs::write(f.root.join("crates/mpf-a/src/lib.rs"), "reference\n").unwrap();
        let (outcomes, had_error) =
            compare_all(&f.root, &f.refs, &rows("dlls/a mpf-a ported P1\n"), "diff").unwrap();
        assert!(!had_error);
        assert_eq!(outcomes[0].differing, 0);
        assert!(outcomes[0].diff.is_empty());
        assert_eq!(outcomes[0].note, None);
    }

    #[test]
    fn compare_all_fails_without_the_reference_tree() {
        let f = fixture("dlls/a mpf-a ported P1\n", &[], &[("mpf-a", &["lib.rs"])]);
        let error = compare_all(
            &f.root,
            &f.tmp.0.join("empty"),
            &rows("dlls/a mpf-a ported P1\n"),
            "diff",
        )
        .unwrap_err();
        assert!(error.contains("reference wine tree not found"), "{error}");
        assert!(error.contains("MPF_REFS"), "{error}");
    }

    #[cfg(unix)]
    #[test]
    fn compare_all_propagates_an_unreadable_reference_or_port_dir() {
        use std::os::unix::fs::PermissionsExt;
        // Unreadable reference subdir: counting the reference fails.
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs", "locked/x.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        let locked = f.refs.join("wine/dlls/a/locked");
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
        let error =
            compare_all(&f.root, &f.refs, &rows("dlls/a mpf-a ported P1\n"), "diff").unwrap_err();
        assert!(error.contains("locked"), "{error}");
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();
        // Unreadable port subdir: counting the port fails.
        let locked = f.root.join("crates/mpf-a/src/locked");
        fs::create_dir_all(&locked).unwrap();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();
        let error =
            compare_all(&f.root, &f.refs, &rows("dlls/a mpf-a ported P1\n"), "diff").unwrap_err();
        assert!(error.contains("locked"), "{error}");
    }

    #[test]
    fn compare_all_marks_a_failing_diff_and_reports_it() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        // A `diff` that always exits 2 (trouble, not differences).
        let fake = f.tmp.0.join("fake-diff");
        fs::write(&fake, "#!/bin/sh\nexit 2\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        }
        let (outcomes, had_error) = compare_all(
            &f.root,
            &f.refs,
            &rows("dlls/a mpf-a ported P1\n"),
            fake.to_str().unwrap(),
        )
        .unwrap();
        assert!(had_error);
        assert!(outcomes[0]
            .note
            .as_deref()
            .unwrap()
            .contains("exited with Some(2)"));
        // A `diff` that cannot start is a hard error.
        let error = compare_all(
            &f.root,
            &f.refs,
            &rows("dlls/a mpf-a ported P1\n"),
            "mpf-no-such-diff",
        )
        .unwrap_err();
        assert!(
            error.starts_with("failed to run `mpf-no-such-diff`"),
            "{error}"
        );
    }

    #[test]
    fn render_report_lists_components_diffs_and_totals() {
        let outcomes = vec![
            Outcome {
                wine_dir: "dlls/a".into(),
                krate: "mpf-a".into(),
                reference_files: 3,
                port_files: 2,
                differing: 1,
                note: None,
                diff: "diff -ru ref/lib.rs port/lib.rs\n-old\n+new\n".into(),
            },
            Outcome {
                wine_dir: "dlls/ghost".into(),
                krate: "mpf-ghost".into(),
                reference_files: 0,
                port_files: 0,
                differing: 0,
                note: Some("missing in the reference tree".into()),
                diff: String::new(),
            },
        ];
        let tmp = TempDir::new();
        let refs = tmp.0.join("refs");
        let text = render_report(&refs, &outcomes);
        assert!(text.contains("dlls/a"), "{text}");
        assert!(
            text.contains("3 ref files, 2 port files, 1 differing"),
            "{text}"
        );
        assert!(text.contains("diff -ru ref/lib.rs port/lib.rs"), "{text}");
        assert!(text.contains("dlls/ghost"), "{text}");
        assert!(
            text.contains("skipped: missing in the reference tree"),
            "{text}"
        );
        assert!(
            text.trim_end().ends_with(
                "== 1 compared, 1 skipped, 3 reference files, 2 port files, 1 differing files"
            ),
            "{text}"
        );
    }

    #[test]
    fn run_once_writes_and_prints_the_report() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        let report = f.tmp.0.join("out/report.txt");
        let code = run_once(&f.root, &f.refs, "diff", &report).unwrap();
        assert_eq!(code, ExitCode::SUCCESS);
        let written = fs::read_to_string(&report).unwrap();
        assert!(written.contains("dlls/a"), "{written}");
        assert!(written.contains("1 differing files"), "{written}");
        assert!(written.contains("== 1 compared, 0 skipped"), "{written}");
    }

    #[test]
    fn run_once_propagates_setup_errors() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        // No reference tree.
        let report = f.tmp.0.join("out/report.txt");
        let error = run_once(&f.root, &f.tmp.0.join("empty"), "diff", &report).unwrap_err();
        assert!(error.contains("reference wine tree not found"), "{error}");
        // No mapping file.
        let bare = f.tmp.0.join("bare-project");
        fs::create_dir_all(&bare).unwrap();
        let error = run_once(&bare, &f.refs, "diff", &report).unwrap_err();
        assert!(error.contains("cannot read "), "{error}");
        assert!(error.contains("mapping.txt"), "{error}");
        // Malformed mapping.
        fs::create_dir_all(bare.join("coverage")).unwrap();
        fs::write(bare.join("coverage/mapping.txt"), "dlls/a\n").unwrap();
        let error = run_once(&bare, &f.refs, "diff", &report).unwrap_err();
        assert!(error.contains("mapping line 1"), "{error}");
    }

    #[test]
    fn run_once_reports_an_unwritable_report_path() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        // The report's parent path is a regular file: creating it fails.
        let blocker = f.tmp.0.join("blocker");
        fs::write(&blocker, "a file, not a directory").unwrap();
        let error = run_once(&f.root, &f.refs, "diff", &blocker.join("report.txt")).unwrap_err();
        assert!(error.contains("cannot create "), "{error}");
        // The report path is a directory: writing it fails.
        let dir_report = f.tmp.0.join("a-dir");
        fs::create_dir_all(&dir_report).unwrap();
        let error = run_once(&f.root, &f.refs, "diff", &dir_report).unwrap_err();
        assert!(error.contains("cannot write "), "{error}");
        // A report path without a parent (`/`) skips the create and fails on
        // the write itself.
        let error = run_once(&f.root, &f.refs, "diff", Path::new("/")).unwrap_err();
        assert!(error.contains("cannot write "), "{error}");
    }

    #[test]
    fn run_without_watch_is_a_single_pass() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        assert_eq!(run(&f.root, &f.refs, None, "diff"), ExitCode::SUCCESS);
        assert!(f.root.join("target/compare/report.txt").is_file());
        // Setup error: exit failure, no report.
        let bare_refs = f.tmp.0.join("empty");
        assert_eq!(run(&f.root, &bare_refs, None, "diff"), ExitCode::FAILURE);
    }

    #[test]
    fn watch_stops_immediately_on_a_setup_error() {
        let f = fixture("dlls/a mpf-a ported P1\n", &[], &[("mpf-a", &["lib.rs"])]);
        // No reference tree: the first pass errors and the loop must not spin.
        assert_eq!(
            run(&f.root, &f.tmp.0.join("empty"), Some(0), "diff"),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn watch_loops_until_the_setup_breaks() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        let root = f.root.clone();
        let refs = f.refs.clone();
        let handle = thread::spawn(move || run(&root, &refs, Some(0), "diff"));
        let report = f.root.join("target/compare/report.txt");
        // Wait for the complete report, not merely file creation: fs::write
        // makes the path visible before all report bytes have been written.
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        let text = loop {
            let text = fs::read_to_string(&report).unwrap_or_default();
            if text.contains("dlls/a") {
                break text;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "the watch loop never wrote a complete report; last contents: {text:?}"
            );
            thread::sleep(Duration::from_millis(10));
        };
        assert!(text.contains("dlls/a"), "{text}");
        // Dropping the fixture breaks the mapping read, so the loop stops.
        drop(f);
        let code = handle.join().unwrap();
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[test]
    fn watch_stops_on_a_failing_diff() {
        let f = fixture(
            "dlls/a mpf-a ported P1\n",
            &[("dlls/a", &["lib.rs"])],
            &[("mpf-a", &["lib.rs"])],
        );
        let fake = f.tmp.0.join("fake-diff");
        fs::write(&fake, "#!/bin/sh\nexit 2\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        }
        assert_eq!(
            run(&f.root, &f.refs, Some(0), fake.to_str().unwrap()),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn row_statuses_and_priorities_are_reachable_for_the_report() {
        // Keep the imports used: the report renders mapping data.
        let row = Row {
            wine_dir: "dlls/x".into(),
            krate: "mpf-x".into(),
            status: Status::Partial,
            priority: Priority::P1,
        };
        assert_eq!(row.status.as_str(), "partial");
        assert_eq!(row.priority.as_str(), "P1");
    }
}
