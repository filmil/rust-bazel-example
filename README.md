# rust-bazel-example ![Build](https://github.com/filmil/rust-bazel-example/actions/workflows/build.yml/badge.svg)

> Automated builds are executed on each PR, and once a week even if no changes.*

An example starter project that uses bazel to compile rust programs.

I wanted to do this because I think [bazel] is useful for building
project with many target types, and we should use it more.

One of the issues with bazel is that many rules have poor IDE integration.
Rust rules are *not* one of those, which is nice. Here is how it
works with nvim and [LanguageClient-neovim][lcneovim].

# Try it out

```
bazel run //program:my_program
```

## Things I did to make this happen

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

  The file [program/BUILD.bazel](program/BUILD.bazel) shows how you can build a
  rust binary, that uses a dependency, in this case [bumpalo][bpl].  Note how
  the dependency is referred through its label `@crate_index//:bumpalo`.

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
