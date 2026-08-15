use markdown_it::MarkdownIt;
use std::sync::LazyLock;

static WITH_LAZYLOAD_MARKDOWN_IT: LazyLock<MarkdownIt> = LazyLock::new(|| {
    let mut mdit = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut mdit);
    markdown_it_lazyload::add(&mut mdit);
    markdown_it_image_caption::add(&mut mdit);

    mdit
});

#[test]
fn test_with_lazyload_render() {
    let input = r#"![image-description](https://example.com/example.png)"#;
    let out = WITH_LAZYLOAD_MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img loading="lazy" src="https://example.com/example.png" alt="image-description"><sub>image-description</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_with_lazyload_title_render() {
    let input = r#"![Rust](https://example.com/example.png 'img_title')"#;
    let out = WITH_LAZYLOAD_MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img loading="lazy" src="https://example.com/example.png" alt="Rust" title="img_title"><sub>Rust</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_with_lazyload_empty_src_render() {
    let input = "![desc]()";
    let out = WITH_LAZYLOAD_MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="" alt="desc"></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_with_lazyload_empty_description_render() {
    let input = r#"![](https://example.com/example.png)"#;
    let out = WITH_LAZYLOAD_MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img loading="lazy" src="https://example.com/example.png" alt=""></p>
"#;

    assert_eq!(out, expected);
}
