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
    parser::{core::CoreRule, extset::NodeExt},
    plugins::cmark::inline::image::Image,
    MarkdownIt, Node, NodeValue, Renderer,
};

struct ImageCaptionRule;

impl CoreRule for ImageCaptionRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_mut(|node: &mut Node, _| {
            // Skip nodes this rule already wrapped, otherwise walk_mut would
            // descend into the newly added child and wrap it again.
            if node.ext.contains::<Captioned>() {
                return;
            }

            let has_src = node
                .cast::<Image>()
                .is_some_and(|img| !img.url.trim().is_empty());
            let has_description = !node.children.is_empty();

            if has_src && has_description {
                node.ext.insert(Captioned);
                let image = std::mem::take(node);
                *node = Node::new(CaptionImage);
                node.children.push(image);
            }
        });
    }
}

#[derive(Debug)]
struct Captioned;

impl NodeExt for Captioned {}

/// Wraps an `Image` node, keeping it intact as its only child instead of
/// replacing it, so plugins that inspect `Image` nodes directly (e.g.
/// `markdown-it-lazyload`) keep working regardless of registration order.
#[derive(Debug)]
pub struct CaptionImage;

impl NodeValue for CaptionImage {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        fmt.contents(&node.children);

        fmt.open("sub", &[]);
        fmt.contents(&node.children[0].children);
        fmt.close("sub");
    }
}

/// add image-caption plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<ImageCaptionRule>().after_all();
}
