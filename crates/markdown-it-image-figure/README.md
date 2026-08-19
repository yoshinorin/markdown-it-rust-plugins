# markdown-it-image-figure

[![markdown-it-image-figure](https://img.shields.io/crates/v/markdown-it-image-figure.svg)](https://crates.io/crates/markdown-it-image-figure) [![markdown-it-image-figure](https://docs.rs/markdown-it-image-figure/badge.svg)](https://docs.rs/markdown-it-image-figure)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin that wraps a standalone image (a paragraph whose only content is a single image) in `<figure>`, rendering the image description as `<figcaption>`. The `alt` attribute is populated with the plain-text description per the [CommonMark](https://commonmark.org/help/tutorial/08-images.html) specification.

`<figure>`/`<figcaption>` are not part of the CommonMark spec. This plugin is a non-standard extension.

## Usage

```rs
let mut parser = markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(&mut parser);
markdown_it_image_figure::add(&mut parser);

parser.parse("![image-description](img.jpg)").render();
// <figure><img src="img.jpg" alt="image-description"><figcaption>image-description</figcaption></figure>
```

See the [tests](./tests/lib.rs) for more examples.

## Examples

| Input | Output |
|---|---|
| `![image-description](img.jpg)` | `<figure><img src="img.jpg" alt="image-description"><figcaption>image-description</figcaption></figure>` |
| `![Rust](img.jpg "img_title")` | `<figure><img src="img.jpg" alt="Rust" title="img_title"><figcaption>Rust</figcaption></figure>` |
| `![*italic* **bold**](img.jpg)` | `<figure><img src="img.jpg" alt="italic bold"><figcaption><em>italic</em> <strong>bold</strong></figcaption></figure>` |
| `![](img.jpg)` | `<p><img src="img.jpg" alt=""></p>` |
| `![desc]()` | `<p><img src="" alt="desc"></p>` |
| `foobar ![desc](img.jpg) hoge` | `<p>foobar <img src="img.jpg" alt="desc"> hoge</p>` |

## Behavior

`<figure>` is a block-level element, so this plugin only wraps an image when its paragraph contains nothing else — no surrounding text, no other inline content. The `<figure>` wrapping (and `<figcaption>`) is applied only when all of the following hold:

- the image is the sole content of its paragraph
- `src` is non-empty (whitespace-only counts as empty)
- the image has a description

If an image shares a paragraph with other text, it is left untouched as a plain `<img>` — no caption is emitted, and the paragraph is not split. Splitting a paragraph into separate blocks around an inline image would change the document's structure in a way the author likely didn't intend.

## Interoperability

This plugin wraps the `Paragraph` node instead of replacing the `Image` node, keeping the `Image` node intact as its only child.
Plugins that inspect or append attributes to `Image` nodes (e.g. [markdown-it-lazyload](../markdown-it-lazyload)) compose with this plugin regardless of registration order:

```rs
markdown_it_lazyload::add(&mut parser);     // adds `loading` to the Image node
markdown_it_image_figure::add(&mut parser); // wraps the Paragraph node (Image stays intact)

parser.parse("![Rust](img.jpg)").render();
// <figure><img loading="lazy" src="img.jpg" alt="Rust"><figcaption>Rust</figcaption></figure>
```

The same output is produced if the two `add()` calls are swapped.

## Specification

- [CommonMark: Image](https://commonmark.org/help/tutorial/08-images.html)
- [MDN: `<figure>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure)
- [MDN: `<figcaption>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
