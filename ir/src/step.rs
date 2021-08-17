use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Step {
    pub content: Vec<IrItem>,
}
