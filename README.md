# Temporary repo.

<a aria-label="synopkg logo" href="https://synopkg.github.io/synotest">
  <img alt="" src="https://img.shields.io/badge/Made%20by%20synopkg-000000.svg?style=flat-square&logo=synopkg&labelColor=000">
</a>
<a aria-label="NPM version" href="https://www.npmjs.com/package/octotask">
  <img alt="" src="https://img.shields.io/npm/v/synopkg.svg?style=flat-square&labelColor=000000">
</a>
<a aria-label="CI status" href="https://github.com/synopkg/synopkg-test/actions/workflows/test-site.yaml?query=event%3Apush+branch%3Amain">
  <img alt="" src="https://img.shields.io/github/actions/workflow/status/synopkg/synopkg-test/test-site.yaml?event=push&branch=main&style=flat-square&labelColor=000000">
</a>

## Rust

A work in progress implementation of Synopkg in Rust. It is not ready for public use.

## Develop

```shell
git clone https://github.com/synopkg/synotest.git -b main synotest-rust
cd synotest-rust
```

## Run (Development)

There are 2 commands, `lint` and `fix`.

```shell
cargo run -- lint
cargo run -- fix
```

Both will check formatting and version/range mismatches by default, but can be filtered with `--format` and `--versions`.

## Build and Run (Production)

```shell
cargo build --release
target/release/synotest lint
target/release/synotest fix
```
