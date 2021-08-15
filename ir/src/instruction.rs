use crate::location::Location;
use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Instruction {
    name: String,
    loc: Option<Location>,
    content: Vec<IrItem>,
}
