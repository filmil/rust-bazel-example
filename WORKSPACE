workspace(name = "rust_bazel_examples")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "36ab8f9facae745c9c9c1b33d225623d976e78f2cc3f729b7973d8c20934ab95",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.31.0/rules_rust-v0.31.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(edition = "2021", versions = [ "1.79.0" ])

######################################################################

# Use to generate rust-project.json.
# Use bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

load(
    "@rules_rust//tools/rust_analyzer:deps.bzl",
    "rust_analyzer_dependencies")

rust_analyzer_dependencies()

######################################################################

# This is how to generate new lock files.  At the project outset, you must
#   (1) create the empty files `//third_party/cargo:Cargo.lock`, and
#       `//third_party/cargo:Cargo.Bazel.lock`.
#   (2) Run `env CARGO_BAZEL_REPI=true bazel build //...` to initialize the
#   lockfiles.
load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate", "render_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    packages = {
        # Add any other crates you need here.
        "bumpalo": crate.spec(
            version = "3.6.1",
        ),
    },
    render_config = render_config(
        default_package_name = ""
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
