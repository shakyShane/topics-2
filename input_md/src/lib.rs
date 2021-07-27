use comrak::arena_tree::Node;
use comrak::nodes::{Ast, AstNode, NodeHeading, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use eyre::Report;
use std::str::FromStr;

pub struct InputMd {}

impl FromStr for InputMd {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s)
    }
}

impl InputMd {
    pub fn try_new(s: &str) -> eyre::Result<Self> {
        from_str(s)
    }
}

fn from_str(input: &str) -> eyre::Result<InputMd> {
    let arena = Arena::new();
    let d = parse_document(&arena, input, &ComrakOptions::default());
    process_node(&d);
    Ok(InputMd {})
}

fn process_node<'a>(node: &'a AstNode<'a>) {
    for child in node.children() {
        let ast = child.data.borrow();
        let start_line = ast.start_line;
        match &ast.value {
            NodeValue::Heading(NodeHeading { level: 1, .. }) => {
                let text = collect_single_line_text(&child);
                println!("heading: {}", text)
            }
            NodeValue::CodeBlock(..) => {}
            _ => {
                println!("...");
            }
        }
    }
}

///
/// Single-line text items are identifiers and follow special rules.
///
fn collect_single_line_text<'a>(node: &'a AstNode<'a>) -> String {
    node.children()
        .filter_map(|n| match &n.data.borrow().value {
            NodeValue::Text(t) => Some(std::str::from_utf8(t).unwrap().to_string()),
            // todo, preserve this information?
            NodeValue::Code(t) => Some(std::str::from_utf8(&t.literal).unwrap().to_string()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() -> eyre::Result<()> {
        let cm = InputMd::from_str(
            "# Command: hello world

```shell
echo hello $NAME
```

        ",
        )?;
        Ok(())
    }
}
