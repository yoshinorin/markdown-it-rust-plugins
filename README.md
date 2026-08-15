# markdown-it-rust-plugins

[![CI](https://github.com/yoshinorin/markdown-it-rust-plugins/actions/workflows/ci.yml/badge.svg)](https://github.com/yoshinorin/markdown-it-rust-plugins/actions/workflows/ci.yml)

Monorepo of plugins for [markdown-it.rs](https://crates.io/crates/markdown-it) that I use.

- [markdown-it-lazyload](crates/markdown-it-lazyload/README.md)
- [markdown-it-image-caption](crates/markdown-it-image-caption/README.md)

## Development

To use this crate from another Rust project before it is published to crates.io, see [docs/LOCAL_USAGE.md](./docs/LOCAL_USAGE.md).

```sh
# fmt
$ cargo fmt

# test
$ cargo test
$ cargo test -- --nocapture

# docs
$ cargo doc --open
```

## Publish

```
$ cd ./crates/<crate_name>
$ cargo publish --dry-run
$ cargo package --list
$ cargo login
$ cargo publish
```