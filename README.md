# rust-bazel-example ![Build](https://github.com/filmil/rust-bazel-example/actions/workflows/build.yml/badge.svg)

> Automated builds are executed on each PR, and once a week even if no code changed the week prior.

This is an example starter project that uses bazel to compile rust programs.

I wanted to do this because I think [bazel] is useful for building
project with many target types, and we should use it more.

One of the issues with bazel is that many rules have poor IDE integration.
Rust rules are *not* one of those, which is nice. This example generates
`rust-project.json`, which you can plug into the IDE of your choice, if your
IDE supports the Language Server Protocol ([LSP][lsp]).

[lsp]: https://langserver.org/

# Try it out *now*

The below command should simply work. File a bug if it does not.

```
bazel run //program:my_program
```

## Generate `rust-project.json` for LSP support

[rust-analyzer][ra] is the language server behind Rust support in most editors.
It normally discovers a project through `Cargo.toml`, which Bazel-built projects
do not necessarily have. Instead, it can read a project description from a file
named [`rust-project.json`][rpj], which `rules_rust` knows how to generate for
you from the Bazel build graph.

[rpj]: https://rust-analyzer.github.io/book/non_cargo_based_projects.html

Run this from the workspace root:

```
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```

That writes `//rust-project.json`, describing every `rust_*` target reachable
from `//...`, including the external crates pulled in by `crate_index`. It also
creates a `//.rules_rust_analyzer` cache directory. Both are generated files, so
both are in [.gitignore](.gitignore).

Re-run the command whenever you:

* add, remove, or rename a Rust target in a `BUILD.bazel` file,
* change a target's `srcs`, `deps`, `proc_macro_deps`, or `crate_features`, or
* change the list of external crates in [MODULE.bazel](MODULE.bazel).

To restrict generation to a subset of the build graph, pass target patterns:

```
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project -- //program/...
```

No extra setup is needed to make this work: `rules_rust` registers a
`rust_analyzer_toolchain` as part of the `@rust_toolchains//:all` toolchains
that [MODULE.bazel](MODULE.bazel) already registers.

### Point your editor at it

Most editors pick up `rust-project.json` automatically once it sits at the root
of the folder you opened, and no further configuration is required. A few need
to be told to stop looking for `Cargo.toml`:

* **VS Code** (rust-analyzer extension): usually automatic. If it insists on
  Cargo, add to `.vscode/settings.json`:

  ```json
  {
    "rust-analyzer.linkedProjects": ["rust-project.json"]
  }
  ```

* **Neovim** (`nvim-lspconfig`), **Helix**, **Emacs** (`lsp-mode` / `eglot`),
  and anything else speaking [LSP][lsp]: point the `rust-analyzer` server at
  this directory as the workspace root; it finds `rust-project.json` on its own.

* **JetBrains RustRover / IntelliJ Rust** do *not* read `rust-project.json`. Use
  the [Bazel plugin][bazelplugin] instead.

[bazelplugin]: https://plugins.jetbrains.com/plugin/8609-bazel

The upstream documentation for this tool lives at:
https://bazelbuild.github.io/rules_rust/rust_analyzer.html

Note that `rust-analyzer` reads `rust-project.json` but does not build through
Bazel, so its view of the code can drift from what `bazel build` sees until you
regenerate the file.

## Re-generate lockfiles

You want to run the command below whenever your list of rust external dependencies changes.
```
env CARGO_BAZEL_REPIN=true bazel build //...
```

# Where to go from here?

If you want to see more examples of rust use in Bazel, check out the `rules_rust` examples page
at: https://github.com/bazelbuild/rules_rust/tree/main/examples

# Things I did to make this happen

Here are the changes needed to make a minimal program that uses bazel to build
a rust binary.

The instructions below were current at the time of this writing, which was on
August 16, 2026. Previous versions of this repository used the legacy
`WORKSPACE` file, which Bazel 9 no longer supports; external dependencies are
now declared with [Bzlmod][bzlmod] in [MODULE.bazel](MODULE.bazel).

[bzlmod]: https://bazel.build/external/module

* Edited [MODULE.bazel](MODULE.bazel) file to:

  * add a `bazel_dep` on `rules_rust`, the rules for building rust.
  * use the `rust` module extension, which makes bazel download and register
    a remote rust toolchain.
  * use the `crate` module extension, which downloads external rust crates.

* Made sure that `target` and `rust-project.json` are in
  [.gitignore](.gitignore). This allows us to use regular cargo builds in rust
  only directories. This *might* come in handy.

* Ran the following, to generate `rust-project.json` in `//`:

  ```
  bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
  ```

  This will enable you to use [rust-analyzer][ra] as your language server for
  the project and all vendored crates. See [Generate `rust-project.json` for LSP
  support](#generate-rust-projectjson-for-lsp-support) above for the details.

# Notes

* The files `//:Cargo.Bazel.lock` and `//:Cargo.lock` would probably better fit
  into a directory `//third_party/cargo`, but that does not seem to work.

* External crates are declared directly with `crate.spec` in
  [MODULE.bazel](MODULE.bazel) rather than in a `Cargo.toml`. They therefore
  belong to `crate_index`'s root package, whose name is the empty string, which
  is why `all_crate_deps` in [program/BUILD.bazel](program/BUILD.bazel) is
  called with `package_name = ""`.

[bazel]: https://bazel.io
[bpl]: https://docs.rs/bumpalo 
[cr]: https://github.com/google/cargo-raze
[lcneovim]: https://github.com/autozimu/LanguageClient-neovim
[ra]: https://rust-analyzer.github.io
