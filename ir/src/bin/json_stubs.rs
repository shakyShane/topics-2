use ir::output_rep::OutputRep;
use ir::Ir;
use std::convert::TryInto;

fn main() {
    let input1 = include_str!("../../fixtures/run-screenshots.yaml");
    let input2 = include_str!("../../fixtures/global-config.yaml");
    let ir = Ir::from_yaml_str(input1, "run-screenshots.yaml").unwrap();
    let ir2 = Ir::from_yaml_str(input2, "global-config.yaml").unwrap();
    let irs = vec![ir, ir2];
    let output: OutputRep = irs.try_into().expect("can convert");
    let json = serde_json::to_string_pretty(&output).expect("can run yaml");
    println!("{}", json);
}
