extern crate winresource;

use std::env;
use std::path::Path;

fn main() {
    let binaries = ["uuidgen", "hashcalc"];

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        for binary in binaries {
            let mut res = winresource::WindowsResource::new();

            let build_dir = env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_e| env::current_dir().unwrap().display().to_string());

            let icon_file = Path::new(&build_dir)
                .join("src")
                .join("bin")
                .join(binary)
                .join("app.ico");

            println!("cargo:warning=Checking: {}/{}", binary, icon_file.display());
            println!("Checking: {}", icon_file.display());
            if icon_file.exists() {
                println!("Setting APP Icon: {}", icon_file.display());
                println!("cargo:rerun-if-changed={}", icon_file.display());
                res.set_icon(icon_file.to_str().unwrap());
            }

            res.set("'FileDescription", "My utility to do something");

            res.compile().unwrap();
        }
    }
}
