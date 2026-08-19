use markdown_it::MarkdownIt;
use std::sync::LazyLock;

// image-figure wraps the `Paragraph` node and keeps the original `Image`
// node intact as its only child instead of replacing it, so lazyload can be
// registered either before or after image-figure and still find the `Image`
// node.
static WITH_LAZYLOAD_REGISTERED_AFTER_MARKDOWN_IT: LazyLock<MarkdownIt> = LazyLock::new(|| {
    let mut mdit = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut mdit);
    markdown_it_image_figure::add(&mut mdit);
    markdown_it_lazyload::add(&mut mdit);

    mdit
});

#[test]
fn test_with_lazyload_registered_after_render() {
    let input = r#"![image-description](https://example.com/example.png)"#;
    let out = WITH_LAZYLOAD_REGISTERED_AFTER_MARKDOWN_IT
        .parse(input)
        .render();
    let expected = r#"<figure><img loading="lazy" src="https://example.com/example.png" alt="image-description"><figcaption>image-description</figcaption></figure>
"#;

    assert_eq!(out, expected);
}
