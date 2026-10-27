# ToDo 2

Outstanding findings from [Recommendations.md](Recommendations.md) (review of 2026-10-26, branch `task/ai-recommendations`).
Tick each item as it is completed.

## printformat structure and gaps

- [ ] Extract one tokenizer for the `{}` scanning loop repeated in `preprocess_format`, `count_required_args` and `translate_csharp_format`
- [ ] Split `printformat/src/main.rs` (534 lines) into modules, the way `hashcalc` splits its hashers
- [ ] Support C# `X` (hex), `N` (thousands separator) and `C` (currency) in `translate_csharp_specifier`
- [ ] Remove `impl Display for FormatType`, which duplicates what `ValueEnum` provides
- [ ] Make `%c` reject (or warn about) arguments longer than one character instead of silently using the first

## Cargo and workspace (priority 3)

- [ ] Add `[workspace.package]` and hoist the shared package metadata
- [ ] Add `[workspace.dependencies]` and hoist shared versions (`clap`, `assert_cmd`, `predicates`, `winresource`, `build-print`, …)
- [ ] Change `repository` from `http://` to `https://` in all four manifests
- [ ] Resolve the license: add the Apache-2.0 text, or declare `MIT` only
- [ ] Fix `readme = "../README.md"` so it no longer points outside each crate (breaks `cargo package`/`publish`)
- [ ] Add `[profile.release]` tuning (`lto`, `strip`, `panic = "abort"`, `codegen-units`)
- [ ] Add `rust-version` (MSRV; `edition = "2024"` requires Rust 1.85)
- [ ] Add `[workspace.lints]`

## CI and repository (priority 4)

- [ ] Add a `cargo clippy --all-targets -- -D warnings` step (clippy is installed but never run)
- [ ] Add a `cargo audit` or `cargo deny` step
- [ ] Add `Swatinem/rust-cache` (or equivalent)
- [ ] Replace `permissions: write-all` on the tag and release jobs with `contents: write`
- [ ] Split the 592-line monolithic workflow (consider reusable workflows)
- [ ] Add the `cargo` ecosystem to Dependabot

## Repository and documentation

- [x] Make the project name consistent (`PROJECT_NAME` in the workflow, the repository name, the README title)
- [x] Fix README typos: "framworks", "sonsumption", "bulids", "currenly"
- [x] Rewrite the garbled README overview ("...as the complemented and are consumed by...")
- [x] Remove README references to PauseN and per-user file storage
- [ ] Decide whether `.squad/` (118 files) and `.copilot/` (33 files) belong in this repository
- [ ] Decide whether to add non-PowerShell scripts (currently PowerShell-only with `.cmd` wrappers)
- [ ] Check `.vscode/`: commit it or add it to `.gitignore` (in the original analysis, missing from the 2026-10-26 review)

## Shared code and tests (priorities 5 and 6)

- [ ] Extract the duplicated clap boilerplate (`-h/-?`, `-V/-!`, `about`/`long_about`, `help_expected`, `after_help`) into a shared crate
- [ ] Move each app's logic into `lib.rs` and keep `main.rs` thin
  - [ ] bannertext
  - [ ] hashcalc
  - [ ] printformat
  - [ ] uuidgen
- [ ] Make the test layout consistent (`src/main_tests.rs`, `#[path]` hasher modules, `tests/<app>.rs` stubs)
- [ ] Consider insta snapshots for the 42 expected-output `.example` files
- [ ] Add property-based tests with `proptest` (banner width invariant, format parsers)

## Minor

- [ ] Stop treating base64 as a hash: rename the `hash_content`/`hash_reader` abstraction (e.g. `convert`/`transform`) or keep the encoding path separate, and revisit the `[base64]` output line
