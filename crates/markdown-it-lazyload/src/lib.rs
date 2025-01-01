use markdown_it::{
    parser::core::CoreRule,
    plugins::cmark::{self, inline::image::Image},
    MarkdownIt, Node, NodeValue,
};

struct LazyLoadImageRule;

impl CoreRule for LazyLoadImageRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_mut(|node: &mut Node, _| {
            if node.is::<Image>() {
                if let Some(img) = node.cast::<cmark::inline::image::Image>() {
                    node.replace(LazyLoadImage {
                        cmark_image: Image {
                            url: img.url.clone(),
                            title: img.title.clone(),
                        },
                        loading: "lazy".to_string(),
                    })
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct LazyLoadImage {
    cmark_image: Image,
    loading: String,
}

impl NodeValue for LazyLoadImage {
    // https://github.com/markdown-it-rust/markdown-it/blob/2d7c085046a144d221490331b25ca565ecddbb1b/src/plugins/cmark/inline/image.rs#L16
    fn render(&self, node: &markdown_it::Node, fmt: &mut dyn markdown_it::Renderer) {
        let mut attrs = node.attrs.clone();

        attrs.push(("src", self.cmark_image.url.clone()));
        attrs.push(("alt", node.collect_text()));

        if let Some(title) = &self.cmark_image.title {
            attrs.push(("title", title.clone()));
        }

        if !self.cmark_image.url.trim().is_empty() {
            attrs.push(("loading", self.loading.clone()));
        }

        fmt.self_close("img", &attrs);
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<LazyLoadImageRule>().after_all();
}

// https://markdown-it-rust.github.io/markdown-it/
// https://commonmark.org/help/tutorial/08-images.html
#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::LazyLock;

    static MARKDOWN_IT: LazyLock<MarkdownIt> = LazyLock::new(|| {
        let mut mdit = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut mdit);
        add(&mut mdit);

        mdit
    });

    static ADDED_LAZYLOAD_RULE_BEFORE_CMARK_MARKDOWN_IT: LazyLock<MarkdownIt> =
        LazyLock::new(|| {
            let mut mdit = markdown_it::MarkdownIt::new();
            add(&mut mdit);
            markdown_it::plugins::cmark::add(&mut mdit);

            mdit
        });

    #[test]
    fn test_inline_full_url_render() {
        let input = r#"![Rust](https://example.com/example.png)"#;
        let out = MARKDOWN_IT.parse(input).render();
        let expected = r#"<p><img src="https://example.com/example.png" alt="Rust" loading="lazy"></p>
"#;

        assert_eq!(out, expected);
    }

    #[test]
    fn test_add_plugin_before_cmark_render() {
        let input = r#"![Rust](https://example.com/example.png)"#;
        let out = ADDED_LAZYLOAD_RULE_BEFORE_CMARK_MARKDOWN_IT
            .parse(input)
            .render();
        let expected = r#"<p><img src="https://example.com/example.png" alt="Rust" loading="lazy"></p>
"#;

        assert_eq!(out, expected);
    }

    #[test]
    fn test_inline_relative_url_render() {
        let input = r#"![Rust](./example.png)"#;
        let out = MARKDOWN_IT.parse(input).render();
        let expected = r#"<p><img src="./example.png" alt="Rust" loading="lazy"></p>
"#;

        assert_eq!(out, expected);
    }

    #[test]
    fn test_empty_render() {
        let input = "![]()";
        let out = MARKDOWN_IT.parse(input).render();
        // https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt#value
        let expected = r#"<p><img src="" alt=""></p>
"#;

        assert_eq!(out, expected);
    }

    #[test]
    fn test_block_render() {
        let input = r#"
## H2 Title

![](https://example.com/example1.png)
foobar
![](https://example.net/example2.png)
hoge
![](./example3.png) ![Example4](./../example4.png)
"#;

        let out = MARKDOWN_IT.parse(input).render();
        let expected = r#"<h2>H2 Title</h2>
<p><img src="https://example.com/example1.png" alt="" loading="lazy">
foobar
<img src="https://example.net/example2.png" alt="" loading="lazy">
hoge
<img src="./example3.png" alt="" loading="lazy"> <img src="./../example4.png" alt="Example4" loading="lazy"></p>
"#;

        assert_eq!(out, expected);
    }

    #[test]
    fn test_block_title_render() {
        let input = r#"
![Rust][id]

foobar

[id]: example.rust.png "image_title"
"#;

        let out = MARKDOWN_IT.parse(input).render();
        let expected = r#"<p><img src="example.rust.png" alt="Rust" title="image_title" loading="lazy"></p>
<p>foobar</p>
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
        let expected = r#"<p><img src="https://example.com/example.png" alt="Rust" loading="lazy" /></p>
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
<img src="https://example.com/example.png" alt="Rust" loading="lazy"></p>
"#;

        assert_eq!(out, expected);
    }
}
