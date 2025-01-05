# markdown-it-lazyload

[![markdown-it-lazyload](https://img.shields.io/crates/v/markdown-it-lazyload.svg)](https://crates.io/crates/markdown-it-lazyload) [![markdown-it-lazyload](https://docs.rs/markdown-it-lazyload/badge.svg)](https://docs.rs/markdown-it-lazyload)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to add `lazy` to [`loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading) property in `<img>` tag.

## Usage

```rs
let mut parser = markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(parser);
markdown_it_lazyload::add(parser);

parser.parse("![Rust](https://example.com/example.png)").render();
// <p><img src="https://example.com/example.png" alt="Rust" loading="lazy"></p>
```

See the [tests](./tests/lib.rs) for more examples.

## Specification

- [MDN: HTMLImageElement: loading property](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading)
- [CommonMark: Image](https://commonmark.org/help/tutorial/08-images.html)
