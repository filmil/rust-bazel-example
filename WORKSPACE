workspace(name = "rust_bazel_examples")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


http_archive(
    name = "rules_rust",
    sha256 = "accb5a89cbe63d55dcdae85938e56ff3aa56f21eb847ed826a28a83db8500ae6",
    strip_prefix = "rules_rust-9aa49569b2b0dacecc51c05cee52708b7255bd98",
    urls = [
        # Main branch as of 2021-02-19
        "https://github.com/bazelbuild/rules_rust/archive/9aa49569b2b0dacecc51c05cee52708b7255bd98.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(version="1.56.0",edition="2021")

######################################################################

# Use to generate rust-project.json.
# Use bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

load(
    "@rules_rust//tools/rust_analyzer:deps.bzl",
    "gen_rust_project_dependencies")

gen_rust_project_dependencies()

# Used to make bazel download remote rust repositories.
load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
