use crate::action::Action;
use instruction::Instruction;
use markdown::Markdown;

mod action;
mod command;
mod dependency_list;
mod instruction;
mod location;
mod markdown;
mod named_ref;
mod named_ref_list;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Ir {
    pub items: Vec<IrItem>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum IrItem {
    Action(Action),
    Instruction(Instruction),
    Markdown(Markdown),
}
