extern crate winresource;

use std::env;
use std::path::Path;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();

        let build_dir = env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_e| env::current_dir().unwrap().display().to_string());
        println!("cargo:warning=Using Build Dir: {}", build_dir);

        let icon_file = Path::new(&build_dir).join("app.ico");

        println!(
            "cargo:warning=Checking icon: {} - {}",
            icon_file.display(),
            icon_file.exists()
        );
        if icon_file.exists() {
            println!("Setting APP Icon: {}", icon_file.display());
            println!("cargo:rerun-if-changed={}", icon_file.display());
            res.set_icon(icon_file.to_str().unwrap());
        }

        res.set("FileDescription", &env::var("CARGO_PKG_DESCRIPTION").unwrap_or_default());
        res.compile().unwrap();
    }
}
