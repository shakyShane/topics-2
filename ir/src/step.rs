use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Default, Deserialize, Serialize, TypeScriptify)]
pub struct Step {
    pub(crate) name: String,
    content: Vec<IrItem>,
}
