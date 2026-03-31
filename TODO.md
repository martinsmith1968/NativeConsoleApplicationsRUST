# To Do

## Fixes

- None

### General

- [ ] Rename Help short char to '?'
  - May need to be done via fluent builder as derive doesn't seem to work
  - See :
    - https://github.com/clap-rs/clap/issues/1127
    - https://stackoverflow.com/questions/78354022/when-i-disable-the-default-help-option-and-add-it-as-a-custom-arg-it-says-that
- [ ] CI Build GitHub Action
  - https://docs.github.com/en/actions/tutorials/build-and-test-code/rust
  - https://github.com/actions-rust-lang/setup-rust-toolchain
  - https://users.rust-lang.org/t/github-actions-for-rust/116704
  - https://jondot.medium.com/building-rust-on-multiple-platforms-using-github-6f3e6f8b8458
  - https://blog.urth.org/2023/03/05/cross-compiling-rust-projects-in-github-actions/
  - https://eertmans.be/posts/rust-binaries-to-github-action/
  - https://www.reddit.com/r/rust/comments/ga80lj/example_github_actions_with_rust_build_test/
- [ ] Project / Solution Structure for Executables, libraries, tests, externals, etc
  - i.e. Support multiple cargo targets 
- [ ] Tests
  - https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
  - https://doc.rust-lang.org/book/ch11-01-writing-tests.html
  - https://www.reddit.com/r/rust/comments/18u8n38/mistakes_to_avoid_while_writing_unit_test_for/
  - https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/
  - https://doc.rust-lang.org/book/ch11-03-test-organization.html
  - https://stackoverflow.com/questions/68217374/writing-comprehensive-unit-tests-in-rust
  - https://www.freecodecamp.org/news/unit-testing-in-rust/
  - https://www.walknsqualk.com/020-rust-unit-test-layout/
- [ ] Introduce some app output tests
- [ ] Invoke via CI
- [ ] Add Windows File Properties
  - See : https://stackoverflow.com/questions/74509880/add-exe-file-details-to-binary-of-compiled-rust-code
  -     : https://github.com/BenjaminRi/winresource
  -     : https://crates.io/crates/windows_exe_info
  -     : https://doc.rust-lang.org/cargo/reference/build-scripts.html
  -     : https://www.40tude.fr/docs/06_programmation/rust/014_build_system/from_src_to_exe.html

### UUIDGen

- [ ] Move UUIDType (uuid/nanoid) to become a command (so won't require -t prefix) 

### HashCalc
- 
- [ ] Implement all required hashers
  - Will require each hasher in its own file / module
    - https://doc.rust-lang.org/rust-by-example/mod/split.html
    - https://www.reddit.com/r/rust/comments/16e571d/breaking_down_rust_code_into_seperate_rs_files_is/
    - https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
    - https://stackoverflow.com/questions/70317526/how-to-use-multiple-files-in-rust

## Enhancements

- [ ] All App implementations
