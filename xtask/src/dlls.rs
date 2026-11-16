//! The `dlls` command: the inventory of the reference Wine `dlls/` components
//! with their port status and Proton game-compatibility priority. The
//! classification comes from `coverage/mapping.txt` (see its header for the
//! format); components of the reference tree with no mapping row are `todo`
//! and unranked.

use std::fmt;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

/// Where a component stands in the port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Fully ported.
    Ported,
    /// Ported in part (a subset of the reference API).
    Partial,
    /// A crate is planned but not created yet.
    Planned,
    /// Stays a native companion under Proton (dxvk, vkd3d-proton, ...).
    Native,
    /// Not ported and not planned yet.
    Todo,
}

impl Status {
    /// Parses the `status` field of a mapping row.
    fn parse(text: &str) -> Result<Self, String> {
        match text {
            "ported" => Ok(Status::Ported),
            "partial" => Ok(Status::Partial),
            "planned" => Ok(Status::Planned),
            "native" => Ok(Status::Native),
            "todo" => Ok(Status::Todo),
            other => Err(format!(
                "unknown status `{other}` (ported | partial | planned | native | todo)"
            )),
        }
    }

    /// The `status` field of a mapping row.
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Ported => "ported",
            Status::Partial => "partial",
            Status::Planned => "planned",
            Status::Native => "native",
            Status::Todo => "todo",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The Proton game-compatibility priority: P1 is ported first. Declared in
/// this order so the derived `Ord` sorts P1 before P2 before P3 before
/// unranked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Loaded by virtually every game run under Proton.
    P1,
    /// Common.
    P2,
    /// Rare or specialized.
    P3,
    /// Not ranked yet.
    Unranked,
}

impl Priority {
    /// Parses the `priority` field of a mapping row (`-` means unranked).
    fn parse(text: &str) -> Result<Self, String> {
        match text {
            "P1" => Ok(Priority::P1),
            "P2" => Ok(Priority::P2),
            "P3" => Ok(Priority::P3),
            "-" => Ok(Priority::Unranked),
            other => Err(format!("unknown priority `{other}` (P1 | P2 | P3 | -)")),
        }
    }

    /// The `priority` field of a mapping row.
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::P1 => "P1",
            Priority::P2 => "P2",
            Priority::P3 => "P3",
            Priority::Unranked => "-",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One row of `coverage/mapping.txt`.
#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    /// Path under `repos/wine`, e.g. `dlls/kernel32`.
    pub wine_dir: String,
    /// The port crate under `crates/`, or `-` when there is none yet.
    pub krate: String,
    /// Port status.
    pub status: Status,
    /// Proton game-compatibility priority.
    pub priority: Priority,
}

/// Parses a whole `coverage/mapping.txt`: one `wine-subdir crate status
/// priority` row per line, `#` comments and blank lines ignored. Errors name
/// the 1-based line.
pub fn parse_mapping(text: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let at = |what: String| format!("mapping line {}: {what}", index + 1);
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [wine_dir, krate, status, priority] = match fields.as_slice() {
            [wine_dir, krate, status, priority] => [*wine_dir, *krate, *status, *priority],
            _ => {
                return Err(at(
                    "expected 4 fields: <wine-subdir> <crate> <status> <priority>".to_string(),
                ))
            }
        };
        let status = Status::parse(status).map_err(at)?;
        let priority = Priority::parse(priority).map_err(at)?;
        rows.push(Row {
            wine_dir: wine_dir.to_string(),
            krate: krate.to_string(),
            status,
            priority,
        });
    }
    Ok(rows)
}

/// One component of the reference `dlls/` tree, classified by the mapping.
#[derive(Debug, PartialEq, Eq)]
pub struct Component {
    /// The component, e.g. `dlls/kernel32`.
    pub name: String,
    /// The port crate, or `-`.
    pub krate: String,
    /// Port status.
    pub status: Status,
    /// Priority.
    pub priority: Priority,
    /// Whether the component exists in the reference tree (a mapping row can
    /// name a directory the pinned reference no longer has).
    pub in_reference: bool,
}

/// The inventory: every directory of `dlls_dir`, classified by `mapping`,
/// plus mapping rows under `dlls/` with no matching directory (marked
/// `in_reference: false`). Sorted by priority, then name.
pub fn inventory(dlls_dir: &Path, mapping: &[Row]) -> Result<Vec<Component>, String> {
    // A directory entry that fails to read is skipped, not fatal: the
    // inventory is a best-effort listing of the reference tree.
    let mut names: Vec<String> = fs::read_dir(dlls_dir)
        .map_err(|error| format!("cannot read {}: {error}", dlls_dir.display()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort_unstable();
    let mut components: Vec<Component> = names
        .into_iter()
        .map(|name| {
            let wine_dir = format!("dlls/{name}");
            match mapping.iter().find(|row| row.wine_dir == wine_dir) {
                Some(row) => Component {
                    name: wine_dir,
                    krate: row.krate.clone(),
                    status: row.status,
                    priority: row.priority,
                    in_reference: true,
                },
                None => Component {
                    name: wine_dir,
                    krate: "-".to_string(),
                    status: Status::Todo,
                    priority: Priority::Unranked,
                    in_reference: true,
                },
            }
        })
        .collect();
    for row in mapping
        .iter()
        .filter(|row| row.wine_dir.starts_with("dlls/"))
    {
        if !components.iter().any(|c| c.name == row.wine_dir) {
            components.push(Component {
                name: row.wine_dir.clone(),
                krate: row.krate.clone(),
                status: row.status,
                priority: row.priority,
                in_reference: false,
            });
        }
    }
    components.sort_by(|a, b| (a.priority, &a.name).cmp(&(b.priority, &b.name)));
    Ok(components)
}

/// The inventory as a table plus a per-status summary line.
pub fn render(dlls_dir: &Path, components: &[Component]) -> String {
    let mut out = format!("== dlls inventory: {}\n", dlls_dir.display());
    for component in components {
        let note = if component.in_reference {
            String::new()
        } else {
            "  (not in the reference tree)".to_string()
        };
        out.push_str(&format!(
            "{:24} {:8} {:2} {}{}\n",
            component.name, component.status, component.priority, component.krate, note
        ));
    }
    let count = |status: Status| components.iter().filter(|c| c.status == status).count();
    out.push_str(&format!(
        "== {} components: {} ported, {} partial, {} planned, {} native, {} todo\n",
        components.len(),
        count(Status::Ported),
        count(Status::Partial),
        count(Status::Planned),
        count(Status::Native),
        count(Status::Todo),
    ));
    out
}

/// The `dlls` command: read `coverage/mapping.txt` and the reference
/// `dlls/` tree, print the inventory. Any setup problem is an error.
pub fn run(root: &Path, refs: &Path) -> ExitCode {
    let outcome = (|| -> Result<String, String> {
        let wine = refs.join("wine");
        if !wine.is_dir() {
            return Err(format!(
                "reference wine tree not found at {} (set MPF_REFS)",
                wine.display()
            ));
        }
        let dlls_dir = wine.join("dlls");
        let mapping_path = root.join("coverage").join("mapping.txt");
        let mapping = fs::read_to_string(&mapping_path)
            .map_err(|error| format!("cannot read {}: {error}", mapping_path.display()))
            .and_then(|text| {
                parse_mapping(&text).map_err(|error| format!("{}: {error}", mapping_path.display()))
            })?;
        let components = inventory(&dlls_dir, &mapping)?;
        Ok(render(&dlls_dir, &components))
    })();
    match outcome {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TempDir;

    #[test]
    fn statuses_parse_and_display() {
        for (text, status) in [
            ("ported", Status::Ported),
            ("partial", Status::Partial),
            ("planned", Status::Planned),
            ("native", Status::Native),
            ("todo", Status::Todo),
        ] {
            assert_eq!(Status::parse(text), Ok(status));
            assert_eq!(status.to_string(), text);
            assert_eq!(status.as_str(), text);
        }
        let error = Status::parse("half").unwrap_err();
        assert!(error.contains("unknown status `half`"), "{error}");
        assert!(error.contains("todo"), "{error}");
    }

    #[test]
    fn priorities_parse_display_and_sort() {
        for (text, priority) in [
            ("P1", Priority::P1),
            ("P2", Priority::P2),
            ("P3", Priority::P3),
            ("-", Priority::Unranked),
        ] {
            assert_eq!(Priority::parse(text), Ok(priority));
            assert_eq!(priority.to_string(), text);
        }
        assert!(Priority::parse("p1")
            .unwrap_err()
            .contains("unknown priority `p1`"));
        assert!(Priority::P1 < Priority::P2);
        assert!(Priority::P2 < Priority::P3);
        assert!(Priority::P3 < Priority::Unranked);
    }

    #[test]
    fn mapping_parses_rows_and_ignores_comments_and_blanks() {
        let text = "# comment\n\n  \ndlls/a mpf-a ported P1\n  loader mpf-loader ported P1  \n";
        let rows = parse_mapping(text).unwrap();
        assert_eq!(
            rows,
            vec![
                Row {
                    wine_dir: "dlls/a".into(),
                    krate: "mpf-a".into(),
                    status: Status::Ported,
                    priority: Priority::P1
                },
                Row {
                    wine_dir: "loader".into(),
                    krate: "mpf-loader".into(),
                    status: Status::Ported,
                    priority: Priority::P1
                },
            ]
        );
        assert_eq!(parse_mapping(""), Ok(vec![]));
        assert_eq!(parse_mapping("# only a comment\n"), Ok(vec![]));
    }

    #[test]
    fn mapping_errors_name_the_line() {
        let error = parse_mapping("dlls/a mpf-a ported P1\ndlls/b mpf-b\n").unwrap_err();
        assert_eq!(
            error,
            "mapping line 2: expected 4 fields: <wine-subdir> <crate> <status> <priority>"
        );
        let error = parse_mapping("dlls/a mpf-a weird P1").unwrap_err();
        assert!(
            error.starts_with("mapping line 1: unknown status `weird`"),
            "{error}"
        );
        let error = parse_mapping("dlls/a mpf-a ported P9").unwrap_err();
        assert!(
            error.starts_with("mapping line 1: unknown priority `P9`"),
            "{error}"
        );
    }

    /// A temp project: `refs/wine/dlls` with the given component dirs and
    /// `root/crates` left empty.
    fn fixture(components: &[&str]) -> (TempDir, std::path::PathBuf, std::path::PathBuf) {
        let tmp = TempDir::new();
        let root = tmp.0.join("project");
        let refs = tmp.0.join("refs");
        for name in components {
            fs::create_dir_all(refs.join("wine").join("dlls").join(name)).unwrap();
        }
        fs::create_dir_all(root.join("crates")).unwrap();
        (tmp, root, refs)
    }

    #[test]
    fn inventory_classifies_scanned_components_and_sorts_by_priority() {
        let (_tmp, _root, refs) = fixture(&["c", "a", "b"]);
        let mapping = parse_mapping(
            "dlls/b mpf-b partial P2\ndlls/a mpf-a ported P1\ndlls/ghost mpf-ghost planned P3\nloader mpf-loader ported P1\n",
        )
        .unwrap();
        let dlls_dir = refs.join("wine").join("dlls");
        let components = inventory(&dlls_dir, &mapping).unwrap();
        let names: Vec<&str> = components.iter().map(|c| c.name.as_str()).collect();
        // P1 first, then P2, then P3, then the unranked, each by name; the
        // `loader` row is not under dlls/ and does not join the inventory.
        assert_eq!(names, ["dlls/a", "dlls/b", "dlls/ghost", "dlls/c"]);
        assert_eq!(components[0].krate, "mpf-a");
        assert_eq!(components[0].status, Status::Ported);
        assert!(
            !components[2].in_reference,
            "the ghost row is not in the reference tree"
        );
        let unranked = &components[3];
        assert_eq!(unranked.krate, "-");
        assert_eq!(unranked.status, Status::Todo);
        assert_eq!(unranked.priority, Priority::Unranked);
        assert!(unranked.in_reference);
    }

    #[test]
    fn inventory_reports_an_unreadable_dlls_dir() {
        let tmp = TempDir::new();
        let missing = tmp.0.join("no-such-dir");
        let error = inventory(&missing, &[]).unwrap_err();
        assert!(error.starts_with("cannot read "), "{error}");
        assert!(error.contains("no-such-dir"), "{error}");
    }

    #[test]
    fn inventory_skips_files_and_keeps_only_directories() {
        let (_tmp, _root, refs) = fixture(&["a"]);
        let dlls_dir = refs.join("wine").join("dlls");
        fs::write(dlls_dir.join("README.txt"), "not a component").unwrap();
        let components = inventory(&dlls_dir, &[]).unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "dlls/a");
        // An empty reference dir yields an empty inventory, not an error.
        let empty = TempDir::new();
        let empty_dir = empty.0.join("dlls");
        fs::create_dir_all(&empty_dir).unwrap();
        assert_eq!(inventory(&empty_dir, &[]).unwrap(), vec![]);
    }

    #[test]
    fn render_lists_every_component_and_the_summary() {
        let (_tmp, _root, refs) = fixture(&["a", "b"]);
        let mapping = parse_mapping("dlls/a mpf-a ported P1\ndlls/ghost - todo P2\n").unwrap();
        let dlls_dir = refs.join("wine").join("dlls");
        let text = render(&dlls_dir, &inventory(&dlls_dir, &mapping).unwrap());
        assert!(text.starts_with("== dlls inventory: "), "{text}");
        assert!(text.contains("dlls/a"), "{text}");
        assert!(text.contains("dlls/b"), "{text}");
        assert!(text.contains("mpf-a"), "{text}");
        assert!(text.contains("(not in the reference tree)"), "{text}");
        assert!(
            text.trim_end()
                .ends_with("== 3 components: 1 ported, 0 partial, 0 planned, 0 native, 2 todo"),
            "{text}"
        );
    }

    #[test]
    fn run_prints_the_inventory_and_succeeds() {
        let (tmp, root, refs) = fixture(&["a"]);
        fs::create_dir_all(root.join("coverage")).unwrap();
        fs::write(
            root.join("coverage/mapping.txt"),
            "dlls/a mpf-a partial P1\n",
        )
        .unwrap();
        assert_eq!(run(&root, &refs), ExitCode::SUCCESS);
        drop(tmp);
    }

    #[test]
    fn run_fails_without_the_mapping_file_or_the_reference_tree() {
        let (tmp, root, refs) = fixture(&["a"]);
        // No coverage/mapping.txt.
        assert_eq!(run(&root, &refs), ExitCode::FAILURE);
        // Mapping present, reference tree missing.
        fs::create_dir_all(root.join("coverage")).unwrap();
        fs::write(
            root.join("coverage/mapping.txt"),
            "dlls/a mpf-a ported P1\n",
        )
        .unwrap();
        assert_eq!(run(&root, &tmp.0.join("empty-refs")), ExitCode::FAILURE);
        // Mapping present, wine tree present but no dlls/ dir.
        let no_dlls = tmp.0.join("refs-without-dlls");
        fs::create_dir_all(no_dlls.join("wine")).unwrap();
        assert_eq!(run(&root, &no_dlls), ExitCode::FAILURE);
        // A malformed mapping row is an error too.
        fs::write(root.join("coverage/mapping.txt"), "dlls/a mpf-a\n").unwrap();
        assert_eq!(run(&root, &refs), ExitCode::FAILURE);
    }
}
