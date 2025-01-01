# markdown-it-rust-plugins

Monorepo of plugins for [markdown-it.rs](https://crates.io/crates/markdown-it) that I use.

- [markdown-it-lazyload](crates/markdown-it-lazyload/README.md)
- Other plugins will be added as they are created.

## Development

```sh
# fmt
$ cargo fmt

# test
$ cargo test
$ cargo test -- --nocapture

# docs
$ cargo doc --open

# publish
$ cd ./crates/<crate_name>
$ cargo publish
```