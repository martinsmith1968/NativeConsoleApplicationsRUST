//! Shared helpers for the snapshot (insta) integration tests.
//!
//! Tests capture an application's output, normalise it with [`normalize_output`] (or
//! [`normalize_output_trimmed`]) and compare it against a stored snapshot with
//! [`assert_output_snapshot!`]. Review and accept changed snapshots with `cargo insta review`.

/// Re-exported so the macros below resolve `insta` without every app depending on it.
pub use insta;

/// Filter that normalises a GUID in the default (hyphenated) layout, keeping the version.
///
/// Use with the `filters` argument of [`assert_output_snapshot!`] for apps that emit random values.
pub const GUID_FILTERS: &[(&str, &str)] = &[
    (
        r"\b[0-9a-f]{8}-[0-9a-f]{4}-([0-9a-f])[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\b",
        "[guid-v$1]",
    ),
    (
        r"\b[0-9A-F]{8}-[0-9A-F]{4}-([0-9A-F])[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}\b",
        "[GUID-V$1]",
    ),
    (
        r"\b[0-9a-f]{12}([0-9a-f])[0-9a-f]{3}[89ab][0-9a-f]{15}\b",
        "[guid-nodashes-v$1]",
    ),
    (
        r"\b[0-9A-F]{12}([0-9A-F])[0-9A-F]{3}[89AB][0-9A-F]{15}\b",
        "[GUID-NODASHES-V$1]",
    ),
];

/// Filter that normalises a default length (21 character) NanoId on its own line.
pub const NANOID_FILTERS: &[(&str, &str)] = &[(r"(?m)^[A-Za-z0-9_-]{21}$", "[nanoid]")];

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

/// Escape the characters that are special in a regular expression.
fn escape_regex(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if "\\.+*?()|[]{}^$#&-~".contains(c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

/// Build the insta settings shared by every output snapshot.
///
/// The application version and build year are replaced with `[VERSION]` and `[YEAR]` so a
/// snapshot does not change when the version is stamped or the year rolls over; `filters`
/// are extra `(regex, replacement)` pairs for values that vary from run to run.
pub fn snapshot_settings(
    app_version: &str,
    build_year: &str,
    filters: &[(&str, &str)],
) -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.set_omit_expression(true);
    settings.add_filter(&escape_regex(app_version), "[VERSION]");
    settings.add_filter(
        &format!(r"(\d{{4}})-{}\b", escape_regex(build_year)),
        "${1}-[YEAR]",
    );
    for (pattern, replacement) in filters {
        settings.add_filter(pattern, *replacement);
    }
    settings
}

/// Assert that captured (already normalised) output matches its stored snapshot.
///
/// The snapshot is named after the calling test function and stored in `tests/snapshots/`.
/// The optional second argument is a slice of extra `(regex, replacement)` filters, for example
/// [`GUID_FILTERS`] or [`NANOID_FILTERS`].
#[macro_export]
macro_rules! assert_output_snapshot {
    ($output:expr) => {
        $crate::assert_output_snapshot!($output, &[])
    };
    ($output:expr, $filters:expr) => {{
        let _settings_guard =
            $crate::snapshot_settings(env!("CARGO_PKG_VERSION"), env!("BUILD_YEAR"), $filters)
                .bind_to_scope();
        $crate::insta::assert_snapshot!($output);
    }};
}
