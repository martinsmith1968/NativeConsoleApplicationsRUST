extern crate winresource;

use build_print::*;
use std::env;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let build_year = 1970 + (secs / 31_557_600);
    std::println!("cargo:rustc-env=BUILD_YEAR={}", build_year);

    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();

        let build_dir = env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_e| env::current_dir().unwrap().display().to_string());
        note!("Using Build Dir: {}", build_dir);

        let icon_file = Path::new(&build_dir).join("app.ico");

        if icon_file.exists() {
            info!("Setting APP Icon: {}", icon_file.display());
            std::println!("cargo:rerun-if-changed={}", icon_file.display());
            res.set_icon(icon_file.to_str().unwrap());
        } else {
            warn!("No APP Icon Found: {}", icon_file.display());
        }

        res.set(
            "OriginalFilename",
            &env::var("CARGO_PKG_NAME").unwrap_or_default(),
        );

        res.set(
            "FileDescription",
            &env::var("CARGO_PKG_DESCRIPTION").unwrap_or_default(),
        );
        res.compile().unwrap();
    }
}
