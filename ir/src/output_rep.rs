use crate::{IdRef, Ir, IrItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Serialize, Deserialize, TypeScriptify)]
pub struct OutputRep {
    errors: HashMap<String, Vec<ErrorRep>>,
    refs: HashMap<String, Vec<String>>,
    items: HashMap<String, IrItem>,
}

impl TryInto<OutputRep> for Vec<Ir> {
    type Error = eyre::Report;

    fn try_into(self) -> Result<OutputRep, Self::Error> {
        try_into(&self)
    }
}

fn try_into(irs: &[Ir]) -> eyre::Result<OutputRep> {
    let mut output = OutputRep::default();
    for ir in irs {
        println!("ir ns: {:?}", ir.ns);
        for item in &ir.items {
            let id = slug::slugify(&item.id());
            // output.refs.entry(id).or_insert(Default::default());
            // let output_refs = visit(&item, &mut output);
            // println!("output_refs=>{:?}", output_refs);
        }
    }
    println!("refs = {:?}", output.refs);
    Ok(Default::default())
}

// fn visit(item: &IrItem, output: &mut OutputRep) -> Vec<String> {
//     let mut refs: Vec<String> = vec![];
//     match item {
//         IrItem::Action(action) => {
//             for item in &action.content {
//                 println!("here");
//                 refs.extend(visit(item, output));
//             }
//         }
//         IrItem::Instruction(_) => {}
//         IrItem::Markdown(md) => {
//             println!("got markdown");
//         }
//         IrItem::DependencyList(dl) => {
//             let id = slug::slugify(&dl.name);
//             output.refs.entry(id).or_insert(Default::default());
//
//         }
//         IrItem::NamedRefList(_) => {}
//         IrItem::NamedRef(_) => {}
//         IrItem::IdRef(_) => {}
//         IrItem::Step(_) => {}
//         IrItem::Command(_) => {}
//         IrItem::CommandDefinition(_) => {}
//         IrItem::CommandConfig(_) => {}
//         IrItem::Config(_) => {}
//         IrItem::ConfigDefinition(_) => {}
//     }
//     refs
// }

#[derive(thiserror::Error, Debug, Serialize, Deserialize, TypeScriptify)]
#[serde(tag = "error_kind")]
pub enum ErrorRep {
    #[error("{given:?} not found")]
    NotFound { given: Option<String> },
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Ir;
    use std::convert::TryInto;

    #[test]
    fn test_convert_ir_to_output() -> eyre::Result<()> {
        let input1 = include_str!("../fixtures/run-screenshots.yaml");
        let input2 = include_str!("../fixtures/global-config.yaml");
        let ir = Ir::from_yaml_str(input1, "run-screenshots.yaml")?;
        let ir2 = Ir::from_yaml_str(input2, "global-config.yaml")?;
        let irs = vec![ir, ir2];
        let output: OutputRep = irs.try_into()?;
        println!("output {:?}", output);
        Ok(())
    }

    #[test]
    fn test_deserialize_output_rep() -> eyre::Result<()> {
        let input1 = include_str!("../fixtures/output_rep.yaml");
        let output_rep: OutputRep = serde_yaml::from_str(input1)?;
        println!("{:?}", output_rep);
        let as_json = serde_json::to_string_pretty(&output_rep)?;
        println!("{}", as_json);
        Ok(())
    }
}
