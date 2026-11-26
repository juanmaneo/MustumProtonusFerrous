use crate::exports::Export;

/// The target CPU winebuild writes the `.def` file for. Stdcall names carry their argument bytes
/// only on i386, from `get_args_size` in `tools/winebuild/utils.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    I386,
    X86_64,
}

/// One line of the `.def` file for `export`, from the loop body of `output_def_file` in
/// `tools/winebuild/spec32.c`. The line ends with a newline.
pub fn def_line(export: &Export, target: Target) -> String {
    let name = match target {
        Target::I386 => format!("{}@{}", export.name, export.arg_bytes),
        Target::X86_64 => export.name.to_string(),
    };
    let forward = export
        .forward
        .map(|module_function| format!("={module_function}"))
        .unwrap_or_default();
    let private = if export.private { " PRIVATE" } else { "" };
    format!("  {name}{forward} @{}{private}\n", export.ordinal)
}

/// The `.def` file body for `exports`: the `LIBRARY` line, the `EXPORTS` line, then one line per
/// export. The generated-file header is left out because it names the local spec path.
pub fn def_file(module: &str, exports: &[Export], target: Target) -> String {
    let mut file = format!("LIBRARY {module}\n\nEXPORTS\n");
    for export in exports {
        file.push_str(&def_line(export, target));
    }
    file
}
