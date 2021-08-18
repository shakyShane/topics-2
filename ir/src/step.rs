use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct Step {
    pub content: Vec<IrItem>,
}
