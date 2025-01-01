# markdown-it-lazyload

Add `lazy` to [`loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading) property in `<img>` tag.

## Usage

```rs
let mut md = markdown_it::MarkdownIt::new();
markdown_it::plugins::cmark::add(&mut md);
markdown_it_lazyload::add(&mut parser);

md.parse("![Rust](https://example.com/example.png)").render();
// <p><img src="https://example.com/example.png" alt="Rust" loading="lazy"></p>
```

## Specification

- [MDN: HTMLImageElement: loading property](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading)
- [CommonMark: Image](https://commonmark.org/help/tutorial/08-images.html)
