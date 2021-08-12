use eyre::Report;
use ir::Ir;
use std::convert::TryInto;

fn from_string() -> eyre::Result<()> {
    let input: Result<Ir, _> = input_md::InputMd::try_new("#hey!")?.try_into();
    println!("{:?}", input);
    Ok(())
}

#[test]
fn test_from_string() {
    let result = from_string();
}
