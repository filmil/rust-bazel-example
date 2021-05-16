# rust-bazel-example

An example starter project that uses bazel to compile rust programs.

I wanted to do this because I think [bazel] is super-useful for building larger
project with many targets, and we should use it more.

One of the issues with bazel is that many languages have poor IDE integration.
Rust is *not* one of those languages, which is nice. I wanted to check how it
works with nvim and [LanguageClient-neovim][lcneovim].

## Things I did to make this happen

Changes needed to make a minimal program that uses bazel to build a rust binary.

* Edited [WORKSPACE](workspace) file:

  * to include rules for building rust.
  * to add a rule `gen_rust_project_dependencies` that allow running
    `bazel run @rules_rust//tools/rust_analyzer:gen_rust_project`
    to refresh the `rust-project.json` file that lives in the workspace
    root.
  * to add a rule that makes bazel download remote rust repositories.

* Made the directory [//third_party/cargo](third_party/cargo) that contains the
  file [Cargo.toml](third_party/cargo/Cargo.toml) which is defined to direct
  `cargo raze` runs.  `cargo raze` should be run in that directory when cargo
  dependencies are modified.

  While the `cargo-raze` documentation says that crates go by default to `//cargo`,
  I wanted to see what it would look like if rust crates were added at the same level
  as any other external dependencies.

* Made the file [program/Cargo.toml](program/Cargo.toml) which can be used to 
  compile `program` with the regular `cargo build` from that directory.

  The file [program/BUILD.bazel](program/BUILD.bazel) shows how you can build a
  rust binary, that uses a dependency, in this case [bumpalo][bpl].  Note how
  the dependency is referred through its label `//third_party/cargo:bumpalo`.

* Made sure that `target` and `rust-project.json` are in
  [.gitignore](.gitignore).

* Add the `rust_analyzer` rule in [//BUILD.bazel](BUILD.bazel).

* Ran the following, which generates `rust-project.json` in `//`:

  ```
  bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
  ```

[cr]: https://github.com/google/cargo-raze
[bpl]: https://docs.rs/bumpalo 
[bazel]: https://bazel.io
[lcneovim]: https://github.com/autozimu/LanguageClient-neovim
