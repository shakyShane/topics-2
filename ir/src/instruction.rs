use crate::location::Location;
use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct Instruction {
    pub name: String,
    loc: Option<Location>,
    pub(crate) content: Vec<IrItem>,
}
