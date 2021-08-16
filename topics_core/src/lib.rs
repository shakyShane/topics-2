use ir::Ir;
use std::convert::TryInto;
use std::path::PathBuf;

fn from_string() -> eyre::Result<()> {
    let input: Result<Ir, _> =
        input_md::InputMd::try_new("# Instruction: ", PathBuf::from("md.md"))?.try_into();
    println!("{:?}", input.expect("can upwrap").ns);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() -> eyre::Result<()> {
        from_string()
    }

    #[test]
    fn test_deserialize() -> eyre::Result<()> {
        let input1 = include_str!("../../ir/fixtures/run-screenshots.yaml");
        let _ir = Ir::from_yaml_str(input1, "run-screenshots.yaml")?;
        let input2 = include_str!("../../ir/fixtures/global-config.yaml");
        let _ir2 = Ir::from_yaml_str(input2, "global-config.yaml")?;
        Ok(())
    }
}
