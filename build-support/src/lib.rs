/// Configure the build for a console application in this workspace.
///
/// Emits the `BUILD_YEAR` environment variable for the binary and its tests,
/// embeds the application icon and version metadata on Windows, and records
/// the files that should trigger a build script rerun.
pub fn configure() {
    emit_build_year();
    embed_windows_resources();
}

fn emit_build_year() {
    // The binary and the expected-output tests both read this value, so they
    // cannot disagree about the year around New Year.
    let build_year = chrono::Utc::now().format("%Y");
    println!("cargo:rustc-env=BUILD_YEAR={build_year}");

    // Rerun when the build script changes; Cargo does not do this by itself.
    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(windows)]
fn embed_windows_resources() {
    use build_print::{info, note, warn};
    use std::env;
    use std::path::Path;

    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set by Cargo");
    note!("Using Build Dir: {}", manifest_dir);

    let icon_path = Path::new(&manifest_dir).join("app.ico");

    let mut resource = winresource::WindowsResource::new();

    if icon_path.exists() {
        info!("Setting APP Icon: {}", icon_path.display());
        println!("cargo:rerun-if-changed={}", icon_path.display());

        let icon = icon_path
            .to_str()
            .expect("the app.ico path is not valid UTF-8");
        resource.set_icon(icon);
    } else {
        warn!("No APP Icon Found: {}", icon_path.display());
    }

    resource.set(
        "OriginalFilename",
        &env::var("CARGO_PKG_NAME").unwrap_or_default(),
    );
    resource.set(
        "FileDescription",
        &env::var("CARGO_PKG_DESCRIPTION").unwrap_or_default(),
    );

    if let Err(error) = resource.compile() {
        panic!("failed to embed Windows resources: {error}");
    }
}

#[cfg(not(windows))]
fn embed_windows_resources() {}
