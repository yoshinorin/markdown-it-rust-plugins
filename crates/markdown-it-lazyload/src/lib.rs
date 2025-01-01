//! A [markdown_it] plugin to add lazy to image tags' loading property
//!
//! ```
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_lazyload::add(parser);
//! let node = parser.parse("![Rust](https://example.com/example.png)");
//! ```

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

/// add lazyload plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<LazyLoadImageRule>().after_all();
}
