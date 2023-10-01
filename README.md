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

## Re-generate `rust-project.json`

You want to run the command below whenever your list of rust external dependencies changes.
This command is documented at: https://bazelbuild.github.io/rules_rust/rust_analyzer.html

```
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```

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
September 30, 2023. Previous versions of this repository used an older version
of `rules_rust`, and the procedure to bring in upstream crates was quite
different.

* Edited [WORKSPACE](workspace) file to:

  * include rules for building rust.
  * add a rule that makes bazel download remote rust repositories.
  * add a rule to download external rust crates.

* Made sure that `target` and `rust-project.json` are in
  [.gitignore](.gitignore). This allows us to use regular cargo builds in rust
  only directories. This *might* come in handy.

* Ran the following, to generate `rust-project.json` in `//`:

  ```
  bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
  ```

  This will enable you to use [rust-analyzer][ra] as your language server for
  the project and all vendored crates.

# Notes

* The files `//:Cargo.Bazel.lock` and `//:Cargo.lock` would probably better fit
  into a directory `//third_party/cargo`, but that does not seem to work.

[bazel]: https://bazel.io
[bpl]: https://docs.rs/bumpalo 
[cr]: https://github.com/google/cargo-raze
[lcneovim]: https://github.com/autozimu/LanguageClient-neovim
[ra]: https://rust-analyzer.github.io
