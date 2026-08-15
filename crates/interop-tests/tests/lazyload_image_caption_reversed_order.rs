use markdown_it::MarkdownIt;
use std::sync::LazyLock;

// image-caption keeps the original `Image` node intact (as a child of its
// own wrapper node) instead of replacing it, so lazyload can be registered
// either before or after image-caption and still find the `Image` node.
static WITH_LAZYLOAD_REGISTERED_AFTER_MARKDOWN_IT: LazyLock<MarkdownIt> = LazyLock::new(|| {
    let mut mdit = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut mdit);
    markdown_it_image_caption::add(&mut mdit);
    markdown_it_lazyload::add(&mut mdit);

    mdit
});

#[test]
fn test_with_lazyload_registered_after_render() {
    let input = r#"![image-description](https://example.com/example.png)"#;
    let out = WITH_LAZYLOAD_REGISTERED_AFTER_MARKDOWN_IT
        .parse(input)
        .render();
    let expected = r#"<p><img loading="lazy" src="https://example.com/example.png" alt="image-description"><sub>image-description</sub></p>
"#;

    assert_eq!(out, expected);
}
