//! A [markdown_it] plugin to add lazy to image tags' loading property
//!
//! ```
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_lazyload::add(parser);
//! let node = parser.parse("![Rust](https://example.com/example.png)");
//! ```

use markdown_it::{parser::core::CoreRule, plugins::cmark::inline::image::Image, MarkdownIt, Node};

struct LazyLoadImageRule;

impl CoreRule for LazyLoadImageRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        root.walk_mut(|node: &mut Node, _| {
            let has_src = node
                .cast::<Image>()
                .is_some_and(|img| !img.url.trim().is_empty());

            if has_src {
                node.attrs.push(("loading", "lazy".to_string()));
            }
        });
    }
}

/// add lazyload plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<LazyLoadImageRule>().after_all();
}
