# Marcus dependency upgrade notes

- Updated workspace manifest minimum versions only where a newer published crate version was confirmed.
- Left `sha1`, `sha2`, `tempfile`, `winresource`, `build-print`, `hex`, `md5`, and `strfmt` unchanged because the currently declared versions are already the latest published releases.
- Bumped manifest minimums for `clap`, `assert_cmd`, `predicates`, `regex`, `uuid`, and `nanoid`, then regenerated `Cargo.lock` with `cargo update`.
- Verified the dependency refresh with `cargo build --workspace` and `cargo test --workspace`; all crates still compile and all 345 tests pass.
