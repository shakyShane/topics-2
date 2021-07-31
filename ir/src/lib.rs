#[derive(Debug)]
pub struct Ir {
    items: Vec<IrItem>,
}

#[derive(Debug)]
pub enum IrItem {
    Instruction(Instruction),
    Markdown(Markdown),
}

#[derive(Debug, Default)]
pub struct Instruction {
    name: String,
    loc: Option<Location>,
    content: Vec<IrItem>,
}

#[derive(Debug, Default)]
pub struct Markdown {
    content: String,
    loc: Option<Location>,
}

#[derive(Debug, Default)]
pub struct Command {
    name: String,
    description: Option<String>,
    loc: Option<Location>,
    raw: String,
}

#[derive(Debug, Default)]
pub struct Location {
    line_start: usize,
    line_end: usize,
}

fn take_item(Ir { items }: Ir) {
    for item in items {
        match item {
            IrItem::Instruction(inst) => {
                println!("instruction {:#?}", inst);
            }
            IrItem::Markdown(md) => {}
        }
    }
}

#[test]
fn test_take_item() {
    let first_block = "
The screenshot testing process requires at least the following 2 images to be built before they are run.

```
ncouk-client
ncouk-disconnectedlayoutservice
```
You can build these with the following command
";
    let inst = Instruction {
        name: String::from("Build the images"),
        content: vec![IrItem::Markdown(Markdown {
            content: String::from(first_block),
            ..Default::default()
        })],
        ..Default::default()
    };
    let ir = Ir {
        items: vec![IrItem::Instruction(inst)],
    };
    take_item(ir);
}
