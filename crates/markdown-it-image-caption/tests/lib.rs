use markdown_it::MarkdownIt;
use markdown_it_image_caption;
use std::sync::LazyLock;

static MARKDOWN_IT: LazyLock<MarkdownIt> = LazyLock::new(|| {
    let mut mdit = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut mdit);
    markdown_it_image_caption::add(&mut mdit);

    mdit
});

static ADDED_CAPTION_RULE_BEFORE_CMARK_MARKDOWN_IT: LazyLock<MarkdownIt> = LazyLock::new(|| {
    let mut mdit = markdown_it::MarkdownIt::new();
    markdown_it_image_caption::add(&mut mdit);
    markdown_it::plugins::cmark::add(&mut mdit);

    mdit
});

#[test]
fn test_inline_full_url_render() {
    let input = r#"![image-description](https://example.com/example.png)"#;
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="image-description"><sub>image-description</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_title_render() {
    let input = r#"![Rust](https://example.com/example.png 'img_title')"#;
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="Rust" title="img_title"><sub>Rust</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_add_plugin_before_cmark_render() {
    let input = r#"![Rust](https://example.com/example.png)"#;
    let out = ADDED_CAPTION_RULE_BEFORE_CMARK_MARKDOWN_IT
        .parse(input)
        .render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="Rust"><sub>Rust</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_description_with_spaces_render() {
    let input = r#"![  hello  world  ](https://example.com/example.png)"#;
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="  hello  world  "><sub>  hello  world  </sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_description_with_emphasis_render() {
    let input = r#"![*italic* and **bold**](https://example.com/example.png)"#;
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="italic and bold"><sub><em>italic</em> and <strong>bold</strong></sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_description_with_code_render() {
    let input = "![see `Vec<T>`](https://example.com/example.png)";
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="see Vec&lt;T&gt;"><sub>see <code>Vec&lt;T&gt;</code></sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_description_html_escaped_render() {
    let input = r#"![1 < 2 & 3 > 0](https://example.com/example.png)"#;
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt="1 &lt; 2 &amp; 3 &gt; 0"><sub>1 &lt; 2 &amp; 3 &gt; 0</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_empty_description_render() {
    let input = r#"![](https://example.com/example.png)"#;
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="https://example.com/example.png" alt=""></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_empty_src_render() {
    let input = "![desc]()";
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="" alt="desc"></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_empty_render() {
    let input = "![]()";
    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<p><img src="" alt=""></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_block_render() {
    let input = r#"
## H2 Title

![desc1](https://example.com/example1.png)
foobar
![](https://example.net/example2.png)
hoge
![](./example3.png) ![Example4](./../example4.png)
"#;

    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<h2>H2 Title</h2>
<p><img src="https://example.com/example1.png" alt="desc1"><sub>desc1</sub>
foobar
<img src="https://example.net/example2.png" alt="">
hoge
<img src="./example3.png" alt=""> <img src="./../example4.png" alt="Example4"><sub>Example4</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_block_without_render() {
    let input = r#"
## H2 Title
foobar
hoge
[link](https://example.com)
"#;

    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<h2>H2 Title</h2>
<p>foobar
hoge
<a href="https://example.com">link</a></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_inline_xrender() {
    let input = r#"![Rust](https://example.com/example.png)"#;
    let out = MARKDOWN_IT.parse(input).xrender();
    let expected = r#"<p><img src="https://example.com/example.png" alt="Rust" /><sub>Rust</sub></p>
"#;

    assert_eq!(out, expected);
}

#[test]
fn test_raw_html_img_tag_render() {
    let input = r#"
## H2 Title
foobar
hoge
<img src="raw.rust.png" alt="Raw" title="raw_image_title">
![Rust](https://example.com/example.png)
"#;

    let out = MARKDOWN_IT.parse(input).render();
    let expected = r#"<h2>H2 Title</h2>
<p>foobar
hoge
&lt;img src=&quot;raw.rust.png&quot; alt=&quot;Raw&quot; title=&quot;raw_image_title&quot;&gt;
<img src="https://example.com/example.png" alt="Rust"><sub>Rust</sub></p>
"#;

    assert_eq!(out, expected);
}
