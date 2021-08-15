use ir::Ir;

fn main() {
    let input1 = include_str!("../../fixtures/run-screenshots.yaml");
    let input2 = include_str!("../../fixtures/global-config.yaml");
    let mut ir = Ir::from_yaml_str(input1).expect("can deserialze 1 from yaml");
    let ir2 = Ir::from_yaml_str(input2).expect("can deserialze 2 from yaml");
    ir.items.extend(ir2.items);
    let json = serde_json::to_string_pretty(&ir).expect("can run yaml");
    println!("{}", json);
}
