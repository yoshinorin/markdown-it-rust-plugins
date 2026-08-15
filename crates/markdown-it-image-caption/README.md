# markdown-it-image-caption

[![markdown-it-image-caption](https://img.shields.io/crates/v/markdown-it-image-caption.svg)](https://crates.io/crates/markdown-it-image-caption) [![markdown-it-image-caption](https://docs.rs/markdown-it-image-caption/badge.svg)](https://docs.rs/markdown-it-image-caption)

A [markdown-it.rs](https://crates.io/crates/markdown-it) plugin that renders the image description as a `<sub>` caption placed immediately after the `<img>` tag. The `alt` attribute is populated with the plain-text description per the [CommonMark](https://commonmark.org/help/tutorial/08-images.html) specification.

## Usage

```rs
let mut parser = markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(&mut parser);
markdown_it_image_caption::add(&mut parser);

parser.parse("![image-description](img.jpg)").render();
// <p><img src="img.jpg" alt="image-description"><sub>image-description</sub></p>
```

See the [tests](./tests/lib.rs) for more examples.

## Examples

| Input | Output |
|---|---|
| `![image-description](img.jpg)` | `<img src="img.jpg" alt="image-description"><sub>image-description</sub>` |
| `![Rust](img.jpg "img_title")` | `<img src="img.jpg" alt="Rust" title="img_title"><sub>Rust</sub>` |
| `![*italic* **bold**](img.jpg)` | `<img src="img.jpg" alt="italic bold"><sub><em>italic</em> <strong>bold</strong></sub>` |
| `` ![see `Vec<T>`](img.jpg) `` | `<img src="img.jpg" alt="see Vec&lt;T&gt;"><sub>see <code>Vec&lt;T&gt;</code></sub>` |
| `![](img.jpg)` | `<img src="img.jpg" alt="">` |
| `![desc]()` | `<img src="" alt="desc">` |

## Behavior

The `<sub>` caption is rendered only when both of the following hold:

- `src` is non-empty (whitespace-only counts as empty)
- the image has a description

If `src` is empty, the image is invalid, so no caption is emitted either.

The `alt` attribute and the `<sub>` element are generated differently:

| Attribute / Element | Value | Reason |
|---|---|---|
| `alt` | plain text of the description | per the CommonMark specification |
| `<sub>` | description rendered as inline HTML | keeps inline formatting (emphasis, code, ...) in the caption |

For example, `![*italic*](img.jpg)` renders `alt="italic"` (plain text) but `<sub><em>italic</em></sub>` (keeps `<em>`).

## Interoperability

This plugin wraps the `Image` node instead of replacing it, keeping it intact as a child of the wrapper.
Plugins that inspect or append attributes to `Image` nodes (e.g. [markdown-it-lazyload](../markdown-it-lazyload)) compose with this plugin regardless of registration order:

```rs
markdown_it_lazyload::add(&mut parser);      // adds `loading` to the Image node
markdown_it_image_caption::add(&mut parser); // wraps the Image node (keeps it intact)

parser.parse("![Rust](img.jpg)").render();
// <p><img loading="lazy" src="img.jpg" alt="Rust"><sub>Rust</sub></p>
```

The same output is produced if the two `add()` calls are swapped.

## Specification

- [CommonMark: Image](https://commonmark.org/help/tutorial/08-images.html)
- [MDN: `<sub>` element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/sub)
