pub use crate::action::Action;
pub use crate::command::{Command, CommandConfig, CommandDefinition, CommandParams, InputLanguage};
pub use crate::dependency_list::DependencyList;
pub use crate::named_ref::NamedRef;
pub use crate::named_ref_list::NamedRefList;
pub use crate::step::Step;
pub use instruction::Instruction;
pub use markdown::Markdown;
pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;
pub use typescript_definitions::TypeScriptify;

pub mod action;
pub mod command;
pub mod dependency_list;
pub mod instruction;
pub mod location;
pub mod markdown;
pub mod named_ref;
pub mod named_ref_list;
pub mod step;

#[derive(Debug, Deserialize, Serialize, TypeScriptify)]
pub struct Ir {
    pub items: Vec<IrItem>,
}

#[derive(Debug, Deserialize, Serialize, TypeScriptify)]
#[serde(tag = "kind")]
pub enum IrItem {
    Action(Action),
    Instruction(Instruction),
    Markdown(Markdown),
    DependencyList(DependencyList),
    NamedRefList(NamedRefList),
    NamedRef(NamedRef),
    Step(Step),
    Command(Command),
    CommandDefinition(CommandDefinition),
    CommandConfig(CommandConfig),
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
        let ir = Ir::from_yaml_str(input1)?;
        let json = serde_json::to_string_pretty(&ir)?;
        println!("{}", json);
        Ok(())
    }
}
