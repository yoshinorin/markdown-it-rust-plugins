# markdown-it-lazyload

Add `lazy` to [`loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading) property in `<img>` tag.

## Usage

```rs
let mut parser = markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(parser);
markdown_it_lazyload::add(parser);

md.parse("![Rust](https://example.com/example.png)").render();
// <p><img src="https://example.com/example.png" alt="Rust" loading="lazy"></p>
```

See the [tests](./tests/lib.rs) for more examples.

## Specification

- [MDN: HTMLImageElement: loading property](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading)
- [CommonMark: Image](https://commonmark.org/help/tutorial/08-images.html)
