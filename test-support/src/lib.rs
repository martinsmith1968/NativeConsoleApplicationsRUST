use std::fs;
use std::path::Path;

/// Capture the name of the function that invokes this macro.
///
/// From <https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust>.
#[macro_export]
macro_rules! get_current_function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

/// Normalize line endings in captured output.
pub fn normalize_output(output: String) -> String {
    output.replace("\r\n", "\n")
}

/// Normalize line endings and strip trailing whitespace from each line.
pub fn normalize_output_trimmed(output: String) -> String {
    let normalized = normalize_output(output);
    let trimmed = normalized
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");

    if normalized.ends_with('\n') {
        trimmed + "\n"
    } else {
        trimmed
    }
}

/// Read an expected-output file, substitute the version and year placeholders,
/// and normalize the result with `normalize`.
pub fn load_expected_output(
    path: &Path,
    app_version: &str,
    build_year: &str,
    normalize: fn(String) -> String,
) -> String {
    let content = fs::read_to_string(path).unwrap_or_else(|error| {
        panic!(
            "failed to read expected output file {}: {error}",
            path.display()
        )
    });

    normalize(
        content
            .replace("%APP_VERSION%", app_version)
            .replace("%CURRENT_YEAR%", build_year),
    )
}
