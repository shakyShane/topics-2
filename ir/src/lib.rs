pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;
pub use typescript_definitions::TypeScriptify;

pub use input_lang::InputLanguage;
pub use instruction::Instruction;
pub use markdown::Markdown;

pub use crate::{
    action::Action,
    command::{Command, CommandConfig, CommandDefinition, CommandParams},
    config::{Config, ConfigDefinition, ConfigParams},
    dependency_list::DependencyList,
    named_ref::IdRef,
    named_ref::NamedRef,
    named_ref_list::NamedRefList,
    step::Step,
};

pub mod action;
pub mod command;
pub mod config;
pub mod dependency_list;
pub mod input_lang;
pub mod instruction;
pub mod location;
pub mod markdown;
pub mod named_ref;
pub mod named_ref_list;
pub mod output_rep;
pub mod step;

#[derive(Debug, Deserialize, Serialize, TypeScriptify)]
pub struct Ir {
    pub ns: String,
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
    IdRef(IdRef),
    Step(Step),
    Command(Command),
    CommandDefinition(CommandDefinition),
    CommandConfig(CommandConfig),
    Config(Config),
    ConfigDefinition(ConfigDefinition),
}

impl IrItem {
    pub fn id(&self) -> String {
        match self {
            IrItem::Action(ac) => format!("Action_{}", slug::slugify(&ac.name)),
            IrItem::Instruction(ins) => format!("Instruction_{}", slug::slugify(&ins.name)),
            IrItem::Markdown(_) => format!("Markdown___{}", nanoid::nanoid!()),
            IrItem::DependencyList(_dl) => format!("DependencyList___{}", nanoid::nanoid!()),
            IrItem::NamedRefList(_) => format!("NamedRefList___{}", nanoid::nanoid!()),
            IrItem::NamedRef(_) => format!("NamedRef___{}", nanoid::nanoid!()),
            IrItem::IdRef(_) => format!("IdRef___{}", nanoid::nanoid!()),
            IrItem::Step(_step) => format!("Step___{}", nanoid::nanoid!()),
            IrItem::Command(cmd) => format!("Command_{}", slug::slugify(&cmd.name)),
            IrItem::CommandDefinition(_) => format!("CommandDefinition___{}", nanoid::nanoid!()),
            IrItem::CommandConfig(_) => format!("CommandConfig___{}", nanoid::nanoid!()),
            IrItem::Config(cfg) => format!("Config_{}", slug::slugify(&cfg.name)),
            IrItem::ConfigDefinition(_) => format!("ConfigDefinition___{}", nanoid::nanoid!()),
        }
    }
    pub fn children(&self) -> Option<&Vec<IrItem>> {
        match self {
            IrItem::Action(act) => Some(&act.content),
            IrItem::Instruction(_) => None,
            IrItem::Markdown(_) => None,
            IrItem::DependencyList(dl) => Some(&dl.content),
            IrItem::NamedRefList(nrl) => Some(&nrl.content),
            IrItem::NamedRef(_) => None,
            IrItem::IdRef(_) => None,
            IrItem::Step(step) => Some(&step.content),
            IrItem::Command(cmd) => Some(&cmd.content),
            IrItem::CommandDefinition(_) => None,
            IrItem::CommandConfig(_) => None,
            IrItem::Config(cnf) => Some(&cnf.content),
            IrItem::ConfigDefinition(_) => None,
        }
    }
}

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
    /// let r = Ir::from_yaml_str(input1, "run-screenshots.yaml");
    /// assert!(r.is_ok())
    /// ```
    ///
    pub fn from_yaml_str(s: &str, ns: &str) -> eyre::Result<Self> {
        let item: IrItem = serde_yaml::from_str(s)?;
        Ok(Self {
            items: vec![item],
            ns: ns.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_ir() -> eyre::Result<()> {
        let input1 = include_str!("../fixtures/run-screenshots.yaml");
        let ir = Ir::from_yaml_str(input1, "run-screenshots.yaml")?;
        let json = serde_json::to_string_pretty(&ir)?;
        println!("{}", json);
        Ok(())
    }

    #[test]
    fn test_resolve() -> eyre::Result<()> {
        let yaml = r#"
        items:
          - { kind: IdRef, id: "01_01" } 
          "#;

        let s: Result<Ir, _> = serde_yaml::from_str(yaml);
        println!("{:?}", s);
        Ok(())
    }
}
