# synotest

<p align="center">
  <img src="https://synopkg.github.io/synotest/logo.svg" width="200" height="179" alt="">
  <br>Consistent dependency versions in large JavaScript Monorepos.
  <br><a href="https://synopkg.github.io/synotest">https://synopkg.github.io/synotest</a>
</p>

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
