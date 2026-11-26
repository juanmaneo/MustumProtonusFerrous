//! The coverage gate: every file must be at 100% regions, lines and functions.
//!
//! cargo-llvm-cov can only gate regions on the workspace total (there is no
//! per-file regions flag) and its per-file lines flag compares with a strict
//! `>` that rejects exactly 100%. So the gate is owned here instead: run the
//! tests, export the per-file summary and require `covered == count` for
//! every metric of every file. Every Rust source file must also appear in that
//! summary: cargo-llvm-cov leaves out files whose names match its ignore pattern
//! (such as `*tests.rs`) and files with no code, so the latter are listed in
//! `NO_CODE_FILES` and any other missing file fails the gate.

use crate::json::{self, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};

/// Metrics that must be fully covered in every file.
const METRICS: [&str; 3] = ["regions", "lines", "functions"];

/// Source files that hold only constants, type definitions, `mod` declarations
/// or re-exports, which llvm-cov never lists because they contain no code.
const NO_CODE_FILES: [&str; 138] = [
    "crates/mpf-amsi/src/lib.rs",
    "crates/mpf-apphelp/src/lib.rs",
    "crates/mpf-appxdeploymentclient/src/lib.rs",
    "crates/mpf-atlthunk/src/lib.rs",
    "crates/mpf-atmlib/src/lib.rs",
    "crates/mpf-authz/src/lib.rs",
    "crates/mpf-avrt/src/lib.rs",
    "crates/mpf-bcryptprimitives/src/lib.rs",
    "crates/mpf-capabilityaccessmanagerclient/src/lib.rs",
    "crates/mpf-cdosys/src/lib.rs",
    "crates/mpf-clusapi/src/lib.rs",
    "crates/mpf-cng_sys/src/lib.rs",
    "crates/mpf-cryptext/src/lib.rs",
    "crates/mpf-cryptxml/src/lib.rs",
    "crates/mpf-ctapi32/src/lib.rs",
    "crates/mpf-dataexchange/src/lib.rs",
    "crates/mpf-dbgeng/src/lib.rs",
    "crates/mpf-dciman32/src/lib.rs",
    "crates/mpf-dcomp/src/lib.rs",
    "crates/mpf-dhcpcsvc/src/lib.rs",
    "crates/mpf-dhtmled_ocx/src/lib.rs",
    "crates/mpf-difxapi/src/lib.rs",
    "crates/mpf-directmanipulation/src/lib.rs",
    "crates/mpf-dmusic32/src/lib.rs",
    "crates/mpf-dpnhpast/src/lib.rs",
    "crates/mpf-dpnhupnp/src/lib.rs",
    "crates/mpf-dsrole/src/lib.rs",
    "crates/mpf-dsuiext/src/lib.rs",
    "crates/mpf-dx8vb/src/lib.rs",
    "crates/mpf-dxtrans/src/lib.rs",
    "crates/mpf-explorerframe/src/lib.rs",
    "crates/mpf-fltlib/src/lib.rs",
    "crates/mpf-fltmgr_sys/src/lib.rs",
    "crates/mpf-fntcache/src/lib.rs",
    "crates/mpf-fontsub/src/lib.rs",
    "crates/mpf-fwpuclnt/src/lib.rs",
    "crates/mpf-gamingtcui/src/lib.rs",
    "crates/mpf-geolocation/src/lib.rs",
    "crates/mpf-graphicscapture/src/lib.rs",
    "crates/mpf-hal/src/lib.rs",
    "crates/mpf-hhctrl_ocx/src/lib.rs",
    "crates/mpf-hrtfapo/src/lib.rs",
    "crates/mpf-hvsimanagementapi/src/lib.rs",
    "crates/mpf-iccvid/src/oracle.rs",
    "crates/mpf-icmui/src/lib.rs",
    "crates/mpf-inkobj/src/lib.rs",
    "crates/mpf-inseng/src/lib.rs",
    "crates/mpf-ksecdd_sys/src/lib.rs",
    "crates/mpf-ktmw32/src/lib.rs",
    "crates/mpf-l3codecx_ax/src/lib.rs",
    "crates/mpf-loadperf/src/lib.rs",
    "crates/mpf-magnification/src/lib.rs",
    "crates/mpf-mfh264enc/src/lib.rs",
    "crates/mpf-mfmediaengine/src/lib.rs",
    "crates/mpf-mfsrcsnk/src/lib.rs",
    "crates/mpf-mgmtapi/src/lib.rs",
    "crates/mpf-mmcndmgr/src/lib.rs",
    "crates/mpf-mountmgr_sys/src/lib.rs",
    "crates/mpf-mprapi/src/lib.rs",
    "crates/mpf-msauddecmft/src/lib.rs",
    "crates/mpf-msctfmonitor/src/lib.rs",
    "crates/mpf-msdaps/src/lib.rs",
    "crates/mpf-msdrm/src/lib.rs",
    "crates/mpf-msftedit/src/lib.rs",
    "crates/mpf-msident/src/lib.rs",
    "crates/mpf-msimg32/src/lib.rs",
    "crates/mpf-msimtf/src/lib.rs",
    "crates/mpf-msisys_ocx/src/lib.rs",
    "crates/mpf-msnet32/src/lib.rs",
    "crates/mpf-msports/src/lib.rs",
    "crates/mpf-mssign32/src/lib.rs",
    "crates/mpf-msttsengine/src/lib.rs",
    "crates/mpf-msvcm80/src/lib.rs",
    "crates/mpf-msvcp140_atomic_wait/src/lib.rs",
    "crates/mpf-msvcp140_codecvt_ids/src/lib.rs",
    "crates/mpf-msvcrt20/src/lib.rs",
    "crates/mpf-msvdsp/src/lib.rs",
    "crates/mpf-msvfw32/src/lib.rs",
    "crates/mpf-mswsock/src/lib.rs",
    "crates/mpf-mtxdm/src/lib.rs",
    "crates/mpf-nddeapi/src/lib.rs",
    "crates/mpf-ndfapi/src/lib.rs",
    "crates/mpf-netcfgx/src/lib.rs",
    "crates/mpf-netutils/src/lib.rs",
    "crates/mpf-newdev/src/lib.rs",
    "crates/mpf-npmshtml/src/lib.rs",
    "crates/mpf-nsiproxy_sys/src/lib.rs",
    "crates/mpf-ntdll/src/lib.rs",
    "crates/mpf-nsi/src/oracle_data.rs",
    "crates/mpf-objsel/src/lib.rs",
    "crates/mpf-olecli32/src/lib.rs",
    "crates/mpf-oledb32/src/lib.rs",
    "crates/mpf-olepro32/src/lib.rs",
    "crates/mpf-olesvr32/src/lib.rs",
    "crates/mpf-packager/src/lib.rs",
    "crates/mpf-pidgen/src/lib.rs",
    "crates/mpf-printui/src/lib.rs",
    "crates/mpf-pstorec/src/lib.rs",
    "crates/mpf-qdvd/src/lib.rs",
    "crates/mpf-qedit/src/lib.rs",
    "crates/mpf-query/src/lib.rs",
    "crates/mpf-rasdlg/src/lib.rs",
    "crates/mpf-riched32/src/lib.rs",
    "crates/mpf-rstrtmgr/src/lib.rs",
    "crates/mpf-rtscom/src/lib.rs",
    "crates/mpf-rtutils/src/lib.rs",
    "crates/mpf-scardsvr/src/lib.rs",
    "crates/mpf-scrobj/src/lib.rs",
    "crates/mpf-scsiport_sys/src/lib.rs",
    "crates/mpf-sensapi/src/lib.rs",
    "crates/mpf-sfc_os/src/lib.rs",
    "crates/mpf-shdocvw/src/lib.rs",
    "crates/mpf-sppc/src/lib.rs",
    "crates/mpf-srvsvc/src/lib.rs",
    "crates/mpf-sti/src/lib.rs",
    "crates/mpf-tapi32/src/lib.rs",
    "crates/mpf-tbs/src/lib.rs",
    "crates/mpf-tdh/src/lib.rs",
    "crates/mpf-tdi_sys/src/lib.rs",
    "crates/mpf-threadpoolwinrt/src/lib.rs",
    "crates/mpf-thumbcache/src/guid.rs",
    "crates/mpf-thumbcache/src/lib.rs",
    "crates/mpf-tiptsf/src/lib.rs",
    "crates/mpf-traffic/src/lib.rs",
    "crates/mpf-twaindsm/src/data.rs",
    "crates/mpf-twaindsm/src/lib.rs",
    "crates/mpf-uianimation/src/lib.rs",
    "crates/mpf-uiribbon/src/lib.rs",
    "crates/mpf-updspapi/src/lib.rs",
    "crates/mpf-url/src/lib.rs",
    "crates/mpf-usp10/src/lib.rs",
    "crates/mpf-vcruntime140/src/lib.rs",
    "crates/mpf-vdmdbg/src/lib.rs",
    "crates/mpf-virtdisk/src/lib.rs",
    "crates/mpf-vssapi/src/lib.rs",
    "crates/mpf-w32skrnl/src/lib.rs",
    "crates/mpf-wbemdisp/src/lib.rs",
    "crates/mpf-winexinput_sys/src/lib.rs",
];

/// Runs the workspace tests under coverage and prints the usual table.
const TEST_ARGS: [&str; 3] = ["llvm-cov", "--workspace", "--no-cfg-coverage"];

/// Exports the per-file summary of the run above as JSON on stdout.
const REPORT_ARGS: [&str; 4] = ["llvm-cov", "report", "--json", "--summary-only"];

/// Runs `program`, returning its stdout when `capture` is set (otherwise the
/// child shares ours and the result is empty). Failing to start and a
/// non-zero exit are both errors.
fn run(program: &str, args: &[&str], capture: bool) -> Result<String, String> {
    let stdout = if capture {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };
    Command::new(program)
        .args(args)
        .stdout(stdout)
        .spawn()
        .and_then(Child::wait_with_output)
        .map_err(|error| format!("failed to run `{program}`: {error}"))
        .and_then(|output| {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).into_owned())
            } else {
                Err(format!(
                    "`{program} {}` exited with {}",
                    args.join(" "),
                    output.status
                ))
            }
        })
}

/// Runs the tests, then returns the JSON summary of that run.
fn coverage_report(cargo: &str) -> Result<String, String> {
    run(cargo, &TEST_ARGS, false).and_then(|_| run(cargo, &REPORT_ARGS, true))
}

/// The value reached by following `keys` from `value`.
fn path<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    keys.iter().try_fold(value, |value, key| value.get(key))
}

fn count_at(file: &Value, metric: &str, field: &str) -> Option<f64> {
    path(file, &["summary", metric, field]).and_then(Value::as_f64)
}

/// Describes how `file` falls short on `metric`, or `None` if it is fully
/// covered. A metric the report does not state counts as a shortfall, so a
/// change in the report format cannot make the gate pass silently.
fn shortfall(file: &Value, metric: &str) -> Option<String> {
    let name = file
        .get("filename")
        .and_then(Value::as_str)
        .unwrap_or("<unnamed file>");
    match (
        count_at(file, metric, "covered"),
        count_at(file, metric, "count"),
    ) {
        (Some(covered), Some(count)) if covered == count => None,
        (Some(covered), Some(count)) => Some(format!("{name}: {metric} {covered}/{count}")),
        _ => Some(format!("{name}: {metric} is missing from the report")),
    }
}

/// The Rust sources of every crate and of xtask: the files the gate must measure.
fn source_files(root: &Path) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    let crate_src_dirs = fs::read_dir(root.join("crates"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path().join("src"));
    for dir in crate_src_dirs.chain([root.join("xtask").join("src")]) {
        collect_rust_files(&dir, &mut sources);
    }
    sources
}

fn collect_rust_files(dir: &Path, sources: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, sources);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            sources.push(path);
        }
    }
}

fn is_no_code(source: &Path) -> bool {
    NO_CODE_FILES.iter().any(|file| source.ends_with(file))
}

/// Checks a cargo-llvm-cov JSON summary against the workspace `sources`:
/// `Ok` with a one-line summary if every file is complete and every source is
/// listed, otherwise `Err` naming each file, metric or source that is not. A
/// report without any file is an error, so the gate cannot pass by measuring
/// nothing.
pub fn verify(report: &str, sources: &[PathBuf]) -> Result<String, String> {
    let report = json::parse(report)?;
    let files: Vec<&Value> = report
        .get("data")
        .and_then(Value::as_array)
        .unwrap_or_default()
        .iter()
        .flat_map(|export| {
            export
                .get("files")
                .and_then(Value::as_array)
                .unwrap_or_default()
        })
        .collect();
    let reported: Vec<&str> = files
        .iter()
        .filter_map(|file| file.get("filename").and_then(Value::as_str))
        .collect();
    let unmeasured = sources
        .iter()
        .filter(|source| {
            !is_no_code(source)
                && !reported
                    .iter()
                    .any(|name| Path::new(name) == source.as_path())
        })
        .map(|source| format!("{}: not in the coverage report", source.display()));
    let problems: Vec<String> = files
        .iter()
        .flat_map(|file| {
            METRICS
                .iter()
                .filter_map(move |metric| shortfall(file, metric))
        })
        .chain(unmeasured)
        .collect();

    if files.is_empty() {
        Err("the coverage report lists no files".to_string())
    } else if problems.is_empty() {
        Ok(format!(
            "coverage gate passed: {} files, each at 100% of {}",
            files.len(),
            METRICS.join(", ")
        ))
    } else {
        Err(format!(
            "coverage is below 100% in {} place(s):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

fn conclude(outcome: Result<String, String>) -> ExitCode {
    match outcome {
        Ok(summary) => {
            println!("{summary}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// The `coverage` command, with the cargo executable injectable for tests.
pub fn run_coverage_with(cargo: &str) -> ExitCode {
    let sources = source_files(&crate::project_root());
    conclude(coverage_report(cargo).and_then(|report| verify(&report, &sources)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(count, covered)` of one metric, in the shape llvm-cov exports.
    type Counts = (u32, u32);

    const FULL: Counts = (10, 10);

    fn metric((count, covered): Counts) -> String {
        format!(r#"{{"count":{count},"covered":{covered},"percent":0.0}}"#)
    }

    fn file(name: &str, regions: Counts, lines: Counts, functions: Counts) -> String {
        format!(
            r#"{{"filename":"{name}","summary":{{"regions":{},"lines":{},"functions":{}}}}}"#,
            metric(regions),
            metric(lines),
            metric(functions)
        )
    }

    fn complete(name: &str) -> String {
        file(name, FULL, FULL, FULL)
    }

    fn report(files: &[String]) -> String {
        format!(r#"{{"data":[{{"files":[{}]}}]}}"#, files.join(","))
    }

    /// Two files and the trailer of a real `cargo llvm-cov report --json
    /// --summary-only` (0.9.1, export format 3.1.0), to pin the layout the
    /// helpers above only imitate.
    const REAL_SAMPLE: &str = r#"{"data":[{"files":[{"filename":"/home/neo/dev/mustum/crates/mpf-cli/src/main.rs","summary":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":28,"covered":28,"percent":100.0},"instantiations":{"count":34,"covered":34,"percent":100.0},"lines":{"count":342,"covered":342,"percent":100.0},"regions":{"count":477,"covered":477,"notcovered":0,"percent":100.0}}},{"filename":"/home/neo/dev/mustum/crates/mpf-kernel32/src/lib.rs","summary":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0.0},"functions":{"count":9,"covered":9,"percent":100.0},"instantiations":{"count":13,"covered":9,"percent":69.23076923076923},"lines":{"count":46,"covered":46,"percent":100.0},"regions":{"count":78,"covered":78,"notcovered":0,"percent":100.0}}}],"totals":{"branches":{"count":0,"covered":0,"notcovered":0,"percent":0},"functions":{"count":37,"covered":37,"percent":100},"instantiations":{"count":47,"covered":43,"percent":91.48936170212765},"lines":{"count":388,"covered":388,"percent":100},"mcdc":{"count":0,"covered":0,"notcovered":0,"percent":0},"regions":{"count":555,"covered":555,"notcovered":0,"percent":100}}}],"type":"llvm.coverage.json.export","version":"3.1.0","cargo_llvm_cov":{"version":"0.9.1","manifest_path":"/home/neo/dev/mustum/Cargo.toml"}}"#;

    fn failure(report: &str) -> String {
        verify(report, &[]).unwrap_err()
    }

    #[test]
    fn accepts_a_report_where_every_file_is_complete() {
        let summary = verify(&report(&[complete("a.rs"), complete("b.rs")]), &[]).unwrap();
        assert_eq!(
            summary,
            "coverage gate passed: 2 files, each at 100% of regions, lines, functions"
        );
    }

    #[test]
    fn accepts_the_real_report_format() {
        assert!(verify(REAL_SAMPLE, &[]).unwrap().contains("2 files"));
    }

    #[test]
    fn rejects_the_real_report_format_when_a_metric_slips() {
        for (metric, from, to) in [
            (
                "regions",
                r#""count":78,"covered":78"#,
                r#""count":78,"covered":77"#,
            ),
            (
                "lines",
                r#""count":342,"covered":342"#,
                r#""count":342,"covered":341"#,
            ),
            (
                "functions",
                r#""count":9,"covered":9,"#,
                r#""count":9,"covered":8,"#,
            ),
        ] {
            let tampered = REAL_SAMPLE.replacen(from, to, 1);
            assert_ne!(tampered, REAL_SAMPLE, "{metric}: sample did not change");
            let message = failure(&tampered);
            assert!(message.contains(metric), "{metric}: {message}");
            assert!(message.contains("in 1 place(s)"), "{metric}: {message}");
        }
    }

    #[test]
    fn a_regions_shortfall_names_the_file_and_only_regions() {
        let message = failure(&report(&[
            complete("ok.rs"),
            file("a.rs", (4, 3), FULL, FULL),
        ]));
        assert_eq!(
            message,
            "coverage is below 100% in 1 place(s):\n  a.rs: regions 3/4"
        );
    }

    #[test]
    fn a_lines_shortfall_names_the_file_and_only_lines() {
        let message = failure(&report(&[file("a.rs", FULL, (7, 6), FULL)]));
        assert_eq!(
            message,
            "coverage is below 100% in 1 place(s):\n  a.rs: lines 6/7"
        );
    }

    #[test]
    fn a_functions_shortfall_names_the_file_and_only_functions() {
        let message = failure(&report(&[file("a.rs", FULL, FULL, (2, 1))]));
        assert_eq!(
            message,
            "coverage is below 100% in 1 place(s):\n  a.rs: functions 1/2"
        );
    }

    #[test]
    fn a_file_is_judged_on_its_own_not_on_the_total() {
        // Across the two files 19 of 20 regions are covered, yet the gate
        // must still name the file that is short.
        let message = failure(&report(&[
            file("a.rs", (10, 10), FULL, FULL),
            file("b.rs", (10, 9), FULL, FULL),
        ]));
        assert!(message.ends_with("b.rs: regions 9/10"), "{message}");
        assert!(!message.contains("a.rs"), "{message}");
    }

    #[test]
    fn every_shortfall_of_every_file_is_listed() {
        let message = failure(&report(&[
            file("a.rs", (4, 3), (7, 6), FULL),
            complete("ok.rs"),
            file("b.rs", FULL, (5, 0), (3, 2)),
        ]));
        assert_eq!(
            message,
            "coverage is below 100% in 4 place(s):\n  \
             a.rs: regions 3/4\n  a.rs: lines 6/7\n  \
             b.rs: lines 0/5\n  b.rs: functions 2/3"
        );
    }

    #[test]
    fn files_of_every_data_entry_are_checked() {
        let report = format!(
            r#"{{"data":[{{"files":[{}]}},{{"files":[{}]}}]}}"#,
            complete("a.rs"),
            file("b.rs", (2, 1), FULL, FULL)
        );
        assert!(failure(&report).ends_with("b.rs: regions 1/2"));
    }

    #[test]
    fn a_file_without_a_name_is_still_reported() {
        let report = r#"{"data":[{"files":[{"summary":{
            "regions":{"count":2,"covered":1},
            "lines":{"count":1,"covered":1},
            "functions":{"count":1,"covered":1}}}]}]}"#;
        assert!(failure(report).ends_with("<unnamed file>: regions 1/2"));
    }

    #[test]
    fn a_metric_the_report_does_not_state_is_a_shortfall() {
        const OK: &str = r#"{"count":1,"covered":1}"#;
        for (name, regions) in [
            ("absent", String::new()),
            ("no covered", r#""regions":{"count":1},"#.to_string()),
            ("no count", r#""regions":{"covered":1},"#.to_string()),
            (
                "not a number",
                r#""regions":{"count":"1","covered":1},"#.to_string(),
            ),
        ] {
            let report = format!(
                r#"{{"data":[{{"files":[{{"filename":"a.rs","summary":{{{regions}"lines":{OK},"functions":{OK}}}}}]}}]}}"#
            );
            assert_eq!(
                failure(&report),
                "coverage is below 100% in 1 place(s):\n  \
                 a.rs: regions is missing from the report",
                "{name}"
            );
        }
    }

    #[test]
    fn a_file_without_any_summary_misses_every_metric() {
        let message = failure(r#"{"data":[{"files":[{"filename":"a.rs"}]}]}"#);
        assert_eq!(
            message,
            "coverage is below 100% in 3 place(s):\n  \
             a.rs: regions is missing from the report\n  \
             a.rs: lines is missing from the report\n  \
             a.rs: functions is missing from the report"
        );
    }

    #[test]
    fn a_report_without_files_is_an_error_not_a_pass() {
        for (name, report) in [
            ("empty object", r#"{}"#),
            ("no entries", r#"{"data":[]}"#),
            ("data not an array", r#"{"data":{}}"#),
            ("entry without files", r#"{"data":[{}]}"#),
            ("files not an array", r#"{"data":[{"files":5}]}"#),
            ("no files", r#"{"data":[{"files":[]}]}"#),
            ("not an object", r#"[]"#),
        ] {
            assert_eq!(
                failure(report),
                "the coverage report lists no files",
                "{name}"
            );
        }
    }

    #[test]
    fn unparsable_reports_are_errors() {
        assert!(failure("").starts_with("invalid JSON at offset 0"));
        assert!(failure(r#"{"data":"#).starts_with("invalid JSON"));
    }

    #[test]
    fn conclude_maps_outcomes_to_exit_codes() {
        assert_eq!(conclude(Ok("fine".into())), ExitCode::SUCCESS);
        assert_eq!(conclude(Err("bad".into())), ExitCode::FAILURE);
    }

    #[test]
    fn run_returns_stdout_only_when_capturing() {
        assert_eq!(run("echo", &["hello"], true), Ok("hello\n".to_string()));
        assert_eq!(run("true", &[], false), Ok(String::new()));
    }

    #[test]
    fn run_reports_a_failing_program() {
        for capture in [true, false] {
            let message = run("false", &["--flag", "x"], capture).unwrap_err();
            assert!(
                message.starts_with("`false --flag x` exited with"),
                "{message}"
            );
        }
    }

    #[test]
    fn run_reports_a_program_that_cannot_start() {
        let message = run("mpf-no-such-program", &[], true).unwrap_err();
        assert!(
            message.starts_with("failed to run `mpf-no-such-program`:"),
            "{message}"
        );
    }

    #[test]
    fn the_command_stops_when_the_tests_fail() {
        assert_eq!(run_coverage_with("false"), ExitCode::FAILURE);
        // The error is the test step's, so the report step never ran.
        let message = coverage_report("false").unwrap_err();
        assert!(
            message.starts_with("`false llvm-cov --workspace --no-cfg-coverage` exited"),
            "{message}"
        );
    }

    #[test]
    fn the_command_fails_when_cargo_cannot_start() {
        assert_eq!(
            run_coverage_with("mpf-no-such-cargo-command"),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn a_source_missing_from_the_report_is_a_failure() {
        let sources = [PathBuf::from("a.rs"), PathBuf::from("b.rs")];
        assert_eq!(
            verify(&report(&[complete("a.rs")]), &sources).unwrap_err(),
            "coverage is below 100% in 1 place(s):\n  b.rs: not in the coverage report"
        );
    }

    #[test]
    fn a_source_with_no_code_need_not_be_reported() {
        let sources = [
            PathBuf::from("a.rs"),
            PathBuf::from("/repo/crates/mpf-nsi/src/oracle_data.rs"),
        ];
        assert!(verify(&report(&[complete("a.rs")]), &sources).is_ok());
    }

    #[test]
    fn the_no_code_list_names_real_files() {
        let root = crate::project_root();
        for file in NO_CODE_FILES {
            assert!(root.join(file).is_file(), "{file} is not a file");
        }
    }

    #[test]
    fn source_files_walks_nested_directories_and_keeps_rust_only() {
        let root = std::env::temp_dir().join(format!("mpf-xtask-sources-{}", std::process::id()));
        for dir in ["crates/mpf-a/src/nested", "xtask/src"] {
            fs::create_dir_all(root.join(dir)).unwrap();
        }
        for file in [
            "crates/mpf-a/src/lib.rs",
            "crates/mpf-a/src/nested/deep.rs",
            "crates/mpf-a/src/notes.txt",
            "xtask/src/main.rs",
        ] {
            fs::write(root.join(file), "").unwrap();
        }
        let mut found = source_files(&root);
        found.sort();
        fs::remove_dir_all(&root).unwrap();
        assert_eq!(
            found,
            [
                root.join("crates/mpf-a/src/lib.rs"),
                root.join("crates/mpf-a/src/nested/deep.rs"),
                root.join("xtask/src/main.rs"),
            ]
        );
    }

    #[test]
    fn the_command_fails_on_an_empty_report() {
        // `true` succeeds at both steps but exports nothing to verify.
        assert_eq!(coverage_report("true"), Ok(String::new()));
        assert_eq!(run_coverage_with("true"), ExitCode::FAILURE);
    }
}
