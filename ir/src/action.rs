use crate::IrItem;
use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, Hash, TypeScriptify)]
pub struct Action {
    pub(crate) name: String,
    pub content: Vec<IrItem>,
}
