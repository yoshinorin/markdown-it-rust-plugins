//! A [markdown_it] plugin that wraps a standalone image (a paragraph whose only
//! content is a single image) in `<figure>`, rendering the image description as
//! `<figcaption>`. `<figure>` is a block-level element, so an image that shares
//! its paragraph with other text is left untouched (plain `<img>`, no caption).
//!
//! ```
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_image_figure::add(parser);
//! let node = parser.parse("![image-description](img.jpg)");
//! ```

use markdown_it::{
    parser::core::CoreRule,
    plugins::cmark::{block::paragraph::Paragraph, inline::image::Image},
    MarkdownIt, Node, NodeValue, Renderer,
};

struct ImageFigureRule;

impl CoreRule for ImageFigureRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_mut(|node: &mut Node, _| {
            if !node.is::<Paragraph>() || node.children.len() != 1 {
                return;
            }

            let should_wrap = node.children[0]
                .cast::<Image>()
                .is_some_and(|img| !img.url.trim().is_empty())
                && !node.children[0].children.is_empty();

            if should_wrap {
                node.replace(Figure);
            }
        });
    }
}

/// Wraps a standalone `Image` node, keeping it intact as its only child
/// instead of replacing it, so plugins that inspect `Image` nodes directly
/// (e.g. `markdown-it-lazyload`) keep working regardless of registration order.
#[derive(Debug)]
pub struct Figure;

impl NodeValue for Figure {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.cr();
        fmt.open("figure", &node.attrs);
        fmt.contents(&node.children);

        fmt.open("figcaption", &[]);
        fmt.contents(&node.children[0].children);
        fmt.close("figcaption");

        fmt.close("figure");
        fmt.cr();
    }
}

/// add image-figure plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<ImageFigureRule>().after_all();
}
