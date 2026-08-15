//! A [markdown_it] plugin that renders the image description as a `<sub>` caption
//! placed immediately after the `<img>` tag. The `alt` attribute is also populated
//! with the plain-text description per the CommonMark specification.
//!
//! ```
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_image_caption::add(parser);
//! let node = parser.parse("![image-description](img.jpg)");
//! ```

use markdown_it::{
    parser::core::CoreRule,
    plugins::cmark::{self, inline::image::Image},
    MarkdownIt, Node, NodeValue,
};

struct ImageCaptionRule;

impl CoreRule for ImageCaptionRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_mut(|node: &mut Node, _| {
            if node.is::<Image>() {
                if let Some(img) = node.cast::<cmark::inline::image::Image>() {
                    node.replace(CaptionImage {
                        cmark_image: Image {
                            url: img.url.clone(),
                            title: img.title.clone(),
                        },
                    })
                }
            }
        });
    }
}

#[derive(Debug)]
pub struct CaptionImage {
    pub cmark_image: Image,
}

impl NodeValue for CaptionImage {
    fn render(&self, node: &markdown_it::Node, fmt: &mut dyn markdown_it::Renderer) {
        let mut attrs = node.attrs.clone();
        let has_src = !self.cmark_image.url.trim().is_empty();
        let has_description = !node.children.is_empty();

        attrs.push(("src", self.cmark_image.url.clone()));
        attrs.push(("alt", node.collect_text()));

        if let Some(title) = &self.cmark_image.title {
            attrs.push(("title", title.clone()));
        }

        fmt.self_close("img", &attrs);

        if has_src && has_description {
            fmt.open("sub", &[]);
            fmt.contents(&node.children);
            fmt.close("sub");
        }
    }
}

/// add image-caption plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<ImageCaptionRule>().after_all();
}
