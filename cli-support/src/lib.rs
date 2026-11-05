//! Command-line setup shared by the console applications.

/// The `about` / `long_about` banner: `<name> v<version> - <description>` and the
/// copyright line.
///
/// A macro rather than a constant so that `env!` reads the calling application's
/// package metadata and the `BUILD_YEAR` its build script sets.
#[macro_export]
macro_rules! banner {
    () => {
        concat!(
            env!("CARGO_PKG_NAME"),
            " v",
            env!("CARGO_PKG_VERSION"),
            " - ",
            env!("CARGO_PKG_DESCRIPTION"),
            "\nCopyright \u{00A9} 2025-",
            env!("BUILD_YEAR"),
            " Martin Smith"
        )
    };
}

/// The help (`-h`, `-?`) and version (`-V`, `-!`) flags every application takes.
/// Flatten it into the application's arguments, last, so the flags are listed last.
///
/// It replaces clap's own help and version flags and requires every argument to
/// have help text.
#[derive(clap::Args, Debug, Default)]
#[command(
    disable_help_flag = true,
    disable_version_flag = true,
    help_expected = true
)]
pub struct StandardArgs {
    /// Print help
    #[arg(short = 'h', long, visible_short_alias = '?', action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, visible_short_alias = '!', action = clap::ArgAction::Version)]
    version: Option<bool>,
}
