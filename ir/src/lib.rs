use crate::action::Action;
use crate::dependency_list::DependencyList;
use crate::named_ref::NamedRef;
use crate::named_ref_list::NamedRefList;
use instruction::Instruction;
use markdown::Markdown;
use thiserror::Error;

mod action;
mod command;
mod dependency_list;
mod instruction;
mod location;
mod markdown;
mod named_ref;
mod named_ref_list;
mod step;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Ir {
    pub items: Vec<IrItem>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind")]
pub enum IrItem {
    Action(Action),
    Instruction(Instruction),
    Markdown(Markdown),
    DependencyList(DependencyList),
    NamedRefList(NamedRefList),
    NamedRef(NamedRef),
    Step(step::Step),
    Command(command::Command),
    CommandDefinition(command::CommandDefinition),
    CommandConfig(command::CommandConfig),
}
//
// #[derive(thiserror::Error, Debug)]
// pub enum FormatError {
//     #[error("Invalid header (expected {expected:?}, got {found:?})")]
//     InvalidHeader { expected: String, found: String },
// }

impl Ir {
    ///
    /// Helper to create an [Ir] directly from a yaml string
    ///
    /// ```rust
    /// use ir::Ir;
    ///
    /// let input1 = include_str!("../fixtures/run-screenshots.yaml");
    /// let r = Ir::from_yaml_str(input1);
    /// assert!(r.is_ok())
    /// ```
    ///
    pub fn from_yaml_str(s: &str) -> eyre::Result<Self> {
        let item: IrItem = serde_yaml::from_str(s)?;
        Ok(Self { items: vec![item] })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_deserialize() -> eyre::Result<()> {
        let input1 = include_str!("../fixtures/run-screenshots.yaml");
        let ir: Result<IrItem, _> = serde_yaml::from_str(input1);
        println!("ir={:#?}", ir);
        Ok(())
    }
}
