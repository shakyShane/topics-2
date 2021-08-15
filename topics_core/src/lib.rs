use ir::Ir;
use std::convert::TryInto;

fn from_string() -> eyre::Result<()> {
    let input: Result<Ir, _> = input_md::InputMd::try_new("# Instruction: ")?.try_into();
    println!("{:?}", input);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use ir::IrItem;

    #[test]
    fn test_deserialize() -> eyre::Result<()> {
        let input1 = include_str!("../../ir/fixtures/run-screenshots.yaml");
        let ir = Ir::from_yaml_str(input1);
        println!("ir={:#?}", ir);
        Ok(())
    }
}
