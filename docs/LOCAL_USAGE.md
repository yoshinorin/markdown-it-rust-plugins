# Using Local Crates from Another Rust Project

How to use the plugins in this repository (`crates/*`) from another Rust project before publishing them to crates.io.

In the examples below, replace `<plugin-name>` with the actual crate name (e.g. `markdown-it-image-caption`, `markdown-it-lazyload`), and `<path-to-repo>` with the path to this repository from the consumer project (e.g. `../markdown-it-rust-plugins`).

## Method 1: Path dependency (basic)

Add a local path to the consumer project's `Cargo.toml`.

```toml
[dependencies]
markdown-it = "0.6"
<plugin-name> = { path = "<path-to-repo>/crates/<plugin-name>" }
```

- Both relative and absolute paths work. Forward slashes (`/`) work on Windows as well
- A relative path is resolved relative to the consumer's `Cargo.toml`
- Edits to the plugin source are picked up automatically on the consumer's next build (no publish step required)

Or via the command line:

```sh
cargo add <plugin-name> --path <path-to-repo>/crates/<plugin-name>
```

Usage example:

```rust
fn main() {
    let mut md = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut md);
    <plugin-name>::add(&mut md); // crate name with `-` replaced by `_`, e.g. markdown_it_lazyload

    let html = md.parse("![Rust](https://example.com/example.png)").render();
    println!("{html}");
}
```

## Method 2: `[patch.crates-io]` (pre-release verification)

Keep the dependency declaration identical to the published form and only swap the implementation with the local one.
After publishing, removing the patch line switches the consumer to the crates.io version, which makes this well suited for final verification just before a release.

```toml
[dependencies]
<plugin-name> = "x.y.z"

[patch.crates-io]
<plugin-name> = { path = "<path-to-repo>/crates/<plugin-name>" }
```

Notes:

- The patched crate's version must be compatible with the version requirement in `[dependencies]`
- Patching a crate that has never been published fails to resolve because it does not exist in the registry. Use Method 1 while the crate is unpublished, and Method 2 to verify the switchover after publishing

## Method 3: Git dependency (supplementary)

If the changes are pushed, a git reference works instead of a path.
Useful for verifying from another machine or CI.

```toml
[dependencies]
<plugin-name> = { git = "https://github.com/yoshinorin/markdown-it-rust-plugins", branch = "main" }
```

## Pre-publish checklist

- Do not publish with path/git dependencies: `cargo publish` ignores `path` / `git` in dependency specifications and uses only `version`, so any dependency that consumers need must also declare a `version`
- Specify only one of `license` or `license-file` in `Cargo.toml`. cargo rejects manifests that set both
- The manifest key for excluding files from the package is `exclude`. Unknown keys are silently ignored, so a typo means the files get packaged
- Verify the package contents before publishing:

```sh
cargo publish --dry-run -p <plugin-name>
cargo package --list -p <plugin-name>
```
