# markdown-it-lazyload

[![markdown-it-lazyload](https://img.shields.io/crates/v/markdown-it-lazyload.svg)](https://crates.io/crates/markdown-it-lazyload) [![markdown-it-lazyload](https://docs.rs/markdown-it-lazyload/badge.svg)](https://docs.rs/markdown-it-lazyload)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin to add `lazy` to [`loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading) property in `<img>` tag.

## Usage

```rs
let mut parser = markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(parser);
markdown_it_lazyload::add(parser);

parser.parse("![Rust](https://example.com/example.png)").render();
// <p><img loading="lazy" src="https://example.com/example.png" alt="Rust"></p>
```

See the [tests](./tests/lib.rs) for more examples.

## Interoperability

> [!NOTE]
> Applies to `1.0.0` or later. In `0.1.x`, this plugin replaced the `Image` node with its own node, so it did not compose with other plugins that process the `Image` node.

Since `1.0.0`, this plugin does not replace the `Image` node; it only appends a `loading` attribute to it.
Therefore it composes with other plugins that process the `Image` node (e.g. [markdown-it-image-caption](../markdown-it-image-caption)). That plugin wraps rather than replaces the `Image` node, so registration order between the two does not matter:

```rs
markdown_it_lazyload::add(&mut parser);      // adds `loading` to the Image node
markdown_it_image_caption::add(&mut parser); // wraps the Image node (keeps it intact)
```

## Specification

- [MDN: HTMLImageElement: loading property](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading)
- [CommonMark: Image](https://commonmark.org/help/tutorial/08-images.html)
