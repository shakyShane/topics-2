use eyre::Report;
use ir::Ir;
use std::convert::TryInto;

fn from_string() -> eyre::Result<()> {
    let input: Result<Ir, _> = input_md::InputMd::try_new("# Instruction: ")?.try_into();
    println!("{:?}", input);
    Ok(())
}

#[test]
fn test_from_string() {
    let input1 = include_str!("../../input_md/fixtures/run-screenshots.md");
    let input2 = include_str!("../../input_md/fixtures/global-config.md");

    let result = from_string();
}
