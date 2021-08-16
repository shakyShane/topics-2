use comrak::nodes::{AstNode, NodeHeading, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use eyre::Report;
use ir::Ir;
use std::convert::TryInto;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct InputMd {
    pb: PathBuf,
}

impl InputMd {
    pub fn try_new(s: &str, pb: PathBuf) -> eyre::Result<Self> {
        from_str(s, pb)
    }
}

impl TryInto<Ir> for InputMd {
    type Error = Report;

    fn try_into(self) -> Result<Ir, Self::Error> {
        Ok(Ir {
            items: vec![],
            ns: self.pb.to_string_lossy().to_string(),
        })
    }
}

fn from_str(input: &str, pb: PathBuf) -> eyre::Result<InputMd> {
    let arena = Arena::new();
    let d = parse_document(&arena, input, &ComrakOptions::default());
    process_node(&d);
    Ok(InputMd { pb })
}

fn process_node<'a>(node: &'a AstNode<'a>) {
    for child in node.children() {
        let ast = child.data.borrow();
        let _start_line = ast.start_line;
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
        let input1 = include_str!("../fixtures/run-screenshots.md");
        let _input2 = include_str!("../fixtures/global-config.md");
        let _cm = InputMd::from_str(input1)?;
        Ok(())
    }
}
